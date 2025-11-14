//! CSS Parser Values - Advanced CSS value parsing
//!
//! This module provides advanced CSS value parsing including:
//! - Attribute selectors ([attr], [attr=value], etc.)
//! - Complex color values (hex, rgb, rgba, hsl, hsla, named colors)
//! - CSS functions (url(), calc(), var(), gradients)
//! - Generic value parsing (numbers, strings, lengths, keywords)

use css_types::{Color, CssError, CssValue, Length};

// ============================================================================
// Attribute Selector Types
// ============================================================================

/// Attribute matching operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttributeOperator {
    /// [attr] - Attribute exists
    Exists,
    /// [attr=value] - Exact match
    Equals,
    /// [attr~=value] - Whitespace-separated list contains value
    Includes,
    /// [attr|=value] - Starts with value or value-
    DashMatch,
    /// [attr^=value] - Starts with value
    Prefix,
    /// [attr$=value] - Ends with value
    Suffix,
    /// [attr*=value] - Contains value
    Substring,
}

/// Case sensitivity for attribute matching
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaseSensitivity {
    /// Case-sensitive matching
    CaseSensitive,
    /// ASCII case-insensitive matching
    AsciiCaseInsensitive,
}

/// Represents an attribute selector like [attr=value]
#[derive(Debug, Clone, PartialEq)]
pub struct AttributeSelector {
    /// Attribute name
    name: String,
    /// Optional namespace prefix
    namespace: Option<String>,
    /// Matching operator
    operator: AttributeOperator,
    /// Optional value to match
    value: Option<String>,
    /// Case sensitivity
    case_sensitivity: CaseSensitivity,
}

impl AttributeSelector {
    /// Create a new attribute selector
    pub fn new(
        name: String,
        namespace: Option<String>,
        operator: AttributeOperator,
        value: Option<String>,
        case_sensitivity: CaseSensitivity,
    ) -> Self {
        Self {
            name,
            namespace,
            operator,
            value,
            case_sensitivity,
        }
    }

    /// Get the attribute name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the namespace prefix
    pub fn namespace(&self) -> Option<&str> {
        self.namespace.as_deref()
    }

    /// Get the operator
    pub fn operator(&self) -> AttributeOperator {
        self.operator
    }

    /// Get the value to match
    pub fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }

    /// Get the case sensitivity
    pub fn case_sensitivity(&self) -> CaseSensitivity {
        self.case_sensitivity
    }
}

/// Parse an attribute selector from a string
///
/// # Examples
/// ```
/// use css_parser_values::parse_attribute_selector;
///
/// let selector = parse_attribute_selector("[href^=\"https\"]").unwrap();
/// assert_eq!(selector.name(), "href");
/// ```
///
/// # Errors
/// Returns `CssError::ParseError` if the input is not a valid attribute selector.
pub fn parse_attribute_selector(input: &str) -> Result<AttributeSelector, CssError> {
    let input = input.trim();

    // Must start with [ and end with ]
    if !input.starts_with('[') || !input.ends_with(']') {
        return Err(CssError::ParseError(
            "Attribute selector must be enclosed in brackets".to_string(),
        ));
    }

    // Remove brackets
    let content = input[1..input.len() - 1].trim();

    if content.is_empty() {
        return Err(CssError::ParseError(
            "Attribute selector cannot be empty".to_string(),
        ));
    }

    // Parse case sensitivity flag at the end
    let (content, case_sensitivity) = if content.ends_with(" i") || content.ends_with(" I") {
        (
            content[..content.len() - 2].trim(),
            CaseSensitivity::AsciiCaseInsensitive,
        )
    } else if content.ends_with(" s") || content.ends_with(" S") {
        (
            content[..content.len() - 2].trim(),
            CaseSensitivity::CaseSensitive,
        )
    } else {
        (content, CaseSensitivity::CaseSensitive)
    };

    // Check for operator
    let (operator, split_pos) = if let Some(pos) = content.find("~=") {
        (AttributeOperator::Includes, Some(pos))
    } else if let Some(pos) = content.find("|=") {
        (AttributeOperator::DashMatch, Some(pos))
    } else if let Some(pos) = content.find("^=") {
        (AttributeOperator::Prefix, Some(pos))
    } else if let Some(pos) = content.find("$=") {
        (AttributeOperator::Suffix, Some(pos))
    } else if let Some(pos) = content.find("*=") {
        (AttributeOperator::Substring, Some(pos))
    } else if let Some(pos) = content.find('=') {
        (AttributeOperator::Equals, Some(pos))
    } else {
        (AttributeOperator::Exists, None)
    };

    if let Some(split_pos) = split_pos {
        // Has operator and value
        let attr_part = content[..split_pos].trim();
        let value_part = content[split_pos
            + if operator == AttributeOperator::Equals {
                1
            } else {
                2
            }..]
            .trim();

        // Parse namespace and attribute name
        let (namespace, name) = if let Some(pipe_pos) = attr_part.find('|') {
            let ns = attr_part[..pipe_pos].trim().to_string();
            let nm = attr_part[pipe_pos + 1..].trim().to_string();
            (Some(ns), nm)
        } else {
            (None, attr_part.to_string())
        };

        // Remove quotes from value if present
        let value = if (value_part.starts_with('"') && value_part.ends_with('"'))
            || (value_part.starts_with('\'') && value_part.ends_with('\''))
        {
            value_part[1..value_part.len() - 1].to_string()
        } else {
            value_part.to_string()
        };

        Ok(AttributeSelector::new(
            name,
            namespace,
            operator,
            Some(value),
            case_sensitivity,
        ))
    } else {
        // Just attribute existence check
        let (namespace, name) = if let Some(pipe_pos) = content.find('|') {
            let ns = content[..pipe_pos].trim().to_string();
            let nm = content[pipe_pos + 1..].trim().to_string();
            (Some(ns), nm)
        } else {
            (None, content.to_string())
        };

        Ok(AttributeSelector::new(
            name,
            namespace,
            operator,
            None,
            case_sensitivity,
        ))
    }
}

// ============================================================================
// Color Parsing with Named Colors and HSL
// ============================================================================

/// Parse a CSS color value including named colors and HSL
///
/// Supports:
/// - Hex colors: #RGB, #RRGGBB
/// - RGB/RGBA: rgb(r, g, b), rgba(r, g, b, a)
/// - HSL/HSLA: hsl(h, s, l), hsla(h, s, l, a)
/// - Named colors: red, blue, green, etc.
///
/// # Examples
/// ```
/// use css_parser_values::parse_color_value;
///
/// let color = parse_color_value("red").unwrap();
/// assert_eq!(color.r(), 255);
/// ```
///
/// # Errors
/// Returns `CssError::ParseError` if the input is not a valid color.
pub fn parse_color_value(input: &str) -> Result<Color, CssError> {
    let input = input.trim();

    if input.is_empty() {
        return Err(CssError::ParseError("Empty color string".to_string()));
    }

    // Try parsing as hex or rgb/rgba (already in css_types)
    if input.starts_with('#') || input.starts_with("rgb") {
        return Color::parse(input);
    }

    // Try parsing as HSL/HSLA
    if input.starts_with("hsl") {
        return parse_hsl(input);
    }

    // Try parsing as named color
    parse_named_color(input)
}

/// Parse HSL/HSLA color
fn parse_hsl(input: &str) -> Result<Color, CssError> {
    let input = input.trim();

    let (is_hsla, content) = if let Some(stripped) = input.strip_prefix("hsla(") {
        (true, stripped)
    } else if let Some(stripped) = input.strip_prefix("hsl(") {
        (false, stripped)
    } else {
        return Err(CssError::ParseError("Invalid HSL function".to_string()));
    };

    let content = content
        .strip_suffix(')')
        .ok_or_else(|| CssError::ParseError("Missing closing parenthesis".to_string()))?;

    let parts: Vec<&str> = content.split(',').map(|s| s.trim()).collect();

    if is_hsla {
        if parts.len() != 4 {
            return Err(CssError::ParseError("hsla() requires 4 values".to_string()));
        }

        let h = parse_hue(parts[0])?;
        let s = parse_percentage(parts[1])?;
        let l = parse_percentage(parts[2])?;
        let a = parts[3]
            .parse::<f32>()
            .map_err(|_| CssError::ParseError("Invalid alpha value".to_string()))?;

        if !(0.0..=1.0).contains(&a) {
            return Err(CssError::OutOfRange(
                "Alpha must be between 0 and 1".to_string(),
            ));
        }

        let (r, g, b) = hsl_to_rgb(h, s, l);
        Ok(Color::rgba(r, g, b, a))
    } else {
        if parts.len() != 3 {
            return Err(CssError::ParseError("hsl() requires 3 values".to_string()));
        }

        let h = parse_hue(parts[0])?;
        let s = parse_percentage(parts[1])?;
        let l = parse_percentage(parts[2])?;

        let (r, g, b) = hsl_to_rgb(h, s, l);
        Ok(Color::rgb(r, g, b))
    }
}

/// Parse hue value (0-360)
fn parse_hue(s: &str) -> Result<f32, CssError> {
    let s = s.trim();
    let value = s
        .parse::<f32>()
        .map_err(|_| CssError::ParseError("Invalid hue value".to_string()))?;

    // Normalize to 0-360 range
    Ok(value % 360.0)
}

/// Parse percentage value (0-100%)
fn parse_percentage(s: &str) -> Result<f32, CssError> {
    let s = s.trim();
    if !s.ends_with('%') {
        return Err(CssError::ParseError(
            "Expected percentage value".to_string(),
        ));
    }

    let value_str = &s[..s.len() - 1];
    let value = value_str
        .parse::<f32>()
        .map_err(|_| CssError::ParseError("Invalid percentage".to_string()))?;

    if !(0.0..=100.0).contains(&value) {
        return Err(CssError::OutOfRange(
            "Percentage must be 0-100%".to_string(),
        ));
    }

    Ok(value / 100.0)
}

/// Convert HSL to RGB
/// H is in degrees (0-360), S and L are 0-1
fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let h_prime = h / 60.0;
    let x = c * (1.0 - ((h_prime % 2.0) - 1.0).abs());

    let (r1, g1, b1) = if h_prime < 1.0 {
        (c, x, 0.0)
    } else if h_prime < 2.0 {
        (x, c, 0.0)
    } else if h_prime < 3.0 {
        (0.0, c, x)
    } else if h_prime < 4.0 {
        (0.0, x, c)
    } else if h_prime < 5.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    let m = l - c / 2.0;

    let r = ((r1 + m) * 255.0).round() as u8;
    let g = ((g1 + m) * 255.0).round() as u8;
    let b = ((b1 + m) * 255.0).round() as u8;

    (r, g, b)
}

/// Parse named CSS color
fn parse_named_color(name: &str) -> Result<Color, CssError> {
    let name = name.trim().to_lowercase();

    match name.as_str() {
        // Basic colors
        "black" => Ok(Color::rgb(0, 0, 0)),
        "white" => Ok(Color::rgb(255, 255, 255)),
        "red" => Ok(Color::rgb(255, 0, 0)),
        "green" => Ok(Color::rgb(0, 128, 0)),
        "blue" => Ok(Color::rgb(0, 0, 255)),
        "yellow" => Ok(Color::rgb(255, 255, 0)),
        "cyan" => Ok(Color::rgb(0, 255, 255)),
        "magenta" => Ok(Color::rgb(255, 0, 255)),
        "silver" => Ok(Color::rgb(192, 192, 192)),
        "gray" | "grey" => Ok(Color::rgb(128, 128, 128)),
        "maroon" => Ok(Color::rgb(128, 0, 0)),
        "olive" => Ok(Color::rgb(128, 128, 0)),
        "lime" => Ok(Color::rgb(0, 255, 0)),
        "aqua" => Ok(Color::rgb(0, 255, 255)),
        "teal" => Ok(Color::rgb(0, 128, 128)),
        "navy" => Ok(Color::rgb(0, 0, 128)),
        "fuchsia" => Ok(Color::rgb(255, 0, 255)),
        "purple" => Ok(Color::rgb(128, 0, 128)),
        // Transparent
        "transparent" => Ok(Color::rgba(0, 0, 0, 0.0)),
        _ => Err(CssError::ParseError(format!(
            "Unknown color name: {}",
            name
        ))),
    }
}

// ============================================================================
// Function Value Parsing
// ============================================================================

/// Represents a parsed CSS function value
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionValue {
    /// Function name (e.g., "url", "calc", "var")
    name: String,
    /// Function arguments
    args: Vec<String>,
}

impl FunctionValue {
    /// Create a new function value
    pub fn new(name: String, args: Vec<String>) -> Self {
        Self { name, args }
    }

    /// Get the function name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the function arguments
    pub fn args(&self) -> &[String] {
        &self.args
    }
}

/// Parse a CSS function value
///
/// Supports:
/// - url() - URLs
/// - calc() - Calculations
/// - var() - CSS variables
/// - rgb(), rgba() - Colors
/// - linear-gradient(), radial-gradient() - Gradients
/// - And other CSS functions
///
/// # Examples
/// ```
/// use css_parser_values::parse_function_value;
///
/// let func = parse_function_value("url(\"image.png\")").unwrap();
/// assert_eq!(func.name(), "url");
/// ```
///
/// # Errors
/// Returns `CssError::ParseError` if the input is not a valid function.
pub fn parse_function_value(input: &str) -> Result<FunctionValue, CssError> {
    let input = input.trim();

    // Find the opening parenthesis
    let paren_pos = input
        .find('(')
        .ok_or_else(|| CssError::ParseError("Function must have parentheses".to_string()))?;

    let name = input[..paren_pos].trim().to_string();

    if name.is_empty() {
        return Err(CssError::ParseError(
            "Function name cannot be empty".to_string(),
        ));
    }

    // Check for closing parenthesis
    if !input.ends_with(')') {
        return Err(CssError::ParseError(
            "Missing closing parenthesis".to_string(),
        ));
    }

    // Extract arguments
    let args_str = &input[paren_pos + 1..input.len() - 1].trim();

    // Parse arguments
    let args = if args_str.is_empty() {
        vec![]
    } else {
        // Special handling for url() - don't split by comma
        if name == "url" {
            let arg = if (args_str.starts_with('"') && args_str.ends_with('"'))
                || (args_str.starts_with('\'') && args_str.ends_with('\''))
            {
                args_str[1..args_str.len() - 1].to_string()
            } else {
                args_str.to_string()
            };
            vec![arg]
        } else {
            // Split by commas for other functions
            args_str.split(',').map(|s| s.trim().to_string()).collect()
        }
    };

    Ok(FunctionValue::new(name, args))
}

// ============================================================================
// Complex Value Types
// ============================================================================

/// Type of CSS value
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueKind {
    /// Numeric value
    Number,
    /// Percentage value
    Percentage,
    /// Length value with unit
    Length,
    /// Color value
    Color,
    /// String value
    String,
    /// URL value
    Url,
    /// Function value
    Function,
    /// Keyword value
    Keyword,
}

/// Complex CSS value (can be any type)
#[derive(Debug, Clone, PartialEq)]
pub struct ComplexValue {
    kind: ValueKind,
    // Store as string for now; could be enum with different data types
    data: String,
}

impl ComplexValue {
    /// Create a new complex value
    pub fn new(kind: ValueKind, data: String) -> Self {
        Self { kind, data }
    }

    /// Get the value kind
    pub fn kind(&self) -> ValueKind {
        self.kind
    }

    /// Get the value data as string
    pub fn data(&self) -> &str {
        &self.data
    }
}

/// Parse a CSS property value
///
/// Determines the type of value and parses it appropriately.
///
/// # Examples
/// ```
/// use css_parser_values::{parse_value, ValueKind};
///
/// let value = parse_value("10px", "margin").unwrap();
/// assert_eq!(value.kind(), ValueKind::Length);
/// ```
///
/// # Errors
/// Returns `CssError::ParseError` if the input is not a valid value.
pub fn parse_value(input: &str, _property: &str) -> Result<ComplexValue, CssError> {
    let input = input.trim();

    if input.is_empty() {
        return Err(CssError::ParseError("Empty value string".to_string()));
    }

    // Check for string (quoted)
    if (input.starts_with('"') && input.ends_with('"'))
        || (input.starts_with('\'') && input.ends_with('\''))
    {
        return Ok(ComplexValue::new(
            ValueKind::String,
            input[1..input.len() - 1].to_string(),
        ));
    }

    // Check for URL function
    if input.starts_with("url(") {
        return Ok(ComplexValue::new(ValueKind::Url, input.to_string()));
    }

    // Check for function
    if input.contains('(') && input.ends_with(')') {
        return Ok(ComplexValue::new(ValueKind::Function, input.to_string()));
    }

    // Check for color (hex or named)
    if input.starts_with('#') || parse_named_color(input).is_ok() {
        return Ok(ComplexValue::new(ValueKind::Color, input.to_string()));
    }

    // Check for percentage
    if input.ends_with('%') {
        return Ok(ComplexValue::new(ValueKind::Percentage, input.to_string()));
    }

    // Check for length (has unit)
    if Length::parse(input).is_ok() {
        return Ok(ComplexValue::new(ValueKind::Length, input.to_string()));
    }

    // Check for number
    if input.parse::<f32>().is_ok() {
        return Ok(ComplexValue::new(ValueKind::Number, input.to_string()));
    }

    // Otherwise treat as keyword
    Ok(ComplexValue::new(ValueKind::Keyword, input.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attribute_operator_exists() {
        let selector = AttributeSelector::new(
            "test".to_string(),
            None,
            AttributeOperator::Exists,
            None,
            CaseSensitivity::CaseSensitive,
        );
        assert_eq!(selector.operator(), AttributeOperator::Exists);
    }

    #[test]
    fn test_hsl_to_rgb_red() {
        let (r, g, b) = hsl_to_rgb(0.0, 1.0, 0.5);
        assert_eq!(r, 255);
        assert_eq!(g, 0);
        assert_eq!(b, 0);
    }

    #[test]
    fn test_parse_named_color_basic() {
        let color = parse_named_color("red").unwrap();
        assert_eq!(color.r(), 255);
        assert_eq!(color.g(), 0);
        assert_eq!(color.b(), 0);
    }

    #[test]
    fn test_function_value_creation() {
        let func = FunctionValue::new("test".to_string(), vec!["arg1".to_string()]);
        assert_eq!(func.name(), "test");
        assert_eq!(func.args().len(), 1);
    }

    #[test]
    fn test_complex_value_creation() {
        let value = ComplexValue::new(ValueKind::Number, "42".to_string());
        assert_eq!(value.kind(), ValueKind::Number);
        assert_eq!(value.data(), "42");
    }
}
