// TDD: Tests for simple selector matching (GREEN phase)

use css_matcher_core::{Component, Selector, SelectorMatcher, ElementLike};

// Helper to create mock elements for testing
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
fn test_universal_selector_matches_any_element() {
    let matcher = SelectorMatcher;
    let selector = Selector {
        components: vec![Component::Universal],
    };

    let div = Element::new("div");
    let span = Element::new("span");
    let p = Element::new("p");

    assert!(matcher.matches(&selector, &div));
    assert!(matcher.matches(&selector, &span));
    assert!(matcher.matches(&selector, &p));
}

#[test]
fn test_tag_selector_matches_correct_tag() {
    let matcher = SelectorMatcher;
    let selector = Selector {
        components: vec![Component::Tag("div".to_string())],
    };

    let div = Element::new("div");
    let span = Element::new("span");

    assert!(matcher.matches(&selector, &div));
    assert!(!matcher.matches(&selector, &span));
}

#[test]
fn test_tag_selector_case_insensitive() {
    let matcher = SelectorMatcher;
    let selector = Selector {
        components: vec![Component::Tag("DIV".to_string())],
    };

    let div = Element::new("div");

    assert!(matcher.matches(&selector, &div));
}

#[test]
fn test_class_selector_matches_element_with_class() {
    let matcher = SelectorMatcher;
    let selector = Selector {
        components: vec![Component::Class("button".to_string())],
    };

    let with_class = Element::new("div").with_class("button");
    let without_class = Element::new("div");
    let multiple_classes = Element::new("div").with_classes(&["btn", "button", "primary"]);

    assert!(matcher.matches(&selector, &with_class));
    assert!(!matcher.matches(&selector, &without_class));
    assert!(matcher.matches(&selector, &multiple_classes));
}

#[test]
fn test_id_selector_matches_element_with_id() {
    let matcher = SelectorMatcher;
    let selector = Selector {
        components: vec![Component::Id("header".to_string())],
    };

    let with_id = Element::new("div").with_id("header");
    let without_id = Element::new("div");
    let different_id = Element::new("div").with_id("footer");

    assert!(matcher.matches(&selector, &with_id));
    assert!(!matcher.matches(&selector, &without_id));
    assert!(!matcher.matches(&selector, &different_id));
}

#[test]
fn test_empty_selector_never_matches() {
    let matcher = SelectorMatcher;
    let selector = Selector {
        components: vec![],
    };

    let element = Element::new("div");

    assert!(!matcher.matches(&selector, &element));
}
