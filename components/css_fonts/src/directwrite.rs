//! DirectWrite Font Discovery Implementation (Windows)
//!
//! This module provides font discovery using Microsoft's DirectWrite API,
//! which is the modern text rendering and font management API on Windows.
//!
//! # Platform Support
//!
//! This implementation is designed for Windows Vista and later versions.
//! DirectWrite provides access to:
//! - System fonts (C:\Windows\Fonts)
//! - User fonts (per-user font installations)
//! - Font collections and families
//! - OpenType font features
//!
//! # Feature Flag
//!
//! This module is only compiled when the `directwrite` feature is enabled:
//!
//! ```toml
//! [dependencies]
//! css-fonts = { version = "0.1", features = ["directwrite"] }
//! ```
//!
//! # Implementation Note
//!
//! This is currently a stub implementation that returns mock data.
//! A full implementation would use the windows-rs crate to interface
//! with the DirectWrite API (IDWriteFactory, IDWriteFontCollection, etc.).

use crate::{FontDiscovery, FontInfo, FontStretch, FontStyle, FontWeight};
use std::path::PathBuf;

/// Font discovery using DirectWrite (Windows)
///
/// This implementation uses Microsoft's DirectWrite API to enumerate
/// and query system fonts on Windows.
///
/// # Example
///
/// ```ignore
/// use css_fonts::directwrite::DirectWriteDiscovery;
/// use css_fonts::{FontDiscovery, FontStyle, FontWeight};
///
/// let discovery = DirectWriteDiscovery::new();
/// if let Some(font) = discovery.find_font("Segoe UI", FontStyle::Normal, FontWeight::NORMAL) {
///     println!("Found: {} at {:?}", font.family(), font.path());
/// }
/// ```
#[derive(Debug, Clone, Default)]
pub struct DirectWriteDiscovery {
    /// Cached font list from DirectWrite
    fonts: Vec<FontInfo>,
    /// Whether the cache has been populated
    initialized: bool,
}

impl DirectWriteDiscovery {
    /// Create a new DirectWrite discovery instance
    ///
    /// This initializes the DirectWrite factory and prepares for font queries.
    /// The actual font enumeration is done lazily on first query.
    pub fn new() -> Self {
        let mut discovery = Self {
            fonts: Vec::new(),
            initialized: false,
        };
        discovery.initialize();
        discovery
    }

    /// Initialize DirectWrite and populate the font cache
    ///
    /// In a real implementation, this would:
    /// 1. Create an IDWriteFactory instance
    /// 2. Get the system font collection
    /// 3. Enumerate all font families
    /// 4. For each family, enumerate font faces
    /// 5. Extract metadata (weight, style, stretch, path)
    fn initialize(&mut self) {
        if self.initialized {
            return;
        }

        // Stub implementation: populate with typical Windows fonts
        self.populate_mock_fonts();
        self.initialized = true;
    }

    /// Populate with mock Windows font data
    ///
    /// This represents fonts commonly found on Windows systems.
    /// A real implementation would query DirectWrite directly.
    fn populate_mock_fonts(&mut self) {
        // Common Windows font families with their typical locations
        let windows_fonts = [
            // Segoe UI family (Windows system font since Vista)
            (
                "Segoe UI",
                "C:\\Windows\\Fonts\\segoeui.ttf",
                FontWeight::NORMAL,
                FontStyle::Normal,
            ),
            (
                "Segoe UI",
                "C:\\Windows\\Fonts\\segoeuib.ttf",
                FontWeight::BOLD,
                FontStyle::Normal,
            ),
            (
                "Segoe UI",
                "C:\\Windows\\Fonts\\segoeuii.ttf",
                FontWeight::NORMAL,
                FontStyle::Italic,
            ),
            (
                "Segoe UI",
                "C:\\Windows\\Fonts\\segoeuiz.ttf",
                FontWeight::BOLD,
                FontStyle::Italic,
            ),
            (
                "Segoe UI",
                "C:\\Windows\\Fonts\\segoeuil.ttf",
                FontWeight::LIGHT,
                FontStyle::Normal,
            ),
            (
                "Segoe UI",
                "C:\\Windows\\Fonts\\seguisb.ttf",
                FontWeight::SEMI_BOLD,
                FontStyle::Normal,
            ),
            (
                "Segoe UI",
                "C:\\Windows\\Fonts\\seguibl.ttf",
                FontWeight::BLACK,
                FontStyle::Normal,
            ),
            // Arial family
            (
                "Arial",
                "C:\\Windows\\Fonts\\arial.ttf",
                FontWeight::NORMAL,
                FontStyle::Normal,
            ),
            (
                "Arial",
                "C:\\Windows\\Fonts\\arialbd.ttf",
                FontWeight::BOLD,
                FontStyle::Normal,
            ),
            (
                "Arial",
                "C:\\Windows\\Fonts\\ariali.ttf",
                FontWeight::NORMAL,
                FontStyle::Italic,
            ),
            (
                "Arial",
                "C:\\Windows\\Fonts\\arialbi.ttf",
                FontWeight::BOLD,
                FontStyle::Italic,
            ),
            (
                "Arial Black",
                "C:\\Windows\\Fonts\\ariblk.ttf",
                FontWeight::BLACK,
                FontStyle::Normal,
            ),
            // Times New Roman family
            (
                "Times New Roman",
                "C:\\Windows\\Fonts\\times.ttf",
                FontWeight::NORMAL,
                FontStyle::Normal,
            ),
            (
                "Times New Roman",
                "C:\\Windows\\Fonts\\timesbd.ttf",
                FontWeight::BOLD,
                FontStyle::Normal,
            ),
            (
                "Times New Roman",
                "C:\\Windows\\Fonts\\timesi.ttf",
                FontWeight::NORMAL,
                FontStyle::Italic,
            ),
            (
                "Times New Roman",
                "C:\\Windows\\Fonts\\timesbi.ttf",
                FontWeight::BOLD,
                FontStyle::Italic,
            ),
            // Verdana family
            (
                "Verdana",
                "C:\\Windows\\Fonts\\verdana.ttf",
                FontWeight::NORMAL,
                FontStyle::Normal,
            ),
            (
                "Verdana",
                "C:\\Windows\\Fonts\\verdanab.ttf",
                FontWeight::BOLD,
                FontStyle::Normal,
            ),
            (
                "Verdana",
                "C:\\Windows\\Fonts\\verdanai.ttf",
                FontWeight::NORMAL,
                FontStyle::Italic,
            ),
            (
                "Verdana",
                "C:\\Windows\\Fonts\\verdanaz.ttf",
                FontWeight::BOLD,
                FontStyle::Italic,
            ),
            // Georgia family
            (
                "Georgia",
                "C:\\Windows\\Fonts\\georgia.ttf",
                FontWeight::NORMAL,
                FontStyle::Normal,
            ),
            (
                "Georgia",
                "C:\\Windows\\Fonts\\georgiab.ttf",
                FontWeight::BOLD,
                FontStyle::Normal,
            ),
            (
                "Georgia",
                "C:\\Windows\\Fonts\\georgiai.ttf",
                FontWeight::NORMAL,
                FontStyle::Italic,
            ),
            (
                "Georgia",
                "C:\\Windows\\Fonts\\georgiaz.ttf",
                FontWeight::BOLD,
                FontStyle::Italic,
            ),
            // Tahoma
            (
                "Tahoma",
                "C:\\Windows\\Fonts\\tahoma.ttf",
                FontWeight::NORMAL,
                FontStyle::Normal,
            ),
            (
                "Tahoma",
                "C:\\Windows\\Fonts\\tahomabd.ttf",
                FontWeight::BOLD,
                FontStyle::Normal,
            ),
            // Trebuchet MS
            (
                "Trebuchet MS",
                "C:\\Windows\\Fonts\\trebuc.ttf",
                FontWeight::NORMAL,
                FontStyle::Normal,
            ),
            (
                "Trebuchet MS",
                "C:\\Windows\\Fonts\\trebucbd.ttf",
                FontWeight::BOLD,
                FontStyle::Normal,
            ),
            (
                "Trebuchet MS",
                "C:\\Windows\\Fonts\\trebucit.ttf",
                FontWeight::NORMAL,
                FontStyle::Italic,
            ),
            (
                "Trebuchet MS",
                "C:\\Windows\\Fonts\\trebucbi.ttf",
                FontWeight::BOLD,
                FontStyle::Italic,
            ),
            // Calibri family (Office fonts)
            (
                "Calibri",
                "C:\\Windows\\Fonts\\calibri.ttf",
                FontWeight::NORMAL,
                FontStyle::Normal,
            ),
            (
                "Calibri",
                "C:\\Windows\\Fonts\\calibrib.ttf",
                FontWeight::BOLD,
                FontStyle::Normal,
            ),
            (
                "Calibri",
                "C:\\Windows\\Fonts\\calibrii.ttf",
                FontWeight::NORMAL,
                FontStyle::Italic,
            ),
            (
                "Calibri",
                "C:\\Windows\\Fonts\\calibriz.ttf",
                FontWeight::BOLD,
                FontStyle::Italic,
            ),
            (
                "Calibri",
                "C:\\Windows\\Fonts\\calibril.ttf",
                FontWeight::LIGHT,
                FontStyle::Normal,
            ),
            // Cambria family
            (
                "Cambria",
                "C:\\Windows\\Fonts\\cambria.ttc",
                FontWeight::NORMAL,
                FontStyle::Normal,
            ),
            (
                "Cambria",
                "C:\\Windows\\Fonts\\cambriab.ttf",
                FontWeight::BOLD,
                FontStyle::Normal,
            ),
            (
                "Cambria",
                "C:\\Windows\\Fonts\\cambriai.ttf",
                FontWeight::NORMAL,
                FontStyle::Italic,
            ),
            (
                "Cambria",
                "C:\\Windows\\Fonts\\cambriaz.ttf",
                FontWeight::BOLD,
                FontStyle::Italic,
            ),
            // Consolas (monospace)
            (
                "Consolas",
                "C:\\Windows\\Fonts\\consola.ttf",
                FontWeight::NORMAL,
                FontStyle::Normal,
            ),
            (
                "Consolas",
                "C:\\Windows\\Fonts\\consolab.ttf",
                FontWeight::BOLD,
                FontStyle::Normal,
            ),
            (
                "Consolas",
                "C:\\Windows\\Fonts\\consolai.ttf",
                FontWeight::NORMAL,
                FontStyle::Italic,
            ),
            (
                "Consolas",
                "C:\\Windows\\Fonts\\consolaz.ttf",
                FontWeight::BOLD,
                FontStyle::Italic,
            ),
            // Courier New
            (
                "Courier New",
                "C:\\Windows\\Fonts\\cour.ttf",
                FontWeight::NORMAL,
                FontStyle::Normal,
            ),
            (
                "Courier New",
                "C:\\Windows\\Fonts\\courbd.ttf",
                FontWeight::BOLD,
                FontStyle::Normal,
            ),
            (
                "Courier New",
                "C:\\Windows\\Fonts\\couri.ttf",
                FontWeight::NORMAL,
                FontStyle::Italic,
            ),
            (
                "Courier New",
                "C:\\Windows\\Fonts\\courbi.ttf",
                FontWeight::BOLD,
                FontStyle::Italic,
            ),
            // Lucida Console
            (
                "Lucida Console",
                "C:\\Windows\\Fonts\\lucon.ttf",
                FontWeight::NORMAL,
                FontStyle::Normal,
            ),
            // Impact
            (
                "Impact",
                "C:\\Windows\\Fonts\\impact.ttf",
                FontWeight::NORMAL,
                FontStyle::Normal,
            ),
            // Comic Sans MS
            (
                "Comic Sans MS",
                "C:\\Windows\\Fonts\\comic.ttf",
                FontWeight::NORMAL,
                FontStyle::Normal,
            ),
            (
                "Comic Sans MS",
                "C:\\Windows\\Fonts\\comicbd.ttf",
                FontWeight::BOLD,
                FontStyle::Normal,
            ),
            // Cascadia Code (Windows Terminal font)
            (
                "Cascadia Code",
                "C:\\Windows\\Fonts\\CascadiaCode.ttf",
                FontWeight::NORMAL,
                FontStyle::Normal,
            ),
            (
                "Cascadia Mono",
                "C:\\Windows\\Fonts\\CascadiaMono.ttf",
                FontWeight::NORMAL,
                FontStyle::Normal,
            ),
        ];

        for (family, path, weight, style) in windows_fonts.iter() {
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
        w if w == FontWeight::EXTRA_LIGHT => "ExtraLight",
        w if w == FontWeight::LIGHT => "Light",
        w if w == FontWeight::NORMAL => "",
        w if w == FontWeight::MEDIUM => "Medium",
        w if w == FontWeight::SEMI_BOLD => "SemiBold",
        w if w == FontWeight::BOLD => "Bold",
        w if w == FontWeight::EXTRA_BOLD => "ExtraBold",
        w if w == FontWeight::BLACK => "Black",
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

impl FontDiscovery for DirectWriteDiscovery {
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
    fn test_directwrite_discovery_new() {
        let discovery = DirectWriteDiscovery::new();
        assert!(discovery.initialized);
        assert!(!discovery.fonts.is_empty());
    }

    #[test]
    fn test_directwrite_list_families() {
        let discovery = DirectWriteDiscovery::new();
        let families = discovery.list_families();

        // Should have typical Windows fonts
        assert!(families.iter().any(|f| f == "Segoe UI"));
        assert!(families.iter().any(|f| f == "Arial"));
        assert!(families.iter().any(|f| f == "Times New Roman"));
    }

    #[test]
    fn test_directwrite_find_font() {
        let discovery = DirectWriteDiscovery::new();

        let font = discovery.find_font("Segoe UI", FontStyle::Normal, FontWeight::NORMAL);
        assert!(font.is_some());

        let font = font.unwrap();
        assert_eq!(font.family(), "Segoe UI");
        assert!(font.path().is_some());
    }

    #[test]
    fn test_directwrite_find_bold() {
        let discovery = DirectWriteDiscovery::new();

        let font = discovery.find_font("Arial", FontStyle::Normal, FontWeight::BOLD);
        assert!(font.is_some());

        let font = font.unwrap();
        assert_eq!(font.weight(), FontWeight::BOLD);
    }

    #[test]
    fn test_directwrite_find_italic() {
        let discovery = DirectWriteDiscovery::new();

        let font = discovery.find_font("Times New Roman", FontStyle::Italic, FontWeight::NORMAL);
        assert!(font.is_some());

        let font = font.unwrap();
        assert_eq!(font.style(), FontStyle::Italic);
    }

    #[test]
    fn test_directwrite_system_fonts() {
        let discovery = DirectWriteDiscovery::new();
        let fonts = discovery.system_fonts();

        assert!(!fonts.is_empty());
        // All fonts should have paths
        assert!(fonts.iter().all(|f| f.path().is_some()));
    }

    #[test]
    fn test_directwrite_paths_are_windows_style() {
        let discovery = DirectWriteDiscovery::new();
        let fonts = discovery.system_fonts();

        // Paths should be Windows-style
        for font in fonts {
            if let Some(path) = font.path() {
                let path_str = path.to_string_lossy();
                assert!(path_str.contains("\\Windows\\Fonts") || path_str.starts_with("C:\\"));
            }
        }
    }

    #[test]
    fn test_directwrite_segoe_ui_variants() {
        let discovery = DirectWriteDiscovery::new();
        let segoe_fonts = discovery.find_family("Segoe UI");

        // Should have multiple variants
        assert!(segoe_fonts.len() >= 4);

        // Should have weight variations
        let has_light = segoe_fonts.iter().any(|f| f.weight() == FontWeight::LIGHT);
        let has_normal = segoe_fonts.iter().any(|f| f.weight() == FontWeight::NORMAL);
        let has_bold = segoe_fonts.iter().any(|f| f.weight() == FontWeight::BOLD);

        assert!(has_normal);
        assert!(has_bold);
        assert!(has_light);
    }

    #[test]
    fn test_generate_postscript_name() {
        assert_eq!(
            generate_postscript_name("Arial", FontWeight::NORMAL, FontStyle::Normal),
            "Arial-Regular"
        );
        assert_eq!(
            generate_postscript_name("Arial", FontWeight::BOLD, FontStyle::Normal),
            "Arial-Bold"
        );
        assert_eq!(
            generate_postscript_name("Arial", FontWeight::NORMAL, FontStyle::Italic),
            "Arial-Italic"
        );
        assert_eq!(
            generate_postscript_name("Arial", FontWeight::BOLD, FontStyle::Italic),
            "Arial-BoldItalic"
        );
        assert_eq!(
            generate_postscript_name("Segoe UI", FontWeight::LIGHT, FontStyle::Normal),
            "SegoeUI-Light"
        );
    }

    #[test]
    fn test_directwrite_case_insensitive() {
        let discovery = DirectWriteDiscovery::new();

        let font1 = discovery.find_font("Arial", FontStyle::Normal, FontWeight::NORMAL);
        let font2 = discovery.find_font("arial", FontStyle::Normal, FontWeight::NORMAL);
        let font3 = discovery.find_font("ARIAL", FontStyle::Normal, FontWeight::NORMAL);

        assert!(font1.is_some());
        assert!(font2.is_some());
        assert!(font3.is_some());
    }
}
