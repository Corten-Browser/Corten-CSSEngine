//! CSS Nesting Syntax Support
//!
//! This module implements native CSS nesting syntax with `&` selector support,
//! as defined in the CSS Nesting Module Level 1 specification.
//!
//! ## Features
//!
//! - Nested style rules within other style rules
//! - `&` nesting selector for parent reference
//! - Nested at-rules (@media, @supports inside rules)
//! - Flattening algorithm to convert nested CSS to flat rulesets
//!
//! ## Example
//!
//! ```ignore
//! use css_parser_core::nesting::{NestingParser, flatten_nested_rules};
//!
//! let css = r#"
//!     .card {
//!         padding: 16px;
//!
//!         &:hover {
//!             background: #eee;
//!         }
//!
//!         .title {
//!             font-size: 24px;
//!         }
//!     }
//! "#;
//!
//! let parser = NestingParser::new();
//! let nested = parser.parse(css)?;
//! let flat = flatten_nested_rules(&nested);
//! ```

use crate::{ParseError, PropertyDeclaration, StyleRule};

/// A nested CSS rule that can contain declarations and other nested rules
#[derive(Debug, Clone, PartialEq)]
pub struct NestedRule {
    /// The selector for this rule (can contain &)
    pub selector: NestedSelector,
    /// Property declarations in this rule
    pub declarations: Vec<PropertyDeclaration>,
    /// Nested rules within this rule
    pub nested_rules: Vec<NestedRule>,
    /// Nested at-rules (media queries, supports)
    pub nested_at_rules: Vec<NestedAtRule>,
}

impl NestedRule {
    /// Create a new nested rule
    pub fn new(selector: NestedSelector) -> Self {
        NestedRule {
            selector,
            declarations: Vec::new(),
            nested_rules: Vec::new(),
            nested_at_rules: Vec::new(),
        }
    }

    /// Add a declaration to this rule
    pub fn add_declaration(&mut self, decl: PropertyDeclaration) {
        self.declarations.push(decl);
    }

    /// Add a nested rule
    pub fn add_nested_rule(&mut self, rule: NestedRule) {
        self.nested_rules.push(rule);
    }

    /// Add a nested at-rule
    pub fn add_nested_at_rule(&mut self, at_rule: NestedAtRule) {
        self.nested_at_rules.push(at_rule);
    }
}

/// Represents different forms of nested selectors
#[derive(Debug, Clone, PartialEq)]
pub enum NestedSelector {
    /// Explicit & reference (just the parent)
    Ampersand,
    /// & followed by suffix: `&.class`, `&:hover`, `&[attr]`
    AmpersandSuffix(String),
    /// Selector with implicit & prepend (descendant): `.child` becomes `parent .child`
    Implicit(String),
    /// Complex selector with explicit & placement: `& > .child`, `.sibling &`
    Complex(ComplexNestedSelector),
    /// Root level selector (not nested, used for top-level rules)
    Root(String),
}

impl NestedSelector {
    /// Check if this selector contains an explicit & reference
    pub fn has_explicit_ampersand(&self) -> bool {
        matches!(
            self,
            NestedSelector::Ampersand
                | NestedSelector::AmpersandSuffix(_)
                | NestedSelector::Complex(_)
        )
    }

    /// Get the raw selector string
    pub fn as_str(&self) -> &str {
        match self {
            NestedSelector::Ampersand => "&",
            NestedSelector::AmpersandSuffix(s) => s,
            NestedSelector::Implicit(s) => s,
            NestedSelector::Complex(c) => &c.raw,
            NestedSelector::Root(s) => s,
        }
    }
}

/// Complex nested selector with explicit & placement
#[derive(Debug, Clone, PartialEq)]
pub struct ComplexNestedSelector {
    /// The raw selector string with & references
    pub raw: String,
    /// Positions of & in the selector
    pub ampersand_positions: Vec<usize>,
}

impl ComplexNestedSelector {
    /// Create a new complex nested selector
    pub fn new(raw: String) -> Self {
        let ampersand_positions = raw
            .char_indices()
            .filter_map(|(i, c)| if c == '&' { Some(i) } else { None })
            .collect();

        ComplexNestedSelector {
            raw,
            ampersand_positions,
        }
    }

    /// Resolve the selector by replacing & with the parent selector
    pub fn resolve(&self, parent: &str) -> String {
        self.raw.replace('&', parent)
    }
}

/// Nested at-rule (media query or supports within a style rule)
#[derive(Debug, Clone, PartialEq)]
pub enum NestedAtRule {
    /// Nested @media rule
    Media {
        /// Media query conditions
        query: String,
        /// Rules within the media query
        rules: Vec<NestedRule>,
    },
    /// Nested @supports rule
    Supports {
        /// Feature query condition
        condition: String,
        /// Rules within the supports query
        rules: Vec<NestedRule>,
    },
}

/// Parser for CSS nesting syntax
#[derive(Debug, Default)]
pub struct NestingParser {
    /// Current parsing position
    position: usize,
}

impl NestingParser {
    /// Create a new nesting parser
    pub fn new() -> Self {
        NestingParser { position: 0 }
    }

    /// Parse CSS with nesting into a list of nested rules
    pub fn parse(&mut self, input: &str) -> Result<Vec<NestedRule>, ParseError> {
        self.position = 0;
        let input = input.trim();

        if input.is_empty() {
            return Ok(Vec::new());
        }

        self.parse_rules(input)
    }

    /// Parse multiple rules from input
    fn parse_rules(&mut self, input: &str) -> Result<Vec<NestedRule>, ParseError> {
        let mut rules = Vec::new();
        let chars: Vec<char> = input.chars().collect();
        let mut start = 0;
        let mut brace_depth = 0;
        let mut i = 0;

        while i < chars.len() {
            match chars[i] {
                '{' => brace_depth += 1,
                '}' => {
                    brace_depth -= 1;
                    if brace_depth == 0 {
                        let rule_text: String = chars[start..=i].iter().collect();
                        let rule_text = rule_text.trim();
                        if !rule_text.is_empty() {
                            let rule = self.parse_single_rule(rule_text)?;
                            rules.push(rule);
                        }
                        start = i + 1;
                    }
                }
                _ => {}
            }
            i += 1;
        }

        if brace_depth != 0 {
            return Err(ParseError::new(1, 1, "Mismatched braces in nested CSS"));
        }

        Ok(rules)
    }

    /// Parse a single nested rule
    fn parse_single_rule(&mut self, input: &str) -> Result<NestedRule, ParseError> {
        let input = input.trim();

        // Find the opening brace
        let open_brace = input
            .find('{')
            .ok_or_else(|| ParseError::new(1, 1, "Expected '{' in rule"))?;

        let close_brace = input
            .rfind('}')
            .ok_or_else(|| ParseError::new(1, 1, "Expected '}' in rule"))?;

        if open_brace >= close_brace {
            return Err(ParseError::new(1, 1, "Mismatched braces"));
        }

        // Extract selector and body
        let selector_text = input[..open_brace].trim();
        let body = &input[open_brace + 1..close_brace];

        // Parse the selector
        let selector = self.parse_nested_selector(selector_text)?;

        // Create the rule
        let mut rule = NestedRule::new(selector);

        // Parse the body (declarations and nested rules)
        self.parse_rule_body(body, &mut rule)?;

        Ok(rule)
    }

    /// Parse a nested selector
    fn parse_nested_selector(&self, input: &str) -> Result<NestedSelector, ParseError> {
        let input = input.trim();

        if input.is_empty() {
            return Err(ParseError::new(1, 1, "Empty selector"));
        }

        // Check for & references
        let ampersand_count = input.matches('&').count();

        if ampersand_count == 0 {
            // No &, could be root-level or implicit nesting
            // For now, treat as root (will be resolved during flattening)
            return Ok(NestedSelector::Root(input.to_string()));
        }

        if input == "&" {
            // Just the ampersand
            return Ok(NestedSelector::Ampersand);
        }

        if input.starts_with('&') && !input[1..].contains('&') {
            // Starts with & and no other & references
            let suffix = &input[1..];

            // Check if it's a suffix (no space after &) or complex (has space)
            if suffix.starts_with('.') || suffix.starts_with(':') || suffix.starts_with('[') {
                // Direct suffix: &.class, &:hover, &[attr]
                return Ok(NestedSelector::AmpersandSuffix(input.to_string()));
            }
        }

        // Complex selector with & placement
        Ok(NestedSelector::Complex(ComplexNestedSelector::new(
            input.to_string(),
        )))
    }

    /// Parse the body of a rule (declarations and nested rules)
    fn parse_rule_body(&mut self, body: &str, rule: &mut NestedRule) -> Result<(), ParseError> {
        let body = body.trim();
        if body.is_empty() {
            return Ok(());
        }

        // Strategy: Split body into segments that are either:
        // 1. Declarations (end with ; at depth 0)
        // 2. Nested rules (selector { ... })
        // 3. Nested at-rules (@media/supports { ... })

        let chars: Vec<char> = body.chars().collect();
        let len = chars.len();
        let mut i = 0;
        let mut segment_start = 0;
        let mut brace_depth = 0;

        while i < len {
            let ch = chars[i];

            match ch {
                '{' => {
                    if brace_depth == 0 {
                        // Starting a nested block - find where the selector starts
                        // by looking back from current position
                        let selector_start = self.find_selector_start(&chars, segment_start, i);

                        // Parse any declarations between segment_start and selector_start
                        if selector_start > segment_start {
                            let decl_text: String =
                                chars[segment_start..selector_start].iter().collect();
                            self.parse_declarations_text(&decl_text, rule)?;
                        }

                        // Update segment_start to be the start of this nested rule
                        segment_start = selector_start;
                    }
                    brace_depth += 1;
                }
                '}' => {
                    brace_depth -= 1;
                    if brace_depth == 0 {
                        // End of nested rule - extract and parse it
                        let nested_text: String = chars[segment_start..=i].iter().collect();
                        self.parse_nested_content(&nested_text, rule)?;
                        segment_start = i + 1;
                    }
                }
                _ => {}
            }
            i += 1;
        }

        // Parse any remaining content as declarations
        if segment_start < len {
            let remaining: String = chars[segment_start..].iter().collect();
            self.parse_declarations_text(&remaining, rule)?;
        }

        Ok(())
    }

    /// Find where the selector starts by scanning backwards from the brace position
    fn find_selector_start(&self, chars: &[char], earliest: usize, brace_pos: usize) -> usize {
        // Scan backwards from brace_pos to find the start of the selector
        // The selector starts after a ; or at earliest, whichever is later
        let mut pos = brace_pos;

        // Skip whitespace before the brace
        while pos > earliest && chars[pos - 1].is_whitespace() {
            pos -= 1;
        }

        // Find the last semicolon before this position
        let mut last_semicolon = None;
        for (j, &ch) in chars.iter().enumerate().take(pos).skip(earliest) {
            if ch == ';' {
                last_semicolon = Some(j);
            }
        }

        if let Some(semi_pos) = last_semicolon {
            // Selector starts after the semicolon
            semi_pos + 1
        } else {
            // No semicolon found, selector starts at earliest
            earliest
        }
    }

    /// Parse declarations from text (semicolon-separated)
    fn parse_declarations_text(&self, text: &str, rule: &mut NestedRule) -> Result<(), ParseError> {
        let text = text.trim();
        if text.is_empty() {
            return Ok(());
        }

        // Split by semicolons at depth 0
        let mut decl_start = 0;
        let chars: Vec<char> = text.chars().collect();
        let mut paren_depth: i32 = 0;
        let mut i = 0;

        while i < chars.len() {
            match chars[i] {
                '(' => paren_depth += 1,
                ')' => paren_depth = paren_depth.saturating_sub(1),
                ';' if paren_depth == 0 => {
                    let decl_text: String = chars[decl_start..i].iter().collect();
                    self.try_parse_declaration(&decl_text, rule)?;
                    decl_start = i + 1;
                }
                _ => {}
            }
            i += 1;
        }

        // Parse remaining content
        if decl_start < chars.len() {
            let remaining: String = chars[decl_start..].iter().collect();
            self.try_parse_declaration(&remaining, rule)?;
        }

        Ok(())
    }

    /// Try to parse a single declaration from text
    fn try_parse_declaration(&self, text: &str, rule: &mut NestedRule) -> Result<(), ParseError> {
        let text = text.trim();
        if text.is_empty() {
            return Ok(());
        }

        // Must have a colon to be a declaration
        if let Some(colon_pos) = find_colon_outside_parens(text) {
            let name = text[..colon_pos].trim();
            let value = text[colon_pos + 1..].trim();

            // Skip if name looks like a selector or at-rule
            if name.is_empty()
                || value.is_empty()
                || name.starts_with('@')
                || name.starts_with('.')
                || name.starts_with('#')
                || name.starts_with('&')
                || name.starts_with(':')
                || name.starts_with('>')
                || name.starts_with('+')
                || name.starts_with('~')
            {
                return Ok(());
            }

            let decl = parse_declaration(name, value)?;
            rule.add_declaration(decl);
        }

        Ok(())
    }

    /// Parse nested content (rule or at-rule)
    fn parse_nested_content(
        &mut self,
        content: &str,
        parent: &mut NestedRule,
    ) -> Result<(), ParseError> {
        let content = content.trim();

        if content.starts_with("@media") {
            // Nested media query
            let at_rule = self.parse_nested_media(content)?;
            parent.add_nested_at_rule(at_rule);
        } else if content.starts_with("@supports") {
            // Nested supports query
            let at_rule = self.parse_nested_supports(content)?;
            parent.add_nested_at_rule(at_rule);
        } else {
            // Nested style rule
            let nested = self.parse_single_rule(content)?;
            // Convert root selector to implicit if it doesn't have &
            let converted = convert_to_implicit_if_needed(nested);
            parent.add_nested_rule(converted);
        }

        Ok(())
    }

    /// Parse a nested @media rule
    fn parse_nested_media(&mut self, input: &str) -> Result<NestedAtRule, ParseError> {
        let prefix = "@media";
        let rest = input[prefix.len()..].trim();

        let open_brace = rest
            .find('{')
            .ok_or_else(|| ParseError::new(1, 1, "Expected '{' in @media"))?;
        let close_brace = rest
            .rfind('}')
            .ok_or_else(|| ParseError::new(1, 1, "Expected '}' in @media"))?;

        let query = rest[..open_brace].trim().to_string();
        let body = &rest[open_brace + 1..close_brace];

        let rules = self.parse_rules(body)?;

        Ok(NestedAtRule::Media { query, rules })
    }

    /// Parse a nested @supports rule
    fn parse_nested_supports(&mut self, input: &str) -> Result<NestedAtRule, ParseError> {
        let prefix = "@supports";
        let rest = input[prefix.len()..].trim();

        let open_brace = rest
            .find('{')
            .ok_or_else(|| ParseError::new(1, 1, "Expected '{' in @supports"))?;
        let close_brace = rest
            .rfind('}')
            .ok_or_else(|| ParseError::new(1, 1, "Expected '}' in @supports"))?;

        let condition = rest[..open_brace].trim().to_string();
        let body = &rest[open_brace + 1..close_brace];

        let rules = self.parse_rules(body)?;

        Ok(NestedAtRule::Supports { condition, rules })
    }
}

/// Convert a root selector to implicit if it doesn't already have &
fn convert_to_implicit_if_needed(mut rule: NestedRule) -> NestedRule {
    if let NestedSelector::Root(ref s) = rule.selector {
        // Check if the selector has any & references
        if !s.contains('&') {
            rule.selector = NestedSelector::Implicit(s.clone());
        }
    }
    rule
}

/// Find colon position outside of parentheses and brackets
fn find_colon_outside_parens(s: &str) -> Option<usize> {
    let mut paren_depth: i32 = 0;
    let mut bracket_depth: i32 = 0;

    for (i, c) in s.char_indices() {
        match c {
            '(' => paren_depth += 1,
            ')' => paren_depth = paren_depth.saturating_sub(1),
            '[' => bracket_depth += 1,
            ']' => bracket_depth = bracket_depth.saturating_sub(1),
            ':' if paren_depth == 0 && bracket_depth == 0 => return Some(i),
            _ => {}
        }
    }
    None
}

/// Parse a single declaration from name and value
fn parse_declaration(name: &str, value: &str) -> Result<PropertyDeclaration, ParseError> {
    use crate::declaration::parse_declarations;

    let decl_str = format!("{}: {}", name, value);
    let decls = parse_declarations(&decl_str)?;

    decls.into_iter().next().ok_or_else(|| {
        ParseError::new(
            1,
            1,
            format!("Failed to parse declaration: {}: {}", name, value),
        )
    })
}

/// Flattened CSS rule (result of flattening nested rules)
#[derive(Debug, Clone, PartialEq)]
pub struct FlattenedRule {
    /// Resolved selector (no & references)
    pub selector: String,
    /// Property declarations
    pub declarations: Vec<PropertyDeclaration>,
    /// Optional media query wrapper
    pub media_query: Option<String>,
    /// Optional supports query wrapper
    pub supports_query: Option<String>,
}

impl FlattenedRule {
    /// Create a new flattened rule
    pub fn new(selector: String) -> Self {
        FlattenedRule {
            selector,
            declarations: Vec::new(),
            media_query: None,
            supports_query: None,
        }
    }

    /// Convert to a StyleRule
    pub fn to_style_rule(&self) -> Result<StyleRule, ParseError> {
        use crate::selector::parse_selector_list;

        let selectors = parse_selector_list(&self.selector)?;
        Ok(StyleRule {
            selectors,
            declarations: self.declarations.clone(),
        })
    }
}

/// Context for flattening nested rules
#[derive(Debug, Clone)]
struct FlattenContext {
    /// Current parent selector chain
    parent_selectors: Vec<String>,
    /// Current media query (if any)
    media_query: Option<String>,
    /// Current supports query (if any)
    supports_query: Option<String>,
}

impl FlattenContext {
    fn new() -> Self {
        FlattenContext {
            parent_selectors: Vec::new(),
            media_query: None,
            supports_query: None,
        }
    }

    fn with_parent(&self, parent: &str) -> Self {
        let mut ctx = self.clone();
        ctx.parent_selectors.push(parent.to_string());
        ctx
    }

    fn with_media(&self, query: &str) -> Self {
        let mut ctx = self.clone();
        ctx.media_query = Some(query.to_string());
        ctx
    }

    fn with_supports(&self, condition: &str) -> Self {
        let mut ctx = self.clone();
        ctx.supports_query = Some(condition.to_string());
        ctx
    }

    fn current_parent(&self) -> Option<&str> {
        self.parent_selectors.last().map(|s| s.as_str())
    }
}

/// Flatten nested rules into a list of flat rules
pub fn flatten_nested_rules(rules: &[NestedRule]) -> Vec<FlattenedRule> {
    let ctx = FlattenContext::new();
    let mut result = Vec::new();

    for rule in rules {
        flatten_rule(rule, &ctx, &mut result);
    }

    result
}

/// Flatten a single nested rule
fn flatten_rule(rule: &NestedRule, ctx: &FlattenContext, result: &mut Vec<FlattenedRule>) {
    // Resolve the selector
    let resolved_selector = resolve_selector(&rule.selector, ctx.current_parent());

    // Create flattened rule for this level's declarations
    if !rule.declarations.is_empty() {
        let mut flat = FlattenedRule::new(resolved_selector.clone());
        flat.declarations = rule.declarations.clone();
        flat.media_query = ctx.media_query.clone();
        flat.supports_query = ctx.supports_query.clone();
        result.push(flat);
    }

    // Create new context with current selector as parent
    let child_ctx = ctx.with_parent(&resolved_selector);

    // Flatten nested rules
    for nested in &rule.nested_rules {
        flatten_rule(nested, &child_ctx, result);
    }

    // Flatten nested at-rules
    for at_rule in &rule.nested_at_rules {
        match at_rule {
            NestedAtRule::Media { query, rules } => {
                let media_ctx = child_ctx.with_media(query);
                for nested in rules {
                    flatten_rule(nested, &media_ctx, result);
                }
            }
            NestedAtRule::Supports { condition, rules } => {
                let supports_ctx = child_ctx.with_supports(condition);
                for nested in rules {
                    flatten_rule(nested, &supports_ctx, result);
                }
            }
        }
    }
}

/// Resolve a nested selector against a parent selector
pub fn resolve_selector(selector: &NestedSelector, parent: Option<&str>) -> String {
    match selector {
        NestedSelector::Root(s) => s.clone(),
        NestedSelector::Ampersand => parent.unwrap_or("").to_string(),
        NestedSelector::AmpersandSuffix(s) => {
            if let Some(p) = parent {
                s.replace('&', p)
            } else {
                s.replace('&', "")
            }
        }
        NestedSelector::Implicit(s) => {
            if let Some(p) = parent {
                format!("{} {}", p, s)
            } else {
                s.clone()
            }
        }
        NestedSelector::Complex(c) => {
            if let Some(p) = parent {
                c.resolve(p)
            } else {
                c.raw.replace('&', "")
            }
        }
    }
}

/// Convert flattened rules back to standard StyleRules (grouped by media/supports)
pub fn to_style_rules(flattened: &[FlattenedRule]) -> Result<Vec<StyleRule>, ParseError> {
    let mut result = Vec::new();

    // For now, just convert rules without media/supports wrappers
    // Media/supports wrapped rules would need CssRule::Media/Supports wrappers
    for flat in flattened {
        if flat.media_query.is_none() && flat.supports_query.is_none() {
            result.push(flat.to_style_rule()?);
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== Basic Nesting Tests ====================

    #[test]
    fn test_parse_simple_nested_rule() {
        let mut parser = NestingParser::new();
        let css = r#"
            .parent {
                color: red;

                .child {
                    color: blue;
                }
            }
        "#;

        let rules = parser.parse(css).unwrap();
        assert_eq!(rules.len(), 1);

        let parent = &rules[0];
        assert_eq!(parent.declarations.len(), 1);
        assert_eq!(parent.nested_rules.len(), 1);

        let child = &parent.nested_rules[0];
        assert!(matches!(child.selector, NestedSelector::Implicit(_)));
    }

    #[test]
    fn test_parse_ampersand_selector() {
        let mut parser = NestingParser::new();
        let css = r#"
            .button {
                background: blue;

                &:hover {
                    background: darkblue;
                }
            }
        "#;

        let rules = parser.parse(css).unwrap();
        assert_eq!(rules.len(), 1);

        let button = &rules[0];
        assert_eq!(button.nested_rules.len(), 1);

        let hover = &button.nested_rules[0];
        assert!(matches!(hover.selector, NestedSelector::AmpersandSuffix(_)));
    }

    #[test]
    fn test_parse_ampersand_class() {
        let mut parser = NestingParser::new();
        let css = r#"
            .card {
                padding: 16px;

                &.active {
                    border: 1px solid blue;
                }
            }
        "#;

        let rules = parser.parse(css).unwrap();
        let card = &rules[0];
        let active = &card.nested_rules[0];

        if let NestedSelector::AmpersandSuffix(s) = &active.selector {
            assert_eq!(s, "&.active");
        } else {
            panic!("Expected AmpersandSuffix selector");
        }
    }

    // ==================== Selector Resolution Tests ====================

    #[test]
    fn test_resolve_ampersand_only() {
        let selector = NestedSelector::Ampersand;
        let resolved = resolve_selector(&selector, Some(".parent"));
        assert_eq!(resolved, ".parent");
    }

    #[test]
    fn test_resolve_ampersand_suffix_hover() {
        let selector = NestedSelector::AmpersandSuffix("&:hover".to_string());
        let resolved = resolve_selector(&selector, Some(".button"));
        assert_eq!(resolved, ".button:hover");
    }

    #[test]
    fn test_resolve_ampersand_suffix_class() {
        let selector = NestedSelector::AmpersandSuffix("&.active".to_string());
        let resolved = resolve_selector(&selector, Some(".card"));
        assert_eq!(resolved, ".card.active");
    }

    #[test]
    fn test_resolve_implicit_descendant() {
        let selector = NestedSelector::Implicit(".child".to_string());
        let resolved = resolve_selector(&selector, Some(".parent"));
        assert_eq!(resolved, ".parent .child");
    }

    #[test]
    fn test_resolve_complex_child_combinator() {
        let selector =
            NestedSelector::Complex(ComplexNestedSelector::new("& > .child".to_string()));
        let resolved = resolve_selector(&selector, Some(".parent"));
        assert_eq!(resolved, ".parent > .child");
    }

    #[test]
    fn test_resolve_complex_sibling_ampersand() {
        let selector =
            NestedSelector::Complex(ComplexNestedSelector::new(".sibling &".to_string()));
        let resolved = resolve_selector(&selector, Some(".target"));
        assert_eq!(resolved, ".sibling .target");
    }

    #[test]
    fn test_resolve_complex_multiple_ampersands() {
        let selector = NestedSelector::Complex(ComplexNestedSelector::new("& + &".to_string()));
        let resolved = resolve_selector(&selector, Some(".item"));
        assert_eq!(resolved, ".item + .item");
    }

    // ==================== Flattening Tests ====================

    #[test]
    fn test_flatten_simple_nesting() {
        let mut parser = NestingParser::new();
        let css = r#"
            .parent {
                color: red;

                .child {
                    color: blue;
                }
            }
        "#;

        let rules = parser.parse(css).unwrap();
        let flattened = flatten_nested_rules(&rules);

        assert_eq!(flattened.len(), 2);
        assert_eq!(flattened[0].selector, ".parent");
        assert_eq!(flattened[1].selector, ".parent .child");
    }

    #[test]
    fn test_flatten_ampersand_hover() {
        let mut parser = NestingParser::new();
        let css = r#"
            .button {
                background: blue;

                &:hover {
                    background: darkblue;
                }
            }
        "#;

        let rules = parser.parse(css).unwrap();
        let flattened = flatten_nested_rules(&rules);

        assert_eq!(flattened.len(), 2);
        assert_eq!(flattened[0].selector, ".button");
        assert_eq!(flattened[1].selector, ".button:hover");
    }

    #[test]
    fn test_flatten_deep_nesting() {
        let mut parser = NestingParser::new();
        let css = r#"
            .level1 {
                color: red;

                .level2 {
                    color: green;

                    .level3 {
                        color: blue;
                    }
                }
            }
        "#;

        let rules = parser.parse(css).unwrap();
        let flattened = flatten_nested_rules(&rules);

        assert_eq!(flattened.len(), 3);
        assert_eq!(flattened[0].selector, ".level1");
        assert_eq!(flattened[1].selector, ".level1 .level2");
        assert_eq!(flattened[2].selector, ".level1 .level2 .level3");
    }

    #[test]
    fn test_flatten_with_child_combinator() {
        let mut parser = NestingParser::new();
        let css = r#"
            .parent {
                margin: 0;

                & > .child {
                    padding: 10px;
                }
            }
        "#;

        let rules = parser.parse(css).unwrap();
        let flattened = flatten_nested_rules(&rules);

        assert_eq!(flattened.len(), 2);
        assert_eq!(flattened[1].selector, ".parent > .child");
    }

    // ==================== Nested At-Rules Tests ====================

    #[test]
    fn test_parse_nested_media_query() {
        let mut parser = NestingParser::new();
        let css = r#"
            .card {
                width: 100%;

                @media (min-width: 768px) {
                    & {
                        width: 50%;
                    }
                }
            }
        "#;

        let rules = parser.parse(css).unwrap();
        assert_eq!(rules.len(), 1);

        let card = &rules[0];
        assert_eq!(card.nested_at_rules.len(), 1);

        if let NestedAtRule::Media { query, rules } = &card.nested_at_rules[0] {
            assert_eq!(query, "(min-width: 768px)");
            assert_eq!(rules.len(), 1);
        } else {
            panic!("Expected Media at-rule");
        }
    }

    #[test]
    fn test_flatten_nested_media_query() {
        let mut parser = NestingParser::new();
        let css = r#"
            .container {
                width: 100%;

                @media (min-width: 768px) {
                    & {
                        width: 750px;
                    }
                }
            }
        "#;

        let rules = parser.parse(css).unwrap();
        let flattened = flatten_nested_rules(&rules);

        // Should have 2 rules: base .container and media-wrapped .container
        assert_eq!(flattened.len(), 2);

        assert_eq!(flattened[0].selector, ".container");
        assert!(flattened[0].media_query.is_none());

        assert_eq!(flattened[1].selector, ".container");
        assert_eq!(
            flattened[1].media_query,
            Some("(min-width: 768px)".to_string())
        );
    }

    #[test]
    fn test_parse_nested_supports() {
        let mut parser = NestingParser::new();
        let css = r#"
            .flex-container {
                display: block;

                @supports (display: flex) {
                    & {
                        display: flex;
                    }
                }
            }
        "#;

        let rules = parser.parse(css).unwrap();
        let container = &rules[0];
        assert_eq!(container.nested_at_rules.len(), 1);

        if let NestedAtRule::Supports { condition, .. } = &container.nested_at_rules[0] {
            assert_eq!(condition, "(display: flex)");
        } else {
            panic!("Expected Supports at-rule");
        }
    }

    // ==================== Edge Cases Tests ====================

    #[test]
    fn test_empty_input() {
        let mut parser = NestingParser::new();
        let rules = parser.parse("").unwrap();
        assert!(rules.is_empty());
    }

    #[test]
    fn test_whitespace_only_input() {
        let mut parser = NestingParser::new();
        let rules = parser.parse("   \n\t  ").unwrap();
        assert!(rules.is_empty());
    }

    #[test]
    fn test_rule_without_declarations() {
        let mut parser = NestingParser::new();
        let css = r#"
            .empty {
                .child {
                    color: red;
                }
            }
        "#;

        let rules = parser.parse(css).unwrap();
        assert_eq!(rules.len(), 1);

        let empty = &rules[0];
        assert!(empty.declarations.is_empty());
        assert_eq!(empty.nested_rules.len(), 1);
    }

    #[test]
    fn test_multiple_top_level_rules() {
        let mut parser = NestingParser::new();
        let css = r#"
            .first {
                color: red;
            }
            .second {
                color: blue;
            }
        "#;

        let rules = parser.parse(css).unwrap();
        assert_eq!(rules.len(), 2);
    }

    #[test]
    fn test_nested_selector_enum_variants() {
        // Test Ampersand
        let amp = NestedSelector::Ampersand;
        assert!(amp.has_explicit_ampersand());
        assert_eq!(amp.as_str(), "&");

        // Test AmpersandSuffix
        let suffix = NestedSelector::AmpersandSuffix("&:hover".to_string());
        assert!(suffix.has_explicit_ampersand());
        assert_eq!(suffix.as_str(), "&:hover");

        // Test Implicit
        let implicit = NestedSelector::Implicit(".child".to_string());
        assert!(!implicit.has_explicit_ampersand());
        assert_eq!(implicit.as_str(), ".child");

        // Test Root
        let root = NestedSelector::Root(".parent".to_string());
        assert!(!root.has_explicit_ampersand());
        assert_eq!(root.as_str(), ".parent");
    }

    #[test]
    fn test_complex_nested_selector() {
        let complex = ComplexNestedSelector::new("& > .child + &".to_string());
        // "& > .child + &" has & at positions 0 and 13
        assert_eq!(complex.ampersand_positions, vec![0, 13]);
        assert_eq!(complex.resolve(".parent"), ".parent > .child + .parent");
    }

    // ==================== Real-World Example Tests ====================

    #[test]
    fn test_bem_like_nesting() {
        let mut parser = NestingParser::new();
        let css = r#"
            .block {
                display: block;

                &__element {
                    display: inline;
                }

                &--modifier {
                    display: flex;
                }
            }
        "#;

        let rules = parser.parse(css).unwrap();
        let flattened = flatten_nested_rules(&rules);

        assert_eq!(flattened.len(), 3);
        assert_eq!(flattened[0].selector, ".block");
        assert_eq!(flattened[1].selector, ".block__element");
        assert_eq!(flattened[2].selector, ".block--modifier");
    }

    #[test]
    fn test_component_styling() {
        let mut parser = NestingParser::new();
        let css = r#"
            .card {
                padding: 16px;

                &:hover {
                    background: #f5f5f5;
                }

                .card-title {
                    font-size: 20px;
                }

                .card-body {
                    margin-top: 8px;

                    p {
                        line-height: 1.5;
                    }
                }
            }
        "#;

        let rules = parser.parse(css).unwrap();
        let flattened = flatten_nested_rules(&rules);

        assert_eq!(flattened.len(), 5);
        assert_eq!(flattened[0].selector, ".card");
        assert_eq!(flattened[1].selector, ".card:hover");
        assert_eq!(flattened[2].selector, ".card .card-title");
        assert_eq!(flattened[3].selector, ".card .card-body");
        assert_eq!(flattened[4].selector, ".card .card-body p");
    }

    #[test]
    fn test_responsive_component() {
        let mut parser = NestingParser::new();
        let css = r#"
            .grid {
                display: block;

                @media (min-width: 768px) {
                    & {
                        display: grid;
                        grid-template-columns: repeat(2, 1fr);
                    }
                }

                @media (min-width: 1024px) {
                    & {
                        grid-template-columns: repeat(3, 1fr);
                    }
                }
            }
        "#;

        let rules = parser.parse(css).unwrap();
        let flattened = flatten_nested_rules(&rules);

        assert_eq!(flattened.len(), 3);

        // Base rule
        assert_eq!(flattened[0].selector, ".grid");
        assert!(flattened[0].media_query.is_none());

        // Tablet
        assert_eq!(flattened[1].selector, ".grid");
        assert_eq!(
            flattened[1].media_query,
            Some("(min-width: 768px)".to_string())
        );

        // Desktop
        assert_eq!(flattened[2].selector, ".grid");
        assert_eq!(
            flattened[2].media_query,
            Some("(min-width: 1024px)".to_string())
        );
    }

    // ==================== Flattened Rule Conversion Tests ====================

    #[test]
    fn test_flattened_to_style_rule() {
        let mut flat = FlattenedRule::new(".test".to_string());
        flat.declarations.push(PropertyDeclaration {
            name: "color".to_string(),
            value: crate::PropertyValue::Keyword("red".to_string()),
            important: false,
        });

        let style_rule = flat.to_style_rule().unwrap();
        assert_eq!(style_rule.selectors.len(), 1);
        assert_eq!(style_rule.declarations.len(), 1);
    }

    #[test]
    fn test_to_style_rules_filters_media() {
        let flattened = vec![
            FlattenedRule {
                selector: ".base".to_string(),
                declarations: vec![],
                media_query: None,
                supports_query: None,
            },
            FlattenedRule {
                selector: ".media".to_string(),
                declarations: vec![],
                media_query: Some("(min-width: 768px)".to_string()),
                supports_query: None,
            },
        ];

        let style_rules = to_style_rules(&flattened).unwrap();
        // Should only include the base rule (not media-wrapped)
        assert_eq!(style_rules.len(), 1);
    }

    // ==================== Helper Function Tests ====================

    #[test]
    fn test_find_colon_outside_parens() {
        assert_eq!(find_colon_outside_parens("color: red"), Some(5));
        assert_eq!(
            find_colon_outside_parens("background: rgb(255, 0, 0)"),
            Some(10)
        );
        assert_eq!(find_colon_outside_parens("no-colon"), None);
    }

    #[test]
    fn test_nested_rule_builder_methods() {
        let mut rule = NestedRule::new(NestedSelector::Root(".test".to_string()));

        rule.add_declaration(PropertyDeclaration {
            name: "color".to_string(),
            value: crate::PropertyValue::Keyword("red".to_string()),
            important: false,
        });
        assert_eq!(rule.declarations.len(), 1);

        rule.add_nested_rule(NestedRule::new(NestedSelector::Implicit(
            ".child".to_string(),
        )));
        assert_eq!(rule.nested_rules.len(), 1);

        rule.add_nested_at_rule(NestedAtRule::Media {
            query: "(min-width: 768px)".to_string(),
            rules: vec![],
        });
        assert_eq!(rule.nested_at_rules.len(), 1);
    }

    // ==================== Error Handling Tests ====================

    #[test]
    fn test_mismatched_braces_error() {
        let mut parser = NestingParser::new();
        let css = ".broken { color: red;";

        let result = parser.parse(css);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_selector_error() {
        let parser = NestingParser::new();
        let result = parser.parse_nested_selector("");
        assert!(result.is_err());
    }
}
