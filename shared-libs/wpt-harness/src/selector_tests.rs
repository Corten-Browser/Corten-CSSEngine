//! WPT Selector Tests - Selector Matching Conformance Tests
//!
//! This module contains WPT-style test cases for CSS selector matching.
//! Tests cover simple selectors, compound selectors, and combinators.

use crate::harness::{WptExpectation, WptTestCase, WptTestResult, WptTestRunner, TestElement};

/// A DOM tree for selector testing
#[derive(Debug, Clone)]
pub struct TestDom {
    /// Root element of the DOM
    pub root: TestDomNode,
}

/// A node in the test DOM tree
#[derive(Debug, Clone)]
pub struct TestDomNode {
    /// Element data
    pub element: TestElement,
    /// Child nodes
    pub children: Vec<TestDomNode>,
}

impl TestDomNode {
    /// Create a new DOM node
    pub fn new(element: TestElement) -> Self {
        Self {
            element,
            children: Vec::new(),
        }
    }

    /// Add a child node
    pub fn with_child(mut self, child: TestDomNode) -> Self {
        self.children.push(child);
        self
    }

    /// Find all elements matching a simple selector
    pub fn query_all(&self, selector: &SimpleSelector) -> Vec<&TestElement> {
        let mut matches = Vec::new();
        self.query_all_recursive(selector, &mut matches);
        matches
    }

    fn query_all_recursive<'a>(&'a self, selector: &SimpleSelector, matches: &mut Vec<&'a TestElement>) {
        if selector.matches(&self.element) {
            matches.push(&self.element);
        }
        for child in &self.children {
            child.query_all_recursive(selector, matches);
        }
    }

    /// Get all element IDs in the tree
    pub fn all_ids(&self) -> Vec<String> {
        let mut ids = Vec::new();
        self.collect_ids(&mut ids);
        ids
    }

    fn collect_ids(&self, ids: &mut Vec<String>) {
        if let Some(id) = &self.element.id {
            ids.push(id.clone());
        }
        for child in &self.children {
            child.collect_ids(ids);
        }
    }
}

/// A simplified selector for testing
#[derive(Debug, Clone, PartialEq)]
pub enum SimpleSelector {
    /// Universal selector (*)
    Universal,
    /// Element/tag selector (e.g., div)
    Element(String),
    /// Class selector (e.g., .button)
    Class(String),
    /// ID selector (e.g., #header)
    Id(String),
    /// Compound selector (multiple conditions ANDed together)
    Compound(Vec<SimpleSelector>),
}

impl SimpleSelector {
    /// Check if this selector matches an element
    pub fn matches(&self, element: &TestElement) -> bool {
        match self {
            SimpleSelector::Universal => true,
            SimpleSelector::Element(tag) => {
                element.tag_name.eq_ignore_ascii_case(tag)
            }
            SimpleSelector::Class(class) => {
                element.classes.iter().any(|c| c == class)
            }
            SimpleSelector::Id(id) => {
                element.id.as_ref() == Some(id)
            }
            SimpleSelector::Compound(selectors) => {
                selectors.iter().all(|s| s.matches(element))
            }
        }
    }

    /// Parse a simple selector string
    pub fn parse(input: &str) -> Option<Self> {
        let input = input.trim();

        if input.is_empty() {
            return None;
        }

        if input == "*" {
            return Some(SimpleSelector::Universal);
        }

        // Check for compound selector
        if input.contains('.') || input.contains('#') {
            let mut selectors = Vec::new();
            let mut current = String::new();
            let mut current_type = 'T'; // T = tag, C = class, I = id

            for ch in input.chars() {
                match ch {
                    '.' => {
                        if !current.is_empty() {
                            match current_type {
                                'T' => selectors.push(SimpleSelector::Element(current.clone())),
                                'C' => selectors.push(SimpleSelector::Class(current.clone())),
                                'I' => selectors.push(SimpleSelector::Id(current.clone())),
                                _ => {}
                            }
                        }
                        current.clear();
                        current_type = 'C';
                    }
                    '#' => {
                        if !current.is_empty() {
                            match current_type {
                                'T' => selectors.push(SimpleSelector::Element(current.clone())),
                                'C' => selectors.push(SimpleSelector::Class(current.clone())),
                                'I' => selectors.push(SimpleSelector::Id(current.clone())),
                                _ => {}
                            }
                        }
                        current.clear();
                        current_type = 'I';
                    }
                    _ => {
                        current.push(ch);
                    }
                }
            }

            // Handle remaining
            if !current.is_empty() {
                match current_type {
                    'T' => selectors.push(SimpleSelector::Element(current)),
                    'C' => selectors.push(SimpleSelector::Class(current)),
                    'I' => selectors.push(SimpleSelector::Id(current)),
                    _ => {}
                }
            }

            if selectors.len() == 1 {
                return selectors.into_iter().next();
            }
            return Some(SimpleSelector::Compound(selectors));
        }

        // Simple element selector
        Some(SimpleSelector::Element(input.to_string()))
    }
}

/// Run a selector matching test
pub fn run_selector_test(test: &WptTestCase) -> WptTestResult {
    match &test.expected {
        WptExpectation::MatchesElements(expected_ids) => {
            // Parse the selector from CSS
            let selector = match parse_selector_from_css(&test.css) {
                Some(s) => s,
                None => return WptTestResult::Error("Failed to parse selector".to_string()),
            };

            // Build DOM from HTML or use a default test DOM
            let dom = match &test.html {
                Some(html) => match build_test_dom(html) {
                    Some(d) => d,
                    None => return WptTestResult::Error("Failed to build test DOM".to_string()),
                },
                None => create_default_test_dom(),
            };

            // Find matching elements
            let matches = dom.root.query_all(&selector);
            let matched_ids: Vec<String> = matches
                .iter()
                .filter_map(|e| e.id.clone())
                .collect();

            // Compare with expected
            let expected_set: std::collections::HashSet<_> = expected_ids.iter().cloned().collect();
            let matched_set: std::collections::HashSet<_> = matched_ids.iter().cloned().collect();

            if expected_set == matched_set {
                WptTestResult::Pass
            } else {
                WptTestResult::Fail(format!(
                    "Selector mismatch:\n  Expected: {:?}\n  Got: {:?}",
                    expected_ids, matched_ids
                ))
            }
        }
        WptExpectation::ShouldParse => {
            // Just check that the selector parses
            match parse_selector_from_css(&test.css) {
                Some(_) => WptTestResult::Pass,
                None => WptTestResult::Fail("Expected selector to parse".to_string()),
            }
        }
        WptExpectation::ShouldNotParse => {
            // Check that the selector doesn't parse
            match parse_selector_from_css(&test.css) {
                Some(_) => WptTestResult::Fail("Expected selector not to parse".to_string()),
                None => WptTestResult::Pass,
            }
        }
        _ => WptTestResult::Error("Unsupported expectation for selector test".to_string()),
    }
}

/// Extract selector from CSS rule (e.g., "div.class { }" -> "div.class")
fn parse_selector_from_css(css: &str) -> Option<SimpleSelector> {
    let css = css.trim();

    // Extract selector part (before {)
    let selector_str = css.split('{').next()?.trim();

    if selector_str.is_empty() {
        return None;
    }

    // For now, handle only the first selector in a list
    let first_selector = selector_str.split(',').next()?.trim();

    SimpleSelector::parse(first_selector)
}

/// Build a test DOM from a simplified HTML-like string
fn build_test_dom(html: &str) -> Option<TestDom> {
    // Simple parser for test DOM strings
    // Format: tag#id.class1.class2 > child1, child2

    let html = html.trim();
    if html.is_empty() {
        return None;
    }

    // Parse root element
    let element = parse_element_string(html)?;

    Some(TestDom {
        root: TestDomNode::new(element),
    })
}

/// Parse a single element from string
fn parse_element_string(input: &str) -> Option<TestElement> {
    let input = input.trim();
    if input.is_empty() {
        return None;
    }

    let mut tag = String::new();
    let mut id = None;
    let mut classes = Vec::new();

    let mut current = String::new();
    let mut mode = 'T'; // T = tag, I = id, C = class

    for ch in input.chars() {
        match ch {
            '#' => {
                if mode == 'T' && !current.is_empty() {
                    tag = current.clone();
                } else if mode == 'C' && !current.is_empty() {
                    classes.push(current.clone());
                }
                current.clear();
                mode = 'I';
            }
            '.' => {
                if mode == 'T' && !current.is_empty() {
                    tag = current.clone();
                } else if mode == 'I' && !current.is_empty() {
                    id = Some(current.clone());
                } else if mode == 'C' && !current.is_empty() {
                    classes.push(current.clone());
                }
                current.clear();
                mode = 'C';
            }
            ' ' | '>' | ',' | '{' => break,
            _ => current.push(ch),
        }
    }

    // Handle remaining
    match mode {
        'T' if !current.is_empty() => tag = current,
        'I' if !current.is_empty() => id = Some(current),
        'C' if !current.is_empty() => classes.push(current),
        _ => {}
    }

    if tag.is_empty() {
        tag = "div".to_string();
    }

    let mut element = TestElement::new(tag);
    element.id = id;
    element.classes = classes;

    Some(element)
}

/// Create a default test DOM for selector tests
pub fn create_default_test_dom() -> TestDom {
    // Create a diverse DOM tree for testing
    TestDom {
        root: TestDomNode::new(
            TestElement::new("div")
                .with_id("root")
                .with_class("container")
        )
        .with_child(
            TestDomNode::new(
                TestElement::new("header")
                    .with_id("header")
                    .with_class("top")
            )
            .with_child(
                TestDomNode::new(
                    TestElement::new("nav")
                        .with_id("nav")
                        .with_class("navigation")
                )
            )
        )
        .with_child(
            TestDomNode::new(
                TestElement::new("main")
                    .with_id("main")
                    .with_class("content")
            )
            .with_child(
                TestDomNode::new(
                    TestElement::new("article")
                        .with_id("article1")
                        .with_class("post")
                        .with_class("featured")
                )
            )
            .with_child(
                TestDomNode::new(
                    TestElement::new("article")
                        .with_id("article2")
                        .with_class("post")
                )
            )
            .with_child(
                TestDomNode::new(
                    TestElement::new("aside")
                        .with_id("sidebar")
                        .with_class("sidebar")
                )
            )
        )
        .with_child(
            TestDomNode::new(
                TestElement::new("footer")
                    .with_id("footer")
                    .with_class("bottom")
            )
        ),
    }
}

/// Create the standard set of selector matching tests
pub fn create_selector_tests() -> Vec<WptTestCase> {
    vec![
        // ============================================
        // Simple Selectors
        // ============================================

        // Universal selector
        WptTestCase::new(
            "selectors/simple/universal",
            "* { }",
        )
        .with_expectation(WptExpectation::MatchesElements(vec![
            "root".to_string(),
            "header".to_string(),
            "nav".to_string(),
            "main".to_string(),
            "article1".to_string(),
            "article2".to_string(),
            "sidebar".to_string(),
            "footer".to_string(),
        ]))
        .with_flag("universal"),

        // Element selectors
        WptTestCase::new(
            "selectors/simple/element-div",
            "div { }",
        )
        .with_expectation(WptExpectation::MatchesElements(vec![
            "root".to_string(),
        ]))
        .with_flag("element"),

        WptTestCase::new(
            "selectors/simple/element-article",
            "article { }",
        )
        .with_expectation(WptExpectation::MatchesElements(vec![
            "article1".to_string(),
            "article2".to_string(),
        ]))
        .with_flag("element"),

        WptTestCase::new(
            "selectors/simple/element-header",
            "header { }",
        )
        .with_expectation(WptExpectation::MatchesElements(vec![
            "header".to_string(),
        ]))
        .with_flag("element"),

        WptTestCase::new(
            "selectors/simple/element-nonexistent",
            "span { }",
        )
        .with_expectation(WptExpectation::MatchesElements(vec![]))
        .with_flag("element"),

        // Class selectors
        WptTestCase::new(
            "selectors/simple/class-post",
            ".post { }",
        )
        .with_expectation(WptExpectation::MatchesElements(vec![
            "article1".to_string(),
            "article2".to_string(),
        ]))
        .with_flag("class"),

        WptTestCase::new(
            "selectors/simple/class-featured",
            ".featured { }",
        )
        .with_expectation(WptExpectation::MatchesElements(vec![
            "article1".to_string(),
        ]))
        .with_flag("class"),

        WptTestCase::new(
            "selectors/simple/class-container",
            ".container { }",
        )
        .with_expectation(WptExpectation::MatchesElements(vec![
            "root".to_string(),
        ]))
        .with_flag("class"),

        WptTestCase::new(
            "selectors/simple/class-nonexistent",
            ".nonexistent { }",
        )
        .with_expectation(WptExpectation::MatchesElements(vec![]))
        .with_flag("class"),

        // ID selectors
        WptTestCase::new(
            "selectors/simple/id-root",
            "#root { }",
        )
        .with_expectation(WptExpectation::MatchesElements(vec![
            "root".to_string(),
        ]))
        .with_flag("id"),

        WptTestCase::new(
            "selectors/simple/id-header",
            "#header { }",
        )
        .with_expectation(WptExpectation::MatchesElements(vec![
            "header".to_string(),
        ]))
        .with_flag("id"),

        WptTestCase::new(
            "selectors/simple/id-article1",
            "#article1 { }",
        )
        .with_expectation(WptExpectation::MatchesElements(vec![
            "article1".to_string(),
        ]))
        .with_flag("id"),

        WptTestCase::new(
            "selectors/simple/id-nonexistent",
            "#nonexistent { }",
        )
        .with_expectation(WptExpectation::MatchesElements(vec![]))
        .with_flag("id"),

        // ============================================
        // Compound Selectors
        // ============================================

        WptTestCase::new(
            "selectors/compound/element-class",
            "article.post { }",
        )
        .with_expectation(WptExpectation::MatchesElements(vec![
            "article1".to_string(),
            "article2".to_string(),
        ]))
        .with_flag("compound"),

        WptTestCase::new(
            "selectors/compound/element-class-specific",
            "article.featured { }",
        )
        .with_expectation(WptExpectation::MatchesElements(vec![
            "article1".to_string(),
        ]))
        .with_flag("compound"),

        WptTestCase::new(
            "selectors/compound/element-id",
            "header#header { }",
        )
        .with_expectation(WptExpectation::MatchesElements(vec![
            "header".to_string(),
        ]))
        .with_flag("compound"),

        WptTestCase::new(
            "selectors/compound/class-class",
            ".post.featured { }",
        )
        .with_expectation(WptExpectation::MatchesElements(vec![
            "article1".to_string(),
        ]))
        .with_flag("compound"),

        WptTestCase::new(
            "selectors/compound/element-class-id",
            "div#root.container { }",
        )
        .with_expectation(WptExpectation::MatchesElements(vec![
            "root".to_string(),
        ]))
        .with_flag("compound"),

        WptTestCase::new(
            "selectors/compound/no-match-wrong-element",
            "span.post { }",
        )
        .with_expectation(WptExpectation::MatchesElements(vec![]))
        .with_flag("compound"),

        WptTestCase::new(
            "selectors/compound/no-match-wrong-class",
            "article.sidebar { }",
        )
        .with_expectation(WptExpectation::MatchesElements(vec![]))
        .with_flag("compound"),

        // ============================================
        // Selector Parsing Tests
        // ============================================

        WptTestCase::new(
            "selectors/parse/valid-element",
            "div { }",
        )
        .with_expectation(WptExpectation::ShouldParse)
        .with_flag("parse"),

        WptTestCase::new(
            "selectors/parse/valid-class",
            ".class { }",
        )
        .with_expectation(WptExpectation::ShouldParse)
        .with_flag("parse"),

        WptTestCase::new(
            "selectors/parse/valid-id",
            "#id { }",
        )
        .with_expectation(WptExpectation::ShouldParse)
        .with_flag("parse"),

        WptTestCase::new(
            "selectors/parse/valid-universal",
            "* { }",
        )
        .with_expectation(WptExpectation::ShouldParse)
        .with_flag("parse"),

        WptTestCase::new(
            "selectors/parse/valid-compound",
            "div.class#id { }",
        )
        .with_expectation(WptExpectation::ShouldParse)
        .with_flag("parse"),

        // ============================================
        // Case Sensitivity Tests
        // ============================================

        WptTestCase::new(
            "selectors/case/element-lowercase",
            "header { }",
        )
        .with_expectation(WptExpectation::MatchesElements(vec![
            "header".to_string(),
        ]))
        .with_flag("case"),

        WptTestCase::new(
            "selectors/case/element-uppercase",
            "HEADER { }",
        )
        .with_expectation(WptExpectation::MatchesElements(vec![
            "header".to_string(),
        ]))
        .with_flag("case"),

        WptTestCase::new(
            "selectors/case/element-mixedcase",
            "HeAdEr { }",
        )
        .with_expectation(WptExpectation::MatchesElements(vec![
            "header".to_string(),
        ]))
        .with_flag("case"),
    ]
}

/// Run all selector tests and return statistics
pub fn run_all_selector_tests() -> crate::harness::WptTestStats {
    let mut runner = WptTestRunner::new().verbose(true);
    let tests = create_selector_tests();
    runner.run_tests(tests, run_selector_test)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_selector_parse_universal() {
        let selector = SimpleSelector::parse("*").unwrap();
        assert_eq!(selector, SimpleSelector::Universal);
    }

    #[test]
    fn test_simple_selector_parse_element() {
        let selector = SimpleSelector::parse("div").unwrap();
        assert_eq!(selector, SimpleSelector::Element("div".to_string()));
    }

    #[test]
    fn test_simple_selector_parse_class() {
        let selector = SimpleSelector::parse(".button").unwrap();
        assert_eq!(selector, SimpleSelector::Class("button".to_string()));
    }

    #[test]
    fn test_simple_selector_parse_id() {
        let selector = SimpleSelector::parse("#header").unwrap();
        assert_eq!(selector, SimpleSelector::Id("header".to_string()));
    }

    #[test]
    fn test_simple_selector_parse_compound() {
        let selector = SimpleSelector::parse("div.class#id").unwrap();
        if let SimpleSelector::Compound(parts) = selector {
            assert_eq!(parts.len(), 3);
            assert!(parts.contains(&SimpleSelector::Element("div".to_string())));
            assert!(parts.contains(&SimpleSelector::Class("class".to_string())));
            assert!(parts.contains(&SimpleSelector::Id("id".to_string())));
        } else {
            panic!("Expected compound selector");
        }
    }

    #[test]
    fn test_simple_selector_matches_universal() {
        let selector = SimpleSelector::Universal;
        let element = TestElement::new("div").with_class("test");
        assert!(selector.matches(&element));
    }

    #[test]
    fn test_simple_selector_matches_element() {
        let selector = SimpleSelector::Element("div".to_string());
        assert!(selector.matches(&TestElement::new("div")));
        assert!(!selector.matches(&TestElement::new("span")));
    }

    #[test]
    fn test_simple_selector_matches_class() {
        let selector = SimpleSelector::Class("button".to_string());
        assert!(selector.matches(&TestElement::new("div").with_class("button")));
        assert!(!selector.matches(&TestElement::new("div").with_class("other")));
    }

    #[test]
    fn test_simple_selector_matches_id() {
        let selector = SimpleSelector::Id("header".to_string());
        assert!(selector.matches(&TestElement::new("div").with_id("header")));
        assert!(!selector.matches(&TestElement::new("div").with_id("other")));
    }

    #[test]
    fn test_simple_selector_matches_compound() {
        let selector = SimpleSelector::Compound(vec![
            SimpleSelector::Element("div".to_string()),
            SimpleSelector::Class("button".to_string()),
        ]);
        assert!(selector.matches(&TestElement::new("div").with_class("button")));
        assert!(!selector.matches(&TestElement::new("span").with_class("button")));
        assert!(!selector.matches(&TestElement::new("div").with_class("other")));
    }

    #[test]
    fn test_parse_selector_from_css() {
        let selector = parse_selector_from_css("div { color: red; }").unwrap();
        assert_eq!(selector, SimpleSelector::Element("div".to_string()));

        let selector = parse_selector_from_css(".class { }").unwrap();
        assert_eq!(selector, SimpleSelector::Class("class".to_string()));

        let selector = parse_selector_from_css("#id { }").unwrap();
        assert_eq!(selector, SimpleSelector::Id("id".to_string()));
    }

    #[test]
    fn test_default_test_dom_structure() {
        let dom = create_default_test_dom();
        let ids = dom.root.all_ids();

        assert!(ids.contains(&"root".to_string()));
        assert!(ids.contains(&"header".to_string()));
        assert!(ids.contains(&"nav".to_string()));
        assert!(ids.contains(&"main".to_string()));
        assert!(ids.contains(&"article1".to_string()));
        assert!(ids.contains(&"article2".to_string()));
        assert!(ids.contains(&"sidebar".to_string()));
        assert!(ids.contains(&"footer".to_string()));
    }

    #[test]
    fn test_dom_query_all_universal() {
        let dom = create_default_test_dom();
        let matches = dom.root.query_all(&SimpleSelector::Universal);
        assert_eq!(matches.len(), 8); // All elements
    }

    #[test]
    fn test_dom_query_all_element() {
        let dom = create_default_test_dom();
        let matches = dom.root.query_all(&SimpleSelector::Element("article".to_string()));
        assert_eq!(matches.len(), 2);
    }

    #[test]
    fn test_dom_query_all_class() {
        let dom = create_default_test_dom();
        let matches = dom.root.query_all(&SimpleSelector::Class("post".to_string()));
        assert_eq!(matches.len(), 2);
    }

    #[test]
    fn test_run_selector_test_matches() {
        let test = WptTestCase::new("test", ".post { }")
            .with_expectation(WptExpectation::MatchesElements(vec![
                "article1".to_string(),
                "article2".to_string(),
            ]));

        let result = run_selector_test(&test);
        assert!(result.is_pass(), "Result: {:?}", result);
    }

    #[test]
    fn test_run_selector_test_no_match() {
        let test = WptTestCase::new("test", ".nonexistent { }")
            .with_expectation(WptExpectation::MatchesElements(vec![]));

        let result = run_selector_test(&test);
        assert!(result.is_pass(), "Result: {:?}", result);
    }

    #[test]
    fn test_selector_tests_collection() {
        let tests = create_selector_tests();
        assert!(tests.len() >= 20);

        // Check that tests have names
        for test in &tests {
            assert!(!test.name.is_empty());
        }
    }

    #[test]
    fn test_all_selector_tests_run() {
        let mut runner = WptTestRunner::new();
        let tests = create_selector_tests();
        let stats = runner.run_tests(tests, run_selector_test);

        // All tests should complete
        assert_eq!(stats.timeouts, 0);
        assert_eq!(stats.errors, 0);

        // Most tests should pass
        assert!(stats.pass_rate() > 90.0, "Pass rate: {}", stats.pass_rate());
    }
}
