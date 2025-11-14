//! Test Data Generator
//!
//! Utilities for generating test data for integration and E2E tests.

use css_engine::{DomNode, ElementId};

/// Mock element for testing
#[derive(Debug, Clone)]
pub struct MockElement {
    pub id: u64,
    pub tag_name: String,
    pub id_attr: Option<String>,
    pub classes: Vec<String>,
    pub parent: Option<Box<MockElement>>,
    pub previous_sibling: Option<Box<MockElement>>,
}

impl MockElement {
    /// Create a new mock element
    pub fn new(id: u64, tag: &str) -> Self {
        Self {
            id,
            tag_name: tag.to_string(),
            id_attr: None,
            classes: Vec::new(),
            parent: None,
            previous_sibling: None,
        }
    }

    /// Add a class to the element
    pub fn with_class(mut self, class: &str) -> Self {
        self.classes.push(class.to_string());
        self
    }

    /// Add an ID attribute to the element
    pub fn with_id(mut self, id: &str) -> Self {
        self.id_attr = Some(id.to_string());
        self
    }

    /// Set the parent element
    pub fn with_parent(mut self, parent: MockElement) -> Self {
        self.parent = Some(Box::new(parent));
        self
    }

    /// Set the previous sibling
    pub fn with_prev_sibling(mut self, sibling: MockElement) -> Self {
        self.previous_sibling = Some(Box::new(sibling));
        self
    }
}

impl css_matcher_core::ElementLike for MockElement {
    fn tag_name(&self) -> &str {
        &self.tag_name
    }

    fn id(&self) -> Option<&str> {
        self.id_attr.as_deref()
    }

    fn classes(&self) -> &[String] {
        &self.classes
    }

    fn parent(&self) -> Option<&Self> {
        self.parent.as_deref()
    }

    fn previous_sibling(&self) -> Option<&Self> {
        self.previous_sibling.as_deref()
    }
}

/// Generate a simple DOM tree for testing
pub fn generate_simple_dom_tree() -> DomNode {
    // Create a simple tree:
    // body
    //   ├── div#main.container
    //   │   ├── p.text
    //   │   └── span.highlight
    //   └── footer
    //       └── p

    let p1 = DomNode::new(ElementId::new(3), "p").with_class("text");

    let span = DomNode::new(ElementId::new(4), "span").with_class("highlight");

    let main = DomNode::new(ElementId::new(2), "div")
        .with_class("container")
        .with_attribute("id", "main")
        .with_child(p1)
        .with_child(span);

    let p2 = DomNode::new(ElementId::new(6), "p");

    let footer = DomNode::new(ElementId::new(5), "footer").with_child(p2);

    DomNode::new(ElementId::new(1), "body")
        .with_child(main)
        .with_child(footer)
}

/// Generate a complex DOM tree for testing
pub fn generate_complex_dom_tree() -> DomNode {
    // Create a more complex tree:
    // body
    //   ├── header#header
    //   │   ├── nav.navbar
    //   │   │   ├── a.nav-link
    //   │   │   ├── a.nav-link.active
    //   │   │   └── a.nav-link
    //   │   └── div.logo
    //   ├── main.content
    //   │   ├── section.intro
    //   │   │   ├── h1
    //   │   │   └── p
    //   │   ├── section.features
    //   │   │   ├── div.feature
    //   │   │   │   ├── h2
    //   │   │   │   └── p
    //   │   │   ├── div.feature
    //   │   │   │   ├── h2
    //   │   │   │   └── p
    //   │   │   └── div.feature
    //   │   │       ├── h2
    //   │   │       └── p
    //   │   └── section.cta
    //   │       └── button.btn.btn-primary
    //   └── footer
    //       ├── div.footer-content
    //       │   ├── p
    //       │   └── p
    //       └── div.footer-links
    //           ├── a
    //           ├── a
    //           └── a

    let mut element_id = 1;

    // Helper to get next ID
    let mut next_id = || {
        let id = element_id;
        element_id += 1;
        ElementId::new(id)
    };

    // Header section
    let nav_link1 = DomNode::new(next_id(), "a").with_class("nav-link");
    let nav_link2 = DomNode::new(next_id(), "a")
        .with_class("nav-link")
        .with_class("active");
    let nav_link3 = DomNode::new(next_id(), "a").with_class("nav-link");

    let nav = DomNode::new(next_id(), "nav")
        .with_class("navbar")
        .with_child(nav_link1)
        .with_child(nav_link2)
        .with_child(nav_link3);

    let logo = DomNode::new(next_id(), "div").with_class("logo");

    let header = DomNode::new(next_id(), "header")
        .with_attribute("id", "header")
        .with_child(nav)
        .with_child(logo);

    // Main section - intro
    let intro_h1 = DomNode::new(next_id(), "h1");
    let intro_p = DomNode::new(next_id(), "p");

    let intro = DomNode::new(next_id(), "section")
        .with_class("intro")
        .with_child(intro_h1)
        .with_child(intro_p);

    // Main section - features
    let feature1_h2 = DomNode::new(next_id(), "h2");
    let feature1_p = DomNode::new(next_id(), "p");
    let feature1 = DomNode::new(next_id(), "div")
        .with_class("feature")
        .with_child(feature1_h2)
        .with_child(feature1_p);

    let feature2_h2 = DomNode::new(next_id(), "h2");
    let feature2_p = DomNode::new(next_id(), "p");
    let feature2 = DomNode::new(next_id(), "div")
        .with_class("feature")
        .with_child(feature2_h2)
        .with_child(feature2_p);

    let feature3_h2 = DomNode::new(next_id(), "h2");
    let feature3_p = DomNode::new(next_id(), "p");
    let feature3 = DomNode::new(next_id(), "div")
        .with_class("feature")
        .with_child(feature3_h2)
        .with_child(feature3_p);

    let features = DomNode::new(next_id(), "section")
        .with_class("features")
        .with_child(feature1)
        .with_child(feature2)
        .with_child(feature3);

    // Main section - CTA
    let cta_button = DomNode::new(next_id(), "button")
        .with_class("btn")
        .with_class("btn-primary");

    let cta = DomNode::new(next_id(), "section")
        .with_class("cta")
        .with_child(cta_button);

    // Main
    let main = DomNode::new(next_id(), "main")
        .with_class("content")
        .with_child(intro)
        .with_child(features)
        .with_child(cta);

    // Footer
    let footer_p1 = DomNode::new(next_id(), "p");
    let footer_p2 = DomNode::new(next_id(), "p");

    let footer_content = DomNode::new(next_id(), "div")
        .with_class("footer-content")
        .with_child(footer_p1)
        .with_child(footer_p2);

    let footer_link1 = DomNode::new(next_id(), "a");
    let footer_link2 = DomNode::new(next_id(), "a");
    let footer_link3 = DomNode::new(next_id(), "a");

    let footer_links = DomNode::new(next_id(), "div")
        .with_class("footer-links")
        .with_child(footer_link1)
        .with_child(footer_link2)
        .with_child(footer_link3);

    let footer = DomNode::new(next_id(), "footer")
        .with_child(footer_content)
        .with_child(footer_links);

    // Body
    DomNode::new(next_id(), "body")
        .with_child(header)
        .with_child(main)
        .with_child(footer)
}

/// Test stylesheet samples
pub mod stylesheets {
    /// Basic stylesheet with simple selectors
    pub const BASIC: &str = r#"
        body {
            color: #333;
            font-size: 16px;
        }

        .container {
            width: 100%;
            padding: 20px;
        }

        #main {
            background-color: #fff;
        }

        p {
            line-height: 1.5em;
            margin: 10px;
        }
    "#;

    /// Stylesheet with cascading rules
    pub const CASCADE: &str = r#"
        * {
            margin: 0px;
            padding: 0px;
        }

        div {
            color: #000;
        }

        .container {
            color: #333;
        }

        #main {
            color: #666;
        }

        div.container {
            font-size: 14px;
        }

        div#main.container {
            font-size: 16px;
        }
    "#;

    /// Stylesheet with inheritance
    pub const INHERITANCE: &str = r#"
        body {
            color: #000;
            font-size: 16px;
        }

        .highlight {
            color: #f00;
        }

        footer {
            font-size: 12px;
        }
    "#;

    /// Complex stylesheet with multiple features
    pub const COMPLEX: &str = r#"
        /* Reset */
        * {
            margin: 0px;
            padding: 0px;
        }

        /* Typography */
        body {
            font-size: 16px;
            color: #333;
        }

        h1 {
            font-size: 32px;
            color: #000;
        }

        h2 {
            font-size: 24px;
            color: #222;
        }

        p {
            font-size: 16px;
            line-height: 1.6em;
            margin: 10px;
        }

        /* Layout */
        .container {
            width: 100%;
            padding: 20px;
        }

        .content {
            max-width: 1200px;
        }

        /* Components */
        .navbar {
            background-color: #fff;
            padding: 10px;
        }

        .nav-link {
            color: #666;
            padding: 5px;
        }

        .nav-link.active {
            color: #000;
        }

        .btn {
            padding: 10px;
            border: 1px;
        }

        .btn-primary {
            background-color: #007bff;
            color: #fff;
        }

        /* Sections */
        #header {
            background-color: #f8f9fa;
        }

        .intro {
            padding: 40px;
        }

        .features {
            padding: 20px;
        }

        .feature {
            margin: 10px;
            padding: 15px;
        }

        footer {
            background-color: #343a40;
            color: #fff;
            padding: 20px;
        }

        .footer-content {
            margin: 10px;
        }

        .footer-links {
            margin-top: 20px;
        }
    "#;

    /// Stylesheet with various units
    pub const UNITS: &str = r#"
        body {
            font-size: 16px;
        }

        .px-units {
            width: 100px;
            height: 50px;
            margin: 10px;
        }

        .em-units {
            font-size: 1.5em;
            padding: 0.5em;
        }

        .rem-units {
            font-size: 1.2rem;
            margin: 2rem;
        }

        .percent-units {
            width: 50%;
            height: 100%;
        }

        .vw-units {
            width: 50vw;
        }

        .vh-units {
            height: 100vh;
        }

        .mixed-units {
            width: 50%;
            height: 300px;
            padding: 1em;
            margin: 10px;
        }
    "#;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_element_creation() {
        let element = MockElement::new(1, "div")
            .with_class("test")
            .with_id("my-id");

        assert_eq!(element.tag_name(), "div");
        assert_eq!(element.id(), Some("my-id"));
        assert_eq!(element.classes().len(), 1);
        assert_eq!(element.classes()[0], "test");
    }

    #[test]
    fn test_simple_dom_tree_generation() {
        let dom = generate_simple_dom_tree();

        assert_eq!(dom.tag_name, "body");
        assert_eq!(dom.children.len(), 2);
        assert_eq!(dom.children[0].tag_name, "div");
        assert_eq!(dom.children[1].tag_name, "footer");
    }

    #[test]
    fn test_complex_dom_tree_generation() {
        let dom = generate_complex_dom_tree();

        assert_eq!(dom.tag_name, "body");
        assert_eq!(dom.children.len(), 3); // header, main, footer

        // Check header
        assert_eq!(dom.children[0].tag_name, "header");

        // Check main
        assert_eq!(dom.children[1].tag_name, "main");
        assert!(dom.children[1].classes.contains(&"content".to_string()));

        // Check footer
        assert_eq!(dom.children[2].tag_name, "footer");
    }

    #[test]
    fn test_stylesheet_samples_are_valid() {
        // Just verify they're not empty
        assert!(!stylesheets::BASIC.is_empty());
        assert!(!stylesheets::CASCADE.is_empty());
        assert!(!stylesheets::INHERITANCE.is_empty());
        assert!(!stylesheets::COMPLEX.is_empty());
        assert!(!stylesheets::UNITS.is_empty());
    }
}
