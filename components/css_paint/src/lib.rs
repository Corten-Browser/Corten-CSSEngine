//! CSS Paint API (Houdini) Implementation
//!
//! This module provides structures for custom CSS Paint worklets, enabling
//! programmatic drawing for CSS backgrounds, borders, and other properties.
//!
//! # Overview
//!
//! The CSS Paint API allows developers to define custom paint worklets that
//! can be used in CSS via the `paint()` function. This implementation provides:
//!
//! - [`PaintWorklet`] trait for defining custom paint worklets
//! - [`PaintContext`] for 2D drawing operations
//! - [`PaintSize`] for geometry information
//! - [`PaintValue`] for worklet arguments
//! - [`PaintWorkletRegistry`] for worklet registration
//!
//! # Example
//!
//! ```rust
//! use css_paint::{PaintWorklet, PaintContext, PaintSize, PaintValue, PaintArgumentType};
//!
//! struct CheckerboardWorklet;
//!
//! impl PaintWorklet for CheckerboardWorklet {
//!     fn name(&self) -> &str {
//!         "checkerboard"
//!     }
//!
//!     fn input_properties(&self) -> Vec<String> {
//!         vec!["--checkerboard-size".to_string()]
//!     }
//!
//!     fn input_arguments(&self) -> Vec<PaintArgumentType> {
//!         vec![PaintArgumentType::Number]
//!     }
//!
//!     fn paint(&self, ctx: &mut PaintContext, size: PaintSize, args: &[PaintValue]) {
//!         // Draw checkerboard pattern
//!     }
//! }
//! ```
//!
//! # Note
//!
//! This is a framework/structure implementation. Actual JavaScript worklet
//! execution is out of scope.

pub mod parser;
pub mod registry;

use css_types::Color;
use std::fmt;

// ============================================================================
// Error Types
// ============================================================================

/// Errors that can occur in the Paint API
#[derive(Debug, Clone, PartialEq)]
pub enum PaintError {
    /// Worklet not found in registry
    WorkletNotFound(String),
    /// Invalid argument type
    InvalidArgumentType {
        expected: PaintArgumentType,
        got: String,
    },
    /// Wrong number of arguments
    ArgumentCountMismatch { expected: usize, got: usize },
    /// Parse error for paint() function
    ParseError(String),
    /// Worklet already registered
    WorkletAlreadyRegistered(String),
}

impl fmt::Display for PaintError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PaintError::WorkletNotFound(name) => {
                write!(f, "Paint worklet '{}' not found", name)
            }
            PaintError::InvalidArgumentType { expected, got } => {
                write!(
                    f,
                    "Invalid argument type: expected {:?}, got '{}'",
                    expected, got
                )
            }
            PaintError::ArgumentCountMismatch { expected, got } => {
                write!(
                    f,
                    "Wrong number of arguments: expected {}, got {}",
                    expected, got
                )
            }
            PaintError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            PaintError::WorkletAlreadyRegistered(name) => {
                write!(f, "Paint worklet '{}' is already registered", name)
            }
        }
    }
}

impl std::error::Error for PaintError {}

// ============================================================================
// Paint Value Types
// ============================================================================

/// Types of arguments that can be passed to paint worklets
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaintArgumentType {
    /// Numeric value (e.g., 10, 3.14)
    Number,
    /// Length value with unit (e.g., 10px, 2em)
    Length,
    /// Color value (e.g., #ff0000, rgb(255, 0, 0))
    Color,
    /// Image reference
    Image,
    /// Percentage value (e.g., 50%)
    Percentage,
    /// Angle value (e.g., 45deg, 0.5turn)
    Angle,
    /// String/identifier value
    Ident,
}

/// A value passed to a paint worklet
#[derive(Debug, Clone, PartialEq)]
pub enum PaintValue {
    /// Numeric value
    Number(f32),
    /// Length value in pixels (resolved)
    Length(f32),
    /// Color value
    Color(Color),
    /// Image reference (as URL string)
    Image(String),
    /// Percentage (0.0 - 1.0 normalized)
    Percentage(f32),
    /// Angle in radians
    Angle(f32),
    /// Identifier/string value
    Ident(String),
}

impl PaintValue {
    /// Get the argument type for this value
    pub fn argument_type(&self) -> PaintArgumentType {
        match self {
            PaintValue::Number(_) => PaintArgumentType::Number,
            PaintValue::Length(_) => PaintArgumentType::Length,
            PaintValue::Color(_) => PaintArgumentType::Color,
            PaintValue::Image(_) => PaintArgumentType::Image,
            PaintValue::Percentage(_) => PaintArgumentType::Percentage,
            PaintValue::Angle(_) => PaintArgumentType::Angle,
            PaintValue::Ident(_) => PaintArgumentType::Ident,
        }
    }

    /// Try to get a number value
    pub fn as_number(&self) -> Option<f32> {
        match self {
            PaintValue::Number(n) => Some(*n),
            _ => None,
        }
    }

    /// Try to get a length value (in pixels)
    pub fn as_length(&self) -> Option<f32> {
        match self {
            PaintValue::Length(l) => Some(*l),
            _ => None,
        }
    }

    /// Try to get a color value
    pub fn as_color(&self) -> Option<&Color> {
        match self {
            PaintValue::Color(c) => Some(c),
            _ => None,
        }
    }

    /// Try to get a percentage value
    pub fn as_percentage(&self) -> Option<f32> {
        match self {
            PaintValue::Percentage(p) => Some(*p),
            _ => None,
        }
    }

    /// Try to get an angle value (in radians)
    pub fn as_angle(&self) -> Option<f32> {
        match self {
            PaintValue::Angle(a) => Some(*a),
            _ => None,
        }
    }

    /// Try to get an identifier value
    pub fn as_ident(&self) -> Option<&str> {
        match self {
            PaintValue::Ident(s) => Some(s),
            _ => None,
        }
    }
}

// ============================================================================
// Paint Size
// ============================================================================

/// Size information passed to paint worklets
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PaintSize {
    /// Width in pixels
    pub width: f32,
    /// Height in pixels
    pub height: f32,
}

impl PaintSize {
    /// Create a new paint size
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    /// Get the aspect ratio (width / height)
    pub fn aspect_ratio(&self) -> f32 {
        if self.height != 0.0 {
            self.width / self.height
        } else {
            1.0
        }
    }

    /// Get the area
    pub fn area(&self) -> f32 {
        self.width * self.height
    }

    /// Check if the size is empty (zero area)
    pub fn is_empty(&self) -> bool {
        self.width <= 0.0 || self.height <= 0.0
    }
}

impl Default for PaintSize {
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}

// ============================================================================
// Paint Operations
// ============================================================================

/// 2D drawing operations (similar to Canvas API)
#[derive(Debug, Clone, PartialEq)]
pub enum PaintOperation {
    /// Fill a rectangle: x, y, width, height
    FillRect(f32, f32, f32, f32),
    /// Stroke a rectangle: x, y, width, height
    StrokeRect(f32, f32, f32, f32),
    /// Clear a rectangle: x, y, width, height
    ClearRect(f32, f32, f32, f32),
    /// Begin a new path
    BeginPath,
    /// Move to position: x, y
    MoveTo(f32, f32),
    /// Draw line to position: x, y
    LineTo(f32, f32),
    /// Draw arc: x, y, radius, start_angle, end_angle (angles in radians)
    Arc(f32, f32, f32, f32, f32),
    /// Draw arc with counterclockwise flag
    ArcWithDirection(f32, f32, f32, f32, f32, bool),
    /// Draw quadratic Bezier curve: cpx, cpy, x, y
    QuadraticCurveTo(f32, f32, f32, f32),
    /// Draw cubic Bezier curve: cp1x, cp1y, cp2x, cp2y, x, y
    BezierCurveTo(f32, f32, f32, f32, f32, f32),
    /// Draw ellipse: x, y, radius_x, radius_y, rotation, start_angle, end_angle
    Ellipse(f32, f32, f32, f32, f32, f32, f32),
    /// Draw rectangle path: x, y, width, height
    Rect(f32, f32, f32, f32),
    /// Close the current path
    ClosePath,
    /// Fill the current path
    Fill,
    /// Stroke the current path
    Stroke,
    /// Clip to current path
    Clip,
    /// Set fill color
    SetFillColor(Color),
    /// Set stroke color
    SetStrokeColor(Color),
    /// Set line width
    SetLineWidth(f32),
    /// Set line cap style
    SetLineCap(LineCap),
    /// Set line join style
    SetLineJoin(LineJoin),
    /// Set miter limit
    SetMiterLimit(f32),
    /// Set line dash pattern
    SetLineDash(Vec<f32>),
    /// Set line dash offset
    SetLineDashOffset(f32),
    /// Set global alpha
    SetGlobalAlpha(f32),
    /// Save the current state
    Save,
    /// Restore the previous state
    Restore,
    /// Translate the origin: x, y
    Translate(f32, f32),
    /// Rotate around the origin (angle in radians)
    Rotate(f32),
    /// Scale: x, y
    Scale(f32, f32),
    /// Apply transform matrix: a, b, c, d, e, f
    Transform(f32, f32, f32, f32, f32, f32),
    /// Set transform matrix: a, b, c, d, e, f
    SetTransform(f32, f32, f32, f32, f32, f32),
    /// Reset transform to identity
    ResetTransform,
}

/// Line cap styles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LineCap {
    /// Flat edge at the endpoint
    #[default]
    Butt,
    /// Rounded end extending beyond endpoint
    Round,
    /// Square end extending beyond endpoint
    Square,
}

/// Line join styles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LineJoin {
    /// Sharp corner
    #[default]
    Miter,
    /// Rounded corner
    Round,
    /// Beveled corner
    Bevel,
}

// ============================================================================
// Paint Context
// ============================================================================

/// Context for paint worklet rendering operations
///
/// This provides a 2D drawing API similar to the HTML Canvas API.
/// Operations are recorded and can be replayed by a renderer.
#[derive(Debug, Clone, Default)]
pub struct PaintContext {
    /// Recorded drawing operations
    operations: Vec<PaintOperation>,
    /// Current fill color
    fill_color: Option<Color>,
    /// Current stroke color
    stroke_color: Option<Color>,
    /// Current line width
    line_width: f32,
    /// Current global alpha
    global_alpha: f32,
}

impl PaintContext {
    /// Create a new paint context
    pub fn new() -> Self {
        Self {
            operations: Vec::new(),
            fill_color: Some(Color::rgb(0, 0, 0)),
            stroke_color: Some(Color::rgb(0, 0, 0)),
            line_width: 1.0,
            global_alpha: 1.0,
        }
    }

    /// Get the recorded operations
    pub fn operations(&self) -> &[PaintOperation] {
        &self.operations
    }

    /// Take ownership of the recorded operations
    pub fn take_operations(self) -> Vec<PaintOperation> {
        self.operations
    }

    /// Clear all recorded operations
    pub fn clear(&mut self) {
        self.operations.clear();
    }

    // ========================================================================
    // Rectangle operations
    // ========================================================================

    /// Fill a rectangle
    pub fn fill_rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.operations
            .push(PaintOperation::FillRect(x, y, width, height));
    }

    /// Stroke a rectangle
    pub fn stroke_rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.operations
            .push(PaintOperation::StrokeRect(x, y, width, height));
    }

    /// Clear a rectangle
    pub fn clear_rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.operations
            .push(PaintOperation::ClearRect(x, y, width, height));
    }

    // ========================================================================
    // Path operations
    // ========================================================================

    /// Begin a new path
    pub fn begin_path(&mut self) {
        self.operations.push(PaintOperation::BeginPath);
    }

    /// Move to a position
    pub fn move_to(&mut self, x: f32, y: f32) {
        self.operations.push(PaintOperation::MoveTo(x, y));
    }

    /// Draw a line to a position
    pub fn line_to(&mut self, x: f32, y: f32) {
        self.operations.push(PaintOperation::LineTo(x, y));
    }

    /// Draw an arc
    pub fn arc(&mut self, x: f32, y: f32, radius: f32, start_angle: f32, end_angle: f32) {
        self.operations
            .push(PaintOperation::Arc(x, y, radius, start_angle, end_angle));
    }

    /// Draw an arc with counterclockwise flag
    pub fn arc_with_direction(
        &mut self,
        x: f32,
        y: f32,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
        counterclockwise: bool,
    ) {
        self.operations.push(PaintOperation::ArcWithDirection(
            x,
            y,
            radius,
            start_angle,
            end_angle,
            counterclockwise,
        ));
    }

    /// Draw a quadratic Bezier curve
    pub fn quadratic_curve_to(&mut self, cpx: f32, cpy: f32, x: f32, y: f32) {
        self.operations
            .push(PaintOperation::QuadraticCurveTo(cpx, cpy, x, y));
    }

    /// Draw a cubic Bezier curve
    pub fn bezier_curve_to(&mut self, cp1x: f32, cp1y: f32, cp2x: f32, cp2y: f32, x: f32, y: f32) {
        self.operations
            .push(PaintOperation::BezierCurveTo(cp1x, cp1y, cp2x, cp2y, x, y));
    }

    /// Draw an ellipse
    #[allow(clippy::too_many_arguments)]
    pub fn ellipse(
        &mut self,
        x: f32,
        y: f32,
        radius_x: f32,
        radius_y: f32,
        rotation: f32,
        start_angle: f32,
        end_angle: f32,
    ) {
        self.operations.push(PaintOperation::Ellipse(
            x,
            y,
            radius_x,
            radius_y,
            rotation,
            start_angle,
            end_angle,
        ));
    }

    /// Draw a rectangle path
    pub fn rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.operations
            .push(PaintOperation::Rect(x, y, width, height));
    }

    /// Close the current path
    pub fn close_path(&mut self) {
        self.operations.push(PaintOperation::ClosePath);
    }

    /// Fill the current path
    pub fn fill(&mut self) {
        self.operations.push(PaintOperation::Fill);
    }

    /// Stroke the current path
    pub fn stroke(&mut self) {
        self.operations.push(PaintOperation::Stroke);
    }

    /// Clip to current path
    pub fn clip(&mut self) {
        self.operations.push(PaintOperation::Clip);
    }

    // ========================================================================
    // Style operations
    // ========================================================================

    /// Set the fill color
    pub fn set_fill_color(&mut self, color: Color) {
        self.fill_color = Some(color);
        self.operations.push(PaintOperation::SetFillColor(color));
    }

    /// Set the stroke color
    pub fn set_stroke_color(&mut self, color: Color) {
        self.stroke_color = Some(color);
        self.operations.push(PaintOperation::SetStrokeColor(color));
    }

    /// Set the line width
    pub fn set_line_width(&mut self, width: f32) {
        self.line_width = width;
        self.operations.push(PaintOperation::SetLineWidth(width));
    }

    /// Set the line cap style
    pub fn set_line_cap(&mut self, cap: LineCap) {
        self.operations.push(PaintOperation::SetLineCap(cap));
    }

    /// Set the line join style
    pub fn set_line_join(&mut self, join: LineJoin) {
        self.operations.push(PaintOperation::SetLineJoin(join));
    }

    /// Set the miter limit
    pub fn set_miter_limit(&mut self, limit: f32) {
        self.operations.push(PaintOperation::SetMiterLimit(limit));
    }

    /// Set the line dash pattern
    pub fn set_line_dash(&mut self, segments: Vec<f32>) {
        self.operations.push(PaintOperation::SetLineDash(segments));
    }

    /// Set the line dash offset
    pub fn set_line_dash_offset(&mut self, offset: f32) {
        self.operations
            .push(PaintOperation::SetLineDashOffset(offset));
    }

    /// Set the global alpha
    pub fn set_global_alpha(&mut self, alpha: f32) {
        self.global_alpha = alpha.clamp(0.0, 1.0);
        self.operations
            .push(PaintOperation::SetGlobalAlpha(self.global_alpha));
    }

    /// Get the current fill color
    pub fn fill_color(&self) -> Option<&Color> {
        self.fill_color.as_ref()
    }

    /// Get the current stroke color
    pub fn stroke_color(&self) -> Option<&Color> {
        self.stroke_color.as_ref()
    }

    /// Get the current line width
    pub fn line_width(&self) -> f32 {
        self.line_width
    }

    /// Get the current global alpha
    pub fn global_alpha(&self) -> f32 {
        self.global_alpha
    }

    // ========================================================================
    // State operations
    // ========================================================================

    /// Save the current state
    pub fn save(&mut self) {
        self.operations.push(PaintOperation::Save);
    }

    /// Restore the previous state
    pub fn restore(&mut self) {
        self.operations.push(PaintOperation::Restore);
    }

    // ========================================================================
    // Transform operations
    // ========================================================================

    /// Translate the origin
    pub fn translate(&mut self, x: f32, y: f32) {
        self.operations.push(PaintOperation::Translate(x, y));
    }

    /// Rotate around the origin
    pub fn rotate(&mut self, angle: f32) {
        self.operations.push(PaintOperation::Rotate(angle));
    }

    /// Scale
    pub fn scale(&mut self, x: f32, y: f32) {
        self.operations.push(PaintOperation::Scale(x, y));
    }

    /// Apply a transform matrix
    #[allow(clippy::too_many_arguments)]
    pub fn transform(&mut self, a: f32, b: f32, c: f32, d: f32, e: f32, f: f32) {
        self.operations
            .push(PaintOperation::Transform(a, b, c, d, e, f));
    }

    /// Set the transform matrix
    #[allow(clippy::too_many_arguments)]
    pub fn set_transform(&mut self, a: f32, b: f32, c: f32, d: f32, e: f32, f: f32) {
        self.operations
            .push(PaintOperation::SetTransform(a, b, c, d, e, f));
    }

    /// Reset the transform to identity
    pub fn reset_transform(&mut self) {
        self.operations.push(PaintOperation::ResetTransform);
    }

    // ========================================================================
    // Convenience methods
    // ========================================================================

    /// Draw a circle
    pub fn circle(&mut self, x: f32, y: f32, radius: f32) {
        self.begin_path();
        self.arc(x, y, radius, 0.0, std::f32::consts::TAU);
        self.close_path();
    }

    /// Draw a filled circle
    pub fn fill_circle(&mut self, x: f32, y: f32, radius: f32) {
        self.circle(x, y, radius);
        self.fill();
    }

    /// Draw a stroked circle
    pub fn stroke_circle(&mut self, x: f32, y: f32, radius: f32) {
        self.circle(x, y, radius);
        self.stroke();
    }

    /// Draw a line
    pub fn line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        self.begin_path();
        self.move_to(x1, y1);
        self.line_to(x2, y2);
        self.stroke();
    }
}

// ============================================================================
// Paint Worklet Trait
// ============================================================================

/// Trait representing a CSS Paint worklet
///
/// Implement this trait to create custom paint worklets that can be
/// used with the CSS `paint()` function.
///
/// # Example
///
/// ```rust
/// use css_paint::{PaintWorklet, PaintContext, PaintSize, PaintValue, PaintArgumentType};
/// use css_types::Color;
///
/// struct GradientWorklet;
///
/// impl PaintWorklet for GradientWorklet {
///     fn name(&self) -> &str {
///         "custom-gradient"
///     }
///
///     fn input_properties(&self) -> Vec<String> {
///         vec!["--gradient-start".to_string(), "--gradient-end".to_string()]
///     }
///
///     fn input_arguments(&self) -> Vec<PaintArgumentType> {
///         vec![] // No additional arguments
///     }
///
///     fn paint(&self, ctx: &mut PaintContext, size: PaintSize, _args: &[PaintValue]) {
///         // Draw gradient from top to bottom
///         ctx.set_fill_color(Color::rgb(255, 0, 0));
///         ctx.fill_rect(0.0, 0.0, size.width, size.height / 2.0);
///         ctx.set_fill_color(Color::rgb(0, 0, 255));
///         ctx.fill_rect(0.0, size.height / 2.0, size.width, size.height / 2.0);
///     }
/// }
/// ```
pub trait PaintWorklet {
    /// Get the name of this worklet (used in CSS paint() function)
    fn name(&self) -> &str;

    /// Get the CSS custom properties this worklet reads
    ///
    /// These properties will be passed to the worklet at paint time.
    /// Example: `["--my-color", "--my-size"]`
    fn input_properties(&self) -> Vec<String>;

    /// Get the argument types this worklet accepts in the paint() function
    ///
    /// Example: For `paint(my-worklet, 10, red)`, return
    /// `[PaintArgumentType::Number, PaintArgumentType::Color]`
    fn input_arguments(&self) -> Vec<PaintArgumentType>;

    /// Perform the paint operations
    ///
    /// This is called when the element needs to be painted.
    /// Use the provided context to draw, and the size for geometry.
    fn paint(&self, ctx: &mut PaintContext, size: PaintSize, args: &[PaintValue]);
}

// ============================================================================
// Re-exports
// ============================================================================

pub use parser::{parse_paint_function, ParsedPaintFunction};
pub use registry::PaintWorkletRegistry;

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paint_size_creation() {
        let size = PaintSize::new(100.0, 50.0);
        assert_eq!(size.width, 100.0);
        assert_eq!(size.height, 50.0);
    }

    #[test]
    fn test_paint_size_aspect_ratio() {
        let size = PaintSize::new(200.0, 100.0);
        assert_eq!(size.aspect_ratio(), 2.0);
    }

    #[test]
    fn test_paint_size_aspect_ratio_zero_height() {
        let size = PaintSize::new(100.0, 0.0);
        assert_eq!(size.aspect_ratio(), 1.0);
    }

    #[test]
    fn test_paint_size_area() {
        let size = PaintSize::new(10.0, 20.0);
        assert_eq!(size.area(), 200.0);
    }

    #[test]
    fn test_paint_size_is_empty() {
        assert!(PaintSize::new(0.0, 100.0).is_empty());
        assert!(PaintSize::new(100.0, 0.0).is_empty());
        assert!(PaintSize::new(-1.0, 100.0).is_empty());
        assert!(!PaintSize::new(100.0, 100.0).is_empty());
    }

    #[test]
    fn test_paint_value_number() {
        let value = PaintValue::Number(42.0);
        assert_eq!(value.argument_type(), PaintArgumentType::Number);
        assert_eq!(value.as_number(), Some(42.0));
        assert_eq!(value.as_color(), None);
    }

    #[test]
    fn test_paint_value_color() {
        let color = Color::rgb(255, 0, 0);
        let value = PaintValue::Color(color);
        assert_eq!(value.argument_type(), PaintArgumentType::Color);
        assert!(value.as_color().is_some());
        assert_eq!(value.as_number(), None);
    }

    #[test]
    fn test_paint_value_length() {
        let value = PaintValue::Length(100.0);
        assert_eq!(value.argument_type(), PaintArgumentType::Length);
        assert_eq!(value.as_length(), Some(100.0));
    }

    #[test]
    fn test_paint_value_percentage() {
        let value = PaintValue::Percentage(0.5);
        assert_eq!(value.argument_type(), PaintArgumentType::Percentage);
        assert_eq!(value.as_percentage(), Some(0.5));
    }

    #[test]
    fn test_paint_value_angle() {
        let value = PaintValue::Angle(std::f32::consts::PI);
        assert_eq!(value.argument_type(), PaintArgumentType::Angle);
        assert_eq!(value.as_angle(), Some(std::f32::consts::PI));
    }

    #[test]
    fn test_paint_value_ident() {
        let value = PaintValue::Ident("solid".to_string());
        assert_eq!(value.argument_type(), PaintArgumentType::Ident);
        assert_eq!(value.as_ident(), Some("solid"));
    }

    #[test]
    fn test_paint_context_creation() {
        let ctx = PaintContext::new();
        assert!(ctx.operations().is_empty());
        assert_eq!(ctx.line_width(), 1.0);
        assert_eq!(ctx.global_alpha(), 1.0);
    }

    #[test]
    fn test_paint_context_fill_rect() {
        let mut ctx = PaintContext::new();
        ctx.fill_rect(10.0, 20.0, 100.0, 50.0);

        assert_eq!(ctx.operations().len(), 1);
        assert_eq!(
            ctx.operations()[0],
            PaintOperation::FillRect(10.0, 20.0, 100.0, 50.0)
        );
    }

    #[test]
    fn test_paint_context_stroke_rect() {
        let mut ctx = PaintContext::new();
        ctx.stroke_rect(0.0, 0.0, 50.0, 50.0);

        assert_eq!(
            ctx.operations()[0],
            PaintOperation::StrokeRect(0.0, 0.0, 50.0, 50.0)
        );
    }

    #[test]
    fn test_paint_context_path_operations() {
        let mut ctx = PaintContext::new();
        ctx.begin_path();
        ctx.move_to(0.0, 0.0);
        ctx.line_to(100.0, 100.0);
        ctx.close_path();
        ctx.stroke();

        assert_eq!(ctx.operations().len(), 5);
        assert_eq!(ctx.operations()[0], PaintOperation::BeginPath);
        assert_eq!(ctx.operations()[1], PaintOperation::MoveTo(0.0, 0.0));
        assert_eq!(ctx.operations()[2], PaintOperation::LineTo(100.0, 100.0));
        assert_eq!(ctx.operations()[3], PaintOperation::ClosePath);
        assert_eq!(ctx.operations()[4], PaintOperation::Stroke);
    }

    #[test]
    fn test_paint_context_arc() {
        let mut ctx = PaintContext::new();
        ctx.arc(50.0, 50.0, 25.0, 0.0, std::f32::consts::PI);

        assert_eq!(
            ctx.operations()[0],
            PaintOperation::Arc(50.0, 50.0, 25.0, 0.0, std::f32::consts::PI)
        );
    }

    #[test]
    fn test_paint_context_set_fill_color() {
        let mut ctx = PaintContext::new();
        let color = Color::rgb(255, 128, 64);
        ctx.set_fill_color(color);

        assert_eq!(ctx.fill_color(), Some(&color));
        assert_eq!(ctx.operations()[0], PaintOperation::SetFillColor(color));
    }

    #[test]
    fn test_paint_context_set_stroke_color() {
        let mut ctx = PaintContext::new();
        let color = Color::rgb(0, 255, 0);
        ctx.set_stroke_color(color);

        assert_eq!(ctx.stroke_color(), Some(&color));
        assert_eq!(ctx.operations()[0], PaintOperation::SetStrokeColor(color));
    }

    #[test]
    fn test_paint_context_set_line_width() {
        let mut ctx = PaintContext::new();
        ctx.set_line_width(5.0);

        assert_eq!(ctx.line_width(), 5.0);
        assert_eq!(ctx.operations()[0], PaintOperation::SetLineWidth(5.0));
    }

    #[test]
    fn test_paint_context_set_global_alpha() {
        let mut ctx = PaintContext::new();
        ctx.set_global_alpha(0.5);

        assert_eq!(ctx.global_alpha(), 0.5);
        assert_eq!(ctx.operations()[0], PaintOperation::SetGlobalAlpha(0.5));
    }

    #[test]
    fn test_paint_context_global_alpha_clamp() {
        let mut ctx = PaintContext::new();

        ctx.set_global_alpha(1.5);
        assert_eq!(ctx.global_alpha(), 1.0);

        ctx.set_global_alpha(-0.5);
        assert_eq!(ctx.global_alpha(), 0.0);
    }

    #[test]
    fn test_paint_context_save_restore() {
        let mut ctx = PaintContext::new();
        ctx.save();
        ctx.restore();

        assert_eq!(ctx.operations()[0], PaintOperation::Save);
        assert_eq!(ctx.operations()[1], PaintOperation::Restore);
    }

    #[test]
    fn test_paint_context_transforms() {
        let mut ctx = PaintContext::new();
        ctx.translate(10.0, 20.0);
        ctx.rotate(std::f32::consts::FRAC_PI_2);
        ctx.scale(2.0, 2.0);

        assert_eq!(ctx.operations()[0], PaintOperation::Translate(10.0, 20.0));
        assert_eq!(
            ctx.operations()[1],
            PaintOperation::Rotate(std::f32::consts::FRAC_PI_2)
        );
        assert_eq!(ctx.operations()[2], PaintOperation::Scale(2.0, 2.0));
    }

    #[test]
    fn test_paint_context_fill_circle() {
        let mut ctx = PaintContext::new();
        ctx.fill_circle(50.0, 50.0, 25.0);

        // Should have: begin_path, arc, close_path, fill
        assert!(ctx.operations().len() >= 4);
        assert_eq!(ctx.operations()[0], PaintOperation::BeginPath);
        // Last operation should be fill
        assert_eq!(*ctx.operations().last().unwrap(), PaintOperation::Fill);
    }

    #[test]
    fn test_paint_context_line() {
        let mut ctx = PaintContext::new();
        ctx.line(0.0, 0.0, 100.0, 100.0);

        // Should have: begin_path, move_to, line_to, stroke
        assert_eq!(ctx.operations().len(), 4);
        assert_eq!(ctx.operations()[0], PaintOperation::BeginPath);
        assert_eq!(ctx.operations()[1], PaintOperation::MoveTo(0.0, 0.0));
        assert_eq!(ctx.operations()[2], PaintOperation::LineTo(100.0, 100.0));
        assert_eq!(ctx.operations()[3], PaintOperation::Stroke);
    }

    #[test]
    fn test_paint_context_clear() {
        let mut ctx = PaintContext::new();
        ctx.fill_rect(0.0, 0.0, 100.0, 100.0);
        assert!(!ctx.operations().is_empty());

        ctx.clear();
        assert!(ctx.operations().is_empty());
    }

    #[test]
    fn test_paint_context_take_operations() {
        let mut ctx = PaintContext::new();
        ctx.fill_rect(0.0, 0.0, 100.0, 100.0);

        let ops = ctx.take_operations();
        assert_eq!(ops.len(), 1);
    }

    #[test]
    fn test_paint_error_display() {
        let error = PaintError::WorkletNotFound("my-worklet".to_string());
        assert!(error.to_string().contains("my-worklet"));

        let error = PaintError::ArgumentCountMismatch {
            expected: 2,
            got: 3,
        };
        assert!(error.to_string().contains("2"));
        assert!(error.to_string().contains("3"));
    }

    #[test]
    fn test_line_cap_default() {
        assert_eq!(LineCap::default(), LineCap::Butt);
    }

    #[test]
    fn test_line_join_default() {
        assert_eq!(LineJoin::default(), LineJoin::Miter);
    }

    // Example worklet for testing
    struct TestWorklet;

    impl PaintWorklet for TestWorklet {
        fn name(&self) -> &str {
            "test-worklet"
        }

        fn input_properties(&self) -> Vec<String> {
            vec!["--test-color".to_string()]
        }

        fn input_arguments(&self) -> Vec<PaintArgumentType> {
            vec![PaintArgumentType::Number, PaintArgumentType::Color]
        }

        fn paint(&self, ctx: &mut PaintContext, size: PaintSize, args: &[PaintValue]) {
            let num = args.first().and_then(|a| a.as_number()).unwrap_or(10.0);
            ctx.fill_rect(0.0, 0.0, size.width * num / 100.0, size.height);
        }
    }

    #[test]
    fn test_worklet_trait() {
        let worklet = TestWorklet;
        assert_eq!(worklet.name(), "test-worklet");
        assert_eq!(worklet.input_properties().len(), 1);
        assert_eq!(worklet.input_arguments().len(), 2);
    }

    #[test]
    fn test_worklet_paint() {
        let worklet = TestWorklet;
        let mut ctx = PaintContext::new();
        let size = PaintSize::new(100.0, 50.0);
        let args = vec![
            PaintValue::Number(50.0),
            PaintValue::Color(Color::rgb(255, 0, 0)),
        ];

        worklet.paint(&mut ctx, size, &args);

        assert!(!ctx.operations().is_empty());
        assert_eq!(
            ctx.operations()[0],
            PaintOperation::FillRect(0.0, 0.0, 50.0, 50.0)
        );
    }
}
