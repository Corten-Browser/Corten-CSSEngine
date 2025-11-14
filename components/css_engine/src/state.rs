//! State management for the CSS Engine

use crate::error::{ElementId, StyleSheetId};
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
