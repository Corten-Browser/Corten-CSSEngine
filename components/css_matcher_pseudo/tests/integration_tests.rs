//! Integration tests for complex pseudo-selector combinations

use css_matcher_core::ElementLike;
use css_matcher_pseudo::{
    evaluate_pseudo_class, DefaultPseudoElementMatcher, ElementLikeExt, MatchContext, PseudoClass,
    PseudoClassKind, PseudoElement, PseudoElementMatcher,
};

// Test element implementation with full support for pseudo-class matching
#[derive(Debug, Clone)]
struct TestElement {
    tag_name: String,
    element_id: Option<String>,
    classes: Vec<String>,
    sibling_pos: usize,
    sibling_count: usize,
    sibling_pos_of_type: usize,
    sibling_count_of_type: usize,
    has_children: bool,
    enabled: bool,
    checked: bool,
    link_url: Option<String>,
}

impl TestElement {
    fn new(tag: &str) -> Self {
        Self {
            tag_name: tag.to_string(),
            element_id: None,
            classes: Vec::new(),
            sibling_pos: 1,
            sibling_count: 1,
            sibling_pos_of_type: 1,
            sibling_count_of_type: 1,
            has_children: false,
            enabled: true,
            checked: false,
            link_url: None,
        }
    }

    fn with_id(mut self, id: &str) -> Self {
        self.element_id = Some(id.to_string());
        self
    }

    #[allow(dead_code)]
    fn with_class(mut self, class: &str) -> Self {
        self.classes.push(class.to_string());
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
        self.element_id.as_deref()
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
// Integration Tests - Complex Pseudo-Class Combinations
// ========================================================================

#[test]
fn test_first_child_hover_combination() {
    // Test: first child that is hovered
    let element = TestElement::new("div")
        .with_id("elem1")
        .with_sibling_position(1, 5);

    let mut context = MatchContext::new();
    context.hovered_elements.push("elem1".to_string());

    let first_child = PseudoClass::new(PseudoClassKind::FirstChild);
    let hover = PseudoClass::new(PseudoClassKind::Hover);

    assert!(evaluate_pseudo_class(&element, &first_child, &context));
    assert!(evaluate_pseudo_class(&element, &hover, &context));
}

#[test]
fn test_nth_child_disabled_combination() {
    // Test: nth-child(even) that is disabled
    let element = TestElement::new("input")
        .with_sibling_position(4, 10)
        .with_enabled(false);

    let context = MatchContext::new();

    let nth_child = PseudoClass::with_argument(PseudoClassKind::NthChild, "even".to_string());
    let disabled = PseudoClass::new(PseudoClassKind::Disabled);

    assert!(evaluate_pseudo_class(&element, &nth_child, &context));
    assert!(evaluate_pseudo_class(&element, &disabled, &context));
}

#[test]
fn test_last_child_empty_combination() {
    // Test: last child that is empty
    let element = TestElement::new("div")
        .with_sibling_position(3, 3)
        .with_children(false);

    let context = MatchContext::new();

    let last_child = PseudoClass::new(PseudoClassKind::LastChild);
    let empty = PseudoClass::new(PseudoClassKind::Empty);

    assert!(evaluate_pseudo_class(&element, &last_child, &context));
    assert!(evaluate_pseudo_class(&element, &empty, &context));
}

#[test]
fn test_link_hover_visited_states() {
    // Test: unvisited link that is hovered
    let element = TestElement::new("a")
        .with_id("link1")
        .with_link("https://example.com");

    let mut context = MatchContext::new();
    context.hovered_elements.push("link1".to_string());

    let link = PseudoClass::new(PseudoClassKind::Link);
    let hover = PseudoClass::new(PseudoClassKind::Hover);
    let visited = PseudoClass::new(PseudoClassKind::Visited);

    assert!(evaluate_pseudo_class(&element, &link, &context)); // Unvisited
    assert!(evaluate_pseudo_class(&element, &hover, &context)); // Hovered
    assert!(!evaluate_pseudo_class(&element, &visited, &context)); // Not visited

    // Now mark as visited
    context
        .visited_links
        .push("https://example.com".to_string());

    assert!(!evaluate_pseudo_class(&element, &link, &context)); // No longer unvisited
    assert!(evaluate_pseudo_class(&element, &visited, &context)); // Now visited
}

#[test]
fn test_checked_enabled_combination() {
    // Test: checkbox that is checked and enabled
    let element = TestElement::new("input")
        .with_checked(true)
        .with_enabled(true);

    let context = MatchContext::new();

    let checked = PseudoClass::new(PseudoClassKind::Checked);
    let enabled = PseudoClass::new(PseudoClassKind::Enabled);

    assert!(evaluate_pseudo_class(&element, &checked, &context));
    assert!(evaluate_pseudo_class(&element, &enabled, &context));
}

#[test]
fn test_only_child_focus_combination() {
    // Test: only child that has focus
    let element = TestElement::new("input")
        .with_id("input1")
        .with_sibling_position(1, 1);

    let mut context = MatchContext::new();
    context.focused_element = Some("input1".to_string());

    let only_child = PseudoClass::new(PseudoClassKind::OnlyChild);
    let focus = PseudoClass::new(PseudoClassKind::Focus);

    assert!(evaluate_pseudo_class(&element, &only_child, &context));
    assert!(evaluate_pseudo_class(&element, &focus, &context));
}

#[test]
fn test_nth_of_type_complex() {
    // Test: 3rd paragraph among siblings, 2nd paragraph of type
    let element = TestElement::new("p")
        .with_sibling_position(3, 7)
        .with_type_position(2, 3);

    let context = MatchContext::new();

    let nth_child = PseudoClass::with_argument(PseudoClassKind::NthChild, "3".to_string());
    let nth_of_type = PseudoClass::with_argument(PseudoClassKind::NthOfType, "2".to_string());

    assert!(evaluate_pseudo_class(&element, &nth_child, &context));
    assert!(evaluate_pseudo_class(&element, &nth_of_type, &context));
}

#[test]
fn test_root_element() {
    // Test: root element
    let element = TestElement::new("html").with_id("root");

    let mut context = MatchContext::new();
    context.root_element = Some("root".to_string());

    let root = PseudoClass::new(PseudoClassKind::Root);

    assert!(evaluate_pseudo_class(&element, &root, &context));
}

#[test]
fn test_target_element() {
    // Test: element is the target of URL fragment
    let element = TestElement::new("div").with_id("section1");

    let mut context = MatchContext::new();
    context.target_element = Some("section1".to_string());

    let target = PseudoClass::new(PseudoClassKind::Target);

    assert!(evaluate_pseudo_class(&element, &target, &context));
}

// ========================================================================
// Integration Tests - Pseudo-Elements
// ========================================================================

#[test]
fn test_pseudo_element_on_block_elements() {
    let matcher = DefaultPseudoElementMatcher;

    let div = TestElement::new("div");
    let p = TestElement::new("p");
    let h1 = TestElement::new("h1");

    let before = PseudoElement::before();
    let after = PseudoElement::after();
    let first_line = PseudoElement::first_line();
    let first_letter = PseudoElement::first_letter();

    // All block elements support ::before and ::after
    assert!(matcher.matches_pseudo_element(&div, &before));
    assert!(matcher.matches_pseudo_element(&div, &after));
    assert!(matcher.matches_pseudo_element(&p, &before));
    assert!(matcher.matches_pseudo_element(&p, &after));
    assert!(matcher.matches_pseudo_element(&h1, &before));
    assert!(matcher.matches_pseudo_element(&h1, &after));

    // Block elements support ::first-line and ::first-letter
    assert!(matcher.matches_pseudo_element(&div, &first_line));
    assert!(matcher.matches_pseudo_element(&div, &first_letter));
    assert!(matcher.matches_pseudo_element(&p, &first_line));
    assert!(matcher.matches_pseudo_element(&p, &first_letter));
}

#[test]
fn test_pseudo_element_on_inline_elements() {
    let matcher = DefaultPseudoElementMatcher;

    let span = TestElement::new("span");
    let a = TestElement::new("a");
    let em = TestElement::new("em");

    let before = PseudoElement::before();
    let after = PseudoElement::after();
    let first_line = PseudoElement::first_line();
    let first_letter = PseudoElement::first_letter();

    // Inline elements support ::before and ::after
    assert!(matcher.matches_pseudo_element(&span, &before));
    assert!(matcher.matches_pseudo_element(&span, &after));

    // Inline elements do NOT support ::first-line and ::first-letter
    assert!(!matcher.matches_pseudo_element(&span, &first_line));
    assert!(!matcher.matches_pseudo_element(&span, &first_letter));
    assert!(!matcher.matches_pseudo_element(&a, &first_line));
    assert!(!matcher.matches_pseudo_element(&em, &first_letter));
}

#[test]
fn test_pseudo_element_marker_on_list_items() {
    let matcher = DefaultPseudoElementMatcher;

    let li = TestElement::new("li");
    let div = TestElement::new("div");

    let marker = PseudoElement::marker();

    // Only list items support ::marker
    assert!(matcher.matches_pseudo_element(&li, &marker));
    assert!(!matcher.matches_pseudo_element(&div, &marker));
}

#[test]
fn test_pseudo_element_selection_on_all_elements() {
    let matcher = DefaultPseudoElementMatcher;

    let div = TestElement::new("div");
    let span = TestElement::new("span");
    let p = TestElement::new("p");

    let selection = PseudoElement::selection();

    // All elements support ::selection
    assert!(matcher.matches_pseudo_element(&div, &selection));
    assert!(matcher.matches_pseudo_element(&span, &selection));
    assert!(matcher.matches_pseudo_element(&p, &selection));
}

#[test]
fn test_pseudo_element_style_retrieval() {
    let matcher = DefaultPseudoElementMatcher;

    let div = TestElement::new("div");
    let before = PseudoElement::before();

    let style = matcher.get_pseudo_element_style(&div, &before);
    assert!(style.is_some());

    let style = style.unwrap();
    assert_eq!(style.kind, css_matcher_pseudo::PseudoElementKind::Before);
    assert!(style.rendered);
}

// ========================================================================
// Integration Tests - Edge Cases
// ========================================================================

#[test]
fn test_nth_child_with_invalid_argument() {
    let element = TestElement::new("div").with_sibling_position(2, 5);

    let context = MatchContext::new();

    // Invalid argument should not match
    let nth_child = PseudoClass::with_argument(PseudoClassKind::NthChild, "invalid".to_string());

    assert!(!evaluate_pseudo_class(&element, &nth_child, &context));
}

#[test]
fn test_nth_last_child_edge_cases() {
    let context = MatchContext::new();

    // Test 1st from last in 5 children (position 5)
    let elem = TestElement::new("div").with_sibling_position(5, 5);
    let nth = PseudoClass::with_argument(PseudoClassKind::NthLastChild, "1".to_string());
    assert!(evaluate_pseudo_class(&elem, &nth, &context));

    // Test 2nd from last in 5 children (position 4)
    let elem = TestElement::new("div").with_sibling_position(4, 5);
    let nth = PseudoClass::with_argument(PseudoClassKind::NthLastChild, "2".to_string());
    assert!(evaluate_pseudo_class(&elem, &nth, &context));
}

#[test]
fn test_element_without_id_state_pseudo_classes() {
    // Elements without IDs should not match state pseudo-classes
    let element = TestElement::new("div"); // No ID set

    let mut context = MatchContext::new();
    context.hovered_elements.push("elem1".to_string());
    context.focused_element = Some("elem1".to_string());

    let hover = PseudoClass::new(PseudoClassKind::Hover);
    let focus = PseudoClass::new(PseudoClassKind::Focus);

    assert!(!evaluate_pseudo_class(&element, &hover, &context));
    assert!(!evaluate_pseudo_class(&element, &focus, &context));
}
