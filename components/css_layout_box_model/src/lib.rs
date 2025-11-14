//! CSS Layout Box Model
//!
//! This crate provides box model computation for CSS elements including:
//! - Box model structures (content, padding, border, margin)
//! - Box sizing modes (content-box, border-box)
//! - Display property values
//! - Box model calculation trait and implementation

use css_stylist_core::ComputedValues;
use css_types::{Length, LengthUnit};

// ============================================================================
// Core Types
// ============================================================================

/// Rectangle dimensions
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Rect {
    /// Create a new rectangle
    ///
    /// # Arguments
    /// * `x` - X coordinate
    /// * `y` - Y coordinate
    /// * `width` - Width
    /// * `height` - Height
    ///
    /// # Examples
    /// ```
    /// use css_layout_box_model::Rect;
    ///
    /// let rect = Rect::new(10.0, 20.0, 100.0, 50.0);
    /// assert_eq!(rect.width(), 100.0);
    /// ```
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Get the x coordinate
    pub fn x(&self) -> f32 {
        self.x
    }

    /// Get the y coordinate
    pub fn y(&self) -> f32 {
        self.y
    }

    /// Get the width
    pub fn width(&self) -> f32 {
        self.width
    }

    /// Get the height
    pub fn height(&self) -> f32 {
        self.height
    }

    /// Calculate the area of the rectangle
    pub fn area(&self) -> f32 {
        self.width * self.height
    }

    /// Check if a point is contained within the rectangle
    ///
    /// # Arguments
    /// * `x` - X coordinate of the point
    /// * `y` - Y coordinate of the point
    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }
}

impl Default for Rect {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }
}

/// Sizes for all four edges (top, right, bottom, left)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EdgeSizes {
    top: f32,
    right: f32,
    bottom: f32,
    left: f32,
}

impl EdgeSizes {
    /// Create new edge sizes
    ///
    /// # Arguments
    /// * `top` - Top edge size
    /// * `right` - Right edge size
    /// * `bottom` - Bottom edge size
    /// * `left` - Left edge size
    ///
    /// # Examples
    /// ```
    /// use css_layout_box_model::EdgeSizes;
    ///
    /// let edges = EdgeSizes::new(10.0, 20.0, 30.0, 40.0);
    /// assert_eq!(edges.top(), 10.0);
    /// ```
    pub fn new(top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    /// Create uniform edge sizes (all edges the same)
    ///
    /// # Arguments
    /// * `size` - Size for all edges
    ///
    /// # Examples
    /// ```
    /// use css_layout_box_model::EdgeSizes;
    ///
    /// let edges = EdgeSizes::uniform(10.0);
    /// assert_eq!(edges.top(), 10.0);
    /// assert_eq!(edges.left(), 10.0);
    /// ```
    pub fn uniform(size: f32) -> Self {
        Self::new(size, size, size, size)
    }

    /// Get the top edge size
    pub fn top(&self) -> f32 {
        self.top
    }

    /// Get the right edge size
    pub fn right(&self) -> f32 {
        self.right
    }

    /// Get the bottom edge size
    pub fn bottom(&self) -> f32 {
        self.bottom
    }

    /// Get the left edge size
    pub fn left(&self) -> f32 {
        self.left
    }

    /// Get the total horizontal size (left + right)
    pub fn horizontal(&self) -> f32 {
        self.left + self.right
    }

    /// Get the total vertical size (top + bottom)
    pub fn vertical(&self) -> f32 {
        self.top + self.bottom
    }
}

impl Default for EdgeSizes {
    fn default() -> Self {
        Self::uniform(0.0)
    }
}

/// Box sizing model
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoxSizing {
    /// width/height = content only
    ContentBox,
    /// width/height = content + padding + border
    BorderBox,
}

/// Display property values
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Display {
    /// Block display
    Block,
    /// Inline display
    Inline,
    /// Inline block
    InlineBlock,
    /// None (hidden)
    None,
    /// Flex container
    Flex,
    /// Grid container
    Grid,
    /// Table
    Table,
}

/// Computed box model for an element
#[derive(Debug, Clone, PartialEq)]
pub struct BoxModel {
    /// Content box
    content: Rect,
    /// Padding sizes
    padding: EdgeSizes,
    /// Border widths
    border: EdgeSizes,
    /// Margin sizes
    margin: EdgeSizes,
    /// Box sizing mode
    box_sizing: BoxSizing,
}

impl BoxModel {
    /// Create a new box model
    ///
    /// # Arguments
    /// * `content` - Content box rectangle
    /// * `padding` - Padding edge sizes
    /// * `border` - Border edge sizes
    /// * `margin` - Margin edge sizes
    /// * `box_sizing` - Box sizing mode
    ///
    /// # Examples
    /// ```
    /// use css_layout_box_model::{BoxModel, BoxSizing, EdgeSizes, Rect};
    ///
    /// let content = Rect::new(0.0, 0.0, 200.0, 100.0);
    /// let padding = EdgeSizes::uniform(10.0);
    /// let border = EdgeSizes::uniform(2.0);
    /// let margin = EdgeSizes::uniform(5.0);
    ///
    /// let box_model = BoxModel::new(content, padding, border, margin, BoxSizing::ContentBox);
    /// assert_eq!(box_model.content().width(), 200.0);
    /// ```
    pub fn new(
        content: Rect,
        padding: EdgeSizes,
        border: EdgeSizes,
        margin: EdgeSizes,
        box_sizing: BoxSizing,
    ) -> Self {
        Self {
            content,
            padding,
            border,
            margin,
            box_sizing,
        }
    }

    /// Get the content box
    pub fn content(&self) -> &Rect {
        &self.content
    }

    /// Get the padding sizes
    pub fn padding(&self) -> &EdgeSizes {
        &self.padding
    }

    /// Get the border widths
    pub fn border(&self) -> &EdgeSizes {
        &self.border
    }

    /// Get the margin sizes
    pub fn margin(&self) -> &EdgeSizes {
        &self.margin
    }

    /// Get the box sizing mode
    pub fn box_sizing(&self) -> BoxSizing {
        self.box_sizing
    }

    /// Calculate the padding box (content + padding)
    ///
    /// # Examples
    /// ```
    /// use css_layout_box_model::{BoxModel, BoxSizing, EdgeSizes, Rect};
    ///
    /// let content = Rect::new(0.0, 0.0, 200.0, 100.0);
    /// let padding = EdgeSizes::uniform(10.0);
    /// let border = EdgeSizes::uniform(2.0);
    /// let margin = EdgeSizes::uniform(5.0);
    ///
    /// let box_model = BoxModel::new(content, padding, border, margin, BoxSizing::ContentBox);
    /// let padding_box = box_model.padding_box();
    ///
    /// assert_eq!(padding_box.width(), 220.0); // 200 + 10 + 10
    /// ```
    pub fn padding_box(&self) -> Rect {
        Rect::new(
            self.content.x - self.padding.left,
            self.content.y - self.padding.top,
            self.content.width + self.padding.horizontal(),
            self.content.height + self.padding.vertical(),
        )
    }

    /// Calculate the border box (content + padding + border)
    ///
    /// # Examples
    /// ```
    /// use css_layout_box_model::{BoxModel, BoxSizing, EdgeSizes, Rect};
    ///
    /// let content = Rect::new(0.0, 0.0, 200.0, 100.0);
    /// let padding = EdgeSizes::uniform(10.0);
    /// let border = EdgeSizes::uniform(2.0);
    /// let margin = EdgeSizes::uniform(5.0);
    ///
    /// let box_model = BoxModel::new(content, padding, border, margin, BoxSizing::ContentBox);
    /// let border_box = box_model.border_box();
    ///
    /// assert_eq!(border_box.width(), 224.0); // 200 + 10*2 + 2*2
    /// ```
    pub fn border_box(&self) -> Rect {
        let padding_box = self.padding_box();
        Rect::new(
            padding_box.x - self.border.left,
            padding_box.y - self.border.top,
            padding_box.width + self.border.horizontal(),
            padding_box.height + self.border.vertical(),
        )
    }

    /// Calculate the margin box (content + padding + border + margin)
    ///
    /// # Examples
    /// ```
    /// use css_layout_box_model::{BoxModel, BoxSizing, EdgeSizes, Rect};
    ///
    /// let content = Rect::new(0.0, 0.0, 200.0, 100.0);
    /// let padding = EdgeSizes::uniform(10.0);
    /// let border = EdgeSizes::uniform(2.0);
    /// let margin = EdgeSizes::uniform(5.0);
    ///
    /// let box_model = BoxModel::new(content, padding, border, margin, BoxSizing::ContentBox);
    /// let margin_box = box_model.margin_box();
    ///
    /// assert_eq!(margin_box.width(), 234.0); // 200 + 10*2 + 2*2 + 5*2
    /// ```
    pub fn margin_box(&self) -> Rect {
        let border_box = self.border_box();
        Rect::new(
            border_box.x - self.margin.left,
            border_box.y - self.margin.top,
            border_box.width + self.margin.horizontal(),
            border_box.height + self.margin.vertical(),
        )
    }
}

// ============================================================================
// Computation Functions
// ============================================================================

/// Compute padding for all edges
///
/// Resolves padding values from computed styles, handling percentage values
/// relative to the containing block width.
///
/// # Arguments
/// * `style` - Computed style values
/// * `containing_block_width` - Width of containing block in pixels
///
/// # Examples
/// ```
/// use css_layout_box_model::compute_padding;
/// use css_stylist_core::ComputedValues;
/// use css_types::{Length, LengthUnit};
///
/// let mut style = ComputedValues::default();
/// style.padding_top = Length::new(10.0, LengthUnit::Px);
/// style.padding_right = Length::new(5.0, LengthUnit::Percent);
///
/// let padding = compute_padding(&style, 800.0);
/// assert_eq!(padding.top(), 10.0);
/// assert_eq!(padding.right(), 40.0); // 5% of 800
/// ```
pub fn compute_padding(style: &ComputedValues, containing_block_width: f32) -> EdgeSizes {
    EdgeSizes::new(
        resolve_length(&style.padding_top, containing_block_width),
        resolve_length(&style.padding_right, containing_block_width),
        resolve_length(&style.padding_bottom, containing_block_width),
        resolve_length(&style.padding_left, containing_block_width),
    )
}

/// Compute border widths for all edges
///
/// # Arguments
/// * `style` - Computed style values
///
/// # Examples
/// ```
/// use css_layout_box_model::compute_border;
/// use css_stylist_core::ComputedValues;
///
/// let style = ComputedValues::default();
/// let border = compute_border(&style);
/// // Default border is 0
/// assert_eq!(border.top(), 0.0);
/// ```
pub fn compute_border(_style: &ComputedValues) -> EdgeSizes {
    // For now, return zero borders
    // In a full implementation, this would read border-width properties from style
    EdgeSizes::uniform(0.0)
}

/// Compute margins for all edges
///
/// Resolves margin values from computed styles, handling percentage values
/// relative to the containing block width.
///
/// # Arguments
/// * `style` - Computed style values
/// * `containing_block_width` - Width of containing block in pixels
///
/// # Examples
/// ```
/// use css_layout_box_model::compute_margin;
/// use css_stylist_core::ComputedValues;
/// use css_types::{Length, LengthUnit};
///
/// let mut style = ComputedValues::default();
/// style.margin_top = Length::new(10.0, LengthUnit::Px);
/// style.margin_left = Length::new(5.0, LengthUnit::Percent);
///
/// let margin = compute_margin(&style, 800.0);
/// assert_eq!(margin.top(), 10.0);
/// assert_eq!(margin.left(), 40.0); // 5% of 800
/// ```
pub fn compute_margin(style: &ComputedValues, containing_block_width: f32) -> EdgeSizes {
    EdgeSizes::new(
        resolve_length(&style.margin_top, containing_block_width),
        resolve_length(&style.margin_right, containing_block_width),
        resolve_length(&style.margin_bottom, containing_block_width),
        resolve_length(&style.margin_left, containing_block_width),
    )
}

/// Compute content box dimensions
///
/// # Arguments
/// * `style` - Computed style values
/// * `containing_block` - Containing block rectangle
///
/// # Examples
/// ```
/// use css_layout_box_model::{compute_content_box, Rect};
/// use css_stylist_core::ComputedValues;
/// use css_types::{Length, LengthUnit};
///
/// let mut style = ComputedValues::default();
/// style.width = Length::new(200.0, LengthUnit::Px);
/// style.height = Length::new(100.0, LengthUnit::Px);
///
/// let containing_block = Rect::new(0.0, 0.0, 800.0, 600.0);
/// let content = compute_content_box(&style, &containing_block);
///
/// assert_eq!(content.width(), 200.0);
/// assert_eq!(content.height(), 100.0);
/// ```
pub fn compute_content_box(style: &ComputedValues, containing_block: &Rect) -> Rect {
    let width = resolve_length(&style.width, containing_block.width);
    let height = resolve_length(&style.height, containing_block.height);

    Rect::new(containing_block.x, containing_block.y, width, height)
}

/// Resolve a length value to pixels
///
/// Handles different length units:
/// - Px: Direct pixel value
/// - Percent: Percentage of reference value
/// - Other units: Not yet supported, returns 0
///
/// # Arguments
/// * `length` - Length value to resolve
/// * `reference_value` - Reference value for percentage calculations
fn resolve_length(length: &Length, reference_value: f32) -> f32 {
    match length.unit() {
        LengthUnit::Px => length.value(),
        LengthUnit::Percent => (length.value() / 100.0) * reference_value,
        // Other units not yet supported
        _ => 0.0,
    }
}

// ============================================================================
// Box Model Calculator Trait
// ============================================================================

/// Trait for calculating box model from computed styles
///
/// Implementations can provide different strategies for box model computation,
/// such as handling different box-sizing modes or layout contexts.
pub trait BoxModelCalculator {
    /// Compute complete box model for element
    ///
    /// # Arguments
    /// * `style` - Computed style values
    /// * `containing_block` - Containing block rectangle
    ///
    /// # Returns
    /// Complete box model with content, padding, border, and margin
    fn compute_box_model(&self, style: &ComputedValues, containing_block: &Rect) -> BoxModel;

    /// Resolve width value to pixels
    ///
    /// # Arguments
    /// * `width` - Width length value
    /// * `containing_block_width` - Containing block width in pixels
    fn resolve_width(&self, width: &Length, containing_block_width: f32) -> f32;

    /// Resolve height value to pixels
    ///
    /// # Arguments
    /// * `height` - Height length value
    /// * `containing_block_height` - Containing block height in pixels
    fn resolve_height(&self, height: &Length, containing_block_height: f32) -> f32;
}

/// Default box model calculator implementation
///
/// Provides standard CSS box model computation following CSS2.1 specification.
///
/// # Examples
/// ```
/// use css_layout_box_model::{BoxModelCalculator, DefaultBoxModelCalculator, Rect};
/// use css_stylist_core::ComputedValues;
/// use css_types::{Length, LengthUnit};
///
/// let calculator = DefaultBoxModelCalculator;
/// let mut style = ComputedValues::default();
/// style.width = Length::new(200.0, LengthUnit::Px);
/// style.height = Length::new(100.0, LengthUnit::Px);
///
/// let containing_block = Rect::new(0.0, 0.0, 800.0, 600.0);
/// let box_model = calculator.compute_box_model(&style, &containing_block);
///
/// assert_eq!(box_model.content().width(), 200.0);
/// ```
pub struct DefaultBoxModelCalculator;

impl BoxModelCalculator for DefaultBoxModelCalculator {
    fn compute_box_model(&self, style: &ComputedValues, containing_block: &Rect) -> BoxModel {
        let content = compute_content_box(style, containing_block);
        let padding = compute_padding(style, containing_block.width);
        let border = compute_border(style);
        let margin = compute_margin(style, containing_block.width);

        BoxModel::new(content, padding, border, margin, BoxSizing::ContentBox)
    }

    fn resolve_width(&self, width: &Length, containing_block_width: f32) -> f32 {
        resolve_length(width, containing_block_width)
    }

    fn resolve_height(&self, height: &Length, containing_block_height: f32) -> f32 {
        resolve_length(height, containing_block_height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect_basic() {
        let rect = Rect::new(10.0, 20.0, 100.0, 50.0);
        assert_eq!(rect.x(), 10.0);
        assert_eq!(rect.y(), 20.0);
        assert_eq!(rect.width(), 100.0);
        assert_eq!(rect.height(), 50.0);
    }

    #[test]
    fn test_edge_sizes_basic() {
        let edges = EdgeSizes::new(10.0, 20.0, 30.0, 40.0);
        assert_eq!(edges.top(), 10.0);
        assert_eq!(edges.horizontal(), 60.0);
        assert_eq!(edges.vertical(), 40.0);
    }

    #[test]
    fn test_box_model_basic() {
        let content = Rect::new(0.0, 0.0, 200.0, 100.0);
        let padding = EdgeSizes::uniform(10.0);
        let border = EdgeSizes::uniform(2.0);
        let margin = EdgeSizes::uniform(5.0);

        let box_model = BoxModel::new(content, padding, border, margin, BoxSizing::ContentBox);

        assert_eq!(box_model.content().width(), 200.0);
        assert_eq!(box_model.padding_box().width(), 220.0);
        assert_eq!(box_model.border_box().width(), 224.0);
        assert_eq!(box_model.margin_box().width(), 234.0);
    }
}
