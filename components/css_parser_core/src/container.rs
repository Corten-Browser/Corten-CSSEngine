//! CSS Container Query parsing
//!
//! This module provides parsing support for CSS Container Queries:
//! - `@container` rules for responsive container-based layouts
//! - Size queries: `(min-width: 400px)`, `(width > 400px)`
//! - Style queries: `style(--theme: dark)`
//!
//! # Example
//! ```css
//! @container sidebar (min-width: 400px) {
//!     .card {
//!         display: grid;
//!     }
//! }
//! ```

use crate::{CssRule, ParseError};
use css_types::{
    Comparison, ContainerCondition, Length, LengthUnit, Orientation, SizeCondition, StyleCondition,
};

// ============================================================================
// Container Rule
// ============================================================================

/// A CSS @container rule
#[derive(Debug, Clone, PartialEq)]
pub struct ContainerRule {
    /// Optional container name to query
    pub name: Option<String>,
    /// The container query condition
    pub condition: ContainerCondition,
    /// Rules within the @container block
    pub rules: Vec<CssRule>,
}

impl ContainerRule {
    /// Create a new @container rule
    pub fn new(condition: ContainerCondition, rules: Vec<CssRule>) -> Self {
        ContainerRule {
            name: None,
            condition,
            rules,
        }
    }

    /// Create a @container rule with a named container
    pub fn named(
        name: impl Into<String>,
        condition: ContainerCondition,
        rules: Vec<CssRule>,
    ) -> Self {
        ContainerRule {
            name: Some(name.into()),
            condition,
            rules,
        }
    }
}

// ============================================================================
// Container Query Parser
// ============================================================================

/// Parser for @container rules
pub struct ContainerQueryParser;

impl ContainerQueryParser {
    /// Parse a @container rule from its condition text and body
    pub fn parse(condition_text: &str, body: &str) -> Result<ContainerRule, ParseError> {
        let condition_text = condition_text.trim();

        // Parse optional container name and condition
        let (name, condition_str) = Self::extract_name_and_condition(condition_text)?;

        // Parse the condition
        let condition = Self::parse_condition(condition_str)?;

        // Parse nested rules within the body
        let rules = parse_nested_rules(body)?;

        Ok(ContainerRule {
            name,
            condition,
            rules,
        })
    }

    /// Extract the optional container name and condition from the query text
    fn extract_name_and_condition(input: &str) -> Result<(Option<String>, &str), ParseError> {
        let input = input.trim();

        // If it starts with '(' or 'not' or 'style', there's no name
        if input.starts_with('(')
            || input.to_lowercase().starts_with("not ")
            || input.to_lowercase().starts_with("style(")
        {
            return Ok((None, input));
        }

        // Find the first '(' to locate where condition starts
        if let Some(paren_pos) = input.find('(') {
            let potential_name = input[..paren_pos].trim();

            // Check if it's a keyword like 'not', 'and', 'or'
            let lower = potential_name.to_lowercase();
            if lower == "not" || lower == "and" || lower == "or" {
                return Ok((None, input));
            }

            // It's a container name
            if !potential_name.is_empty() {
                return Ok((Some(potential_name.to_string()), &input[paren_pos..]));
            }
        }

        // No parenthesis found, might be just a name (invalid query) or the whole thing is condition
        Ok((None, input))
    }

    /// Parse a container query condition
    pub fn parse_condition(input: &str) -> Result<ContainerCondition, ParseError> {
        let input = input.trim();

        // Handle 'not' prefix
        if input.to_lowercase().starts_with("not ") {
            let rest = input[4..].trim();
            let inner = Self::parse_condition(rest)?;
            return Ok(ContainerCondition::Not(Box::new(inner)));
        }

        // Handle style() function
        if input.to_lowercase().starts_with("style(") {
            return Self::parse_style_condition(input);
        }

        // Check for 'and' or 'or' operators
        if let Some(result) = Self::try_parse_binary_condition(input)? {
            return Ok(result);
        }

        // Must be a size condition: (feature: value) or (feature op value)
        Self::parse_size_condition(input)
    }

    /// Parse a style() condition: style(--property: value)
    fn parse_style_condition(input: &str) -> Result<ContainerCondition, ParseError> {
        let input = input.trim();

        // Remove "style(" prefix and ")" suffix
        let start = input
            .to_lowercase()
            .find("style(")
            .ok_or_else(|| ParseError::new(1, 1, "Expected 'style(' in condition"))?;
        let content_start = start + 6;

        let end = find_closing_paren(input, content_start)
            .ok_or_else(|| ParseError::new(1, 1, "Missing closing parenthesis in style()"))?;

        let content = &input[content_start..end];

        // Parse property: value
        let colon_pos = content
            .find(':')
            .ok_or_else(|| ParseError::new(1, 1, "Invalid style condition: missing colon"))?;

        let property = content[..colon_pos].trim().to_string();
        let value = content[colon_pos + 1..].trim().to_string();

        Ok(ContainerCondition::Style(StyleCondition {
            property,
            value,
        }))
    }

    /// Try to parse a binary (and/or) condition
    fn try_parse_binary_condition(input: &str) -> Result<Option<ContainerCondition>, ParseError> {
        let input_lower = input.to_lowercase();

        let mut depth = 0;
        let mut last_op_start = 0;
        let mut conditions = Vec::new();
        let mut current_op: Option<&str> = None;

        let bytes = input.as_bytes();
        let mut i = 0;

        while i < bytes.len() {
            match bytes[i] {
                b'(' => depth += 1,
                b')' => depth -= 1,
                _ if depth == 0 => {
                    let rest = &input_lower[i..];
                    if rest.starts_with(" and ") {
                        let condition_text = &input[last_op_start..i];
                        if !condition_text.trim().is_empty() {
                            conditions.push(Self::parse_condition(condition_text.trim())?);
                        }
                        if current_op.is_none() {
                            current_op = Some("and");
                        } else if current_op != Some("and") {
                            return Err(ParseError::new(
                                1,
                                1,
                                "Cannot mix 'and' and 'or' without parentheses",
                            ));
                        }
                        i += 5;
                        last_op_start = i;
                        continue;
                    } else if rest.starts_with(" or ") {
                        let condition_text = &input[last_op_start..i];
                        if !condition_text.trim().is_empty() {
                            conditions.push(Self::parse_condition(condition_text.trim())?);
                        }
                        if current_op.is_none() {
                            current_op = Some("or");
                        } else if current_op != Some("or") {
                            return Err(ParseError::new(
                                1,
                                1,
                                "Cannot mix 'and' and 'or' without parentheses",
                            ));
                        }
                        i += 4;
                        last_op_start = i;
                        continue;
                    }
                }
                _ => {}
            }
            i += 1;
        }

        // Handle the last part
        if current_op.is_some() {
            let condition_text = &input[last_op_start..];
            if !condition_text.trim().is_empty() {
                conditions.push(Self::parse_condition(condition_text.trim())?);
            }

            return match current_op {
                Some("and") => Ok(Some(ContainerCondition::And(conditions))),
                Some("or") => Ok(Some(ContainerCondition::Or(conditions))),
                _ => Ok(None),
            };
        }

        Ok(None)
    }

    /// Parse a size condition: (min-width: 400px), (width > 400px), etc.
    fn parse_size_condition(input: &str) -> Result<ContainerCondition, ParseError> {
        let input = input.trim();

        // Remove outer parentheses if present
        let inner = if input.starts_with('(') && input.ends_with(')') {
            &input[1..input.len() - 1]
        } else {
            input
        };

        let inner = inner.trim();

        // Try to parse as range syntax: width > 400px, height <= 600px
        if let Some(condition) = Self::try_parse_range_syntax(inner)? {
            return Ok(ContainerCondition::Size(condition));
        }

        // Parse as traditional syntax: min-width: 400px, orientation: portrait
        Self::parse_traditional_syntax(inner)
    }

    /// Try to parse range syntax: width > 400px
    fn try_parse_range_syntax(input: &str) -> Result<Option<SizeCondition>, ParseError> {
        // Look for comparison operators
        let operators = [">=", "<=", ">", "<", "="];

        for op in &operators {
            if let Some(pos) = input.find(op) {
                let feature = input[..pos].trim();
                let value = input[pos + op.len()..].trim();

                let comparison = Comparison::parse(op)
                    .map_err(|e| ParseError::new(1, 1, format!("Invalid comparison: {}", e)))?;

                let length = parse_length(value)?;

                return match feature.to_lowercase().as_str() {
                    "width" => Ok(Some(SizeCondition::Width(comparison, length))),
                    "height" => Ok(Some(SizeCondition::Height(comparison, length))),
                    "inline-size" => Ok(Some(SizeCondition::InlineSize(comparison, length))),
                    "block-size" => Ok(Some(SizeCondition::BlockSize(comparison, length))),
                    _ => Err(ParseError::new(
                        1,
                        1,
                        format!("Unknown size feature in range syntax: {}", feature),
                    )),
                };
            }
        }

        Ok(None)
    }

    /// Parse traditional syntax: min-width: 400px, orientation: portrait
    fn parse_traditional_syntax(input: &str) -> Result<ContainerCondition, ParseError> {
        let colon_pos = input
            .find(':')
            .ok_or_else(|| ParseError::new(1, 1, "Invalid container condition: missing colon"))?;

        let feature = input[..colon_pos].trim().to_lowercase();
        let value = input[colon_pos + 1..].trim();

        match feature.as_str() {
            "min-width" => {
                let length = parse_length(value)?;
                Ok(ContainerCondition::Size(SizeCondition::MinWidth(length)))
            }
            "max-width" => {
                let length = parse_length(value)?;
                Ok(ContainerCondition::Size(SizeCondition::MaxWidth(length)))
            }
            "min-height" => {
                let length = parse_length(value)?;
                Ok(ContainerCondition::Size(SizeCondition::MinHeight(length)))
            }
            "max-height" => {
                let length = parse_length(value)?;
                Ok(ContainerCondition::Size(SizeCondition::MaxHeight(length)))
            }
            "width" => {
                let length = parse_length(value)?;
                Ok(ContainerCondition::Size(SizeCondition::Width(
                    Comparison::Equal,
                    length,
                )))
            }
            "height" => {
                let length = parse_length(value)?;
                Ok(ContainerCondition::Size(SizeCondition::Height(
                    Comparison::Equal,
                    length,
                )))
            }
            "aspect-ratio" => {
                let ratio = parse_aspect_ratio(value)?;
                Ok(ContainerCondition::Size(SizeCondition::AspectRatio(ratio)))
            }
            "orientation" => {
                let orientation = Orientation::parse(value)
                    .map_err(|e| ParseError::new(1, 1, format!("Invalid orientation: {}", e)))?;
                Ok(ContainerCondition::Size(SizeCondition::Orientation(
                    orientation,
                )))
            }
            _ => Err(ParseError::new(
                1,
                1,
                format!("Unknown container query feature: {}", feature),
            )),
        }
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Find the closing parenthesis matching an opening one
fn find_closing_paren(s: &str, start: usize) -> Option<usize> {
    let bytes = s.as_bytes();
    let mut depth = 1;

    for (i, &byte) in bytes.iter().enumerate().skip(start) {
        match byte {
            b'(' => depth += 1,
            b')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
    }

    None
}

/// Parse a CSS length value
fn parse_length(value: &str) -> Result<Length, ParseError> {
    let value = value.trim();

    // Find where the number ends and the unit begins
    let mut num_end = 0;
    for (i, ch) in value.char_indices() {
        if ch.is_ascii_digit() || ch == '.' || ch == '-' || ch == '+' {
            num_end = i + ch.len_utf8();
        } else {
            break;
        }
    }

    if num_end == 0 {
        return Err(ParseError::new(
            1,
            1,
            "Length must start with a number".to_string(),
        ));
    }

    let num_str = &value[..num_end];
    let unit_str = &value[num_end..].trim();

    let num = num_str
        .parse::<f32>()
        .map_err(|_| ParseError::new(1, 1, format!("Invalid number: {}", num_str)))?;

    let unit = match unit_str.to_lowercase().as_str() {
        "px" | "" => LengthUnit::Px,
        "em" => LengthUnit::Em,
        "rem" => LengthUnit::Rem,
        "%" => LengthUnit::Percent,
        "vw" => LengthUnit::Vw,
        "vh" => LengthUnit::Vh,
        "cqw" => LengthUnit::Cqw,
        "cqh" => LengthUnit::Cqh,
        "cqi" => LengthUnit::Cqi,
        "cqb" => LengthUnit::Cqb,
        "cqmin" => LengthUnit::Cqmin,
        "cqmax" => LengthUnit::Cqmax,
        _ => return Err(ParseError::new(1, 1, format!("Unknown unit: {}", unit_str))),
    };

    Ok(Length::new(num, unit))
}

/// Parse an aspect ratio value (e.g., "16/9" or "1.777")
fn parse_aspect_ratio(value: &str) -> Result<f32, ParseError> {
    let value = value.trim();

    // Try ratio format: 16/9
    if let Some(slash_pos) = value.find('/') {
        let numerator = value[..slash_pos]
            .trim()
            .parse::<f32>()
            .map_err(|_| ParseError::new(1, 1, "Invalid aspect ratio numerator"))?;
        let denominator = value[slash_pos + 1..]
            .trim()
            .parse::<f32>()
            .map_err(|_| ParseError::new(1, 1, "Invalid aspect ratio denominator"))?;

        if denominator == 0.0 {
            return Err(ParseError::new(
                1,
                1,
                "Aspect ratio denominator cannot be zero",
            ));
        }

        return Ok(numerator / denominator);
    }

    // Try decimal format: 1.777
    value
        .parse::<f32>()
        .map_err(|_| ParseError::new(1, 1, "Invalid aspect ratio value"))
}

/// Parse nested CSS rules (simplified)
fn parse_nested_rules(body: &str) -> Result<Vec<CssRule>, ParseError> {
    let mut rules = Vec::new();
    let body = body.trim();

    if body.is_empty() {
        return Ok(rules);
    }

    let mut start = 0;
    let mut brace_depth = 0;

    for (i, ch) in body.char_indices() {
        match ch {
            '{' => brace_depth += 1,
            '}' => {
                brace_depth -= 1;
                if brace_depth == 0 {
                    let rule_text = &body[start..=i];
                    if let Ok(rule) = parse_style_rule(rule_text) {
                        rules.push(CssRule::Style(rule));
                    }
                    start = i + 1;
                }
            }
            _ => {}
        }
    }

    Ok(rules)
}

/// Parse a style rule (simplified for nested contexts)
fn parse_style_rule(input: &str) -> Result<crate::StyleRule, ParseError> {
    let input = input.trim();

    let open_brace = input
        .find('{')
        .ok_or_else(|| ParseError::new(1, 1, "Expected '{' in rule"))?;
    let close_brace = input
        .rfind('}')
        .ok_or_else(|| ParseError::new(1, 1, "Expected '}' in rule"))?;

    let selector_text = input[..open_brace].trim();
    let declaration_text = &input[open_brace + 1..close_brace];

    let selectors = crate::selector::parse_selector_list(selector_text)?;
    let declarations = crate::declaration::parse_declarations(declaration_text)?;

    Ok(crate::StyleRule {
        selectors,
        declarations,
    })
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ========== Container Rule Tests ==========

    #[test]
    fn test_container_rule_new() {
        let condition =
            ContainerCondition::Size(SizeCondition::MinWidth(Length::new(400.0, LengthUnit::Px)));
        let rule = ContainerRule::new(condition.clone(), vec![]);

        assert!(rule.name.is_none());
        assert_eq!(rule.condition, condition);
        assert!(rule.rules.is_empty());
    }

    #[test]
    fn test_container_rule_named() {
        let condition =
            ContainerCondition::Size(SizeCondition::MinWidth(Length::new(400.0, LengthUnit::Px)));
        let rule = ContainerRule::named("sidebar", condition.clone(), vec![]);

        assert_eq!(rule.name, Some("sidebar".to_string()));
        assert_eq!(rule.condition, condition);
    }

    // ========== Name and Condition Extraction Tests ==========

    #[test]
    fn test_extract_name_and_condition_no_name() {
        let (name, condition) =
            ContainerQueryParser::extract_name_and_condition("(min-width: 400px)").unwrap();
        assert!(name.is_none());
        assert_eq!(condition, "(min-width: 400px)");
    }

    #[test]
    fn test_extract_name_and_condition_with_name() {
        let (name, condition) =
            ContainerQueryParser::extract_name_and_condition("sidebar (min-width: 400px)").unwrap();
        assert_eq!(name, Some("sidebar".to_string()));
        assert_eq!(condition, "(min-width: 400px)");
    }

    #[test]
    fn test_extract_name_and_condition_not_prefix() {
        let (name, condition) =
            ContainerQueryParser::extract_name_and_condition("not (min-width: 400px)").unwrap();
        assert!(name.is_none());
        assert_eq!(condition, "not (min-width: 400px)");
    }

    // ========== Size Condition Parsing Tests ==========

    #[test]
    fn test_parse_min_width() {
        let condition = ContainerQueryParser::parse_condition("(min-width: 400px)").unwrap();
        assert!(matches!(
            condition,
            ContainerCondition::Size(SizeCondition::MinWidth(length)) if length.value() == 400.0
        ));
    }

    #[test]
    fn test_parse_max_width() {
        let condition = ContainerQueryParser::parse_condition("(max-width: 800px)").unwrap();
        assert!(matches!(
            condition,
            ContainerCondition::Size(SizeCondition::MaxWidth(length)) if length.value() == 800.0
        ));
    }

    #[test]
    fn test_parse_min_height() {
        let condition = ContainerQueryParser::parse_condition("(min-height: 300px)").unwrap();
        assert!(matches!(
            condition,
            ContainerCondition::Size(SizeCondition::MinHeight(length)) if length.value() == 300.0
        ));
    }

    #[test]
    fn test_parse_max_height() {
        let condition = ContainerQueryParser::parse_condition("(max-height: 600px)").unwrap();
        assert!(matches!(
            condition,
            ContainerCondition::Size(SizeCondition::MaxHeight(length)) if length.value() == 600.0
        ));
    }

    // ========== Range Syntax Tests ==========

    #[test]
    fn test_parse_width_greater_than() {
        let condition = ContainerQueryParser::parse_condition("(width > 400px)").unwrap();
        assert!(matches!(
            condition,
            ContainerCondition::Size(SizeCondition::Width(Comparison::GreaterThan, length))
            if length.value() == 400.0
        ));
    }

    #[test]
    fn test_parse_width_less_than() {
        let condition = ContainerQueryParser::parse_condition("(width < 800px)").unwrap();
        assert!(matches!(
            condition,
            ContainerCondition::Size(SizeCondition::Width(Comparison::LessThan, length))
            if length.value() == 800.0
        ));
    }

    #[test]
    fn test_parse_width_greater_than_or_equal() {
        let condition = ContainerQueryParser::parse_condition("(width >= 400px)").unwrap();
        assert!(matches!(
            condition,
            ContainerCondition::Size(SizeCondition::Width(Comparison::GreaterThanOrEqual, length))
            if length.value() == 400.0
        ));
    }

    #[test]
    fn test_parse_height_less_than_or_equal() {
        let condition = ContainerQueryParser::parse_condition("(height <= 600px)").unwrap();
        assert!(matches!(
            condition,
            ContainerCondition::Size(SizeCondition::Height(Comparison::LessThanOrEqual, length))
            if length.value() == 600.0
        ));
    }

    #[test]
    fn test_parse_inline_size_comparison() {
        let condition = ContainerQueryParser::parse_condition("(inline-size > 500px)").unwrap();
        assert!(matches!(
            condition,
            ContainerCondition::Size(SizeCondition::InlineSize(Comparison::GreaterThan, length))
            if length.value() == 500.0
        ));
    }

    #[test]
    fn test_parse_block_size_comparison() {
        let condition = ContainerQueryParser::parse_condition("(block-size < 400px)").unwrap();
        assert!(matches!(
            condition,
            ContainerCondition::Size(SizeCondition::BlockSize(Comparison::LessThan, length))
            if length.value() == 400.0
        ));
    }

    // ========== Style Condition Tests ==========

    #[test]
    fn test_parse_style_condition() {
        let condition = ContainerQueryParser::parse_condition("style(--theme: dark)").unwrap();
        if let ContainerCondition::Style(style) = condition {
            assert_eq!(style.property, "--theme");
            assert_eq!(style.value, "dark");
        } else {
            panic!("Expected Style condition");
        }
    }

    #[test]
    fn test_parse_style_condition_with_spaces() {
        let condition = ContainerQueryParser::parse_condition("style( --color : blue )").unwrap();
        if let ContainerCondition::Style(style) = condition {
            assert_eq!(style.property, "--color");
            assert_eq!(style.value, "blue");
        } else {
            panic!("Expected Style condition");
        }
    }

    // ========== Orientation Tests ==========

    #[test]
    fn test_parse_orientation_portrait() {
        let condition = ContainerQueryParser::parse_condition("(orientation: portrait)").unwrap();
        assert!(matches!(
            condition,
            ContainerCondition::Size(SizeCondition::Orientation(Orientation::Portrait))
        ));
    }

    #[test]
    fn test_parse_orientation_landscape() {
        let condition = ContainerQueryParser::parse_condition("(orientation: landscape)").unwrap();
        assert!(matches!(
            condition,
            ContainerCondition::Size(SizeCondition::Orientation(Orientation::Landscape))
        ));
    }

    // ========== Aspect Ratio Tests ==========

    #[test]
    fn test_parse_aspect_ratio_fraction() {
        let condition = ContainerQueryParser::parse_condition("(aspect-ratio: 16/9)").unwrap();
        if let ContainerCondition::Size(SizeCondition::AspectRatio(ratio)) = condition {
            assert!((ratio - 16.0 / 9.0).abs() < 0.001);
        } else {
            panic!("Expected AspectRatio condition");
        }
    }

    #[test]
    fn test_parse_aspect_ratio_decimal() {
        let condition = ContainerQueryParser::parse_condition("(aspect-ratio: 1.5)").unwrap();
        if let ContainerCondition::Size(SizeCondition::AspectRatio(ratio)) = condition {
            assert!((ratio - 1.5).abs() < 0.001);
        } else {
            panic!("Expected AspectRatio condition");
        }
    }

    // ========== Logical Operator Tests ==========

    #[test]
    fn test_parse_not_condition() {
        let condition = ContainerQueryParser::parse_condition("not (min-width: 400px)").unwrap();
        assert!(matches!(condition, ContainerCondition::Not(_)));
    }

    #[test]
    fn test_parse_and_condition() {
        let condition =
            ContainerQueryParser::parse_condition("(min-width: 400px) and (max-width: 800px)")
                .unwrap();
        if let ContainerCondition::And(conditions) = condition {
            assert_eq!(conditions.len(), 2);
        } else {
            panic!("Expected And condition");
        }
    }

    #[test]
    fn test_parse_or_condition() {
        let condition =
            ContainerQueryParser::parse_condition("(min-width: 400px) or (min-height: 300px)")
                .unwrap();
        if let ContainerCondition::Or(conditions) = condition {
            assert_eq!(conditions.len(), 2);
        } else {
            panic!("Expected Or condition");
        }
    }

    #[test]
    fn test_parse_multiple_and_conditions() {
        let condition = ContainerQueryParser::parse_condition(
            "(min-width: 400px) and (max-width: 800px) and (orientation: landscape)",
        )
        .unwrap();
        if let ContainerCondition::And(conditions) = condition {
            assert_eq!(conditions.len(), 3);
        } else {
            panic!("Expected And condition");
        }
    }

    // ========== Container Units in Conditions ==========

    #[test]
    fn test_parse_condition_with_cqw_unit() {
        let condition = ContainerQueryParser::parse_condition("(min-width: 50cqw)").unwrap();
        if let ContainerCondition::Size(SizeCondition::MinWidth(length)) = condition {
            assert_eq!(length.value(), 50.0);
            assert_eq!(length.unit(), LengthUnit::Cqw);
        } else {
            panic!("Expected MinWidth with cqw unit");
        }
    }

    #[test]
    fn test_parse_condition_with_cqh_unit() {
        let condition = ContainerQueryParser::parse_condition("(height > 25cqh)").unwrap();
        if let ContainerCondition::Size(SizeCondition::Height(comp, length)) = condition {
            assert_eq!(comp, Comparison::GreaterThan);
            assert_eq!(length.value(), 25.0);
            assert_eq!(length.unit(), LengthUnit::Cqh);
        } else {
            panic!("Expected Height comparison with cqh unit");
        }
    }

    // ========== Full Container Rule Parsing Tests ==========

    #[test]
    fn test_parse_container_rule_simple() {
        let rule =
            ContainerQueryParser::parse("(min-width: 400px)", ".card { display: grid; }").unwrap();

        assert!(rule.name.is_none());
        assert!(matches!(
            &rule.condition,
            ContainerCondition::Size(SizeCondition::MinWidth(_))
        ));
        assert_eq!(rule.rules.len(), 1);
    }

    #[test]
    fn test_parse_container_rule_with_name() {
        let rule =
            ContainerQueryParser::parse("sidebar (min-width: 400px)", ".widget { padding: 1rem; }")
                .unwrap();

        assert_eq!(rule.name, Some("sidebar".to_string()));
    }

    #[test]
    fn test_parse_container_rule_empty_body() {
        let rule = ContainerQueryParser::parse("(min-width: 400px)", "").unwrap();
        assert!(rule.rules.is_empty());
    }

    // ========== Length Parsing Tests ==========

    #[test]
    fn test_parse_length_px() {
        let length = parse_length("100px").unwrap();
        assert_eq!(length.value(), 100.0);
        assert_eq!(length.unit(), LengthUnit::Px);
    }

    #[test]
    fn test_parse_length_em() {
        let length = parse_length("2.5em").unwrap();
        assert_eq!(length.value(), 2.5);
        assert_eq!(length.unit(), LengthUnit::Em);
    }

    #[test]
    fn test_parse_length_rem() {
        let length = parse_length("1.5rem").unwrap();
        assert_eq!(length.value(), 1.5);
        assert_eq!(length.unit(), LengthUnit::Rem);
    }

    #[test]
    fn test_parse_length_percent() {
        let length = parse_length("50%").unwrap();
        assert_eq!(length.value(), 50.0);
        assert_eq!(length.unit(), LengthUnit::Percent);
    }

    #[test]
    fn test_parse_length_vw() {
        let length = parse_length("100vw").unwrap();
        assert_eq!(length.value(), 100.0);
        assert_eq!(length.unit(), LengthUnit::Vw);
    }

    #[test]
    fn test_parse_length_container_units() {
        assert_eq!(parse_length("50cqw").unwrap().unit(), LengthUnit::Cqw);
        assert_eq!(parse_length("50cqh").unwrap().unit(), LengthUnit::Cqh);
        assert_eq!(parse_length("50cqi").unwrap().unit(), LengthUnit::Cqi);
        assert_eq!(parse_length("50cqb").unwrap().unit(), LengthUnit::Cqb);
        assert_eq!(parse_length("50cqmin").unwrap().unit(), LengthUnit::Cqmin);
        assert_eq!(parse_length("50cqmax").unwrap().unit(), LengthUnit::Cqmax);
    }

    #[test]
    fn test_parse_length_negative() {
        let length = parse_length("-10px").unwrap();
        assert_eq!(length.value(), -10.0);
    }

    // ========== Aspect Ratio Parsing Tests ==========

    #[test]
    fn test_parse_aspect_ratio_16_9() {
        let ratio = parse_aspect_ratio("16/9").unwrap();
        assert!((ratio - 16.0 / 9.0).abs() < 0.001);
    }

    #[test]
    fn test_parse_aspect_ratio_4_3() {
        let ratio = parse_aspect_ratio("4/3").unwrap();
        assert!((ratio - 4.0 / 3.0).abs() < 0.001);
    }

    #[test]
    fn test_parse_aspect_ratio_decimal_value() {
        let ratio = parse_aspect_ratio("1.777").unwrap();
        assert!((ratio - 1.777).abs() < 0.001);
    }

    #[test]
    fn test_parse_aspect_ratio_with_spaces() {
        let ratio = parse_aspect_ratio(" 16 / 9 ").unwrap();
        assert!((ratio - 16.0 / 9.0).abs() < 0.001);
    }

    // ========== Error Cases ==========

    #[test]
    fn test_parse_invalid_feature() {
        let result = ContainerQueryParser::parse_condition("(invalid-feature: 400px)");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_missing_colon() {
        let result = ContainerQueryParser::parse_condition("(min-width 400px)");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_length() {
        let result = ContainerQueryParser::parse_condition("(min-width: abc)");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_orientation() {
        let result = ContainerQueryParser::parse_condition("(orientation: invalid)");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_mixed_and_or() {
        let result = ContainerQueryParser::parse_condition(
            "(min-width: 400px) and (max-width: 800px) or (min-height: 300px)",
        );
        assert!(result.is_err());
    }
}
