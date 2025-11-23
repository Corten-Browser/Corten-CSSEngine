//! Fallback Font Discovery Implementation
//!
//! This module provides a fallback font discovery implementation that returns
//! mock/common font data when no platform-specific font API is available.
//!
//! This is used when:
//! - No platform feature is enabled (`fontconfig`, `directwrite`, `coretext`)
//! - Running on an unsupported platform
//! - Testing without actual system font access

use crate::{FontDiscovery, FontInfo, FontStyle, FontWeight};

/// Common font families found across most operating systems
const COMMON_FAMILIES: &[&str] = &[
    // Sans-serif fonts
    "Arial",
    "Helvetica",
    "Verdana",
    "Tahoma",
    "Trebuchet MS",
    "Segoe UI",
    "Roboto",
    "Open Sans",
    "Noto Sans",
    "Liberation Sans",
    "DejaVu Sans",
    // Serif fonts
    "Times New Roman",
    "Georgia",
    "Palatino",
    "Book Antiqua",
    "Cambria",
    "Noto Serif",
    "Liberation Serif",
    "DejaVu Serif",
    // Monospace fonts
    "Courier New",
    "Consolas",
    "Monaco",
    "Menlo",
    "Source Code Pro",
    "Fira Code",
    "JetBrains Mono",
    "Liberation Mono",
    "DejaVu Sans Mono",
    // Generic families
    "sans-serif",
    "serif",
    "monospace",
    "cursive",
    "fantasy",
    "system-ui",
    "ui-sans-serif",
    "ui-serif",
    "ui-monospace",
];

/// Fallback font discovery that returns mock font data
///
/// This implementation doesn't actually query the system for fonts.
/// Instead, it returns a predefined list of common font families that
/// are typically available on most systems.
///
/// Useful for:
/// - Testing without system font access
/// - Providing basic functionality on unsupported platforms
/// - Default fallback when platform detection fails
#[derive(Debug, Clone, Default)]
pub struct FallbackDiscovery {
    /// Cached list of mock fonts
    fonts: Vec<FontInfo>,
}

impl FallbackDiscovery {
    /// Create a new fallback discovery instance
    pub fn new() -> Self {
        let mut discovery = Self { fonts: Vec::new() };
        discovery.populate_fonts();
        discovery
    }

    /// Populate the font list with mock data
    fn populate_fonts(&mut self) {
        self.fonts.clear();

        for family in COMMON_FAMILIES {
            // Add regular variant
            self.fonts.push(
                FontInfo::new(*family, FontStyle::Normal, FontWeight::NORMAL)
                    .with_full_name(format!("{} Regular", family)),
            );

            // Add bold variant
            self.fonts.push(
                FontInfo::new(*family, FontStyle::Normal, FontWeight::BOLD)
                    .with_full_name(format!("{} Bold", family)),
            );

            // Add italic variant
            self.fonts.push(
                FontInfo::new(*family, FontStyle::Italic, FontWeight::NORMAL)
                    .with_full_name(format!("{} Italic", family)),
            );

            // Add bold italic variant
            self.fonts.push(
                FontInfo::new(*family, FontStyle::Italic, FontWeight::BOLD)
                    .with_full_name(format!("{} Bold Italic", family)),
            );

            // Add some weight variations for common fonts
            if *family == "Arial"
                || *family == "Helvetica"
                || *family == "Roboto"
                || *family == "Open Sans"
            {
                // Light
                self.fonts.push(
                    FontInfo::new(*family, FontStyle::Normal, FontWeight::LIGHT)
                        .with_full_name(format!("{} Light", family)),
                );

                // Medium
                self.fonts.push(
                    FontInfo::new(*family, FontStyle::Normal, FontWeight::MEDIUM)
                        .with_full_name(format!("{} Medium", family)),
                );

                // Semi-bold
                self.fonts.push(
                    FontInfo::new(*family, FontStyle::Normal, FontWeight::SEMI_BOLD)
                        .with_full_name(format!("{} Semi Bold", family)),
                );

                // Extra bold
                self.fonts.push(
                    FontInfo::new(*family, FontStyle::Normal, FontWeight::EXTRA_BOLD)
                        .with_full_name(format!("{} Extra Bold", family)),
                );

                // Black
                self.fonts.push(
                    FontInfo::new(*family, FontStyle::Normal, FontWeight::BLACK)
                        .with_full_name(format!("{} Black", family)),
                );
            }
        }

        // Sort fonts for consistent ordering
        self.fonts.sort_by(|a, b| {
            a.family()
                .cmp(b.family())
                .then_with(|| a.weight().value().cmp(&b.weight().value()))
                .then_with(|| format!("{:?}", a.style()).cmp(&format!("{:?}", b.style())))
        });
    }

    /// Find the best matching font from a list of candidates
    fn find_best_match<'a>(
        candidates: impl Iterator<Item = &'a FontInfo>,
        style: FontStyle,
        weight: FontWeight,
    ) -> Option<&'a FontInfo> {
        candidates.min_by_key(|font| font.match_score(style, weight))
    }
}

impl FontDiscovery for FallbackDiscovery {
    fn find_font(&self, family: &str, style: FontStyle, weight: FontWeight) -> Option<FontInfo> {
        // Filter to matching family
        let candidates = self
            .fonts
            .iter()
            .filter(|f| f.family().eq_ignore_ascii_case(family));

        Self::find_best_match(candidates, style, weight).cloned()
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fallback_discovery_new() {
        let discovery = FallbackDiscovery::new();
        assert!(!discovery.fonts.is_empty());
    }

    #[test]
    fn test_fallback_list_families() {
        let discovery = FallbackDiscovery::new();
        let families = discovery.list_families();

        // Should have common fonts
        assert!(families.iter().any(|f| f == "Arial"));
        assert!(families.iter().any(|f| f == "sans-serif"));

        // Should be sorted
        let mut sorted = families.clone();
        sorted.sort();
        assert_eq!(families, sorted);

        // Should be deduplicated
        let mut deduped = families.clone();
        deduped.dedup();
        assert_eq!(families.len(), deduped.len());
    }

    #[test]
    fn test_fallback_find_font_exact_match() {
        let discovery = FallbackDiscovery::new();

        let font = discovery.find_font("Arial", FontStyle::Normal, FontWeight::NORMAL);
        assert!(font.is_some());
        let font = font.unwrap();
        assert_eq!(font.family(), "Arial");
        assert_eq!(font.style(), FontStyle::Normal);
        assert_eq!(font.weight(), FontWeight::NORMAL);
    }

    #[test]
    fn test_fallback_find_font_bold() {
        let discovery = FallbackDiscovery::new();

        let font = discovery.find_font("Arial", FontStyle::Normal, FontWeight::BOLD);
        assert!(font.is_some());
        let font = font.unwrap();
        assert_eq!(font.family(), "Arial");
        assert_eq!(font.weight(), FontWeight::BOLD);
    }

    #[test]
    fn test_fallback_find_font_italic() {
        let discovery = FallbackDiscovery::new();

        let font = discovery.find_font("Times New Roman", FontStyle::Italic, FontWeight::NORMAL);
        assert!(font.is_some());
        let font = font.unwrap();
        assert_eq!(font.style(), FontStyle::Italic);
    }

    #[test]
    fn test_fallback_find_font_case_insensitive() {
        let discovery = FallbackDiscovery::new();

        let font = discovery.find_font("arial", FontStyle::Normal, FontWeight::NORMAL);
        assert!(font.is_some());
        assert_eq!(font.unwrap().family(), "Arial");

        let font = discovery.find_font("ARIAL", FontStyle::Normal, FontWeight::NORMAL);
        assert!(font.is_some());
    }

    #[test]
    fn test_fallback_find_font_not_found() {
        let discovery = FallbackDiscovery::new();

        let font = discovery.find_font(
            "NonExistentFont12345",
            FontStyle::Normal,
            FontWeight::NORMAL,
        );
        assert!(font.is_none());
    }

    #[test]
    fn test_fallback_find_font_weight_fallback() {
        let discovery = FallbackDiscovery::new();

        // Request weight 550 (not a standard weight)
        // Should find closest match
        let font = discovery.find_font("Arial", FontStyle::Normal, FontWeight::clamped(550));
        assert!(font.is_some());
        let font = font.unwrap();
        assert_eq!(font.family(), "Arial");
        // Should get Medium (500) or Semi Bold (600)
        assert!(font.weight().value() >= 500 && font.weight().value() <= 600);
    }

    #[test]
    fn test_fallback_system_fonts() {
        let discovery = FallbackDiscovery::new();
        let fonts = discovery.system_fonts();

        assert!(!fonts.is_empty());
        // Should have multiple variants per family
        let arial_count = fonts.iter().filter(|f| f.family() == "Arial").count();
        assert!(arial_count >= 4); // At least normal, bold, italic, bold-italic
    }

    #[test]
    fn test_fallback_find_family() {
        let discovery = FallbackDiscovery::new();
        let arial_fonts = discovery.find_family("Arial");

        assert!(!arial_fonts.is_empty());
        assert!(arial_fonts.iter().all(|f| f.family() == "Arial"));

        // Should have different styles/weights
        let has_bold = arial_fonts.iter().any(|f| f.weight() == FontWeight::BOLD);
        let has_italic = arial_fonts.iter().any(|f| f.style() == FontStyle::Italic);
        assert!(has_bold);
        assert!(has_italic);
    }

    #[test]
    fn test_fallback_has_family() {
        let discovery = FallbackDiscovery::new();

        assert!(discovery.has_family("Arial"));
        assert!(discovery.has_family("arial")); // Case insensitive
        assert!(discovery.has_family("sans-serif"));
        assert!(!discovery.has_family("NonExistent12345"));
    }

    #[test]
    fn test_fallback_find_with_fallbacks() {
        let discovery = FallbackDiscovery::new();

        // First family doesn't exist, should fall back
        let font = discovery.find_with_fallbacks(
            &["NonExistent", "Arial", "sans-serif"],
            FontStyle::Normal,
            FontWeight::NORMAL,
        );
        assert!(font.is_some());
        assert_eq!(font.unwrap().family(), "Arial");
    }

    #[test]
    fn test_fallback_find_with_all_fallbacks_missing() {
        let discovery = FallbackDiscovery::new();

        let font = discovery.find_with_fallbacks(
            &["NonExistent1", "NonExistent2"],
            FontStyle::Normal,
            FontWeight::NORMAL,
        );
        assert!(font.is_none());
    }

    #[test]
    fn test_fallback_weight_variations() {
        let discovery = FallbackDiscovery::new();
        let arial_fonts = discovery.find_family("Arial");

        // Arial should have weight variations
        let weights: Vec<u16> = arial_fonts.iter().map(|f| f.weight().value()).collect();

        assert!(weights.contains(&100) || weights.contains(&300)); // Light
        assert!(weights.contains(&400)); // Normal
        assert!(weights.contains(&700)); // Bold
    }

    #[test]
    fn test_common_families_coverage() {
        // Verify we have all the common family categories
        assert!(COMMON_FAMILIES.iter().any(|f| *f == "Arial")); // Sans-serif
        assert!(COMMON_FAMILIES.iter().any(|f| *f == "Times New Roman")); // Serif
        assert!(COMMON_FAMILIES.iter().any(|f| *f == "Courier New")); // Monospace
        assert!(COMMON_FAMILIES.iter().any(|f| *f == "sans-serif")); // Generic
    }
}
