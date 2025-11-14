//! CSS Multi-Column Layout - Multi-column layout computation
//!
//! This module provides CSS multi-column layout computation including:
//! - Column count parsing (auto, integer)
//! - Column width parsing (auto, length)
//! - Column gap parsing (normal, length)
//! - Column rule parsing (width, style, color)
//! - Column layout computation (determine actual columns)
//! - Content balancing across columns

use css_parser_core::ParseError;
use css_types::{Color, CssValue, Length};

// ============================================================================
// Border Style Type
// ============================================================================

/// Border style for column rules
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorderStyle {
    /// No border
    None,
    /// Solid line
    Solid,
    /// Dashed line
    Dashed,
    /// Dotted line
    Dotted,
    /// Double line
    Double,
    /// 3D groove
    Groove,
    /// 3D ridge
    Ridge,
    /// 3D inset
    Inset,
    /// 3D outset
    Outset,
}

impl BorderStyle {
    /// Parse a border style from a string
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        match input.trim().to_lowercase().as_str() {
            "none" => Ok(BorderStyle::None),
            "solid" => Ok(BorderStyle::Solid),
            "dashed" => Ok(BorderStyle::Dashed),
            "dotted" => Ok(BorderStyle::Dotted),
            "double" => Ok(BorderStyle::Double),
            "groove" => Ok(BorderStyle::Groove),
            "ridge" => Ok(BorderStyle::Ridge),
            "inset" => Ok(BorderStyle::Inset),
            "outset" => Ok(BorderStyle::Outset),
            _ => Err(ParseError::new(
                0,
                0,
                format!("Invalid border style: {}", input),
            )),
        }
    }
}

// ============================================================================
// Column Count Type
// ============================================================================

/// Number of columns (auto or specific count)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColumnCount {
    /// Automatically determine column count
    Auto,
    /// Specific number of columns
    Count(u32),
}

// ============================================================================
// Column Width Type
// ============================================================================

/// Column width specification (auto or specific length)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColumnWidth {
    /// Automatically determine column width
    Auto,
    /// Specific column width
    Length(Length),
}

// ============================================================================
// Column Gap Type
// ============================================================================

/// Gap between columns (normal or specific length)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColumnGap {
    /// Normal gap (typically 1em)
    Normal,
    /// Specific gap size
    Length(Length),
}

// ============================================================================
// Column Rule Type
// ============================================================================

/// Rule (border) between columns
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColumnRule {
    /// Width of the rule
    pub width: Length,
    /// Style of the rule
    pub style: BorderStyle,
    /// Color of the rule
    pub color: Color,
}

impl ColumnRule {
    /// Create a new column rule
    pub fn new(width: Length, style: BorderStyle, color: Color) -> Self {
        Self {
            width,
            style,
            color,
        }
    }
}

// ============================================================================
// Column Span Type
// ============================================================================

/// Column spanning behavior
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColumnSpan {
    /// Don't span columns
    None,
    /// Span all columns
    All,
}

// ============================================================================
// Column Fill Type
// ============================================================================

/// How to balance content across columns
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColumnFill {
    /// Balance content evenly across columns
    Balance,
    /// Fill columns sequentially (auto)
    Auto,
}

// ============================================================================
// Multi-Column Layout Type
// ============================================================================

/// Complete multi-column layout configuration
#[derive(Debug, Clone, PartialEq)]
pub struct MultiColumnLayout {
    /// Number of columns
    pub column_count: ColumnCount,
    /// Width of each column
    pub column_width: ColumnWidth,
    /// Gap between columns
    pub column_gap: ColumnGap,
    /// Rule between columns
    pub column_rule: Option<ColumnRule>,
    /// Column spanning behavior
    pub column_span: ColumnSpan,
    /// Content fill behavior
    pub column_fill: ColumnFill,
}

impl MultiColumnLayout {
    /// Create a new multi-column layout with defaults
    pub fn new() -> Self {
        Self {
            column_count: ColumnCount::Auto,
            column_width: ColumnWidth::Auto,
            column_gap: ColumnGap::Normal,
            column_rule: None,
            column_span: ColumnSpan::None,
            column_fill: ColumnFill::Balance,
        }
    }
}

impl Default for MultiColumnLayout {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Computed Columns Type
// ============================================================================

/// Computed column layout details
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ComputedColumns {
    /// Actual number of columns
    pub column_count: u32,
    /// Actual width of each column
    pub column_width: f32,
    /// Actual gap width
    pub gap_width: f32,
    /// Total width used
    pub total_width: f32,
}

impl ComputedColumns {
    /// Create a new computed columns result
    pub fn new(column_count: u32, column_width: f32, gap_width: f32, total_width: f32) -> Self {
        Self {
            column_count,
            column_width,
            gap_width,
            total_width,
        }
    }
}

// ============================================================================
// Parsing Functions
// ============================================================================

/// Parse column-count property
///
/// # Examples
/// ```
/// use css_layout_multicolumn::{parse_column_count, ColumnCount};
///
/// let count = parse_column_count("auto").unwrap();
/// assert_eq!(count, ColumnCount::Auto);
///
/// let count = parse_column_count("3").unwrap();
/// assert_eq!(count, ColumnCount::Count(3));
/// ```
pub fn parse_column_count(input: &str) -> Result<ColumnCount, ParseError> {
    let input = input.trim();

    if input == "auto" {
        return Ok(ColumnCount::Auto);
    }

    match input.parse::<u32>() {
        Ok(count) if count > 0 => Ok(ColumnCount::Count(count)),
        Ok(_) => Err(ParseError::new(0, 0, "Column count must be greater than 0")),
        Err(_) => Err(ParseError::new(
            0,
            0,
            format!("Invalid column count: {}", input),
        )),
    }
}

/// Parse column-width property
///
/// # Examples
/// ```
/// use css_layout_multicolumn::{parse_column_width, ColumnWidth};
///
/// let width = parse_column_width("auto").unwrap();
/// assert_eq!(width, ColumnWidth::Auto);
///
/// let width = parse_column_width("200px").unwrap();
/// assert!(matches!(width, ColumnWidth::Length(_)));
/// ```
pub fn parse_column_width(input: &str) -> Result<ColumnWidth, ParseError> {
    let input = input.trim();

    if input == "auto" {
        return Ok(ColumnWidth::Auto);
    }

    match Length::parse(input) {
        Ok(length) => Ok(ColumnWidth::Length(length)),
        Err(e) => Err(ParseError::new(
            0,
            0,
            format!("Invalid column width: {}", e),
        )),
    }
}

/// Parse column-gap property
///
/// # Examples
/// ```
/// use css_layout_multicolumn::{parse_column_gap, ColumnGap};
///
/// let gap = parse_column_gap("normal").unwrap();
/// assert_eq!(gap, ColumnGap::Normal);
///
/// let gap = parse_column_gap("1em").unwrap();
/// assert!(matches!(gap, ColumnGap::Length(_)));
/// ```
pub fn parse_column_gap(input: &str) -> Result<ColumnGap, ParseError> {
    let input = input.trim();

    if input == "normal" {
        return Ok(ColumnGap::Normal);
    }

    match Length::parse(input) {
        Ok(length) => Ok(ColumnGap::Length(length)),
        Err(e) => Err(ParseError::new(0, 0, format!("Invalid column gap: {}", e))),
    }
}

/// Parse column-rule shorthand property
///
/// # Examples
/// ```
/// use css_layout_multicolumn::parse_column_rule;
///
/// let rule = parse_column_rule("1px solid #000000").unwrap();
/// assert_eq!(rule.style, css_layout_multicolumn::BorderStyle::Solid);
/// ```
pub fn parse_column_rule(input: &str) -> Result<ColumnRule, ParseError> {
    let input = input.trim();
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.len() < 3 {
        return Err(ParseError::new(
            0,
            0,
            "Column rule requires width, style, and color",
        ));
    }

    // Parse width
    let width = Length::parse(parts[0])
        .map_err(|e| ParseError::new(0, 0, format!("Invalid rule width: {}", e)))?;

    // Parse style
    let style = BorderStyle::parse(parts[1])?;

    // Parse color (everything after style)
    let color_str = parts[2..].join(" ");
    let color = Color::parse(&color_str)
        .map_err(|e| ParseError::new(0, 0, format!("Invalid rule color: {}", e)))?;

    Ok(ColumnRule::new(width, style, color))
}

// ============================================================================
// Layout Computation Functions
// ============================================================================

/// Compute actual column count and widths based on configuration
///
/// This function resolves auto values and calculates the actual column layout
/// that will fit within the available width.
///
/// # Algorithm
/// - If count is specified and width is auto: divide available width by count
/// - If width is specified and count is auto: fit as many columns as possible
/// - If both are auto: use a default count (typically 1)
/// - If both are specified: use specified values (may overflow container)
///
/// # Examples
/// ```
/// use css_layout_multicolumn::{compute_column_layout, MultiColumnLayout, ColumnCount};
///
/// let mut config = MultiColumnLayout::new();
/// config.column_count = ColumnCount::Count(3);
///
/// let computed = compute_column_layout(&config, 600.0);
/// assert_eq!(computed.column_count, 3);
/// ```
pub fn compute_column_layout(config: &MultiColumnLayout, available_width: f32) -> ComputedColumns {
    // Determine gap width (default to 1em = 16px for normal)
    let gap_width = match config.column_gap {
        ColumnGap::Normal => 16.0,
        ColumnGap::Length(length) => length.value(),
    };

    match (config.column_count, config.column_width) {
        // Both auto: default to 1 column
        (ColumnCount::Auto, ColumnWidth::Auto) => {
            ComputedColumns::new(1, available_width, gap_width, available_width)
        }

        // Count specified, width auto: divide available width
        (ColumnCount::Count(count), ColumnWidth::Auto) => {
            let total_gap_width = gap_width * (count - 1) as f32;
            let column_width = (available_width - total_gap_width) / count as f32;
            ComputedColumns::new(count, column_width, gap_width, available_width)
        }

        // Width specified, count auto: fit as many columns as possible
        (ColumnCount::Auto, ColumnWidth::Length(width)) => {
            let col_width = width.value();
            let mut count = 1;
            let mut total = col_width;

            // Add columns while they fit
            while total + gap_width + col_width <= available_width {
                count += 1;
                total += gap_width + col_width;
            }

            ComputedColumns::new(count, col_width, gap_width, total)
        }

        // Both specified: use specified values
        (ColumnCount::Count(count), ColumnWidth::Length(width)) => {
            let col_width = width.value();
            let total_gap_width = gap_width * (count - 1) as f32;
            let total = col_width * count as f32 + total_gap_width;
            ComputedColumns::new(count, col_width, gap_width, total)
        }
    }
}

/// Balance content height across columns
///
/// Divides the total content height evenly across the specified number of columns.
///
/// # Examples
/// ```
/// use css_layout_multicolumn::balance_content;
///
/// let height_per_column = balance_content(1000.0, 4);
/// assert_eq!(height_per_column, 250.0);
/// ```
pub fn balance_content(content_height: f32, column_count: u32) -> f32 {
    if column_count == 0 {
        return content_height;
    }
    content_height / column_count as f32
}

// ============================================================================
// Trait Implementation
// ============================================================================

/// Multi-column layout computation interface
pub trait MultiColumnComputer {
    /// Compute the complete column layout including content balancing
    fn compute_layout(
        &self,
        config: &MultiColumnLayout,
        available_width: f32,
        content_height: f32,
    ) -> ComputedColumns;
}

/// Default implementation of MultiColumnComputer
pub struct DefaultMultiColumnComputer;

impl MultiColumnComputer for DefaultMultiColumnComputer {
    fn compute_layout(
        &self,
        config: &MultiColumnLayout,
        available_width: f32,
        _content_height: f32,
    ) -> ComputedColumns {
        compute_column_layout(config, available_width)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_types() {
        let count = ColumnCount::Auto;
        assert_eq!(count, ColumnCount::Auto);

        let width = ColumnWidth::Auto;
        assert_eq!(width, ColumnWidth::Auto);

        let gap = ColumnGap::Normal;
        assert_eq!(gap, ColumnGap::Normal);
    }

    #[test]
    fn test_multicolumn_layout_default() {
        let layout = MultiColumnLayout::new();
        assert_eq!(layout.column_count, ColumnCount::Auto);
        assert_eq!(layout.column_width, ColumnWidth::Auto);
        assert_eq!(layout.column_gap, ColumnGap::Normal);
        assert!(layout.column_rule.is_none());
        assert_eq!(layout.column_span, ColumnSpan::None);
        assert_eq!(layout.column_fill, ColumnFill::Balance);
    }
}
