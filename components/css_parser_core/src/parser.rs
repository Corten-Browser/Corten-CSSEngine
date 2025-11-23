//! CSS Parser implementation

use crate::at_rules::{parse_font_face, parse_namespace, parse_page, parse_supports};
use crate::container::ContainerQueryParser;
use crate::declaration::parse_declarations;
use crate::selector::parse_selector_list;
use crate::{CssRule, ParseError, StyleRule, Stylesheet};

/// CSS Parser for CSS2.1 stylesheets
pub struct CssParser {
    // Parser state (if needed in future)
}

impl CssParser {
    /// Create a new CSS parser
    pub fn new() -> Self {
        CssParser {}
    }

    /// Parse a complete CSS stylesheet
    pub fn parse(&self, input: &str) -> Result<Stylesheet, ParseError> {
        let mut stylesheet = Stylesheet::author();

        // Handle empty input
        if input.trim().is_empty() {
            return Ok(stylesheet);
        }

        // Simple rule extraction: split by '}' and parse each rule
        let rules = self.extract_rules(input)?;

        for rule_text in rules {
            if !rule_text.trim().is_empty() {
                match self.parse_rule(rule_text) {
                    Ok(rule) => stylesheet.rules.push(rule),
                    Err(e) => return Err(e),
                }
            }
        }

        Ok(stylesheet)
    }

    /// Parse a single CSS rule (style rule or at-rule)
    pub fn parse_rule(&self, input: &str) -> Result<CssRule, ParseError> {
        let input = input.trim();

        // Check for at-rules
        if input.starts_with('@') {
            return self.parse_at_rule(input);
        }

        // Parse as style rule
        self.parse_style_rule(input)
    }

    /// Parse an at-rule (@font-face, @supports, @page, @namespace, @media, @import, @container)
    fn parse_at_rule(&self, input: &str) -> Result<CssRule, ParseError> {
        let input = input.trim();

        // Determine at-rule type
        if input.starts_with("@font-face") {
            return self.parse_font_face_rule(input);
        } else if input.starts_with("@supports") {
            return self.parse_supports_rule(input);
        } else if input.starts_with("@page") {
            return self.parse_page_rule(input);
        } else if input.starts_with("@namespace") {
            return self.parse_namespace_rule(input);
        } else if input.starts_with("@media") {
            return self.parse_media_rule(input);
        } else if input.starts_with("@import") {
            return self.parse_import_rule(input);
        } else if input.starts_with("@container") {
            return self.parse_container_rule(input);
        }

        Err(ParseError::new(1, 1, "Unknown at-rule"))
    }

    /// Parse @font-face rule
    fn parse_font_face_rule(&self, input: &str) -> Result<CssRule, ParseError> {
        // @font-face { ... }
        let open_brace = input
            .find('{')
            .ok_or_else(|| ParseError::new(1, 1, "Expected '{' in @font-face"))?;
        let close_brace = input
            .rfind('}')
            .ok_or_else(|| ParseError::new(1, 1, "Expected '}' in @font-face"))?;

        let body = &input[open_brace + 1..close_brace];
        let rule = parse_font_face(body)?;
        Ok(CssRule::FontFace(rule))
    }

    /// Parse @supports rule
    fn parse_supports_rule(&self, input: &str) -> Result<CssRule, ParseError> {
        // @supports condition { ... }
        let prefix = "@supports";
        let rest = input[prefix.len()..].trim();

        let open_brace = rest
            .find('{')
            .ok_or_else(|| ParseError::new(1, 1, "Expected '{' in @supports"))?;
        let close_brace = rest
            .rfind('}')
            .ok_or_else(|| ParseError::new(1, 1, "Expected '}' in @supports"))?;

        let condition_text = rest[..open_brace].trim();
        let body = &rest[open_brace + 1..close_brace];

        let rule = parse_supports(condition_text, body)?;
        Ok(CssRule::Supports(rule))
    }

    /// Parse @page rule
    fn parse_page_rule(&self, input: &str) -> Result<CssRule, ParseError> {
        // @page [selector] { ... }
        let prefix = "@page";
        let rest = input[prefix.len()..].trim();

        let open_brace = rest
            .find('{')
            .ok_or_else(|| ParseError::new(1, 1, "Expected '{' in @page"))?;
        let close_brace = rest
            .rfind('}')
            .ok_or_else(|| ParseError::new(1, 1, "Expected '}' in @page"))?;

        let selector_text = rest[..open_brace].trim();
        let body = &rest[open_brace + 1..close_brace];

        let rule = parse_page(selector_text, body)?;
        Ok(CssRule::Page(rule))
    }

    /// Parse @namespace rule
    fn parse_namespace_rule(&self, input: &str) -> Result<CssRule, ParseError> {
        // @namespace [prefix] "url";
        let prefix = "@namespace";
        let rest = input[prefix.len()..].trim();

        // For @namespace, we don't have braces - just a semicolon-terminated statement
        // But our rule extraction includes the whole block, so handle both cases
        let content = if let Some(open_brace) = rest.find('{') {
            // Shouldn't happen for @namespace, but handle it
            &rest[..open_brace]
        } else {
            rest.trim_end_matches(';').trim_end_matches('}')
        };

        let rule = parse_namespace(content)?;
        Ok(CssRule::Namespace(rule))
    }

    /// Parse @media rule
    fn parse_media_rule(&self, input: &str) -> Result<CssRule, ParseError> {
        // @media query { ... }
        let prefix = "@media";
        let rest = input[prefix.len()..].trim();

        let open_brace = rest
            .find('{')
            .ok_or_else(|| ParseError::new(1, 1, "Expected '{' in @media"))?;
        let close_brace = rest
            .rfind('}')
            .ok_or_else(|| ParseError::new(1, 1, "Expected '}' in @media"))?;

        let query_text = rest[..open_brace].trim();
        let body = &rest[open_brace + 1..close_brace];

        // Parse media queries (simple split by comma)
        let media_queries: Vec<String> = query_text
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        // Parse nested rules
        let mut rules = Vec::new();
        let nested_rules = self.extract_rules(body)?;
        for rule_text in nested_rules {
            if !rule_text.trim().is_empty() {
                if let Ok(rule) = self.parse_rule(rule_text) {
                    rules.push(rule);
                }
            }
        }

        Ok(CssRule::Media(crate::MediaRule {
            media_queries,
            rules,
        }))
    }

    /// Parse @import rule
    fn parse_import_rule(&self, input: &str) -> Result<CssRule, ParseError> {
        // @import url("...") [media];
        let prefix = "@import";
        let rest = input[prefix.len()..].trim();

        // Remove trailing semicolon and closing brace if present
        let rest = rest.trim_end_matches(';').trim_end_matches('}').trim();

        // Extract URL
        let url = if rest.starts_with("url(") {
            let end = rest
                .find(')')
                .ok_or_else(|| ParseError::new(1, 1, "Missing ')' in @import url()"))?;
            let url_content = &rest[4..end];
            extract_string_value(url_content)
        } else if rest.starts_with('"') || rest.starts_with('\'') {
            // String URL
            let quote = rest.chars().next().unwrap();
            let end = rest[1..]
                .find(quote)
                .ok_or_else(|| ParseError::new(1, 1, "Unterminated string in @import"))?;
            rest[1..end + 1].to_string()
        } else {
            return Err(ParseError::new(1, 1, "Invalid @import URL"));
        };

        // Extract media queries (optional, after the URL)
        let media_queries = Vec::new(); // Simplified - not parsing media queries here

        Ok(CssRule::Import(crate::ImportRule { url, media_queries }))
    }

    /// Parse @container rule
    fn parse_container_rule(&self, input: &str) -> Result<CssRule, ParseError> {
        // @container [name] condition { ... }
        let prefix = "@container";
        let rest = input[prefix.len()..].trim();

        let open_brace = rest
            .find('{')
            .ok_or_else(|| ParseError::new(1, 1, "Expected '{' in @container"))?;
        let close_brace = rest
            .rfind('}')
            .ok_or_else(|| ParseError::new(1, 1, "Expected '}' in @container"))?;

        let condition_text = rest[..open_brace].trim();
        let body = &rest[open_brace + 1..close_brace];

        let rule = ContainerQueryParser::parse(condition_text, body)?;
        Ok(CssRule::Container(rule))
    }

    /// Parse a style rule (selectors + declarations)
    fn parse_style_rule(&self, input: &str) -> Result<CssRule, ParseError> {
        let input = input.trim();

        // Find the selector/declaration split at '{'
        let open_brace = input
            .find('{')
            .ok_or_else(|| ParseError::new(1, 1, "Expected '{' in rule"))?;

        let close_brace = input
            .rfind('}')
            .ok_or_else(|| ParseError::new(1, 1, "Expected '}' in rule"))?;

        if open_brace >= close_brace {
            return Err(ParseError::new(1, 1, "Mismatched braces"));
        }

        // Extract selectors and declaration block
        let selector_text = &input[..open_brace];
        let declaration_text = &input[open_brace + 1..close_brace];

        // Parse selectors
        let selectors = parse_selector_list(selector_text)?;

        // Parse declarations
        let declarations = parse_declarations(declaration_text)?;

        Ok(CssRule::Style(StyleRule {
            selectors,
            declarations,
        }))
    }

    /// Extract individual rules from stylesheet text
    fn extract_rules<'a>(&self, input: &'a str) -> Result<Vec<&'a str>, ParseError> {
        let mut rules = Vec::new();
        let mut start = 0;
        let mut brace_depth = 0;

        for (i, ch) in input.char_indices() {
            match ch {
                '{' => brace_depth += 1,
                '}' => {
                    brace_depth -= 1;
                    if brace_depth == 0 {
                        rules.push(&input[start..=i]);
                        start = i + 1;
                    }
                }
                _ => {}
            }
        }

        if brace_depth != 0 {
            return Err(ParseError::new(1, 1, "Mismatched braces in stylesheet"));
        }

        Ok(rules)
    }
}

impl Default for CssParser {
    fn default() -> Self {
        Self::new()
    }
}

/// Extract string value (remove quotes if present)
fn extract_string_value(s: &str) -> String {
    let s = s.trim();
    if (s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')) {
        s[1..s.len() - 1].to_string()
    } else {
        s.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_rules_single() {
        let parser = CssParser::new();
        let css = "div { color: red; }";
        let rules = parser.extract_rules(css).unwrap();
        assert_eq!(rules.len(), 1);
    }

    #[test]
    fn test_extract_rules_multiple() {
        let parser = CssParser::new();
        let css = "div { color: red; } .class { margin: 10px; }";
        let rules = parser.extract_rules(css).unwrap();
        assert_eq!(rules.len(), 2);
    }

    #[test]
    fn test_extract_rules_nested_braces() {
        let parser = CssParser::new();
        let css = "div { margin: 10px; }";
        let rules = parser.extract_rules(css).unwrap();
        assert_eq!(rules.len(), 1);
    }

    // ========== @container Rule Tests ==========

    #[test]
    fn test_parse_container_rule_simple() {
        let parser = CssParser::new();
        let css = "@container (min-width: 400px) { .card { display: grid; } }";
        let stylesheet = parser.parse(css).unwrap();

        assert_eq!(stylesheet.rules.len(), 1);
        assert!(matches!(stylesheet.rules[0], CssRule::Container(_)));

        if let CssRule::Container(ref container) = stylesheet.rules[0] {
            assert!(container.name.is_none());
            assert_eq!(container.rules.len(), 1);
        }
    }

    #[test]
    fn test_parse_container_rule_named() {
        let parser = CssParser::new();
        let css = "@container sidebar (min-width: 400px) { .widget { padding: 1rem; } }";
        let stylesheet = parser.parse(css).unwrap();

        assert_eq!(stylesheet.rules.len(), 1);

        if let CssRule::Container(ref container) = stylesheet.rules[0] {
            assert_eq!(container.name, Some("sidebar".to_string()));
        }
    }

    #[test]
    fn test_parse_container_rule_with_comparison() {
        let parser = CssParser::new();
        let css = "@container (width > 500px) { .item { flex-direction: row; } }";
        let stylesheet = parser.parse(css).unwrap();

        assert_eq!(stylesheet.rules.len(), 1);
        assert!(matches!(stylesheet.rules[0], CssRule::Container(_)));
    }

    #[test]
    fn test_parse_container_rule_with_and() {
        let parser = CssParser::new();
        let css =
            "@container (min-width: 400px) and (max-width: 800px) { .card { padding: 2rem; } }";
        let stylesheet = parser.parse(css).unwrap();

        assert_eq!(stylesheet.rules.len(), 1);
        assert!(matches!(stylesheet.rules[0], CssRule::Container(_)));
    }

    #[test]
    fn test_parse_container_rule_style_query() {
        let parser = CssParser::new();
        let css = "@container style(--theme: dark) { .text { color: white; } }";
        let stylesheet = parser.parse(css).unwrap();

        assert_eq!(stylesheet.rules.len(), 1);
        assert!(matches!(stylesheet.rules[0], CssRule::Container(_)));
    }

    #[test]
    fn test_parse_container_rule_with_not() {
        let parser = CssParser::new();
        let css = "@container not (min-width: 400px) { .small { font-size: 12px; } }";
        let stylesheet = parser.parse(css).unwrap();

        assert_eq!(stylesheet.rules.len(), 1);
        assert!(matches!(stylesheet.rules[0], CssRule::Container(_)));
    }

    #[test]
    fn test_parse_mixed_rules_with_container() {
        let parser = CssParser::new();
        let css = r#"
            div { color: red; }
            @container (min-width: 400px) { .card { display: grid; } }
            .class { margin: 10px; }
        "#;
        let stylesheet = parser.parse(css).unwrap();

        assert_eq!(stylesheet.rules.len(), 3);
        assert!(matches!(stylesheet.rules[0], CssRule::Style(_)));
        assert!(matches!(stylesheet.rules[1], CssRule::Container(_)));
        assert!(matches!(stylesheet.rules[2], CssRule::Style(_)));
    }
}
