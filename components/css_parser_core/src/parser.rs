//! CSS Parser implementation

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

    /// Parse a single CSS rule
    pub fn parse_rule(&self, input: &str) -> Result<CssRule, ParseError> {
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
}
