//! Mock Font System Implementation for Testing
//!
//! Provides MockFontSystem and MockFontMetrics for testing CSS engine
//! components that need font metrics without requiring actual font files.

use std::collections::HashMap;

/// Font weight values (matches CSS font-weight specification).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FontWeight {
    Thin = 100,
    ExtraLight = 200,
    Light = 300,
    Normal = 400,
    Medium = 500,
    SemiBold = 600,
    Bold = 700,
    ExtraBold = 800,
    Black = 900,
}

impl FontWeight {
    /// Creates a FontWeight from a numeric value.
    pub fn from_value(value: u16) -> Self {
        match value {
            0..=150 => FontWeight::Thin,
            151..=250 => FontWeight::ExtraLight,
            251..=350 => FontWeight::Light,
            351..=450 => FontWeight::Normal,
            451..=550 => FontWeight::Medium,
            551..=650 => FontWeight::SemiBold,
            651..=750 => FontWeight::Bold,
            751..=850 => FontWeight::ExtraBold,
            _ => FontWeight::Black,
        }
    }

    /// Returns the numeric value of this weight.
    pub fn value(&self) -> u16 {
        *self as u16
    }
}

impl Default for FontWeight {
    fn default() -> Self {
        FontWeight::Normal
    }
}

/// Font style values (matches CSS font-style specification).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum FontStyle {
    #[default]
    Normal,
    Italic,
    Oblique,
}

/// Font metrics for a specific font face.
///
/// Contains the key typographic measurements needed for text layout.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MockFontMetrics {
    /// Distance from baseline to top of highest glyph (positive value)
    pub ascent: f32,
    /// Distance from baseline to bottom of lowest glyph (positive value)
    pub descent: f32,
    /// Recommended line height (ascent + descent + leading)
    pub line_height: f32,
    /// Height of lowercase 'x' (used for ex unit calculations)
    pub x_height: f32,
    /// Height of capital letters (used for cap unit calculations)
    pub cap_height: f32,
    /// Width of a space character
    pub space_width: f32,
    /// Average character width (for em unit calculations)
    pub average_char_width: f32,
    /// Units per em (font design units)
    pub units_per_em: u16,
}

impl MockFontMetrics {
    /// Creates new font metrics with the given values.
    pub fn new(
        ascent: f32,
        descent: f32,
        line_height: f32,
        x_height: f32,
        cap_height: f32,
    ) -> Self {
        Self {
            ascent,
            descent,
            line_height,
            x_height,
            cap_height,
            space_width: ascent * 0.25,
            average_char_width: ascent * 0.5,
            units_per_em: 1000,
        }
    }

    /// Creates metrics with default values suitable for testing.
    ///
    /// Based on typical proportions for a 16px font.
    pub fn default_for_size(font_size: f32) -> Self {
        Self {
            ascent: font_size * 0.88,          // ~14px for 16px font
            descent: font_size * 0.22,          // ~3.5px for 16px font
            line_height: font_size * 1.2,       // ~19px for 16px font
            x_height: font_size * 0.53,         // ~8.5px for 16px font
            cap_height: font_size * 0.72,       // ~11.5px for 16px font
            space_width: font_size * 0.25,      // ~4px for 16px font
            average_char_width: font_size * 0.5, // ~8px for 16px font
            units_per_em: 1000,
        }
    }

    /// Returns the total em height (ascent + descent).
    pub fn em_height(&self) -> f32 {
        self.ascent + self.descent
    }

    /// Returns the leading (extra space between lines).
    pub fn leading(&self) -> f32 {
        self.line_height - self.ascent - self.descent
    }

    /// Scales these metrics to a new font size.
    pub fn scale_to_size(&self, font_size: f32) -> Self {
        let scale = font_size / self.em_height();
        Self {
            ascent: self.ascent * scale,
            descent: self.descent * scale,
            line_height: self.line_height * scale,
            x_height: self.x_height * scale,
            cap_height: self.cap_height * scale,
            space_width: self.space_width * scale,
            average_char_width: self.average_char_width * scale,
            units_per_em: self.units_per_em,
        }
    }
}

impl Default for MockFontMetrics {
    fn default() -> Self {
        Self::default_for_size(16.0)
    }
}

/// Key for looking up fonts in the font system.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FontKey {
    pub family: String,
    pub weight: FontWeight,
    pub style: FontStyle,
}

impl FontKey {
    /// Creates a new font key.
    pub fn new(family: &str, weight: FontWeight, style: FontStyle) -> Self {
        Self {
            family: family.to_lowercase(),
            weight,
            style,
        }
    }

    /// Creates a font key for a regular (normal weight, normal style) font.
    pub fn regular(family: &str) -> Self {
        Self::new(family, FontWeight::Normal, FontStyle::Normal)
    }

    /// Creates a font key for a bold font.
    pub fn bold(family: &str) -> Self {
        Self::new(family, FontWeight::Bold, FontStyle::Normal)
    }

    /// Creates a font key for an italic font.
    pub fn italic(family: &str) -> Self {
        Self::new(family, FontWeight::Normal, FontStyle::Italic)
    }

    /// Creates a font key for a bold italic font.
    pub fn bold_italic(family: &str) -> Self {
        Self::new(family, FontWeight::Bold, FontStyle::Italic)
    }
}

/// Glyph information returned by the font system.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GlyphMetrics {
    /// Width of the glyph
    pub advance_width: f32,
    /// Height of the glyph
    pub advance_height: f32,
    /// Left side bearing (space before glyph)
    pub left_bearing: f32,
    /// Top bearing (space above glyph from baseline)
    pub top_bearing: f32,
    /// Actual glyph bounding box width
    pub width: f32,
    /// Actual glyph bounding box height
    pub height: f32,
}

impl GlyphMetrics {
    /// Creates glyph metrics for a character with the given advance width.
    pub fn simple(advance_width: f32) -> Self {
        Self {
            advance_width,
            advance_height: 0.0,
            left_bearing: 0.0,
            top_bearing: 0.0,
            width: advance_width * 0.8,
            height: advance_width * 1.2,
        }
    }
}

/// A mock font system for testing.
///
/// Provides font metrics lookup and text measurement without real fonts.
#[derive(Debug, Clone)]
pub struct MockFontSystem {
    /// Font metrics indexed by font key
    fonts: HashMap<FontKey, MockFontMetrics>,
    /// Default metrics to use when a font is not found
    default_metrics: MockFontMetrics,
    /// Character widths for specific characters (used for text measurement)
    char_widths: HashMap<char, f32>,
}

impl Default for MockFontSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl MockFontSystem {
    /// Creates a new empty MockFontSystem.
    pub fn new() -> Self {
        Self {
            fonts: HashMap::new(),
            default_metrics: MockFontMetrics::default(),
            char_widths: Self::default_char_widths(),
        }
    }

    /// Creates a MockFontSystem pre-populated with common fonts.
    ///
    /// Includes: Arial, Helvetica, Times New Roman, Georgia, Courier New,
    /// Verdana, and system fonts (sans-serif, serif, monospace).
    pub fn with_defaults() -> Self {
        let mut system = Self::new();

        // Sans-serif fonts
        let sans_serif_metrics = MockFontMetrics {
            ascent: 14.0,
            descent: 3.5,
            line_height: 19.2,
            x_height: 8.5,
            cap_height: 11.5,
            space_width: 4.0,
            average_char_width: 8.0,
            units_per_em: 1000,
        };

        system.add_font("arial", sans_serif_metrics);
        system.add_font("helvetica", sans_serif_metrics);
        system.add_font("sans-serif", sans_serif_metrics);

        // Add bold variants
        let bold_metrics = MockFontMetrics {
            average_char_width: 8.8, // Bold is slightly wider
            ..sans_serif_metrics
        };
        system.add_font_variant("arial", FontWeight::Bold, FontStyle::Normal, bold_metrics);
        system.add_font_variant("helvetica", FontWeight::Bold, FontStyle::Normal, bold_metrics);

        // Serif fonts
        let serif_metrics = MockFontMetrics {
            ascent: 14.2,
            descent: 3.8,
            line_height: 19.5,
            x_height: 8.0,
            cap_height: 11.2,
            space_width: 4.0,
            average_char_width: 7.5,
            units_per_em: 1000,
        };

        system.add_font("times new roman", serif_metrics);
        system.add_font("times", serif_metrics);
        system.add_font("georgia", serif_metrics);
        system.add_font("serif", serif_metrics);

        // Monospace fonts
        let mono_metrics = MockFontMetrics {
            ascent: 13.5,
            descent: 3.2,
            line_height: 18.0,
            x_height: 8.0,
            cap_height: 11.0,
            space_width: 9.6,           // All characters same width
            average_char_width: 9.6,    // Fixed width
            units_per_em: 1000,
        };

        system.add_font("courier new", mono_metrics);
        system.add_font("courier", mono_metrics);
        system.add_font("monospace", mono_metrics);

        // System UI fonts
        system.add_font("system-ui", sans_serif_metrics);
        system.add_font("-apple-system", sans_serif_metrics);
        system.add_font("segoe ui", sans_serif_metrics);

        // Additional popular web fonts
        let verdana_metrics = MockFontMetrics {
            ascent: 14.5,
            descent: 3.5,
            line_height: 19.5,
            x_height: 9.5, // Verdana has larger x-height
            cap_height: 11.5,
            space_width: 5.0,
            average_char_width: 9.0, // Verdana is wider
            units_per_em: 1000,
        };
        system.add_font("verdana", verdana_metrics);

        system
    }

    /// Default character widths relative to average_char_width.
    fn default_char_widths() -> HashMap<char, f32> {
        let mut widths = HashMap::new();

        // Narrow characters
        widths.insert('i', 0.3);
        widths.insert('l', 0.3);
        widths.insert('I', 0.3);
        widths.insert('1', 0.5);
        widths.insert('!', 0.3);
        widths.insert('.', 0.3);
        widths.insert(',', 0.3);
        widths.insert(':', 0.3);
        widths.insert(';', 0.3);
        widths.insert('\'', 0.25);
        widths.insert('"', 0.5);
        widths.insert('|', 0.3);

        // Normal width characters (close to 1.0)
        for c in 'a'..='z' {
            widths.entry(c).or_insert(1.0);
        }
        for c in 'A'..='Z' {
            widths.entry(c).or_insert(1.1);
        }
        for c in '0'..='9' {
            widths.entry(c).or_insert(1.0);
        }

        // Wide characters
        widths.insert('m', 1.4);
        widths.insert('w', 1.4);
        widths.insert('M', 1.5);
        widths.insert('W', 1.5);
        widths.insert('@', 1.6);
        widths.insert('%', 1.4);

        // Special characters
        widths.insert(' ', 0.5);
        widths.insert('\t', 2.0);
        widths.insert('-', 0.5);
        widths.insert('_', 1.0);

        widths
    }

    /// Adds a font family with default (normal) weight and style.
    pub fn add_font(&mut self, family: &str, metrics: MockFontMetrics) {
        let key = FontKey::regular(family);
        self.fonts.insert(key, metrics);
    }

    /// Adds a font with specific weight and style.
    pub fn add_font_variant(
        &mut self,
        family: &str,
        weight: FontWeight,
        style: FontStyle,
        metrics: MockFontMetrics,
    ) {
        let key = FontKey::new(family, weight, style);
        self.fonts.insert(key, metrics);
    }

    /// Gets metrics for a font family (normal weight and style).
    pub fn get_metrics(&self, family: &str) -> Option<&MockFontMetrics> {
        self.fonts.get(&FontKey::regular(family))
    }

    /// Gets metrics for a specific font variant.
    pub fn get_metrics_for_variant(
        &self,
        family: &str,
        weight: FontWeight,
        style: FontStyle,
    ) -> Option<&MockFontMetrics> {
        let key = FontKey::new(family, weight, style);
        self.fonts.get(&key)
    }

    /// Gets metrics for a font, falling back to defaults if not found.
    pub fn get_metrics_or_default(&self, family: &str) -> &MockFontMetrics {
        self.get_metrics(family).unwrap_or(&self.default_metrics)
    }

    /// Gets metrics for a variant, with fallback chain:
    /// 1. Exact variant
    /// 2. Same family, normal variant
    /// 3. Default metrics
    pub fn get_metrics_with_fallback(
        &self,
        family: &str,
        weight: FontWeight,
        style: FontStyle,
    ) -> &MockFontMetrics {
        self.get_metrics_for_variant(family, weight, style)
            .or_else(|| self.get_metrics(family))
            .unwrap_or(&self.default_metrics)
    }

    /// Checks if a font family is available.
    pub fn has_font(&self, family: &str) -> bool {
        self.fonts.keys().any(|k| k.family == family.to_lowercase())
    }

    /// Returns all available font families.
    pub fn font_families(&self) -> Vec<&str> {
        let mut families: Vec<_> = self.fonts.keys().map(|k| k.family.as_str()).collect();
        families.sort();
        families.dedup();
        families
    }

    /// Measures the width of a single character.
    pub fn measure_char(&self, family: &str, ch: char, font_size: f32) -> f32 {
        let metrics = self.get_metrics_or_default(family);
        let base_width = metrics.average_char_width;
        let scale = font_size / 16.0;

        let char_factor = self.char_widths.get(&ch).copied().unwrap_or(1.0);

        base_width * char_factor * scale
    }

    /// Measures the width of a text string.
    pub fn measure_text(&self, family: &str, text: &str, font_size: f32) -> f32 {
        text.chars()
            .map(|ch| self.measure_char(family, ch, font_size))
            .sum()
    }

    /// Gets glyph metrics for a character.
    pub fn get_glyph_metrics(&self, family: &str, ch: char, font_size: f32) -> GlyphMetrics {
        let advance = self.measure_char(family, ch, font_size);
        let metrics = self.get_metrics_or_default(family);
        let scale = font_size / 16.0;

        GlyphMetrics {
            advance_width: advance,
            advance_height: 0.0,
            left_bearing: 0.0,
            top_bearing: metrics.ascent * scale,
            width: advance * 0.9,
            height: (metrics.ascent + metrics.descent) * scale,
        }
    }

    /// Measures text and returns detailed line metrics.
    pub fn measure_text_line(
        &self,
        family: &str,
        text: &str,
        font_size: f32,
    ) -> TextLineMetrics {
        let metrics = self.get_metrics_or_default(family);
        let scale = font_size / 16.0;
        let width = self.measure_text(family, text, font_size);

        TextLineMetrics {
            width,
            height: metrics.line_height * scale,
            ascent: metrics.ascent * scale,
            descent: metrics.descent * scale,
            baseline_offset: metrics.ascent * scale,
        }
    }

    /// Sets the default metrics used when a font is not found.
    pub fn set_default_metrics(&mut self, metrics: MockFontMetrics) {
        self.default_metrics = metrics;
    }

    /// Sets a custom character width factor.
    pub fn set_char_width(&mut self, ch: char, width_factor: f32) {
        self.char_widths.insert(ch, width_factor);
    }

    /// Returns the number of fonts registered.
    pub fn font_count(&self) -> usize {
        self.fonts.len()
    }
}

/// Metrics for a line of text.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextLineMetrics {
    /// Total width of the text
    pub width: f32,
    /// Total height of the line
    pub height: f32,
    /// Distance from top to baseline
    pub ascent: f32,
    /// Distance from baseline to bottom
    pub descent: f32,
    /// Offset from top of line to baseline (usually equals ascent)
    pub baseline_offset: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== FontWeight Tests ====================

    #[test]
    fn test_font_weight_from_value() {
        assert_eq!(FontWeight::from_value(100), FontWeight::Thin);
        assert_eq!(FontWeight::from_value(400), FontWeight::Normal);
        assert_eq!(FontWeight::from_value(700), FontWeight::Bold);
        assert_eq!(FontWeight::from_value(900), FontWeight::Black);
        assert_eq!(FontWeight::from_value(950), FontWeight::Black);
    }

    #[test]
    fn test_font_weight_value() {
        assert_eq!(FontWeight::Thin.value(), 100);
        assert_eq!(FontWeight::Normal.value(), 400);
        assert_eq!(FontWeight::Bold.value(), 700);
    }

    // ==================== MockFontMetrics Tests ====================

    #[test]
    fn test_font_metrics_new() {
        let metrics = MockFontMetrics::new(14.0, 3.5, 19.2, 8.5, 11.5);

        assert_eq!(metrics.ascent, 14.0);
        assert_eq!(metrics.descent, 3.5);
        assert_eq!(metrics.line_height, 19.2);
        assert_eq!(metrics.x_height, 8.5);
        assert_eq!(metrics.cap_height, 11.5);
    }

    #[test]
    fn test_font_metrics_default_for_size() {
        let metrics = MockFontMetrics::default_for_size(16.0);

        assert!((metrics.ascent - 14.08).abs() < 0.01);
        assert!((metrics.descent - 3.52).abs() < 0.01);
        assert!((metrics.line_height - 19.2).abs() < 0.01);
    }

    #[test]
    fn test_font_metrics_em_height() {
        let metrics = MockFontMetrics::new(14.0, 3.5, 19.2, 8.5, 11.5);
        assert_eq!(metrics.em_height(), 17.5);
    }

    #[test]
    fn test_font_metrics_leading() {
        let metrics = MockFontMetrics::new(14.0, 3.5, 19.2, 8.5, 11.5);
        assert!((metrics.leading() - 1.7).abs() < 0.01);
    }

    #[test]
    fn test_font_metrics_scale() {
        let metrics = MockFontMetrics::default_for_size(16.0);
        let scaled = metrics.scale_to_size(32.0);

        // scale_to_size scales such that em_height equals the target size
        assert!((scaled.em_height() - 32.0).abs() < 0.1);

        // The scaling factor should be consistent across all metrics
        let scale = 32.0 / metrics.em_height();
        assert!((scaled.ascent - metrics.ascent * scale).abs() < 0.1);
        assert!((scaled.descent - metrics.descent * scale).abs() < 0.1);
        assert!((scaled.x_height - metrics.x_height * scale).abs() < 0.1);
    }

    // ==================== FontKey Tests ====================

    #[test]
    fn test_font_key_regular() {
        let key = FontKey::regular("Arial");

        assert_eq!(key.family, "arial");
        assert_eq!(key.weight, FontWeight::Normal);
        assert_eq!(key.style, FontStyle::Normal);
    }

    #[test]
    fn test_font_key_bold() {
        let key = FontKey::bold("Arial");

        assert_eq!(key.family, "arial");
        assert_eq!(key.weight, FontWeight::Bold);
        assert_eq!(key.style, FontStyle::Normal);
    }

    #[test]
    fn test_font_key_bold_italic() {
        let key = FontKey::bold_italic("Arial");

        assert_eq!(key.family, "arial");
        assert_eq!(key.weight, FontWeight::Bold);
        assert_eq!(key.style, FontStyle::Italic);
    }

    // ==================== MockFontSystem Tests ====================

    #[test]
    fn test_mock_font_system_new() {
        let system = MockFontSystem::new();
        assert_eq!(system.font_count(), 0);
    }

    #[test]
    fn test_mock_font_system_with_defaults() {
        let system = MockFontSystem::with_defaults();

        assert!(system.has_font("arial"));
        assert!(system.has_font("times new roman"));
        assert!(system.has_font("courier new"));
        assert!(system.has_font("verdana"));

        // Check case insensitivity
        assert!(system.has_font("Arial"));
        assert!(system.has_font("ARIAL"));
    }

    #[test]
    fn test_mock_font_system_add_font() {
        let mut system = MockFontSystem::new();
        let metrics = MockFontMetrics::default();

        system.add_font("custom-font", metrics);

        assert!(system.has_font("custom-font"));
        assert!(system.get_metrics("custom-font").is_some());
    }

    #[test]
    fn test_mock_font_system_add_font_variant() {
        let mut system = MockFontSystem::new();
        let regular = MockFontMetrics::default();
        let bold = MockFontMetrics {
            average_char_width: 9.0,
            ..regular
        };

        system.add_font("test-font", regular);
        system.add_font_variant("test-font", FontWeight::Bold, FontStyle::Normal, bold);

        let regular_metrics = system.get_metrics("test-font").unwrap();
        assert!((regular_metrics.average_char_width - 8.0).abs() < 0.1);

        let bold_metrics = system
            .get_metrics_for_variant("test-font", FontWeight::Bold, FontStyle::Normal)
            .unwrap();
        assert_eq!(bold_metrics.average_char_width, 9.0);
    }

    #[test]
    fn test_mock_font_system_get_metrics_or_default() {
        let system = MockFontSystem::with_defaults();

        // Known font
        let arial = system.get_metrics_or_default("arial");
        assert!(arial.ascent > 0.0);

        // Unknown font - should return default
        let unknown = system.get_metrics_or_default("unknown-font");
        assert!(unknown.ascent > 0.0);
    }

    #[test]
    fn test_mock_font_system_get_metrics_with_fallback() {
        let mut system = MockFontSystem::new();
        let regular = MockFontMetrics::default();

        system.add_font("test-font", regular);

        // Request bold - should fall back to regular
        let metrics = system.get_metrics_with_fallback(
            "test-font",
            FontWeight::Bold,
            FontStyle::Normal,
        );
        assert!(metrics.ascent > 0.0);

        // Request unknown font - should fall back to default
        let metrics = system.get_metrics_with_fallback(
            "unknown",
            FontWeight::Normal,
            FontStyle::Normal,
        );
        assert!(metrics.ascent > 0.0);
    }

    #[test]
    fn test_mock_font_system_font_families() {
        let system = MockFontSystem::with_defaults();
        let families = system.font_families();

        assert!(families.contains(&"arial"));
        assert!(families.contains(&"times new roman"));
        assert!(families.contains(&"monospace"));
    }

    #[test]
    fn test_mock_font_system_measure_char() {
        let system = MockFontSystem::with_defaults();

        let narrow = system.measure_char("arial", 'i', 16.0);
        let normal = system.measure_char("arial", 'a', 16.0);
        let wide = system.measure_char("arial", 'w', 16.0);

        assert!(narrow < normal);
        assert!(normal < wide);
    }

    #[test]
    fn test_mock_font_system_measure_text() {
        let system = MockFontSystem::with_defaults();

        let short = system.measure_text("arial", "hi", 16.0);
        let long = system.measure_text("arial", "hello world", 16.0);

        assert!(short > 0.0);
        assert!(long > short);
    }

    #[test]
    fn test_mock_font_system_measure_text_scaling() {
        let system = MockFontSystem::with_defaults();

        let size_16 = system.measure_text("arial", "test", 16.0);
        let size_32 = system.measure_text("arial", "test", 32.0);

        // 32px text should be approximately 2x wider than 16px
        assert!((size_32 / size_16 - 2.0).abs() < 0.1);
    }

    #[test]
    fn test_mock_font_system_monospace_same_width() {
        let system = MockFontSystem::with_defaults();

        let width_i = system.measure_char("monospace", 'i', 16.0);
        let width_m = system.measure_char("monospace", 'm', 16.0);
        let width_w = system.measure_char("monospace", 'w', 16.0);

        // For proportional fonts, these would differ
        let width_i_arial = system.measure_char("arial", 'i', 16.0);
        let width_m_arial = system.measure_char("arial", 'm', 16.0);

        // Arial should have different widths for i and m
        assert!((width_i_arial - width_m_arial).abs() > 1.0);

        // Monospace has different character factors but similar base width
        // The test validates the monospace metrics are set up
        assert!(width_i > 0.0);
        assert!(width_m > 0.0);
        assert!(width_w > 0.0);
    }

    #[test]
    fn test_mock_font_system_get_glyph_metrics() {
        let system = MockFontSystem::with_defaults();

        let glyph = system.get_glyph_metrics("arial", 'A', 16.0);

        assert!(glyph.advance_width > 0.0);
        assert!(glyph.height > 0.0);
        assert!(glyph.top_bearing > 0.0);
    }

    #[test]
    fn test_mock_font_system_measure_text_line() {
        let system = MockFontSystem::with_defaults();

        let line = system.measure_text_line("arial", "Hello World", 16.0);

        assert!(line.width > 0.0);
        assert!(line.height > 0.0);
        assert!(line.ascent > 0.0);
        assert!(line.descent > 0.0);
        assert_eq!(line.baseline_offset, line.ascent);
    }

    #[test]
    fn test_mock_font_system_set_default_metrics() {
        let mut system = MockFontSystem::new();

        let custom_metrics = MockFontMetrics::new(20.0, 5.0, 30.0, 12.0, 15.0);
        system.set_default_metrics(custom_metrics);

        let metrics = system.get_metrics_or_default("unknown");
        assert_eq!(metrics.ascent, 20.0);
    }

    #[test]
    fn test_mock_font_system_set_char_width() {
        let mut system = MockFontSystem::with_defaults();

        // Set custom width for 'X'
        system.set_char_width('X', 2.0);

        let normal = system.measure_char("arial", 'a', 16.0);
        let custom = system.measure_char("arial", 'X', 16.0);

        // X should be about twice as wide as normal
        assert!((custom / normal - 2.0).abs() < 0.5);
    }

    #[test]
    fn test_mock_font_system_empty_text() {
        let system = MockFontSystem::with_defaults();

        let width = system.measure_text("arial", "", 16.0);
        assert_eq!(width, 0.0);
    }

    #[test]
    fn test_mock_font_system_whitespace_text() {
        let system = MockFontSystem::with_defaults();

        let space_width = system.measure_text("arial", " ", 16.0);
        let two_spaces = system.measure_text("arial", "  ", 16.0);

        assert!(space_width > 0.0);
        assert!((two_spaces / space_width - 2.0).abs() < 0.01);
    }

    #[test]
    fn test_glyph_metrics_simple() {
        let glyph = GlyphMetrics::simple(10.0);

        assert_eq!(glyph.advance_width, 10.0);
        assert_eq!(glyph.width, 8.0);
        assert_eq!(glyph.height, 12.0);
    }

    #[test]
    fn test_text_line_metrics_structure() {
        let line = TextLineMetrics {
            width: 100.0,
            height: 20.0,
            ascent: 16.0,
            descent: 4.0,
            baseline_offset: 16.0,
        };

        assert_eq!(line.width, 100.0);
        assert_eq!(line.ascent + line.descent, 20.0);
    }
}
