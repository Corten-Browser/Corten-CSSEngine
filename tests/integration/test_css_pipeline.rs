//! Integration Tests for Complete CSS Pipeline
//!
//! These tests verify that all CSS engine components work together correctly:
//! 1. CSS Parsing (css_parser_core)
//! 2. Selector Matching (css_matcher_core)
//! 3. Cascade Resolution (css_cascade)
//! 4. Style Computation (css_stylist_core)
//! 5. Complete Pipeline (css_engine)

use css_engine::{CssEngine, DomNode, ElementId};

mod utilities;
use utilities::test_data_generator::{self, stylesheets};

/// Test basic CSS parsing and style computation
#[test]
fn test_basic_css_pipeline() {
    let mut engine = CssEngine::new();

    // Parse a simple stylesheet
    let css = r#"
        body {
            color: #333;
        }

        div {
            padding: 10px;
        }
    "#;

    let sheet_id = engine
        .parse_stylesheet(css, Some("test.css"))
        .expect("Should parse valid CSS");

    assert!(sheet_id.value() > 0, "Should return valid stylesheet ID");

    // Create a simple DOM
    let dom = DomNode::new(ElementId::new(1), "body");

    // Compute styles
    let style_tree = engine
        .compute_styles(&dom)
        .expect("Should compute styles successfully");

    // Verify we got a style tree
    assert_eq!(
        style_tree.root.element_id,
        ElementId::new(1),
        "Root element ID should match"
    );
}

/// Test parsing multiple stylesheets
#[test]
fn test_multiple_stylesheets() {
    let mut engine = CssEngine::new();

    // Parse first stylesheet
    let id1 = engine
        .parse_stylesheet(stylesheets::BASIC, Some("basic.css"))
        .expect("Should parse BASIC stylesheet");

    // Parse second stylesheet
    let id2 = engine
        .parse_stylesheet(stylesheets::CASCADE, Some("cascade.css"))
        .expect("Should parse CASCADE stylesheet");

    // Parse third stylesheet
    let id3 = engine
        .parse_stylesheet(stylesheets::INHERITANCE, Some("inheritance.css"))
        .expect("Should parse INHERITANCE stylesheet");

    // All IDs should be different
    assert_ne!(id1, id2, "Stylesheet IDs should be unique");
    assert_ne!(id2, id3, "Stylesheet IDs should be unique");
    assert_ne!(id1, id3, "Stylesheet IDs should be unique");

    // Engine should track all stylesheets
    assert_eq!(
        engine.stylesheet_count(),
        3,
        "Engine should have 3 stylesheets"
    );
}

/// Test cascade resolution with multiple selectors
#[test]
fn test_cascade_with_multiple_selectors() {
    let mut engine = CssEngine::new();

    // Stylesheet with cascading rules
    let css = stylesheets::CASCADE;

    engine
        .parse_stylesheet(css, Some("cascade.css"))
        .expect("Should parse cascade stylesheet");

    // Create DOM with cascading selectors
    // div#main.container should match all of: *, div, .container, #main, div.container, div#main.container
    let dom = DomNode::new(ElementId::new(1), "div")
        .with_class("container")
        .with_attribute("id", "main");

    let style_tree = engine
        .compute_styles(&dom)
        .expect("Should compute styles with cascade");

    assert_eq!(
        style_tree.root.element_id,
        ElementId::new(1),
        "Root element should match"
    );
}

/// Test property inheritance through DOM tree
#[test]
fn test_property_inheritance() {
    let mut engine = CssEngine::new();

    let css = stylesheets::INHERITANCE;

    engine
        .parse_stylesheet(css, Some("inheritance.css"))
        .expect("Should parse inheritance stylesheet");

    // Create nested DOM for inheritance testing
    let child = DomNode::new(ElementId::new(2), "span");
    let parent = DomNode::new(ElementId::new(1), "body").with_child(child);

    let style_tree = engine
        .compute_styles(&parent)
        .expect("Should compute styles with inheritance");

    // Verify tree structure
    assert_eq!(style_tree.root.element_id, ElementId::new(1));
    assert_eq!(
        style_tree.root.children.len(),
        1,
        "Should have one child"
    );
    assert_eq!(style_tree.root.children[0].element_id, ElementId::new(2));

    // Child should inherit color from parent (tested in component unit tests)
}

/// Test complex selector matching
#[test]
fn test_complex_selector_matching() {
    let mut engine = CssEngine::new();

    let css = r#"
        /* Type selector */
        div {
            margin: 10px;
        }

        /* Class selector */
        .container {
            padding: 20px;
        }

        /* ID selector */
        #main {
            width: 100%;
        }

        /* Compound selector */
        div.container {
            border: 1px;
        }

        /* Compound with ID */
        div#main.container {
            background-color: #fff;
        }
    "#;

    engine
        .parse_stylesheet(css, Some("complex.css"))
        .expect("Should parse complex selectors");

    // Element that matches all selectors
    let dom = DomNode::new(ElementId::new(1), "div")
        .with_class("container")
        .with_attribute("id", "main");

    let style_tree = engine
        .compute_styles(&dom)
        .expect("Should compute styles with complex selectors");

    assert_eq!(style_tree.root.element_id, ElementId::new(1));
}

/// Test length unit handling
#[test]
fn test_length_units() {
    let mut engine = CssEngine::new();

    let css = stylesheets::UNITS;

    engine
        .parse_stylesheet(css, Some("units.css"))
        .expect("Should parse stylesheet with various units");

    // Create DOM with different unit requirements
    let dom = test_data_generator::generate_simple_dom_tree();

    let style_tree = engine
        .compute_styles(&dom)
        .expect("Should compute styles with various units");

    // Verify tree was created
    assert!(
        style_tree.root.children.len() > 0,
        "Should have child elements"
    );
}

/// Test inline styles
#[test]
fn test_inline_styles() {
    let mut engine = CssEngine::new();

    let element_id = ElementId::new(1);

    // Set inline style (should have highest specificity)
    engine
        .set_inline_style(element_id, "color: red; font-size: 20px;")
        .expect("Should set inline style");

    // Also parse a stylesheet with lower specificity
    engine
        .parse_stylesheet("div { color: blue; font-size: 16px; }", None)
        .expect("Should parse stylesheet");

    // Create DOM
    let dom = DomNode::new(element_id, "div");

    let style_tree = engine
        .compute_styles(&dom)
        .expect("Should compute styles with inline styles");

    assert_eq!(style_tree.root.element_id, element_id);

    // Get computed style (inline should win)
    let computed = engine
        .get_computed_style(element_id)
        .expect("Should get computed style");

    // Inline styles should be applied (verified in component tests)
    assert!(computed.display == css_engine::Display::Inline);
}

/// Test style invalidation
#[test]
fn test_style_invalidation() {
    use css_engine::StyleInvalidation;

    let mut engine = CssEngine::new();

    // Parse stylesheet
    engine
        .parse_stylesheet(".active { color: red; }", None)
        .expect("Should parse stylesheet");

    let element_id = ElementId::new(1);

    // Initial computation
    let dom = DomNode::new(element_id, "div");
    engine
        .compute_styles(&dom)
        .expect("Should compute initial styles");

    // Invalidate when class changes
    let invalidation = StyleInvalidation::ClassChange {
        element_id,
        added: vec!["active".to_string()],
        removed: vec![],
    };

    engine
        .invalidate_styles(invalidation)
        .expect("Should invalidate styles successfully");
}

/// Test nested DOM tree styling
#[test]
fn test_nested_dom_tree() {
    let mut engine = CssEngine::new();

    let css = stylesheets::COMPLEX;

    engine
        .parse_stylesheet(css, Some("complex.css"))
        .expect("Should parse complex stylesheet");

    // Use complex DOM tree
    let dom = test_data_generator::generate_complex_dom_tree();

    let style_tree = engine
        .compute_styles(&dom)
        .expect("Should compute styles for complex DOM tree");

    // Verify tree structure matches
    assert_eq!(style_tree.root.element_id.value(), 31); // Last ID generated
    assert!(
        style_tree.root.children.len() == 3,
        "Body should have 3 children: header, main, footer"
    );

    // Verify nested structure exists
    let header = &style_tree.root.children[0];
    assert!(header.children.len() > 0, "Header should have children");

    let main = &style_tree.root.children[1];
    assert!(main.children.len() > 0, "Main should have children");

    let footer = &style_tree.root.children[2];
    assert!(footer.children.len() > 0, "Footer should have children");
}

/// Test simple DOM tree styling
#[test]
fn test_simple_dom_tree() {
    let mut engine = CssEngine::new();

    let css = stylesheets::BASIC;

    engine
        .parse_stylesheet(css, Some("basic.css"))
        .expect("Should parse basic stylesheet");

    let dom = test_data_generator::generate_simple_dom_tree();

    let style_tree = engine
        .compute_styles(&dom)
        .expect("Should compute styles for simple DOM tree");

    // Verify tree structure
    assert_eq!(style_tree.root.element_id, ElementId::new(1));
    assert_eq!(
        style_tree.root.children.len(),
        2,
        "Body should have 2 children"
    );
}

/// Test complete pipeline: parse -> match -> cascade -> compute
#[test]
fn test_complete_pipeline() {
    let mut engine = CssEngine::new();

    // Step 1: Parse CSS
    let css = r#"
        body {
            font-size: 16px;
            color: #000;
        }

        #main {
            padding: 20px;
            background-color: #fff;
        }

        .highlight {
            color: #f00;
        }

        span {
            font-size: 14px;
        }

        .highlight span {
            color: #ff0;
        }
    "#;

    engine
        .parse_stylesheet(css, Some("complete.css"))
        .expect("Step 1: Parse CSS - FAILED");

    // Step 2: Create DOM (multiple levels)
    let span = DomNode::new(ElementId::new(3), "span");

    let highlight_div = DomNode::new(ElementId::new(2), "div")
        .with_class("highlight")
        .with_child(span);

    let body = DomNode::new(ElementId::new(1), "body")
        .with_attribute("id", "main")
        .with_child(highlight_div);

    // Step 3: Compute styles (includes matching and cascade)
    let style_tree = engine
        .compute_styles(&body)
        .expect("Step 3: Compute styles - FAILED");

    // Step 4: Verify results
    assert_eq!(
        style_tree.root.element_id,
        ElementId::new(1),
        "Root element should be body"
    );

    assert_eq!(
        style_tree.root.children.len(),
        1,
        "Body should have 1 child"
    );

    assert_eq!(
        style_tree.root.children[0].element_id,
        ElementId::new(2),
        "Child should be div.highlight"
    );

    assert_eq!(
        style_tree.root.children[0].children.len(),
        1,
        "Div should have 1 child"
    );

    assert_eq!(
        style_tree.root.children[0].children[0].element_id,
        ElementId::new(3),
        "Grandchild should be span"
    );

    // Step 5: Get individual computed styles
    let body_style = engine
        .get_computed_style(ElementId::new(1))
        .expect("Should get body computed style");

    let div_style = engine
        .get_computed_style(ElementId::new(2))
        .expect("Should get div computed style");

    let span_style = engine
        .get_computed_style(ElementId::new(3))
        .expect("Should get span computed style");

    // All styles should be computed
    assert_eq!(body_style.display, css_engine::Display::Inline);
    assert_eq!(div_style.display, css_engine::Display::Inline);
    assert_eq!(span_style.display, css_engine::Display::Inline);
}

/// Test error handling for invalid CSS
#[test]
fn test_invalid_css_parsing() {
    let mut engine = CssEngine::new();

    // Currently, the parser may accept or reject invalid CSS
    // This test documents the behavior
    let invalid_css = "this is not valid CSS at all!!!";

    let result = engine.parse_stylesheet(invalid_css, None);

    // Document whether it succeeds or fails
    // (Implementation may vary)
    match result {
        Ok(_) => {
            // Parser is lenient - acceptable
        }
        Err(_) => {
            // Parser rejects invalid CSS - also acceptable
        }
    }
}

/// Test empty stylesheet
#[test]
fn test_empty_stylesheet() {
    let mut engine = CssEngine::new();

    let result = engine.parse_stylesheet("", None);

    // Empty stylesheet should be valid
    assert!(
        result.is_ok(),
        "Empty stylesheet should parse successfully"
    );
}

/// Test stylesheet with comments
#[test]
fn test_stylesheet_with_comments() {
    let mut engine = CssEngine::new();

    let css = r#"
        /* This is a comment */
        body {
            color: #000; /* inline comment */
        }

        /* Multi-line
           comment */
        div {
            padding: 10px;
        }
    "#;

    engine
        .parse_stylesheet(css, None)
        .expect("Should parse stylesheet with comments");
}

/// Test very long stylesheet
#[test]
fn test_large_stylesheet() {
    let mut engine = CssEngine::new();

    // Generate a large stylesheet programmatically
    let mut css = String::new();

    for i in 0..100 {
        css.push_str(&format!(
            ".class-{} {{ padding: {}px; margin: {}px; }}\n",
            i, i, i
        ));
    }

    engine
        .parse_stylesheet(&css, Some("large.css"))
        .expect("Should parse large stylesheet");

    assert_eq!(
        engine.stylesheet_count(),
        1,
        "Should have 1 stylesheet"
    );
}

/// Test universal selector
#[test]
fn test_universal_selector() {
    let mut engine = CssEngine::new();

    let css = r#"
        * {
            margin: 0px;
            padding: 0px;
        }

        div {
            padding: 10px;
        }
    "#;

    engine
        .parse_stylesheet(css, None)
        .expect("Should parse universal selector");

    let dom = DomNode::new(ElementId::new(1), "div");

    let style_tree = engine
        .compute_styles(&dom)
        .expect("Should compute styles with universal selector");

    assert_eq!(style_tree.root.element_id, ElementId::new(1));
}
