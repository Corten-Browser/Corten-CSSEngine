// Integration tests for css_matcher_core

use css_matcher_core::{
    Combinator, ComplexSelector, Component, ElementLike, Selector, SelectorMatcher,
};

// Test element with parent and sibling support
#[derive(Debug, Clone)]
struct Element {
    tag_name: String,
    id: Option<String>,
    classes: Vec<String>,
    parent: Option<Box<Element>>,
    previous_sibling: Option<Box<Element>>,
}

impl Element {
    fn new(tag: &str) -> Self {
        Self {
            tag_name: tag.to_string(),
            id: None,
            classes: Vec::new(),
            parent: None,
            previous_sibling: None,
        }
    }

    fn with_id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    fn with_class(mut self, class: &str) -> Self {
        self.classes.push(class.to_string());
        self
    }

    fn with_classes(mut self, classes: &[&str]) -> Self {
        self.classes.extend(classes.iter().map(|s| s.to_string()));
        self
    }

    fn with_parent(mut self, parent: Element) -> Self {
        self.parent = Some(Box::new(parent));
        self
    }

    fn with_previous_sibling(mut self, sibling: Element) -> Self {
        self.previous_sibling = Some(Box::new(sibling));
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
        self.parent.as_deref()
    }

    fn previous_sibling(&self) -> Option<&Self> {
        self.previous_sibling.as_deref()
    }
}

// Simple Selector Tests

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
    let selector = Selector { components: vec![] };

    let element = Element::new("div");

    assert!(!matcher.matches(&selector, &element));
}

// Compound Selector Tests

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

    let matches = Element::new("div").with_id("main").with_class("container");
    let missing_class = Element::new("div").with_id("main");
    let wrong_tag = Element::new("span").with_id("main").with_class("container");

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

// Combinator Tests

#[test]
fn test_descendant_combinator_direct_child() {
    // Selector: div span (descendant)
    let matcher = SelectorMatcher;
    let selector = ComplexSelector {
        components: vec![
            (
                Selector {
                    components: vec![Component::Tag("div".to_string())],
                },
                Some(Combinator::Descendant),
            ),
            (
                Selector {
                    components: vec![Component::Tag("span".to_string())],
                },
                None,
            ),
        ],
    };

    let parent = Element::new("div");
    let span = Element::new("span").with_parent(parent.clone());

    assert!(matcher.matches_complex(&selector, &span));
}

#[test]
fn test_descendant_combinator_deep_nesting() {
    // Selector: div span (descendant, multiple levels)
    let matcher = SelectorMatcher;
    let selector = ComplexSelector {
        components: vec![
            (
                Selector {
                    components: vec![Component::Tag("div".to_string())],
                },
                Some(Combinator::Descendant),
            ),
            (
                Selector {
                    components: vec![Component::Tag("span".to_string())],
                },
                None,
            ),
        ],
    };

    // div > p > span
    let div = Element::new("div");
    let p = Element::new("p").with_parent(div.clone());
    let span = Element::new("span").with_parent(p);

    assert!(matcher.matches_complex(&selector, &span));
}

#[test]
fn test_descendant_combinator_no_match() {
    // Selector: div span
    let matcher = SelectorMatcher;
    let selector = ComplexSelector {
        components: vec![
            (
                Selector {
                    components: vec![Component::Tag("div".to_string())],
                },
                Some(Combinator::Descendant),
            ),
            (
                Selector {
                    components: vec![Component::Tag("span".to_string())],
                },
                None,
            ),
        ],
    };

    // span with wrong parent
    let parent = Element::new("p");
    let span = Element::new("span").with_parent(parent);

    assert!(!matcher.matches_complex(&selector, &span));
}

#[test]
fn test_child_combinator_direct_child() {
    // Selector: div > span (child)
    let matcher = SelectorMatcher;
    let selector = ComplexSelector {
        components: vec![
            (
                Selector {
                    components: vec![Component::Tag("div".to_string())],
                },
                Some(Combinator::Child),
            ),
            (
                Selector {
                    components: vec![Component::Tag("span".to_string())],
                },
                None,
            ),
        ],
    };

    let parent = Element::new("div");
    let span = Element::new("span").with_parent(parent);

    assert!(matcher.matches_complex(&selector, &span));
}

#[test]
fn test_child_combinator_rejects_grandchild() {
    // Selector: div > span (child, not descendant)
    let matcher = SelectorMatcher;
    let selector = ComplexSelector {
        components: vec![
            (
                Selector {
                    components: vec![Component::Tag("div".to_string())],
                },
                Some(Combinator::Child),
            ),
            (
                Selector {
                    components: vec![Component::Tag("span".to_string())],
                },
                None,
            ),
        ],
    };

    // div > p > span (span is grandchild, not child)
    let div = Element::new("div");
    let p = Element::new("p").with_parent(div);
    let span = Element::new("span").with_parent(p);

    assert!(!matcher.matches_complex(&selector, &span));
}

#[test]
fn test_adjacent_sibling_combinator() {
    // Selector: div + span (adjacent sibling)
    let matcher = SelectorMatcher;
    let selector = ComplexSelector {
        components: vec![
            (
                Selector {
                    components: vec![Component::Tag("div".to_string())],
                },
                Some(Combinator::Adjacent),
            ),
            (
                Selector {
                    components: vec![Component::Tag("span".to_string())],
                },
                None,
            ),
        ],
    };

    let div = Element::new("div");
    let span = Element::new("span").with_previous_sibling(div);

    assert!(matcher.matches_complex(&selector, &span));
}

#[test]
fn test_adjacent_sibling_combinator_no_match() {
    // Selector: div + span
    let matcher = SelectorMatcher;
    let selector = ComplexSelector {
        components: vec![
            (
                Selector {
                    components: vec![Component::Tag("div".to_string())],
                },
                Some(Combinator::Adjacent),
            ),
            (
                Selector {
                    components: vec![Component::Tag("span".to_string())],
                },
                None,
            ),
        ],
    };

    // Wrong previous sibling
    let p = Element::new("p");
    let span = Element::new("span").with_previous_sibling(p);

    assert!(!matcher.matches_complex(&selector, &span));
}

#[test]
fn test_complex_selector_multiple_combinators() {
    // Selector: div.container > p + span.highlight
    let matcher = SelectorMatcher;
    let selector = ComplexSelector {
        components: vec![
            (
                Selector {
                    components: vec![
                        Component::Tag("div".to_string()),
                        Component::Class("container".to_string()),
                    ],
                },
                Some(Combinator::Child),
            ),
            (
                Selector {
                    components: vec![Component::Tag("p".to_string())],
                },
                Some(Combinator::Adjacent),
            ),
            (
                Selector {
                    components: vec![
                        Component::Tag("span".to_string()),
                        Component::Class("highlight".to_string()),
                    ],
                },
                None,
            ),
        ],
    };

    // Build the structure: div.container > (p + span.highlight)
    let div = Element::new("div").with_class("container");
    let p = Element::new("p").with_parent(div.clone());
    let span = Element::new("span")
        .with_class("highlight")
        .with_parent(div)
        .with_previous_sibling(p);

    assert!(matcher.matches_complex(&selector, &span));
}
