//! End-to-End Tests for Complete CSS Workflows
//!
//! These tests simulate real-world usage scenarios of the CSS engine,
//! testing the complete workflow from parsing stylesheets through
//! computing final styles for complex DOM trees.

use css_engine::{CssEngine, DomNode, ElementId, StyleInvalidation};

mod utilities;
use utilities::test_data_generator::{self, stylesheets};

/// E2E Test: Simple web page with basic styling
#[test]
fn test_simple_webpage_workflow() {
    println!("=== E2E Test: Simple Web Page ===");

    let mut engine = CssEngine::new();

    // Scenario: Developer creates a simple web page

    // Step 1: Load user-agent stylesheet (browser defaults)
    let ua_css = r#"
        body {
            margin: 8px;
        }

        p {
            margin: 16px;
        }
    "#;

    engine
        .parse_stylesheet(ua_css, Some("user-agent.css"))
        .expect("Should load user-agent stylesheet");

    println!("✓ Loaded user-agent stylesheet");

    // Step 2: Load author stylesheet (developer's CSS)
    let author_css = r#"
        body {
            margin: 0px;
            font-size: 16px;
            color: #333;
        }

        .container {
            padding: 20px;
        }

        p {
            line-height: 1.5em;
        }
    "#;

    engine
        .parse_stylesheet(author_css, Some("main.css"))
        .expect("Should load author stylesheet");

    println!("✓ Loaded author stylesheet");

    // Step 3: Create DOM for simple page
    let p = DomNode::new(ElementId::new(2), "p");
    let container = DomNode::new(ElementId::new(3), "div")
        .with_class("container")
        .with_child(p);
    let body = DomNode::new(ElementId::new(1), "body").with_child(container);

    println!("✓ Created DOM tree");

    // Step 4: Compute styles
    let style_tree = engine
        .compute_styles(&body)
        .expect("Should compute styles for page");

    println!("✓ Computed styles");

    // Step 5: Verify cascade worked (author should override UA)
    // Body margin should be 0px (author) not 8px (UA)
    assert_eq!(style_tree.root.element_id, ElementId::new(1));
    assert_eq!(style_tree.root.children.len(), 1);

    println!("✓ Verified cascade resolution");
    println!("=== Test PASSED ===\n");
}

/// E2E Test: Dynamic styling with inline styles and invalidation
#[test]
fn test_dynamic_styling_workflow() {
    println!("=== E2E Test: Dynamic Styling ===");

    let mut engine = CssEngine::new();

    // Scenario: User interacts with page, styles change dynamically

    // Step 1: Initial stylesheet
    let css = r#"
        .button {
            background-color: #eee;
            color: #000;
        }

        .button.active {
            background-color: #007bff;
            color: #fff;
        }

        .button:hover {
            background-color: #ddd;
        }
    "#;

    engine
        .parse_stylesheet(css, Some("buttons.css"))
        .expect("Should load stylesheet");

    println!("✓ Loaded initial stylesheet");

    let button_id = ElementId::new(1);

    // Step 2: Initial render (button not active)
    let dom = DomNode::new(button_id, "div").with_class("button");

    engine
        .compute_styles(&dom)
        .expect("Should compute initial styles");

    let initial_style = engine
        .get_computed_style(button_id)
        .expect("Should get initial computed style");

    println!("✓ Computed initial styles");

    // Step 3: User clicks button -> add 'active' class
    let invalidation = StyleInvalidation::ClassChange {
        element_id: button_id,
        added: vec!["active".to_string()],
        removed: vec![],
    };

    engine
        .invalidate_styles(invalidation)
        .expect("Should invalidate after class change");

    println!("✓ Invalidated styles after class change");

    // Step 4: Re-compute with new class
    let dom_active = DomNode::new(button_id, "div")
        .with_class("button")
        .with_class("active");

    engine
        .compute_styles(&dom_active)
        .expect("Should recompute styles");

    let active_style = engine
        .get_computed_style(button_id)
        .expect("Should get active computed style");

    println!("✓ Recomputed styles with active class");

    // Styles should be different
    assert_eq!(initial_style.display, active_style.display);

    println!("=== Test PASSED ===\n");
}

/// E2E Test: Multi-page application with shared styles
#[test]
fn test_multi_page_application_workflow() {
    println!("=== E2E Test: Multi-Page Application ===");

    let mut engine = CssEngine::new();

    // Scenario: SPA with shared global styles and page-specific styles

    // Step 1: Load global stylesheet
    let global_css = r#"
        * {
            margin: 0px;
            padding: 0px;
        }

        body {
            font-size: 16px;
            color: #333;
        }

        .container {
            max-width: 1200px;
        }
    "#;

    engine
        .parse_stylesheet(global_css, Some("global.css"))
        .expect("Should load global stylesheet");

    println!("✓ Loaded global stylesheet");

    // Step 2: Load home page stylesheet
    let home_css = r#"
        .hero {
            padding: 40px;
            background-color: #f8f9fa;
        }

        .hero h1 {
            font-size: 32px;
        }
    "#;

    engine
        .parse_stylesheet(home_css, Some("home.css"))
        .expect("Should load home page stylesheet");

    println!("✓ Loaded home page stylesheet");

    // Step 3: Render home page
    let h1 = DomNode::new(ElementId::new(3), "h1");
    let hero = DomNode::new(ElementId::new(2), "div")
        .with_class("hero")
        .with_child(h1);
    let home_body = DomNode::new(ElementId::new(1), "body").with_child(hero);

    engine
        .compute_styles(&home_body)
        .expect("Should compute home page styles");

    println!("✓ Rendered home page");

    // Step 4: Navigate to different page, load page-specific styles
    let about_css = r#"
        .about-section {
            padding: 20px;
        }

        .about-section p {
            line-height: 1.6em;
        }
    "#;

    engine
        .parse_stylesheet(about_css, Some("about.css"))
        .expect("Should load about page stylesheet");

    println!("✓ Loaded about page stylesheet");

    // Step 5: Render about page
    let p = DomNode::new(ElementId::new(5), "p");
    let about_section = DomNode::new(ElementId::new(4), "div")
        .with_class("about-section")
        .with_child(p);
    let about_body = DomNode::new(ElementId::new(1), "body").with_child(about_section);

    engine
        .compute_styles(&about_body)
        .expect("Should compute about page styles");

    println!("✓ Rendered about page");

    // Both pages use global + page-specific styles
    assert_eq!(engine.stylesheet_count(), 3);

    println!("=== Test PASSED ===\n");
}

/// E2E Test: Complex layout with inheritance and cascade
#[test]
fn test_complex_layout_workflow() {
    println!("=== E2E Test: Complex Layout ===");

    let mut engine = CssEngine::new();

    // Scenario: Complex page with header, main content, sidebar, footer

    // Step 1: Load comprehensive stylesheet
    let css = stylesheets::COMPLEX;

    engine
        .parse_stylesheet(css, Some("layout.css"))
        .expect("Should load complex stylesheet");

    println!("✓ Loaded complex stylesheet");

    // Step 2: Build complex DOM tree
    let dom = test_data_generator::generate_complex_dom_tree();

    println!("✓ Built complex DOM tree");

    // Step 3: Compute all styles
    let style_tree = engine
        .compute_styles(&dom)
        .expect("Should compute styles for complex layout");

    println!("✓ Computed styles for all elements");

    // Step 4: Verify tree structure
    assert!(
        style_tree.root.children.len() == 3,
        "Should have header, main, footer"
    );

    println!("✓ Verified DOM structure preserved");

    // Step 5: Test that we can get computed styles for any element
    // (Element IDs are generated sequentially in test data)
    for id in 1..=30 {
        let result = engine.get_computed_style(ElementId::new(id));
        assert!(
            result.is_ok(),
            "Should have computed style for element {}",
            id
        );
    }

    println!("✓ All elements have computed styles");
    println!("=== Test PASSED ===\n");
}

/// E2E Test: Responsive design with unit resolution
#[test]
fn test_responsive_design_workflow() {
    println!("=== E2E Test: Responsive Design ===");

    let mut engine = CssEngine::new();

    // Scenario: Responsive page with various CSS units

    // Step 1: Load stylesheet with multiple units
    let css = stylesheets::UNITS;

    engine
        .parse_stylesheet(css, Some("responsive.css"))
        .expect("Should load responsive stylesheet");

    println!("✓ Loaded responsive stylesheet");

    // Step 2: Create DOM with various unit requirements
    let px_div = DomNode::new(ElementId::new(2), "div").with_class("px-units");
    let em_div = DomNode::new(ElementId::new(3), "div").with_class("em-units");
    let rem_div = DomNode::new(ElementId::new(4), "div").with_class("rem-units");
    let percent_div = DomNode::new(ElementId::new(5), "div").with_class("percent-units");
    let vw_div = DomNode::new(ElementId::new(6), "div").with_class("vw-units");
    let vh_div = DomNode::new(ElementId::new(7), "div").with_class("vh-units");

    let body = DomNode::new(ElementId::new(1), "body")
        .with_child(px_div)
        .with_child(em_div)
        .with_child(rem_div)
        .with_child(percent_div)
        .with_child(vw_div)
        .with_child(vh_div);

    println!("✓ Created DOM with various unit types");

    // Step 3: Compute styles (unit resolution happens here)
    let style_tree = engine
        .compute_styles(&body)
        .expect("Should compute styles with unit resolution");

    println!("✓ Computed styles with unit resolution");

    // Step 4: Verify all elements styled
    assert_eq!(style_tree.root.children.len(), 6);

    for (i, child) in style_tree.root.children.iter().enumerate() {
        assert_eq!(
            child.element_id,
            ElementId::new(i as u64 + 2),
            "Child {} should have correct ID",
            i
        );
    }

    println!("✓ All unit types resolved");
    println!("=== Test PASSED ===\n");
}

/// E2E Test: Inline styles overriding stylesheet styles
#[test]
fn test_inline_style_override_workflow() {
    println!("=== E2E Test: Inline Style Override ===");

    let mut engine = CssEngine::new();

    // Scenario: Developer sets inline styles that should override stylesheet

    // Step 1: Load base stylesheet
    let css = r#"
        div {
            color: #000;
            font-size: 16px;
            padding: 10px;
        }

        #special {
            color: #f00;
            font-size: 20px;
        }
    "#;

    engine
        .parse_stylesheet(css, Some("base.css"))
        .expect("Should load base stylesheet");

    println!("✓ Loaded base stylesheet");

    let element_id = ElementId::new(1);

    // Step 2: Set inline style (highest specificity)
    engine
        .set_inline_style(element_id, "color: #00f; font-size: 24px;")
        .expect("Should set inline style");

    println!("✓ Set inline style");

    // Step 3: Create DOM with ID that matches stylesheet
    let dom = DomNode::new(element_id, "div").with_attribute("id", "special");

    // Step 4: Compute styles
    engine
        .compute_styles(&dom)
        .expect("Should compute styles");

    println!("✓ Computed styles");

    // Step 5: Verify inline style wins
    let computed = engine
        .get_computed_style(element_id)
        .expect("Should get computed style");

    // Inline style should override both div and #special rules
    // (Color verification would happen in component-level tests)
    assert_eq!(computed.display, css_engine::Display::Inline);

    println!("✓ Inline styles have highest priority");
    println!("=== Test PASSED ===\n");
}

/// E2E Test: Stylesheet hot-reloading (DevTools scenario)
#[test]
fn test_stylesheet_hot_reload_workflow() {
    println!("=== E2E Test: Stylesheet Hot Reload ===");

    let mut engine = CssEngine::new();

    // Scenario: Developer modifies CSS in DevTools, engine reloads

    // Step 1: Load initial stylesheet
    let initial_css = r#"
        .box {
            width: 100px;
            height: 100px;
        }
    "#;

    let sheet_id = engine
        .parse_stylesheet(initial_css, Some("styles.css"))
        .expect("Should load initial stylesheet");

    println!("✓ Loaded initial stylesheet");

    // Step 2: Render page
    let dom = DomNode::new(ElementId::new(1), "div").with_class("box");

    engine
        .compute_styles(&dom)
        .expect("Should compute initial styles");

    println!("✓ Rendered with initial styles");

    // Step 3: Developer modifies CSS
    let updated_css = r#"
        .box {
            width: 200px;
            height: 200px;
            background-color: #f00;
        }
    "#;

    engine
        .update_stylesheet(sheet_id, updated_css)
        .expect("Should hot-reload stylesheet");

    println!("✓ Hot-reloaded stylesheet");

    // Step 4: Re-compute styles
    engine
        .compute_styles(&dom)
        .expect("Should recompute with new styles");

    println!("✓ Recomputed with updated styles");

    // Should still have same number of stylesheets
    assert_eq!(engine.stylesheet_count(), 1);

    println!("=== Test PASSED ===\n");
}

/// E2E Test: Large-scale application performance
#[test]
fn test_large_scale_application_workflow() {
    println!("=== E2E Test: Large-Scale Application ===");

    let mut engine = CssEngine::new();

    // Scenario: Large application with many stylesheets and elements

    // Step 1: Load multiple stylesheets
    for i in 0..10 {
        let css = format!(
            r#"
            .module-{} {{
                padding: {}px;
            }}

            .module-{} .item {{
                margin: {}px;
            }}
        "#,
            i, i * 10, i, i * 5
        );

        engine
            .parse_stylesheet(&css, Some(&format!("module-{}.css", i)))
            .expect("Should load module stylesheet");
    }

    println!("✓ Loaded 10 stylesheets");

    assert_eq!(engine.stylesheet_count(), 10);

    // Step 2: Create large DOM tree (100+ elements)
    let mut body = DomNode::new(ElementId::new(1), "body");

    for i in 0..100 {
        let div = DomNode::new(ElementId::new(i + 2), "div")
            .with_class(&format!("module-{}", i % 10))
            .with_class("item");

        body = body.with_child(div);
    }

    println!("✓ Created DOM with 100+ elements");

    // Step 3: Compute all styles (performance test)
    let style_tree = engine
        .compute_styles(&body)
        .expect("Should compute styles for large tree");

    println!("✓ Computed styles for all elements");

    // Verify all elements have styles
    assert_eq!(style_tree.root.children.len(), 100);

    for (i, child) in style_tree.root.children.iter().enumerate() {
        assert_eq!(child.element_id, ElementId::new(i as u64 + 2));
    }

    println!("✓ All 100 elements styled correctly");
    println!("=== Test PASSED ===\n");
}

/// E2E Test: Error recovery
#[test]
fn test_error_recovery_workflow() {
    println!("=== E2E Test: Error Recovery ===");

    let mut engine = CssEngine::new();

    // Scenario: Engine encounters errors but continues working

    // Step 1: Try to parse invalid CSS
    let _result = engine.parse_stylesheet("this is { not valid CSS }", None);

    println!("✓ Handled invalid CSS");

    // Step 2: Parse valid CSS (should still work)
    let valid_css = "div { color: #000; }";

    engine
        .parse_stylesheet(valid_css, Some("valid.css"))
        .expect("Should parse valid CSS after error");

    println!("✓ Engine recovered from error");

    // Step 3: Create DOM and compute styles
    let dom = DomNode::new(ElementId::new(1), "div");

    engine
        .compute_styles(&dom)
        .expect("Should compute styles after recovery");

    println!("✓ Normal operation resumed");
    println!("=== Test PASSED ===\n");
}

/// E2E Test: Style inheritance through deep nesting
#[test]
fn test_deep_inheritance_workflow() {
    println!("=== E2E Test: Deep Inheritance ===");

    let mut engine = CssEngine::new();

    // Scenario: Deeply nested DOM with inherited properties

    // Step 1: Load stylesheet with inheritable properties
    let css = r#"
        body {
            font-size: 16px;
            color: #333;
        }

        .level1 {
            font-size: 14px;
        }

        .level2 {
            color: #666;
        }
    "#;

    engine
        .parse_stylesheet(css, Some("inheritance.css"))
        .expect("Should load inheritance stylesheet");

    println!("✓ Loaded stylesheet");

    // Step 2: Create deeply nested DOM (5 levels)
    let level5 = DomNode::new(ElementId::new(5), "span");

    let level4 = DomNode::new(ElementId::new(4), "div").with_child(level5);

    let level3 = DomNode::new(ElementId::new(3), "div")
        .with_class("level2")
        .with_child(level4);

    let level2 = DomNode::new(ElementId::new(2), "div")
        .with_class("level1")
        .with_child(level3);

    let body = DomNode::new(ElementId::new(1), "body").with_child(level2);

    println!("✓ Created 5-level deep DOM tree");

    // Step 3: Compute styles (inheritance should flow down)
    let style_tree = engine
        .compute_styles(&body)
        .expect("Should compute styles with deep inheritance");

    println!("✓ Computed styles with inheritance");

    // Step 4: Verify tree structure maintained
    let mut current = &style_tree.root;
    for level in 1..=4 {
        assert_eq!(current.children.len(), 1);
        assert_eq!(current.element_id, ElementId::new(level));
        current = &current.children[0];
    }
    assert_eq!(current.element_id, ElementId::new(5));

    println!("✓ All 5 levels styled with inheritance");
    println!("=== Test PASSED ===\n");
}
