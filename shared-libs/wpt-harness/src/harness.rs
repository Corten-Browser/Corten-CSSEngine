//! WPT (Web Platform Tests) Test Harness for CSS Conformance Testing
//!
//! This module provides a test harness for running WPT-style CSS tests.
//! It supports parsing CSS test metadata, running tests, and reporting results.

use std::collections::HashSet;
use std::fmt;
use std::time::{Duration, Instant};

/// Result of running a WPT test
#[derive(Debug, Clone, PartialEq)]
pub enum WptTestResult {
    /// Test passed successfully
    Pass,
    /// Test failed with a message
    Fail(String),
    /// Test timed out after the specified duration
    Timeout(Duration),
    /// Test encountered an error during execution
    Error(String),
}

impl WptTestResult {
    /// Returns true if the test passed
    pub fn is_pass(&self) -> bool {
        matches!(self, WptTestResult::Pass)
    }

    /// Returns true if the test failed (not pass, timeout, or error)
    pub fn is_fail(&self) -> bool {
        matches!(self, WptTestResult::Fail(_))
    }
}

impl fmt::Display for WptTestResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WptTestResult::Pass => write!(f, "PASS"),
            WptTestResult::Fail(msg) => write!(f, "FAIL: {}", msg),
            WptTestResult::Timeout(dur) => write!(f, "TIMEOUT after {:?}", dur),
            WptTestResult::Error(msg) => write!(f, "ERROR: {}", msg),
        }
    }
}

/// Expected outcome of a WPT test
#[derive(Debug, Clone, PartialEq, Default)]
pub enum WptExpectation {
    /// CSS should parse successfully
    #[default]
    ShouldParse,
    /// CSS should NOT parse (expect parse error)
    ShouldNotParse,
    /// Selector should match these element IDs
    MatchesElements(Vec<String>),
    /// Property should compute to expected value
    ComputedValue(String, String),
    /// CSS should serialize to expected string
    SerializesTo(String),
    /// Custom assertion with a description
    Custom(String),
}


/// A single WPT test case
#[derive(Debug, Clone)]
pub struct WptTestCase {
    /// Unique name/identifier for this test
    pub name: String,
    /// CSS content to test
    pub css: String,
    /// Expected outcome
    pub expected: WptExpectation,
    /// Test flags (e.g., "invalid", "dom-setup-required", "slow")
    pub flags: HashSet<String>,
    /// Optional HTML context for selector tests
    pub html: Option<String>,
    /// Optional timeout override (default is 5 seconds)
    pub timeout: Option<Duration>,
}

impl WptTestCase {
    /// Create a new test case with minimal required fields
    pub fn new(name: impl Into<String>, css: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            css: css.into(),
            expected: WptExpectation::default(),
            flags: HashSet::new(),
            html: None,
            timeout: None,
        }
    }

    /// Set the expected outcome for this test
    pub fn with_expectation(mut self, expected: WptExpectation) -> Self {
        self.expected = expected;
        self
    }

    /// Add a flag to this test
    pub fn with_flag(mut self, flag: impl Into<String>) -> Self {
        self.flags.insert(flag.into());
        self
    }

    /// Set HTML context for selector tests
    pub fn with_html(mut self, html: impl Into<String>) -> Self {
        self.html = Some(html.into());
        self
    }

    /// Set a custom timeout for this test
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Check if this test has a specific flag
    pub fn has_flag(&self, flag: &str) -> bool {
        self.flags.contains(flag)
    }

    /// Check if this test is expected to fail parsing
    pub fn expects_parse_failure(&self) -> bool {
        matches!(self.expected, WptExpectation::ShouldNotParse)
    }
}

/// Statistics for a test run
#[derive(Debug, Clone, Default)]
pub struct WptTestStats {
    /// Number of tests that passed
    pub passed: usize,
    /// Number of tests that failed
    pub failed: usize,
    /// Number of tests that timed out
    pub timeouts: usize,
    /// Number of tests that had errors
    pub errors: usize,
    /// Number of tests that were skipped
    pub skipped: usize,
    /// Total test execution time
    pub total_time: Duration,
}

impl WptTestStats {
    /// Total number of tests run (excluding skipped)
    pub fn total_run(&self) -> usize {
        self.passed + self.failed + self.timeouts + self.errors
    }

    /// Total number of tests
    pub fn total(&self) -> usize {
        self.total_run() + self.skipped
    }

    /// Pass rate as a percentage
    pub fn pass_rate(&self) -> f64 {
        if self.total_run() == 0 {
            0.0
        } else {
            (self.passed as f64 / self.total_run() as f64) * 100.0
        }
    }
}

impl fmt::Display for WptTestStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "WPT Test Results")?;
        writeln!(f, "================")?;
        writeln!(f, "Passed:   {} ({:.1}%)", self.passed, self.pass_rate())?;
        writeln!(f, "Failed:   {}", self.failed)?;
        writeln!(f, "Timeouts: {}", self.timeouts)?;
        writeln!(f, "Errors:   {}", self.errors)?;
        writeln!(f, "Skipped:  {}", self.skipped)?;
        writeln!(f, "Total:    {}", self.total())?;
        writeln!(f, "Time:     {:?}", self.total_time)?;
        Ok(())
    }
}

/// Result of running a single test
#[derive(Debug, Clone)]
pub struct WptTestRun {
    /// The test case that was run
    pub test: WptTestCase,
    /// The result of the test
    pub result: WptTestResult,
    /// Time taken to run the test
    pub duration: Duration,
}

/// Configuration for the test runner
#[derive(Debug, Clone)]
pub struct WptRunnerConfig {
    /// Default timeout for tests
    pub default_timeout: Duration,
    /// Stop on first failure
    pub fail_fast: bool,
    /// Name filter (only run tests matching this pattern)
    pub filter: Option<String>,
    /// Verbose output
    pub verbose: bool,
}

impl Default for WptRunnerConfig {
    fn default() -> Self {
        Self {
            default_timeout: Duration::from_secs(5),
            fail_fast: false,
            filter: None,
            verbose: false,
        }
    }
}

/// The main WPT test runner
pub struct WptTestRunner {
    /// Configuration for the runner
    config: WptRunnerConfig,
    /// Collected test results
    results: Vec<WptTestRun>,
}

impl WptTestRunner {
    /// Create a new test runner with default configuration
    pub fn new() -> Self {
        Self {
            config: WptRunnerConfig::default(),
            results: Vec::new(),
        }
    }

    /// Create a new test runner with custom configuration
    pub fn with_config(config: WptRunnerConfig) -> Self {
        Self {
            config,
            results: Vec::new(),
        }
    }

    /// Set the name filter
    pub fn filter(mut self, pattern: impl Into<String>) -> Self {
        self.config.filter = Some(pattern.into());
        self
    }

    /// Enable verbose output
    pub fn verbose(mut self, verbose: bool) -> Self {
        self.config.verbose = verbose;
        self
    }

    /// Enable fail-fast mode
    pub fn fail_fast(mut self, fail_fast: bool) -> Self {
        self.config.fail_fast = fail_fast;
        self
    }

    /// Check if a test matches the current filter
    fn matches_filter(&self, test: &WptTestCase) -> bool {
        match &self.config.filter {
            Some(pattern) => test.name.contains(pattern),
            None => true,
        }
    }

    /// Run a single test case with the provided test function
    pub fn run_test<F>(&mut self, test: WptTestCase, test_fn: F) -> WptTestResult
    where
        F: FnOnce(&WptTestCase) -> WptTestResult,
    {
        if !self.matches_filter(&test) {
            return WptTestResult::Pass; // Skipped tests count as pass for filtering
        }

        let start = Instant::now();
        let result = test_fn(&test);
        let duration = start.elapsed();

        if self.config.verbose {
            println!("[{}] {} ({:?})", result, test.name, duration);
        }

        self.results.push(WptTestRun {
            test,
            result: result.clone(),
            duration,
        });

        result
    }

    /// Run multiple test cases
    pub fn run_tests<F>(&mut self, tests: Vec<WptTestCase>, mut test_fn: F) -> WptTestStats
    where
        F: FnMut(&WptTestCase) -> WptTestResult,
    {
        let start = Instant::now();
        let mut stats = WptTestStats::default();

        for test in tests {
            if !self.matches_filter(&test) {
                stats.skipped += 1;
                continue;
            }

            let start_test = Instant::now();
            let result = test_fn(&test);
            let duration = start_test.elapsed();

            if self.config.verbose {
                println!("[{}] {} ({:?})", result, test.name, duration);
            }

            self.results.push(WptTestRun {
                test,
                result: result.clone(),
                duration,
            });

            match result {
                WptTestResult::Pass => stats.passed += 1,
                WptTestResult::Fail(_) => stats.failed += 1,
                WptTestResult::Timeout(_) => stats.timeouts += 1,
                WptTestResult::Error(_) => stats.errors += 1,
            }

            if self.config.fail_fast && !result.is_pass() {
                break;
            }
        }

        stats.total_time = start.elapsed();
        stats
    }

    /// Get all test results
    pub fn results(&self) -> &[WptTestRun] {
        &self.results
    }

    /// Get only failed test results
    pub fn failures(&self) -> Vec<&WptTestRun> {
        self.results
            .iter()
            .filter(|r| !r.result.is_pass())
            .collect()
    }

    /// Calculate statistics from current results
    pub fn stats(&self) -> WptTestStats {
        let mut stats = WptTestStats::default();

        for run in &self.results {
            match &run.result {
                WptTestResult::Pass => stats.passed += 1,
                WptTestResult::Fail(_) => stats.failed += 1,
                WptTestResult::Timeout(_) => stats.timeouts += 1,
                WptTestResult::Error(_) => stats.errors += 1,
            }
            stats.total_time += run.duration;
        }

        stats
    }

    /// Clear all collected results
    pub fn clear(&mut self) {
        self.results.clear();
    }
}

impl Default for WptTestRunner {
    fn default() -> Self {
        Self::new()
    }
}

/// Parse WPT test metadata from a comment block
///
/// WPT metadata format:
/// ```css
/// /* @test: test-name
///  * @expect: should-parse
///  * @flags: invalid, slow
///  */
/// ```
pub fn parse_wpt_metadata(comment: &str) -> Option<WptTestCase> {
    let mut name = None;
    let mut expected = WptExpectation::ShouldParse;
    let mut flags = HashSet::new();

    for line in comment.lines() {
        let line = line.trim().trim_start_matches(['*', '/', ' ']);

        if let Some(value) = line.strip_prefix("@test:") {
            name = Some(value.trim().to_string());
        } else if let Some(value) = line.strip_prefix("@expect:") {
            let value = value.trim();
            expected = match value {
                "should-parse" => WptExpectation::ShouldParse,
                "should-not-parse" => WptExpectation::ShouldNotParse,
                _ if value.starts_with("serializes-to:") => {
                    WptExpectation::SerializesTo(value.trim_start_matches("serializes-to:").trim().to_string())
                }
                _ if value.starts_with("matches:") => {
                    let elements: Vec<String> = value
                        .trim_start_matches("matches:")
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect();
                    WptExpectation::MatchesElements(elements)
                }
                _ if value.starts_with("computed:") => {
                    let parts: Vec<&str> = value.trim_start_matches("computed:").splitn(2, '=').collect();
                    if parts.len() == 2 {
                        WptExpectation::ComputedValue(
                            parts[0].trim().to_string(),
                            parts[1].trim().to_string(),
                        )
                    } else {
                        WptExpectation::Custom(value.to_string())
                    }
                }
                _ => WptExpectation::Custom(value.to_string()),
            };
        } else if let Some(value) = line.strip_prefix("@flags:") {
            for flag in value.split(',') {
                flags.insert(flag.trim().to_string());
            }
        }
    }

    name.map(|n| WptTestCase {
        name: n,
        css: String::new(), // Will be filled in later
        expected,
        flags,
        html: None,
        timeout: None,
    })
}

/// A simple test element for selector matching tests
#[derive(Debug, Clone)]
pub struct TestElement {
    /// Element tag name
    pub tag_name: String,
    /// Element ID
    pub id: Option<String>,
    /// Element classes
    pub classes: Vec<String>,
    /// Parent element (if any)
    pub parent: Option<Box<TestElement>>,
    /// Previous sibling (if any)
    pub previous_sibling: Option<Box<TestElement>>,
    /// Child elements
    pub children: Vec<Box<TestElement>>,
}

impl TestElement {
    /// Create a new element with the given tag name
    pub fn new(tag_name: impl Into<String>) -> Self {
        Self {
            tag_name: tag_name.into(),
            id: None,
            classes: Vec::new(),
            parent: None,
            previous_sibling: None,
            children: Vec::new(),
        }
    }

    /// Set the element ID
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Add a class to the element
    pub fn with_class(mut self, class: impl Into<String>) -> Self {
        self.classes.push(class.into());
        self
    }

    /// Add multiple classes to the element
    pub fn with_classes(mut self, classes: impl IntoIterator<Item = impl Into<String>>) -> Self {
        for class in classes {
            self.classes.push(class.into());
        }
        self
    }

    /// Get the element's tag name
    pub fn tag_name(&self) -> &str {
        &self.tag_name
    }

    /// Get the element's ID
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    /// Get the element's classes
    pub fn classes(&self) -> &[String] {
        &self.classes
    }

    /// Get the element's parent
    pub fn parent(&self) -> Option<&TestElement> {
        self.parent.as_deref()
    }

    /// Get the element's previous sibling
    pub fn previous_sibling(&self) -> Option<&TestElement> {
        self.previous_sibling.as_deref()
    }
}

/// Build a simple DOM tree from an HTML-like structure for testing
///
/// Format: `tag#id.class1.class2 > child1, child2`
///
/// Example: `div#main.container > span.text, p.content`
pub fn parse_simple_dom(input: &str) -> Option<TestElement> {
    let input = input.trim();
    if input.is_empty() {
        return None;
    }

    // Parse just the first element for now (simple implementation)
    let mut tag = String::new();
    let mut id = None;
    let mut classes = Vec::new();

    let mut current_part = String::new();
    let mut mode = 'T'; // T = tag, I = id, C = class

    for ch in input.chars() {
        match ch {
            '#' => {
                if mode == 'T' && !current_part.is_empty() {
                    tag = current_part.clone();
                } else if mode == 'C' && !current_part.is_empty() {
                    classes.push(current_part.clone());
                }
                current_part.clear();
                mode = 'I';
            }
            '.' => {
                if mode == 'T' && !current_part.is_empty() {
                    tag = current_part.clone();
                } else if mode == 'I' && !current_part.is_empty() {
                    id = Some(current_part.clone());
                } else if mode == 'C' && !current_part.is_empty() {
                    classes.push(current_part.clone());
                }
                current_part.clear();
                mode = 'C';
            }
            ' ' | '>' | ',' => {
                // Stop at structure characters
                break;
            }
            _ => {
                current_part.push(ch);
            }
        }
    }

    // Handle remaining content
    match mode {
        'T' if !current_part.is_empty() => tag = current_part,
        'I' if !current_part.is_empty() => id = Some(current_part),
        'C' if !current_part.is_empty() => classes.push(current_part),
        _ => {}
    }

    if tag.is_empty() {
        tag = "div".to_string(); // Default tag
    }

    Some(TestElement {
        tag_name: tag,
        id,
        classes,
        parent: None,
        previous_sibling: None,
        children: Vec::new(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wpt_test_result_display() {
        assert_eq!(WptTestResult::Pass.to_string(), "PASS");
        assert_eq!(
            WptTestResult::Fail("mismatch".to_string()).to_string(),
            "FAIL: mismatch"
        );
        assert!(WptTestResult::Timeout(Duration::from_secs(5))
            .to_string()
            .contains("TIMEOUT"));
        assert_eq!(
            WptTestResult::Error("panic".to_string()).to_string(),
            "ERROR: panic"
        );
    }

    #[test]
    fn test_wpt_test_case_builder() {
        let test = WptTestCase::new("test-colors", ".red { color: red; }")
            .with_expectation(WptExpectation::ShouldParse)
            .with_flag("valid")
            .with_flag("css-colors")
            .with_timeout(Duration::from_secs(10));

        assert_eq!(test.name, "test-colors");
        assert_eq!(test.css, ".red { color: red; }");
        assert!(test.has_flag("valid"));
        assert!(test.has_flag("css-colors"));
        assert!(!test.has_flag("invalid"));
        assert_eq!(test.timeout, Some(Duration::from_secs(10)));
    }

    #[test]
    fn test_wpt_test_case_parse_expectations() {
        let test1 = WptTestCase::new("t1", "")
            .with_expectation(WptExpectation::ShouldParse);
        assert!(!test1.expects_parse_failure());

        let test2 = WptTestCase::new("t2", "")
            .with_expectation(WptExpectation::ShouldNotParse);
        assert!(test2.expects_parse_failure());
    }

    #[test]
    fn test_wpt_test_stats_calculations() {
        let mut stats = WptTestStats::default();
        stats.passed = 8;
        stats.failed = 2;
        stats.timeouts = 0;
        stats.errors = 0;
        stats.skipped = 5;

        assert_eq!(stats.total_run(), 10);
        assert_eq!(stats.total(), 15);
        assert!((stats.pass_rate() - 80.0).abs() < 0.01);
    }

    #[test]
    fn test_wpt_test_runner_basic() {
        let mut runner = WptTestRunner::new();

        let test = WptTestCase::new("basic-test", "div { }");
        let result = runner.run_test(test, |_tc| WptTestResult::Pass);

        assert!(result.is_pass());
        assert_eq!(runner.results().len(), 1);
    }

    #[test]
    fn test_wpt_test_runner_filter() {
        let mut runner = WptTestRunner::new().filter("color");

        let tests = vec![
            WptTestCase::new("color-red", ""),
            WptTestCase::new("color-blue", ""),
            WptTestCase::new("layout-flex", ""), // Should be filtered out
        ];

        let stats = runner.run_tests(tests, |_tc| WptTestResult::Pass);

        assert_eq!(stats.passed, 2);
        assert_eq!(stats.skipped, 1);
    }

    #[test]
    fn test_wpt_test_runner_statistics() {
        let mut runner = WptTestRunner::new();

        let tests = vec![
            WptTestCase::new("test1", ""),
            WptTestCase::new("test2", ""),
            WptTestCase::new("test3", ""),
        ];

        let mut counter = 0;
        let stats = runner.run_tests(tests, |_tc| {
            counter += 1;
            if counter == 2 {
                WptTestResult::Fail("expected failure".to_string())
            } else {
                WptTestResult::Pass
            }
        });

        assert_eq!(stats.passed, 2);
        assert_eq!(stats.failed, 1);
        assert!((stats.pass_rate() - 66.66).abs() < 1.0);
    }

    #[test]
    fn test_parse_wpt_metadata() {
        let comment = r#"
        /* @test: color-parsing-hex
         * @expect: should-parse
         * @flags: valid, css-color
         */
        "#;

        let test = parse_wpt_metadata(comment).unwrap();
        assert_eq!(test.name, "color-parsing-hex");
        assert_eq!(test.expected, WptExpectation::ShouldParse);
        assert!(test.has_flag("valid"));
        assert!(test.has_flag("css-color"));
    }

    #[test]
    fn test_parse_wpt_metadata_matches() {
        let comment = "@test: selector-test\n@expect: matches: elem1, elem2, elem3";
        let test = parse_wpt_metadata(comment).unwrap();

        if let WptExpectation::MatchesElements(elements) = test.expected {
            assert_eq!(elements, vec!["elem1", "elem2", "elem3"]);
        } else {
            panic!("Expected MatchesElements");
        }
    }

    #[test]
    fn test_parse_wpt_metadata_computed() {
        let comment = "@test: computed-test\n@expect: computed: color = rgb(255, 0, 0)";
        let test = parse_wpt_metadata(comment).unwrap();

        if let WptExpectation::ComputedValue(prop, val) = test.expected {
            assert_eq!(prop, "color");
            assert_eq!(val, "rgb(255, 0, 0)");
        } else {
            panic!("Expected ComputedValue");
        }
    }

    #[test]
    fn test_test_element_builder() {
        let elem = TestElement::new("div")
            .with_id("main")
            .with_class("container")
            .with_class("active");

        assert_eq!(elem.tag_name(), "div");
        assert_eq!(elem.id(), Some("main"));
        assert_eq!(elem.classes(), &["container", "active"]);
    }

    #[test]
    fn test_parse_simple_dom_basic() {
        let elem = parse_simple_dom("div#main.container").unwrap();
        assert_eq!(elem.tag_name, "div");
        assert_eq!(elem.id, Some("main".to_string()));
        assert_eq!(elem.classes, vec!["container"]);
    }

    #[test]
    fn test_parse_simple_dom_multiple_classes() {
        let elem = parse_simple_dom("span.class1.class2.class3").unwrap();
        assert_eq!(elem.tag_name, "span");
        assert_eq!(elem.id, None);
        assert_eq!(elem.classes, vec!["class1", "class2", "class3"]);
    }

    #[test]
    fn test_parse_simple_dom_id_only() {
        let elem = parse_simple_dom("p#intro").unwrap();
        assert_eq!(elem.tag_name, "p");
        assert_eq!(elem.id, Some("intro".to_string()));
        assert!(elem.classes.is_empty());
    }

    #[test]
    fn test_wpt_result_is_methods() {
        assert!(WptTestResult::Pass.is_pass());
        assert!(!WptTestResult::Pass.is_fail());

        assert!(!WptTestResult::Fail("x".to_string()).is_pass());
        assert!(WptTestResult::Fail("x".to_string()).is_fail());

        assert!(!WptTestResult::Error("x".to_string()).is_pass());
        assert!(!WptTestResult::Error("x".to_string()).is_fail());
    }
}
