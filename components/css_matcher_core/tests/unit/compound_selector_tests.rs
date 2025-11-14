// TDD: Tests for compound selector matching (GREEN phase)

use css_matcher_core::{Component, Selector, SelectorMatcher, ElementLike};

// Re-use Element from simple_selector_tests
#[derive(Debug, Clone)]
pub struct Element {
    pub tag_name: String,
    pub id: Option<String>,
    pub classes: Vec<String>,
}

impl Element {
    pub fn new(tag: &str) -> Self {
        Self {
            tag_name: tag.to_string(),
            id: None,
            classes: Vec::new(),
        }
    }

    pub fn with_id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    pub fn with_class(mut self, class: &str) -> Self {
        self.classes.push(class.to_string());
        self
    }

    pub fn with_classes(mut self, classes: &[&str]) -> Self {
        self.classes.extend(classes.iter().map(|s| s.to_string()));
        self
    }
}

impl ElementLike for Element {
    fn tag_name(&self) -> &str {
        &self.tag_name
    }

    fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    fn classes(&self) -> &[String] {
        &self.classes
    }

    fn parent(&self) -> Option<&Self> {
        None
    }

    fn previous_sibling(&self) -> Option<&Self> {
        None
    }
}

#[test]
fn test_compound_selector_tag_and_class() {
    // Selector: div.button
    let matcher = SelectorMatcher;
    let selector = Selector {
        components: vec![
            Component::Tag("div".to_string()),
            Component::Class("button".to_string()),
        ],
    };

    let matches = Element::new("div").with_class("button");
    let wrong_tag = Element::new("span").with_class("button");
    let no_class = Element::new("div");

    assert!(matcher.matches(&selector, &matches));
    assert!(!matcher.matches(&selector, &wrong_tag));
    assert!(!matcher.matches(&selector, &no_class));
}

#[test]
fn test_compound_selector_tag_and_id() {
    // Selector: div#header
    let matcher = SelectorMatcher;
    let selector = Selector {
        components: vec![
            Component::Tag("div".to_string()),
            Component::Id("header".to_string()),
        ],
    };

    let matches = Element::new("div").with_id("header");
    let wrong_tag = Element::new("span").with_id("header");
    let no_id = Element::new("div");

    assert!(matcher.matches(&selector, &matches));
    assert!(!matcher.matches(&selector, &wrong_tag));
    assert!(!matcher.matches(&selector, &no_id));
}

#[test]
fn test_compound_selector_multiple_classes() {
    // Selector: .button.primary
    let matcher = SelectorMatcher;
    let selector = Selector {
        components: vec![
            Component::Class("button".to_string()),
            Component::Class("primary".to_string()),
        ],
    };

    let matches = Element::new("div").with_classes(&["button", "primary"]);
    let partial_match = Element::new("div").with_class("button");
    let no_match = Element::new("div");

    assert!(matcher.matches(&selector, &matches));
    assert!(!matcher.matches(&selector, &partial_match));
    assert!(!matcher.matches(&selector, &no_match));
}

#[test]
fn test_compound_selector_tag_id_and_class() {
    // Selector: div#main.container
    let matcher = SelectorMatcher;
    let selector = Selector {
        components: vec![
            Component::Tag("div".to_string()),
            Component::Id("main".to_string()),
            Component::Class("container".to_string()),
        ],
    };

    let matches = Element::new("div")
        .with_id("main")
        .with_class("container");
    let missing_class = Element::new("div").with_id("main");
    let wrong_tag = Element::new("span")
        .with_id("main")
        .with_class("container");

    assert!(matcher.matches(&selector, &matches));
    assert!(!matcher.matches(&selector, &missing_class));
    assert!(!matcher.matches(&selector, &wrong_tag));
}

#[test]
fn test_compound_selector_with_extra_classes() {
    // Selector: .button
    let matcher = SelectorMatcher;
    let selector = Selector {
        components: vec![Component::Class("button".to_string())],
    };

    // Element with additional classes should still match
    let element = Element::new("div").with_classes(&["btn", "button", "primary", "large"]);

    assert!(matcher.matches(&selector, &element));
}
