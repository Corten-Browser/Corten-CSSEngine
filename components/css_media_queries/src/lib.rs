//! CSS Media Queries - Parsing, evaluation, and viewport management
//!
//! This module provides CSS media query support including:
//! - Media type parsing (all, screen, print, speech)
//! - Media feature parsing (width, height, orientation, resolution, user preferences)
//! - Range queries (min-width, max-width, etc.)
//! - Logical operators (and, or, not)
//! - Media query evaluation against viewport information

pub use css_parser_core::ParseError;
pub use css_types::{Length, LengthUnit};

// ============================================================================
// Media Types
// ============================================================================

/// Media type (all, screen, print, speech)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaType {
    /// All media types
    All,
    /// Screen devices
    Screen,
    /// Print media
    Print,
    /// Speech synthesizers
    Speech,
}

// ============================================================================
// Media Features
// ============================================================================

/// Orientation (portrait or landscape)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    /// Portrait orientation (height > width)
    Portrait,
    /// Landscape orientation (width > height)
    Landscape,
}

/// Resolution unit
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResolutionUnit {
    /// Dots per inch
    Dpi,
    /// Dots per centimeter
    Dpcm,
    /// Device pixel ratio
    Dppx,
}

/// Screen resolution
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Resolution {
    /// Resolution value
    pub value: f32,
    /// Resolution unit
    pub unit: ResolutionUnit,
}

impl Resolution {
    /// Create a new resolution
    pub fn new(value: f32, unit: ResolutionUnit) -> Self {
        Self { value, unit }
    }

    /// Convert resolution to DPI
    pub fn to_dpi(&self) -> f32 {
        match self.unit {
            ResolutionUnit::Dpi => self.value,
            ResolutionUnit::Dpcm => self.value * 2.54,
            ResolutionUnit::Dppx => self.value * 96.0,
        }
    }
}

/// Scanning process (for TV)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scan {
    /// Interlaced scanning
    Interlace,
    /// Progressive scanning
    Progressive,
}

/// Output device update frequency
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Update {
    /// No update capability
    None,
    /// Slow update
    Slow,
    /// Fast update
    Fast,
}

/// Hover capability
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HoverCapability {
    /// No hover capability
    None,
    /// Hover supported
    Hover,
}

/// Pointer accuracy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PointerCapability {
    /// No pointer
    None,
    /// Coarse pointer (e.g., touch)
    Coarse,
    /// Fine pointer (e.g., mouse)
    Fine,
}

/// Preferred color scheme
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorScheme {
    /// Light color scheme
    Light,
    /// Dark color scheme
    Dark,
}

/// Reduced motion preference
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReducedMotion {
    /// No preference
    NoPreference,
    /// Reduce motion
    Reduce,
}

/// Contrast preference
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Contrast {
    /// No preference
    NoPreference,
    /// More contrast
    More,
    /// Less contrast
    Less,
}

/// Media feature query
#[derive(Debug, Clone, PartialEq)]
pub enum MediaFeature {
    /// Width feature (min-width, max-width, width)
    Width(Option<Length>),
    /// Height feature (min-height, max-height, height)
    Height(Option<Length>),
    /// Aspect ratio (numerator:denominator)
    AspectRatio { numerator: u32, denominator: u32 },
    /// Orientation
    Orientation(Orientation),
    /// Resolution
    Resolution(Resolution),
    /// Color index
    ColorIndex(Option<u32>),
    /// Color bits per component
    Color(Option<u32>),
    /// Monochrome bits
    Monochrome(Option<u32>),
    /// Grid device
    Grid(bool),
    /// Scanning process
    Scan(Scan),
    /// Update frequency
    Update(Update),
    /// Hover capability
    Hover(HoverCapability),
    /// Pointer capability
    Pointer(PointerCapability),
    /// Preferred color scheme
    PrefersColorScheme(ColorScheme),
    /// Reduced motion preference
    PrefersReducedMotion(ReducedMotion),
    /// Contrast preference
    PrefersContrast(Contrast),
}

// ============================================================================
// Media Conditions
// ============================================================================

/// Range comparison type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RangeType {
    /// Exact match
    Exact,
    /// Minimum value (>=)
    Min,
    /// Maximum value (<=)
    Max,
}

/// Media query condition (AND/OR/NOT combinations)
#[derive(Debug, Clone, PartialEq)]
pub enum MediaCondition {
    /// Single feature condition
    Feature {
        feature: MediaFeature,
        range: RangeType,
    },
    /// AND combination
    And {
        left: Box<MediaCondition>,
        right: Box<MediaCondition>,
    },
    /// OR combination
    Or {
        left: Box<MediaCondition>,
        right: Box<MediaCondition>,
    },
    /// NOT negation
    Not { condition: Box<MediaCondition> },
}

// ============================================================================
// Media Query
// ============================================================================

/// Complete media query
#[derive(Debug, Clone, PartialEq)]
pub struct MediaQuery {
    /// Media type (optional)
    pub media_type: Option<MediaType>,
    /// Media condition (optional)
    pub condition: Option<MediaCondition>,
    /// Whether the query is negated (NOT)
    pub negated: bool,
}

impl MediaQuery {
    /// Create a new media query
    pub fn new(
        media_type: Option<MediaType>,
        condition: Option<MediaCondition>,
        negated: bool,
    ) -> Self {
        Self {
            media_type,
            condition,
            negated,
        }
    }

    /// Create a simple media type query
    pub fn media_type(media_type: MediaType) -> Self {
        Self::new(Some(media_type), None, false)
    }

    /// Create a condition-only query
    pub fn condition(condition: MediaCondition) -> Self {
        Self::new(None, Some(condition), false)
    }
}

/// List of media queries (comma-separated)
#[derive(Debug, Clone, PartialEq)]
pub struct MediaQueryList {
    /// List of media queries
    pub queries: Vec<MediaQuery>,
}

impl MediaQueryList {
    /// Create a new media query list
    pub fn new(queries: Vec<MediaQuery>) -> Self {
        Self { queries }
    }

    /// Create an empty media query list
    pub fn empty() -> Self {
        Self::new(Vec::new())
    }
}

// ============================================================================
// Viewport Information
// ============================================================================

/// Current viewport information
#[derive(Debug, Clone, PartialEq)]
pub struct ViewportInfo {
    /// Viewport width in pixels
    pub width: u32,
    /// Viewport height in pixels
    pub height: u32,
    /// Device width in pixels
    pub device_width: u32,
    /// Device height in pixels
    pub device_height: u32,
    /// Device pixel ratio
    pub device_pixel_ratio: f32,
    /// Current orientation
    pub orientation: Orientation,
    /// Color bits per component
    pub color_bits: u32,
    /// Monochrome bits
    pub monochrome_bits: u32,
    /// Resolution in DPI
    pub resolution_dpi: f32,
}

impl ViewportInfo {
    /// Create a new viewport
    pub fn new(width: u32, height: u32) -> Self {
        let orientation = if height > width {
            Orientation::Portrait
        } else {
            Orientation::Landscape
        };

        Self {
            width,
            height,
            device_width: width,
            device_height: height,
            device_pixel_ratio: 1.0,
            orientation,
            color_bits: 24,
            monochrome_bits: 0,
            resolution_dpi: 96.0,
        }
    }

    /// Create a desktop viewport (1920x1080)
    pub fn desktop() -> Self {
        Self::new(1920, 1080)
    }

    /// Create a tablet viewport (768x1024)
    pub fn tablet() -> Self {
        Self::new(768, 1024)
    }

    /// Create a mobile viewport (375x667)
    pub fn mobile() -> Self {
        Self::new(375, 667)
    }
}

// ============================================================================
// Parsing Functions
// ============================================================================

mod evaluator;
mod parser;

pub use evaluator::{
    evaluate_media_feature, evaluate_media_query, match_media_type, DefaultEvaluator,
    MediaQueryEvaluator,
};
pub use parser::{parse_media_query, parse_media_query_list};
