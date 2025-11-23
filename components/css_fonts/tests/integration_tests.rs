//! Integration tests for css-fonts crate
//!
//! These tests verify the overall functionality of the font discovery system
//! across all platform implementations.

use css_fonts::{
    get_font_discovery, FallbackDiscovery, FontDiscovery, FontError, FontInfo, FontStretch,
    FontStyle, FontWeight,
};

// ============================================================================
// Factory Function Tests
// ============================================================================

#[test]
fn test_get_font_discovery_returns_valid_implementation() {
    let discovery = get_font_discovery();

    // Should be able to list families
    let families = discovery.list_families();
    assert!(
        !families.is_empty(),
        "Discovery should return at least some font families"
    );

    // Should be able to get system fonts
    let fonts = discovery.system_fonts();
    assert!(
        !fonts.is_empty(),
        "Discovery should return at least some system fonts"
    );
}

#[test]
fn test_factory_returns_thread_safe_implementation() {
    // FontDiscovery is Send + Sync, so we should be able to use it across threads
    let discovery = get_font_discovery();

    std::thread::scope(|s| {
        let handle = s.spawn(|| discovery.list_families());

        let families = handle.join().expect("Thread should complete");
        assert!(!families.is_empty());
    });
}

// ============================================================================
// FontWeight Tests
// ============================================================================

#[test]
fn test_font_weight_full_range() {
    // Test all named weights
    let weights = [
        (FontWeight::THIN, 100),
        (FontWeight::EXTRA_LIGHT, 200),
        (FontWeight::LIGHT, 300),
        (FontWeight::NORMAL, 400),
        (FontWeight::MEDIUM, 500),
        (FontWeight::SEMI_BOLD, 600),
        (FontWeight::BOLD, 700),
        (FontWeight::EXTRA_BOLD, 800),
        (FontWeight::BLACK, 900),
    ];

    for (weight, expected) in weights {
        assert_eq!(
            weight.value(),
            expected,
            "Weight constant should have expected value"
        );
    }
}

#[test]
fn test_font_weight_parse_all_named_variants() {
    // Standard named weights
    let name_pairs = [
        ("thin", 100),
        ("hairline", 100),
        ("extra-light", 200),
        ("extralight", 200),
        ("ultra-light", 200),
        ("ultralight", 200),
        ("light", 300),
        ("normal", 400),
        ("regular", 400),
        ("medium", 500),
        ("semi-bold", 600),
        ("semibold", 600),
        ("demi-bold", 600),
        ("demibold", 600),
        ("bold", 700),
        ("extra-bold", 800),
        ("extrabold", 800),
        ("ultra-bold", 800),
        ("ultrabold", 800),
        ("black", 900),
        ("heavy", 900),
    ];

    for (name, expected) in name_pairs {
        let weight = FontWeight::parse(name);
        assert!(weight.is_some(), "Should parse '{}'", name);
        assert_eq!(
            weight.unwrap().value(),
            expected,
            "Weight '{}' should be {}",
            name,
            expected
        );
    }
}

#[test]
fn test_font_weight_boundaries() {
    // Test boundary conditions
    assert!(FontWeight::new(1).is_ok(), "Weight 1 should be valid");
    assert!(FontWeight::new(1000).is_ok(), "Weight 1000 should be valid");
    assert!(FontWeight::new(0).is_err(), "Weight 0 should be invalid");
    assert!(
        FontWeight::new(1001).is_err(),
        "Weight 1001 should be invalid"
    );

    // Clamped should handle out-of-range
    assert_eq!(FontWeight::clamped(0).value(), 1);
    assert_eq!(FontWeight::clamped(9999).value(), 1000);
}

#[test]
fn test_font_weight_distance_calculation() {
    let w100 = FontWeight::THIN;
    let w400 = FontWeight::NORMAL;
    let w700 = FontWeight::BOLD;

    assert_eq!(w100.distance(&w400), 300);
    assert_eq!(w400.distance(&w700), 300);
    assert_eq!(w100.distance(&w700), 600);

    // Distance should be symmetric
    assert_eq!(w400.distance(&w100), w100.distance(&w400));
}

#[test]
fn test_font_weight_standard_rounding() {
    // Test rounding to standard weights
    assert_eq!(FontWeight::clamped(350).to_standard(), FontWeight::NORMAL);
    assert_eq!(FontWeight::clamped(450).to_standard(), FontWeight::MEDIUM);
    assert_eq!(
        FontWeight::clamped(550).to_standard(),
        FontWeight::SEMI_BOLD
    );
    assert_eq!(FontWeight::clamped(650).to_standard(), FontWeight::BOLD);
}

// ============================================================================
// FontStyle Tests
// ============================================================================

#[test]
fn test_font_style_default_is_normal() {
    assert_eq!(FontStyle::default(), FontStyle::Normal);
}

#[test]
fn test_font_style_css_roundtrip() {
    for style in [FontStyle::Normal, FontStyle::Italic, FontStyle::Oblique] {
        let css = style.to_css();
        let parsed = FontStyle::parse(css);
        assert_eq!(parsed, Some(style), "CSS roundtrip should preserve style");
    }
}

#[test]
fn test_font_style_case_insensitive_parsing() {
    assert_eq!(FontStyle::parse("NORMAL"), Some(FontStyle::Normal));
    assert_eq!(FontStyle::parse("Italic"), Some(FontStyle::Italic));
    assert_eq!(FontStyle::parse("ObLiQuE"), Some(FontStyle::Oblique));
}

// ============================================================================
// FontStretch Tests
// ============================================================================

#[test]
fn test_font_stretch_percentages() {
    let stretches = [
        (FontStretch::UltraCondensed, 50.0),
        (FontStretch::ExtraCondensed, 62.5),
        (FontStretch::Condensed, 75.0),
        (FontStretch::SemiCondensed, 87.5),
        (FontStretch::Normal, 100.0),
        (FontStretch::SemiExpanded, 112.5),
        (FontStretch::Expanded, 125.0),
        (FontStretch::ExtraExpanded, 150.0),
        (FontStretch::UltraExpanded, 200.0),
    ];

    for (stretch, expected_pct) in stretches {
        assert_eq!(
            stretch.percentage(),
            expected_pct,
            "FontStretch::{:?} should have percentage {}",
            stretch,
            expected_pct
        );
    }
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
fn test_font_stretch_parse_variants() {
    let variants = [
        ("ultra-condensed", FontStretch::UltraCondensed),
        ("ultracondensed", FontStretch::UltraCondensed),
        ("condensed", FontStretch::Condensed),
        ("normal", FontStretch::Normal),
        ("expanded", FontStretch::Expanded),
        ("ultra-expanded", FontStretch::UltraExpanded),
        ("ultraexpanded", FontStretch::UltraExpanded),
    ];

    for (name, expected) in variants {
        assert_eq!(
            FontStretch::parse(name),
            Some(expected),
            "Should parse '{}' as {:?}",
            name,
            expected
        );
    }
}

// ============================================================================
// FontInfo Tests
// ============================================================================

#[test]
fn test_font_info_builder_pattern() {
    let font = FontInfo::new("Test Family", FontStyle::Italic, FontWeight::BOLD)
        .with_postscript_name("TestFamily-BoldItalic")
        .with_full_name("Test Family Bold Italic")
        .with_stretch(FontStretch::Condensed)
        .with_path("/path/to/font.ttf")
        .with_index(2)
        .with_variable(true);

    assert_eq!(font.family(), "Test Family");
    assert_eq!(font.style(), FontStyle::Italic);
    assert_eq!(font.weight(), FontWeight::BOLD);
    assert_eq!(font.postscript_name(), Some("TestFamily-BoldItalic"));
    assert_eq!(font.full_name(), Some("Test Family Bold Italic"));
    assert_eq!(font.stretch(), FontStretch::Condensed);
    assert_eq!(font.index(), 2);
    assert!(font.is_variable());
}

#[test]
fn test_font_info_match_score_perfect_match() {
    let font = FontInfo::new("Arial", FontStyle::Normal, FontWeight::NORMAL);
    let score = font.match_score(FontStyle::Normal, FontWeight::NORMAL);
    assert_eq!(score, 0, "Perfect match should have score 0");
}

#[test]
fn test_font_info_match_score_weight_only_difference() {
    let font = FontInfo::new("Arial", FontStyle::Normal, FontWeight::NORMAL);

    // Weight distance should be the score for same style
    let bold_score = font.match_score(FontStyle::Normal, FontWeight::BOLD);
    assert_eq!(bold_score, 300, "Weight 400 vs 700 = 300 distance");

    let light_score = font.match_score(FontStyle::Normal, FontWeight::LIGHT);
    assert_eq!(light_score, 100, "Weight 400 vs 300 = 100 distance");
}

#[test]
fn test_font_info_match_score_style_penalty() {
    let normal_font = FontInfo::new("Arial", FontStyle::Normal, FontWeight::NORMAL);
    let italic_font = FontInfo::new("Arial", FontStyle::Italic, FontWeight::NORMAL);

    // Style mismatch should have high penalty
    let normal_italic_score = normal_font.match_score(FontStyle::Italic, FontWeight::NORMAL);
    assert!(
        normal_italic_score >= 1000,
        "Normal vs Italic should have high penalty"
    );

    // Oblique vs Italic should have lower penalty
    let italic_oblique_score = italic_font.match_score(FontStyle::Oblique, FontWeight::NORMAL);
    assert!(
        italic_oblique_score < normal_italic_score,
        "Italic vs Oblique should be closer than Normal vs Italic"
    );
}

// ============================================================================
// FontDiscovery Trait Tests
// ============================================================================

#[test]
fn test_discovery_find_font_basic() {
    let discovery = FallbackDiscovery::new();

    // Should find common fonts
    let arial = discovery.find_font("Arial", FontStyle::Normal, FontWeight::NORMAL);
    assert!(arial.is_some(), "Should find Arial");
    assert_eq!(arial.unwrap().family(), "Arial");
}

#[test]
fn test_discovery_find_font_returns_best_match() {
    let discovery = FallbackDiscovery::new();

    // Request bold
    let font = discovery.find_font("Arial", FontStyle::Normal, FontWeight::BOLD);
    assert!(font.is_some());

    let font = font.unwrap();
    assert_eq!(font.family(), "Arial");
    assert_eq!(font.weight(), FontWeight::BOLD);
}

#[test]
fn test_discovery_find_family_returns_all_variants() {
    let discovery = FallbackDiscovery::new();

    let arial_fonts = discovery.find_family("Arial");
    assert!(!arial_fonts.is_empty(), "Should find Arial variants");

    // All should be Arial
    assert!(
        arial_fonts.iter().all(|f| f.family() == "Arial"),
        "All variants should be Arial"
    );

    // Should have different weights
    let weights: std::collections::HashSet<u16> =
        arial_fonts.iter().map(|f| f.weight().value()).collect();
    assert!(weights.len() > 1, "Should have multiple weight variants");
}

#[test]
fn test_discovery_list_families_sorted_and_unique() {
    let discovery = FallbackDiscovery::new();

    let families = discovery.list_families();

    // Check sorted
    let mut sorted = families.clone();
    sorted.sort();
    assert_eq!(families, sorted, "Families should be sorted");

    // Check unique
    let mut deduped = families.clone();
    deduped.dedup();
    assert_eq!(families.len(), deduped.len(), "Families should be unique");
}

#[test]
fn test_discovery_has_family_case_insensitive() {
    let discovery = FallbackDiscovery::new();

    assert!(discovery.has_family("Arial"));
    assert!(discovery.has_family("arial"));
    assert!(discovery.has_family("ARIAL"));
}

#[test]
fn test_discovery_find_with_fallbacks() {
    let discovery = FallbackDiscovery::new();

    // First doesn't exist, should fall back
    let font = discovery.find_with_fallbacks(
        &["NonExistent123", "Arial", "Helvetica"],
        FontStyle::Normal,
        FontWeight::NORMAL,
    );

    assert!(font.is_some(), "Should find fallback");
    assert_eq!(font.unwrap().family(), "Arial");
}

#[test]
fn test_discovery_find_with_fallbacks_all_missing() {
    let discovery = FallbackDiscovery::new();

    let font = discovery.find_with_fallbacks(
        &["NonExistent1", "NonExistent2", "NonExistent3"],
        FontStyle::Normal,
        FontWeight::NORMAL,
    );

    assert!(
        font.is_none(),
        "Should return None when all fallbacks missing"
    );
}

#[test]
fn test_discovery_system_fonts_not_empty() {
    let discovery = FallbackDiscovery::new();
    let fonts = discovery.system_fonts();
    assert!(!fonts.is_empty(), "System fonts should not be empty");
}

// ============================================================================
// Error Type Tests
// ============================================================================

#[test]
fn test_font_error_display() {
    let errors = [
        (FontError::NotFound("Arial".to_string()), "Arial"),
        (
            FontError::InvalidFont("corrupt.ttf".to_string()),
            "corrupt.ttf",
        ),
        (
            FontError::PlatformError("API failed".to_string()),
            "API failed",
        ),
        (FontError::InvalidWeight(0), "0"),
    ];

    for (error, expected_content) in errors {
        let display = format!("{}", error);
        assert!(
            display.contains(expected_content),
            "Error display '{}' should contain '{}'",
            display,
            expected_content
        );
    }
}

#[test]
fn test_font_error_is_std_error() {
    fn accepts_std_error<E: std::error::Error>(_: E) {}

    accepts_std_error(FontError::NotFound("test".to_string()));
    accepts_std_error(FontError::InvalidWeight(0));
}

// ============================================================================
// Platform-Specific Tests (Feature-Gated)
// ============================================================================

#[cfg(feature = "fontconfig")]
mod fontconfig_tests {
    use super::*;
    use css_fonts::FontconfigDiscovery;

    #[test]
    fn test_fontconfig_has_linux_fonts() {
        let discovery = FontconfigDiscovery::new();
        let families = discovery.list_families();

        // Should have typical Linux fonts
        assert!(
            families
                .iter()
                .any(|f| f.contains("DejaVu") || f.contains("Liberation")),
            "Should have Linux-specific fonts"
        );
    }

    #[test]
    fn test_fontconfig_paths_are_unix() {
        let discovery = FontconfigDiscovery::new();
        let fonts = discovery.system_fonts();

        for font in fonts {
            if let Some(path) = font.path() {
                let path_str = path.to_string_lossy();
                assert!(
                    path_str.starts_with("/"),
                    "Path should be Unix-style: {}",
                    path_str
                );
            }
        }
    }
}

#[cfg(feature = "directwrite")]
mod directwrite_tests {
    use super::*;
    use css_fonts::DirectWriteDiscovery;

    #[test]
    fn test_directwrite_has_windows_fonts() {
        let discovery = DirectWriteDiscovery::new();
        let families = discovery.list_families();

        // Should have typical Windows fonts
        assert!(
            families.iter().any(|f| f == "Segoe UI"),
            "Should have Segoe UI"
        );
    }

    #[test]
    fn test_directwrite_paths_are_windows() {
        let discovery = DirectWriteDiscovery::new();
        let fonts = discovery.system_fonts();

        for font in fonts {
            if let Some(path) = font.path() {
                let path_str = path.to_string_lossy();
                assert!(
                    path_str.contains("\\Windows\\Fonts"),
                    "Path should be Windows-style: {}",
                    path_str
                );
            }
        }
    }
}

#[cfg(feature = "coretext")]
mod coretext_tests {
    use super::*;
    use css_fonts::CoreTextDiscovery;

    #[test]
    fn test_coretext_has_macos_fonts() {
        let discovery = CoreTextDiscovery::new();
        let families = discovery.list_families();

        // Should have typical macOS fonts
        assert!(
            families
                .iter()
                .any(|f| f.contains("SF") || f == "Helvetica Neue"),
            "Should have macOS-specific fonts"
        );
    }

    #[test]
    fn test_coretext_paths_are_macos() {
        let discovery = CoreTextDiscovery::new();
        let fonts = discovery.system_fonts();

        for font in fonts {
            if let Some(path) = font.path() {
                let path_str = path.to_string_lossy();
                assert!(
                    path_str.contains("/Library/Fonts") || path_str.starts_with("/System"),
                    "Path should be macOS-style: {}",
                    path_str
                );
            }
        }
    }
}

// ============================================================================
// Cross-Platform Consistency Tests
// ============================================================================

#[test]
fn test_all_implementations_share_common_api() {
    // This test verifies that all implementations can be used through the trait
    fn test_discovery<D: FontDiscovery>(discovery: &D, name: &str) {
        let families = discovery.list_families();
        assert!(!families.is_empty(), "{} should return families", name);

        let fonts = discovery.system_fonts();
        assert!(!fonts.is_empty(), "{} should return fonts", name);

        // Should be able to query for a font (may or may not find it)
        let _ = discovery.find_font("Arial", FontStyle::Normal, FontWeight::NORMAL);
        let _ = discovery.has_family("Arial");
        let _ = discovery.find_family("Arial");
    }

    test_discovery(&FallbackDiscovery::new(), "FallbackDiscovery");

    #[cfg(feature = "fontconfig")]
    test_discovery(
        &css_fonts::FontconfigDiscovery::new(),
        "FontconfigDiscovery",
    );

    #[cfg(feature = "directwrite")]
    test_discovery(
        &css_fonts::DirectWriteDiscovery::new(),
        "DirectWriteDiscovery",
    );

    #[cfg(feature = "coretext")]
    test_discovery(&css_fonts::CoreTextDiscovery::new(), "CoreTextDiscovery");
}
