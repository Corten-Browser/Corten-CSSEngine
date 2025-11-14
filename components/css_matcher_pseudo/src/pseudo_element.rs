//! Pseudo-element matching
//!
//! This module provides types and traits for matching CSS pseudo-elements.

use css_matcher_core::ElementLike;

/// Types of pseudo-elements supported
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PseudoElementKind {
    /// ::before - generated content before element
    Before,
    /// ::after - generated content after element
    After,
    /// ::first-line - first line of element's text
    FirstLine,
    /// ::first-letter - first letter of element's text
    FirstLetter,
    /// ::selection - portion of element selected by user
    Selection,
    /// ::marker - marker box of a list item
    Marker,
}

/// Represents a pseudo-element selector
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PseudoElement {
    /// The kind of pseudo-element
    pub kind: PseudoElementKind,
}

impl PseudoElement {
    /// Create a new pseudo-element
    pub fn new(kind: PseudoElementKind) -> Self {
        Self { kind }
    }

    /// Create a ::before pseudo-element
    pub fn before() -> Self {
        Self::new(PseudoElementKind::Before)
    }

    /// Create a ::after pseudo-element
    pub fn after() -> Self {
        Self::new(PseudoElementKind::After)
    }

    /// Create a ::first-line pseudo-element
    pub fn first_line() -> Self {
        Self::new(PseudoElementKind::FirstLine)
    }

    /// Create a ::first-letter pseudo-element
    pub fn first_letter() -> Self {
        Self::new(PseudoElementKind::FirstLetter)
    }

    /// Create a ::selection pseudo-element
    pub fn selection() -> Self {
        Self::new(PseudoElementKind::Selection)
    }

    /// Create a ::marker pseudo-element
    pub fn marker() -> Self {
        Self::new(PseudoElementKind::Marker)
    }
}

/// Style context for a pseudo-element
///
/// This represents the computed style that should be applied to a pseudo-element.
#[derive(Debug, Clone, PartialEq)]
pub struct PseudoElementStyle {
    /// The pseudo-element kind
    pub kind: PseudoElementKind,
    /// Generated content (for ::before and ::after)
    pub content: Option<String>,
    /// Whether the pseudo-element is rendered
    pub rendered: bool,
}

impl PseudoElementStyle {
    /// Create a new pseudo-element style
    pub fn new(kind: PseudoElementKind) -> Self {
        Self {
            kind,
            content: None,
            rendered: true,
        }
    }

    /// Set the generated content
    pub fn with_content(mut self, content: String) -> Self {
        self.content = Some(content);
        self
    }

    /// Set whether the pseudo-element is rendered
    pub fn with_rendered(mut self, rendered: bool) -> Self {
        self.rendered = rendered;
        self
    }
}

/// Trait for matching pseudo-elements
///
/// Implementations should provide logic for determining if an element
/// supports a particular pseudo-element and retrieving its style.
pub trait PseudoElementMatcher {
    /// Check if an element has a matching pseudo-element
    ///
    /// Returns true if the element can have the specified pseudo-element.
    ///
    /// # Examples
    ///
    /// ```
    /// use css_matcher_pseudo::{PseudoElement, PseudoElementMatcher};
    /// use css_matcher_core::ElementLike;
    ///
    /// # struct DefaultMatcher;
    /// # impl PseudoElementMatcher for DefaultMatcher {
    /// #     fn matches_pseudo_element<E: ElementLike>(&self, _element: &E, pseudo: &PseudoElement) -> bool {
    /// #         matches!(pseudo.kind, css_matcher_pseudo::PseudoElementKind::Before | css_matcher_pseudo::PseudoElementKind::After)
    /// #     }
    /// #     fn get_pseudo_element_style<E: ElementLike>(&self, _element: &E, pseudo: &PseudoElement) -> Option<css_matcher_pseudo::PseudoElementStyle> {
    /// #         None
    /// #     }
    /// # }
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
    /// let matcher = DefaultMatcher;
    /// let element = Element::new("div");
    /// let pseudo = PseudoElement::before();
    ///
    /// assert!(matcher.matches_pseudo_element(&element, &pseudo));
    /// ```
    fn matches_pseudo_element<E: ElementLike>(&self, element: &E, pseudo: &PseudoElement) -> bool;

    /// Get the style context for a pseudo-element
    ///
    /// Returns the computed style for the pseudo-element if it exists.
    ///
    /// # Examples
    ///
    /// ```
    /// use css_matcher_pseudo::{PseudoElement, PseudoElementMatcher, PseudoElementStyle, PseudoElementKind};
    /// use css_matcher_core::ElementLike;
    ///
    /// # struct DefaultMatcher;
    /// # impl PseudoElementMatcher for DefaultMatcher {
    /// #     fn matches_pseudo_element<E: ElementLike>(&self, _element: &E, _pseudo: &PseudoElement) -> bool {
    /// #         true
    /// #     }
    /// #     fn get_pseudo_element_style<E: ElementLike>(&self, _element: &E, pseudo: &PseudoElement) -> Option<css_matcher_pseudo::PseudoElementStyle> {
    /// #         Some(PseudoElementStyle::new(pseudo.kind).with_content("test".to_string()))
    /// #     }
    /// # }
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
    /// let matcher = DefaultMatcher;
    /// let element = Element::new("div");
    /// let pseudo = PseudoElement::before();
    ///
    /// let style = matcher.get_pseudo_element_style(&element, &pseudo);
    /// assert!(style.is_some());
    /// ```
    fn get_pseudo_element_style<E: ElementLike>(
        &self,
        element: &E,
        pseudo: &PseudoElement,
    ) -> Option<PseudoElementStyle>;
}

/// Default implementation of PseudoElementMatcher
///
/// This provides basic pseudo-element matching without style computation.
#[derive(Debug, Clone, Copy)]
pub struct DefaultPseudoElementMatcher;

impl PseudoElementMatcher for DefaultPseudoElementMatcher {
    fn matches_pseudo_element<E: ElementLike>(&self, element: &E, pseudo: &PseudoElement) -> bool {
        match pseudo.kind {
            PseudoElementKind::Before | PseudoElementKind::After => {
                // All elements can have ::before and ::after
                true
            }
            PseudoElementKind::FirstLine | PseudoElementKind::FirstLetter => {
                // Only block-level elements can have ::first-line and ::first-letter
                // For simplicity, we check if it's not an inline element
                let tag = element.tag_name();
                !matches!(
                    tag,
                    "span" | "a" | "em" | "strong" | "code" | "b" | "i" | "u"
                )
            }
            PseudoElementKind::Selection => {
                // All elements can have ::selection
                true
            }
            PseudoElementKind::Marker => {
                // Only list items can have ::marker
                element.tag_name() == "li"
            }
        }
    }

    fn get_pseudo_element_style<E: ElementLike>(
        &self,
        element: &E,
        pseudo: &PseudoElement,
    ) -> Option<PseudoElementStyle> {
        if self.matches_pseudo_element(element, pseudo) {
            Some(PseudoElementStyle::new(pseudo.kind))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test element implementation
    #[derive(Debug, Clone)]
    struct TestElement {
        tag_name: String,
    }

    impl TestElement {
        fn new(tag: &str) -> Self {
            Self {
                tag_name: tag.to_string(),
            }
        }
    }

    impl ElementLike for TestElement {
        fn tag_name(&self) -> &str {
            &self.tag_name
        }

        fn id(&self) -> Option<&str> {
            None
        }

        fn classes(&self) -> &[String] {
            &[]
        }

        fn parent(&self) -> Option<&Self> {
            None
        }

        fn previous_sibling(&self) -> Option<&Self> {
            None
        }
    }

    // ========================================================================
    // PseudoElement creation tests
    // ========================================================================

    #[test]
    fn test_pseudo_element_new() {
        let pe = PseudoElement::new(PseudoElementKind::Before);
        assert_eq!(pe.kind, PseudoElementKind::Before);
    }

    #[test]
    fn test_pseudo_element_before() {
        let pe = PseudoElement::before();
        assert_eq!(pe.kind, PseudoElementKind::Before);
    }

    #[test]
    fn test_pseudo_element_after() {
        let pe = PseudoElement::after();
        assert_eq!(pe.kind, PseudoElementKind::After);
    }

    #[test]
    fn test_pseudo_element_first_line() {
        let pe = PseudoElement::first_line();
        assert_eq!(pe.kind, PseudoElementKind::FirstLine);
    }

    #[test]
    fn test_pseudo_element_first_letter() {
        let pe = PseudoElement::first_letter();
        assert_eq!(pe.kind, PseudoElementKind::FirstLetter);
    }

    #[test]
    fn test_pseudo_element_selection() {
        let pe = PseudoElement::selection();
        assert_eq!(pe.kind, PseudoElementKind::Selection);
    }

    #[test]
    fn test_pseudo_element_marker() {
        let pe = PseudoElement::marker();
        assert_eq!(pe.kind, PseudoElementKind::Marker);
    }

    // ========================================================================
    // PseudoElementStyle tests
    // ========================================================================

    #[test]
    fn test_pseudo_element_style_new() {
        let style = PseudoElementStyle::new(PseudoElementKind::Before);
        assert_eq!(style.kind, PseudoElementKind::Before);
        assert!(style.content.is_none());
        assert!(style.rendered);
    }

    #[test]
    fn test_pseudo_element_style_with_content() {
        let style =
            PseudoElementStyle::new(PseudoElementKind::Before).with_content("Hello".to_string());
        assert_eq!(style.content.as_deref(), Some("Hello"));
    }

    #[test]
    fn test_pseudo_element_style_with_rendered() {
        let style = PseudoElementStyle::new(PseudoElementKind::Before).with_rendered(false);
        assert!(!style.rendered);
    }

    // ========================================================================
    // DefaultPseudoElementMatcher tests
    // ========================================================================

    #[test]
    fn test_matches_before_on_div() {
        let matcher = DefaultPseudoElementMatcher;
        let element = TestElement::new("div");
        let pseudo = PseudoElement::before();

        assert!(matcher.matches_pseudo_element(&element, &pseudo));
    }

    #[test]
    fn test_matches_after_on_span() {
        let matcher = DefaultPseudoElementMatcher;
        let element = TestElement::new("span");
        let pseudo = PseudoElement::after();

        assert!(matcher.matches_pseudo_element(&element, &pseudo));
    }

    #[test]
    fn test_matches_first_line_on_div() {
        let matcher = DefaultPseudoElementMatcher;
        let element = TestElement::new("div");
        let pseudo = PseudoElement::first_line();

        assert!(matcher.matches_pseudo_element(&element, &pseudo));
    }

    #[test]
    fn test_matches_first_line_not_on_span() {
        let matcher = DefaultPseudoElementMatcher;
        let element = TestElement::new("span");
        let pseudo = PseudoElement::first_line();

        assert!(!matcher.matches_pseudo_element(&element, &pseudo));
    }

    #[test]
    fn test_matches_first_letter_on_p() {
        let matcher = DefaultPseudoElementMatcher;
        let element = TestElement::new("p");
        let pseudo = PseudoElement::first_letter();

        assert!(matcher.matches_pseudo_element(&element, &pseudo));
    }

    #[test]
    fn test_matches_first_letter_not_on_inline() {
        let matcher = DefaultPseudoElementMatcher;
        let element = TestElement::new("em");
        let pseudo = PseudoElement::first_letter();

        assert!(!matcher.matches_pseudo_element(&element, &pseudo));
    }

    #[test]
    fn test_matches_selection_on_any() {
        let matcher = DefaultPseudoElementMatcher;
        let element = TestElement::new("div");
        let pseudo = PseudoElement::selection();

        assert!(matcher.matches_pseudo_element(&element, &pseudo));
    }

    #[test]
    fn test_matches_marker_on_li() {
        let matcher = DefaultPseudoElementMatcher;
        let element = TestElement::new("li");
        let pseudo = PseudoElement::marker();

        assert!(matcher.matches_pseudo_element(&element, &pseudo));
    }

    #[test]
    fn test_matches_marker_not_on_div() {
        let matcher = DefaultPseudoElementMatcher;
        let element = TestElement::new("div");
        let pseudo = PseudoElement::marker();

        assert!(!matcher.matches_pseudo_element(&element, &pseudo));
    }

    #[test]
    fn test_get_pseudo_element_style_when_matches() {
        let matcher = DefaultPseudoElementMatcher;
        let element = TestElement::new("div");
        let pseudo = PseudoElement::before();

        let style = matcher.get_pseudo_element_style(&element, &pseudo);
        assert!(style.is_some());
        assert_eq!(style.unwrap().kind, PseudoElementKind::Before);
    }

    #[test]
    fn test_get_pseudo_element_style_when_no_match() {
        let matcher = DefaultPseudoElementMatcher;
        let element = TestElement::new("div");
        let pseudo = PseudoElement::marker();

        let style = matcher.get_pseudo_element_style(&element, &pseudo);
        assert!(style.is_none());
    }
}
