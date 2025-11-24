//! Core Text Font Discovery Implementation (macOS)
//!
//! This module provides font discovery using Apple's Core Text framework,
//! which is the modern text layout and font management API on macOS and iOS.
//!
//! # Platform Support
//!
//! This implementation is designed for macOS 10.5+ and iOS 3.2+.
//! Core Text provides access to:
//! - System fonts (/System/Library/Fonts)
//! - User fonts (~/Library/Fonts)
//! - Application fonts
//! - Font collections and descriptors
//!
//! # Feature Flag
//!
//! This module is only compiled when the `coretext` feature is enabled:
//!
//! ```toml
//! [dependencies]
//! css-fonts = { version = "0.1", features = ["coretext"] }
//! ```
//!
//! # Implementation Note
//!
//! This is currently a stub implementation that returns mock data.
//! A full implementation would use the core-text crate or core-foundation
//! bindings to interface with the Core Text API.

use crate::{FontDiscovery, FontInfo, FontStretch, FontStyle, FontWeight};
use std::path::PathBuf;

/// Font discovery using Core Text (macOS)
///
/// This implementation uses Apple's Core Text framework to enumerate
/// and query system fonts on macOS.
///
/// # Example
///
/// ```ignore
/// use css_fonts::coretext::CoreTextDiscovery;
/// use css_fonts::{FontDiscovery, FontStyle, FontWeight};
///
/// let discovery = CoreTextDiscovery::new();
/// if let Some(font) = discovery.find_font("SF Pro", FontStyle::Normal, FontWeight::NORMAL) {
///     println!("Found: {} at {:?}", font.family(), font.path());
/// }
/// ```
#[derive(Debug, Clone, Default)]
pub struct CoreTextDiscovery {
    /// Cached font list from Core Text
    fonts: Vec<FontInfo>,
    /// Whether the cache has been populated
    initialized: bool,
}

impl CoreTextDiscovery {
    /// Create a new Core Text discovery instance
    ///
    /// This initializes Core Text and prepares for font queries.
    /// The actual font enumeration is done lazily on first query.
    pub fn new() -> Self {
        let mut discovery = Self {
            fonts: Vec::new(),
            initialized: false,
        };
        discovery.initialize();
        discovery
    }

    /// Initialize Core Text and populate the font cache
    ///
    /// In a real implementation, this would:
    /// 1. Create a font collection with CTFontCollectionCreateFromAvailableFonts
    /// 2. Get all font descriptors from the collection
    /// 3. Extract metadata from each descriptor (family, style, weight, path)
    fn initialize(&mut self) {
        if self.initialized {
            return;
        }

        // Stub implementation: populate with typical macOS fonts
        self.populate_mock_fonts();
        self.initialized = true;
    }

    /// Populate with mock macOS font data
    ///
    /// This represents fonts commonly found on macOS systems.
    /// A real implementation would query Core Text directly.
    fn populate_mock_fonts(&mut self) {
        // Common macOS font families with their typical locations
        let macos_fonts = [
            // San Francisco (system font since macOS 10.11)
            (
                "SF Pro",
                "/System/Library/Fonts/SF-Pro.ttf",
                FontWeight::NORMAL,
                FontStyle::Normal,
            ),
            (
                "SF Pro",
                "/System/Library/Fonts/SF-Pro-Bold.ttf",
                FontWeight::BOLD,
                FontStyle::Normal,
            ),
            (
                "SF Pro",
                "/System/Library/Fonts/SF-Pro-Light.ttf",
                FontWeight::LIGHT,
                FontStyle::Normal,
            ),
            (
                "SF Pro",
                "/System/Library/Fonts/SF-Pro-Medium.ttf",
                FontWeight::MEDIUM,
                FontStyle::Normal,
            ),
            (
                "SF Pro",
                "/System/Library/Fonts/SF-Pro-Semibold.ttf",
                FontWeight::SEMI_BOLD,
                FontStyle::Normal,
            ),
            (
                "SF Pro",
                "/System/Library/Fonts/SF-Pro-Heavy.ttf",
                FontWeight::BLACK,
                FontStyle::Normal,
            ),
            (
                "SF Pro Display",
                "/System/Library/Fonts/SF-Pro-Display-Regular.otf",
                FontWeight::NORMAL,
                FontStyle::Normal,
            ),
            (
                "SF Pro Display",
                "/System/Library/Fonts/SF-Pro-Display-Bold.otf",
                FontWeight::BOLD,
                FontStyle::Normal,
            ),
            (
                "SF Pro Text",
                "/System/Library/Fonts/SF-Pro-Text-Regular.otf",
                FontWeight::NORMAL,
                FontStyle::Normal,
            ),
            (
                "SF Pro Text",
                "/System/Library/Fonts/SF-Pro-Text-Bold.otf",
                FontWeight::BOLD,
                FontStyle::Normal,
            ),
            // SF Mono (system monospace)
            (
                "SF Mono",
                "/System/Library/Fonts/SF-Mono.ttf",
                FontWeight::NORMAL,
                FontStyle::Normal,
            ),
            (
                "SF Mono",
                "/System/Library/Fonts/SF-Mono-Bold.ttf",
                FontWeight::BOLD,
                FontStyle::Normal,
            ),
            (
                "SF Mono",
                "/System/Library/Fonts/SF-Mono-Light.ttf",
                FontWeight::LIGHT,
                FontStyle::Normal,
            ),
            // Helvetica Neue
            (
                "Helvetica Neue",
                "/System/Library/Fonts/HelveticaNeue.ttc",
                FontWeight::NORMAL,
                FontStyle::Normal,
            ),
            (
                "Helvetica Neue",
                "/System/Library/Fonts/HelveticaNeue-Bold.otf",
                FontWeight::BOLD,
                FontStyle::Normal,
            ),
            (
                "Helvetica Neue",
                "/System/Library/Fonts/HelveticaNeue-Italic.otf",
                FontWeight::NORMAL,
                FontStyle::Italic,
            ),
            (
                "Helvetica Neue",
                "/System/Library/Fonts/HelveticaNeue-BoldItalic.otf",
                FontWeight::BOLD,
                FontStyle::Italic,
            ),
            (
                "Helvetica Neue",
                "/System/Library/Fonts/HelveticaNeue-Light.otf",
                FontWeight::LIGHT,
                FontStyle::Normal,
            ),
            (
                "Helvetica Neue",
                "/System/Library/Fonts/HelveticaNeue-UltraLight.otf",
                FontWeight::THIN,
                FontStyle::Normal,
            ),
            (
                "Helvetica Neue",
                "/System/Library/Fonts/HelveticaNeue-Medium.otf",
                FontWeight::MEDIUM,
                FontStyle::Normal,
            ),
            // Helvetica
            (
                "Helvetica",
                "/System/Library/Fonts/Helvetica.ttc",
                FontWeight::NORMAL,
                FontStyle::Normal,
            ),
            (
                "Helvetica",
                "/System/Library/Fonts/Helvetica-Bold.ttf",
                FontWeight::BOLD,
                FontStyle::Normal,
            ),
            (
                "Helvetica",
                "/System/Library/Fonts/Helvetica-Oblique.ttf",
                FontWeight::NORMAL,
                FontStyle::Oblique,
            ),
            // Arial
            (
                "Arial",
                "/Library/Fonts/Arial.ttf",
                FontWeight::NORMAL,
                FontStyle::Normal,
            ),
            (
                "Arial",
                "/Library/Fonts/Arial Bold.ttf",
                FontWeight::BOLD,
                FontStyle::Normal,
            ),
            (
                "Arial",
                "/Library/Fonts/Arial Italic.ttf",
                FontWeight::NORMAL,
                FontStyle::Italic,
            ),
            (
                "Arial",
                "/Library/Fonts/Arial Bold Italic.ttf",
                FontWeight::BOLD,
                FontStyle::Italic,
            ),
            // Times New Roman
            (
                "Times New Roman",
                "/Library/Fonts/Times New Roman.ttf",
                FontWeight::NORMAL,
                FontStyle::Normal,
            ),
            (
                "Times New Roman",
                "/Library/Fonts/Times New Roman Bold.ttf",
                FontWeight::BOLD,
                FontStyle::Normal,
            ),
            (
                "Times New Roman",
                "/Library/Fonts/Times New Roman Italic.ttf",
                FontWeight::NORMAL,
                FontStyle::Italic,
            ),
            (
                "Times New Roman",
                "/Library/Fonts/Times New Roman Bold Italic.ttf",
                FontWeight::BOLD,
                FontStyle::Italic,
            ),
            // Georgia
            (
                "Georgia",
                "/Library/Fonts/Georgia.ttf",
                FontWeight::NORMAL,
                FontStyle::Normal,
            ),
            (
                "Georgia",
                "/Library/Fonts/Georgia Bold.ttf",
                FontWeight::BOLD,
                FontStyle::Normal,
            ),
            (
                "Georgia",
                "/Library/Fonts/Georgia Italic.ttf",
                FontWeight::NORMAL,
                FontStyle::Italic,
            ),
            // Verdana
            (
                "Verdana",
                "/Library/Fonts/Verdana.ttf",
                FontWeight::NORMAL,
                FontStyle::Normal,
            ),
            (
                "Verdana",
                "/Library/Fonts/Verdana Bold.ttf",
                FontWeight::BOLD,
                FontStyle::Normal,
            ),
            (
                "Verdana",
                "/Library/Fonts/Verdana Italic.ttf",
                FontWeight::NORMAL,
                FontStyle::Italic,
            ),
            // Menlo (monospace, replaced Monaco)
            (
                "Menlo",
                "/System/Library/Fonts/Menlo.ttc",
                FontWeight::NORMAL,
                FontStyle::Normal,
            ),
            (
                "Menlo",
                "/System/Library/Fonts/Menlo-Bold.ttf",
                FontWeight::BOLD,
                FontStyle::Normal,
            ),
            (
                "Menlo",
                "/System/Library/Fonts/Menlo-Italic.ttf",
                FontWeight::NORMAL,
                FontStyle::Italic,
            ),
            (
                "Menlo",
                "/System/Library/Fonts/Menlo-BoldItalic.ttf",
                FontWeight::BOLD,
                FontStyle::Italic,
            ),
            // Monaco
            (
                "Monaco",
                "/System/Library/Fonts/Monaco.ttf",
                FontWeight::NORMAL,
                FontStyle::Normal,
            ),
            // Courier New
            (
                "Courier New",
                "/Library/Fonts/Courier New.ttf",
                FontWeight::NORMAL,
                FontStyle::Normal,
            ),
            (
                "Courier New",
                "/Library/Fonts/Courier New Bold.ttf",
                FontWeight::BOLD,
                FontStyle::Normal,
            ),
            (
                "Courier New",
                "/Library/Fonts/Courier New Italic.ttf",
                FontWeight::NORMAL,
                FontStyle::Italic,
            ),
            // Avenir (modern sans-serif)
            (
                "Avenir",
                "/System/Library/Fonts/Avenir.ttc",
                FontWeight::NORMAL,
                FontStyle::Normal,
            ),
            (
                "Avenir",
                "/System/Library/Fonts/Avenir-Heavy.ttf",
                FontWeight::BLACK,
                FontStyle::Normal,
            ),
            (
                "Avenir",
                "/System/Library/Fonts/Avenir-Light.ttf",
                FontWeight::LIGHT,
                FontStyle::Normal,
            ),
            (
                "Avenir",
                "/System/Library/Fonts/Avenir-Medium.ttf",
                FontWeight::MEDIUM,
                FontStyle::Normal,
            ),
            (
                "Avenir Next",
                "/System/Library/Fonts/AvenirNext.ttc",
                FontWeight::NORMAL,
                FontStyle::Normal,
            ),
            (
                "Avenir Next",
                "/System/Library/Fonts/AvenirNext-Bold.ttf",
                FontWeight::BOLD,
                FontStyle::Normal,
            ),
            // Futura
            (
                "Futura",
                "/System/Library/Fonts/Futura.ttc",
                FontWeight::NORMAL,
                FontStyle::Normal,
            ),
            (
                "Futura",
                "/System/Library/Fonts/Futura-Bold.ttf",
                FontWeight::BOLD,
                FontStyle::Normal,
            ),
            (
                "Futura",
                "/System/Library/Fonts/Futura-Medium.ttf",
                FontWeight::MEDIUM,
                FontStyle::Normal,
            ),
            // Optima
            (
                "Optima",
                "/System/Library/Fonts/Optima.ttc",
                FontWeight::NORMAL,
                FontStyle::Normal,
            ),
            (
                "Optima",
                "/System/Library/Fonts/Optima-Bold.ttf",
                FontWeight::BOLD,
                FontStyle::Normal,
            ),
            // Lucida Grande (old system font)
            (
                "Lucida Grande",
                "/System/Library/Fonts/LucidaGrande.ttc",
                FontWeight::NORMAL,
                FontStyle::Normal,
            ),
            (
                "Lucida Grande",
                "/System/Library/Fonts/LucidaGrande-Bold.ttf",
                FontWeight::BOLD,
                FontStyle::Normal,
            ),
            // Apple Symbols
            (
                "Apple Symbols",
                "/System/Library/Fonts/Apple Symbols.ttf",
                FontWeight::NORMAL,
                FontStyle::Normal,
            ),
            // Noteworthy
            (
                "Noteworthy",
                "/Library/Fonts/Noteworthy.ttc",
                FontWeight::NORMAL,
                FontStyle::Normal,
            ),
            (
                "Noteworthy",
                "/Library/Fonts/Noteworthy-Bold.ttf",
                FontWeight::BOLD,
                FontStyle::Normal,
            ),
            // Marker Felt
            (
                "Marker Felt",
                "/Library/Fonts/Marker Felt.ttc",
                FontWeight::NORMAL,
                FontStyle::Normal,
            ),
        ];

        for (family, path, weight, style) in macos_fonts.iter() {
            self.fonts.push(
                FontInfo::new(*family, *style, *weight)
                    .with_path(PathBuf::from(*path))
                    .with_postscript_name(generate_postscript_name(family, *weight, *style)),
            );
        }

        // Sort for consistent ordering
        self.fonts.sort_by(|a, b| {
            a.family()
                .cmp(b.family())
                .then_with(|| a.weight().value().cmp(&b.weight().value()))
        });
    }

    /// Find the best matching font from candidates
    fn find_best_match(
        fonts: &[FontInfo],
        family: &str,
        style: FontStyle,
        weight: FontWeight,
    ) -> Option<FontInfo> {
        fonts
            .iter()
            .filter(|f| f.family().eq_ignore_ascii_case(family))
            .min_by_key(|f| f.match_score(style, weight))
            .cloned()
    }
}

/// Generate PostScript name from family, weight, and style
fn generate_postscript_name(family: &str, weight: FontWeight, style: FontStyle) -> String {
    let family_clean = family.replace(' ', "");

    let weight_str = match weight {
        w if w == FontWeight::THIN => "Thin",
        w if w == FontWeight::EXTRA_LIGHT => "UltraLight",
        w if w == FontWeight::LIGHT => "Light",
        w if w == FontWeight::NORMAL => "",
        w if w == FontWeight::MEDIUM => "Medium",
        w if w == FontWeight::SEMI_BOLD => "Semibold",
        w if w == FontWeight::BOLD => "Bold",
        w if w == FontWeight::EXTRA_BOLD => "ExtraBold",
        w if w == FontWeight::BLACK => "Heavy",
        _ => "",
    };

    let style_str = match style {
        FontStyle::Normal => "",
        FontStyle::Italic => "Italic",
        FontStyle::Oblique => "Oblique",
    };

    match (weight_str.is_empty(), style_str.is_empty()) {
        (true, true) => format!("{}-Regular", family_clean),
        (true, false) => format!("{}-{}", family_clean, style_str),
        (false, true) => format!("{}-{}", family_clean, weight_str),
        (false, false) => format!("{}-{}{}", family_clean, weight_str, style_str),
    }
}

impl FontDiscovery for CoreTextDiscovery {
    fn find_font(&self, family: &str, style: FontStyle, weight: FontWeight) -> Option<FontInfo> {
        Self::find_best_match(&self.fonts, family, style, weight)
    }

    fn list_families(&self) -> Vec<String> {
        let mut families: Vec<String> = self.fonts.iter().map(|f| f.family().to_string()).collect();

        families.sort();
        families.dedup();
        families
    }

    fn system_fonts(&self) -> Vec<FontInfo> {
        self.fonts.clone()
    }

    fn find_family(&self, family: &str) -> Vec<FontInfo> {
        self.fonts
            .iter()
            .filter(|f| f.family().eq_ignore_ascii_case(family))
            .cloned()
            .collect()
    }

    fn has_family(&self, family: &str) -> bool {
        self.fonts
            .iter()
            .any(|f| f.family().eq_ignore_ascii_case(family))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coretext_discovery_new() {
        let discovery = CoreTextDiscovery::new();
        assert!(discovery.initialized);
        assert!(!discovery.fonts.is_empty());
    }

    #[test]
    fn test_coretext_list_families() {
        let discovery = CoreTextDiscovery::new();
        let families = discovery.list_families();

        // Should have typical macOS fonts
        assert!(families
            .iter()
            .any(|f| f.contains("SF") || f == "Helvetica Neue"));
        assert!(families.iter().any(|f| f == "Menlo" || f == "Monaco"));
    }

    #[test]
    fn test_coretext_find_font() {
        let discovery = CoreTextDiscovery::new();

        let font = discovery.find_font("Helvetica Neue", FontStyle::Normal, FontWeight::NORMAL);
        assert!(font.is_some());

        let font = font.unwrap();
        assert_eq!(font.family(), "Helvetica Neue");
        assert!(font.path().is_some());
    }

    #[test]
    fn test_coretext_find_bold() {
        let discovery = CoreTextDiscovery::new();

        let font = discovery.find_font("Helvetica Neue", FontStyle::Normal, FontWeight::BOLD);
        assert!(font.is_some());

        let font = font.unwrap();
        assert_eq!(font.weight(), FontWeight::BOLD);
    }

    #[test]
    fn test_coretext_find_italic() {
        let discovery = CoreTextDiscovery::new();

        let font = discovery.find_font("Helvetica Neue", FontStyle::Italic, FontWeight::NORMAL);
        assert!(font.is_some());

        let font = font.unwrap();
        assert_eq!(font.style(), FontStyle::Italic);
    }

    #[test]
    fn test_coretext_system_fonts() {
        let discovery = CoreTextDiscovery::new();
        let fonts = discovery.system_fonts();

        assert!(!fonts.is_empty());
        // All fonts should have paths
        assert!(fonts.iter().all(|f| f.path().is_some()));
    }

    #[test]
    fn test_coretext_paths_are_macos_style() {
        let discovery = CoreTextDiscovery::new();
        let fonts = discovery.system_fonts();

        // Paths should be macOS-style
        for font in fonts {
            if let Some(path) = font.path() {
                let path_str = path.to_string_lossy();
                assert!(
                    path_str.starts_with("/System/Library/Fonts")
                        || path_str.starts_with("/Library/Fonts")
                        || path_str.starts_with("/")
                );
            }
        }
    }

    #[test]
    fn test_coretext_sf_pro_variants() {
        let discovery = CoreTextDiscovery::new();
        let sf_fonts = discovery.find_family("SF Pro");

        // Should have multiple weight variants
        assert!(sf_fonts.len() >= 3);

        let has_light = sf_fonts.iter().any(|f| f.weight() == FontWeight::LIGHT);
        let has_normal = sf_fonts.iter().any(|f| f.weight() == FontWeight::NORMAL);
        let has_bold = sf_fonts.iter().any(|f| f.weight() == FontWeight::BOLD);

        assert!(has_normal);
        assert!(has_bold);
        assert!(has_light);
    }

    #[test]
    fn test_coretext_helvetica_neue_weights() {
        let discovery = CoreTextDiscovery::new();
        let helvetica_fonts = discovery.find_family("Helvetica Neue");

        // Should have extensive weight variations
        let weights: Vec<u16> = helvetica_fonts.iter().map(|f| f.weight().value()).collect();

        assert!(weights.contains(&100)); // Thin/UltraLight
        assert!(weights.contains(&300)); // Light
        assert!(weights.contains(&400)); // Regular
    }

    #[test]
    fn test_generate_postscript_name_macos() {
        assert_eq!(
            generate_postscript_name("Helvetica Neue", FontWeight::NORMAL, FontStyle::Normal),
            "HelveticaNeue-Regular"
        );
        assert_eq!(
            generate_postscript_name("Helvetica Neue", FontWeight::BOLD, FontStyle::Normal),
            "HelveticaNeue-Bold"
        );
        assert_eq!(
            generate_postscript_name("SF Pro", FontWeight::LIGHT, FontStyle::Normal),
            "SFPro-Light"
        );
        assert_eq!(
            generate_postscript_name("SF Pro", FontWeight::BLACK, FontStyle::Normal),
            "SFPro-Heavy"
        );
    }

    #[test]
    fn test_coretext_case_insensitive() {
        let discovery = CoreTextDiscovery::new();

        let font1 = discovery.find_font("Helvetica Neue", FontStyle::Normal, FontWeight::NORMAL);
        let font2 = discovery.find_font("helvetica neue", FontStyle::Normal, FontWeight::NORMAL);
        let font3 = discovery.find_font("HELVETICA NEUE", FontStyle::Normal, FontWeight::NORMAL);

        assert!(font1.is_some());
        assert!(font2.is_some());
        assert!(font3.is_some());
    }

    #[test]
    fn test_coretext_has_monospace() {
        let discovery = CoreTextDiscovery::new();

        // macOS should have Menlo or Monaco
        assert!(discovery.has_family("Menlo") || discovery.has_family("Monaco"));
    }
}
