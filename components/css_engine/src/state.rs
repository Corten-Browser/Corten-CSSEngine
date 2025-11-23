//! State management for the CSS Engine
//!
//! This module provides state management types for the CSS engine, including:
//! - StylesheetRegistry: Manages parsed stylesheets
//! - StyleCache: Caches computed styles
//! - CssEngineState: Complete engine state with animations, media queries, etc.
//!
//! # Example
//!
//! ```
//! use css_engine::state::{CssEngineState, ParsedStylesheet};
//! use css_engine::ResourceLimits;
//!
//! let mut state = CssEngineState::new(ResourceLimits::default());
//! state.add_stylesheet(ParsedStylesheet::new("body { color: red; }".to_string()));
//! ```

use crate::config::ResourceLimits;
use crate::error::{ElementId, StyleSheetId};
use crate::message::ViewportInfo;
use crate::types::ComputedStyle;
use fxhash::FxHashMap;

/// Registry for managing parsed stylesheets
#[derive(Debug)]
pub struct StylesheetRegistry {
    /// Map of stylesheet IDs to stylesheet data
    stylesheets: FxHashMap<StyleSheetId, Stylesheet>,
    /// Next available stylesheet ID
    next_id: u32,
}

impl StylesheetRegistry {
    /// Create a new empty stylesheet registry
    pub fn new() -> Self {
        StylesheetRegistry {
            stylesheets: FxHashMap::default(),
            next_id: 0,
        }
    }

    /// Register a new stylesheet and return its ID
    pub fn register(&mut self, stylesheet: Stylesheet) -> StyleSheetId {
        let id = StyleSheetId::new(self.next_id);
        self.next_id += 1;
        self.stylesheets.insert(id, stylesheet);
        id
    }

    /// Get the number of registered stylesheets
    pub fn len(&self) -> usize {
        self.stylesheets.len()
    }
}

impl Default for StylesheetRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Parsed stylesheet data
#[derive(Debug, Clone)]
#[allow(dead_code)] // Fields will be used when parser is integrated
pub struct Stylesheet {
    /// Original CSS source
    pub source: String,
    /// Source URL (if available)
    pub source_url: Option<String>,
    /// Parsed rules (placeholder for now)
    pub rules: Vec<Rule>,
}

impl Stylesheet {
    /// Create a new stylesheet
    pub fn new(source: String, source_url: Option<String>) -> Self {
        Stylesheet {
            source,
            source_url,
            rules: Vec::new(),
        }
    }
}

/// CSS rule placeholder
#[derive(Debug, Clone)]
#[allow(dead_code)] // Fields will be used when parser is integrated
pub struct Rule {
    /// Selector text
    pub selector: String,
    /// Declarations
    pub declarations: Vec<Declaration>,
}

/// CSS declaration placeholder
#[derive(Debug, Clone)]
#[allow(dead_code)] // Fields will be used when parser is integrated
pub struct Declaration {
    /// Property name
    pub property: String,
    /// Property value
    pub value: String,
}

/// Cache for storing computed styles
#[derive(Debug)]
pub struct StyleCache {
    /// Map of element IDs to computed styles
    cache: FxHashMap<ElementId, ComputedStyle>,
}

impl StyleCache {
    /// Create a new empty style cache
    pub fn new() -> Self {
        StyleCache {
            cache: FxHashMap::default(),
        }
    }

    /// Insert a computed style for an element
    pub fn insert(&mut self, element_id: ElementId, style: ComputedStyle) {
        self.cache.insert(element_id, style);
    }

    /// Get a computed style for an element
    pub fn get(&self, element_id: ElementId) -> Option<&ComputedStyle> {
        self.cache.get(&element_id)
    }

    /// Clear all cached styles
    pub fn clear(&mut self) {
        self.cache.clear();
    }

    /// Invalidate specific element
    pub fn invalidate(&mut self, element_id: ElementId) {
        self.cache.remove(&element_id);
    }

    /// Get the number of cached styles
    pub fn len(&self) -> usize {
        self.cache.len()
    }
}

impl Default for StyleCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Engine configuration
#[derive(Debug, Clone)]
pub struct EngineConfig {
    /// Enable style sharing optimization
    pub enable_style_sharing: bool,
    /// Enable parallel computation
    pub enable_parallel: bool,
    /// Maximum cache size
    pub max_cache_size: usize,
}

impl EngineConfig {
    /// Create default configuration
    pub fn new() -> Self {
        EngineConfig {
            enable_style_sharing: true,
            enable_parallel: false, // Start with single-threaded for simplicity
            max_cache_size: 10000,
        }
    }
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stylesheet_registry_register() {
        let mut registry = StylesheetRegistry::new();
        let sheet = Stylesheet::new("body { color: red; }".to_string(), None);
        let id = registry.register(sheet);

        assert_eq!(id.0, 0);
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn test_stylesheet_registry_multiple() {
        let mut registry = StylesheetRegistry::new();
        let id1 = registry.register(Stylesheet::new("".to_string(), None));
        let id2 = registry.register(Stylesheet::new("".to_string(), None));

        assert_eq!(id1.0, 0);
        assert_eq!(id2.0, 1);
        assert_eq!(registry.len(), 2);
    }


    #[test]
    fn test_style_cache_insert_and_get() {
        let mut cache = StyleCache::new();
        let element_id = ElementId::new(1);
        let style = ComputedStyle::default();

        cache.insert(element_id, style.clone());
        let retrieved = cache.get(element_id);

        assert!(retrieved.is_some());
    }

    #[test]
    fn test_style_cache_invalidate() {
        let mut cache = StyleCache::new();
        let element_id = ElementId::new(1);
        cache.insert(element_id, ComputedStyle::default());

        assert_eq!(cache.len(), 1);
        cache.invalidate(element_id);
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn test_style_cache_clear() {
        let mut cache = StyleCache::new();
        cache.insert(ElementId::new(1), ComputedStyle::default());
        cache.insert(ElementId::new(2), ComputedStyle::default());

        assert_eq!(cache.len(), 2);
        cache.clear();
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn test_stylesheet_registry_len() {
        let mut registry = StylesheetRegistry::new();
        assert_eq!(registry.len(), 0);

        registry.register(Stylesheet::new("".to_string(), None));
        assert_eq!(registry.len(), 1);

        registry.register(Stylesheet::new("".to_string(), None));
        assert_eq!(registry.len(), 2);
    }

    #[test]
    fn test_engine_config_defaults() {
        let config = EngineConfig::new();
        assert!(config.enable_style_sharing);
        assert!(!config.enable_parallel);
        assert_eq!(config.max_cache_size, 10000);
    }
}

// ============================================================================
// Phase 2: Enhanced State Management Types
// ============================================================================

/// A parsed stylesheet with metadata
#[derive(Debug, Clone)]
pub struct ParsedStylesheet {
    /// Unique identifier for this stylesheet
    pub id: StyleSheetId,
    /// Original CSS source
    pub source: String,
    /// Source URL (if available)
    pub source_url: Option<String>,
    /// Parsed rules
    pub rules: Vec<ParsedRule>,
    /// Whether stylesheet is enabled
    pub enabled: bool,
    /// Media query conditions for this stylesheet
    pub media: Option<String>,
}

impl ParsedStylesheet {
    /// Create a new parsed stylesheet
    pub fn new(source: String) -> Self {
        ParsedStylesheet {
            id: StyleSheetId::new(0),
            source,
            source_url: None,
            rules: Vec::new(),
            enabled: true,
            media: None,
        }
    }

    /// Create stylesheet with a specific ID
    pub fn with_id(mut self, id: StyleSheetId) -> Self {
        self.id = id;
        self
    }

    /// Set the source URL
    pub fn with_source_url(mut self, url: String) -> Self {
        self.source_url = Some(url);
        self
    }

    /// Set the media query condition
    pub fn with_media(mut self, media: String) -> Self {
        self.media = Some(media);
        self
    }

    /// Enable or disable the stylesheet
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Add a parsed rule
    pub fn add_rule(&mut self, rule: ParsedRule) {
        self.rules.push(rule);
    }
}

/// A parsed CSS rule
#[derive(Debug, Clone)]
pub struct ParsedRule {
    /// Selector text
    pub selector: String,
    /// Declarations in this rule
    pub declarations: Vec<PropertyDeclaration>,
    /// Specificity of the selector (a, b, c, d format)
    pub specificity: (u32, u32, u32, u32),
}

impl ParsedRule {
    /// Create a new parsed rule
    pub fn new(selector: String) -> Self {
        ParsedRule {
            selector,
            declarations: Vec::new(),
            specificity: (0, 0, 0, 0),
        }
    }

    /// Set the specificity
    pub fn with_specificity(mut self, specificity: (u32, u32, u32, u32)) -> Self {
        self.specificity = specificity;
        self
    }

    /// Add a declaration to this rule
    pub fn add_declaration(&mut self, declaration: PropertyDeclaration) {
        self.declarations.push(declaration);
    }
}

/// A CSS property declaration
#[derive(Debug, Clone, PartialEq)]
pub struct PropertyDeclaration {
    /// Property name
    pub property: String,
    /// Property value
    pub value: String,
    /// Whether this declaration is marked !important
    pub important: bool,
}

impl PropertyDeclaration {
    /// Create a new property declaration
    pub fn new(property: impl Into<String>, value: impl Into<String>) -> Self {
        PropertyDeclaration {
            property: property.into(),
            value: value.into(),
            important: false,
        }
    }

    /// Mark as important
    pub fn with_important(mut self) -> Self {
        self.important = true;
        self
    }
}

/// Media query evaluator for responsive styles
#[derive(Debug, Clone)]
pub struct MediaQueryEvaluator {
    /// Current viewport information
    viewport: ViewportInfo,
    /// Cached evaluation results
    cache: FxHashMap<String, bool>,
}

impl MediaQueryEvaluator {
    /// Create a new media query evaluator
    pub fn new() -> Self {
        MediaQueryEvaluator {
            viewport: ViewportInfo::default(),
            cache: FxHashMap::default(),
        }
    }

    /// Create with specific viewport
    pub fn with_viewport(viewport: ViewportInfo) -> Self {
        MediaQueryEvaluator {
            viewport,
            cache: FxHashMap::default(),
        }
    }

    /// Update the viewport and clear cache
    pub fn update_viewport(&mut self, viewport: ViewportInfo) {
        self.viewport = viewport;
        self.cache.clear();
    }

    /// Get the current viewport
    pub fn viewport(&self) -> &ViewportInfo {
        &self.viewport
    }

    /// Evaluate a media query
    pub fn evaluate(&mut self, query: &str) -> bool {
        // Check cache first
        if let Some(&result) = self.cache.get(query) {
            return result;
        }

        // Simple media query evaluation (placeholder for full implementation)
        let result = self.evaluate_query(query);
        self.cache.insert(query.to_string(), result);
        result
    }

    /// Perform the actual media query evaluation
    fn evaluate_query(&self, query: &str) -> bool {
        let query = query.trim().to_lowercase();

        // Handle common media queries
        if query.is_empty() || query == "all" || query == "screen" {
            return true;
        }

        if query == "print" {
            return false;
        }

        // Simple width-based queries
        if query.contains("min-width") {
            if let Some(value) = self.extract_px_value(&query, "min-width") {
                return self.viewport.width >= value;
            }
        }

        if query.contains("max-width") {
            if let Some(value) = self.extract_px_value(&query, "max-width") {
                return self.viewport.width <= value;
            }
        }

        // Simple height-based queries
        if query.contains("min-height") {
            if let Some(value) = self.extract_px_value(&query, "min-height") {
                return self.viewport.height >= value;
            }
        }

        if query.contains("max-height") {
            if let Some(value) = self.extract_px_value(&query, "max-height") {
                return self.viewport.height <= value;
            }
        }

        // Preference queries
        if query.contains("prefers-color-scheme: dark") {
            return self.viewport.prefers_dark_mode;
        }

        if query.contains("prefers-color-scheme: light") {
            return !self.viewport.prefers_dark_mode;
        }

        if query.contains("prefers-reduced-motion: reduce") {
            return self.viewport.prefers_reduced_motion;
        }

        // Default: assume true for unknown queries
        true
    }

    /// Extract a pixel value from a media query
    fn extract_px_value(&self, query: &str, property: &str) -> Option<f32> {
        // Find property in query
        let start = query.find(property)?;
        let remaining = &query[start + property.len()..];

        // Find the colon and value
        let colon_pos = remaining.find(':')?;
        let after_colon = &remaining[colon_pos + 1..];

        // Extract numeric value
        let value_str: String = after_colon
            .chars()
            .take_while(|c| c.is_ascii_digit() || *c == '.' || c.is_whitespace())
            .collect();

        value_str.trim().parse().ok()
    }

    /// Clear the evaluation cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

impl Default for MediaQueryEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

/// Animation state
#[derive(Debug, Clone)]
pub struct Animation {
    /// Animation name
    pub name: String,
    /// Target element ID
    pub target_element: u64,
    /// Duration in milliseconds
    pub duration_ms: f64,
    /// Current progress (0.0 to 1.0)
    pub progress: f64,
    /// Start timestamp
    pub start_time: f64,
    /// Whether the animation is paused
    pub paused: bool,
    /// Iteration count (-1 for infinite)
    pub iteration_count: i32,
    /// Current iteration
    pub current_iteration: i32,
    /// Animation direction
    pub direction: AnimationDirection,
    /// Fill mode
    pub fill_mode: AnimationFillMode,
}

impl Animation {
    /// Create a new animation
    pub fn new(name: impl Into<String>, target_element: u64, duration_ms: f64) -> Self {
        Animation {
            name: name.into(),
            target_element,
            duration_ms,
            progress: 0.0,
            start_time: 0.0,
            paused: false,
            iteration_count: 1,
            current_iteration: 0,
            direction: AnimationDirection::Normal,
            fill_mode: AnimationFillMode::None,
        }
    }

    /// Set infinite iterations
    pub fn with_infinite(mut self) -> Self {
        self.iteration_count = -1;
        self
    }

    /// Set iteration count
    pub fn with_iterations(mut self, count: i32) -> Self {
        self.iteration_count = count;
        self
    }

    /// Set animation direction
    pub fn with_direction(mut self, direction: AnimationDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Set fill mode
    pub fn with_fill_mode(mut self, fill_mode: AnimationFillMode) -> Self {
        self.fill_mode = fill_mode;
        self
    }

    /// Start the animation at a given timestamp
    pub fn start(&mut self, timestamp: f64) {
        self.start_time = timestamp;
        self.progress = 0.0;
        self.current_iteration = 0;
        self.paused = false;
    }

    /// Update animation progress based on current timestamp
    pub fn tick(&mut self, timestamp: f64) -> bool {
        if self.paused || self.duration_ms <= 0.0 {
            return self.is_active();
        }

        let elapsed = timestamp - self.start_time;
        let iteration_progress = (elapsed % self.duration_ms) / self.duration_ms;
        let iterations_completed = (elapsed / self.duration_ms) as i32;

        self.current_iteration = iterations_completed;

        // Check if animation has completed
        if self.iteration_count >= 0 && iterations_completed >= self.iteration_count {
            self.progress = 1.0;
            return false; // Animation completed
        }

        // Calculate progress based on direction
        self.progress = match self.direction {
            AnimationDirection::Normal => iteration_progress,
            AnimationDirection::Reverse => 1.0 - iteration_progress,
            AnimationDirection::Alternate => {
                if iterations_completed % 2 == 0 {
                    iteration_progress
                } else {
                    1.0 - iteration_progress
                }
            }
            AnimationDirection::AlternateReverse => {
                if iterations_completed % 2 == 0 {
                    1.0 - iteration_progress
                } else {
                    iteration_progress
                }
            }
        };

        true // Animation still active
    }

    /// Check if animation is active
    pub fn is_active(&self) -> bool {
        if self.paused {
            return true; // Paused but not finished
        }
        if self.iteration_count < 0 {
            return true; // Infinite animation
        }
        self.current_iteration < self.iteration_count
    }

    /// Pause the animation
    pub fn pause(&mut self) {
        self.paused = true;
    }

    /// Resume the animation
    pub fn resume(&mut self, timestamp: f64) {
        if self.paused {
            // Adjust start time to maintain progress
            let elapsed = self.progress * self.duration_ms;
            self.start_time = timestamp - elapsed;
            self.paused = false;
        }
    }
}

/// Animation direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AnimationDirection {
    #[default]
    Normal,
    Reverse,
    Alternate,
    AlternateReverse,
}

/// Animation fill mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AnimationFillMode {
    #[default]
    None,
    Forwards,
    Backwards,
    Both,
}

/// CSS Engine metrics for monitoring
#[derive(Debug, Clone, Default)]
pub struct CssEngineMetrics {
    /// Number of stylesheets parsed
    pub stylesheets_parsed: usize,
    /// Number of style computations
    pub style_computations: usize,
    /// Number of cache hits
    pub cache_hits: usize,
    /// Number of cache misses
    pub cache_misses: usize,
    /// Number of style invalidations
    pub invalidations: usize,
    /// Total parse time in milliseconds
    pub total_parse_time_ms: f64,
    /// Total computation time in milliseconds
    pub total_compute_time_ms: f64,
}

impl CssEngineMetrics {
    /// Create new metrics
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a stylesheet parse
    pub fn record_parse(&mut self, duration_ms: f64) {
        self.stylesheets_parsed += 1;
        self.total_parse_time_ms += duration_ms;
    }

    /// Record a style computation
    pub fn record_computation(&mut self, duration_ms: f64) {
        self.style_computations += 1;
        self.total_compute_time_ms += duration_ms;
    }

    /// Record a cache hit
    pub fn record_cache_hit(&mut self) {
        self.cache_hits += 1;
    }

    /// Record a cache miss
    pub fn record_cache_miss(&mut self) {
        self.cache_misses += 1;
    }

    /// Record an invalidation
    pub fn record_invalidation(&mut self) {
        self.invalidations += 1;
    }

    /// Get the cache hit rate (0.0 to 1.0)
    pub fn cache_hit_rate(&self) -> f64 {
        let total = self.cache_hits + self.cache_misses;
        if total == 0 {
            0.0
        } else {
            self.cache_hits as f64 / total as f64
        }
    }

    /// Reset all metrics
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

/// Complete CSS Engine state
#[derive(Debug)]
pub struct CssEngineState {
    /// Parsed stylesheets
    stylesheets: Vec<ParsedStylesheet>,
    /// Inline styles for elements
    inline_styles: FxHashMap<u64, Vec<PropertyDeclaration>>,
    /// Media query evaluator
    media_evaluator: MediaQueryEvaluator,
    /// Active animations
    active_animations: Vec<Animation>,
    /// Custom CSS properties (CSS variables)
    custom_properties: FxHashMap<String, String>,
    /// Resource limits configuration
    config: ResourceLimits,
    /// Engine metrics
    metrics: CssEngineMetrics,
    /// Next stylesheet ID
    next_stylesheet_id: u32,
    /// Loaded fonts
    loaded_fonts: Vec<String>,
    /// Style cache
    style_cache: StyleCache,
}

impl CssEngineState {
    /// Create a new CSS engine state with the given configuration
    pub fn new(config: ResourceLimits) -> Self {
        CssEngineState {
            stylesheets: Vec::new(),
            inline_styles: FxHashMap::default(),
            media_evaluator: MediaQueryEvaluator::new(),
            active_animations: Vec::new(),
            custom_properties: FxHashMap::default(),
            config,
            metrics: CssEngineMetrics::new(),
            next_stylesheet_id: 0,
            loaded_fonts: Vec::new(),
            style_cache: StyleCache::new(),
        }
    }

    /// Add a stylesheet to the engine
    pub fn add_stylesheet(&mut self, mut stylesheet: ParsedStylesheet) -> StyleSheetId {
        let id = StyleSheetId::new(self.next_stylesheet_id);
        self.next_stylesheet_id += 1;
        stylesheet.id = id;
        self.stylesheets.push(stylesheet);
        self.metrics.record_parse(0.0); // Duration tracked elsewhere
        id
    }

    /// Remove a stylesheet by ID
    pub fn remove_stylesheet(&mut self, id: StyleSheetId) -> bool {
        let initial_len = self.stylesheets.len();
        self.stylesheets.retain(|s| s.id != id);
        let removed = self.stylesheets.len() < initial_len;
        if removed {
            // Invalidate cache when stylesheet is removed
            self.style_cache.clear();
        }
        removed
    }

    /// Get a stylesheet by ID
    pub fn get_stylesheet(&self, id: StyleSheetId) -> Option<&ParsedStylesheet> {
        self.stylesheets.iter().find(|s| s.id == id)
    }

    /// Get a mutable stylesheet by ID
    pub fn get_stylesheet_mut(&mut self, id: StyleSheetId) -> Option<&mut ParsedStylesheet> {
        self.stylesheets.iter_mut().find(|s| s.id == id)
    }

    /// Get all stylesheets
    pub fn stylesheets(&self) -> &[ParsedStylesheet] {
        &self.stylesheets
    }

    /// Get the number of stylesheets
    pub fn stylesheet_count(&self) -> usize {
        self.stylesheets.len()
    }

    /// Set inline style for an element
    pub fn set_inline_style(&mut self, element_id: u64, declarations: Vec<PropertyDeclaration>) {
        self.inline_styles.insert(element_id, declarations);
        // Invalidate cache for this element
        self.style_cache.invalidate(ElementId::new(element_id));
    }

    /// Get inline style for an element
    pub fn get_inline_style(&self, element_id: u64) -> Option<&Vec<PropertyDeclaration>> {
        self.inline_styles.get(&element_id)
    }

    /// Remove inline style for an element
    pub fn remove_inline_style(&mut self, element_id: u64) -> bool {
        let removed = self.inline_styles.remove(&element_id).is_some();
        if removed {
            self.style_cache.invalidate(ElementId::new(element_id));
        }
        removed
    }

    /// Update media query viewport
    pub fn update_media(&mut self, viewport: ViewportInfo) {
        self.media_evaluator.update_viewport(viewport);
        // Invalidate all cached styles as media queries may have changed
        self.style_cache.clear();
    }

    /// Get the current viewport
    pub fn viewport(&self) -> &ViewportInfo {
        self.media_evaluator.viewport()
    }

    /// Evaluate a media query
    pub fn evaluate_media_query(&mut self, query: &str) -> bool {
        self.media_evaluator.evaluate(query)
    }

    /// Add an animation
    pub fn add_animation(&mut self, animation: Animation) {
        self.active_animations.push(animation);
    }

    /// Tick all animations
    pub fn tick_animations(&mut self, timestamp: f64) -> usize {
        // Update all animations and remove completed ones
        self.active_animations.retain_mut(|anim| anim.tick(timestamp));
        self.active_animations.len()
    }

    /// Get active animations
    pub fn active_animations(&self) -> &[Animation] {
        &self.active_animations
    }

    /// Get active animations for a specific element
    pub fn animations_for_element(&self, element_id: u64) -> Vec<&Animation> {
        self.active_animations
            .iter()
            .filter(|a| a.target_element == element_id)
            .collect()
    }

    /// Pause all animations
    pub fn pause_all_animations(&mut self) {
        for anim in &mut self.active_animations {
            anim.pause();
        }
    }

    /// Resume all animations
    pub fn resume_all_animations(&mut self, timestamp: f64) {
        for anim in &mut self.active_animations {
            anim.resume(timestamp);
        }
    }

    /// Set a custom property (CSS variable)
    pub fn set_custom_property(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.custom_properties.insert(name.into(), value.into());
    }

    /// Get a custom property
    pub fn get_custom_property(&self, name: &str) -> Option<&String> {
        self.custom_properties.get(name)
    }

    /// Remove a custom property
    pub fn remove_custom_property(&mut self, name: &str) -> bool {
        self.custom_properties.remove(name).is_some()
    }

    /// Get all custom properties
    pub fn custom_properties(&self) -> &FxHashMap<String, String> {
        &self.custom_properties
    }

    /// Register a loaded font
    pub fn register_font(&mut self, family: String) {
        if !self.loaded_fonts.contains(&family) {
            self.loaded_fonts.push(family);
        }
    }

    /// Check if a font is loaded
    pub fn is_font_loaded(&self, family: &str) -> bool {
        self.loaded_fonts.iter().any(|f| f == family)
    }

    /// Get loaded fonts
    pub fn loaded_fonts(&self) -> &[String] {
        &self.loaded_fonts
    }

    /// Get the resource limits configuration
    pub fn config(&self) -> &ResourceLimits {
        &self.config
    }

    /// Get the metrics
    pub fn metrics(&self) -> &CssEngineMetrics {
        &self.metrics
    }

    /// Get mutable metrics
    pub fn metrics_mut(&mut self) -> &mut CssEngineMetrics {
        &mut self.metrics
    }

    /// Get the style cache
    pub fn style_cache(&self) -> &StyleCache {
        &self.style_cache
    }

    /// Get mutable style cache
    pub fn style_cache_mut(&mut self) -> &mut StyleCache {
        &mut self.style_cache
    }

    /// Clear all caches
    pub fn clear_caches(&mut self) {
        self.style_cache.clear();
        self.media_evaluator.clear_cache();
    }

    /// Reset the entire state
    pub fn reset(&mut self) {
        self.stylesheets.clear();
        self.inline_styles.clear();
        self.media_evaluator = MediaQueryEvaluator::new();
        self.active_animations.clear();
        self.custom_properties.clear();
        self.next_stylesheet_id = 0;
        self.loaded_fonts.clear();
        self.style_cache.clear();
        self.metrics.reset();
    }
}

impl Default for CssEngineState {
    fn default() -> Self {
        Self::new(ResourceLimits::default())
    }
}

// ============================================================================
// Additional tests for Phase 2 types
// ============================================================================

#[cfg(test)]
mod phase2_tests {
    use super::*;

    // ParsedStylesheet tests
    #[test]
    fn test_parsed_stylesheet_new() {
        let stylesheet = ParsedStylesheet::new("body { }".to_string());
        assert_eq!(stylesheet.source, "body { }");
        assert!(stylesheet.enabled);
        assert!(stylesheet.source_url.is_none());
    }

    #[test]
    fn test_parsed_stylesheet_builder() {
        let stylesheet = ParsedStylesheet::new("body { }".to_string())
            .with_id(StyleSheetId::new(5))
            .with_source_url("test.css".to_string())
            .with_media("screen".to_string());

        assert_eq!(stylesheet.id.0, 5);
        assert_eq!(stylesheet.source_url.as_deref(), Some("test.css"));
        assert_eq!(stylesheet.media.as_deref(), Some("screen"));
    }

    #[test]
    fn test_parsed_stylesheet_rules() {
        let mut stylesheet = ParsedStylesheet::new("".to_string());
        let rule = ParsedRule::new("body".to_string());
        stylesheet.add_rule(rule);
        assert_eq!(stylesheet.rules.len(), 1);
    }

    // PropertyDeclaration tests
    #[test]
    fn test_property_declaration_new() {
        let decl = PropertyDeclaration::new("color", "red");
        assert_eq!(decl.property, "color");
        assert_eq!(decl.value, "red");
        assert!(!decl.important);
    }

    #[test]
    fn test_property_declaration_important() {
        let decl = PropertyDeclaration::new("color", "red").with_important();
        assert!(decl.important);
    }

    // MediaQueryEvaluator tests
    #[test]
    fn test_media_query_evaluator_default() {
        let mut evaluator = MediaQueryEvaluator::new();
        assert!(evaluator.evaluate("all"));
        assert!(evaluator.evaluate("screen"));
        assert!(!evaluator.evaluate("print"));
    }

    #[test]
    fn test_media_query_min_width() {
        let mut evaluator = MediaQueryEvaluator::with_viewport(ViewportInfo::new(800.0, 600.0));
        assert!(evaluator.evaluate("(min-width: 600)"));
        assert!(evaluator.evaluate("(min-width: 800)"));
        assert!(!evaluator.evaluate("(min-width: 801)"));
    }

    #[test]
    fn test_media_query_max_width() {
        let mut evaluator = MediaQueryEvaluator::with_viewport(ViewportInfo::new(800.0, 600.0));
        assert!(evaluator.evaluate("(max-width: 1000)"));
        assert!(evaluator.evaluate("(max-width: 800)"));
        assert!(!evaluator.evaluate("(max-width: 799)"));
    }

    #[test]
    fn test_media_query_prefers_dark() {
        let viewport = ViewportInfo::default().with_dark_mode();
        let mut evaluator = MediaQueryEvaluator::with_viewport(viewport);
        assert!(evaluator.evaluate("(prefers-color-scheme: dark)"));
        assert!(!evaluator.evaluate("(prefers-color-scheme: light)"));
    }

    #[test]
    fn test_media_query_caching() {
        let mut evaluator = MediaQueryEvaluator::new();

        // First evaluation
        let result1 = evaluator.evaluate("(min-width: 500)");
        // Second evaluation (should use cache)
        let result2 = evaluator.evaluate("(min-width: 500)");

        assert_eq!(result1, result2);
    }

    #[test]
    fn test_media_query_viewport_update() {
        let mut evaluator = MediaQueryEvaluator::with_viewport(ViewportInfo::new(400.0, 300.0));
        assert!(!evaluator.evaluate("(min-width: 500)"));

        evaluator.update_viewport(ViewportInfo::new(800.0, 600.0));
        assert!(evaluator.evaluate("(min-width: 500)"));
    }

    // Animation tests
    #[test]
    fn test_animation_new() {
        let anim = Animation::new("fade-in", 42, 1000.0);
        assert_eq!(anim.name, "fade-in");
        assert_eq!(anim.target_element, 42);
        assert_eq!(anim.duration_ms, 1000.0);
        assert_eq!(anim.progress, 0.0);
    }

    #[test]
    fn test_animation_start_and_tick() {
        let mut anim = Animation::new("fade-in", 1, 1000.0);
        anim.start(0.0);

        // Tick at 500ms (50% progress)
        assert!(anim.tick(500.0));
        assert!((anim.progress - 0.5).abs() < 0.01);

        // Tick at 1000ms (100% progress, animation ends)
        assert!(!anim.tick(1000.0));
    }

    #[test]
    fn test_animation_infinite() {
        let mut anim = Animation::new("spin", 1, 1000.0).with_infinite();
        anim.start(0.0);

        // Should still be active after many iterations
        assert!(anim.tick(5500.0));
        assert!(anim.is_active());
    }

    #[test]
    fn test_animation_alternate() {
        let mut anim = Animation::new("bounce", 1, 1000.0)
            .with_iterations(2)
            .with_direction(AnimationDirection::Alternate);
        anim.start(0.0);

        // First iteration: normal direction
        anim.tick(500.0);
        let progress1 = anim.progress;

        // Second iteration: reverse direction
        anim.tick(1500.0);
        let progress2 = anim.progress;

        // In alternate mode, second iteration should be reversed
        assert!((progress1 - 0.5).abs() < 0.01);
        assert!((progress2 - 0.5).abs() < 0.01); // 500ms into reverse is 0.5
    }

    #[test]
    fn test_animation_pause_resume() {
        let mut anim = Animation::new("fade", 1, 1000.0);
        anim.start(0.0);
        anim.tick(500.0);

        anim.pause();
        assert!(anim.paused);

        anim.resume(600.0);
        assert!(!anim.paused);
    }

    // CssEngineMetrics tests
    #[test]
    fn test_metrics_new() {
        let metrics = CssEngineMetrics::new();
        assert_eq!(metrics.stylesheets_parsed, 0);
        assert_eq!(metrics.cache_hit_rate(), 0.0);
    }

    #[test]
    fn test_metrics_recording() {
        let mut metrics = CssEngineMetrics::new();

        metrics.record_parse(10.0);
        assert_eq!(metrics.stylesheets_parsed, 1);

        metrics.record_cache_hit();
        metrics.record_cache_hit();
        metrics.record_cache_miss();

        // 2 hits out of 3 total = 0.666...
        assert!((metrics.cache_hit_rate() - 0.666).abs() < 0.01);
    }

    #[test]
    fn test_metrics_reset() {
        let mut metrics = CssEngineMetrics::new();
        metrics.record_parse(10.0);
        metrics.record_computation(5.0);

        metrics.reset();

        assert_eq!(metrics.stylesheets_parsed, 0);
        assert_eq!(metrics.style_computations, 0);
    }

    // CssEngineState tests
    #[test]
    fn test_css_engine_state_new() {
        let state = CssEngineState::new(ResourceLimits::default());
        assert_eq!(state.stylesheet_count(), 0);
        assert!(state.active_animations().is_empty());
    }

    #[test]
    fn test_css_engine_state_add_stylesheet() {
        let mut state = CssEngineState::default();

        let stylesheet = ParsedStylesheet::new("body { }".to_string());
        let id = state.add_stylesheet(stylesheet);

        assert_eq!(id.0, 0);
        assert_eq!(state.stylesheet_count(), 1);
        assert!(state.get_stylesheet(id).is_some());
    }

    #[test]
    fn test_css_engine_state_remove_stylesheet() {
        let mut state = CssEngineState::default();

        let id1 = state.add_stylesheet(ParsedStylesheet::new("a { }".to_string()));
        let id2 = state.add_stylesheet(ParsedStylesheet::new("b { }".to_string()));

        assert_eq!(state.stylesheet_count(), 2);

        assert!(state.remove_stylesheet(id1));
        assert_eq!(state.stylesheet_count(), 1);
        assert!(state.get_stylesheet(id1).is_none());
        assert!(state.get_stylesheet(id2).is_some());
    }

    #[test]
    fn test_css_engine_state_inline_styles() {
        let mut state = CssEngineState::default();

        let decls = vec![
            PropertyDeclaration::new("color", "red"),
            PropertyDeclaration::new("font-size", "16px"),
        ];

        state.set_inline_style(42, decls);

        let retrieved = state.get_inline_style(42);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().len(), 2);

        assert!(state.remove_inline_style(42));
        assert!(state.get_inline_style(42).is_none());
    }

    #[test]
    fn test_css_engine_state_media() {
        let mut state = CssEngineState::default();

        state.update_media(ViewportInfo::mobile());
        assert_eq!(state.viewport().width, 375.0);

        assert!(!state.evaluate_media_query("(min-width: 500)"));

        state.update_media(ViewportInfo::desktop());
        assert!(state.evaluate_media_query("(min-width: 500)"));
    }

    #[test]
    fn test_css_engine_state_animations() {
        let mut state = CssEngineState::default();

        let mut anim = Animation::new("fade", 1, 1000.0);
        anim.start(0.0);
        state.add_animation(anim);

        assert_eq!(state.active_animations().len(), 1);

        // Tick past animation end
        let active_count = state.tick_animations(1500.0);
        assert_eq!(active_count, 0);
        assert!(state.active_animations().is_empty());
    }

    #[test]
    fn test_css_engine_state_custom_properties() {
        let mut state = CssEngineState::default();

        state.set_custom_property("--primary-color", "#007bff");
        state.set_custom_property("--spacing", "8px");

        assert_eq!(
            state.get_custom_property("--primary-color"),
            Some(&"#007bff".to_string())
        );

        assert!(state.remove_custom_property("--spacing"));
        assert!(state.get_custom_property("--spacing").is_none());
    }

    #[test]
    fn test_css_engine_state_fonts() {
        let mut state = CssEngineState::default();

        state.register_font("Roboto".to_string());
        state.register_font("Open Sans".to_string());

        assert!(state.is_font_loaded("Roboto"));
        assert!(state.is_font_loaded("Open Sans"));
        assert!(!state.is_font_loaded("Arial"));

        // Duplicate registration should not add again
        state.register_font("Roboto".to_string());
        assert_eq!(state.loaded_fonts().len(), 2);
    }

    #[test]
    fn test_css_engine_state_reset() {
        let mut state = CssEngineState::default();

        state.add_stylesheet(ParsedStylesheet::new("body { }".to_string()));
        state.set_inline_style(1, vec![PropertyDeclaration::new("color", "red")]);
        state.set_custom_property("--x", "1");

        state.reset();

        assert_eq!(state.stylesheet_count(), 0);
        assert!(state.get_inline_style(1).is_none());
        assert!(state.get_custom_property("--x").is_none());
    }

    #[test]
    fn test_css_engine_state_animations_for_element() {
        let mut state = CssEngineState::default();

        let mut anim1 = Animation::new("fade1", 1, 1000.0);
        anim1.start(0.0);
        let mut anim2 = Animation::new("fade2", 2, 1000.0);
        anim2.start(0.0);
        let mut anim3 = Animation::new("fade3", 1, 1000.0);
        anim3.start(0.0);

        state.add_animation(anim1);
        state.add_animation(anim2);
        state.add_animation(anim3);

        let element1_anims = state.animations_for_element(1);
        assert_eq!(element1_anims.len(), 2);

        let element2_anims = state.animations_for_element(2);
        assert_eq!(element2_anims.len(), 1);
    }

    #[test]
    fn test_css_engine_state_pause_resume_animations() {
        let mut state = CssEngineState::default();

        let mut anim = Animation::new("fade", 1, 1000.0);
        anim.start(0.0);
        state.add_animation(anim);

        state.pause_all_animations();
        assert!(state.active_animations()[0].paused);

        state.resume_all_animations(100.0);
        assert!(!state.active_animations()[0].paused);
    }
}
