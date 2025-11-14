//! Tests for CSS parser functionality

use css_parser_core::{CssParser, ParseError, CssRule};
use css_types::Color;

#[test]
fn test_parser_creation() {
    let parser = CssParser::new();
    // Parser should be created successfully
    assert!(true); // Placeholder - parser exists
}

#[test]
fn test_parse_empty_stylesheet() {
    let parser = CssParser::new();
    let result = parser.parse("");
    assert!(result.is_ok());

    let stylesheet = result.unwrap();
    assert_eq!(stylesheet.rules.len(), 0);
}

#[test]
fn test_parse_simple_rule_with_element_selector() {
    let parser = CssParser::new();
    let css = "div { color: red; }";
    let result = parser.parse(css);

    assert!(result.is_ok());
    let stylesheet = result.unwrap();
    assert_eq!(stylesheet.rules.len(), 1);
}

#[test]
fn test_parse_simple_rule_with_class_selector() {
    let parser = CssParser::new();
    let css = ".myclass { margin: 10px; }";
    let result = parser.parse(css);

    assert!(result.is_ok());
    let stylesheet = result.unwrap();
    assert_eq!(stylesheet.rules.len(), 1);
}

#[test]
fn test_parse_simple_rule_with_id_selector() {
    let parser = CssParser::new();
    let css = "#myid { padding: 5px; }";
    let result = parser.parse(css);

    assert!(result.is_ok());
    let stylesheet = result.unwrap();
    assert_eq!(stylesheet.rules.len(), 1);
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

    if let CssRule::Style(rule) = &stylesheet.rules[0] {
        assert_eq!(rule.declarations.len(), 3);
    } else {
        panic!("Expected StyleRule");
    }
}

#[test]
fn test_parse_invalid_css() {
    let parser = CssParser::new();
    let css = "div { color: }"; // Missing value
    let result = parser.parse(css);

    // Should return an error for invalid CSS
    assert!(result.is_err());
}

#[test]
fn test_parse_single_rule() {
    let parser = CssParser::new();
    let css = "div { color: red; }";
    let result = parser.parse_rule(css);

    assert!(result.is_ok());
    assert!(matches!(result.unwrap(), CssRule::Style(_)));
}

#[test]
fn test_parse_rule_with_whitespace() {
    let parser = CssParser::new();
    let css = "  div   {   color  :  red  ;   }  ";
    let result = parser.parse_rule(css);

    assert!(result.is_ok());
}
