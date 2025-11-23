//! Paint Function Parser
//!
//! This module provides parsing for the CSS `paint()` function.
//!
//! # Syntax
//!
//! ```text
//! paint(<worklet-name> [, <arguments>]*)
//! ```
//!
//! # Examples
//!
//! ```text
//! paint(my-worklet)
//! paint(checkerboard, 20)
//! paint(gradient, #ff0000, #0000ff)
//! paint(pattern, 10px, 45deg)
//! ```

use crate::{PaintArgumentType, PaintError, PaintValue, PaintWorkletRegistry};
use css_types::{Color, CssValue};

/// A parsed paint() function call
#[derive(Debug, Clone, PartialEq)]
pub struct ParsedPaintFunction {
    /// The worklet name
    pub name: String,
    /// The raw argument strings
    pub raw_arguments: Vec<String>,
}

impl ParsedPaintFunction {
    /// Create a new parsed paint function
    pub fn new(name: String, raw_arguments: Vec<String>) -> Self {
        Self {
            name,
            raw_arguments,
        }
    }

    /// Resolve arguments to PaintValues using the worklet's type information
    pub fn resolve_arguments(
        &self,
        registry: &PaintWorkletRegistry,
    ) -> Result<Vec<PaintValue>, PaintError> {
        let worklet = registry
            .get(&self.name)
            .ok_or_else(|| PaintError::WorkletNotFound(self.name.clone()))?;

        let expected_types = worklet.input_arguments();

        if self.raw_arguments.len() != expected_types.len() {
            return Err(PaintError::ArgumentCountMismatch {
                expected: expected_types.len(),
                got: self.raw_arguments.len(),
            });
        }

        let mut values = Vec::with_capacity(self.raw_arguments.len());

        for (raw_arg, expected_type) in self.raw_arguments.iter().zip(expected_types.iter()) {
            let value = parse_argument(raw_arg, *expected_type)?;
            values.push(value);
        }

        Ok(values)
    }
}

/// Parse a CSS `paint()` function
///
/// # Arguments
///
/// * `input` - The CSS function string, e.g., "paint(my-worklet, 10, red)"
///
/// # Returns
///
/// A `ParsedPaintFunction` containing the worklet name and raw arguments.
///
/// # Errors
///
/// Returns `PaintError::ParseError` if the input is not a valid paint() function.
///
/// # Example
///
/// ```rust
/// use css_paint::parser::parse_paint_function;
///
/// let parsed = parse_paint_function("paint(checkerboard, 20)").unwrap();
/// assert_eq!(parsed.name, "checkerboard");
/// assert_eq!(parsed.raw_arguments, vec!["20"]);
/// ```
pub fn parse_paint_function(input: &str) -> Result<ParsedPaintFunction, PaintError> {
    let input = input.trim();

    // Check for paint( prefix
    let content = input
        .strip_prefix("paint(")
        .ok_or_else(|| PaintError::ParseError("Expected 'paint(' prefix".to_string()))?;

    // Check for closing parenthesis
    let content = content
        .strip_suffix(')')
        .ok_or_else(|| PaintError::ParseError("Expected closing ')'".to_string()))?;

    let content = content.trim();

    if content.is_empty() {
        return Err(PaintError::ParseError(
            "paint() function requires a worklet name".to_string(),
        ));
    }

    // Split by comma, handling nested parentheses
    let parts = split_arguments(content)?;

    if parts.is_empty() {
        return Err(PaintError::ParseError(
            "paint() function requires a worklet name".to_string(),
        ));
    }

    let name = parts[0].trim().to_string();

    // Validate worklet name
    if name.is_empty() {
        return Err(PaintError::ParseError("Empty worklet name".to_string()));
    }

    if !is_valid_worklet_name(&name) {
        return Err(PaintError::ParseError(format!(
            "Invalid worklet name: '{}'",
            name
        )));
    }

    let raw_arguments: Vec<String> = parts[1..].iter().map(|s| s.trim().to_string()).collect();

    Ok(ParsedPaintFunction::new(name, raw_arguments))
}

/// Split arguments by comma, respecting nested parentheses
fn split_arguments(input: &str) -> Result<Vec<&str>, PaintError> {
    let mut parts = Vec::new();
    let mut start = 0;
    let mut depth = 0;

    for (i, ch) in input.char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => {
                if depth == 0 {
                    return Err(PaintError::ParseError("Unbalanced parentheses".to_string()));
                }
                depth -= 1;
            }
            ',' if depth == 0 => {
                parts.push(&input[start..i]);
                start = i + 1;
            }
            _ => {}
        }
    }

    if depth != 0 {
        return Err(PaintError::ParseError("Unbalanced parentheses".to_string()));
    }

    // Add the last part
    if start < input.len() {
        parts.push(&input[start..]);
    }

    Ok(parts)
}

/// Validate a worklet name
///
/// Valid names follow CSS identifier rules:
/// - Start with a letter, underscore, or hyphen
/// - Contain letters, digits, underscores, or hyphens
fn is_valid_worklet_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    let mut chars = name.chars();
    let first = chars.next().unwrap();

    // First character must be a letter, underscore, or hyphen
    if !first.is_ascii_alphabetic() && first != '_' && first != '-' {
        return false;
    }

    // Remaining characters can be letters, digits, underscores, or hyphens
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}

/// Parse a single argument based on expected type
fn parse_argument(raw: &str, expected_type: PaintArgumentType) -> Result<PaintValue, PaintError> {
    let raw = raw.trim();

    match expected_type {
        PaintArgumentType::Number => parse_number(raw),
        PaintArgumentType::Length => parse_length(raw),
        PaintArgumentType::Color => parse_color(raw),
        PaintArgumentType::Percentage => parse_percentage(raw),
        PaintArgumentType::Angle => parse_angle(raw),
        PaintArgumentType::Ident => Ok(PaintValue::Ident(raw.to_string())),
        PaintArgumentType::Image => parse_image(raw),
    }
}

/// Parse a number argument
fn parse_number(raw: &str) -> Result<PaintValue, PaintError> {
    raw.parse::<f32>()
        .map(PaintValue::Number)
        .map_err(|_| PaintError::InvalidArgumentType {
            expected: PaintArgumentType::Number,
            got: raw.to_string(),
        })
}

/// Parse a length argument (converts to pixels)
fn parse_length(raw: &str) -> Result<PaintValue, PaintError> {
    // Try to parse as plain number (assume px)
    if let Ok(num) = raw.parse::<f32>() {
        return Ok(PaintValue::Length(num));
    }

    // Parse with unit
    let raw_lower = raw.to_lowercase();

    if let Some(stripped) = raw_lower.strip_suffix("px") {
        stripped
            .trim()
            .parse::<f32>()
            .map(PaintValue::Length)
            .map_err(|_| PaintError::InvalidArgumentType {
                expected: PaintArgumentType::Length,
                got: raw.to_string(),
            })
    } else if let Some(stripped) = raw_lower.strip_suffix("rem") {
        // Convert rem to px (assume 16px base)
        // Note: Must check "rem" before "em" since "rem" ends with "em"
        stripped
            .trim()
            .parse::<f32>()
            .map(|v| PaintValue::Length(v * 16.0))
            .map_err(|_| PaintError::InvalidArgumentType {
                expected: PaintArgumentType::Length,
                got: raw.to_string(),
            })
    } else if let Some(stripped) = raw_lower.strip_suffix("em") {
        // Convert em to px (assume 16px base)
        stripped
            .trim()
            .parse::<f32>()
            .map(|v| PaintValue::Length(v * 16.0))
            .map_err(|_| PaintError::InvalidArgumentType {
                expected: PaintArgumentType::Length,
                got: raw.to_string(),
            })
    } else {
        Err(PaintError::InvalidArgumentType {
            expected: PaintArgumentType::Length,
            got: raw.to_string(),
        })
    }
}

/// Parse a color argument
fn parse_color(raw: &str) -> Result<PaintValue, PaintError> {
    Color::parse(raw)
        .map(PaintValue::Color)
        .map_err(|_| PaintError::InvalidArgumentType {
            expected: PaintArgumentType::Color,
            got: raw.to_string(),
        })
}

/// Parse a percentage argument
fn parse_percentage(raw: &str) -> Result<PaintValue, PaintError> {
    if let Some(stripped) = raw.strip_suffix('%') {
        stripped
            .trim()
            .parse::<f32>()
            .map(|v| PaintValue::Percentage(v / 100.0))
            .map_err(|_| PaintError::InvalidArgumentType {
                expected: PaintArgumentType::Percentage,
                got: raw.to_string(),
            })
    } else {
        Err(PaintError::InvalidArgumentType {
            expected: PaintArgumentType::Percentage,
            got: raw.to_string(),
        })
    }
}

/// Parse an angle argument (converts to radians)
fn parse_angle(raw: &str) -> Result<PaintValue, PaintError> {
    let raw_lower = raw.to_lowercase();

    if let Some(stripped) = raw_lower.strip_suffix("deg") {
        stripped
            .trim()
            .parse::<f32>()
            .map(|v| PaintValue::Angle(v.to_radians()))
            .map_err(|_| PaintError::InvalidArgumentType {
                expected: PaintArgumentType::Angle,
                got: raw.to_string(),
            })
    } else if let Some(stripped) = raw_lower.strip_suffix("rad") {
        stripped
            .trim()
            .parse::<f32>()
            .map(PaintValue::Angle)
            .map_err(|_| PaintError::InvalidArgumentType {
                expected: PaintArgumentType::Angle,
                got: raw.to_string(),
            })
    } else if let Some(stripped) = raw_lower.strip_suffix("turn") {
        stripped
            .trim()
            .parse::<f32>()
            .map(|v| PaintValue::Angle(v * std::f32::consts::TAU))
            .map_err(|_| PaintError::InvalidArgumentType {
                expected: PaintArgumentType::Angle,
                got: raw.to_string(),
            })
    } else if let Some(stripped) = raw_lower.strip_suffix("grad") {
        stripped
            .trim()
            .parse::<f32>()
            .map(|v| PaintValue::Angle(v * std::f32::consts::PI / 200.0))
            .map_err(|_| PaintError::InvalidArgumentType {
                expected: PaintArgumentType::Angle,
                got: raw.to_string(),
            })
    } else {
        Err(PaintError::InvalidArgumentType {
            expected: PaintArgumentType::Angle,
            got: raw.to_string(),
        })
    }
}

/// Parse an image argument
fn parse_image(raw: &str) -> Result<PaintValue, PaintError> {
    // Check for url() function
    let raw = raw.trim();

    if let Some(content) = raw.strip_prefix("url(") {
        if let Some(url) = content.strip_suffix(')') {
            // Remove quotes if present
            let url = url.trim();
            let url = url
                .strip_prefix('"')
                .and_then(|s| s.strip_suffix('"'))
                .or_else(|| url.strip_prefix('\'').and_then(|s| s.strip_suffix('\'')))
                .unwrap_or(url);

            return Ok(PaintValue::Image(url.to_string()));
        }
    }

    // Treat as raw URL
    Ok(PaintValue::Image(raw.to_string()))
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    struct TestWorklet;

    impl crate::PaintWorklet for TestWorklet {
        fn name(&self) -> &str {
            "test-worklet"
        }

        fn input_properties(&self) -> Vec<String> {
            vec![]
        }

        fn input_arguments(&self) -> Vec<PaintArgumentType> {
            vec![PaintArgumentType::Number, PaintArgumentType::Color]
        }

        fn paint(
            &self,
            _ctx: &mut crate::PaintContext,
            _size: crate::PaintSize,
            _args: &[PaintValue],
        ) {
        }
    }

    #[test]
    fn test_parse_simple_paint_function() {
        let result = parse_paint_function("paint(my-worklet)");
        assert!(result.is_ok());

        let parsed = result.unwrap();
        assert_eq!(parsed.name, "my-worklet");
        assert!(parsed.raw_arguments.is_empty());
    }

    #[test]
    fn test_parse_paint_function_with_args() {
        let result = parse_paint_function("paint(checkerboard, 20, 30)");
        assert!(result.is_ok());

        let parsed = result.unwrap();
        assert_eq!(parsed.name, "checkerboard");
        assert_eq!(parsed.raw_arguments, vec!["20", "30"]);
    }

    #[test]
    fn test_parse_paint_function_with_whitespace() {
        let result = parse_paint_function("  paint(  my-worklet  ,  arg1  ,  arg2  )  ");
        assert!(result.is_ok());

        let parsed = result.unwrap();
        assert_eq!(parsed.name, "my-worklet");
        assert_eq!(parsed.raw_arguments, vec!["arg1", "arg2"]);
    }

    #[test]
    fn test_parse_paint_function_with_nested_parens() {
        let result = parse_paint_function("paint(my-worklet, rgb(255, 0, 0))");
        assert!(result.is_ok());

        let parsed = result.unwrap();
        assert_eq!(parsed.name, "my-worklet");
        assert_eq!(parsed.raw_arguments, vec!["rgb(255, 0, 0)"]);
    }

    #[test]
    fn test_parse_paint_function_no_prefix() {
        let result = parse_paint_function("my-worklet");
        assert!(matches!(result, Err(PaintError::ParseError(_))));
    }

    #[test]
    fn test_parse_paint_function_no_closing_paren() {
        let result = parse_paint_function("paint(my-worklet");
        assert!(matches!(result, Err(PaintError::ParseError(_))));
    }

    #[test]
    fn test_parse_paint_function_empty() {
        let result = parse_paint_function("paint()");
        assert!(matches!(result, Err(PaintError::ParseError(_))));
    }

    #[test]
    fn test_parse_paint_function_invalid_name() {
        let result = parse_paint_function("paint(123invalid)");
        assert!(matches!(result, Err(PaintError::ParseError(_))));
    }

    #[test]
    fn test_is_valid_worklet_name() {
        assert!(is_valid_worklet_name("my-worklet"));
        assert!(is_valid_worklet_name("_private"));
        assert!(is_valid_worklet_name("myWorklet123"));
        assert!(is_valid_worklet_name("-webkit-test"));

        assert!(!is_valid_worklet_name(""));
        assert!(!is_valid_worklet_name("123abc"));
        assert!(!is_valid_worklet_name("my worklet"));
        assert!(!is_valid_worklet_name("my.worklet"));
    }

    #[test]
    fn test_parse_number() {
        assert_eq!(parse_number("42").unwrap(), PaintValue::Number(42.0));
        assert_eq!(parse_number("3.14").unwrap(), PaintValue::Number(3.14));
        assert_eq!(parse_number("-10").unwrap(), PaintValue::Number(-10.0));
        assert!(parse_number("abc").is_err());
    }

    #[test]
    fn test_parse_length() {
        assert_eq!(parse_length("100px").unwrap(), PaintValue::Length(100.0));
        assert_eq!(parse_length("100").unwrap(), PaintValue::Length(100.0));
        assert_eq!(parse_length("2em").unwrap(), PaintValue::Length(32.0));
        assert_eq!(parse_length("1rem").unwrap(), PaintValue::Length(16.0));
        assert!(parse_length("invalid").is_err());
    }

    #[test]
    fn test_parse_color() {
        let result = parse_color("#ff0000");
        assert!(result.is_ok());

        let result = parse_color("rgb(255, 0, 0)");
        assert!(result.is_ok());

        assert!(parse_color("not-a-color").is_err());
    }

    #[test]
    fn test_parse_percentage() {
        assert_eq!(
            parse_percentage("50%").unwrap(),
            PaintValue::Percentage(0.5)
        );
        assert_eq!(
            parse_percentage("100%").unwrap(),
            PaintValue::Percentage(1.0)
        );
        assert!(parse_percentage("50").is_err());
    }

    #[test]
    fn test_parse_angle() {
        let result = parse_angle("90deg").unwrap();
        if let PaintValue::Angle(rad) = result {
            assert!((rad - std::f32::consts::FRAC_PI_2).abs() < 0.001);
        } else {
            panic!("Expected Angle");
        }

        let result = parse_angle("1turn").unwrap();
        if let PaintValue::Angle(rad) = result {
            assert!((rad - std::f32::consts::TAU).abs() < 0.001);
        } else {
            panic!("Expected Angle");
        }

        let result = parse_angle("3.14159rad").unwrap();
        if let PaintValue::Angle(rad) = result {
            assert!((rad - std::f32::consts::PI).abs() < 0.001);
        } else {
            panic!("Expected Angle");
        }

        assert!(parse_angle("45").is_err());
    }

    #[test]
    fn test_parse_image() {
        assert_eq!(
            parse_image("url(image.png)").unwrap(),
            PaintValue::Image("image.png".to_string())
        );

        assert_eq!(
            parse_image("url(\"image.png\")").unwrap(),
            PaintValue::Image("image.png".to_string())
        );

        assert_eq!(
            parse_image("url('image.png')").unwrap(),
            PaintValue::Image("image.png".to_string())
        );

        assert_eq!(
            parse_image("image.png").unwrap(),
            PaintValue::Image("image.png".to_string())
        );
    }

    #[test]
    fn test_resolve_arguments() {
        let mut registry = PaintWorkletRegistry::new();
        registry.register(Arc::new(TestWorklet)).unwrap();

        let parsed = parse_paint_function("paint(test-worklet, 42, #ff0000)").unwrap();
        let values = parsed.resolve_arguments(&registry).unwrap();

        assert_eq!(values.len(), 2);
        assert_eq!(values[0], PaintValue::Number(42.0));
        assert!(matches!(values[1], PaintValue::Color(_)));
    }

    #[test]
    fn test_resolve_arguments_worklet_not_found() {
        let registry = PaintWorkletRegistry::new();
        let parsed = parse_paint_function("paint(nonexistent)").unwrap();

        let result = parsed.resolve_arguments(&registry);
        assert!(matches!(result, Err(PaintError::WorkletNotFound(_))));
    }

    #[test]
    fn test_resolve_arguments_count_mismatch() {
        let mut registry = PaintWorkletRegistry::new();
        registry.register(Arc::new(TestWorklet)).unwrap();

        let parsed = parse_paint_function("paint(test-worklet, 42)").unwrap();
        let result = parsed.resolve_arguments(&registry);

        assert!(matches!(
            result,
            Err(PaintError::ArgumentCountMismatch {
                expected: 2,
                got: 1
            })
        ));
    }

    #[test]
    fn test_split_arguments_balanced() {
        let parts = split_arguments("a, b, c").unwrap();
        assert_eq!(parts, vec!["a", " b", " c"]);

        let parts = split_arguments("rgb(1, 2, 3), other").unwrap();
        assert_eq!(parts, vec!["rgb(1, 2, 3)", " other"]);
    }

    #[test]
    fn test_split_arguments_unbalanced() {
        let result = split_arguments("rgb(1, 2");
        assert!(result.is_err());

        let result = split_arguments("a, b)");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_complex_paint_function() {
        let result =
            parse_paint_function("paint(custom-gradient, rgba(255, 0, 0, 0.5), 45deg, 100px)");
        assert!(result.is_ok());

        let parsed = result.unwrap();
        assert_eq!(parsed.name, "custom-gradient");
        assert_eq!(parsed.raw_arguments.len(), 3);
        assert_eq!(parsed.raw_arguments[0], "rgba(255, 0, 0, 0.5)");
        assert_eq!(parsed.raw_arguments[1], "45deg");
        assert_eq!(parsed.raw_arguments[2], "100px");
    }
}
