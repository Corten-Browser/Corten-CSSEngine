//! Integration tests for CSS parser

use css_parser_core::{CssParser, CssRule, Origin};

#[test]
fn test_parse_empty_stylesheet() {
    let parser = CssParser::new();
    let result = parser.parse("");
    assert!(result.is_ok());

    let stylesheet = result.unwrap();
    assert_eq!(stylesheet.rules.len(), 0);
    assert_eq!(stylesheet.origin, Origin::Author);
}

#[test]
fn test_parse_simple_rule() {
    let parser = CssParser::new();
    let css = "div { color: red; }";
    let result = parser.parse(css);

    assert!(result.is_ok());
    let stylesheet = result.unwrap();
    assert_eq!(stylesheet.rules.len(), 1);

    match &stylesheet.rules[0] {
        CssRule::Style(rule) => {
            assert_eq!(rule.selectors.len(), 1);
            assert_eq!(rule.declarations.len(), 1);
        }
        _ => panic!("Expected StyleRule"),
    }
}

#[test]
fn test_parse_multiple_rules() {
    let parser = CssParser::new();
    let css = r#"
        div { color: red; }
        .class { margin: 10px; }
        #id { padding: 5px; }
    "#;

    let result = parser.parse(css);
    assert!(result.is_ok());

    let stylesheet = result.unwrap();
    assert_eq!(stylesheet.rules.len(), 3);
}

#[test]
fn test_parse_rule_with_multiple_declarations() {
    let parser = CssParser::new();
    let css = "div { color: red; margin: 10px; padding: 5px; }";

    let result = parser.parse(css);
    assert!(result.is_ok());

    let stylesheet = result.unwrap();
    assert_eq!(stylesheet.rules.len(), 1);

    match &stylesheet.rules[0] {
        CssRule::Style(rule) => {
            assert_eq!(rule.declarations.len(), 3);
        }
        _ => panic!("Expected StyleRule"),
    }
}

#[test]
fn test_parse_invalid_css() {
    let parser = CssParser::new();
    let css = "div { color: }"; // Missing value

    let result = parser.parse(css);
    assert!(result.is_err());
}

#[test]
fn test_parse_single_rule() {
    let parser = CssParser::new();
    let css = "div { color: red; }";

    let result = parser.parse_rule(css);
    assert!(result.is_ok());

    match result.unwrap() {
        CssRule::Style(_) => {} // Success
        _ => panic!("Expected StyleRule"),
    }
}
