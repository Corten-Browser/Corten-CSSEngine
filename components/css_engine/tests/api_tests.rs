//! Integration tests for CSS Engine public API

use css_engine::{
    Color, ComputedStyle, CssEngine, Display, DomNode, ElementId, Length, StyleInvalidation,
    StyleSheetId,
};

#[test]
fn test_engine_lifecycle() {
    // Create engine
    let mut engine = CssEngine::new();
    assert_eq!(engine.stylesheet_count(), 0);
    assert_eq!(engine.cache_size(), 0);

    // Parse stylesheet
    let css = "body { color: red; }";
    let sheet_id = engine.parse_stylesheet(css, Some("test.css")).unwrap();
    assert_eq!(sheet_id.0, 0); // First stylesheet should have ID 0
    assert_eq!(engine.stylesheet_count(), 1);

    // Create DOM
    let dom = DomNode::new(ElementId::new(1), "body");

    // Compute styles
    let style_tree = engine.compute_styles(&dom).unwrap();
    assert_eq!(style_tree.root.element_id, ElementId::new(1));
    assert!(engine.cache_size() > 0);

    // Get computed style
    let computed = engine.get_computed_style(ElementId::new(1)).unwrap();
    assert!(matches!(computed.display, Display::Inline));
}

#[test]
fn test_parse_multiple_stylesheets() {
    let mut engine = CssEngine::new();

    let id1 = engine
        .parse_stylesheet("body { margin: 0; }", None)
        .unwrap();
    let id2 = engine
        .parse_stylesheet("div { padding: 10px; }", None)
        .unwrap();
    let id3 = engine
        .parse_stylesheet("span { color: blue; }", None)
        .unwrap();

    // All IDs should be unique
    assert_ne!(id1, id2);
    assert_ne!(id2, id3);
    assert_ne!(id1, id3);

    assert_eq!(engine.stylesheet_count(), 3);
}

#[test]
fn test_parse_empty_stylesheet_fails() {
    let mut engine = CssEngine::new();
    let result = engine.parse_stylesheet("", None);
    assert!(result.is_err());
}

#[test]
fn test_inline_style_basic() {
    let mut engine = CssEngine::new();
    let element_id = ElementId::new(1);

    // Set inline style
    engine
        .set_inline_style(element_id, "color: blue; margin: 5px;")
        .unwrap();

    // Compute styles
    let dom = DomNode::new(element_id, "div");
    engine.compute_styles(&dom).unwrap();

    // Should be able to get computed style
    let style = engine.get_computed_style(element_id);
    assert!(style.is_ok());
}

#[test]
fn test_inline_style_empty_fails() {
    let mut engine = CssEngine::new();
    let result = engine.set_inline_style(ElementId::new(1), "");
    assert!(result.is_err());
}

#[test]
fn test_compute_styles_simple_tree() {
    let mut engine = CssEngine::new();

    // Parse stylesheet
    engine
        .parse_stylesheet("div { color: red; }", None)
        .unwrap();

    // Create simple DOM
    let dom = DomNode::new(ElementId::new(1), "div");

    // Compute styles
    let style_tree = engine.compute_styles(&dom).unwrap();

    assert_eq!(style_tree.root.element_id, ElementId::new(1));
    assert_eq!(style_tree.root.children.len(), 0);
}

#[test]
fn test_compute_styles_with_children() {
    let mut engine = CssEngine::new();

    // Create DOM tree with children
    let child1 = DomNode::new(ElementId::new(2), "span");
    let child2 = DomNode::new(ElementId::new(3), "span");
    let dom = DomNode::new(ElementId::new(1), "div")
        .with_child(child1)
        .with_child(child2);

    // Compute styles
    let style_tree = engine.compute_styles(&dom).unwrap();

    // Verify structure
    assert_eq!(style_tree.root.element_id, ElementId::new(1));
    assert_eq!(style_tree.root.children.len(), 2);
    assert_eq!(style_tree.root.children[0].element_id, ElementId::new(2));
    assert_eq!(style_tree.root.children[1].element_id, ElementId::new(3));
}

#[test]
fn test_compute_styles_nested_tree() {
    let mut engine = CssEngine::new();

    // Create deeply nested structure
    let level3 = DomNode::new(ElementId::new(4), "em");
    let level2 = DomNode::new(ElementId::new(3), "span").with_child(level3);
    let level1 = DomNode::new(ElementId::new(2), "p").with_child(level2);
    let root = DomNode::new(ElementId::new(1), "div").with_child(level1);

    // Compute styles
    let style_tree = engine.compute_styles(&root).unwrap();

    // Verify deep nesting
    assert_eq!(style_tree.root.element_id, ElementId::new(1));
    assert_eq!(style_tree.root.children.len(), 1);

    let level1_node = &style_tree.root.children[0];
    assert_eq!(level1_node.element_id, ElementId::new(2));
    assert_eq!(level1_node.children.len(), 1);

    let level2_node = &level1_node.children[0];
    assert_eq!(level2_node.element_id, ElementId::new(3));
    assert_eq!(level2_node.children.len(), 1);

    let level3_node = &level2_node.children[0];
    assert_eq!(level3_node.element_id, ElementId::new(4));
    assert_eq!(level3_node.children.len(), 0);
}

#[test]
fn test_get_computed_style_cached() {
    let mut engine = CssEngine::new();

    // Compute styles
    let dom = DomNode::new(ElementId::new(1), "div");
    engine.compute_styles(&dom).unwrap();

    // Get computed style (should come from cache)
    let style1 = engine.get_computed_style(ElementId::new(1)).unwrap();
    let style2 = engine.get_computed_style(ElementId::new(1)).unwrap();

    // Both should succeed
    assert!(matches!(style1.display, Display::Inline));
    assert!(matches!(style2.display, Display::Inline));
}

#[test]
fn test_get_computed_style_not_found() {
    let engine = CssEngine::new();

    // Try to get style for element that doesn't exist
    let result = engine.get_computed_style(ElementId::new(999));
    assert!(result.is_err());
}

#[test]
fn test_invalidation_attribute_change() {
    let mut engine = CssEngine::new();

    // Compute initial styles
    let dom = DomNode::new(ElementId::new(1), "div");
    engine.compute_styles(&dom).unwrap();
    assert_eq!(engine.cache_size(), 1);

    // Invalidate due to attribute change
    let invalidation = StyleInvalidation::AttributeChange {
        element_id: ElementId::new(1),
        attr: "class".to_string(),
    };
    engine.invalidate_styles(invalidation).unwrap();

    // Cache should be cleared for this element
    assert_eq!(engine.cache_size(), 0);
}

#[test]
fn test_invalidation_class_change() {
    let mut engine = CssEngine::new();

    // Compute styles
    let dom = DomNode::new(ElementId::new(1), "div");
    engine.compute_styles(&dom).unwrap();

    // Invalidate due to class change
    let invalidation = StyleInvalidation::ClassChange {
        element_id: ElementId::new(1),
        added: vec!["active".to_string()],
        removed: vec!["inactive".to_string()],
    };
    engine.invalidate_styles(invalidation).unwrap();

    // Element should be invalidated
    assert_eq!(engine.cache_size(), 0);
}

#[test]
fn test_invalidation_element_inserted() {
    let mut engine = CssEngine::new();

    let dom = DomNode::new(ElementId::new(1), "div");
    engine.compute_styles(&dom).unwrap();

    // Invalidate due to element insertion
    let invalidation = StyleInvalidation::ElementInserted {
        element_id: ElementId::new(2),
        parent_id: ElementId::new(1),
    };
    engine.invalidate_styles(invalidation).unwrap();
}

#[test]
fn test_invalidation_element_removed() {
    let mut engine = CssEngine::new();

    let child = DomNode::new(ElementId::new(2), "span");
    let dom = DomNode::new(ElementId::new(1), "div").with_child(child);
    engine.compute_styles(&dom).unwrap();

    // Invalidate due to element removal
    let invalidation = StyleInvalidation::ElementRemoved {
        element_id: ElementId::new(2),
    };
    engine.invalidate_styles(invalidation).unwrap();
}

#[test]
fn test_cache_management() {
    let mut engine = CssEngine::new();

    // Compute styles for multiple elements
    let child1 = DomNode::new(ElementId::new(2), "span");
    let child2 = DomNode::new(ElementId::new(3), "span");
    let dom = DomNode::new(ElementId::new(1), "div")
        .with_child(child1)
        .with_child(child2);

    engine.compute_styles(&dom).unwrap();
    let cache_size = engine.cache_size();
    assert!(cache_size >= 3); // At least root + 2 children

    // Clear cache
    engine.clear_cache();
    assert_eq!(engine.cache_size(), 0);
}

#[test]
fn test_dom_builder_with_classes() {
    let dom = DomNode::new(ElementId::new(1), "div")
        .with_class("container")
        .with_class("main")
        .with_class("active");

    assert_eq!(dom.classes.len(), 3);
    assert_eq!(dom.classes[0], "container");
    assert_eq!(dom.classes[1], "main");
    assert_eq!(dom.classes[2], "active");
}

#[test]
fn test_dom_builder_with_attributes() {
    let dom = DomNode::new(ElementId::new(1), "div")
        .with_attribute("id", "main")
        .with_attribute("data-test", "value")
        .with_attribute("role", "navigation");

    assert_eq!(dom.attributes.len(), 3);
    assert_eq!(dom.attributes[0].0, "id");
    assert_eq!(dom.attributes[0].1, "main");
}

#[test]
fn test_dom_builder_complex() {
    let leaf = DomNode::new(ElementId::new(3), "em").with_class("emphasis");

    let child = DomNode::new(ElementId::new(2), "span")
        .with_class("text")
        .with_attribute("data-id", "123")
        .with_child(leaf);

    let root = DomNode::new(ElementId::new(1), "div")
        .with_class("container")
        .with_class("main")
        .with_attribute("id", "root")
        .with_child(child);

    assert_eq!(root.classes.len(), 2);
    assert_eq!(root.attributes.len(), 1);
    assert_eq!(root.children.len(), 1);
    assert_eq!(root.children[0].children.len(), 1);
}

#[test]
fn test_multiple_stylesheets_and_inline_styles() {
    let mut engine = CssEngine::new();

    // Add multiple stylesheets
    engine
        .parse_stylesheet("body { margin: 0; }", None)
        .unwrap();
    engine
        .parse_stylesheet("div { padding: 10px; }", None)
        .unwrap();

    // Add inline style
    let element_id = ElementId::new(1);
    engine.set_inline_style(element_id, "color: blue;").unwrap();

    // Compute styles
    let dom = DomNode::new(element_id, "div");
    engine.compute_styles(&dom).unwrap();

    // Should have computed style
    let style = engine.get_computed_style(element_id);
    assert!(style.is_ok());
}

#[test]
fn test_recompute_after_invalidation() {
    let mut engine = CssEngine::new();
    let element_id = ElementId::new(1);

    // Initial computation
    let dom = DomNode::new(element_id, "div");
    engine.compute_styles(&dom).unwrap();

    // Verify style exists
    assert!(engine.get_computed_style(element_id).is_ok());

    // Invalidate
    let invalidation = StyleInvalidation::AttributeChange {
        element_id,
        attr: "class".to_string(),
    };
    engine.invalidate_styles(invalidation).unwrap();

    // Style should be gone from cache
    assert!(engine.get_computed_style(element_id).is_err());

    // Recompute
    engine.compute_styles(&dom).unwrap();

    // Style should be available again
    assert!(engine.get_computed_style(element_id).is_ok());
}

#[test]
fn test_stylesheet_with_source_url() {
    let mut engine = CssEngine::new();

    let id = engine
        .parse_stylesheet(
            "body { color: red; }",
            Some("https://example.com/style.css"),
        )
        .unwrap();

    assert_eq!(id.0, 0); // First stylesheet should have ID 0
    assert_eq!(engine.stylesheet_count(), 1);
}

#[test]
fn test_type_safety() {
    // Test that the type system prevents common errors
    let id1 = StyleSheetId::new(1);
    let id2 = StyleSheetId::new(1);
    assert_eq!(id1, id2);

    let elem1 = ElementId::new(100);
    let elem2 = ElementId::new(100);
    assert_eq!(elem1, elem2);

    // Different values should not be equal
    let id3 = StyleSheetId::new(2);
    assert_ne!(id1, id3);
}

#[test]
fn test_computed_style_defaults() {
    let style = ComputedStyle::default();

    assert_eq!(style.display, Display::Inline);
    assert_eq!(style.color, Color::default());
    assert_eq!(style.width, Length::Auto);
    assert_eq!(style.height, Length::Auto);
}

#[test]
fn test_color_helpers() {
    let black = Color::black();
    assert_eq!(black.r, 0);
    assert_eq!(black.g, 0);
    assert_eq!(black.b, 0);
    assert_eq!(black.a, 255);

    let white = Color::white();
    assert_eq!(white.r, 255);
    assert_eq!(white.g, 255);
    assert_eq!(white.b, 255);

    let transparent = Color::transparent();
    assert_eq!(transparent.a, 0);

    let custom = Color::rgba(128, 64, 32, 200);
    assert_eq!(custom.r, 128);
    assert_eq!(custom.g, 64);
    assert_eq!(custom.b, 32);
    assert_eq!(custom.a, 200);
}
