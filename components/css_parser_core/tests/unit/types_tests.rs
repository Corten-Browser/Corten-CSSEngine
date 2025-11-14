//! Tests for CSS parser types

use css_parser_core::{Origin, ParseError, Stylesheet, CssRule, StyleRule};

#[test]
fn test_parse_error_display() {
    let error = ParseError::new(1, 5, "unexpected token");
    let error_str = format!("{}", error);
    assert!(error_str.contains("line 1"));
    assert!(error_str.contains("column 5"));
    assert!(error_str.contains("unexpected token"));
}

#[test]
fn test_parse_error_debug() {
    let error = ParseError::new(10, 15, "syntax error");
    assert_eq!(error.line, 10);
    assert_eq!(error.column, 15);
    assert_eq!(error.message, "syntax error");
}

#[test]
fn test_empty_stylesheet_creation() {
    let stylesheet = Stylesheet {
        rules: vec![],
        origin: Origin::Author,
    };
    assert_eq!(stylesheet.rules.len(), 0);
    assert_eq!(stylesheet.origin, Origin::Author);
}

#[test]
fn test_stylesheet_with_rules() {
    let rule = CssRule::Style(StyleRule {
        selectors: vec![],
        declarations: vec![],
    });

    let stylesheet = Stylesheet {
        rules: vec![rule],
        origin: Origin::UserAgent,
    };

    assert_eq!(stylesheet.rules.len(), 1);
    assert!(matches!(stylesheet.rules[0], CssRule::Style(_)));
}

#[test]
fn test_style_rule_creation() {
    let rule = StyleRule {
        selectors: vec![],
        declarations: vec![],
    };

    assert_eq!(rule.selectors.len(), 0);
    assert_eq!(rule.declarations.len(), 0);
}
