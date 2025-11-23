//! WPT (Web Platform Tests) Test Harness for CSS Engine
//!
//! This crate provides a comprehensive WPT-style test harness for CSS conformance testing.
//! It supports CSS parsing tests, selector matching tests, and serialization verification.
//!
//! # Usage
//!
//! Add to your Cargo.toml:
//! ```toml
//! [dev-dependencies]
//! wpt-harness = { path = "../../shared-libs/wpt-harness" }
//! ```
//!
//! Create tests:
//! ```rust,ignore
//! use wpt_harness::{WptTestCase, WptExpectation, WptTestRunner, WptTestResult};
//!
//! #[test]
//! fn test_css_parsing() {
//!     let test = WptTestCase::new("valid-css", "div { color: red; }")
//!         .with_expectation(WptExpectation::ShouldParse);
//!
//!     let result = run_my_parser_test(&test);
//!     assert!(result.is_pass());
//! }
//! ```
//!
//! # Structure
//!
//! - `WptTestRunner`: Main test runner with filtering and statistics
//! - `WptTestCase`: A single test case with CSS, expectation, and flags
//! - `WptExpectation`: Expected outcome (parse success, failure, matches, etc.)
//! - `WptTestResult`: Test result (Pass, Fail, Timeout, Error)
//! - `TestElement`: Simple DOM element for selector testing

mod harness;
mod parser_tests;
mod selector_tests;

pub use harness::{
    parse_simple_dom, parse_wpt_metadata, TestElement, WptExpectation, WptRunnerConfig,
    WptTestCase, WptTestResult, WptTestRun, WptTestRunner, WptTestStats,
};

pub use parser_tests::{create_parser_tests, run_parser_test};
pub use selector_tests::{
    create_default_test_dom, create_selector_tests, run_selector_test, SimpleSelector, TestDom,
    TestDomNode,
};

/// Run all WPT tests and return combined statistics
///
/// This runs both parser and selector test suites and combines the results.
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

    #[test]
    fn test_full_wpt_suite() {
        let stats = run_all_wpt_tests();

        assert_eq!(stats.errors, 0, "WPT suite had {} errors", stats.errors);
        assert_eq!(
            stats.timeouts, 0,
            "WPT suite had {} timeouts",
            stats.timeouts
        );
        assert!(
            stats.pass_rate() >= 95.0,
            "WPT suite pass rate {} < 95%",
            stats.pass_rate()
        );
    }

    #[test]
    fn test_parser_suite() {
        let tests = create_parser_tests();
        let mut runner = WptTestRunner::new();
        let stats = runner.run_tests(tests, run_parser_test);

        assert_eq!(stats.errors, 0);
        assert_eq!(stats.timeouts, 0);
        assert!(stats.pass_rate() >= 95.0);
    }

    #[test]
    fn test_selector_suite() {
        let tests = create_selector_tests();
        let mut runner = WptTestRunner::new();
        let stats = runner.run_tests(tests, run_selector_test);

        assert_eq!(stats.errors, 0);
        assert_eq!(stats.timeouts, 0);
        assert!(stats.pass_rate() >= 95.0);
    }

    #[test]
    fn test_individual_parser_test() {
        let test = WptTestCase::new("test", "div { color: red; }")
            .with_expectation(WptExpectation::ShouldParse);
        assert!(run_parser_test(&test).is_pass());
    }

    #[test]
    fn test_individual_selector_test() {
        let test = WptTestCase::new("test", ".post { }")
            .with_expectation(WptExpectation::MatchesElements(vec![
                "article1".to_string(),
                "article2".to_string(),
            ]));
        assert!(run_selector_test(&test).is_pass());
    }
}
