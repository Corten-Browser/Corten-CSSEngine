//! Pseudo-class matching
//!
//! This module provides types and functions for matching CSS pseudo-classes.

use css_matcher_core::ElementLike;

/// Types of pseudo-classes supported
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PseudoClassKind {
    /// :hover - element is being hovered
    Hover,
    /// :active - element is being activated
    Active,
    /// :focus - element has focus
    Focus,
    /// :visited - link has been visited
    Visited,
    /// :link - unvisited link
    Link,
    /// :first-child - element is first child of its parent
    FirstChild,
    /// :last-child - element is last child of its parent
    LastChild,
    /// :nth-child(an+b) - element matches nth-child pattern
    NthChild,
    /// :nth-last-child(an+b) - element matches nth-last-child pattern
    NthLastChild,
    /// :nth-of-type(an+b) - element matches nth-of-type pattern
    NthOfType,
    /// :nth-last-of-type(an+b) - element matches nth-last-of-type pattern
    NthLastOfType,
    /// :only-child - element is only child of its parent
    OnlyChild,
    /// :empty - element has no children
    Empty,
    /// :root - element is the document root
    Root,
    /// :target - element is the target of the URL fragment
    Target,
    /// :enabled - form element is enabled
    Enabled,
    /// :disabled - form element is disabled
    Disabled,
    /// :checked - form element is checked
    Checked,
}

/// Represents a pseudo-class selector
#[derive(Debug, Clone, PartialEq)]
pub struct PseudoClass {
    /// The kind of pseudo-class
    pub kind: PseudoClassKind,
    /// Optional argument (for nth-child, etc.)
    pub argument: Option<String>,
}

impl PseudoClass {
    /// Create a new pseudo-class without an argument
    pub fn new(kind: PseudoClassKind) -> Self {
        Self {
            kind,
            argument: None,
        }
    }

    /// Create a new pseudo-class with an argument
    pub fn with_argument(kind: PseudoClassKind, argument: String) -> Self {
        Self {
            kind,
            argument: Some(argument),
        }
    }
}

/// Match context provides state information for pseudo-class matching
///
/// This includes dynamic state (hover, focus) and document state
#[derive(Debug, Clone, Default)]
pub struct MatchContext {
    /// Elements currently being hovered
    pub hovered_elements: Vec<String>, // Element IDs
    /// Element currently focused
    pub focused_element: Option<String>, // Element ID
    /// Elements currently active
    pub active_elements: Vec<String>, // Element IDs
    /// Visited links
    pub visited_links: Vec<String>, // URLs
    /// Root element ID
    pub root_element: Option<String>,
    /// Target element ID (from URL fragment)
    pub target_element: Option<String>,
}

impl MatchContext {
    /// Create a new empty match context
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if element is hovered
    pub fn is_hovered(&self, element_id: &str) -> bool {
        self.hovered_elements.iter().any(|id| id == element_id)
    }

    /// Check if element is focused
    pub fn is_focused(&self, element_id: &str) -> bool {
        self.focused_element
            .as_ref()
            .is_some_and(|id| id == element_id)
    }

    /// Check if element is active
    pub fn is_active(&self, element_id: &str) -> bool {
        self.active_elements.iter().any(|id| id == element_id)
    }

    /// Check if link URL has been visited
    pub fn is_visited(&self, url: &str) -> bool {
        self.visited_links.iter().any(|link| link == url)
    }

    /// Check if element is root
    pub fn is_root(&self, element_id: &str) -> bool {
        self.root_element
            .as_ref()
            .is_some_and(|id| id == element_id)
    }

    /// Check if element is target
    pub fn is_target(&self, element_id: &str) -> bool {
        self.target_element
            .as_ref()
            .is_some_and(|id| id == element_id)
    }
}

/// Trait extension for ElementLike to support pseudo-class matching
pub trait ElementLikeExt {
    /// Get element ID (for matching with context)
    fn element_id(&self) -> Option<&str> {
        None
    }

    /// Get sibling count (number of siblings including self)
    fn sibling_count(&self) -> usize {
        1
    }

    /// Get position among siblings (1-based)
    fn sibling_position(&self) -> usize {
        1
    }

    /// Get position among siblings of same type (1-based)
    fn sibling_position_of_type(&self) -> usize {
        1
    }

    /// Get count of siblings of same type (including self)
    fn sibling_count_of_type(&self) -> usize {
        1
    }

    /// Check if element has children
    fn has_children(&self) -> bool {
        false
    }

    /// Check if element is enabled (for form elements)
    fn is_enabled(&self) -> bool {
        true
    }

    /// Check if element is checked (for form elements)
    fn is_checked(&self) -> bool {
        false
    }

    /// Get link URL (for :link and :visited)
    fn link_url(&self) -> Option<&str> {
        None
    }
}

/// Evaluate if an element matches a pseudo-class
///
/// # Examples
///
/// ```
/// use css_matcher_pseudo::{PseudoClass, PseudoClassKind, MatchContext, evaluate_pseudo_class, ElementLikeExt};
/// use css_matcher_core::ElementLike;
///
/// # #[derive(Debug, Clone)]
/// # struct Element {
/// #     tag_name: String,
/// #     id: Option<String>,
/// #     classes: Vec<String>,
/// # }
/// # impl Element {
/// #     fn new(tag: &str) -> Self {
/// #         Self { tag_name: tag.to_string(), id: Some("test".to_string()), classes: Vec::new() }
/// #     }
/// # }
/// # impl ElementLike for Element {
/// #     fn tag_name(&self) -> &str { &self.tag_name }
/// #     fn id(&self) -> Option<&str> { self.id.as_deref() }
/// #     fn classes(&self) -> &[String] { &self.classes }
/// #     fn parent(&self) -> Option<&Self> { None }
/// #     fn previous_sibling(&self) -> Option<&Self> { None }
/// # }
/// # impl ElementLikeExt for Element {}
/// let element = Element::new("div");
/// let pseudo = PseudoClass::new(PseudoClassKind::FirstChild);
/// let context = MatchContext::new();
///
/// let matches = evaluate_pseudo_class(&element, &pseudo, &context);
/// assert!(matches); // First child when parent has no siblings info
/// ```
pub fn evaluate_pseudo_class<E: ElementLike + ElementLikeExt>(
    element: &E,
    pseudo: &PseudoClass,
    context: &MatchContext,
) -> bool {
    match &pseudo.kind {
        PseudoClassKind::Hover => {
            if let Some(id) = element.element_id() {
                context.is_hovered(id)
            } else {
                false
            }
        }
        PseudoClassKind::Active => {
            if let Some(id) = element.element_id() {
                context.is_active(id)
            } else {
                false
            }
        }
        PseudoClassKind::Focus => {
            if let Some(id) = element.element_id() {
                context.is_focused(id)
            } else {
                false
            }
        }
        PseudoClassKind::Visited => {
            if let Some(url) = element.link_url() {
                context.is_visited(url)
            } else {
                false
            }
        }
        PseudoClassKind::Link => {
            // :link matches unvisited links
            if let Some(url) = element.link_url() {
                !context.is_visited(url)
            } else {
                false
            }
        }
        PseudoClassKind::FirstChild => {
            // Element is first child if its sibling position is 1
            element.sibling_position() == 1
        }
        PseudoClassKind::LastChild => {
            // Element is last child if its position equals the sibling count
            element.sibling_position() == element.sibling_count()
        }
        PseudoClassKind::NthChild => {
            // Parse the nth selector from the argument
            if let Some(ref arg) = pseudo.argument {
                if let Ok(nth) = crate::nth::parse_nth_selector(arg) {
                    nth.matches(element.sibling_position())
                } else {
                    false
                }
            } else {
                false
            }
        }
        PseudoClassKind::NthLastChild => {
            if let Some(ref arg) = pseudo.argument {
                if let Ok(nth) = crate::nth::parse_nth_selector(arg) {
                    let position_from_end =
                        element.sibling_count() - element.sibling_position() + 1;
                    nth.matches(position_from_end)
                } else {
                    false
                }
            } else {
                false
            }
        }
        PseudoClassKind::NthOfType => {
            if let Some(ref arg) = pseudo.argument {
                if let Ok(nth) = crate::nth::parse_nth_selector(arg) {
                    nth.matches(element.sibling_position_of_type())
                } else {
                    false
                }
            } else {
                false
            }
        }
        PseudoClassKind::NthLastOfType => {
            if let Some(ref arg) = pseudo.argument {
                if let Ok(nth) = crate::nth::parse_nth_selector(arg) {
                    let position_from_end =
                        element.sibling_count_of_type() - element.sibling_position_of_type() + 1;
                    nth.matches(position_from_end)
                } else {
                    false
                }
            } else {
                false
            }
        }
        PseudoClassKind::OnlyChild => {
            // Element is only child if sibling count is 1
            element.sibling_count() == 1
        }
        PseudoClassKind::Empty => {
            // Element is empty if it has no children
            !element.has_children()
        }
        PseudoClassKind::Root => {
            if let Some(id) = element.element_id() {
                context.is_root(id)
            } else {
                // If no ID, check if it has no parent
                element.parent().is_none()
            }
        }
        PseudoClassKind::Target => {
            if let Some(id) = element.element_id() {
                context.is_target(id)
            } else {
                false
            }
        }
        PseudoClassKind::Enabled => element.is_enabled(),
        PseudoClassKind::Disabled => !element.is_enabled(),
        PseudoClassKind::Checked => element.is_checked(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test element implementation
    #[derive(Debug, Clone)]
    struct TestElement {
        tag_name: String,
        element_id: Option<String>,
        sibling_pos: usize,
        sibling_count: usize,
        sibling_pos_of_type: usize,
        sibling_count_of_type: usize,
        has_children: bool,
        enabled: bool,
        checked: bool,
        link_url: Option<String>,
        parent: Option<Box<TestElement>>,
    }

    impl TestElement {
        fn new(tag: &str) -> Self {
            Self {
                tag_name: tag.to_string(),
                element_id: None,
                sibling_pos: 1,
                sibling_count: 1,
                sibling_pos_of_type: 1,
                sibling_count_of_type: 1,
                has_children: false,
                enabled: true,
                checked: false,
                link_url: None,
                parent: None,
            }
        }

        fn with_id(mut self, id: &str) -> Self {
            self.element_id = Some(id.to_string());
            self
        }

        fn with_sibling_position(mut self, pos: usize, count: usize) -> Self {
            self.sibling_pos = pos;
            self.sibling_count = count;
            self
        }

        fn with_type_position(mut self, pos: usize, count: usize) -> Self {
            self.sibling_pos_of_type = pos;
            self.sibling_count_of_type = count;
            self
        }

        fn with_children(mut self, has: bool) -> Self {
            self.has_children = has;
            self
        }

        fn with_enabled(mut self, enabled: bool) -> Self {
            self.enabled = enabled;
            self
        }

        fn with_checked(mut self, checked: bool) -> Self {
            self.checked = checked;
            self
        }

        fn with_link(mut self, url: &str) -> Self {
            self.link_url = Some(url.to_string());
            self
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
            self.parent.as_deref()
        }

        fn previous_sibling(&self) -> Option<&Self> {
            None
        }
    }

    impl ElementLikeExt for TestElement {
        fn element_id(&self) -> Option<&str> {
            self.element_id.as_deref()
        }

        fn sibling_count(&self) -> usize {
            self.sibling_count
        }

        fn sibling_position(&self) -> usize {
            self.sibling_pos
        }

        fn sibling_position_of_type(&self) -> usize {
            self.sibling_pos_of_type
        }

        fn sibling_count_of_type(&self) -> usize {
            self.sibling_count_of_type
        }

        fn has_children(&self) -> bool {
            self.has_children
        }

        fn is_enabled(&self) -> bool {
            self.enabled
        }

        fn is_checked(&self) -> bool {
            self.checked
        }

        fn link_url(&self) -> Option<&str> {
            self.link_url.as_deref()
        }
    }

    // ========================================================================
    // PseudoClass creation tests
    // ========================================================================

    #[test]
    fn test_pseudo_class_new() {
        let pc = PseudoClass::new(PseudoClassKind::Hover);
        assert_eq!(pc.kind, PseudoClassKind::Hover);
        assert!(pc.argument.is_none());
    }

    #[test]
    fn test_pseudo_class_with_argument() {
        let pc = PseudoClass::with_argument(PseudoClassKind::NthChild, "2n+1".to_string());
        assert_eq!(pc.kind, PseudoClassKind::NthChild);
        assert_eq!(pc.argument.as_deref(), Some("2n+1"));
    }

    // ========================================================================
    // MatchContext tests
    // ========================================================================

    #[test]
    fn test_match_context_hover() {
        let mut ctx = MatchContext::new();
        ctx.hovered_elements.push("elem1".to_string());

        assert!(ctx.is_hovered("elem1"));
        assert!(!ctx.is_hovered("elem2"));
    }

    #[test]
    fn test_match_context_focus() {
        let mut ctx = MatchContext::new();
        ctx.focused_element = Some("elem1".to_string());

        assert!(ctx.is_focused("elem1"));
        assert!(!ctx.is_focused("elem2"));
    }

    // ========================================================================
    // evaluate_pseudo_class tests - User action pseudo-classes
    // ========================================================================

    #[test]
    fn test_evaluate_hover_matches() {
        let element = TestElement::new("div").with_id("elem1");
        let pseudo = PseudoClass::new(PseudoClassKind::Hover);
        let mut context = MatchContext::new();
        context.hovered_elements.push("elem1".to_string());

        assert!(evaluate_pseudo_class(&element, &pseudo, &context));
    }

    #[test]
    fn test_evaluate_hover_no_match() {
        let element = TestElement::new("div").with_id("elem1");
        let pseudo = PseudoClass::new(PseudoClassKind::Hover);
        let context = MatchContext::new(); // No hovered elements

        assert!(!evaluate_pseudo_class(&element, &pseudo, &context));
    }

    #[test]
    fn test_evaluate_focus_matches() {
        let element = TestElement::new("input").with_id("input1");
        let pseudo = PseudoClass::new(PseudoClassKind::Focus);
        let mut context = MatchContext::new();
        context.focused_element = Some("input1".to_string());

        assert!(evaluate_pseudo_class(&element, &pseudo, &context));
    }

    // ========================================================================
    // evaluate_pseudo_class tests - Structural pseudo-classes
    // ========================================================================

    #[test]
    fn test_evaluate_first_child_matches() {
        let element = TestElement::new("div").with_sibling_position(1, 3);
        let pseudo = PseudoClass::new(PseudoClassKind::FirstChild);
        let context = MatchContext::new();

        assert!(evaluate_pseudo_class(&element, &pseudo, &context));
    }

    #[test]
    fn test_evaluate_first_child_no_match() {
        let element = TestElement::new("div").with_sibling_position(2, 3);
        let pseudo = PseudoClass::new(PseudoClassKind::FirstChild);
        let context = MatchContext::new();

        assert!(!evaluate_pseudo_class(&element, &pseudo, &context));
    }

    #[test]
    fn test_evaluate_last_child_matches() {
        let element = TestElement::new("div").with_sibling_position(3, 3);
        let pseudo = PseudoClass::new(PseudoClassKind::LastChild);
        let context = MatchContext::new();

        assert!(evaluate_pseudo_class(&element, &pseudo, &context));
    }

    #[test]
    fn test_evaluate_last_child_no_match() {
        let element = TestElement::new("div").with_sibling_position(2, 3);
        let pseudo = PseudoClass::new(PseudoClassKind::LastChild);
        let context = MatchContext::new();

        assert!(!evaluate_pseudo_class(&element, &pseudo, &context));
    }

    #[test]
    fn test_evaluate_only_child_matches() {
        let element = TestElement::new("div").with_sibling_position(1, 1);
        let pseudo = PseudoClass::new(PseudoClassKind::OnlyChild);
        let context = MatchContext::new();

        assert!(evaluate_pseudo_class(&element, &pseudo, &context));
    }

    #[test]
    fn test_evaluate_only_child_no_match() {
        let element = TestElement::new("div").with_sibling_position(1, 2);
        let pseudo = PseudoClass::new(PseudoClassKind::OnlyChild);
        let context = MatchContext::new();

        assert!(!evaluate_pseudo_class(&element, &pseudo, &context));
    }

    #[test]
    fn test_evaluate_nth_child_odd() {
        let pseudo = PseudoClass::with_argument(PseudoClassKind::NthChild, "odd".to_string());
        let context = MatchContext::new();

        let elem1 = TestElement::new("div").with_sibling_position(1, 5);
        assert!(evaluate_pseudo_class(&elem1, &pseudo, &context));

        let elem2 = TestElement::new("div").with_sibling_position(2, 5);
        assert!(!evaluate_pseudo_class(&elem2, &pseudo, &context));

        let elem3 = TestElement::new("div").with_sibling_position(3, 5);
        assert!(evaluate_pseudo_class(&elem3, &pseudo, &context));
    }

    #[test]
    fn test_evaluate_nth_child_even() {
        let pseudo = PseudoClass::with_argument(PseudoClassKind::NthChild, "even".to_string());
        let context = MatchContext::new();

        let elem1 = TestElement::new("div").with_sibling_position(1, 5);
        assert!(!evaluate_pseudo_class(&elem1, &pseudo, &context));

        let elem2 = TestElement::new("div").with_sibling_position(2, 5);
        assert!(evaluate_pseudo_class(&elem2, &pseudo, &context));

        let elem4 = TestElement::new("div").with_sibling_position(4, 5);
        assert!(evaluate_pseudo_class(&elem4, &pseudo, &context));
    }

    #[test]
    fn test_evaluate_nth_last_child() {
        let pseudo = PseudoClass::with_argument(PseudoClassKind::NthLastChild, "2".to_string());
        let context = MatchContext::new();

        // 2nd from last in 5 children = position 4
        let elem = TestElement::new("div").with_sibling_position(4, 5);
        assert!(evaluate_pseudo_class(&elem, &pseudo, &context));

        let elem = TestElement::new("div").with_sibling_position(3, 5);
        assert!(!evaluate_pseudo_class(&elem, &pseudo, &context));
    }

    #[test]
    fn test_evaluate_nth_of_type() {
        let pseudo = PseudoClass::with_argument(PseudoClassKind::NthOfType, "2n".to_string());
        let context = MatchContext::new();

        let elem = TestElement::new("div").with_type_position(2, 4);
        assert!(evaluate_pseudo_class(&elem, &pseudo, &context));

        let elem = TestElement::new("div").with_type_position(1, 4);
        assert!(!evaluate_pseudo_class(&elem, &pseudo, &context));
    }

    // ========================================================================
    // evaluate_pseudo_class tests - Other pseudo-classes
    // ========================================================================

    #[test]
    fn test_evaluate_empty_matches() {
        let element = TestElement::new("div").with_children(false);
        let pseudo = PseudoClass::new(PseudoClassKind::Empty);
        let context = MatchContext::new();

        assert!(evaluate_pseudo_class(&element, &pseudo, &context));
    }

    #[test]
    fn test_evaluate_empty_no_match() {
        let element = TestElement::new("div").with_children(true);
        let pseudo = PseudoClass::new(PseudoClassKind::Empty);
        let context = MatchContext::new();

        assert!(!evaluate_pseudo_class(&element, &pseudo, &context));
    }

    #[test]
    fn test_evaluate_root_matches() {
        let element = TestElement::new("html").with_id("root");
        let pseudo = PseudoClass::new(PseudoClassKind::Root);
        let mut context = MatchContext::new();
        context.root_element = Some("root".to_string());

        assert!(evaluate_pseudo_class(&element, &pseudo, &context));
    }

    #[test]
    fn test_evaluate_enabled_matches() {
        let element = TestElement::new("input").with_enabled(true);
        let pseudo = PseudoClass::new(PseudoClassKind::Enabled);
        let context = MatchContext::new();

        assert!(evaluate_pseudo_class(&element, &pseudo, &context));
    }

    #[test]
    fn test_evaluate_disabled_matches() {
        let element = TestElement::new("input").with_enabled(false);
        let pseudo = PseudoClass::new(PseudoClassKind::Disabled);
        let context = MatchContext::new();

        assert!(evaluate_pseudo_class(&element, &pseudo, &context));
    }

    #[test]
    fn test_evaluate_checked_matches() {
        let element = TestElement::new("input").with_checked(true);
        let pseudo = PseudoClass::new(PseudoClassKind::Checked);
        let context = MatchContext::new();

        assert!(evaluate_pseudo_class(&element, &pseudo, &context));
    }

    #[test]
    fn test_evaluate_link_matches_unvisited() {
        let element = TestElement::new("a").with_link("https://example.com");
        let pseudo = PseudoClass::new(PseudoClassKind::Link);
        let context = MatchContext::new(); // No visited links

        assert!(evaluate_pseudo_class(&element, &pseudo, &context));
    }

    #[test]
    fn test_evaluate_visited_matches() {
        let element = TestElement::new("a").with_link("https://example.com");
        let pseudo = PseudoClass::new(PseudoClassKind::Visited);
        let mut context = MatchContext::new();
        context
            .visited_links
            .push("https://example.com".to_string());

        assert!(evaluate_pseudo_class(&element, &pseudo, &context));
    }
}
