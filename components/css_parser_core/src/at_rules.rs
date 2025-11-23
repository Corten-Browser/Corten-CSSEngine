//! CSS At-Rules parsing
//!
//! This module provides parsing support for CSS at-rules:
//! - `@font-face` - Custom font declarations
//! - `@supports` - Feature query parsing and evaluation
//! - `@page` - Print styles
//! - `@namespace` - XML namespace declarations

use crate::{ParseError, PropertyDeclaration};
use std::collections::HashSet;

// ============================================================================
// @font-face Rule (FEAT-053)
// ============================================================================

/// Font weight values for @font-face rules
#[derive(Debug, Clone, PartialEq, Default)]
pub enum FontWeight {
    /// Normal font weight (400)
    #[default]
    Normal,
    /// Bold font weight (700)
    Bold,
    /// Numeric weight (100-900)
    Numeric(u16),
    /// Weight range for variable fonts
    Range(u16, u16),
}

/// Font style values for @font-face rules
#[derive(Debug, Clone, PartialEq, Default)]
pub enum FontStyle {
    /// Normal (upright) style
    #[default]
    Normal,
    /// Italic style
    Italic,
    /// Oblique style with optional angle
    Oblique(Option<f32>),
}

/// Font display strategy for @font-face rules
#[derive(Debug, Clone, PartialEq, Default)]
pub enum FontDisplay {
    /// Block period is short, swap period is infinite
    #[default]
    Auto,
    /// Block period is extremely short, swap period is infinite
    Block,
    /// Block period is extremely short, swap period is short
    Swap,
    /// Block period is extremely short, no swap period
    Fallback,
    /// No block period, no swap period
    Optional,
}

/// Unicode range for @font-face rules
#[derive(Debug, Clone, PartialEq)]
pub struct UnicodeRange {
    /// Start of the unicode range (inclusive)
    pub start: u32,
    /// End of the unicode range (inclusive)
    pub end: u32,
}

impl UnicodeRange {
    /// Create a new unicode range
    pub fn new(start: u32, end: u32) -> Self {
        UnicodeRange { start, end }
    }

    /// Create a single codepoint range
    pub fn single(codepoint: u32) -> Self {
        UnicodeRange {
            start: codepoint,
            end: codepoint,
        }
    }

    /// Check if a codepoint is within this range
    pub fn contains(&self, codepoint: u32) -> bool {
        codepoint >= self.start && codepoint <= self.end
    }
}

/// Font source for @font-face rules
#[derive(Debug, Clone, PartialEq)]
pub enum FontSource {
    /// URL source with optional format hint
    Url(String, Option<String>),
    /// Local font name
    Local(String),
}

/// @font-face rule for custom font declarations
#[derive(Debug, Clone, PartialEq)]
pub struct FontFaceRule {
    /// The font-family name
    pub font_family: String,
    /// Font sources (url or local)
    pub src: Vec<FontSource>,
    /// Font weight (optional)
    pub font_weight: Option<FontWeight>,
    /// Font style (optional)
    pub font_style: Option<FontStyle>,
    /// Font display strategy (optional)
    pub font_display: Option<FontDisplay>,
    /// Unicode range (optional)
    pub unicode_range: Option<Vec<UnicodeRange>>,
}

impl FontFaceRule {
    /// Create a new @font-face rule with required fields
    pub fn new(font_family: String, src: Vec<FontSource>) -> Self {
        FontFaceRule {
            font_family,
            src,
            font_weight: None,
            font_style: None,
            font_display: None,
            unicode_range: None,
        }
    }

    /// Set font weight
    pub fn with_weight(mut self, weight: FontWeight) -> Self {
        self.font_weight = Some(weight);
        self
    }

    /// Set font style
    pub fn with_style(mut self, style: FontStyle) -> Self {
        self.font_style = Some(style);
        self
    }

    /// Set font display
    pub fn with_display(mut self, display: FontDisplay) -> Self {
        self.font_display = Some(display);
        self
    }

    /// Set unicode range
    pub fn with_unicode_range(mut self, ranges: Vec<UnicodeRange>) -> Self {
        self.unicode_range = Some(ranges);
        self
    }
}

// ============================================================================
// @supports Rule (FEAT-054)
// ============================================================================

/// Condition for @supports rules
#[derive(Debug, Clone, PartialEq)]
pub enum SupportsCondition {
    /// Property-value support check: (property: value)
    Property(String, String),
    /// Negation: not (condition)
    Not(Box<SupportsCondition>),
    /// Conjunction: (condition) and (condition)
    And(Vec<SupportsCondition>),
    /// Disjunction: (condition) or (condition)
    Or(Vec<SupportsCondition>),
    /// Selector support check: selector(selector)
    Selector(String),
}

impl SupportsCondition {
    /// Evaluate whether this condition is supported
    ///
    /// # Arguments
    /// * `supported_properties` - Set of supported CSS property names
    ///
    /// # Returns
    /// `true` if the condition is met, `false` otherwise
    pub fn evaluate(&self, supported_properties: &HashSet<String>) -> bool {
        match self {
            SupportsCondition::Property(property, _value) => {
                // Check if the property is in the supported set
                // In a real implementation, we'd also validate the value
                supported_properties.contains(property)
            }
            SupportsCondition::Not(condition) => !condition.evaluate(supported_properties),
            SupportsCondition::And(conditions) => {
                conditions.iter().all(|c| c.evaluate(supported_properties))
            }
            SupportsCondition::Or(conditions) => {
                conditions.iter().any(|c| c.evaluate(supported_properties))
            }
            SupportsCondition::Selector(_selector) => {
                // Selector support checking would require a selector parser
                // For now, assume basic selectors are supported
                true
            }
        }
    }
}

/// @supports rule for feature queries
#[derive(Debug, Clone, PartialEq)]
pub struct SupportsRule {
    /// The supports condition
    pub condition: SupportsCondition,
    /// Rules within the @supports block
    pub rules: Vec<crate::CssRule>,
}

impl SupportsRule {
    /// Create a new @supports rule
    pub fn new(condition: SupportsCondition, rules: Vec<crate::CssRule>) -> Self {
        SupportsRule { condition, rules }
    }

    /// Evaluate whether this @supports rule applies
    pub fn evaluate(&self, supported_properties: &HashSet<String>) -> bool {
        self.condition.evaluate(supported_properties)
    }
}

// ============================================================================
// @page Rule (FEAT-055)
// ============================================================================

/// Page selector for @page rules
#[derive(Debug, Clone, PartialEq)]
pub enum PageSelector {
    /// First page: @page :first
    First,
    /// Left pages: @page :left
    Left,
    /// Right pages: @page :right
    Right,
    /// Blank pages: @page :blank
    Blank,
    /// Named page: @page name
    Named(String),
}

/// @page rule for print styles
#[derive(Debug, Clone, PartialEq)]
pub struct PageRule {
    /// Optional page selector (:first, :left, :right, :blank, or named)
    pub selector: Option<PageSelector>,
    /// Declarations for the page
    pub declarations: Vec<PropertyDeclaration>,
}

impl PageRule {
    /// Create a new @page rule without a selector
    pub fn new(declarations: Vec<PropertyDeclaration>) -> Self {
        PageRule {
            selector: None,
            declarations,
        }
    }

    /// Create a new @page rule with a selector
    pub fn with_selector(selector: PageSelector, declarations: Vec<PropertyDeclaration>) -> Self {
        PageRule {
            selector: Some(selector),
            declarations,
        }
    }
}

// ============================================================================
// @namespace Rule (FEAT-056)
// ============================================================================

/// @namespace rule for XML namespace declarations
#[derive(Debug, Clone, PartialEq)]
pub struct NamespaceRule {
    /// Optional namespace prefix
    pub prefix: Option<String>,
    /// Namespace URI
    pub uri: String,
}

impl NamespaceRule {
    /// Create a default namespace (no prefix)
    pub fn default_namespace(uri: String) -> Self {
        NamespaceRule { prefix: None, uri }
    }

    /// Create a prefixed namespace
    pub fn prefixed(prefix: String, uri: String) -> Self {
        NamespaceRule {
            prefix: Some(prefix),
            uri,
        }
    }
}

// ============================================================================
// Parsing Functions
// ============================================================================

/// Parse a @font-face rule from its body text
pub fn parse_font_face(body: &str) -> Result<FontFaceRule, ParseError> {
    let body = body.trim();

    let mut font_family: Option<String> = None;
    let mut src: Vec<FontSource> = Vec::new();
    let mut font_weight: Option<FontWeight> = None;
    let mut font_style: Option<FontStyle> = None;
    let mut font_display: Option<FontDisplay> = None;
    let mut unicode_range: Option<Vec<UnicodeRange>> = None;

    // Parse declarations
    for decl_text in body.split(';') {
        let decl_text = decl_text.trim();
        if decl_text.is_empty() {
            continue;
        }

        let parts: Vec<&str> = decl_text.splitn(2, ':').collect();
        if parts.len() != 2 {
            continue;
        }

        let property = parts[0].trim().to_lowercase();
        let value = parts[1].trim();

        match property.as_str() {
            "font-family" => {
                font_family = Some(parse_font_family_value(value));
            }
            "src" => {
                src = parse_font_src(value)?;
            }
            "font-weight" => {
                font_weight = Some(parse_font_weight(value)?);
            }
            "font-style" => {
                font_style = Some(parse_font_style(value)?);
            }
            "font-display" => {
                font_display = Some(parse_font_display(value)?);
            }
            "unicode-range" => {
                unicode_range = Some(parse_unicode_range(value)?);
            }
            _ => {
                // Ignore unknown properties
            }
        }
    }

    let font_family =
        font_family.ok_or_else(|| ParseError::new(1, 1, "Missing font-family in @font-face"))?;

    if src.is_empty() {
        return Err(ParseError::new(1, 1, "Missing src in @font-face"));
    }

    Ok(FontFaceRule {
        font_family,
        src,
        font_weight,
        font_style,
        font_display,
        unicode_range,
    })
}

/// Parse font-family value (strip quotes if present)
fn parse_font_family_value(value: &str) -> String {
    let value = value.trim();
    // Remove surrounding quotes if present
    if (value.starts_with('"') && value.ends_with('"'))
        || (value.starts_with('\'') && value.ends_with('\''))
    {
        value[1..value.len() - 1].to_string()
    } else {
        value.to_string()
    }
}

/// Parse font src value
fn parse_font_src(value: &str) -> Result<Vec<FontSource>, ParseError> {
    let mut sources = Vec::new();

    // Split by comma for multiple sources
    for src_part in value.split(',') {
        let src_part = src_part.trim();

        if src_part.starts_with("url(") {
            // Parse url() function
            let (url, format) = parse_url_source(src_part)?;
            sources.push(FontSource::Url(url, format));
        } else if src_part.starts_with("local(") {
            // Parse local() function
            let name = parse_local_source(src_part)?;
            sources.push(FontSource::Local(name));
        }
    }

    Ok(sources)
}

/// Parse url() font source
fn parse_url_source(value: &str) -> Result<(String, Option<String>), ParseError> {
    // Find the url(...) part
    let url_start = value
        .find("url(")
        .ok_or_else(|| ParseError::new(1, 1, "Invalid url() syntax"))?;
    let url_content_start = url_start + 4;

    // Find the matching closing paren
    let url_end = find_closing_paren(value, url_content_start)
        .ok_or_else(|| ParseError::new(1, 1, "Missing closing parenthesis in url()"))?;

    let url = extract_string_value(&value[url_content_start..url_end]);

    // Check for format() hint
    let format = value.find("format(").and_then(|format_start| {
        let format_content_start = format_start + 7;
        find_closing_paren(value, format_content_start)
            .map(|format_end| extract_string_value(&value[format_content_start..format_end]))
    });

    Ok((url, format))
}

/// Parse local() font source
fn parse_local_source(value: &str) -> Result<String, ParseError> {
    let start = value
        .find("local(")
        .ok_or_else(|| ParseError::new(1, 1, "Invalid local() syntax"))?;
    let content_start = start + 6;

    let end = find_closing_paren(value, content_start)
        .ok_or_else(|| ParseError::new(1, 1, "Missing closing parenthesis in local()"))?;

    Ok(extract_string_value(&value[content_start..end]))
}

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

/// Extract string value (remove quotes if present)
fn extract_string_value(s: &str) -> String {
    let s = s.trim();
    if (s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')) {
        s[1..s.len() - 1].to_string()
    } else {
        s.to_string()
    }
}

/// Parse font-weight value
fn parse_font_weight(value: &str) -> Result<FontWeight, ParseError> {
    let value = value.trim().to_lowercase();

    match value.as_str() {
        "normal" => Ok(FontWeight::Normal),
        "bold" => Ok(FontWeight::Bold),
        _ => {
            // Try to parse as number or range
            if value.contains(' ') {
                // Range format: "100 900"
                let parts: Vec<&str> = value.split_whitespace().collect();
                if parts.len() == 2 {
                    let start = parts[0]
                        .parse::<u16>()
                        .map_err(|_| ParseError::new(1, 1, "Invalid font-weight value"))?;
                    let end = parts[1]
                        .parse::<u16>()
                        .map_err(|_| ParseError::new(1, 1, "Invalid font-weight value"))?;
                    return Ok(FontWeight::Range(start, end));
                }
            }

            // Single numeric value
            let weight = value
                .parse::<u16>()
                .map_err(|_| ParseError::new(1, 1, "Invalid font-weight value"))?;
            Ok(FontWeight::Numeric(weight))
        }
    }
}

/// Parse font-style value
fn parse_font_style(value: &str) -> Result<FontStyle, ParseError> {
    let value = value.trim().to_lowercase();

    if value == "normal" {
        Ok(FontStyle::Normal)
    } else if value == "italic" {
        Ok(FontStyle::Italic)
    } else if value.starts_with("oblique") {
        // Parse optional angle
        let rest = value.strip_prefix("oblique").unwrap().trim();
        if rest.is_empty() {
            Ok(FontStyle::Oblique(None))
        } else {
            let angle_str = rest.trim_end_matches("deg");
            let angle = angle_str
                .parse::<f32>()
                .map_err(|_| ParseError::new(1, 1, "Invalid oblique angle"))?;
            Ok(FontStyle::Oblique(Some(angle)))
        }
    } else {
        Err(ParseError::new(1, 1, "Invalid font-style value"))
    }
}

/// Parse font-display value
fn parse_font_display(value: &str) -> Result<FontDisplay, ParseError> {
    match value.trim().to_lowercase().as_str() {
        "auto" => Ok(FontDisplay::Auto),
        "block" => Ok(FontDisplay::Block),
        "swap" => Ok(FontDisplay::Swap),
        "fallback" => Ok(FontDisplay::Fallback),
        "optional" => Ok(FontDisplay::Optional),
        _ => Err(ParseError::new(1, 1, "Invalid font-display value")),
    }
}

/// Parse unicode-range value
fn parse_unicode_range(value: &str) -> Result<Vec<UnicodeRange>, ParseError> {
    let mut ranges = Vec::new();

    for range_str in value.split(',') {
        let range_str = range_str.trim();
        if range_str.is_empty() {
            continue;
        }

        // Parse U+XXXX or U+XXXX-YYYY format
        let range_str = range_str
            .strip_prefix("U+")
            .or_else(|| range_str.strip_prefix("u+"))
            .ok_or_else(|| ParseError::new(1, 1, "Invalid unicode-range format"))?;

        if range_str.contains('-') {
            // Range format: XXXX-YYYY
            let parts: Vec<&str> = range_str.split('-').collect();
            if parts.len() == 2 {
                let start = u32::from_str_radix(parts[0], 16)
                    .map_err(|_| ParseError::new(1, 1, "Invalid unicode codepoint"))?;
                let end = u32::from_str_radix(parts[1], 16)
                    .map_err(|_| ParseError::new(1, 1, "Invalid unicode codepoint"))?;
                ranges.push(UnicodeRange::new(start, end));
            }
        } else if range_str.contains('?') {
            // Wildcard format: XX?? (expands to range)
            let base = range_str.replace('?', "0");
            let end_base = range_str.replace('?', "F");
            let start = u32::from_str_radix(&base, 16)
                .map_err(|_| ParseError::new(1, 1, "Invalid unicode range wildcard"))?;
            let end = u32::from_str_radix(&end_base, 16)
                .map_err(|_| ParseError::new(1, 1, "Invalid unicode range wildcard"))?;
            ranges.push(UnicodeRange::new(start, end));
        } else {
            // Single codepoint
            let codepoint = u32::from_str_radix(range_str, 16)
                .map_err(|_| ParseError::new(1, 1, "Invalid unicode codepoint"))?;
            ranges.push(UnicodeRange::single(codepoint));
        }
    }

    Ok(ranges)
}

/// Parse a @supports rule condition
pub fn parse_supports_condition(input: &str) -> Result<SupportsCondition, ParseError> {
    let input = input.trim();

    // Handle 'not' prefix
    if input.to_lowercase().starts_with("not ") {
        let rest = input[4..].trim();
        let inner = parse_supports_condition(rest)?;
        return Ok(SupportsCondition::Not(Box::new(inner)));
    }

    // Handle selector() function
    if input.to_lowercase().starts_with("selector(") {
        let end = find_closing_paren(input, 9)
            .ok_or_else(|| ParseError::new(1, 1, "Missing closing paren in selector()"))?;
        let selector = input[9..end].trim().to_string();
        return Ok(SupportsCondition::Selector(selector));
    }

    // Check for 'and' or 'or' operators
    if let Some(result) = try_parse_binary_condition(input)? {
        return Ok(result);
    }

    // Must be a property condition: (property: value)
    parse_property_condition(input)
}

/// Try to parse a binary (and/or) condition
fn try_parse_binary_condition(input: &str) -> Result<Option<SupportsCondition>, ParseError> {
    let input_lower = input.to_lowercase();

    // Find 'and' or 'or' operators outside parentheses
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
                // Check for operators at depth 0
                let rest = &input_lower[i..];
                if rest.starts_with(" and ") {
                    let condition_text = &input[last_op_start..i];
                    if !condition_text.trim().is_empty() {
                        conditions.push(parse_supports_condition(condition_text.trim())?);
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
                        conditions.push(parse_supports_condition(condition_text.trim())?);
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
            conditions.push(parse_supports_condition(condition_text.trim())?);
        }

        return match current_op {
            Some("and") => Ok(Some(SupportsCondition::And(conditions))),
            Some("or") => Ok(Some(SupportsCondition::Or(conditions))),
            _ => Ok(None),
        };
    }

    Ok(None)
}

/// Parse a property condition: (property: value)
fn parse_property_condition(input: &str) -> Result<SupportsCondition, ParseError> {
    let input = input.trim();

    // Remove outer parentheses if present
    let inner = if input.starts_with('(') && input.ends_with(')') {
        &input[1..input.len() - 1]
    } else {
        input
    };

    // Split by colon
    let colon_pos = inner
        .find(':')
        .ok_or_else(|| ParseError::new(1, 1, "Invalid supports condition: missing colon"))?;

    let property = inner[..colon_pos].trim().to_string();
    let value = inner[colon_pos + 1..].trim().to_string();

    Ok(SupportsCondition::Property(property, value))
}

/// Parse a @supports rule
pub fn parse_supports(condition_text: &str, body: &str) -> Result<SupportsRule, ParseError> {
    let condition = parse_supports_condition(condition_text)?;

    // Parse rules within the body
    let rules = parse_nested_rules(body)?;

    Ok(SupportsRule::new(condition, rules))
}

/// Parse nested CSS rules (simplified)
fn parse_nested_rules(body: &str) -> Result<Vec<crate::CssRule>, ParseError> {
    // For now, just parse style rules
    // A full implementation would recursively call the main parser
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
                        rules.push(crate::CssRule::Style(rule));
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

/// Parse a @page rule
pub fn parse_page(selector_text: &str, body: &str) -> Result<PageRule, ParseError> {
    let selector = if selector_text.trim().is_empty() {
        None
    } else {
        Some(parse_page_selector(selector_text)?)
    };

    let declarations = crate::declaration::parse_declarations(body)?;

    Ok(PageRule {
        selector,
        declarations,
    })
}

/// Parse a page selector (:first, :left, :right, :blank, or named)
fn parse_page_selector(input: &str) -> Result<PageSelector, ParseError> {
    let input = input.trim();

    if input.starts_with(':') {
        match input.to_lowercase().as_str() {
            ":first" => Ok(PageSelector::First),
            ":left" => Ok(PageSelector::Left),
            ":right" => Ok(PageSelector::Right),
            ":blank" => Ok(PageSelector::Blank),
            _ => Err(ParseError::new(
                1,
                1,
                format!("Unknown page pseudo-class: {}", input),
            )),
        }
    } else {
        Ok(PageSelector::Named(input.to_string()))
    }
}

/// Parse a @namespace rule
pub fn parse_namespace(input: &str) -> Result<NamespaceRule, ParseError> {
    let input = input.trim();

    // Remove trailing semicolon if present
    let input = input.trim_end_matches(';').trim();

    // Split into parts
    let parts: Vec<&str> = input.split_whitespace().collect();

    match parts.len() {
        1 => {
            // Default namespace: @namespace "uri"
            let uri = extract_string_value(parts[0]);
            Ok(NamespaceRule::default_namespace(uri))
        }
        2 => {
            // Prefixed namespace: @namespace prefix "uri"
            let prefix = parts[0].to_string();
            let uri = extract_string_value(parts[1]);
            Ok(NamespaceRule::prefixed(prefix, uri))
        }
        _ => Err(ParseError::new(1, 1, "Invalid @namespace syntax")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== @font-face tests ==========

    #[test]
    fn test_parse_font_face_basic() {
        let body = r#"
            font-family: "MyFont";
            src: url("font.woff2") format("woff2");
        "#;

        let rule = parse_font_face(body).unwrap();
        assert_eq!(rule.font_family, "MyFont");
        assert_eq!(rule.src.len(), 1);
        assert!(matches!(
            &rule.src[0],
            FontSource::Url(url, Some(format)) if url == "font.woff2" && format == "woff2"
        ));
    }

    #[test]
    fn test_parse_font_face_with_weight() {
        let body = r#"
            font-family: "MyFont";
            src: url("font.woff2");
            font-weight: bold;
        "#;

        let rule = parse_font_face(body).unwrap();
        assert_eq!(rule.font_weight, Some(FontWeight::Bold));
    }

    #[test]
    fn test_parse_font_face_with_numeric_weight() {
        let body = r#"
            font-family: "MyFont";
            src: url("font.woff2");
            font-weight: 600;
        "#;

        let rule = parse_font_face(body).unwrap();
        assert_eq!(rule.font_weight, Some(FontWeight::Numeric(600)));
    }

    #[test]
    fn test_parse_font_face_with_style() {
        let body = r#"
            font-family: "MyFont";
            src: url("font.woff2");
            font-style: italic;
        "#;

        let rule = parse_font_face(body).unwrap();
        assert_eq!(rule.font_style, Some(FontStyle::Italic));
    }

    #[test]
    fn test_parse_font_face_with_display() {
        let body = r#"
            font-family: "MyFont";
            src: url("font.woff2");
            font-display: swap;
        "#;

        let rule = parse_font_face(body).unwrap();
        assert_eq!(rule.font_display, Some(FontDisplay::Swap));
    }

    #[test]
    fn test_parse_font_face_with_local() {
        let body = r#"
            font-family: "MyFont";
            src: local("Arial"), url("font.woff2");
        "#;

        let rule = parse_font_face(body).unwrap();
        assert_eq!(rule.src.len(), 2);
        assert!(matches!(&rule.src[0], FontSource::Local(name) if name == "Arial"));
    }

    #[test]
    fn test_parse_font_face_missing_family() {
        let body = r#"src: url("font.woff2");"#;
        let result = parse_font_face(body);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_unicode_range() {
        let ranges = parse_unicode_range("U+0000-00FF, U+0131, U+02??").unwrap();
        assert_eq!(ranges.len(), 3);
        assert_eq!(ranges[0].start, 0x0000);
        assert_eq!(ranges[0].end, 0x00FF);
        assert_eq!(ranges[1].start, 0x0131);
        assert_eq!(ranges[1].end, 0x0131);
        assert_eq!(ranges[2].start, 0x0200);
        assert_eq!(ranges[2].end, 0x02FF);
    }

    // ========== @supports tests ==========

    #[test]
    fn test_parse_supports_property() {
        let condition = parse_supports_condition("(display: grid)").unwrap();
        assert!(matches!(
            condition,
            SupportsCondition::Property(prop, val) if prop == "display" && val == "grid"
        ));
    }

    #[test]
    fn test_parse_supports_not() {
        let condition = parse_supports_condition("not (display: grid)").unwrap();
        assert!(matches!(condition, SupportsCondition::Not(_)));
    }

    #[test]
    fn test_parse_supports_and() {
        let condition = parse_supports_condition("(display: grid) and (gap: 10px)").unwrap();
        assert!(matches!(condition, SupportsCondition::And(ref v) if v.len() == 2));
    }

    #[test]
    fn test_parse_supports_or() {
        let condition = parse_supports_condition("(display: flex) or (display: grid)").unwrap();
        assert!(matches!(condition, SupportsCondition::Or(ref v) if v.len() == 2));
    }

    #[test]
    fn test_supports_evaluate_property() {
        let mut supported = HashSet::new();
        supported.insert("display".to_string());
        supported.insert("color".to_string());

        let condition = SupportsCondition::Property("display".to_string(), "grid".to_string());
        assert!(condition.evaluate(&supported));

        let condition = SupportsCondition::Property("unknown".to_string(), "value".to_string());
        assert!(!condition.evaluate(&supported));
    }

    #[test]
    fn test_supports_evaluate_not() {
        let supported = HashSet::new();
        let condition = SupportsCondition::Not(Box::new(SupportsCondition::Property(
            "display".to_string(),
            "grid".to_string(),
        )));
        assert!(condition.evaluate(&supported));
    }

    #[test]
    fn test_supports_evaluate_and() {
        let mut supported = HashSet::new();
        supported.insert("display".to_string());
        supported.insert("gap".to_string());

        let condition = SupportsCondition::And(vec![
            SupportsCondition::Property("display".to_string(), "grid".to_string()),
            SupportsCondition::Property("gap".to_string(), "10px".to_string()),
        ]);
        assert!(condition.evaluate(&supported));

        // Remove gap support
        supported.remove("gap");
        assert!(!condition.evaluate(&supported));
    }

    #[test]
    fn test_supports_evaluate_or() {
        let mut supported = HashSet::new();
        supported.insert("display".to_string());

        let condition = SupportsCondition::Or(vec![
            SupportsCondition::Property("display".to_string(), "grid".to_string()),
            SupportsCondition::Property("unknown".to_string(), "value".to_string()),
        ]);
        assert!(condition.evaluate(&supported));
    }

    // ========== @page tests ==========

    #[test]
    fn test_parse_page_basic() {
        let rule = parse_page("", "margin: 1cm;").unwrap();
        assert!(rule.selector.is_none());
        assert_eq!(rule.declarations.len(), 1);
    }

    #[test]
    fn test_parse_page_first() {
        let rule = parse_page(":first", "margin: 2cm;").unwrap();
        assert_eq!(rule.selector, Some(PageSelector::First));
    }

    #[test]
    fn test_parse_page_left() {
        let rule = parse_page(":left", "margin-left: 3cm;").unwrap();
        assert_eq!(rule.selector, Some(PageSelector::Left));
    }

    #[test]
    fn test_parse_page_right() {
        let rule = parse_page(":right", "margin-right: 3cm;").unwrap();
        assert_eq!(rule.selector, Some(PageSelector::Right));
    }

    #[test]
    fn test_parse_page_named() {
        let rule = parse_page("toc", "margin: 1in;").unwrap();
        assert_eq!(rule.selector, Some(PageSelector::Named("toc".to_string())));
    }

    // ========== @namespace tests ==========

    #[test]
    fn test_parse_namespace_default() {
        let rule = parse_namespace("\"http://www.w3.org/1999/xhtml\"").unwrap();
        assert!(rule.prefix.is_none());
        assert_eq!(rule.uri, "http://www.w3.org/1999/xhtml");
    }

    #[test]
    fn test_parse_namespace_prefixed() {
        let rule = parse_namespace("svg \"http://www.w3.org/2000/svg\"").unwrap();
        assert_eq!(rule.prefix, Some("svg".to_string()));
        assert_eq!(rule.uri, "http://www.w3.org/2000/svg");
    }

    #[test]
    fn test_parse_namespace_with_semicolon() {
        let rule = parse_namespace("xlink \"http://www.w3.org/1999/xlink\";").unwrap();
        assert_eq!(rule.prefix, Some("xlink".to_string()));
        assert_eq!(rule.uri, "http://www.w3.org/1999/xlink");
    }

    // ========== UnicodeRange tests ==========

    #[test]
    fn test_unicode_range_contains() {
        let range = UnicodeRange::new(0x0041, 0x005A);
        assert!(range.contains(0x0041)); // 'A'
        assert!(range.contains(0x004D)); // 'M'
        assert!(range.contains(0x005A)); // 'Z'
        assert!(!range.contains(0x0061)); // 'a'
    }

    #[test]
    fn test_unicode_range_single() {
        let range = UnicodeRange::single(0x20AC);
        assert!(range.contains(0x20AC)); // Euro sign
        assert!(!range.contains(0x20AD));
    }
}
