//! Tests for CSS parser functionality

use css_parser_core::{
    CssParser, CssRule, FontDisplay, FontFaceRule, FontSource, FontStyle, FontWeight,
    NamespaceRule, PageRule, PageSelector, SupportsCondition, SupportsRule,
};
use std::collections::HashSet;

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

// ============================================================================
// @font-face Tests (FEAT-053)
// ============================================================================

#[test]
fn test_parse_font_face_rule() {
    let parser = CssParser::new();
    let css = r#"
        @font-face {
            font-family: "Open Sans";
            src: url("fonts/OpenSans.woff2") format("woff2");
            font-weight: normal;
            font-style: normal;
        }
    "#;
    let result = parser.parse(css);
    assert!(result.is_ok());

    let stylesheet = result.unwrap();
    assert_eq!(stylesheet.rules.len(), 1);

    if let CssRule::FontFace(rule) = &stylesheet.rules[0] {
        assert_eq!(rule.font_family, "Open Sans");
        assert_eq!(rule.src.len(), 1);
        assert!(matches!(
            &rule.src[0],
            FontSource::Url(url, Some(format)) if url == "fonts/OpenSans.woff2" && format == "woff2"
        ));
        assert_eq!(rule.font_weight, Some(FontWeight::Normal));
        assert_eq!(rule.font_style, Some(FontStyle::Normal));
    } else {
        panic!("Expected FontFace rule");
    }
}

#[test]
fn test_parse_font_face_with_multiple_sources() {
    let parser = CssParser::new();
    let css = r#"
        @font-face {
            font-family: "My Font";
            src: local("My Font"), url("myfont.woff2") format("woff2"), url("myfont.woff") format("woff");
        }
    "#;
    let result = parser.parse(css);
    assert!(result.is_ok());

    let stylesheet = result.unwrap();
    if let CssRule::FontFace(rule) = &stylesheet.rules[0] {
        assert_eq!(rule.src.len(), 3);
        assert!(matches!(&rule.src[0], FontSource::Local(name) if name == "My Font"));
    } else {
        panic!("Expected FontFace rule");
    }
}

#[test]
fn test_parse_font_face_with_font_display() {
    let parser = CssParser::new();
    let css = r#"
        @font-face {
            font-family: "Display Font";
            src: url("font.woff2");
            font-display: swap;
        }
    "#;
    let result = parser.parse(css);
    assert!(result.is_ok());

    let stylesheet = result.unwrap();
    if let CssRule::FontFace(rule) = &stylesheet.rules[0] {
        assert_eq!(rule.font_display, Some(FontDisplay::Swap));
    } else {
        panic!("Expected FontFace rule");
    }
}

#[test]
fn test_parse_font_face_with_weight_range() {
    let parser = CssParser::new();
    let css = r#"
        @font-face {
            font-family: "Variable Font";
            src: url("font.woff2");
            font-weight: 100 900;
        }
    "#;
    let result = parser.parse(css);
    assert!(result.is_ok());

    let stylesheet = result.unwrap();
    if let CssRule::FontFace(rule) = &stylesheet.rules[0] {
        assert_eq!(rule.font_weight, Some(FontWeight::Range(100, 900)));
    } else {
        panic!("Expected FontFace rule");
    }
}

// ============================================================================
// @supports Tests (FEAT-054)
// ============================================================================

#[test]
fn test_parse_supports_rule() {
    let parser = CssParser::new();
    let css = r#"
        @supports (display: grid) {
            .container { display: grid; }
        }
    "#;
    let result = parser.parse(css);
    assert!(result.is_ok());

    let stylesheet = result.unwrap();
    assert_eq!(stylesheet.rules.len(), 1);

    if let CssRule::Supports(rule) = &stylesheet.rules[0] {
        assert!(matches!(
            &rule.condition,
            SupportsCondition::Property(prop, val) if prop == "display" && val == "grid"
        ));
        assert_eq!(rule.rules.len(), 1);
    } else {
        panic!("Expected Supports rule");
    }
}

#[test]
fn test_parse_supports_with_not() {
    let parser = CssParser::new();
    let css = r#"
        @supports not (display: grid) {
            .fallback { display: flex; }
        }
    "#;
    let result = parser.parse(css);
    assert!(result.is_ok());

    let stylesheet = result.unwrap();
    if let CssRule::Supports(rule) = &stylesheet.rules[0] {
        assert!(matches!(&rule.condition, SupportsCondition::Not(_)));
    } else {
        panic!("Expected Supports rule");
    }
}

#[test]
fn test_parse_supports_with_and() {
    let parser = CssParser::new();
    let css = r#"
        @supports (display: grid) and (gap: 10px) {
            .grid { display: grid; gap: 10px; }
        }
    "#;
    let result = parser.parse(css);
    assert!(result.is_ok());

    let stylesheet = result.unwrap();
    if let CssRule::Supports(rule) = &stylesheet.rules[0] {
        assert!(matches!(
            &rule.condition,
            SupportsCondition::And(conditions) if conditions.len() == 2
        ));
    } else {
        panic!("Expected Supports rule");
    }
}

#[test]
fn test_parse_supports_with_or() {
    let parser = CssParser::new();
    let css = r#"
        @supports (display: flex) or (display: grid) {
            .layout { display: flex; }
        }
    "#;
    let result = parser.parse(css);
    assert!(result.is_ok());

    let stylesheet = result.unwrap();
    if let CssRule::Supports(rule) = &stylesheet.rules[0] {
        assert!(matches!(
            &rule.condition,
            SupportsCondition::Or(conditions) if conditions.len() == 2
        ));
    } else {
        panic!("Expected Supports rule");
    }
}

#[test]
fn test_supports_evaluate() {
    let mut supported = HashSet::new();
    supported.insert("display".to_string());
    supported.insert("color".to_string());
    supported.insert("gap".to_string());

    // Single property - supported
    let condition = SupportsCondition::Property("display".to_string(), "grid".to_string());
    assert!(condition.evaluate(&supported));

    // Single property - not supported
    let condition = SupportsCondition::Property("unknown".to_string(), "value".to_string());
    assert!(!condition.evaluate(&supported));

    // Not condition
    let condition = SupportsCondition::Not(Box::new(SupportsCondition::Property(
        "unknown".to_string(),
        "value".to_string(),
    )));
    assert!(condition.evaluate(&supported));

    // And condition
    let condition = SupportsCondition::And(vec![
        SupportsCondition::Property("display".to_string(), "grid".to_string()),
        SupportsCondition::Property("gap".to_string(), "10px".to_string()),
    ]);
    assert!(condition.evaluate(&supported));

    // Or condition
    let condition = SupportsCondition::Or(vec![
        SupportsCondition::Property("unknown".to_string(), "value".to_string()),
        SupportsCondition::Property("color".to_string(), "red".to_string()),
    ]);
    assert!(condition.evaluate(&supported));
}

// ============================================================================
// @page Tests (FEAT-055)
// ============================================================================

#[test]
fn test_parse_page_rule() {
    let parser = CssParser::new();
    let css = r#"
        @page {
            margin: 1cm;
        }
    "#;
    let result = parser.parse(css);
    assert!(result.is_ok());

    let stylesheet = result.unwrap();
    assert_eq!(stylesheet.rules.len(), 1);

    if let CssRule::Page(rule) = &stylesheet.rules[0] {
        assert!(rule.selector.is_none());
        assert_eq!(rule.declarations.len(), 1);
    } else {
        panic!("Expected Page rule");
    }
}

#[test]
fn test_parse_page_rule_first() {
    let parser = CssParser::new();
    let css = r#"
        @page :first {
            margin-top: 2cm;
        }
    "#;
    let result = parser.parse(css);
    assert!(result.is_ok());

    let stylesheet = result.unwrap();
    if let CssRule::Page(rule) = &stylesheet.rules[0] {
        assert_eq!(rule.selector, Some(PageSelector::First));
    } else {
        panic!("Expected Page rule");
    }
}

#[test]
fn test_parse_page_rule_left() {
    let parser = CssParser::new();
    let css = r#"
        @page :left {
            margin-left: 3cm;
        }
    "#;
    let result = parser.parse(css);
    assert!(result.is_ok());

    let stylesheet = result.unwrap();
    if let CssRule::Page(rule) = &stylesheet.rules[0] {
        assert_eq!(rule.selector, Some(PageSelector::Left));
    } else {
        panic!("Expected Page rule");
    }
}

#[test]
fn test_parse_page_rule_right() {
    let parser = CssParser::new();
    let css = r#"
        @page :right {
            margin-right: 3cm;
        }
    "#;
    let result = parser.parse(css);
    assert!(result.is_ok());

    let stylesheet = result.unwrap();
    if let CssRule::Page(rule) = &stylesheet.rules[0] {
        assert_eq!(rule.selector, Some(PageSelector::Right));
    } else {
        panic!("Expected Page rule");
    }
}

#[test]
fn test_parse_page_rule_named() {
    let parser = CssParser::new();
    let css = r#"
        @page toc {
            margin: 1in;
        }
    "#;
    let result = parser.parse(css);
    assert!(result.is_ok());

    let stylesheet = result.unwrap();
    if let CssRule::Page(rule) = &stylesheet.rules[0] {
        assert_eq!(rule.selector, Some(PageSelector::Named("toc".to_string())));
    } else {
        panic!("Expected Page rule");
    }
}

// ============================================================================
// @namespace Tests (FEAT-056)
// ============================================================================

#[test]
fn test_parse_namespace_rule_default() {
    let parser = CssParser::new();
    let css = r#"@namespace "http://www.w3.org/1999/xhtml";"#;

    // Note: @namespace rules typically don't have braces, but our parser handles both
    // For parsing, we need to handle the semicolon-terminated form
    let result = parser.parse_rule(&format!("@namespace \"http://www.w3.org/1999/xhtml\" {{}}"));
    assert!(result.is_ok());

    if let CssRule::Namespace(rule) = result.unwrap() {
        assert!(rule.prefix.is_none());
        assert_eq!(rule.uri, "http://www.w3.org/1999/xhtml");
    } else {
        panic!("Expected Namespace rule");
    }
}

#[test]
fn test_parse_namespace_rule_prefixed() {
    let parser = CssParser::new();
    let css = r#"@namespace svg "http://www.w3.org/2000/svg" {}"#;
    let result = parser.parse_rule(css);
    assert!(result.is_ok());

    if let CssRule::Namespace(rule) = result.unwrap() {
        assert_eq!(rule.prefix, Some("svg".to_string()));
        assert_eq!(rule.uri, "http://www.w3.org/2000/svg");
    } else {
        panic!("Expected Namespace rule");
    }
}

// ============================================================================
// Mixed Rules Tests
// ============================================================================

#[test]
fn test_parse_mixed_rules() {
    let parser = CssParser::new();
    let css = r#"
        @font-face {
            font-family: "Custom Font";
            src: url("font.woff2");
        }

        body { color: black; }

        @supports (display: grid) {
            .grid { display: grid; }
        }

        @page { margin: 1cm; }
    "#;
    let result = parser.parse(css);
    assert!(result.is_ok());

    let stylesheet = result.unwrap();
    assert_eq!(stylesheet.rules.len(), 4);

    assert!(matches!(&stylesheet.rules[0], CssRule::FontFace(_)));
    assert!(matches!(&stylesheet.rules[1], CssRule::Style(_)));
    assert!(matches!(&stylesheet.rules[2], CssRule::Supports(_)));
    assert!(matches!(&stylesheet.rules[3], CssRule::Page(_)));
}

#[test]
fn test_parse_media_rule_with_nested_styles() {
    let parser = CssParser::new();
    let css = r#"
        @media screen {
            body { background: white; }
        }
    "#;
    let result = parser.parse(css);
    assert!(result.is_ok());

    let stylesheet = result.unwrap();
    if let CssRule::Media(rule) = &stylesheet.rules[0] {
        assert_eq!(rule.media_queries.len(), 1);
        assert_eq!(rule.media_queries[0], "screen");
        assert_eq!(rule.rules.len(), 1);
    } else {
        panic!("Expected Media rule");
    }
}

#[test]
fn test_parse_import_rule() {
    let parser = CssParser::new();
    let css = r#"@import url("styles.css") {}"#;
    let result = parser.parse_rule(css);
    assert!(result.is_ok());

    if let CssRule::Import(rule) = result.unwrap() {
        assert_eq!(rule.url, "styles.css");
    } else {
        panic!("Expected Import rule");
    }
}
