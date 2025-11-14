//! Selector type definitions
//!
//! This module defines the types used to represent CSS selectors.

/// A component of a selector
#[derive(Debug, Clone, PartialEq)]
pub enum Component {
    /// Universal selector (*)
    Universal,
    /// Tag selector (e.g., div, span)
    Tag(String),
    /// Class selector (e.g., .button)
    Class(String),
    /// ID selector (e.g., #header)
    Id(String),
}

/// A simple or compound selector
///
/// A selector is a sequence of components that must all match.
/// For example:
/// - `div` is a selector with one component: Tag("div")
/// - `div.button` is a selector with two components: Tag("div"), Class("button")
/// - `.primary.active` is a selector with two components: Class("primary"), Class("active")
#[derive(Debug, Clone, PartialEq)]
pub struct Selector {
    /// The components that make up this selector
    pub components: Vec<Component>,
}

impl Selector {
    /// Create a new empty selector
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }

    /// Create a selector with the given components
    pub fn with_components(components: Vec<Component>) -> Self {
        Self { components }
    }
}

impl Default for Selector {
    fn default() -> Self {
        Self::new()
    }
}

/// A combinator between selectors
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Combinator {
    /// Descendant combinator (space) - matches any descendant
    Descendant,
    /// Child combinator (>) - matches direct child only
    Child,
    /// Adjacent sibling combinator (+) - matches immediately following sibling
    Adjacent,
}

/// A complex selector with combinators
///
/// A complex selector is a sequence of selectors connected by combinators.
/// For example:
/// - `div span` is: [(Selector[Tag("div")], Descendant), (Selector[Tag("span")], None)]
/// - `div > span` is: [(Selector[Tag("div")], Child), (Selector[Tag("span")], None)]
/// - `div + span` is: [(Selector[Tag("div")], Adjacent), (Selector[Tag("span")], None)]
#[derive(Debug, Clone, PartialEq)]
pub struct ComplexSelector {
    /// Pairs of (selector, optional combinator to next selector)
    /// The last selector has None as its combinator
    pub components: Vec<(Selector, Option<Combinator>)>,
}

impl ComplexSelector {
    /// Create a new empty complex selector
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }

    /// Create a complex selector with the given components
    pub fn with_components(components: Vec<(Selector, Option<Combinator>)>) -> Self {
        Self { components }
    }
}

impl Default for ComplexSelector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_equality() {
        assert_eq!(Component::Universal, Component::Universal);
        assert_eq!(
            Component::Tag("div".to_string()),
            Component::Tag("div".to_string())
        );
        assert_ne!(
            Component::Tag("div".to_string()),
            Component::Tag("span".to_string())
        );
    }

    #[test]
    fn test_selector_creation() {
        let selector = Selector::new();
        assert_eq!(selector.components.len(), 0);

        let selector = Selector::with_components(vec![Component::Tag("div".to_string())]);
        assert_eq!(selector.components.len(), 1);
    }

    #[test]
    fn test_combinator_equality() {
        assert_eq!(Combinator::Descendant, Combinator::Descendant);
        assert_ne!(Combinator::Descendant, Combinator::Child);
    }

    #[test]
    fn test_complex_selector_creation() {
        let complex = ComplexSelector::new();
        assert_eq!(complex.components.len(), 0);

        let complex = ComplexSelector::with_components(vec![
            (
                Selector::with_components(vec![Component::Tag("div".to_string())]),
                Some(Combinator::Descendant),
            ),
            (
                Selector::with_components(vec![Component::Tag("span".to_string())]),
                None,
            ),
        ]);
        assert_eq!(complex.components.len(), 2);
    }
}
