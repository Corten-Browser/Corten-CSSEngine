//! CSS Custom Properties - CSS variables and calc() function
//!
//! This module provides support for CSS custom properties (variables) and calc() expressions.
//!
//! # Examples
//!
//! ```
//! use css_custom_properties::*;
//!
//! // Parse a custom property definition
//! let prop = parse_custom_property("--primary-color: #FF5733").unwrap();
//! assert_eq!(prop.name(), "--primary-color");
//! assert_eq!(prop.value(), "#FF5733");
//!
//! // Parse a var() reference
//! let var_ref = parse_var_reference("var(--primary-color, red)").unwrap();
//! assert_eq!(var_ref.name(), "--primary-color");
//! assert_eq!(var_ref.fallback(), Some("red"));
//!
//! // Parse a calc() expression
//! let calc = parse_calc_expression("calc(100% - 20px)").unwrap();
//! ```

use css_types::{CssError, Length, LengthUnit};

// ============================================================================
// Custom Property Types
// ============================================================================

/// CSS custom property definition (e.g., --primary-color: #FF5733)
#[derive(Debug, Clone, PartialEq)]
pub struct CustomProperty {
    name: String,
    value: String,
    inherited: bool,
}

impl CustomProperty {
    /// Create a new custom property
    ///
    /// # Examples
    /// ```
    /// use css_custom_properties::CustomProperty;
    ///
    /// let prop = CustomProperty::new("--color", "blue", true);
    /// assert_eq!(prop.name(), "--color");
    /// ```
    pub fn new(name: impl Into<String>, value: impl Into<String>, inherited: bool) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            inherited,
        }
    }

    /// Get the property name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the property value
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Check if the property is inherited
    pub fn inherited(&self) -> bool {
        self.inherited
    }
}

/// Variable reference with optional fallback (e.g., var(--color, red))
#[derive(Debug, Clone, PartialEq)]
pub struct VariableReference {
    name: String,
    fallback: Option<String>,
}

impl VariableReference {
    /// Create a new variable reference with fallback
    ///
    /// # Examples
    /// ```
    /// use css_custom_properties::VariableReference;
    ///
    /// let var_ref = VariableReference::with_fallback("--color", "red");
    /// assert_eq!(var_ref.name(), "--color");
    /// assert_eq!(var_ref.fallback(), Some("red"));
    /// ```
    pub fn with_fallback(name: impl Into<String>, fallback: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            fallback: Some(fallback.into()),
        }
    }

    /// Create a new variable reference without fallback
    ///
    /// # Examples
    /// ```
    /// use css_custom_properties::VariableReference;
    ///
    /// let var_ref = VariableReference::new("--color");
    /// assert_eq!(var_ref.name(), "--color");
    /// assert_eq!(var_ref.fallback(), None);
    /// ```
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            fallback: None,
        }
    }

    /// Get the variable name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the fallback value
    pub fn fallback(&self) -> Option<&str> {
        self.fallback.as_deref()
    }
}

// ============================================================================
// Calc Expression Types
// ============================================================================

/// Value in a calc() expression
#[derive(Debug, Clone, PartialEq)]
pub enum CalcValue {
    /// Numeric value (unitless)
    Number(f32),
    /// Length value with unit
    Length(Length),
    /// Percentage value
    Percentage(f32),
}

impl CalcValue {
    /// Evaluate the calc value to pixels
    fn to_pixels(&self, context: &CalcContext) -> f32 {
        match self {
            CalcValue::Number(n) => *n,
            CalcValue::Length(length) => match length.unit() {
                LengthUnit::Px => length.value(),
                LengthUnit::Em => length.value() * context.font_size,
                LengthUnit::Rem => length.value() * context.font_size, // Simplified
                LengthUnit::Percent => length.value() * context.viewport_width / 100.0,
                LengthUnit::Vw => length.value() * context.viewport_width / 100.0,
                LengthUnit::Vh => length.value() * context.viewport_width / 100.0, // Simplified
            },
            CalcValue::Percentage(pct) => pct * context.viewport_width / 100.0,
        }
    }
}

/// Calc expression tree
#[derive(Debug, Clone, PartialEq)]
pub enum CalcExpression {
    /// Simple value
    Value(CalcValue),
    /// Addition: left + right
    Add(Box<CalcExpression>, Box<CalcExpression>),
    /// Subtraction: left - right
    Subtract(Box<CalcExpression>, Box<CalcExpression>),
    /// Multiplication: value * number
    Multiply(Box<CalcExpression>, f32),
    /// Division: value / number
    Divide(Box<CalcExpression>, f32),
}

impl CalcExpression {
    /// Evaluate the calc expression to a pixel value
    ///
    /// # Examples
    /// ```
    /// use css_custom_properties::{CalcExpression, CalcValue, CalcContext};
    /// use css_types::{Length, LengthUnit};
    ///
    /// let expr = CalcExpression::Add(
    ///     Box::new(CalcExpression::Value(CalcValue::Length(Length::new(10.0, LengthUnit::Px)))),
    ///     Box::new(CalcExpression::Value(CalcValue::Length(Length::new(20.0, LengthUnit::Px)))),
    /// );
    ///
    /// let context = CalcContext::new(100.0, 16.0);
    /// let result = expr.evaluate(&context);
    /// assert!((result - 30.0).abs() < 0.01);
    /// ```
    pub fn evaluate(&self, context: &CalcContext) -> f32 {
        match self {
            CalcExpression::Value(val) => val.to_pixels(context),
            CalcExpression::Add(left, right) => left.evaluate(context) + right.evaluate(context),
            CalcExpression::Subtract(left, right) => {
                left.evaluate(context) - right.evaluate(context)
            }
            CalcExpression::Multiply(expr, multiplier) => expr.evaluate(context) * multiplier,
            CalcExpression::Divide(expr, divisor) => {
                if *divisor != 0.0 {
                    expr.evaluate(context) / divisor
                } else {
                    0.0
                }
            }
        }
    }
}

/// Context for evaluating calc() expressions
#[derive(Debug, Clone, PartialEq)]
pub struct CalcContext {
    /// Viewport width in pixels
    pub viewport_width: f32,
    /// Font size in pixels
    pub font_size: f32,
}

impl CalcContext {
    /// Create a new calc context
    ///
    /// # Examples
    /// ```
    /// use css_custom_properties::CalcContext;
    ///
    /// let context = CalcContext::new(1920.0, 16.0);
    /// assert_eq!(context.viewport_width, 1920.0);
    /// assert_eq!(context.font_size, 16.0);
    /// ```
    pub fn new(viewport_width: f32, font_size: f32) -> Self {
        Self {
            viewport_width,
            font_size,
        }
    }
}

// ============================================================================
// Custom Property Resolver Trait
// ============================================================================

/// Trait for resolving custom properties and calc() expressions
pub trait CustomPropertyResolver {
    /// Set a custom property value
    fn set_custom_property(&mut self, name: &str, value: &str);

    /// Get a custom property value with inheritance
    fn get_custom_property(&self, name: &str) -> Option<String>;

    /// Resolve a var() reference to its value
    fn resolve_var(&self, var_ref: &VariableReference) -> String;

    /// Evaluate a calc() expression to a pixel value
    fn evaluate_calc(&self, expr: &CalcExpression, context: &CalcContext) -> f32;
}

// ============================================================================
// Parsing Functions
// ============================================================================

/// Parse a custom property definition (e.g., "--primary-color: #FF5733")
///
/// # Examples
/// ```
/// use css_custom_properties::parse_custom_property;
///
/// let prop = parse_custom_property("--primary-color: #FF5733").unwrap();
/// assert_eq!(prop.name(), "--primary-color");
/// assert_eq!(prop.value(), "#FF5733");
/// ```
///
/// # Errors
/// Returns an error if the input is not a valid custom property definition
pub fn parse_custom_property(input: &str) -> Result<CustomProperty, CssError> {
    let input = input.trim();

    // Find the colon separator
    let colon_pos = input
        .find(':')
        .ok_or_else(|| CssError::ParseError("Missing colon in property definition".to_string()))?;

    let name = input[..colon_pos].trim();
    let value = input[colon_pos + 1..].trim();

    // Validate that name starts with --
    if !name.starts_with("--") {
        return Err(CssError::ParseError(
            "Custom property name must start with --".to_string(),
        ));
    }

    // Custom properties are inherited by default
    Ok(CustomProperty::new(name, value, true))
}

/// Parse a var() reference (e.g., "var(--color, red)")
///
/// # Examples
/// ```
/// use css_custom_properties::parse_var_reference;
///
/// let var_ref = parse_var_reference("var(--color, red)").unwrap();
/// assert_eq!(var_ref.name(), "--color");
/// assert_eq!(var_ref.fallback(), Some("red"));
/// ```
///
/// # Errors
/// Returns an error if the input is not a valid var() reference
pub fn parse_var_reference(input: &str) -> Result<VariableReference, CssError> {
    let input = input.trim();

    // Check for var( prefix
    if !input.starts_with("var(") {
        return Err(CssError::ParseError(
            "Variable reference must start with var(".to_string(),
        ));
    }

    // Check for closing paren
    if !input.ends_with(')') {
        return Err(CssError::ParseError(
            "Variable reference must end with )".to_string(),
        ));
    }

    // Extract content between var( and )
    let content = &input[4..input.len() - 1];

    // Split by comma to separate name and fallback
    let parts: Vec<&str> = content.splitn(2, ',').collect();

    let name = parts[0].trim();

    if parts.len() > 1 {
        let fallback = parts[1].trim();
        Ok(VariableReference::with_fallback(name, fallback))
    } else {
        Ok(VariableReference::new(name))
    }
}

/// Parse a calc() expression (e.g., "calc(100% - 20px)")
///
/// # Examples
/// ```
/// use css_custom_properties::parse_calc_expression;
///
/// let expr = parse_calc_expression("calc(100% - 20px)").unwrap();
/// ```
///
/// # Errors
/// Returns an error if the input is not a valid calc() expression
pub fn parse_calc_expression(input: &str) -> Result<CalcExpression, CssError> {
    let input = input.trim();

    // Check for calc( prefix
    if !input.starts_with("calc(") {
        return Err(CssError::ParseError(
            "Calc expression must start with calc(".to_string(),
        ));
    }

    // Check for closing paren
    if !input.ends_with(')') {
        return Err(CssError::ParseError(
            "Calc expression must end with )".to_string(),
        ));
    }

    // Extract content between calc( and )
    let content = &input[5..input.len() - 1].trim();

    parse_calc_content(content)
}

/// Parse the content inside calc()
fn parse_calc_content(content: &str) -> Result<CalcExpression, CssError> {
    let content = content.trim();

    // Handle nested parentheses
    if content.starts_with('(') && content.ends_with(')') {
        // Remove outer parens and parse recursively
        let inner = &content[1..content.len() - 1];
        return parse_calc_content(inner);
    }

    // Try to find operators (division has highest precedence, then multiplication, then +/-)
    // For simplicity, we'll parse from left to right

    // Look for + or - (lowest precedence)
    if let Some(pos) = find_operator(content, &['+', '-']) {
        let left = parse_calc_content(content[..pos].trim())?;
        let right = parse_calc_content(content[pos + 1..].trim())?;

        return Ok(if content.chars().nth(pos) == Some('+') {
            CalcExpression::Add(Box::new(left), Box::new(right))
        } else {
            CalcExpression::Subtract(Box::new(left), Box::new(right))
        });
    }

    // Look for * or /
    if let Some(pos) = find_operator(content, &['*', '/']) {
        let left_str = content[..pos].trim();
        let right_str = content[pos + 1..].trim();

        // For * and /, one side should be a value and the other a number
        let is_multiply = content.chars().nth(pos) == Some('*');

        // Try to parse right side as number first
        if let Ok(num) = right_str.parse::<f32>() {
            let expr = parse_calc_content(left_str)?;
            return Ok(if is_multiply {
                CalcExpression::Multiply(Box::new(expr), num)
            } else {
                CalcExpression::Divide(Box::new(expr), num)
            });
        }

        // Try to parse left side as number
        if let Ok(num) = left_str.parse::<f32>() {
            let expr = parse_calc_content(right_str)?;
            return Ok(if is_multiply {
                CalcExpression::Multiply(Box::new(expr), num)
            } else {
                // For division, we need to flip this
                CalcExpression::Divide(Box::new(expr), num)
            });
        }
    }

    // Try to parse as a simple value
    parse_calc_value(content).map(CalcExpression::Value)
}

/// Find the position of an operator at the top level (not inside parentheses)
fn find_operator(content: &str, operators: &[char]) -> Option<usize> {
    let mut paren_depth = 0;
    for (i, ch) in content.chars().enumerate() {
        match ch {
            '(' => paren_depth += 1,
            ')' => paren_depth -= 1,
            _ if paren_depth == 0 && operators.contains(&ch) => return Some(i),
            _ => {}
        }
    }
    None
}

/// Parse a calc value (number, length, or percentage)
fn parse_calc_value(content: &str) -> Result<CalcValue, CssError> {
    let content = content.trim();

    // Try to parse as percentage
    if let Some(num_str) = content.strip_suffix('%') {
        let value = num_str
            .parse::<f32>()
            .map_err(|_| CssError::ParseError("Invalid percentage value".to_string()))?;
        return Ok(CalcValue::Percentage(value));
    }

    // Try to parse as length
    if let Ok(length) = css_types::CssValue::parse(content) {
        return Ok(CalcValue::Length(length));
    }

    // Try to parse as plain number
    if let Ok(num) = content.parse::<f32>() {
        return Ok(CalcValue::Number(num));
    }

    Err(CssError::ParseError(format!(
        "Invalid calc value: {}",
        content
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_property_basic() {
        let prop = CustomProperty::new("--color", "red", true);
        assert_eq!(prop.name(), "--color");
        assert_eq!(prop.value(), "red");
        assert!(prop.inherited());
    }

    #[test]
    fn test_variable_reference_basic() {
        let var_ref = VariableReference::with_fallback("--size", "10px");
        assert_eq!(var_ref.name(), "--size");
        assert_eq!(var_ref.fallback(), Some("10px"));
    }

    #[test]
    fn test_calc_value_to_pixels() {
        let context = CalcContext::new(100.0, 16.0);

        let val = CalcValue::Number(10.0);
        assert_eq!(val.to_pixels(&context), 10.0);

        let val = CalcValue::Percentage(50.0);
        assert_eq!(val.to_pixels(&context), 50.0);

        let val = CalcValue::Length(Length::new(10.0, LengthUnit::Px));
        assert_eq!(val.to_pixels(&context), 10.0);
    }
}
