// TDD: Tests for combinator selector matching (GREEN phase)

use css_matcher_core::{Combinator, Component, ComplexSelector, Selector, SelectorMatcher, ElementLike};

// Extended Element with parent/sibling support
#[derive(Debug, Clone)]
pub struct Element {
    pub tag_name: String,
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub parent: Option<Box<Element>>,
    pub previous_sibling: Option<Box<Element>>,
}

impl Element {
    pub fn new(tag: &str) -> Self {
        Self {
            tag_name: tag.to_string(),
            id: None,
            classes: Vec::new(),
            parent: None,
            previous_sibling: None,
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

    pub fn with_parent(mut self, parent: Element) -> Self {
        self.parent = Some(Box::new(parent));
        self
    }

    pub fn with_previous_sibling(mut self, sibling: Element) -> Self {
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
