//! WPT (Web Platform Tests) Test Harness for CSS Conformance
//!
//! This module provides a complete WPT-style test harness for testing
//! CSS engine conformance. It supports:
//!
//! - CSS parsing tests (valid/invalid syntax)
//! - Selector matching tests (element matching)
//! - Serialization tests (round-trip parsing)
//!
//! # Usage
//!
//! Run all WPT tests:
//! ```bash
//! cargo test --test wpt
//! ```
//!
//! Run specific test categories:
//! ```bash
//! cargo test --test wpt parser
//! cargo test --test wpt selector
//! ```
//!
//! # Structure
//!
//! - `harness`: Core test harness types and runner
//! - `parser_tests`: CSS parsing conformance tests
//! - `selector_tests`: Selector matching conformance tests

pub mod harness;
pub mod parser_tests;
pub mod selector_tests;

pub use harness::{
    WptExpectation, WptRunnerConfig, WptTestCase, WptTestResult, WptTestRunner, WptTestStats,
};

/// Run all WPT tests and return combined statistics
pub fn run_all_wpt_tests() -> WptTestStats {
    println!("Running WPT Test Suite");
    println!("======================\n");

    // Run parser tests
    println!("--- Parser Tests ---");
    let parser_stats = parser_tests::run_all_parser_tests();
    println!("{}", parser_stats);

    // Run selector tests
    println!("\n--- Selector Tests ---");
    let selector_stats = selector_tests::run_all_selector_tests();
    println!("{}", selector_stats);

    // Combine statistics
    let combined = WptTestStats {
        passed: parser_stats.passed + selector_stats.passed,
        failed: parser_stats.failed + selector_stats.failed,
        timeouts: parser_stats.timeouts + selector_stats.timeouts,
        errors: parser_stats.errors + selector_stats.errors,
        skipped: parser_stats.skipped + selector_stats.skipped,
        total_time: parser_stats.total_time + selector_stats.total_time,
    };

    println!("\n=== Combined Results ===");
    println!("{}", combined);

    combined
}

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================
    // Parser Tests
    // ============================================

    #[test]
    fn wpt_parser_valid_element_selector() {
        let test = WptTestCase::new("parser/valid/element", "div { }")
            .with_expectation(WptExpectation::ShouldParse);
        let result = parser_tests::run_parser_test(&test);
        assert!(result.is_pass(), "Result: {}", result);
    }

    #[test]
    fn wpt_parser_valid_class_selector() {
        let test = WptTestCase::new("parser/valid/class", ".button { color: red; }")
            .with_expectation(WptExpectation::ShouldParse);
        let result = parser_tests::run_parser_test(&test);
        assert!(result.is_pass(), "Result: {}", result);
    }

    #[test]
    fn wpt_parser_valid_id_selector() {
        let test = WptTestCase::new("parser/valid/id", "#header { margin: 0; }")
            .with_expectation(WptExpectation::ShouldParse);
        let result = parser_tests::run_parser_test(&test);
        assert!(result.is_pass(), "Result: {}", result);
    }

    #[test]
    fn wpt_parser_valid_multiple_declarations() {
        let test = WptTestCase::new(
            "parser/valid/multiple-declarations",
            "div { color: red; background: blue; padding: 10px; }",
        )
        .with_expectation(WptExpectation::ShouldParse);
        let result = parser_tests::run_parser_test(&test);
        assert!(result.is_pass(), "Result: {}", result);
    }

    #[test]
    fn wpt_parser_valid_multiple_rules() {
        let test = WptTestCase::new(
            "parser/valid/multiple-rules",
            "div { color: red; } .class { color: blue; }",
        )
        .with_expectation(WptExpectation::ShouldParse);
        let result = parser_tests::run_parser_test(&test);
        assert!(result.is_pass(), "Result: {}", result);
    }

    #[test]
    fn wpt_parser_invalid_unbalanced_braces() {
        let test = WptTestCase::new("parser/invalid/unbalanced", "div { color: red;")
            .with_expectation(WptExpectation::ShouldNotParse);
        let result = parser_tests::run_parser_test(&test);
        assert!(result.is_pass(), "Result: {}", result);
    }

    #[test]
    fn wpt_parser_invalid_missing_colon() {
        let test = WptTestCase::new("parser/invalid/missing-colon", "div { color red; }")
            .with_expectation(WptExpectation::ShouldNotParse);
        let result = parser_tests::run_parser_test(&test);
        assert!(result.is_pass(), "Result: {}", result);
    }

    #[test]
    fn wpt_parser_invalid_empty_value() {
        let test = WptTestCase::new("parser/invalid/empty-value", "div { color: ; }")
            .with_expectation(WptExpectation::ShouldNotParse);
        let result = parser_tests::run_parser_test(&test);
        assert!(result.is_pass(), "Result: {}", result);
    }

    #[test]
    fn wpt_parser_all_tests_pass() {
        let tests = parser_tests::create_parser_tests();
        let mut runner = WptTestRunner::new();
        let stats = runner.run_tests(tests, parser_tests::run_parser_test);

        assert_eq!(stats.errors, 0, "No errors expected");
        assert_eq!(stats.timeouts, 0, "No timeouts expected");
        assert!(stats.pass_rate() >= 95.0, "Pass rate: {}", stats.pass_rate());
    }

    // ============================================
    // Selector Tests
    // ============================================

    #[test]
    fn wpt_selector_universal_matches_all() {
        let test = WptTestCase::new("selector/universal", "* { }")
            .with_expectation(WptExpectation::MatchesElements(vec![
                "root".to_string(),
                "header".to_string(),
                "nav".to_string(),
                "main".to_string(),
                "article1".to_string(),
                "article2".to_string(),
                "sidebar".to_string(),
                "footer".to_string(),
            ]));
        let result = selector_tests::run_selector_test(&test);
        assert!(result.is_pass(), "Result: {}", result);
    }

    #[test]
    fn wpt_selector_element_matches() {
        let test = WptTestCase::new("selector/element", "article { }")
            .with_expectation(WptExpectation::MatchesElements(vec![
                "article1".to_string(),
                "article2".to_string(),
            ]));
        let result = selector_tests::run_selector_test(&test);
        assert!(result.is_pass(), "Result: {}", result);
    }

    #[test]
    fn wpt_selector_class_matches() {
        let test = WptTestCase::new("selector/class", ".post { }")
            .with_expectation(WptExpectation::MatchesElements(vec![
                "article1".to_string(),
                "article2".to_string(),
            ]));
        let result = selector_tests::run_selector_test(&test);
        assert!(result.is_pass(), "Result: {}", result);
    }

    #[test]
    fn wpt_selector_id_matches() {
        let test = WptTestCase::new("selector/id", "#header { }")
            .with_expectation(WptExpectation::MatchesElements(vec!["header".to_string()]));
        let result = selector_tests::run_selector_test(&test);
        assert!(result.is_pass(), "Result: {}", result);
    }

    #[test]
    fn wpt_selector_compound_matches() {
        let test = WptTestCase::new("selector/compound", "article.featured { }")
            .with_expectation(WptExpectation::MatchesElements(vec!["article1".to_string()]));
        let result = selector_tests::run_selector_test(&test);
        assert!(result.is_pass(), "Result: {}", result);
    }

    #[test]
    fn wpt_selector_no_match() {
        let test = WptTestCase::new("selector/no-match", ".nonexistent { }")
            .with_expectation(WptExpectation::MatchesElements(vec![]));
        let result = selector_tests::run_selector_test(&test);
        assert!(result.is_pass(), "Result: {}", result);
    }

    #[test]
    fn wpt_selector_case_insensitive() {
        let test = WptTestCase::new("selector/case", "HEADER { }")
            .with_expectation(WptExpectation::MatchesElements(vec!["header".to_string()]));
        let result = selector_tests::run_selector_test(&test);
        assert!(result.is_pass(), "Result: {}", result);
    }

    #[test]
    fn wpt_selector_all_tests_pass() {
        let tests = selector_tests::create_selector_tests();
        let mut runner = WptTestRunner::new();
        let stats = runner.run_tests(tests, selector_tests::run_selector_test);

        assert_eq!(stats.errors, 0, "No errors expected");
        assert_eq!(stats.timeouts, 0, "No timeouts expected");
        assert!(stats.pass_rate() >= 95.0, "Pass rate: {}", stats.pass_rate());
    }

    // ============================================
    // Harness Tests
    // ============================================

    #[test]
    fn wpt_harness_test_case_builder() {
        let test = WptTestCase::new("test-name", "div { }")
            .with_expectation(WptExpectation::ShouldParse)
            .with_flag("valid")
            .with_flag("selector");

        assert_eq!(test.name, "test-name");
        assert_eq!(test.css, "div { }");
        assert!(test.has_flag("valid"));
        assert!(test.has_flag("selector"));
        assert!(!test.has_flag("invalid"));
    }

    #[test]
    fn wpt_harness_runner_filter() {
        let mut runner = WptTestRunner::new().filter("parser");

        let tests = vec![
            WptTestCase::new("parser/test1", ""),
            WptTestCase::new("parser/test2", ""),
            WptTestCase::new("selector/test1", ""),
        ];

        let stats = runner.run_tests(tests, |_| WptTestResult::Pass);

        assert_eq!(stats.passed, 2);
        assert_eq!(stats.skipped, 1);
    }

    #[test]
    fn wpt_harness_runner_fail_fast() {
        let mut runner = WptTestRunner::new().fail_fast(true);

        let tests = vec![
            WptTestCase::new("test1", ""),
            WptTestCase::new("test2", ""),
            WptTestCase::new("test3", ""),
        ];

        let mut count = 0;
        let stats = runner.run_tests(tests, |_| {
            count += 1;
            if count == 2 {
                WptTestResult::Fail("fail".to_string())
            } else {
                WptTestResult::Pass
            }
        });

        // Should stop after the failure
        assert_eq!(stats.passed, 1);
        assert_eq!(stats.failed, 1);
        assert_eq!(stats.total_run(), 2);
    }

    #[test]
    fn wpt_harness_statistics() {
        let mut stats = WptTestStats::default();
        stats.passed = 80;
        stats.failed = 10;
        stats.timeouts = 5;
        stats.errors = 5;
        stats.skipped = 10;

        assert_eq!(stats.total_run(), 100);
        assert_eq!(stats.total(), 110);
        assert!((stats.pass_rate() - 80.0).abs() < 0.01);
    }

    // ============================================
    // Integration Tests
    // ============================================

    #[test]
    fn wpt_integration_full_suite() {
        // Run all tests and verify no catastrophic failures
        let parser_tests = parser_tests::create_parser_tests();
        let selector_tests = selector_tests::create_selector_tests();

        let total_tests = parser_tests.len() + selector_tests.len();
        assert!(total_tests >= 40, "Should have at least 40 tests");

        let mut runner = WptTestRunner::new();

        let parser_stats = runner.run_tests(parser_tests, parser_tests::run_parser_test);
        assert_eq!(parser_stats.errors, 0);
        assert_eq!(parser_stats.timeouts, 0);

        runner.clear();

        let selector_stats = runner.run_tests(selector_tests, selector_tests::run_selector_test);
        assert_eq!(selector_stats.errors, 0);
        assert_eq!(selector_stats.timeouts, 0);
    }
}
