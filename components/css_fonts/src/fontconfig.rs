//! Fontconfig Font Discovery Implementation (Linux)
//!
//! This module provides font discovery using the fontconfig library,
//! which is the standard font configuration and customization library
//! on Linux and other Unix-like systems.
//!
//! # Platform Support
//!
//! This implementation is designed for Linux and other systems that use
//! fontconfig (FreeBSD, etc.).
//!
//! # Feature Flag
//!
//! This module is only compiled when the `fontconfig` feature is enabled:
//!
//! ```toml
//! [dependencies]
//! css-fonts = { version = "0.1", features = ["fontconfig"] }
//! ```
//!
//! # Implementation Note
//!
//! This is currently a stub implementation that returns mock data.
//! A full implementation would use the fontconfig-sys crate or similar
//! to interface with the system's fontconfig library.

use crate::{FontDiscovery, FontInfo, FontStretch, FontStyle, FontWeight};
use std::path::PathBuf;

/// Font discovery using fontconfig (Linux)
///
/// This implementation queries the system's fontconfig database for fonts.
/// It provides access to all fonts installed on the system, including:
/// - System fonts (e.g., /usr/share/fonts)
/// - User fonts (e.g., ~/.local/share/fonts, ~/.fonts)
/// - Fonts specified in fontconfig configuration files
///
/// # Example
///
/// ```ignore
/// use css_fonts::fontconfig::FontconfigDiscovery;
/// use css_fonts::{FontDiscovery, FontStyle, FontWeight};
///
/// let discovery = FontconfigDiscovery::new();
/// if let Some(font) = discovery.find_font("DejaVu Sans", FontStyle::Normal, FontWeight::NORMAL) {
///     println!("Found: {} at {:?}", font.family(), font.path());
/// }
/// ```
#[derive(Debug, Clone, Default)]
pub struct FontconfigDiscovery {
    /// Cached font list from fontconfig
    fonts: Vec<FontInfo>,
    /// Whether the cache has been populated
    initialized: bool,
}

impl FontconfigDiscovery {
    /// Create a new fontconfig discovery instance
    ///
    /// This initializes the fontconfig library and prepares for font queries.
    /// The actual font enumeration is done lazily on first query.
    pub fn new() -> Self {
        let mut discovery = Self {
            fonts: Vec::new(),
            initialized: false,
        };
        discovery.initialize();
        discovery
    }

    /// Initialize fontconfig and populate the font cache
    ///
    /// In a real implementation, this would:
    /// 1. Call FcInit() to initialize fontconfig
    /// 2. Create a pattern to match all fonts
    /// 3. Query fontconfig for matching fonts
    /// 4. Parse font metadata from the results
    fn initialize(&mut self) {
        if self.initialized {
            return;
        }

        // Stub implementation: populate with typical Linux fonts
        self.populate_mock_fonts();
        self.initialized = true;
    }

    /// Populate with mock Linux font data
    ///
    /// This represents fonts commonly found on Linux systems.
    /// A real implementation would query fontconfig directly.
    fn populate_mock_fonts(&mut self) {
        // Common Linux font families
        let linux_fonts = [
            // DejaVu family (very common on Linux)
            (
                "DejaVu Sans",
                "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
            ),
            (
                "DejaVu Sans",
                "/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf",
            ),
            (
                "DejaVu Sans",
                "/usr/share/fonts/truetype/dejavu/DejaVuSans-Oblique.ttf",
            ),
            (
                "DejaVu Sans",
                "/usr/share/fonts/truetype/dejavu/DejaVuSans-BoldOblique.ttf",
            ),
            (
                "DejaVu Serif",
                "/usr/share/fonts/truetype/dejavu/DejaVuSerif.ttf",
            ),
            (
                "DejaVu Serif",
                "/usr/share/fonts/truetype/dejavu/DejaVuSerif-Bold.ttf",
            ),
            (
                "DejaVu Sans Mono",
                "/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf",
            ),
            (
                "DejaVu Sans Mono",
                "/usr/share/fonts/truetype/dejavu/DejaVuSansMono-Bold.ttf",
            ),
            // Liberation family (metrically compatible with Microsoft fonts)
            (
                "Liberation Sans",
                "/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf",
            ),
            (
                "Liberation Sans",
                "/usr/share/fonts/truetype/liberation/LiberationSans-Bold.ttf",
            ),
            (
                "Liberation Sans",
                "/usr/share/fonts/truetype/liberation/LiberationSans-Italic.ttf",
            ),
            (
                "Liberation Sans",
                "/usr/share/fonts/truetype/liberation/LiberationSans-BoldItalic.ttf",
            ),
            (
                "Liberation Serif",
                "/usr/share/fonts/truetype/liberation/LiberationSerif-Regular.ttf",
            ),
            (
                "Liberation Serif",
                "/usr/share/fonts/truetype/liberation/LiberationSerif-Bold.ttf",
            ),
            (
                "Liberation Mono",
                "/usr/share/fonts/truetype/liberation/LiberationMono-Regular.ttf",
            ),
            (
                "Liberation Mono",
                "/usr/share/fonts/truetype/liberation/LiberationMono-Bold.ttf",
            ),
            // Noto family (Google's universal font)
            (
                "Noto Sans",
                "/usr/share/fonts/truetype/noto/NotoSans-Regular.ttf",
            ),
            (
                "Noto Sans",
                "/usr/share/fonts/truetype/noto/NotoSans-Bold.ttf",
            ),
            (
                "Noto Sans",
                "/usr/share/fonts/truetype/noto/NotoSans-Italic.ttf",
            ),
            (
                "Noto Serif",
                "/usr/share/fonts/truetype/noto/NotoSerif-Regular.ttf",
            ),
            (
                "Noto Serif",
                "/usr/share/fonts/truetype/noto/NotoSerif-Bold.ttf",
            ),
            (
                "Noto Sans Mono",
                "/usr/share/fonts/truetype/noto/NotoSansMono-Regular.ttf",
            ),
            // Ubuntu fonts
            ("Ubuntu", "/usr/share/fonts/truetype/ubuntu/Ubuntu-R.ttf"),
            ("Ubuntu", "/usr/share/fonts/truetype/ubuntu/Ubuntu-B.ttf"),
            ("Ubuntu", "/usr/share/fonts/truetype/ubuntu/Ubuntu-RI.ttf"),
            ("Ubuntu", "/usr/share/fonts/truetype/ubuntu/Ubuntu-BI.ttf"),
            (
                "Ubuntu Mono",
                "/usr/share/fonts/truetype/ubuntu/UbuntuMono-R.ttf",
            ),
            (
                "Ubuntu Mono",
                "/usr/share/fonts/truetype/ubuntu/UbuntuMono-B.ttf",
            ),
            // Droid fonts (common on Android and some Linux distros)
            (
                "Droid Sans",
                "/usr/share/fonts/truetype/droid/DroidSans.ttf",
            ),
            (
                "Droid Sans",
                "/usr/share/fonts/truetype/droid/DroidSans-Bold.ttf",
            ),
            (
                "Droid Serif",
                "/usr/share/fonts/truetype/droid/DroidSerif-Regular.ttf",
            ),
            (
                "Droid Sans Mono",
                "/usr/share/fonts/truetype/droid/DroidSansMono.ttf",
            ),
            // FreeSerif/FreeSans/FreeMono
            (
                "FreeSans",
                "/usr/share/fonts/truetype/freefont/FreeSans.ttf",
            ),
            (
                "FreeSerif",
                "/usr/share/fonts/truetype/freefont/FreeSerif.ttf",
            ),
            (
                "FreeMono",
                "/usr/share/fonts/truetype/freefont/FreeMono.ttf",
            ),
        ];

        for (family, path) in linux_fonts.iter() {
            // Infer style and weight from path
            let (style, weight) = infer_style_weight_from_path(path);

            self.fonts.push(
                FontInfo::new(*family, style, weight)
                    .with_path(PathBuf::from(*path))
                    .with_postscript_name(format!(
                        "{}-{}",
                        family.replace(' ', ""),
                        weight_style_suffix(weight, style)
                    )),
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

/// Infer font style and weight from file path
fn infer_style_weight_from_path(path: &str) -> (FontStyle, FontWeight) {
    let path_lower = path.to_lowercase();

    let style = if path_lower.contains("italic")
        || path_lower.contains("-ri")
        || path_lower.contains("-bi")
    {
        FontStyle::Italic
    } else if path_lower.contains("oblique") {
        FontStyle::Oblique
    } else {
        FontStyle::Normal
    };

    let weight =
        if path_lower.contains("bold") || path_lower.contains("-b.") || path_lower.contains("-bi.")
        {
            FontWeight::BOLD
        } else if path_lower.contains("light") {
            FontWeight::LIGHT
        } else if path_lower.contains("medium") {
            FontWeight::MEDIUM
        } else if path_lower.contains("black") || path_lower.contains("heavy") {
            FontWeight::BLACK
        } else if path_lower.contains("thin") {
            FontWeight::THIN
        } else {
            FontWeight::NORMAL
        };

    (style, weight)
}

/// Generate suffix string for PostScript name
fn weight_style_suffix(weight: FontWeight, style: FontStyle) -> String {
    let weight_str = match weight {
        w if w == FontWeight::THIN => "Thin",
        w if w == FontWeight::LIGHT => "Light",
        w if w == FontWeight::NORMAL => "Regular",
        w if w == FontWeight::MEDIUM => "Medium",
        w if w == FontWeight::SEMI_BOLD => "SemiBold",
        w if w == FontWeight::BOLD => "Bold",
        w if w == FontWeight::EXTRA_BOLD => "ExtraBold",
        w if w == FontWeight::BLACK => "Black",
        _ => "Regular",
    };

    let style_str = match style {
        FontStyle::Normal => "",
        FontStyle::Italic => "Italic",
        FontStyle::Oblique => "Oblique",
    };

    if style_str.is_empty() {
        weight_str.to_string()
    } else {
        format!("{}{}", weight_str, style_str)
    }
}

impl FontDiscovery for FontconfigDiscovery {
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
    fn test_fontconfig_discovery_new() {
        let discovery = FontconfigDiscovery::new();
        assert!(discovery.initialized);
        assert!(!discovery.fonts.is_empty());
    }

    #[test]
    fn test_fontconfig_list_families() {
        let discovery = FontconfigDiscovery::new();
        let families = discovery.list_families();

        // Should have typical Linux fonts
        assert!(families.iter().any(|f| f.contains("DejaVu")));
        assert!(families.iter().any(|f| f.contains("Liberation")));
    }

    #[test]
    fn test_fontconfig_find_font() {
        let discovery = FontconfigDiscovery::new();

        let font = discovery.find_font("DejaVu Sans", FontStyle::Normal, FontWeight::NORMAL);
        assert!(font.is_some());

        let font = font.unwrap();
        assert_eq!(font.family(), "DejaVu Sans");
        assert!(font.path().is_some());
    }

    #[test]
    fn test_fontconfig_find_bold() {
        let discovery = FontconfigDiscovery::new();

        let font = discovery.find_font("DejaVu Sans", FontStyle::Normal, FontWeight::BOLD);
        assert!(font.is_some());

        let font = font.unwrap();
        assert_eq!(font.weight(), FontWeight::BOLD);
    }

    #[test]
    fn test_fontconfig_system_fonts() {
        let discovery = FontconfigDiscovery::new();
        let fonts = discovery.system_fonts();

        assert!(!fonts.is_empty());
        // All fonts should have paths
        assert!(fonts.iter().all(|f| f.path().is_some()));
    }

    #[test]
    fn test_fontconfig_paths_are_linux_style() {
        let discovery = FontconfigDiscovery::new();
        let fonts = discovery.system_fonts();

        // Paths should be Unix-style
        for font in fonts {
            if let Some(path) = font.path() {
                let path_str = path.to_string_lossy();
                assert!(path_str.starts_with("/usr/share/fonts") || path_str.starts_with("/home"));
            }
        }
    }

    #[test]
    fn test_infer_style_weight_from_path() {
        let (style, weight) = infer_style_weight_from_path("/fonts/Arial-Bold.ttf");
        assert_eq!(style, FontStyle::Normal);
        assert_eq!(weight, FontWeight::BOLD);

        let (style, weight) = infer_style_weight_from_path("/fonts/Arial-Italic.ttf");
        assert_eq!(style, FontStyle::Italic);
        assert_eq!(weight, FontWeight::NORMAL);

        let (style, weight) = infer_style_weight_from_path("/fonts/Arial-BoldItalic.ttf");
        assert_eq!(style, FontStyle::Italic);
        assert_eq!(weight, FontWeight::BOLD);
    }

    #[test]
    fn test_weight_style_suffix() {
        assert_eq!(
            weight_style_suffix(FontWeight::NORMAL, FontStyle::Normal),
            "Regular"
        );
        assert_eq!(
            weight_style_suffix(FontWeight::BOLD, FontStyle::Normal),
            "Bold"
        );
        assert_eq!(
            weight_style_suffix(FontWeight::NORMAL, FontStyle::Italic),
            "RegularItalic"
        );
        assert_eq!(
            weight_style_suffix(FontWeight::BOLD, FontStyle::Italic),
            "BoldItalic"
        );
    }
}
