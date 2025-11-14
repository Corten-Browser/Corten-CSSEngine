//! CSS Engine - Main public API
//!
//! This crate provides the main CSS Engine implementation that orchestrates
//! CSS parsing, cascade resolution, selector matching, and style computation.
//!
//! # Example
//!
//! ```
//! use css_engine::{CssEngine, DomNode, ElementId};
//!
//! // Create a new CSS engine
//! let mut engine = CssEngine::new();
//!
//! // Parse a stylesheet
//! let css = "body { color: red; }";
//! let sheet_id = engine.parse_stylesheet(css, None).unwrap();
//!
//! // Create a simple DOM tree
//! let dom_root = DomNode::new(ElementId::new(1), "body");
//!
//! // Compute styles
//! let style_tree = engine.compute_styles(&dom_root).unwrap();
//! ```

// Public modules
pub mod error;
pub mod types;

// Internal modules
mod engine;
mod state;

// Re-export main types
pub use engine::CssEngine;
pub use error::{CssError, ElementId, StyleSheetId};
pub use types::{
    Color, ComputedStyle, Display, DomNode, Length, StyleInvalidation, StyleNode, StyleTree,
};

// Re-export state types for advanced usage
pub use state::EngineConfig;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_public_api_usage() {
        // This test demonstrates the public API
        let mut engine = CssEngine::new();

        // Parse stylesheet
        let css = "div { color: blue; }";
        let result = engine.parse_stylesheet(css, Some("test.css"));
        assert!(result.is_ok());

        // Create DOM
        let dom = DomNode::new(ElementId::new(1), "div");

        // Compute styles
        let style_tree = engine.compute_styles(&dom);
        assert!(style_tree.is_ok());

        // Get computed style
        let computed = engine.get_computed_style(ElementId::new(1));
        assert!(computed.is_ok());
    }

    #[test]
    fn test_inline_styles() {
        let mut engine = CssEngine::new();
        let element_id = ElementId::new(1);

        // Set inline style
        engine.set_inline_style(element_id, "color: red;").unwrap();

        // Compute styles
        let dom = DomNode::new(element_id, "div");
        engine.compute_styles(&dom).unwrap();

        // Should have computed style
        let style = engine.get_computed_style(element_id);
        assert!(style.is_ok());
    }

    #[test]
    fn test_style_invalidation() {
        let mut engine = CssEngine::new();
        let element_id = ElementId::new(1);

        // Compute initial styles
        let dom = DomNode::new(element_id, "div");
        engine.compute_styles(&dom).unwrap();

        // Invalidate
        let invalidation = StyleInvalidation::ClassChange {
            element_id,
            added: vec!["new-class".to_string()],
            removed: vec![],
        };
        let result = engine.invalidate_styles(invalidation);
        assert!(result.is_ok());
    }

    #[test]
    fn test_multiple_stylesheets() {
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

        assert_ne!(id1, id2);
        assert_ne!(id2, id3);
        assert_eq!(engine.stylesheet_count(), 3);
    }

    #[test]
    fn test_nested_dom_tree() {
        let mut engine = CssEngine::new();

        // Create nested structure
        let grandchild = DomNode::new(ElementId::new(3), "span");
        let child = DomNode::new(ElementId::new(2), "div").with_child(grandchild);
        let root = DomNode::new(ElementId::new(1), "body").with_child(child);

        // Compute styles
        let style_tree = engine.compute_styles(&root).unwrap();

        // Verify structure
        assert_eq!(style_tree.root.element_id, ElementId::new(1));
        assert_eq!(style_tree.root.children.len(), 1);
        assert_eq!(style_tree.root.children[0].element_id, ElementId::new(2));
        assert_eq!(style_tree.root.children[0].children.len(), 1);
    }

    #[test]
    fn test_dom_builder_pattern() {
        let dom = DomNode::new(ElementId::new(1), "div")
            .with_class("container")
            .with_class("main")
            .with_attribute("id", "root")
            .with_attribute("data-test", "value");

        assert_eq!(dom.classes.len(), 2);
        assert_eq!(dom.attributes.len(), 2);
        assert_eq!(dom.tag_name, "div");
    }

    #[test]
    fn test_color_types() {
        let black = Color::black();
        assert_eq!(black.r, 0);
        assert_eq!(black.g, 0);
        assert_eq!(black.b, 0);

        let white = Color::white();
        assert_eq!(white.r, 255);
        assert_eq!(white.g, 255);
        assert_eq!(white.b, 255);

        let transparent = Color::transparent();
        assert_eq!(transparent.a, 0);
    }

    #[test]
    fn test_length_types() {
        let px = Length::Px(16.0);
        assert!(matches!(px, Length::Px(16.0)));

        let em = Length::Em(1.5);
        assert!(matches!(em, Length::Em(1.5)));

        let auto = Length::Auto;
        assert!(matches!(auto, Length::Auto));
    }

    #[test]
    fn test_display_types() {
        let display = Display::Block;
        assert_eq!(display, Display::Block);

        let default_display = Display::default();
        assert_eq!(default_display, Display::Inline);
    }
}
