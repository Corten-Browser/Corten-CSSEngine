//! CSS Fonts - Platform Font Discovery for Corten CSS Engine
//!
//! This module provides platform-agnostic font discovery capabilities:
//! - `FontDiscovery` trait for querying system fonts
//! - `FontInfo` struct containing font metadata
//! - `FontStyle` enum (Normal, Italic, Oblique)
//! - `FontWeight` struct (100-900 weight values)
//!
//! Platform-specific implementations:
//! - Linux: fontconfig (feature: `fontconfig`)
//! - Windows: DirectWrite (feature: `directwrite`)
//! - macOS: Core Text (feature: `coretext`)
//!
//! # Example
//!
//! ```
//! use css_fonts::{FontDiscovery, FontStyle, FontWeight, get_font_discovery};
//!
//! let discovery = get_font_discovery();
//! let families = discovery.list_families();
//! println!("Available font families: {:?}", families);
//!
//! if let Some(font) = discovery.find_font("Arial", FontStyle::Normal, FontWeight::NORMAL) {
//!     println!("Found font: {} at {:?}", font.family(), font.path());
//! }
//! ```

use std::fmt;
use std::path::PathBuf;

// Platform-specific modules
#[cfg(feature = "fontconfig")]
mod fontconfig;

#[cfg(feature = "directwrite")]
mod directwrite;

#[cfg(feature = "coretext")]
mod coretext;

mod fallback;

// ============================================================================
// Error Types
// ============================================================================

/// Error type for font operations
#[derive(Debug, Clone, PartialEq)]
pub enum FontError {
    /// Font not found
    NotFound(String),
    /// Invalid font file
    InvalidFont(String),
    /// Platform-specific error
    PlatformError(String),
    /// Invalid font weight value
    InvalidWeight(u16),
}

impl fmt::Display for FontError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FontError::NotFound(msg) => write!(f, "Font not found: {}", msg),
            FontError::InvalidFont(msg) => write!(f, "Invalid font: {}", msg),
            FontError::PlatformError(msg) => write!(f, "Platform error: {}", msg),
            FontError::InvalidWeight(w) => write!(f, "Invalid font weight: {} (must be 1-1000)", w),
        }
    }
}

impl std::error::Error for FontError {}

// ============================================================================
// Font Style
// ============================================================================

/// CSS font-style values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum FontStyle {
    /// Normal (upright) style
    #[default]
    Normal,
    /// Italic style (designed italic glyphs)
    Italic,
    /// Oblique style (slanted version of normal)
    Oblique,
}

impl FontStyle {
    /// Parse font style from string
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "normal" => Some(FontStyle::Normal),
            "italic" => Some(FontStyle::Italic),
            "oblique" => Some(FontStyle::Oblique),
            _ => None,
        }
    }

    /// Convert to CSS string representation
    pub fn to_css(&self) -> &'static str {
        match self {
            FontStyle::Normal => "normal",
            FontStyle::Italic => "italic",
            FontStyle::Oblique => "oblique",
        }
    }
}

impl fmt::Display for FontStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_css())
    }
}

// ============================================================================
// Font Weight
// ============================================================================

/// CSS font-weight value (100-900, with common named values)
///
/// Font weights follow the CSS specification:
/// - 100: Thin (Hairline)
/// - 200: Extra Light (Ultra Light)
/// - 300: Light
/// - 400: Normal (Regular)
/// - 500: Medium
/// - 600: Semi Bold (Demi Bold)
/// - 700: Bold
/// - 800: Extra Bold (Ultra Bold)
/// - 900: Black (Heavy)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FontWeight(u16);

impl FontWeight {
    /// Thin (100)
    pub const THIN: FontWeight = FontWeight(100);
    /// Extra Light (200)
    pub const EXTRA_LIGHT: FontWeight = FontWeight(200);
    /// Light (300)
    pub const LIGHT: FontWeight = FontWeight(300);
    /// Normal/Regular (400)
    pub const NORMAL: FontWeight = FontWeight(400);
    /// Medium (500)
    pub const MEDIUM: FontWeight = FontWeight(500);
    /// Semi Bold (600)
    pub const SEMI_BOLD: FontWeight = FontWeight(600);
    /// Bold (700)
    pub const BOLD: FontWeight = FontWeight(700);
    /// Extra Bold (800)
    pub const EXTRA_BOLD: FontWeight = FontWeight(800);
    /// Black/Heavy (900)
    pub const BLACK: FontWeight = FontWeight(900);

    /// Create a new font weight from a numeric value (1-1000)
    ///
    /// # Errors
    ///
    /// Returns `FontError::InvalidWeight` if the value is outside 1-1000.
    pub fn new(weight: u16) -> Result<Self, FontError> {
        if weight == 0 || weight > 1000 {
            return Err(FontError::InvalidWeight(weight));
        }
        Ok(FontWeight(weight))
    }

    /// Create a font weight, clamping to valid range (1-1000)
    pub fn clamped(weight: u16) -> Self {
        FontWeight(weight.clamp(1, 1000))
    }

    /// Get the numeric weight value
    pub fn value(&self) -> u16 {
        self.0
    }

    /// Parse font weight from string (numeric or named)
    pub fn parse(s: &str) -> Option<Self> {
        // Try numeric first
        if let Ok(num) = s.parse::<u16>() {
            if (1..=1000).contains(&num) {
                return Some(FontWeight(num));
            }
            return None;
        }

        // Try named weights
        match s.to_lowercase().as_str() {
            "thin" | "hairline" => Some(FontWeight::THIN),
            "extra-light" | "extralight" | "ultra-light" | "ultralight" => {
                Some(FontWeight::EXTRA_LIGHT)
            }
            "light" => Some(FontWeight::LIGHT),
            "normal" | "regular" => Some(FontWeight::NORMAL),
            "medium" => Some(FontWeight::MEDIUM),
            "semi-bold" | "semibold" | "demi-bold" | "demibold" => Some(FontWeight::SEMI_BOLD),
            "bold" => Some(FontWeight::BOLD),
            "extra-bold" | "extrabold" | "ultra-bold" | "ultrabold" => Some(FontWeight::EXTRA_BOLD),
            "black" | "heavy" => Some(FontWeight::BLACK),
            _ => None,
        }
    }

    /// Check if this is a "bold" weight (>= 700)
    pub fn is_bold(&self) -> bool {
        self.0 >= 700
    }

    /// Get the closest standard weight (100, 200, ..., 900)
    pub fn to_standard(&self) -> FontWeight {
        let rounded = ((self.0 + 50) / 100) * 100;
        FontWeight(rounded.clamp(100, 900))
    }

    /// Calculate the distance between two weights (for font matching)
    pub fn distance(&self, other: &FontWeight) -> u16 {
        (self.0 as i32 - other.0 as i32).unsigned_abs() as u16
    }
}

impl Default for FontWeight {
    fn default() -> Self {
        FontWeight::NORMAL
    }
}

impl fmt::Display for FontWeight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ============================================================================
// Font Stretch
// ============================================================================

/// CSS font-stretch values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum FontStretch {
    /// Ultra-condensed (50%)
    UltraCondensed,
    /// Extra-condensed (62.5%)
    ExtraCondensed,
    /// Condensed (75%)
    Condensed,
    /// Semi-condensed (87.5%)
    SemiCondensed,
    /// Normal (100%)
    #[default]
    Normal,
    /// Semi-expanded (112.5%)
    SemiExpanded,
    /// Expanded (125%)
    Expanded,
    /// Extra-expanded (150%)
    ExtraExpanded,
    /// Ultra-expanded (200%)
    UltraExpanded,
}

impl FontStretch {
    /// Get the percentage value (50-200)
    pub fn percentage(&self) -> f32 {
        match self {
            FontStretch::UltraCondensed => 50.0,
            FontStretch::ExtraCondensed => 62.5,
            FontStretch::Condensed => 75.0,
            FontStretch::SemiCondensed => 87.5,
            FontStretch::Normal => 100.0,
            FontStretch::SemiExpanded => 112.5,
            FontStretch::Expanded => 125.0,
            FontStretch::ExtraExpanded => 150.0,
            FontStretch::UltraExpanded => 200.0,
        }
    }

    /// Parse from percentage value
    pub fn from_percentage(pct: f32) -> Self {
        if pct <= 50.0 {
            FontStretch::UltraCondensed
        } else if pct <= 62.5 {
            FontStretch::ExtraCondensed
        } else if pct <= 75.0 {
            FontStretch::Condensed
        } else if pct <= 87.5 {
            FontStretch::SemiCondensed
        } else if pct <= 100.0 {
            FontStretch::Normal
        } else if pct <= 112.5 {
            FontStretch::SemiExpanded
        } else if pct <= 125.0 {
            FontStretch::Expanded
        } else if pct <= 150.0 {
            FontStretch::ExtraExpanded
        } else {
            FontStretch::UltraExpanded
        }
    }

    /// Parse from string
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "ultra-condensed" | "ultracondensed" => Some(FontStretch::UltraCondensed),
            "extra-condensed" | "extracondensed" => Some(FontStretch::ExtraCondensed),
            "condensed" => Some(FontStretch::Condensed),
            "semi-condensed" | "semicondensed" => Some(FontStretch::SemiCondensed),
            "normal" => Some(FontStretch::Normal),
            "semi-expanded" | "semiexpanded" => Some(FontStretch::SemiExpanded),
            "expanded" => Some(FontStretch::Expanded),
            "extra-expanded" | "extraexpanded" => Some(FontStretch::ExtraExpanded),
            "ultra-expanded" | "ultraexpanded" => Some(FontStretch::UltraExpanded),
            _ => None,
        }
    }

    /// Convert to CSS string
    pub fn to_css(&self) -> &'static str {
        match self {
            FontStretch::UltraCondensed => "ultra-condensed",
            FontStretch::ExtraCondensed => "extra-condensed",
            FontStretch::Condensed => "condensed",
            FontStretch::SemiCondensed => "semi-condensed",
            FontStretch::Normal => "normal",
            FontStretch::SemiExpanded => "semi-expanded",
            FontStretch::Expanded => "expanded",
            FontStretch::ExtraExpanded => "extra-expanded",
            FontStretch::UltraExpanded => "ultra-expanded",
        }
    }
}

impl fmt::Display for FontStretch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_css())
    }
}

// ============================================================================
// Font Info
// ============================================================================

/// Information about a discovered font
#[derive(Debug, Clone, PartialEq)]
pub struct FontInfo {
    /// Font family name (e.g., "Arial", "Times New Roman")
    family: String,
    /// PostScript name (unique identifier)
    postscript_name: Option<String>,
    /// Full font name (e.g., "Arial Bold Italic")
    full_name: Option<String>,
    /// Font style
    style: FontStyle,
    /// Font weight
    weight: FontWeight,
    /// Font stretch
    stretch: FontStretch,
    /// Path to the font file
    path: Option<PathBuf>,
    /// Index within the font file (for .ttc collections)
    index: u32,
    /// Whether this is a variable font
    is_variable: bool,
}

impl FontInfo {
    /// Create a new FontInfo with required fields
    pub fn new(family: impl Into<String>, style: FontStyle, weight: FontWeight) -> Self {
        Self {
            family: family.into(),
            postscript_name: None,
            full_name: None,
            style,
            weight,
            stretch: FontStretch::Normal,
            path: None,
            index: 0,
            is_variable: false,
        }
    }

    /// Builder method: set PostScript name
    pub fn with_postscript_name(mut self, name: impl Into<String>) -> Self {
        self.postscript_name = Some(name.into());
        self
    }

    /// Builder method: set full name
    pub fn with_full_name(mut self, name: impl Into<String>) -> Self {
        self.full_name = Some(name.into());
        self
    }

    /// Builder method: set font stretch
    pub fn with_stretch(mut self, stretch: FontStretch) -> Self {
        self.stretch = stretch;
        self
    }

    /// Builder method: set file path
    pub fn with_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.path = Some(path.into());
        self
    }

    /// Builder method: set font index (for .ttc collections)
    pub fn with_index(mut self, index: u32) -> Self {
        self.index = index;
        self
    }

    /// Builder method: mark as variable font
    pub fn with_variable(mut self, is_variable: bool) -> Self {
        self.is_variable = is_variable;
        self
    }

    /// Get the font family name
    pub fn family(&self) -> &str {
        &self.family
    }

    /// Get the PostScript name (if available)
    pub fn postscript_name(&self) -> Option<&str> {
        self.postscript_name.as_deref()
    }

    /// Get the full font name (if available)
    pub fn full_name(&self) -> Option<&str> {
        self.full_name.as_deref()
    }

    /// Get the font style
    pub fn style(&self) -> FontStyle {
        self.style
    }

    /// Get the font weight
    pub fn weight(&self) -> FontWeight {
        self.weight
    }

    /// Get the font stretch
    pub fn stretch(&self) -> FontStretch {
        self.stretch
    }

    /// Get the font file path (if available)
    pub fn path(&self) -> Option<&PathBuf> {
        self.path.as_ref()
    }

    /// Get the font index within the file
    pub fn index(&self) -> u32 {
        self.index
    }

    /// Check if this is a variable font
    pub fn is_variable(&self) -> bool {
        self.is_variable
    }

    /// Calculate match score against requested properties (lower is better)
    pub fn match_score(&self, style: FontStyle, weight: FontWeight) -> u32 {
        let mut score = 0u32;

        // Style mismatch penalty
        if self.style != style {
            score += match (self.style, style) {
                // Oblique is closer to Italic than Normal
                (FontStyle::Oblique, FontStyle::Italic)
                | (FontStyle::Italic, FontStyle::Oblique) => 100,
                // Normal vs Italic/Oblique is a bigger mismatch
                _ => 1000,
            };
        }

        // Weight distance
        score += self.weight.distance(&weight) as u32;

        score
    }
}

impl fmt::Display for FontInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.family, self.style, self.weight)
    }
}

// ============================================================================
// Font Discovery Trait
// ============================================================================

/// Trait for platform-specific font discovery
///
/// This trait provides an abstraction over different platform font APIs:
/// - Linux: fontconfig
/// - Windows: DirectWrite
/// - macOS: Core Text
///
/// Use `get_font_discovery()` to get the appropriate implementation for
/// the current platform.
pub trait FontDiscovery: Send + Sync {
    /// Find a font matching the specified criteria
    ///
    /// Returns the best matching font, or None if no suitable font is found.
    ///
    /// # Arguments
    ///
    /// * `family` - Font family name (e.g., "Arial")
    /// * `style` - Desired font style
    /// * `weight` - Desired font weight
    fn find_font(&self, family: &str, style: FontStyle, weight: FontWeight) -> Option<FontInfo>;

    /// Find all fonts matching the family name
    ///
    /// Returns all variants (weights, styles) of the specified font family.
    fn find_family(&self, family: &str) -> Vec<FontInfo> {
        self.system_fonts()
            .into_iter()
            .filter(|f| f.family().eq_ignore_ascii_case(family))
            .collect()
    }

    /// List all available font family names
    ///
    /// Returns a sorted, deduplicated list of font family names.
    fn list_families(&self) -> Vec<String>;

    /// Get all system fonts
    ///
    /// Returns information about every font installed on the system.
    fn system_fonts(&self) -> Vec<FontInfo>;

    /// Find the best match for a font query with fallbacks
    ///
    /// Tries each family in order and returns the first match.
    fn find_with_fallbacks(
        &self,
        families: &[&str],
        style: FontStyle,
        weight: FontWeight,
    ) -> Option<FontInfo> {
        for family in families {
            if let Some(font) = self.find_font(family, style, weight) {
                return Some(font);
            }
        }
        None
    }

    /// Check if a specific font family is available
    fn has_family(&self, family: &str) -> bool {
        self.list_families()
            .iter()
            .any(|f| f.eq_ignore_ascii_case(family))
    }
}

// ============================================================================
// Platform-specific Factory
// ============================================================================

/// Get the font discovery implementation for the current platform
///
/// This function returns the appropriate `FontDiscovery` implementation
/// based on compile-time feature flags:
///
/// - `fontconfig` feature: Linux fontconfig implementation
/// - `directwrite` feature: Windows DirectWrite implementation
/// - `coretext` feature: macOS Core Text implementation
/// - Default: Fallback implementation with common font names
///
/// # Example
///
/// ```
/// use css_fonts::get_font_discovery;
///
/// let discovery = get_font_discovery();
/// let families = discovery.list_families();
/// println!("Found {} font families", families.len());
/// ```
pub fn get_font_discovery() -> Box<dyn FontDiscovery> {
    #[cfg(feature = "fontconfig")]
    {
        Box::new(fontconfig::FontconfigDiscovery::new())
    }

    #[cfg(feature = "directwrite")]
    {
        Box::new(directwrite::DirectWriteDiscovery::new())
    }

    #[cfg(feature = "coretext")]
    {
        Box::new(coretext::CoreTextDiscovery::new())
    }

    #[cfg(not(any(feature = "fontconfig", feature = "directwrite", feature = "coretext")))]
    {
        Box::new(fallback::FallbackDiscovery::new())
    }
}

// ============================================================================
// Re-exports for convenience
// ============================================================================

pub use fallback::FallbackDiscovery;

#[cfg(feature = "fontconfig")]
pub use fontconfig::FontconfigDiscovery;

#[cfg(feature = "directwrite")]
pub use directwrite::DirectWriteDiscovery;

#[cfg(feature = "coretext")]
pub use coretext::CoreTextDiscovery;

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // FontStyle Tests
    // ========================================================================

    #[test]
    fn test_font_style_default() {
        assert_eq!(FontStyle::default(), FontStyle::Normal);
    }

    #[test]
    fn test_font_style_parse() {
        assert_eq!(FontStyle::parse("normal"), Some(FontStyle::Normal));
        assert_eq!(FontStyle::parse("italic"), Some(FontStyle::Italic));
        assert_eq!(FontStyle::parse("oblique"), Some(FontStyle::Oblique));
        assert_eq!(FontStyle::parse("ITALIC"), Some(FontStyle::Italic));
        assert_eq!(FontStyle::parse("unknown"), None);
    }

    #[test]
    fn test_font_style_to_css() {
        assert_eq!(FontStyle::Normal.to_css(), "normal");
        assert_eq!(FontStyle::Italic.to_css(), "italic");
        assert_eq!(FontStyle::Oblique.to_css(), "oblique");
    }

    #[test]
    fn test_font_style_display() {
        assert_eq!(format!("{}", FontStyle::Normal), "normal");
        assert_eq!(format!("{}", FontStyle::Italic), "italic");
    }

    // ========================================================================
    // FontWeight Tests
    // ========================================================================

    #[test]
    fn test_font_weight_constants() {
        assert_eq!(FontWeight::THIN.value(), 100);
        assert_eq!(FontWeight::NORMAL.value(), 400);
        assert_eq!(FontWeight::BOLD.value(), 700);
        assert_eq!(FontWeight::BLACK.value(), 900);
    }

    #[test]
    fn test_font_weight_new_valid() {
        assert!(FontWeight::new(100).is_ok());
        assert!(FontWeight::new(400).is_ok());
        assert!(FontWeight::new(1000).is_ok());
        assert!(FontWeight::new(1).is_ok());
    }

    #[test]
    fn test_font_weight_new_invalid() {
        assert!(matches!(
            FontWeight::new(0),
            Err(FontError::InvalidWeight(0))
        ));
        assert!(matches!(
            FontWeight::new(1001),
            Err(FontError::InvalidWeight(1001))
        ));
    }

    #[test]
    fn test_font_weight_clamped() {
        assert_eq!(FontWeight::clamped(0).value(), 1);
        assert_eq!(FontWeight::clamped(500).value(), 500);
        assert_eq!(FontWeight::clamped(2000).value(), 1000);
    }

    #[test]
    fn test_font_weight_parse_numeric() {
        assert_eq!(FontWeight::parse("400"), Some(FontWeight::NORMAL));
        assert_eq!(FontWeight::parse("700"), Some(FontWeight::BOLD));
        assert_eq!(FontWeight::parse("0"), None);
        assert_eq!(FontWeight::parse("1001"), None);
    }

    #[test]
    fn test_font_weight_parse_named() {
        assert_eq!(FontWeight::parse("normal"), Some(FontWeight::NORMAL));
        assert_eq!(FontWeight::parse("bold"), Some(FontWeight::BOLD));
        assert_eq!(FontWeight::parse("thin"), Some(FontWeight::THIN));
        assert_eq!(FontWeight::parse("black"), Some(FontWeight::BLACK));
        assert_eq!(FontWeight::parse("heavy"), Some(FontWeight::BLACK));
        assert_eq!(FontWeight::parse("semi-bold"), Some(FontWeight::SEMI_BOLD));
        assert_eq!(FontWeight::parse("semibold"), Some(FontWeight::SEMI_BOLD));
    }

    #[test]
    fn test_font_weight_is_bold() {
        assert!(!FontWeight::NORMAL.is_bold());
        assert!(!FontWeight::MEDIUM.is_bold());
        assert!(!FontWeight::SEMI_BOLD.is_bold());
        assert!(FontWeight::BOLD.is_bold());
        assert!(FontWeight::BLACK.is_bold());
    }

    #[test]
    fn test_font_weight_to_standard() {
        assert_eq!(FontWeight::clamped(350).to_standard(), FontWeight::NORMAL);
        assert_eq!(FontWeight::clamped(450).to_standard(), FontWeight::MEDIUM);
        assert_eq!(FontWeight::clamped(650).to_standard(), FontWeight::BOLD);
    }

    #[test]
    fn test_font_weight_distance() {
        assert_eq!(FontWeight::NORMAL.distance(&FontWeight::BOLD), 300);
        assert_eq!(FontWeight::THIN.distance(&FontWeight::BLACK), 800);
        assert_eq!(FontWeight::NORMAL.distance(&FontWeight::NORMAL), 0);
    }

    #[test]
    fn test_font_weight_ordering() {
        assert!(FontWeight::THIN < FontWeight::NORMAL);
        assert!(FontWeight::NORMAL < FontWeight::BOLD);
        assert!(FontWeight::BOLD < FontWeight::BLACK);
    }

    // ========================================================================
    // FontStretch Tests
    // ========================================================================

    #[test]
    fn test_font_stretch_default() {
        assert_eq!(FontStretch::default(), FontStretch::Normal);
    }

    #[test]
    fn test_font_stretch_percentage() {
        assert_eq!(FontStretch::UltraCondensed.percentage(), 50.0);
        assert_eq!(FontStretch::Normal.percentage(), 100.0);
        assert_eq!(FontStretch::UltraExpanded.percentage(), 200.0);
    }

    #[test]
    fn test_font_stretch_from_percentage() {
        assert_eq!(
            FontStretch::from_percentage(50.0),
            FontStretch::UltraCondensed
        );
        assert_eq!(FontStretch::from_percentage(100.0), FontStretch::Normal);
        assert_eq!(
            FontStretch::from_percentage(200.0),
            FontStretch::UltraExpanded
        );
    }

    #[test]
    fn test_font_stretch_parse() {
        assert_eq!(FontStretch::parse("normal"), Some(FontStretch::Normal));
        assert_eq!(
            FontStretch::parse("condensed"),
            Some(FontStretch::Condensed)
        );
        assert_eq!(FontStretch::parse("expanded"), Some(FontStretch::Expanded));
        assert_eq!(
            FontStretch::parse("ultra-condensed"),
            Some(FontStretch::UltraCondensed)
        );
    }

    // ========================================================================
    // FontInfo Tests
    // ========================================================================

    #[test]
    fn test_font_info_new() {
        let info = FontInfo::new("Arial", FontStyle::Normal, FontWeight::NORMAL);
        assert_eq!(info.family(), "Arial");
        assert_eq!(info.style(), FontStyle::Normal);
        assert_eq!(info.weight(), FontWeight::NORMAL);
        assert_eq!(info.stretch(), FontStretch::Normal);
        assert!(info.path().is_none());
        assert_eq!(info.index(), 0);
        assert!(!info.is_variable());
    }

    #[test]
    fn test_font_info_builder() {
        let info = FontInfo::new("Helvetica", FontStyle::Italic, FontWeight::BOLD)
            .with_postscript_name("Helvetica-BoldOblique")
            .with_full_name("Helvetica Bold Italic")
            .with_stretch(FontStretch::Condensed)
            .with_path("/usr/share/fonts/helvetica.ttf")
            .with_index(1)
            .with_variable(true);

        assert_eq!(info.family(), "Helvetica");
        assert_eq!(info.postscript_name(), Some("Helvetica-BoldOblique"));
        assert_eq!(info.full_name(), Some("Helvetica Bold Italic"));
        assert_eq!(info.stretch(), FontStretch::Condensed);
        assert_eq!(
            info.path(),
            Some(&PathBuf::from("/usr/share/fonts/helvetica.ttf"))
        );
        assert_eq!(info.index(), 1);
        assert!(info.is_variable());
    }

    #[test]
    fn test_font_info_match_score() {
        let font = FontInfo::new("Arial", FontStyle::Normal, FontWeight::NORMAL);

        // Perfect match
        assert_eq!(font.match_score(FontStyle::Normal, FontWeight::NORMAL), 0);

        // Weight mismatch
        assert_eq!(font.match_score(FontStyle::Normal, FontWeight::BOLD), 300);

        // Style mismatch (italic vs normal)
        assert!(font.match_score(FontStyle::Italic, FontWeight::NORMAL) >= 1000);

        // Oblique is closer to italic than normal
        let italic_font = FontInfo::new("Arial", FontStyle::Italic, FontWeight::NORMAL);
        let oblique_score = italic_font.match_score(FontStyle::Oblique, FontWeight::NORMAL);
        let normal_score = italic_font.match_score(FontStyle::Normal, FontWeight::NORMAL);
        assert!(oblique_score < normal_score);
    }

    #[test]
    fn test_font_info_display() {
        let info = FontInfo::new("Arial", FontStyle::Normal, FontWeight::BOLD);
        let display = format!("{}", info);
        assert!(display.contains("Arial"));
    }

    // ========================================================================
    // FontError Tests
    // ========================================================================

    #[test]
    fn test_font_error_display() {
        let err = FontError::NotFound("Arial".to_string());
        assert!(format!("{}", err).contains("Arial"));

        let err = FontError::InvalidWeight(0);
        assert!(format!("{}", err).contains("0"));
    }

    // ========================================================================
    // Factory Tests
    // ========================================================================

    #[test]
    fn test_get_font_discovery() {
        let discovery = get_font_discovery();
        // Should return something (fallback at minimum)
        let families = discovery.list_families();
        assert!(!families.is_empty());
    }

    #[test]
    fn test_fallback_discovery_list_families() {
        let discovery = FallbackDiscovery::new();
        let families = discovery.list_families();
        assert!(!families.is_empty());
        assert!(families.iter().any(|f| f == "Arial" || f == "sans-serif"));
    }

    #[test]
    fn test_fallback_discovery_find_font() {
        let discovery = FallbackDiscovery::new();
        let font = discovery.find_font("Arial", FontStyle::Normal, FontWeight::NORMAL);
        assert!(font.is_some());
        assert_eq!(font.unwrap().family(), "Arial");
    }

    #[test]
    fn test_fallback_discovery_system_fonts() {
        let discovery = FallbackDiscovery::new();
        let fonts = discovery.system_fonts();
        assert!(!fonts.is_empty());
    }

    #[test]
    fn test_font_discovery_find_family() {
        let discovery = FallbackDiscovery::new();
        let family_fonts = discovery.find_family("Arial");
        assert!(!family_fonts.is_empty());
        assert!(family_fonts.iter().all(|f| f.family() == "Arial"));
    }

    #[test]
    fn test_font_discovery_has_family() {
        let discovery = FallbackDiscovery::new();
        assert!(discovery.has_family("Arial") || discovery.has_family("sans-serif"));
    }

    #[test]
    fn test_font_discovery_find_with_fallbacks() {
        let discovery = FallbackDiscovery::new();
        let font = discovery.find_with_fallbacks(
            &["NonExistentFont", "Arial", "sans-serif"],
            FontStyle::Normal,
            FontWeight::NORMAL,
        );
        assert!(font.is_some());
    }
}
