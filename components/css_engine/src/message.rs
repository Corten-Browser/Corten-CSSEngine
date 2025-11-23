//! Message-based interface for the CSS Engine
//!
//! This module provides a message-passing interface for interacting with the CSS engine,
//! enabling asynchronous and event-driven usage patterns.
//!
//! # Example
//!
//! ```
//! use css_engine::message::{CssMessage, CssResponse, MessageHandler, StylesheetId};
//! use css_engine::BasicCssEngine;
//!
//! let mut engine = BasicCssEngine::new();
//!
//! // Parse a stylesheet via message
//! let response = engine.handle(CssMessage::ParseStylesheet {
//!     id: StylesheetId::new(0),
//!     content: "body { color: red; }".to_string(),
//! });
//! ```

use crate::error::{CssError, StyleSheetId};
use crate::types::StyleTree;

/// Unique identifier for a stylesheet
pub type StylesheetId = StyleSheetId;

/// Viewport information for media query evaluation
#[derive(Debug, Clone, PartialEq)]
pub struct ViewportInfo {
    /// Width in pixels
    pub width: f32,
    /// Height in pixels
    pub height: f32,
    /// Device pixel ratio
    pub device_pixel_ratio: f32,
    /// Color depth (bits per pixel)
    pub color_depth: u8,
    /// Whether the device prefers dark mode
    pub prefers_dark_mode: bool,
    /// Whether reduced motion is preferred
    pub prefers_reduced_motion: bool,
}

impl Default for ViewportInfo {
    fn default() -> Self {
        ViewportInfo {
            width: 1920.0,
            height: 1080.0,
            device_pixel_ratio: 1.0,
            color_depth: 24,
            prefers_dark_mode: false,
            prefers_reduced_motion: false,
        }
    }
}

impl ViewportInfo {
    /// Create a new viewport with specified dimensions
    pub fn new(width: f32, height: f32) -> Self {
        ViewportInfo {
            width,
            height,
            ..Default::default()
        }
    }

    /// Create a mobile viewport (375x667)
    pub fn mobile() -> Self {
        ViewportInfo::new(375.0, 667.0)
    }

    /// Create a tablet viewport (768x1024)
    pub fn tablet() -> Self {
        ViewportInfo::new(768.0, 1024.0)
    }

    /// Create a desktop viewport (1920x1080)
    pub fn desktop() -> Self {
        ViewportInfo::new(1920.0, 1080.0)
    }

    /// Set device pixel ratio
    pub fn with_dpr(mut self, dpr: f32) -> Self {
        self.device_pixel_ratio = dpr;
        self
    }

    /// Enable dark mode preference
    pub fn with_dark_mode(mut self) -> Self {
        self.prefers_dark_mode = true;
        self
    }

    /// Enable reduced motion preference
    pub fn with_reduced_motion(mut self) -> Self {
        self.prefers_reduced_motion = true;
        self
    }
}

/// Scope of style invalidation
#[derive(Debug, Clone, PartialEq)]
pub enum InvalidationScope {
    /// Invalidate styles for a specific element and its descendants
    Subtree {
        /// Root element ID for the subtree
        root_id: u64,
    },
    /// Invalidate styles matching specific selectors
    Selectors {
        /// Selector patterns to match
        patterns: Vec<String>,
    },
    /// Invalidate all styles (full restyle)
    All,
    /// Invalidate only elements with specific classes
    Classes {
        /// Class names to match
        class_names: Vec<String>,
    },
    /// Invalidate only elements with specific attributes
    Attributes {
        /// Attribute names to match
        attr_names: Vec<String>,
    },
}

impl InvalidationScope {
    /// Create a subtree invalidation scope
    pub fn subtree(root_id: u64) -> Self {
        InvalidationScope::Subtree { root_id }
    }

    /// Create a selector-based invalidation scope
    pub fn selectors(patterns: Vec<String>) -> Self {
        InvalidationScope::Selectors { patterns }
    }

    /// Create a class-based invalidation scope
    pub fn classes(class_names: Vec<String>) -> Self {
        InvalidationScope::Classes { class_names }
    }

    /// Create an attribute-based invalidation scope
    pub fn attributes(attr_names: Vec<String>) -> Self {
        InvalidationScope::Attributes { attr_names }
    }
}

/// Messages that can be sent to the CSS engine
#[derive(Debug, Clone)]
pub enum CssMessage {
    /// Parse a new stylesheet
    ParseStylesheet {
        /// Stylesheet identifier
        id: StylesheetId,
        /// CSS content to parse
        content: String,
    },
    /// Compute styles for a DOM tree
    ComputeStyles {
        /// Root element ID of the DOM tree
        dom_root: u64,
    },
    /// Invalidate computed styles
    InvalidateStyles {
        /// Scope of invalidation
        scope: InvalidationScope,
    },
    /// Handle media query environment change
    MediaQueryChange {
        /// New viewport information
        viewport: ViewportInfo,
    },
    /// Animation tick for updating animations
    AnimationTick {
        /// Current timestamp in milliseconds
        timestamp: f64,
    },
    /// Font has finished loading
    FontLoaded {
        /// Font family name
        family: String,
    },
    /// Request engine shutdown
    Shutdown,
}

/// Responses from the CSS engine
#[derive(Debug)]
pub enum CssResponse {
    /// Stylesheet was parsed
    StylesheetParsed {
        /// Stylesheet identifier
        id: StylesheetId,
        /// Parse error if any
        error: Option<CssError>,
    },
    /// Styles were computed
    StylesComputed {
        /// Computed style tree
        tree: StyleTree,
    },
    /// Styles were invalidated
    StylesInvalidated {
        /// Number of affected elements
        affected_count: usize,
    },
    /// Media queries were updated
    MediaQueriesUpdated,
    /// Animations were updated
    AnimationUpdated {
        /// Number of active animations
        active_animations: usize,
    },
    /// Font was processed
    FontProcessed {
        /// Font family that was processed
        family: String,
        /// Whether any styles need recomputation
        requires_restyle: bool,
    },
    /// Engine is ready
    Ready,
    /// An error occurred
    Error(CssError),
}

/// Trait for handling CSS engine messages
pub trait MessageHandler {
    /// Handle a CSS engine message and return a response
    ///
    /// # Arguments
    /// * `message` - The message to handle
    ///
    /// # Returns
    /// A response indicating the result of handling the message
    fn handle(&mut self, message: CssMessage) -> CssResponse;

    /// Check if the engine is ready to handle messages
    fn is_ready(&self) -> bool {
        true
    }

    /// Get the number of pending operations
    fn pending_count(&self) -> usize {
        0
    }
}

// ============================================================================
// BasicCssEngine - A MessageHandler implementation
// ============================================================================

use crate::config::ResourceLimits;
use crate::state::{CssEngineState, ParsedStylesheet, PropertyDeclaration};
use crate::types::{ComputedStyle, DomNode, StyleNode};

/// A basic CSS engine implementation with message handling
#[derive(Debug)]
pub struct BasicCssEngine {
    /// Internal engine state
    state: CssEngineState,
    /// Whether the engine is running
    running: bool,
    /// Pending style computations
    pending_computations: usize,
}

impl BasicCssEngine {
    /// Create a new BasicCssEngine with default configuration
    pub fn new() -> Self {
        BasicCssEngine {
            state: CssEngineState::default(),
            running: true,
            pending_computations: 0,
        }
    }

    /// Create a new BasicCssEngine with custom resource limits
    pub fn with_config(limits: ResourceLimits) -> Self {
        BasicCssEngine {
            state: CssEngineState::new(limits),
            running: true,
            pending_computations: 0,
        }
    }

    /// Get a reference to the internal state
    pub fn state(&self) -> &CssEngineState {
        &self.state
    }

    /// Get a mutable reference to the internal state
    pub fn state_mut(&mut self) -> &mut CssEngineState {
        &mut self.state
    }

    /// Check if the engine is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Handle stylesheet parsing
    fn handle_parse_stylesheet(&mut self, id: StylesheetId, content: String) -> CssResponse {
        // Check size limits
        let limits = self.state.config();
        if content.len() > limits.max_stylesheet_size {
            return CssResponse::StylesheetParsed {
                id,
                error: Some(CssError::ParseError {
                    line: 0,
                    column: 0,
                    message: format!(
                        "Stylesheet size {} exceeds maximum {}",
                        content.len(),
                        limits.max_stylesheet_size
                    ),
                }),
            };
        }

        // Create and parse stylesheet
        let stylesheet = ParsedStylesheet::new(content);
        let assigned_id = self.state.add_stylesheet(stylesheet);

        // Record metrics
        self.state.metrics_mut().record_parse(0.0);

        CssResponse::StylesheetParsed {
            id: assigned_id,
            error: None,
        }
    }

    /// Handle style computation
    fn handle_compute_styles(&mut self, dom_root: u64) -> CssResponse {
        // Create a simple DOM node for the root
        let dom = DomNode::new(crate::error::ElementId::new(dom_root), "root");

        // Build style tree
        match self.compute_style_tree(&dom) {
            Ok(tree) => {
                self.state.metrics_mut().record_computation(0.0);
                CssResponse::StylesComputed { tree }
            }
            Err(err) => CssResponse::Error(err),
        }
    }

    /// Compute style tree recursively
    fn compute_style_tree(&mut self, dom: &DomNode) -> Result<StyleTree, CssError> {
        let root_node = self.compute_node_style(dom)?;
        Ok(StyleTree { root: root_node })
    }

    /// Compute style for a single node
    fn compute_node_style(&mut self, dom: &DomNode) -> Result<StyleNode, CssError> {
        // Check cache - need to clone to avoid borrow conflict
        let cached_style = self.state.style_cache().get(dom.id).cloned();

        if let Some(cached) = cached_style {
            self.state.metrics_mut().record_cache_hit();
            return Ok(StyleNode {
                element_id: dom.id,
                computed_style: cached,
                children: self.compute_children_styles(dom)?,
            });
        }

        self.state.metrics_mut().record_cache_miss();

        // Compute style (simplified)
        let computed_style = self.compute_element_style(dom)?;

        // Cache the result
        self.state
            .style_cache_mut()
            .insert(dom.id, computed_style.clone());

        // Compute children
        let children = self.compute_children_styles(dom)?;

        Ok(StyleNode {
            element_id: dom.id,
            computed_style,
            children,
        })
    }

    /// Compute styles for children
    fn compute_children_styles(&mut self, dom: &DomNode) -> Result<Vec<StyleNode>, CssError> {
        let mut children = Vec::new();
        for child in &dom.children {
            children.push(self.compute_node_style(child)?);
        }
        Ok(children)
    }

    /// Compute style for a single element
    fn compute_element_style(&self, dom: &DomNode) -> Result<ComputedStyle, CssError> {
        let mut style = ComputedStyle::default();

        // Apply inline styles if present
        if let Some(declarations) = self.state.get_inline_style(dom.id.0) {
            for decl in declarations {
                self.apply_declaration(&mut style, decl);
            }
        }

        Ok(style)
    }

    /// Apply a declaration to a computed style
    fn apply_declaration(&self, style: &mut ComputedStyle, decl: &PropertyDeclaration) {
        // Simplified property application
        if decl.property.as_str() == "display" {
            style.display = match decl.value.as_str() {
                "block" => crate::types::Display::Block,
                "inline" => crate::types::Display::Inline,
                "inline-block" => crate::types::Display::InlineBlock,
                "flex" => crate::types::Display::Flex,
                "grid" => crate::types::Display::Grid,
                "none" => crate::types::Display::None,
                _ => crate::types::Display::default(),
            };
        }
        // Add more properties as needed
    }

    /// Handle style invalidation
    fn handle_invalidate_styles(&mut self, scope: InvalidationScope) -> CssResponse {
        let affected_count = match scope {
            InvalidationScope::All => {
                let count = self.state.style_cache().len();
                self.state.clear_caches();
                count
            }
            InvalidationScope::Subtree { root_id } => {
                // For subtree, we invalidate the root and clear cache
                // In a full implementation, we'd track the tree structure
                self.state
                    .style_cache_mut()
                    .invalidate(crate::error::ElementId::new(root_id));
                1
            }
            InvalidationScope::Selectors { patterns: _ } => {
                // For selector-based invalidation, clear all for now
                // A full implementation would match selectors
                let count = self.state.style_cache().len();
                self.state.clear_caches();
                count
            }
            InvalidationScope::Classes { class_names: _ } => {
                // For class-based invalidation, clear all for now
                let count = self.state.style_cache().len();
                self.state.clear_caches();
                count
            }
            InvalidationScope::Attributes { attr_names: _ } => {
                // For attribute-based invalidation, clear all for now
                let count = self.state.style_cache().len();
                self.state.clear_caches();
                count
            }
        };

        self.state.metrics_mut().record_invalidation();

        CssResponse::StylesInvalidated { affected_count }
    }

    /// Handle media query change
    fn handle_media_query_change(&mut self, viewport: ViewportInfo) -> CssResponse {
        self.state.update_media(viewport);
        CssResponse::MediaQueriesUpdated
    }

    /// Handle animation tick
    fn handle_animation_tick(&mut self, timestamp: f64) -> CssResponse {
        let active_animations = self.state.tick_animations(timestamp);
        CssResponse::AnimationUpdated { active_animations }
    }

    /// Handle font loaded
    fn handle_font_loaded(&mut self, family: String) -> CssResponse {
        let was_loaded = self.state.is_font_loaded(&family);
        self.state.register_font(family.clone());

        // If this is a new font, we may need to restyle elements using it
        let requires_restyle = !was_loaded;

        CssResponse::FontProcessed {
            family,
            requires_restyle,
        }
    }

    /// Handle shutdown
    fn handle_shutdown(&mut self) -> CssResponse {
        self.running = false;
        self.state.pause_all_animations();
        CssResponse::Ready
    }
}

impl Default for BasicCssEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl MessageHandler for BasicCssEngine {
    fn handle(&mut self, message: CssMessage) -> CssResponse {
        if !self.running && !matches!(message, CssMessage::Shutdown) {
            return CssResponse::Error(CssError::ComputationError {
                reason: "Engine has been shut down".to_string(),
            });
        }

        match message {
            CssMessage::ParseStylesheet { id, content } => {
                self.handle_parse_stylesheet(id, content)
            }
            CssMessage::ComputeStyles { dom_root } => self.handle_compute_styles(dom_root),
            CssMessage::InvalidateStyles { scope } => self.handle_invalidate_styles(scope),
            CssMessage::MediaQueryChange { viewport } => self.handle_media_query_change(viewport),
            CssMessage::AnimationTick { timestamp } => self.handle_animation_tick(timestamp),
            CssMessage::FontLoaded { family } => self.handle_font_loaded(family),
            CssMessage::Shutdown => self.handle_shutdown(),
        }
    }

    fn is_ready(&self) -> bool {
        self.running
    }

    fn pending_count(&self) -> usize {
        self.pending_computations
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::Animation;

    #[test]
    fn test_viewport_info_default() {
        let viewport = ViewportInfo::default();
        assert_eq!(viewport.width, 1920.0);
        assert_eq!(viewport.height, 1080.0);
        assert_eq!(viewport.device_pixel_ratio, 1.0);
        assert!(!viewport.prefers_dark_mode);
    }

    #[test]
    fn test_viewport_info_mobile() {
        let viewport = ViewportInfo::mobile();
        assert_eq!(viewport.width, 375.0);
        assert_eq!(viewport.height, 667.0);
    }

    #[test]
    fn test_viewport_info_tablet() {
        let viewport = ViewportInfo::tablet();
        assert_eq!(viewport.width, 768.0);
        assert_eq!(viewport.height, 1024.0);
    }

    #[test]
    fn test_viewport_info_desktop() {
        let viewport = ViewportInfo::desktop();
        assert_eq!(viewport.width, 1920.0);
        assert_eq!(viewport.height, 1080.0);
    }

    #[test]
    fn test_viewport_info_builder() {
        let viewport = ViewportInfo::new(800.0, 600.0)
            .with_dpr(2.0)
            .with_dark_mode()
            .with_reduced_motion();

        assert_eq!(viewport.width, 800.0);
        assert_eq!(viewport.height, 600.0);
        assert_eq!(viewport.device_pixel_ratio, 2.0);
        assert!(viewport.prefers_dark_mode);
        assert!(viewport.prefers_reduced_motion);
    }

    #[test]
    fn test_invalidation_scope_subtree() {
        let scope = InvalidationScope::subtree(42);
        assert!(matches!(scope, InvalidationScope::Subtree { root_id: 42 }));
    }

    #[test]
    fn test_invalidation_scope_selectors() {
        let scope = InvalidationScope::selectors(vec![".active".to_string()]);
        if let InvalidationScope::Selectors { patterns } = scope {
            assert_eq!(patterns.len(), 1);
            assert_eq!(patterns[0], ".active");
        } else {
            panic!("Expected Selectors scope");
        }
    }

    #[test]
    fn test_invalidation_scope_classes() {
        let scope = InvalidationScope::classes(vec!["foo".to_string(), "bar".to_string()]);
        if let InvalidationScope::Classes { class_names } = scope {
            assert_eq!(class_names.len(), 2);
        } else {
            panic!("Expected Classes scope");
        }
    }

    #[test]
    fn test_invalidation_scope_all() {
        let scope = InvalidationScope::All;
        assert!(matches!(scope, InvalidationScope::All));
    }

    #[test]
    fn test_css_message_parse_stylesheet() {
        let msg = CssMessage::ParseStylesheet {
            id: StylesheetId::new(1),
            content: "body { color: red; }".to_string(),
        };

        if let CssMessage::ParseStylesheet { id, content } = msg {
            assert_eq!(id.0, 1);
            assert_eq!(content, "body { color: red; }");
        } else {
            panic!("Expected ParseStylesheet message");
        }
    }

    #[test]
    fn test_css_message_compute_styles() {
        let msg = CssMessage::ComputeStyles { dom_root: 123 };

        if let CssMessage::ComputeStyles { dom_root } = msg {
            assert_eq!(dom_root, 123);
        } else {
            panic!("Expected ComputeStyles message");
        }
    }

    #[test]
    fn test_css_message_invalidate_styles() {
        let msg = CssMessage::InvalidateStyles {
            scope: InvalidationScope::All,
        };

        if let CssMessage::InvalidateStyles { scope } = msg {
            assert!(matches!(scope, InvalidationScope::All));
        } else {
            panic!("Expected InvalidateStyles message");
        }
    }

    #[test]
    fn test_css_message_media_query_change() {
        let viewport = ViewportInfo::mobile();
        let msg = CssMessage::MediaQueryChange {
            viewport: viewport.clone(),
        };

        if let CssMessage::MediaQueryChange { viewport: v } = msg {
            assert_eq!(v.width, 375.0);
        } else {
            panic!("Expected MediaQueryChange message");
        }
    }

    #[test]
    fn test_css_message_animation_tick() {
        let msg = CssMessage::AnimationTick { timestamp: 1000.0 };

        if let CssMessage::AnimationTick { timestamp } = msg {
            assert_eq!(timestamp, 1000.0);
        } else {
            panic!("Expected AnimationTick message");
        }
    }

    #[test]
    fn test_css_message_font_loaded() {
        let msg = CssMessage::FontLoaded {
            family: "Roboto".to_string(),
        };

        if let CssMessage::FontLoaded { family } = msg {
            assert_eq!(family, "Roboto");
        } else {
            panic!("Expected FontLoaded message");
        }
    }

    #[test]
    fn test_css_message_shutdown() {
        let msg = CssMessage::Shutdown;
        assert!(matches!(msg, CssMessage::Shutdown));
    }

    #[test]
    fn test_css_response_stylesheet_parsed_success() {
        let response = CssResponse::StylesheetParsed {
            id: StylesheetId::new(1),
            error: None,
        };

        if let CssResponse::StylesheetParsed { id, error } = response {
            assert_eq!(id.0, 1);
            assert!(error.is_none());
        } else {
            panic!("Expected StylesheetParsed response");
        }
    }

    #[test]
    fn test_css_response_stylesheet_parsed_error() {
        let response = CssResponse::StylesheetParsed {
            id: StylesheetId::new(1),
            error: Some(CssError::ParseError {
                line: 1,
                column: 1,
                message: "syntax error".to_string(),
            }),
        };

        if let CssResponse::StylesheetParsed { id, error } = response {
            assert_eq!(id.0, 1);
            assert!(error.is_some());
        } else {
            panic!("Expected StylesheetParsed response");
        }
    }

    #[test]
    fn test_css_response_styles_invalidated() {
        let response = CssResponse::StylesInvalidated { affected_count: 42 };

        if let CssResponse::StylesInvalidated { affected_count } = response {
            assert_eq!(affected_count, 42);
        } else {
            panic!("Expected StylesInvalidated response");
        }
    }

    #[test]
    fn test_css_response_animation_updated() {
        let response = CssResponse::AnimationUpdated {
            active_animations: 5,
        };

        if let CssResponse::AnimationUpdated { active_animations } = response {
            assert_eq!(active_animations, 5);
        } else {
            panic!("Expected AnimationUpdated response");
        }
    }

    #[test]
    fn test_css_response_font_processed() {
        let response = CssResponse::FontProcessed {
            family: "Arial".to_string(),
            requires_restyle: true,
        };

        if let CssResponse::FontProcessed {
            family,
            requires_restyle,
        } = response
        {
            assert_eq!(family, "Arial");
            assert!(requires_restyle);
        } else {
            panic!("Expected FontProcessed response");
        }
    }

    #[test]
    fn test_css_response_ready() {
        let response = CssResponse::Ready;
        assert!(matches!(response, CssResponse::Ready));
    }

    #[test]
    fn test_css_response_error() {
        let response = CssResponse::Error(CssError::OutOfMemory);
        if let CssResponse::Error(err) = response {
            assert!(matches!(err, CssError::OutOfMemory));
        } else {
            panic!("Expected Error response");
        }
    }

    // ========================================================================
    // BasicCssEngine tests
    // ========================================================================

    #[test]
    fn test_basic_engine_new() {
        let engine = BasicCssEngine::new();
        assert!(engine.is_running());
        assert!(engine.is_ready());
        assert_eq!(engine.pending_count(), 0);
    }

    #[test]
    fn test_basic_engine_with_config() {
        let limits = ResourceLimits::restrictive();
        let engine = BasicCssEngine::with_config(limits);
        assert!(engine.is_running());
    }

    #[test]
    fn test_basic_engine_parse_stylesheet() {
        let mut engine = BasicCssEngine::new();

        let response = engine.handle(CssMessage::ParseStylesheet {
            id: StylesheetId::new(0),
            content: "body { color: red; }".to_string(),
        });

        if let CssResponse::StylesheetParsed { id, error } = response {
            assert_eq!(id.0, 0);
            assert!(error.is_none());
        } else {
            panic!("Expected StylesheetParsed response");
        }

        assert_eq!(engine.state().stylesheet_count(), 1);
    }

    #[test]
    fn test_basic_engine_parse_stylesheet_too_large() {
        let limits = ResourceLimits::new().with_max_stylesheet_size(10);
        let mut engine = BasicCssEngine::with_config(limits);

        let response = engine.handle(CssMessage::ParseStylesheet {
            id: StylesheetId::new(0),
            content: "body { color: red; margin: 0; padding: 0; }".to_string(),
        });

        if let CssResponse::StylesheetParsed { error, .. } = response {
            assert!(error.is_some());
        } else {
            panic!("Expected StylesheetParsed response");
        }
    }

    #[test]
    fn test_basic_engine_compute_styles() {
        let mut engine = BasicCssEngine::new();

        let response = engine.handle(CssMessage::ComputeStyles { dom_root: 1 });

        if let CssResponse::StylesComputed { tree } = response {
            assert_eq!(tree.root.element_id.0, 1);
        } else {
            panic!("Expected StylesComputed response");
        }
    }

    #[test]
    fn test_basic_engine_invalidate_all() {
        let mut engine = BasicCssEngine::new();

        // Compute some styles first
        engine.handle(CssMessage::ComputeStyles { dom_root: 1 });
        engine.handle(CssMessage::ComputeStyles { dom_root: 2 });

        let response = engine.handle(CssMessage::InvalidateStyles {
            scope: InvalidationScope::All,
        });

        if let CssResponse::StylesInvalidated { affected_count: _ } = response {
            // Test passed - just verifying we get the expected response type
        } else {
            panic!("Expected StylesInvalidated response");
        }
    }

    #[test]
    fn test_basic_engine_invalidate_subtree() {
        let mut engine = BasicCssEngine::new();

        // Compute styles first
        engine.handle(CssMessage::ComputeStyles { dom_root: 1 });

        let response = engine.handle(CssMessage::InvalidateStyles {
            scope: InvalidationScope::subtree(1),
        });

        if let CssResponse::StylesInvalidated { affected_count } = response {
            assert_eq!(affected_count, 1);
        } else {
            panic!("Expected StylesInvalidated response");
        }
    }

    #[test]
    fn test_basic_engine_media_query_change() {
        let mut engine = BasicCssEngine::new();

        let response = engine.handle(CssMessage::MediaQueryChange {
            viewport: ViewportInfo::mobile(),
        });

        assert!(matches!(response, CssResponse::MediaQueriesUpdated));
        assert_eq!(engine.state().viewport().width, 375.0);
    }

    #[test]
    fn test_basic_engine_animation_tick() {
        let mut engine = BasicCssEngine::new();

        // Add an animation
        let mut anim = Animation::new("fade", 1, 1000.0);
        anim.start(0.0);
        engine.state_mut().add_animation(anim);

        // Tick at 500ms
        let response = engine.handle(CssMessage::AnimationTick { timestamp: 500.0 });

        if let CssResponse::AnimationUpdated { active_animations } = response {
            assert_eq!(active_animations, 1);
        } else {
            panic!("Expected AnimationUpdated response");
        }

        // Tick past end
        let response = engine.handle(CssMessage::AnimationTick { timestamp: 1500.0 });

        if let CssResponse::AnimationUpdated { active_animations } = response {
            assert_eq!(active_animations, 0);
        } else {
            panic!("Expected AnimationUpdated response");
        }
    }

    #[test]
    fn test_basic_engine_font_loaded() {
        let mut engine = BasicCssEngine::new();

        let response = engine.handle(CssMessage::FontLoaded {
            family: "Roboto".to_string(),
        });

        if let CssResponse::FontProcessed {
            family,
            requires_restyle,
        } = response
        {
            assert_eq!(family, "Roboto");
            assert!(requires_restyle);
        } else {
            panic!("Expected FontProcessed response");
        }

        // Second load of same font should not require restyle
        let response = engine.handle(CssMessage::FontLoaded {
            family: "Roboto".to_string(),
        });

        if let CssResponse::FontProcessed {
            requires_restyle, ..
        } = response
        {
            assert!(!requires_restyle);
        } else {
            panic!("Expected FontProcessed response");
        }
    }

    #[test]
    fn test_basic_engine_shutdown() {
        let mut engine = BasicCssEngine::new();
        assert!(engine.is_running());

        let response = engine.handle(CssMessage::Shutdown);

        assert!(matches!(response, CssResponse::Ready));
        assert!(!engine.is_running());
        assert!(!engine.is_ready());
    }

    #[test]
    fn test_basic_engine_reject_after_shutdown() {
        let mut engine = BasicCssEngine::new();

        engine.handle(CssMessage::Shutdown);

        let response = engine.handle(CssMessage::ComputeStyles { dom_root: 1 });

        if let CssResponse::Error(CssError::ComputationError { reason }) = response {
            assert!(reason.contains("shut down"));
        } else {
            panic!("Expected Error response after shutdown");
        }
    }

    #[test]
    fn test_basic_engine_state_transitions() {
        let mut engine = BasicCssEngine::new();

        // Initial state
        assert_eq!(engine.state().stylesheet_count(), 0);
        assert!(engine.state().active_animations().is_empty());

        // Add stylesheet
        engine.handle(CssMessage::ParseStylesheet {
            id: StylesheetId::new(0),
            content: "body { }".to_string(),
        });
        assert_eq!(engine.state().stylesheet_count(), 1);

        // Add another stylesheet
        engine.handle(CssMessage::ParseStylesheet {
            id: StylesheetId::new(1),
            content: "div { }".to_string(),
        });
        assert_eq!(engine.state().stylesheet_count(), 2);

        // Compute styles
        engine.handle(CssMessage::ComputeStyles { dom_root: 1 });
        assert!(engine.state().metrics().style_computations > 0);

        // Invalidate all
        engine.handle(CssMessage::InvalidateStyles {
            scope: InvalidationScope::All,
        });
        assert!(engine.state().metrics().invalidations > 0);
    }

    #[test]
    fn test_basic_engine_metrics_tracking() {
        let mut engine = BasicCssEngine::new();

        // Parse stylesheet
        engine.handle(CssMessage::ParseStylesheet {
            id: StylesheetId::new(0),
            content: "body { }".to_string(),
        });

        // Compute styles (first time - cache miss)
        engine.handle(CssMessage::ComputeStyles { dom_root: 1 });

        // Compute same styles again (cache hit)
        engine.handle(CssMessage::ComputeStyles { dom_root: 1 });

        let metrics = engine.state().metrics();
        assert!(metrics.stylesheets_parsed >= 1);
        assert!(metrics.style_computations >= 1);
    }

    #[test]
    fn test_basic_engine_inline_style_application() {
        let mut engine = BasicCssEngine::new();

        // Set inline style
        engine
            .state_mut()
            .set_inline_style(1, vec![PropertyDeclaration::new("display", "block")]);

        // Compute styles
        let response = engine.handle(CssMessage::ComputeStyles { dom_root: 1 });

        if let CssResponse::StylesComputed { tree } = response {
            assert_eq!(
                tree.root.computed_style.display,
                crate::types::Display::Block
            );
        } else {
            panic!("Expected StylesComputed response");
        }
    }

    #[test]
    fn test_message_handler_trait_methods() {
        let mut engine = BasicCssEngine::new();

        // Test is_ready
        assert!(engine.is_ready());

        // Test pending_count
        assert_eq!(engine.pending_count(), 0);

        // After shutdown
        engine.handle(CssMessage::Shutdown);
        assert!(!engine.is_ready());
    }

    #[test]
    fn test_invalidation_scope_variants() {
        let mut engine = BasicCssEngine::new();

        // Test each invalidation scope variant
        let scopes = vec![
            InvalidationScope::All,
            InvalidationScope::subtree(1),
            InvalidationScope::selectors(vec![".foo".to_string()]),
            InvalidationScope::classes(vec!["bar".to_string()]),
            InvalidationScope::attributes(vec!["data-x".to_string()]),
        ];

        for scope in scopes {
            let response = engine.handle(CssMessage::InvalidateStyles { scope });
            assert!(matches!(response, CssResponse::StylesInvalidated { .. }));
        }
    }
}
