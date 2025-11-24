//! Testing Mocks Library for Corten CSS Engine
//!
//! This library provides mock implementations for testing CSS engine components
//! without requiring real browser DOM or font systems.
//!
//! # Features
//!
//! - **Mock DOM**: `MockDomTree` and `MockDomNode` for testing CSS selectors and
//!   style computation without a real DOM.
//! - **Mock Fonts**: `MockFontSystem` for testing text layout and font metrics
//!   without real font files.
//!
//! # Quick Start
//!
//! ## Mock DOM Usage
//!
//! ```rust
//! use testing_mocks::mock_dom::{MockDomTree, MockDomNode};
//!
//! // Create a simple DOM tree
//! let mut tree = MockDomTree::new();
//! let root = MockDomNode::builder(1, "html")
//!     .build();
//! tree.add_node(root);
//!
//! let body = MockDomNode::builder(2, "body")
//!     .class("main")
//!     .parent(1)
//!     .build();
//! tree.add_child(1, body);
//!
//! // Navigate and query the tree
//! let body_node = tree.get_node(2).unwrap();
//! assert!(body_node.has_class("main"));
//!
//! // Traverse the tree
//! for node in tree.traverse_depth_first() {
//!     println!("Visiting: {}", node.tag_name);
//! }
//! ```
//!
//! ## Mock Font System Usage
//!
//! ```rust
//! use testing_mocks::mock_fonts::{MockFontSystem, MockFontMetrics};
//!
//! // Use pre-configured common fonts
//! let fonts = MockFontSystem::with_defaults();
//!
//! // Get font metrics
//! let arial = fonts.get_metrics("arial").unwrap();
//! println!("Arial ascent: {}", arial.ascent);
//!
//! // Measure text
//! let width = fonts.measure_text("arial", "Hello World", 16.0);
//! println!("Text width: {}px", width);
//!
//! // Or create custom fonts
//! let mut custom_fonts = MockFontSystem::new();
//! custom_fonts.add_font("my-font", MockFontMetrics::default_for_size(16.0));
//! ```

pub mod mock_dom;
pub mod mock_fonts;

// Re-export commonly used types at crate root
pub use mock_dom::{MockDomNode, MockDomNodeBuilder, MockDomTree};
pub use mock_fonts::{
    FontKey, FontStyle, FontWeight, GlyphMetrics, MockFontMetrics, MockFontSystem,
    TextLineMetrics,
};

#[cfg(test)]
mod integration_tests {
    use super::*;

    /// Integration test: Build a realistic DOM tree and query it
    #[test]
    fn test_realistic_dom_structure() {
        let mut tree = MockDomTree::new();

        // HTML document structure
        let html = MockDomNode::builder(1, "html")
            .attribute("lang", "en")
            .build();
        tree.add_node(html);

        // Head section
        let head = MockDomNode::new(2, "head");
        tree.add_child(1, head);

        let title = MockDomNode::new(3, "title");
        tree.add_child(2, title);

        // Body section
        let body = MockDomNode::builder(4, "body")
            .class("dark-mode")
            .build();
        tree.add_child(1, body);

        // Header
        let header = MockDomNode::builder(5, "header")
            .id("main-header")
            .class("sticky")
            .build();
        tree.add_child(4, header);

        // Nav
        let nav = MockDomNode::builder(6, "nav")
            .class("primary-nav")
            .build();
        tree.add_child(5, nav);

        // Main content
        let main = MockDomNode::builder(7, "main")
            .id("content")
            .class("container")
            .build();
        tree.add_child(4, main);

        // Article
        let article = MockDomNode::builder(8, "article")
            .class("post")
            .class("featured")
            .build();
        tree.add_child(7, article);

        // Footer
        let footer = MockDomNode::builder(9, "footer")
            .id("main-footer")
            .build();
        tree.add_child(4, footer);

        // Verify structure
        assert_eq!(tree.len(), 9);

        // Find by class
        let dark_mode_elements = tree.find_by_class("dark-mode");
        assert_eq!(dark_mode_elements.len(), 1);
        assert_eq!(dark_mode_elements[0].tag_name, "body");

        // Find by ID
        let header = tree.find_by_id("main-header").unwrap();
        assert!(header.has_class("sticky"));

        // Get ancestors
        let article_ancestors = tree.get_ancestors(8);
        let ancestor_tags: Vec<_> = article_ancestors
            .iter()
            .map(|n| n.tag_name.as_str())
            .collect();
        assert_eq!(ancestor_tags, vec!["main", "body", "html"]);

        // Get descendants of body
        let body_descendants = tree.get_descendants(4);
        assert_eq!(body_descendants.len(), 5); // header, nav, main, article, footer

        // Depth check
        assert_eq!(tree.get_depth(1), Some(0)); // html
        assert_eq!(tree.get_depth(8), Some(3)); // article
    }

    /// Integration test: Simulate CSS selector matching context
    #[test]
    fn test_selector_matching_context() {
        let mut tree = MockDomTree::new();

        // Create elements for selector matching
        tree.add_node(MockDomNode::builder(1, "div").id("container").build());

        tree.add_child(
            1,
            MockDomNode::builder(2, "ul")
                .class("menu")
                .class("horizontal")
                .build(),
        );

        for i in 0..3 {
            tree.add_child(
                2,
                MockDomNode::builder(3 + i as u64, "li")
                    .class("menu-item")
                    .attribute("data-index", &i.to_string())
                    .build(),
            );
        }

        // Simulate selector: .menu > li
        let menu = tree.find_by_class("menu");
        assert_eq!(menu.len(), 1);

        let menu_children = tree.get_children(menu[0].id);
        assert_eq!(menu_children.len(), 3);
        assert!(menu_children.iter().all(|n| n.tag_name == "li"));

        // Simulate selector: li[data-index="1"]
        let li_with_index_1 = tree
            .find_by_tag("li")
            .into_iter()
            .find(|n| n.get_attribute("data-index") == Some(&"1".to_string()));
        assert!(li_with_index_1.is_some());

        // Simulate :first-child (first li)
        let first_li = menu_children.first().unwrap();
        let siblings = tree.get_siblings(first_li.id);
        assert_eq!(siblings.len(), 2);
    }

    /// Integration test: Use fonts for text layout calculations
    #[test]
    fn test_text_layout_with_fonts() {
        let fonts = MockFontSystem::with_defaults();

        // Simulate laying out a paragraph
        let text = "The quick brown fox jumps over the lazy dog.";
        let font_size = 16.0;
        let font_family = "arial";

        let line_metrics = fonts.measure_text_line(font_family, text, font_size);

        // Verify we get reasonable metrics
        assert!(line_metrics.width > 0.0);
        assert!(line_metrics.height > font_size);
        assert!(line_metrics.ascent > 0.0);
        assert!(line_metrics.descent > 0.0);

        // Simulate multi-line layout
        let lines = vec![
            "First line of text",
            "Second line that is longer than the first",
            "Third",
        ];

        let mut total_height = 0.0;
        let mut max_width = 0.0f32;

        for line in &lines {
            let metrics = fonts.measure_text_line(font_family, line, font_size);
            total_height += metrics.height;
            max_width = max_width.max(metrics.width);
        }

        assert!(total_height > line_metrics.height);
        assert!(max_width > fonts.measure_text(font_family, "Third", font_size));
    }

    /// Integration test: Font fallback chain
    #[test]
    fn test_font_fallback_chain() {
        let fonts = MockFontSystem::with_defaults();

        // CSS font stack: "Comic Sans MS", "Trebuchet MS", Arial, sans-serif
        let font_stack = vec!["comic sans ms", "trebuchet ms", "arial", "sans-serif"];

        let mut resolved_font = None;
        for family in &font_stack {
            if fonts.has_font(family) {
                resolved_font = Some(*family);
                break;
            }
        }

        // Should resolve to Arial (Comic Sans and Trebuchet not in defaults)
        assert_eq!(resolved_font, Some("arial"));

        // Verify we can get metrics for resolved font
        let metrics = fonts.get_metrics(resolved_font.unwrap()).unwrap();
        assert!(metrics.ascent > 0.0);
    }

    /// Integration test: DOM tree modifications during style computation
    #[test]
    fn test_dom_modification_during_style_computation() {
        let mut tree = MockDomTree::new();

        // Initial structure
        tree.add_node(MockDomNode::new(1, "div"));
        tree.add_child(1, MockDomNode::new(2, "span"));

        // Simulate adding a class during hover
        if let Some(span) = tree.get_node_mut(2) {
            span.add_class("hover");
        }

        assert!(tree.get_node(2).unwrap().has_class("hover"));

        // Simulate removing the class
        if let Some(span) = tree.get_node_mut(2) {
            span.remove_class("hover");
        }

        assert!(!tree.get_node(2).unwrap().has_class("hover"));

        // Simulate adding a new child (dynamic content)
        tree.add_child(1, MockDomNode::new(3, "p"));
        assert_eq!(tree.len(), 3);

        // Simulate removing content
        tree.remove_node(3);
        assert_eq!(tree.len(), 2);
    }

    /// Integration test: Complex font variant selection
    #[test]
    fn test_complex_font_variant_selection() {
        use mock_fonts::{FontStyle, FontWeight};

        let mut fonts = MockFontSystem::new();

        // Add font family with multiple variants
        let regular = MockFontMetrics::default();
        let bold = MockFontMetrics {
            average_char_width: 8.8,
            ..regular
        };
        let italic = MockFontMetrics {
            x_height: 8.2,
            ..regular
        };
        let bold_italic = MockFontMetrics {
            average_char_width: 8.8,
            x_height: 8.2,
            ..regular
        };

        fonts.add_font("roboto", regular);
        fonts.add_font_variant("roboto", FontWeight::Bold, FontStyle::Normal, bold);
        fonts.add_font_variant("roboto", FontWeight::Normal, FontStyle::Italic, italic);
        fonts.add_font_variant("roboto", FontWeight::Bold, FontStyle::Italic, bold_italic);

        // Request specific variants
        let reg = fonts.get_metrics_with_fallback("roboto", FontWeight::Normal, FontStyle::Normal);
        assert!((reg.average_char_width - 8.0).abs() < 0.1);

        let b = fonts.get_metrics_with_fallback("roboto", FontWeight::Bold, FontStyle::Normal);
        assert!((b.average_char_width - 8.8).abs() < 0.1);

        let bi = fonts.get_metrics_with_fallback("roboto", FontWeight::Bold, FontStyle::Italic);
        assert!((bi.average_char_width - 8.8).abs() < 0.1);
        assert!((bi.x_height - 8.2).abs() < 0.1);

        // Request non-existent weight - should fall back to regular
        let light = fonts.get_metrics_with_fallback("roboto", FontWeight::Light, FontStyle::Normal);
        assert!((light.average_char_width - 8.0).abs() < 0.1);
    }
}
