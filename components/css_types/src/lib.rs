//! CSS Types - Base type definitions for CSS Engine
//!
//! This module provides core CSS types including:
//! - Color (RGB/RGBA)
//! - Length (with units: px, em, rem, %, vw, vh)
//! - Specificity (selector specificity calculation)
//! - CssError (error handling)
//! - CssValue trait (parsing and serialization)

use std::cmp::Ordering;
use std::fmt;

// ============================================================================
// Error Types
// ============================================================================

/// Error type for CSS operations
#[derive(Debug, Clone, PartialEq)]
pub enum CssError {
    /// Parse error with description
    ParseError(String),
    /// Invalid value error with description
    InvalidValue(String),
    /// Value out of valid range with description
    OutOfRange(String),
}

impl fmt::Display for CssError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CssError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            CssError::InvalidValue(msg) => write!(f, "Invalid value: {}", msg),
            CssError::OutOfRange(msg) => write!(f, "Out of range: {}", msg),
        }
    }
}

impl std::error::Error for CssError {}

// ============================================================================
// CssValue Trait
// ============================================================================

/// Trait for CSS values that can be parsed and serialized
pub trait CssValue: Sized {
    /// Parse a CSS value from a string
    fn parse(input: &str) -> Result<Self, CssError>;

    /// Serialize the CSS value to a string
    fn serialize(&self) -> String;
}

// ============================================================================
// Color Type
// ============================================================================

/// RGB/RGBA color representation
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: f32,
}

impl Color {
    /// Create a new RGB color (alpha defaults to 1.0)
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    /// Create a new RGBA color
    pub fn rgba(r: u8, g: u8, b: u8, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Get the red component
    pub fn r(&self) -> u8 {
        self.r
    }

    /// Get the green component
    pub fn g(&self) -> u8 {
        self.g
    }

    /// Get the blue component
    pub fn b(&self) -> u8 {
        self.b
    }

    /// Get the alpha component
    pub fn a(&self) -> f32 {
        self.a
    }

    /// Parse a hex color string (#RGB or #RRGGBB)
    fn parse_hex(input: &str) -> Result<Self, CssError> {
        if !input.starts_with('#') {
            return Err(CssError::ParseError(
                "Hex color must start with #".to_string(),
            ));
        }

        let hex = &input[1..];

        match hex.len() {
            3 => {
                // #RGB -> #RRGGBB
                let r = u8::from_str_radix(&hex[0..1].repeat(2), 16)
                    .map_err(|_| CssError::ParseError("Invalid hex digit".to_string()))?;
                let g = u8::from_str_radix(&hex[1..2].repeat(2), 16)
                    .map_err(|_| CssError::ParseError("Invalid hex digit".to_string()))?;
                let b = u8::from_str_radix(&hex[2..3].repeat(2), 16)
                    .map_err(|_| CssError::ParseError("Invalid hex digit".to_string()))?;
                Ok(Self::rgb(r, g, b))
            }
            6 => {
                // #RRGGBB
                let r = u8::from_str_radix(&hex[0..2], 16)
                    .map_err(|_| CssError::ParseError("Invalid hex digit".to_string()))?;
                let g = u8::from_str_radix(&hex[2..4], 16)
                    .map_err(|_| CssError::ParseError("Invalid hex digit".to_string()))?;
                let b = u8::from_str_radix(&hex[4..6], 16)
                    .map_err(|_| CssError::ParseError("Invalid hex digit".to_string()))?;
                Ok(Self::rgb(r, g, b))
            }
            _ => Err(CssError::ParseError(
                "Hex color must be 3 or 6 digits".to_string(),
            )),
        }
    }

    /// Parse an rgb() or rgba() function
    fn parse_rgb_function(input: &str) -> Result<Self, CssError> {
        let input = input.trim();

        let (is_rgba, content) = if let Some(stripped) = input.strip_prefix("rgba(") {
            (true, stripped)
        } else if let Some(stripped) = input.strip_prefix("rgb(") {
            (false, stripped)
        } else {
            return Err(CssError::ParseError(
                "Invalid rgb/rgba function".to_string(),
            ));
        };

        let content = content
            .strip_suffix(')')
            .ok_or_else(|| CssError::ParseError("Missing closing parenthesis".to_string()))?;

        let parts: Vec<&str> = content.split(',').map(|s| s.trim()).collect();

        if is_rgba {
            if parts.len() != 4 {
                return Err(CssError::ParseError("rgba() requires 4 values".to_string()));
            }

            let r = parse_color_component(parts[0])?;
            let g = parse_color_component(parts[1])?;
            let b = parse_color_component(parts[2])?;
            let a = parts[3]
                .parse::<f32>()
                .map_err(|_| CssError::ParseError("Invalid alpha value".to_string()))?;

            if !(0.0..=1.0).contains(&a) {
                return Err(CssError::OutOfRange(
                    "Alpha must be between 0 and 1".to_string(),
                ));
            }

            Ok(Self::rgba(r, g, b, a))
        } else {
            if parts.len() != 3 {
                return Err(CssError::ParseError("rgb() requires 3 values".to_string()));
            }

            let r = parse_color_component(parts[0])?;
            let g = parse_color_component(parts[1])?;
            let b = parse_color_component(parts[2])?;

            Ok(Self::rgb(r, g, b))
        }
    }
}

/// Parse a color component (0-255)
fn parse_color_component(s: &str) -> Result<u8, CssError> {
    let value = s
        .trim()
        .parse::<u16>()
        .map_err(|_| CssError::ParseError("Invalid color component".to_string()))?;

    if value > 255 {
        return Err(CssError::OutOfRange(format!(
            "Color component {} must be 0-255",
            value
        )));
    }

    Ok(value as u8)
}

impl CssValue for Color {
    fn parse(input: &str) -> Result<Self, CssError> {
        let input = input.trim();

        if input.is_empty() {
            return Err(CssError::ParseError("Empty color string".to_string()));
        }

        if input.starts_with('#') {
            Self::parse_hex(input)
        } else if input.starts_with("rgb") {
            Self::parse_rgb_function(input)
        } else {
            Err(CssError::ParseError("Unknown color format".to_string()))
        }
    }

    fn serialize(&self) -> String {
        if self.a < 1.0 {
            format!("rgba({}, {}, {}, {})", self.r, self.g, self.b, self.a)
        } else {
            format!("rgb({}, {}, {})", self.r, self.g, self.b)
        }
    }
}

// ============================================================================
// Length Types
// ============================================================================

/// CSS length units
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LengthUnit {
    /// Pixels
    Px,
    /// Relative to font size
    Em,
    /// Relative to root font size
    Rem,
    /// Percentage
    Percent,
    /// Viewport width
    Vw,
    /// Viewport height
    Vh,
}

impl LengthUnit {
    /// Parse a unit string
    fn parse(s: &str) -> Result<Self, CssError> {
        match s {
            "px" => Ok(LengthUnit::Px),
            "em" => Ok(LengthUnit::Em),
            "rem" => Ok(LengthUnit::Rem),
            "%" => Ok(LengthUnit::Percent),
            "vw" => Ok(LengthUnit::Vw),
            "vh" => Ok(LengthUnit::Vh),
            _ => Err(CssError::ParseError(format!("Unknown unit: {}", s))),
        }
    }

    /// Convert unit to string
    fn to_str(self) -> &'static str {
        match self {
            LengthUnit::Px => "px",
            LengthUnit::Em => "em",
            LengthUnit::Rem => "rem",
            LengthUnit::Percent => "%",
            LengthUnit::Vw => "vw",
            LengthUnit::Vh => "vh",
        }
    }
}

/// CSS length value with unit
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Length {
    value: f32,
    unit: LengthUnit,
}

impl Length {
    /// Create a new length
    pub fn new(value: f32, unit: LengthUnit) -> Self {
        Self { value, unit }
    }

    /// Get the numeric value
    pub fn value(&self) -> f32 {
        self.value
    }

    /// Get the unit
    pub fn unit(&self) -> LengthUnit {
        self.unit
    }
}

impl CssValue for Length {
    fn parse(input: &str) -> Result<Self, CssError> {
        let input = input.trim();

        if input.is_empty() {
            return Err(CssError::ParseError("Empty length string".to_string()));
        }

        // Find where the number ends and the unit begins
        let mut num_end = 0;
        for (i, ch) in input.chars().enumerate() {
            if ch.is_ascii_digit() || ch == '.' || ch == '-' || ch == '+' {
                num_end = i + 1;
            } else {
                break;
            }
        }

        if num_end == 0 {
            return Err(CssError::ParseError(
                "Length must start with a number".to_string(),
            ));
        }

        let value_str = &input[..num_end];
        let unit_str = &input[num_end..];

        if unit_str.is_empty() {
            return Err(CssError::ParseError("Length must have a unit".to_string()));
        }

        let value = value_str
            .parse::<f32>()
            .map_err(|_| CssError::ParseError("Invalid number".to_string()))?;

        let unit = LengthUnit::parse(unit_str)?;

        Ok(Self::new(value, unit))
    }

    fn serialize(&self) -> String {
        format!("{}{}", self.value, self.unit.to_str())
    }
}

// ============================================================================
// Specificity Type
// ============================================================================

/// CSS selector specificity (a, b, c)
/// - a: ID selectors
/// - b: Class selectors, attribute selectors, pseudo-classes
/// - c: Type selectors, pseudo-elements
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Specificity {
    id_selectors: u32,
    class_selectors: u32,
    type_selectors: u32,
}

impl Specificity {
    /// Create a new specificity value
    pub fn new(id_selectors: u32, class_selectors: u32, type_selectors: u32) -> Self {
        Self {
            id_selectors,
            class_selectors,
            type_selectors,
        }
    }

    /// Create a zero specificity
    pub fn zero() -> Self {
        Self::new(0, 0, 0)
    }

    /// Get the ID selector count
    pub fn id_selectors(&self) -> u32 {
        self.id_selectors
    }

    /// Get the class selector count
    pub fn class_selectors(&self) -> u32 {
        self.class_selectors
    }

    /// Get the type selector count
    pub fn type_selectors(&self) -> u32 {
        self.type_selectors
    }

    /// Get the maximum of two specificities
    pub fn max(self, other: Self) -> Self {
        if self >= other {
            self
        } else {
            other
        }
    }

    /// Get the minimum of two specificities
    pub fn min(self, other: Self) -> Self {
        if self <= other {
            self
        } else {
            other
        }
    }
}

impl PartialOrd for Specificity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Specificity {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare ID selectors first (most specific)
        match self.id_selectors.cmp(&other.id_selectors) {
            Ordering::Equal => {
                // Then compare class selectors
                match self.class_selectors.cmp(&other.class_selectors) {
                    Ordering::Equal => {
                        // Finally compare type selectors (least specific)
                        self.type_selectors.cmp(&other.type_selectors)
                    }
                    other => other,
                }
            }
            other => other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_color() {
        let color = Color::rgb(255, 0, 0);
        assert_eq!(color.r(), 255);
        assert_eq!(color.g(), 0);
        assert_eq!(color.b(), 0);
    }

    #[test]
    fn test_basic_length() {
        let length = Length::new(10.0, LengthUnit::Px);
        assert_eq!(length.value(), 10.0);
        assert_eq!(length.unit(), LengthUnit::Px);
    }

    #[test]
    fn test_basic_specificity() {
        let spec = Specificity::new(1, 2, 3);
        assert_eq!(spec.id_selectors(), 1);
        assert_eq!(spec.class_selectors(), 2);
        assert_eq!(spec.type_selectors(), 3);
    }
}
