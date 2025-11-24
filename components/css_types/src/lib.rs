//! CSS Types - Base type definitions for CSS Engine
//!
//! This module provides core CSS types including:
//! - Color (RGB/RGBA)
//! - Length (with units: px, em, rem, %, vw, vh, cqw, cqh, cqi, cqb, cqmin, cqmax)
//! - Specificity (selector specificity calculation)
//! - CssError (error handling)
//! - CssValue trait (parsing and serialization)
//! - Container Query types (ContainerType, ContainerCondition, SizeCondition)

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
    // Container-relative units (CSS Container Queries)
    /// Container query width (1% of query container's width)
    Cqw,
    /// Container query height (1% of query container's height)
    Cqh,
    /// Container query inline size (1% of query container's inline size)
    Cqi,
    /// Container query block size (1% of query container's block size)
    Cqb,
    /// Container query minimum (smaller of cqi or cqb)
    Cqmin,
    /// Container query maximum (larger of cqi or cqb)
    Cqmax,
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
            // Container-relative units
            "cqw" => Ok(LengthUnit::Cqw),
            "cqh" => Ok(LengthUnit::Cqh),
            "cqi" => Ok(LengthUnit::Cqi),
            "cqb" => Ok(LengthUnit::Cqb),
            "cqmin" => Ok(LengthUnit::Cqmin),
            "cqmax" => Ok(LengthUnit::Cqmax),
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
            // Container-relative units
            LengthUnit::Cqw => "cqw",
            LengthUnit::Cqh => "cqh",
            LengthUnit::Cqi => "cqi",
            LengthUnit::Cqb => "cqb",
            LengthUnit::Cqmin => "cqmin",
            LengthUnit::Cqmax => "cqmax",
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

// ============================================================================
// Container Query Types (CSS Container Queries Level 3)
// ============================================================================

/// Container type for the container-type property
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ContainerType {
    /// No containment (default)
    #[default]
    Normal,
    /// Size containment on both axes
    Size,
    /// Size containment on inline axis only
    InlineSize,
}

impl ContainerType {
    /// Parse a container-type value
    pub fn parse(s: &str) -> Result<Self, CssError> {
        match s.trim().to_lowercase().as_str() {
            "normal" => Ok(ContainerType::Normal),
            "size" => Ok(ContainerType::Size),
            "inline-size" => Ok(ContainerType::InlineSize),
            _ => Err(CssError::ParseError(format!(
                "Invalid container-type: {}",
                s
            ))),
        }
    }

    /// Serialize to CSS string
    pub fn to_css(&self) -> &'static str {
        match self {
            ContainerType::Normal => "normal",
            ContainerType::Size => "size",
            ContainerType::InlineSize => "inline-size",
        }
    }
}

/// Comparison operator for container queries
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Comparison {
    /// Equal to (=)
    Equal,
    /// Less than (<)
    LessThan,
    /// Less than or equal (<=)
    LessThanOrEqual,
    /// Greater than (>)
    GreaterThan,
    /// Greater than or equal (>=)
    GreaterThanOrEqual,
}

impl Comparison {
    /// Parse a comparison operator
    pub fn parse(s: &str) -> Result<Self, CssError> {
        match s.trim() {
            "=" => Ok(Comparison::Equal),
            "<" => Ok(Comparison::LessThan),
            "<=" => Ok(Comparison::LessThanOrEqual),
            ">" => Ok(Comparison::GreaterThan),
            ">=" => Ok(Comparison::GreaterThanOrEqual),
            _ => Err(CssError::ParseError(format!(
                "Invalid comparison operator: {}",
                s
            ))),
        }
    }

    /// Serialize to CSS string
    pub fn to_css(&self) -> &'static str {
        match self {
            Comparison::Equal => "=",
            Comparison::LessThan => "<",
            Comparison::LessThanOrEqual => "<=",
            Comparison::GreaterThan => ">",
            Comparison::GreaterThanOrEqual => ">=",
        }
    }
}

/// Size feature for container size queries
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SizeFeature {
    /// Width of the container
    Width,
    /// Height of the container
    Height,
    /// Inline size (width in horizontal writing modes)
    InlineSize,
    /// Block size (height in horizontal writing modes)
    BlockSize,
    /// Aspect ratio
    AspectRatio,
    /// Orientation (portrait/landscape)
    Orientation,
}

impl SizeFeature {
    /// Parse a size feature name
    pub fn parse(s: &str) -> Result<Self, CssError> {
        match s.trim().to_lowercase().as_str() {
            "width" => Ok(SizeFeature::Width),
            "height" => Ok(SizeFeature::Height),
            "inline-size" => Ok(SizeFeature::InlineSize),
            "block-size" => Ok(SizeFeature::BlockSize),
            "aspect-ratio" => Ok(SizeFeature::AspectRatio),
            "orientation" => Ok(SizeFeature::Orientation),
            _ => Err(CssError::ParseError(format!(
                "Invalid size feature: {}",
                s
            ))),
        }
    }

    /// Serialize to CSS string
    pub fn to_css(&self) -> &'static str {
        match self {
            SizeFeature::Width => "width",
            SizeFeature::Height => "height",
            SizeFeature::InlineSize => "inline-size",
            SizeFeature::BlockSize => "block-size",
            SizeFeature::AspectRatio => "aspect-ratio",
            SizeFeature::Orientation => "orientation",
        }
    }
}

/// Size condition for container queries
#[derive(Debug, Clone, PartialEq)]
pub enum SizeCondition {
    /// Minimum width: (min-width: value)
    MinWidth(Length),
    /// Maximum width: (max-width: value)
    MaxWidth(Length),
    /// Minimum height: (min-height: value)
    MinHeight(Length),
    /// Maximum height: (max-height: value)
    MaxHeight(Length),
    /// Width comparison: (width > value), (width = value), etc.
    Width(Comparison, Length),
    /// Height comparison: (height > value), etc.
    Height(Comparison, Length),
    /// Inline size comparison
    InlineSize(Comparison, Length),
    /// Block size comparison
    BlockSize(Comparison, Length),
    /// Aspect ratio: (aspect-ratio: value)
    AspectRatio(f32),
    /// Orientation: (orientation: portrait) or (orientation: landscape)
    Orientation(Orientation),
}

/// Orientation values for container queries
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    /// Portrait orientation (height > width)
    Portrait,
    /// Landscape orientation (width > height)
    Landscape,
}

impl Orientation {
    /// Parse orientation value
    pub fn parse(s: &str) -> Result<Self, CssError> {
        match s.trim().to_lowercase().as_str() {
            "portrait" => Ok(Orientation::Portrait),
            "landscape" => Ok(Orientation::Landscape),
            _ => Err(CssError::ParseError(format!(
                "Invalid orientation: {}",
                s
            ))),
        }
    }

    /// Serialize to CSS string
    pub fn to_css(&self) -> &'static str {
        match self {
            Orientation::Portrait => "portrait",
            Orientation::Landscape => "landscape",
        }
    }
}

/// Style condition for container style queries
#[derive(Debug, Clone, PartialEq)]
pub struct StyleCondition {
    /// Custom property name (e.g., "--theme")
    pub property: String,
    /// Value to check for
    pub value: String,
}

impl StyleCondition {
    /// Create a new style condition
    pub fn new(property: impl Into<String>, value: impl Into<String>) -> Self {
        StyleCondition {
            property: property.into(),
            value: value.into(),
        }
    }
}

/// Container query condition
#[derive(Debug, Clone, PartialEq)]
pub enum ContainerCondition {
    /// Size query condition
    Size(SizeCondition),
    /// Style query condition: style(property: value)
    Style(StyleCondition),
    /// Negation: not (condition)
    Not(Box<ContainerCondition>),
    /// Conjunction: (condition) and (condition)
    And(Vec<ContainerCondition>),
    /// Disjunction: (condition) or (condition)
    Or(Vec<ContainerCondition>),
}

impl ContainerCondition {
    /// Create a size condition
    pub fn size(condition: SizeCondition) -> Self {
        ContainerCondition::Size(condition)
    }

    /// Create a style condition
    pub fn style(property: impl Into<String>, value: impl Into<String>) -> Self {
        ContainerCondition::Style(StyleCondition::new(property, value))
    }

    /// Create a negated condition
    pub fn not(condition: ContainerCondition) -> Self {
        ContainerCondition::Not(Box::new(condition))
    }

    /// Create an AND condition
    pub fn and(conditions: Vec<ContainerCondition>) -> Self {
        ContainerCondition::And(conditions)
    }

    /// Create an OR condition
    pub fn or(conditions: Vec<ContainerCondition>) -> Self {
        ContainerCondition::Or(conditions)
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

    // Container Query Unit Tests

    #[test]
    fn test_container_relative_units_parse() {
        assert_eq!(LengthUnit::parse("cqw").unwrap(), LengthUnit::Cqw);
        assert_eq!(LengthUnit::parse("cqh").unwrap(), LengthUnit::Cqh);
        assert_eq!(LengthUnit::parse("cqi").unwrap(), LengthUnit::Cqi);
        assert_eq!(LengthUnit::parse("cqb").unwrap(), LengthUnit::Cqb);
        assert_eq!(LengthUnit::parse("cqmin").unwrap(), LengthUnit::Cqmin);
        assert_eq!(LengthUnit::parse("cqmax").unwrap(), LengthUnit::Cqmax);
    }

    #[test]
    fn test_container_relative_units_to_str() {
        assert_eq!(LengthUnit::Cqw.to_str(), "cqw");
        assert_eq!(LengthUnit::Cqh.to_str(), "cqh");
        assert_eq!(LengthUnit::Cqi.to_str(), "cqi");
        assert_eq!(LengthUnit::Cqb.to_str(), "cqb");
        assert_eq!(LengthUnit::Cqmin.to_str(), "cqmin");
        assert_eq!(LengthUnit::Cqmax.to_str(), "cqmax");
    }

    #[test]
    fn test_length_with_container_units() {
        let length = Length::new(50.0, LengthUnit::Cqw);
        assert_eq!(length.value(), 50.0);
        assert_eq!(length.unit(), LengthUnit::Cqw);
        assert_eq!(length.serialize(), "50cqw");
    }

    // Container Type Tests

    #[test]
    fn test_container_type_parse() {
        assert_eq!(ContainerType::parse("normal").unwrap(), ContainerType::Normal);
        assert_eq!(ContainerType::parse("size").unwrap(), ContainerType::Size);
        assert_eq!(ContainerType::parse("inline-size").unwrap(), ContainerType::InlineSize);
        assert_eq!(ContainerType::parse("NORMAL").unwrap(), ContainerType::Normal);
        assert!(ContainerType::parse("invalid").is_err());
    }

    #[test]
    fn test_container_type_to_css() {
        assert_eq!(ContainerType::Normal.to_css(), "normal");
        assert_eq!(ContainerType::Size.to_css(), "size");
        assert_eq!(ContainerType::InlineSize.to_css(), "inline-size");
    }

    #[test]
    fn test_container_type_default() {
        assert_eq!(ContainerType::default(), ContainerType::Normal);
    }

    // Comparison Tests

    #[test]
    fn test_comparison_parse() {
        assert_eq!(Comparison::parse("=").unwrap(), Comparison::Equal);
        assert_eq!(Comparison::parse("<").unwrap(), Comparison::LessThan);
        assert_eq!(Comparison::parse("<=").unwrap(), Comparison::LessThanOrEqual);
        assert_eq!(Comparison::parse(">").unwrap(), Comparison::GreaterThan);
        assert_eq!(Comparison::parse(">=").unwrap(), Comparison::GreaterThanOrEqual);
        assert!(Comparison::parse("!=").is_err());
    }

    #[test]
    fn test_comparison_to_css() {
        assert_eq!(Comparison::Equal.to_css(), "=");
        assert_eq!(Comparison::LessThan.to_css(), "<");
        assert_eq!(Comparison::LessThanOrEqual.to_css(), "<=");
        assert_eq!(Comparison::GreaterThan.to_css(), ">");
        assert_eq!(Comparison::GreaterThanOrEqual.to_css(), ">=");
    }

    // Size Feature Tests

    #[test]
    fn test_size_feature_parse() {
        assert_eq!(SizeFeature::parse("width").unwrap(), SizeFeature::Width);
        assert_eq!(SizeFeature::parse("height").unwrap(), SizeFeature::Height);
        assert_eq!(SizeFeature::parse("inline-size").unwrap(), SizeFeature::InlineSize);
        assert_eq!(SizeFeature::parse("block-size").unwrap(), SizeFeature::BlockSize);
        assert_eq!(SizeFeature::parse("aspect-ratio").unwrap(), SizeFeature::AspectRatio);
        assert_eq!(SizeFeature::parse("orientation").unwrap(), SizeFeature::Orientation);
        assert!(SizeFeature::parse("invalid").is_err());
    }

    // Orientation Tests

    #[test]
    fn test_orientation_parse() {
        assert_eq!(Orientation::parse("portrait").unwrap(), Orientation::Portrait);
        assert_eq!(Orientation::parse("landscape").unwrap(), Orientation::Landscape);
        assert_eq!(Orientation::parse("PORTRAIT").unwrap(), Orientation::Portrait);
        assert!(Orientation::parse("invalid").is_err());
    }

    #[test]
    fn test_orientation_to_css() {
        assert_eq!(Orientation::Portrait.to_css(), "portrait");
        assert_eq!(Orientation::Landscape.to_css(), "landscape");
    }

    // Container Condition Tests

    #[test]
    fn test_container_condition_size() {
        let condition = ContainerCondition::size(SizeCondition::MinWidth(Length::new(400.0, LengthUnit::Px)));
        assert!(matches!(condition, ContainerCondition::Size(SizeCondition::MinWidth(_))));
    }

    #[test]
    fn test_container_condition_style() {
        let condition = ContainerCondition::style("--theme", "dark");
        if let ContainerCondition::Style(style) = condition {
            assert_eq!(style.property, "--theme");
            assert_eq!(style.value, "dark");
        } else {
            panic!("Expected Style condition");
        }
    }

    #[test]
    fn test_container_condition_not() {
        let inner = ContainerCondition::size(SizeCondition::MinWidth(Length::new(400.0, LengthUnit::Px)));
        let condition = ContainerCondition::not(inner);
        assert!(matches!(condition, ContainerCondition::Not(_)));
    }

    #[test]
    fn test_container_condition_and() {
        let conditions = vec![
            ContainerCondition::size(SizeCondition::MinWidth(Length::new(400.0, LengthUnit::Px))),
            ContainerCondition::size(SizeCondition::MaxWidth(Length::new(800.0, LengthUnit::Px))),
        ];
        let condition = ContainerCondition::and(conditions);
        if let ContainerCondition::And(conds) = condition {
            assert_eq!(conds.len(), 2);
        } else {
            panic!("Expected And condition");
        }
    }

    #[test]
    fn test_container_condition_or() {
        let conditions = vec![
            ContainerCondition::size(SizeCondition::MinWidth(Length::new(400.0, LengthUnit::Px))),
            ContainerCondition::style("--theme", "dark"),
        ];
        let condition = ContainerCondition::or(conditions);
        if let ContainerCondition::Or(conds) = condition {
            assert_eq!(conds.len(), 2);
        } else {
            panic!("Expected Or condition");
        }
    }

    #[test]
    fn test_style_condition_new() {
        let style = StyleCondition::new("--color", "blue");
        assert_eq!(style.property, "--color");
        assert_eq!(style.value, "blue");
    }
}
