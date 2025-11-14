//! Main CSS Engine implementation

use crate::error::{CssError, ElementId, StyleSheetId};
use crate::state::{EngineConfig, StyleCache, Stylesheet, StylesheetRegistry};
use crate::types::{ComputedStyle, DomNode, StyleInvalidation, StyleNode, StyleTree};

/// Main CSS Engine
#[derive(Debug)]
pub struct CssEngine {
    /// Configuration (will be used for advanced features)
    #[allow(dead_code)]
    config: EngineConfig,
    /// Stylesheet registry
    stylesheets: StylesheetRegistry,
    /// Inline styles for elements
    inline_styles: fxhash::FxHashMap<ElementId, String>,
    /// Computed style cache
    cache: StyleCache,
}

impl CssEngine {
    /// Create a new CSS engine instance
    pub fn new() -> Self {
        CssEngine {
            config: EngineConfig::new(),
            stylesheets: StylesheetRegistry::new(),
            inline_styles: fxhash::FxHashMap::default(),
            cache: StyleCache::new(),
        }
    }

    /// Create a new CSS engine with custom configuration
    pub fn with_config(config: EngineConfig) -> Self {
        CssEngine {
            config,
            stylesheets: StylesheetRegistry::new(),
            inline_styles: fxhash::FxHashMap::default(),
            cache: StyleCache::new(),
        }
    }

    /// Parse a CSS stylesheet from string
    ///
    /// # Arguments
    /// * `css` - CSS source code
    /// * `source_url` - Optional URL for error reporting
    ///
    /// # Returns
    /// StyleSheetId of the parsed stylesheet
    ///
    /// # Errors
    /// Returns `CssError::ParseError` if parsing fails
    pub fn parse_stylesheet(
        &mut self,
        css: &str,
        source_url: Option<&str>,
    ) -> Result<StyleSheetId, CssError> {
        // Basic validation
        if css.is_empty() {
            return Err(CssError::ParseError {
                line: 0,
                column: 0,
                message: "Empty stylesheet".to_string(),
            });
        }

        // Create stylesheet (actual parsing will be implemented later)
        let stylesheet = Stylesheet::new(css.to_string(), source_url.map(|s| s.to_string()));

        // Register and return ID
        let id = self.stylesheets.register(stylesheet);
        Ok(id)
    }

    /// Set inline style for an element
    ///
    /// # Arguments
    /// * `element_id` - Element ID
    /// * `style` - CSS style string
    ///
    /// # Errors
    /// Returns `CssError::ParseError` if style is invalid
    pub fn set_inline_style(&mut self, element_id: ElementId, style: &str) -> Result<(), CssError> {
        // Validate style is not empty
        if style.is_empty() {
            return Err(CssError::InvalidValue {
                property: "style".to_string(),
                value: "empty".to_string(),
            });
        }

        // Store inline style
        self.inline_styles.insert(element_id, style.to_string());

        // Invalidate cache for this element
        self.cache.invalidate(element_id);

        Ok(())
    }

    /// Compute styles for a DOM tree
    ///
    /// # Arguments
    /// * `dom_root` - Root of the DOM tree
    ///
    /// # Returns
    /// StyleTree with computed styles
    ///
    /// # Errors
    /// Returns `CssError::ComputationError` if computation fails
    pub fn compute_styles(&mut self, dom_root: &DomNode) -> Result<StyleTree, CssError> {
        // Recursively compute styles for the tree
        let root_node = self.compute_node_style(dom_root)?;

        Ok(StyleTree { root: root_node })
    }

    /// Compute style for a single node and its children
    fn compute_node_style(&mut self, node: &DomNode) -> Result<StyleNode, CssError> {
        // Get or compute style for this element
        let computed_style = self.compute_element_style(node)?;

        // Cache the computed style
        self.cache.insert(node.id, computed_style.clone());

        // Recursively compute styles for children
        let mut children = Vec::new();
        for child in &node.children {
            let child_node = self.compute_node_style(child)?;
            children.push(child_node);
        }

        Ok(StyleNode {
            element_id: node.id,
            computed_style,
            children,
        })
    }

    /// Compute style for a single element
    fn compute_element_style(&self, node: &DomNode) -> Result<ComputedStyle, CssError> {
        // Check cache first
        if let Some(cached) = self.cache.get(node.id) {
            return Ok(cached.clone());
        }

        // Start with default style
        let mut style = ComputedStyle::default();

        // TODO: Apply cascade algorithm
        // 1. Apply user-agent stylesheet
        // 2. Apply author stylesheets
        // 3. Apply inline styles
        // 4. Apply inheritance

        // For now, just check for inline styles
        if let Some(inline_style) = self.inline_styles.get(&node.id) {
            // Parse and apply inline style
            // This is a simplified version - actual implementation will use parser
            self.apply_inline_style(&mut style, inline_style)?;
        }

        Ok(style)
    }

    /// Apply inline style to computed style
    fn apply_inline_style(
        &self,
        _style: &mut ComputedStyle,
        _inline_css: &str,
    ) -> Result<(), CssError> {
        // Simplified implementation - actual parsing will come from css_parser_core
        // For now, just validate it's not empty
        Ok(())
    }

    /// Get computed style for specific element
    ///
    /// # Arguments
    /// * `element_id` - Element ID
    ///
    /// # Returns
    /// ComputedStyle for the element
    ///
    /// # Errors
    /// Returns `CssError::ElementNotFound` if element is not in cache
    pub fn get_computed_style(&self, element_id: ElementId) -> Result<ComputedStyle, CssError> {
        self.cache
            .get(element_id)
            .cloned()
            .ok_or(CssError::ElementNotFound { element_id })
    }

    /// Invalidate styles for incremental recomputation
    ///
    /// # Arguments
    /// * `invalidation` - Type of invalidation
    ///
    /// # Errors
    /// Returns error if invalidation fails
    pub fn invalidate_styles(&mut self, invalidation: StyleInvalidation) -> Result<(), CssError> {
        match invalidation {
            StyleInvalidation::AttributeChange { element_id, .. } => {
                self.cache.invalidate(element_id);
            }
            StyleInvalidation::ClassChange { element_id, .. } => {
                self.cache.invalidate(element_id);
            }
            StyleInvalidation::ElementInserted { element_id, .. } => {
                self.cache.invalidate(element_id);
            }
            StyleInvalidation::ElementRemoved { element_id } => {
                self.cache.invalidate(element_id);
            }
        }
        Ok(())
    }

    /// Get number of cached styles
    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }

    /// Get number of registered stylesheets
    pub fn stylesheet_count(&self) -> usize {
        self.stylesheets.len()
    }

    /// Clear all caches
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

impl Default for CssEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation() {
        let engine = CssEngine::new();
        assert_eq!(engine.stylesheet_count(), 0);
        assert_eq!(engine.cache_size(), 0);
    }

    #[test]
    fn test_parse_empty_stylesheet_fails() {
        let mut engine = CssEngine::new();
        let result = engine.parse_stylesheet("", None);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_simple_stylesheet() {
        let mut engine = CssEngine::new();
        let css = "body { color: red; }";
        let result = engine.parse_stylesheet(css, None);

        assert!(result.is_ok());
        assert_eq!(engine.stylesheet_count(), 1);
    }

    #[test]
    fn test_parse_multiple_stylesheets() {
        let mut engine = CssEngine::new();

        let id1 = engine
            .parse_stylesheet("body { color: red; }", None)
            .unwrap();
        let id2 = engine
            .parse_stylesheet("div { margin: 10px; }", None)
            .unwrap();

        assert_ne!(id1, id2);
        assert_eq!(engine.stylesheet_count(), 2);
    }

    #[test]
    fn test_set_inline_style() {
        let mut engine = CssEngine::new();
        let element_id = ElementId::new(1);

        let result = engine.set_inline_style(element_id, "color: blue;");
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_empty_inline_style_fails() {
        let mut engine = CssEngine::new();
        let element_id = ElementId::new(1);

        let result = engine.set_inline_style(element_id, "");
        assert!(result.is_err());
    }

    #[test]
    fn test_compute_styles_simple_tree() {
        let mut engine = CssEngine::new();

        // Create a simple DOM tree
        let dom_root = DomNode::new(ElementId::new(1), "div");

        let result = engine.compute_styles(&dom_root);
        assert!(result.is_ok());

        let style_tree = result.unwrap();
        assert_eq!(style_tree.root.element_id, ElementId::new(1));
    }

    #[test]
    fn test_compute_styles_with_children() {
        let mut engine = CssEngine::new();

        // Create a DOM tree with children
        let child1 = DomNode::new(ElementId::new(2), "span");
        let child2 = DomNode::new(ElementId::new(3), "span");

        let dom_root = DomNode::new(ElementId::new(1), "div")
            .with_child(child1)
            .with_child(child2);

        let result = engine.compute_styles(&dom_root);
        assert!(result.is_ok());

        let style_tree = result.unwrap();
        assert_eq!(style_tree.root.children.len(), 2);
    }

    #[test]
    fn test_get_computed_style() {
        let mut engine = CssEngine::new();
        let dom_root = DomNode::new(ElementId::new(1), "div");

        // Compute styles first
        engine.compute_styles(&dom_root).unwrap();

        // Now get the computed style
        let result = engine.get_computed_style(ElementId::new(1));
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_computed_style_not_found() {
        let engine = CssEngine::new();
        let result = engine.get_computed_style(ElementId::new(999));
        assert!(result.is_err());
    }

    #[test]
    fn test_invalidate_styles() {
        let mut engine = CssEngine::new();
        let element_id = ElementId::new(1);

        // Compute and cache a style
        let dom_root = DomNode::new(element_id, "div");
        engine.compute_styles(&dom_root).unwrap();
        assert_eq!(engine.cache_size(), 1);

        // Invalidate it
        let invalidation = StyleInvalidation::AttributeChange {
            element_id,
            attr: "class".to_string(),
        };
        engine.invalidate_styles(invalidation).unwrap();

        // Cache should be cleared for that element
        assert_eq!(engine.cache_size(), 0);
    }

    #[test]
    fn test_cache_persistence() {
        let mut engine = CssEngine::new();
        let dom_root = DomNode::new(ElementId::new(1), "div");

        // First computation
        engine.compute_styles(&dom_root).unwrap();
        let size_after_first = engine.cache_size();

        // Second computation should use cache
        engine.compute_styles(&dom_root).unwrap();
        let size_after_second = engine.cache_size();

        assert_eq!(size_after_first, size_after_second);
    }

    #[test]
    fn test_clear_cache() {
        let mut engine = CssEngine::new();
        let dom_root = DomNode::new(ElementId::new(1), "div");

        engine.compute_styles(&dom_root).unwrap();
        assert!(engine.cache_size() > 0);

        engine.clear_cache();
        assert_eq!(engine.cache_size(), 0);
    }
}
