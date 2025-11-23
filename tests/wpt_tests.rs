//! WPT Integration Tests for CSS Engine
//!
//! This file provides the cargo test integration for the WPT test harness.
//! Run with: `cargo test --test wpt_tests`
//!
//! Or run individual test suites:
//! ```bash
//! cargo test --test wpt_tests parser
//! cargo test --test wpt_tests selector
//! cargo test --test wpt_tests harness
//! ```

mod wpt;

use wpt::{WptExpectation, WptTestCase, WptTestResult, WptTestRunner, WptTestStats};
use wpt::parser_tests;
use wpt::selector_tests;

// ============================================
// Test Entry Point
// ============================================

#[test]
fn wpt_full_suite() {
    let stats = wpt::run_all_wpt_tests();

    // Verify no errors or timeouts
    assert_eq!(stats.errors, 0, "WPT suite had {} errors", stats.errors);
    assert_eq!(stats.timeouts, 0, "WPT suite had {} timeouts", stats.timeouts);

    // Verify high pass rate
    assert!(
        stats.pass_rate() >= 95.0,
        "WPT suite pass rate {} < 95%",
        stats.pass_rate()
    );

    println!("\nWPT Test Suite: {} passed, {} failed, {}% pass rate",
        stats.passed, stats.failed, stats.pass_rate());
}

// ============================================
// Parser Test Suite
// ============================================

#[test]
fn wpt_parser_suite() {
    let tests = parser_tests::create_parser_tests();
    let mut runner = WptTestRunner::new();
    let stats = runner.run_tests(tests, parser_tests::run_parser_test);

    assert_eq!(stats.errors, 0);
    assert_eq!(stats.timeouts, 0);
    assert!(stats.pass_rate() >= 95.0, "Pass rate: {}", stats.pass_rate());

    println!("Parser Tests: {} passed, {} failed", stats.passed, stats.failed);
}

// ============================================
// Selector Test Suite
// ============================================

#[test]
fn wpt_selector_suite() {
    let tests = selector_tests::create_selector_tests();
    let mut runner = WptTestRunner::new();
    let stats = runner.run_tests(tests, selector_tests::run_selector_test);

    assert_eq!(stats.errors, 0);
    assert_eq!(stats.timeouts, 0);
    assert!(stats.pass_rate() >= 95.0, "Pass rate: {}", stats.pass_rate());

    println!("Selector Tests: {} passed, {} failed", stats.passed, stats.failed);
}

// ============================================
// Individual Parser Tests
// ============================================

#[test]
fn parser_element_selector() {
    let test = WptTestCase::new("parser/element", "div { color: red; }")
        .with_expectation(WptExpectation::ShouldParse);
    assert!(parser_tests::run_parser_test(&test).is_pass());
}

#[test]
fn parser_class_selector() {
    let test = WptTestCase::new("parser/class", ".button { padding: 10px; }")
        .with_expectation(WptExpectation::ShouldParse);
    assert!(parser_tests::run_parser_test(&test).is_pass());
}

#[test]
fn parser_id_selector() {
    let test = WptTestCase::new("parser/id", "#main { margin: 0; }")
        .with_expectation(WptExpectation::ShouldParse);
    assert!(parser_tests::run_parser_test(&test).is_pass());
}

#[test]
fn parser_compound_selector() {
    let test = WptTestCase::new("parser/compound", "div.container#main { display: flex; }")
        .with_expectation(WptExpectation::ShouldParse);
    assert!(parser_tests::run_parser_test(&test).is_pass());
}

#[test]
fn parser_multiple_rules() {
    let test = WptTestCase::new("parser/multi-rule", "h1 { font-size: 2em; } p { line-height: 1.5; }")
        .with_expectation(WptExpectation::ShouldParse);
    assert!(parser_tests::run_parser_test(&test).is_pass());
}

#[test]
fn parser_invalid_unbalanced() {
    let test = WptTestCase::new("parser/invalid/unbalanced", "div { color: red")
        .with_expectation(WptExpectation::ShouldNotParse);
    assert!(parser_tests::run_parser_test(&test).is_pass());
}

#[test]
fn parser_invalid_empty() {
    let test = WptTestCase::new("parser/invalid/empty", "")
        .with_expectation(WptExpectation::ShouldNotParse);
    assert!(parser_tests::run_parser_test(&test).is_pass());
}

// ============================================
// Individual Selector Tests
// ============================================

#[test]
fn selector_universal() {
    let test = WptTestCase::new("selector/universal", "* { }")
        .with_expectation(WptExpectation::MatchesElements(vec![
            "root".into(), "header".into(), "nav".into(), "main".into(),
            "article1".into(), "article2".into(), "sidebar".into(), "footer".into(),
        ]));
    assert!(selector_tests::run_selector_test(&test).is_pass());
}

#[test]
fn selector_element() {
    let test = WptTestCase::new("selector/element", "article { }")
        .with_expectation(WptExpectation::MatchesElements(vec!["article1".into(), "article2".into()]));
    assert!(selector_tests::run_selector_test(&test).is_pass());
}

#[test]
fn selector_class() {
    let test = WptTestCase::new("selector/class", ".featured { }")
        .with_expectation(WptExpectation::MatchesElements(vec!["article1".into()]));
    assert!(selector_tests::run_selector_test(&test).is_pass());
}

#[test]
fn selector_id() {
    let test = WptTestCase::new("selector/id", "#sidebar { }")
        .with_expectation(WptExpectation::MatchesElements(vec!["sidebar".into()]));
    assert!(selector_tests::run_selector_test(&test).is_pass());
}

#[test]
fn selector_compound() {
    let test = WptTestCase::new("selector/compound", "article.post { }")
        .with_expectation(WptExpectation::MatchesElements(vec!["article1".into(), "article2".into()]));
    assert!(selector_tests::run_selector_test(&test).is_pass());
}

#[test]
fn selector_no_match() {
    let test = WptTestCase::new("selector/no-match", ".nonexistent { }")
        .with_expectation(WptExpectation::MatchesElements(vec![]));
    assert!(selector_tests::run_selector_test(&test).is_pass());
}

// ============================================
// Harness Tests
// ============================================

#[test]
fn harness_test_case_creation() {
    let test = WptTestCase::new("test", "div { }")
        .with_expectation(WptExpectation::ShouldParse)
        .with_flag("valid");

    assert_eq!(test.name, "test");
    assert_eq!(test.css, "div { }");
    assert!(test.has_flag("valid"));
}

#[test]
fn harness_runner_basic() {
    let mut runner = WptTestRunner::new();
    let test = WptTestCase::new("test", "");
    let result = runner.run_test(test, |_| WptTestResult::Pass);
    assert!(result.is_pass());
}

#[test]
fn harness_runner_filtering() {
    let mut runner = WptTestRunner::new().filter("foo");
    let tests = vec![
        WptTestCase::new("foo/test1", ""),
        WptTestCase::new("bar/test1", ""),
    ];
    let stats = runner.run_tests(tests, |_| WptTestResult::Pass);
    assert_eq!(stats.passed, 1);
    assert_eq!(stats.skipped, 1);
}

#[test]
fn harness_stats_calculation() {
    let mut stats = WptTestStats::default();
    stats.passed = 90;
    stats.failed = 10;

    assert_eq!(stats.total_run(), 100);
    assert!((stats.pass_rate() - 90.0).abs() < 0.01);
}
