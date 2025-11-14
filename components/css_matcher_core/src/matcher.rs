//! Selector matching logic
//!
//! This module implements the core selector matching algorithm.

use crate::types::{Combinator, ComplexSelector, Component, Selector};

/// A trait for elements that can be matched against selectors
///
/// This trait must be implemented by element types to enable selector matching.
pub trait ElementLike {
    /// Get the element's tag name
    fn tag_name(&self) -> &str;

    /// Get the element's ID, if it has one
    fn id(&self) -> Option<&str>;

    /// Get the element's class list
    fn classes(&self) -> &[String];

    /// Get the element's parent, if it has one
    fn parent(&self) -> Option<&Self>;

    /// Get the element's previous sibling, if it has one
    fn previous_sibling(&self) -> Option<&Self>;
}

/// The selector matcher
///
/// This struct provides methods for matching selectors against elements.
#[derive(Debug, Clone, Copy)]
pub struct SelectorMatcher;

impl SelectorMatcher {
    /// Check if an element matches a simple or compound selector
    ///
    /// A selector matches if ALL of its components match the element.
    ///
    /// # Examples
    ///
    /// ```
    /// use css_matcher_core::{Selector, Component, SelectorMatcher, ElementLike};
    ///
    /// # #[derive(Debug, Clone)]
    /// # struct Element {
    /// #     tag_name: String,
    /// #     id: Option<String>,
    /// #     classes: Vec<String>,
    /// # }
    /// # impl Element {
    /// #     fn new(tag: &str) -> Self {
    /// #         Self { tag_name: tag.to_string(), id: None, classes: Vec::new() }
    /// #     }
    /// # }
    /// # impl ElementLike for Element {
    /// #     fn tag_name(&self) -> &str { &self.tag_name }
    /// #     fn id(&self) -> Option<&str> { self.id.as_deref() }
    /// #     fn classes(&self) -> &[String] { &self.classes }
    /// #     fn parent(&self) -> Option<&Self> { None }
    /// #     fn previous_sibling(&self) -> Option<&Self> { None }
    /// # }
    /// let matcher = SelectorMatcher;
    /// let selector = Selector {
    ///     components: vec![Component::Tag("div".to_string())],
    /// };
    /// let element = Element::new("div");
    ///
    /// assert!(matcher.matches(&selector, &element));
    /// ```
    pub fn matches<E: ElementLike>(&self, selector: &Selector, element: &E) -> bool {
        // Empty selector never matches
        if selector.components.is_empty() {
            return false;
        }

        // All components must match
        selector
            .components
            .iter()
            .all(|component| self.match_component(component, element))
    }

    /// Check if a single component matches an element
    fn match_component<E: ElementLike>(&self, component: &Component, element: &E) -> bool {
        match component {
            Component::Universal => true,
            Component::Tag(tag) => {
                // Tag matching is case-insensitive for HTML
                element.tag_name().eq_ignore_ascii_case(tag)
            }
            Component::Class(class) => element.classes().iter().any(|c| c == class),
            Component::Id(id) => element.id().is_some_and(|element_id| element_id == id),
        }
    }

    /// Check if an element matches a complex selector with combinators
    ///
    /// A complex selector matches by matching the rightmost selector against the element,
    /// then checking that ancestors/siblings match the remaining selectors according to
    /// the combinators.
    ///
    /// # Examples
    ///
    /// ```
    /// use css_matcher_core::{Selector, Component, ComplexSelector, Combinator, SelectorMatcher, ElementLike};
    ///
    /// # #[derive(Debug, Clone)]
    /// # struct Element {
    /// #     tag_name: String,
    /// #     id: Option<String>,
    /// #     classes: Vec<String>,
    /// #     parent: Option<Box<Element>>,
    /// #     previous_sibling: Option<Box<Element>>,
    /// # }
    /// # impl Element {
    /// #     fn new(tag: &str) -> Self {
    /// #         Self {
    /// #             tag_name: tag.to_string(),
    /// #             id: None,
    /// #             classes: Vec::new(),
    /// #             parent: None,
    /// #             previous_sibling: None,
    /// #         }
    /// #     }
    /// #     fn with_parent(mut self, parent: Element) -> Self {
    /// #         self.parent = Some(Box::new(parent));
    /// #         self
    /// #     }
    /// # }
    /// # impl ElementLike for Element {
    /// #     fn tag_name(&self) -> &str { &self.tag_name }
    /// #     fn id(&self) -> Option<&str> { self.id.as_deref() }
    /// #     fn classes(&self) -> &[String] { &self.classes }
    /// #     fn parent(&self) -> Option<&Self> { self.parent.as_deref() }
    /// #     fn previous_sibling(&self) -> Option<&Self> { self.previous_sibling.as_deref() }
    /// # }
    /// let matcher = SelectorMatcher;
    /// let selector = ComplexSelector {
    ///     components: vec![
    ///         (Selector { components: vec![Component::Tag("div".to_string())] }, Some(Combinator::Descendant)),
    ///         (Selector { components: vec![Component::Tag("span".to_string())] }, None),
    ///     ],
    /// };
    ///
    /// let parent = Element::new("div");
    /// let element = Element::new("span").with_parent(parent);
    ///
    /// assert!(matcher.matches_complex(&selector, &element));
    /// ```
    pub fn matches_complex<E: ElementLike>(&self, complex: &ComplexSelector, element: &E) -> bool {
        // Empty complex selector never matches
        if complex.components.is_empty() {
            return false;
        }

        // Match from right to left
        self.match_complex_recursive(&complex.components, element)
    }

    /// Recursively match a complex selector from right to left
    fn match_complex_recursive<E: ElementLike>(
        &self,
        components: &[(Selector, Option<Combinator>)],
        element: &E,
    ) -> bool {
        if components.is_empty() {
            // All components matched
            return true;
        }

        let len = components.len();
        let (rightmost_selector, _rightmost_combinator) = &components[len - 1];

        // The rightmost selector must match the current element
        if !self.matches(rightmost_selector, element) {
            return false;
        }

        // If this is the only component, we're done
        if len == 1 {
            return true;
        }

        // Get the remaining components (everything to the left)
        let remaining = &components[..len - 1];
        let (_, combinator) = &remaining[remaining.len() - 1];

        // Match based on the combinator
        match combinator {
            Some(Combinator::Descendant) => {
                // Match any ancestor
                self.match_ancestor(remaining, element)
            }
            Some(Combinator::Child) => {
                // Match direct parent only
                element.parent().is_some_and(|parent| {
                    self.match_complex_recursive(remaining, parent)
                })
            }
            Some(Combinator::Adjacent) => {
                // Match previous sibling only
                element.previous_sibling().is_some_and(|sibling| {
                    self.match_complex_recursive(remaining, sibling)
                })
            }
            None => {
                // This shouldn't happen in well-formed selectors
                false
            }
        }
    }

    /// Match any ancestor (for descendant combinator)
    fn match_ancestor<E: ElementLike>(
        &self,
        components: &[(Selector, Option<Combinator>)],
        element: &E,
    ) -> bool {
        let mut current = element.parent();

        // Walk up the ancestor chain until we find a match or run out of ancestors
        while let Some(ancestor) = current {
            if self.match_complex_recursive(components, ancestor) {
                return true;
            }
            current = ancestor.parent();
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test element implementation
    #[derive(Debug, Clone)]
    struct TestElement {
        tag_name: String,
        id: Option<String>,
        classes: Vec<String>,
    }

    impl TestElement {
        fn new(tag: &str) -> Self {
            Self {
                tag_name: tag.to_string(),
                id: None,
                classes: Vec::new(),
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
    }

    impl ElementLike for TestElement {
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
    fn test_match_universal() {
        let matcher = SelectorMatcher;
        let selector = Selector {
            components: vec![Component::Universal],
        };

        assert!(matcher.matches(&selector, &TestElement::new("div")));
        assert!(matcher.matches(&selector, &TestElement::new("span")));
    }

    #[test]
    fn test_match_tag() {
        let matcher = SelectorMatcher;
        let selector = Selector {
            components: vec![Component::Tag("div".to_string())],
        };

        assert!(matcher.matches(&selector, &TestElement::new("div")));
        assert!(!matcher.matches(&selector, &TestElement::new("span")));
    }

    #[test]
    fn test_match_class() {
        let matcher = SelectorMatcher;
        let selector = Selector {
            components: vec![Component::Class("button".to_string())],
        };

        assert!(matcher.matches(&selector, &TestElement::new("div").with_class("button")));
        assert!(!matcher.matches(&selector, &TestElement::new("div")));
    }

    #[test]
    fn test_match_id() {
        let matcher = SelectorMatcher;
        let selector = Selector {
            components: vec![Component::Id("header".to_string())],
        };

        assert!(matcher.matches(&selector, &TestElement::new("div").with_id("header")));
        assert!(!matcher.matches(&selector, &TestElement::new("div")));
    }

    #[test]
    fn test_match_compound() {
        let matcher = SelectorMatcher;
        let selector = Selector {
            components: vec![
                Component::Tag("div".to_string()),
                Component::Class("button".to_string()),
            ],
        };

        assert!(matcher.matches(&selector, &TestElement::new("div").with_class("button")));
        assert!(!matcher.matches(&selector, &TestElement::new("span").with_class("button")));
        assert!(!matcher.matches(&selector, &TestElement::new("div")));
    }

    #[test]
    fn test_empty_selector() {
        let matcher = SelectorMatcher;
        let selector = Selector { components: vec![] };

        assert!(!matcher.matches(&selector, &TestElement::new("div")));
    }
}
