//! CSS Cascade Layers Implementation
//!
//! This module implements CSS Cascade Layers (@layer) as defined in CSS Cascading and
//! Inheritance Level 5. Layers provide a way to control the cascade order of style rules.
//!
//! # Overview
//!
//! - Layers declared first have lowest priority
//! - Unlayered rules have highest priority (equivalent to an implicit top layer)
//! - Layers can be nested using dot notation (e.g., `framework.theme`)
//! - Anonymous layers are ordered by their first occurrence
//!
//! # Examples
//!
//! ```
//! use css_cascade::layers::{LayerRegistry, CascadeLayer};
//!
//! let mut registry = LayerRegistry::new();
//!
//! // Declare layer order
//! registry.declare_layer("reset");
//! registry.declare_layer("base");
//! registry.declare_layer("components");
//! registry.declare_layer("utilities");
//!
//! // Rules in 'reset' have lowest priority, 'utilities' highest among named layers
//! // Unlayered rules still beat all named layers
//! ```

use std::collections::HashMap;
use std::fmt;

// ============================================================================
// Layer Identifier
// ============================================================================

/// Unique identifier for a cascade layer
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LayerId(u32);

impl LayerId {
    /// Create a new layer ID
    fn new(id: u32) -> Self {
        Self(id)
    }

    /// Get the numeric value of this ID
    pub fn value(&self) -> u32 {
        self.0
    }
}

impl fmt::Display for LayerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LayerId({})", self.0)
    }
}

// ============================================================================
// CSS Rule (placeholder for integration)
// ============================================================================

/// Placeholder for CSS rules that can be stored in layers
/// In a full implementation, this would be the actual CSS rule type
#[derive(Debug, Clone, PartialEq)]
pub struct CssRule {
    /// The raw CSS text of the rule (for testing/debugging)
    pub css_text: String,
}

impl CssRule {
    /// Create a new CSS rule
    pub fn new(css_text: impl Into<String>) -> Self {
        Self {
            css_text: css_text.into(),
        }
    }
}

// ============================================================================
// Cascade Layer
// ============================================================================

/// A CSS cascade layer containing rules and optional sublayers
///
/// Layers provide explicit control over the cascade. Rules within layers
/// are ordered based on when the layer was first declared or referenced.
#[derive(Debug, Clone)]
pub struct CascadeLayer {
    /// Layer name (None for anonymous layers)
    name: Option<String>,
    /// CSS rules contained in this layer
    rules: Vec<CssRule>,
    /// Nested sublayers (e.g., `framework.theme` has `theme` as sublayer of `framework`)
    sublayers: Vec<CascadeLayer>,
    /// Unique identifier for this layer
    id: LayerId,
}

impl CascadeLayer {
    /// Create a new named layer
    pub fn named(name: impl Into<String>, id: LayerId) -> Self {
        Self {
            name: Some(name.into()),
            rules: Vec::new(),
            sublayers: Vec::new(),
            id,
        }
    }

    /// Create a new anonymous layer
    pub fn anonymous(id: LayerId) -> Self {
        Self {
            name: None,
            rules: Vec::new(),
            sublayers: Vec::new(),
            id,
        }
    }

    /// Get the layer name
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Check if this is an anonymous layer
    pub fn is_anonymous(&self) -> bool {
        self.name.is_none()
    }

    /// Get the layer ID
    pub fn id(&self) -> LayerId {
        self.id
    }

    /// Add a CSS rule to this layer
    pub fn add_rule(&mut self, rule: CssRule) {
        self.rules.push(rule);
    }

    /// Get the rules in this layer
    pub fn rules(&self) -> &[CssRule] {
        &self.rules
    }

    /// Get mutable access to rules
    pub fn rules_mut(&mut self) -> &mut Vec<CssRule> {
        &mut self.rules
    }

    /// Add a sublayer
    pub fn add_sublayer(&mut self, sublayer: CascadeLayer) {
        self.sublayers.push(sublayer);
    }

    /// Get sublayers
    pub fn sublayers(&self) -> &[CascadeLayer] {
        &self.sublayers
    }

    /// Get mutable access to sublayers
    pub fn sublayers_mut(&mut self) -> &mut Vec<CascadeLayer> {
        &mut self.sublayers
    }

    /// Find a sublayer by name
    pub fn find_sublayer(&self, name: &str) -> Option<&CascadeLayer> {
        self.sublayers.iter().find(|s| s.name() == Some(name))
    }

    /// Find a sublayer by name (mutable)
    pub fn find_sublayer_mut(&mut self, name: &str) -> Option<&mut CascadeLayer> {
        self.sublayers.iter_mut().find(|s| s.name() == Some(name))
    }

    /// Get the full name including parent layers (e.g., "framework.theme")
    pub fn full_name(&self, parent_path: Option<&str>) -> Option<String> {
        match (&self.name, parent_path) {
            (Some(name), Some(parent)) => Some(format!("{}.{}", parent, name)),
            (Some(name), None) => Some(name.clone()),
            (None, _) => None,
        }
    }
}

impl PartialEq for CascadeLayer {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for CascadeLayer {}

// ============================================================================
// Layer Order
// ============================================================================

/// Tracks the ordering of cascade layers
///
/// Layers are ordered by first declaration/reference. The first layer declared
/// has the lowest priority. Unlayered (implicit layer) rules have the highest
/// priority among author styles.
#[derive(Debug, Clone)]
pub struct LayerOrder {
    /// Ordered list of layer IDs (first = lowest priority)
    layers: Vec<LayerId>,
    /// The implicit layer for unlayered rules (highest priority)
    implicit_layer: LayerId,
}

impl LayerOrder {
    /// Create a new layer order with the implicit layer
    pub fn new(implicit_layer_id: LayerId) -> Self {
        Self {
            layers: Vec::new(),
            implicit_layer: implicit_layer_id,
        }
    }

    /// Add a layer to the order (at the end, giving it higher priority than existing layers)
    pub fn add_layer(&mut self, layer_id: LayerId) {
        if !self.layers.contains(&layer_id) && layer_id != self.implicit_layer {
            self.layers.push(layer_id);
        }
    }

    /// Get the priority of a layer (lower number = lower priority)
    ///
    /// Returns None if the layer is not in the order
    pub fn priority(&self, layer_id: LayerId) -> Option<u32> {
        if layer_id == self.implicit_layer {
            // Implicit layer has highest priority
            return Some(self.layers.len() as u32 + 1);
        }

        self.layers
            .iter()
            .position(|&id| id == layer_id)
            .map(|pos| pos as u32)
    }

    /// Compare two layers by priority
    ///
    /// Returns:
    /// - `Ordering::Less` if `a` has lower priority than `b`
    /// - `Ordering::Greater` if `a` has higher priority than `b`
    /// - `Ordering::Equal` if they have the same priority
    pub fn compare(&self, a: LayerId, b: LayerId) -> std::cmp::Ordering {
        let a_priority = self.priority(a).unwrap_or(u32::MAX);
        let b_priority = self.priority(b).unwrap_or(u32::MAX);
        a_priority.cmp(&b_priority)
    }

    /// Get the implicit layer ID (for unlayered rules)
    pub fn implicit_layer(&self) -> LayerId {
        self.implicit_layer
    }

    /// Get all layers in priority order (lowest first, excluding implicit)
    pub fn layers(&self) -> &[LayerId] {
        &self.layers
    }

    /// Get the number of declared layers (excluding implicit)
    pub fn len(&self) -> usize {
        self.layers.len()
    }

    /// Check if there are no declared layers
    pub fn is_empty(&self) -> bool {
        self.layers.is_empty()
    }

    /// Insert a layer at a specific position
    ///
    /// This is used for pre-declaring layer order with `@layer name1, name2;`
    pub fn insert_at(&mut self, position: usize, layer_id: LayerId) {
        if !self.layers.contains(&layer_id) && layer_id != self.implicit_layer {
            let pos = position.min(self.layers.len());
            self.layers.insert(pos, layer_id);
        }
    }

    /// Reorder layers to match a declaration list
    ///
    /// This handles `@layer name1, name2, name3;` declarations that establish order
    pub fn declare_order(&mut self, layer_ids: &[LayerId]) {
        // Remove layers that are being reordered
        self.layers.retain(|id| !layer_ids.contains(id));

        // Add them back in the declared order at the current position
        for &layer_id in layer_ids {
            if layer_id != self.implicit_layer {
                self.layers.push(layer_id);
            }
        }
    }
}

// ============================================================================
// Layer Registry
// ============================================================================

/// Registry for managing cascade layers
///
/// The registry tracks all named layers, their hierarchy, and maintains
/// the layer ordering for cascade resolution.
#[derive(Debug)]
pub struct LayerRegistry {
    /// Map of layer names to their IDs
    named_layers: HashMap<String, LayerId>,
    /// Map of layer IDs to layer data
    layers: HashMap<LayerId, CascadeLayer>,
    /// Layer ordering
    order: LayerOrder,
    /// Next available layer ID
    next_id: u32,
    /// Anonymous layer counter
    anonymous_count: u32,
}

impl LayerRegistry {
    /// Create a new layer registry
    pub fn new() -> Self {
        let implicit_id = LayerId::new(0);
        Self {
            named_layers: HashMap::new(),
            layers: HashMap::new(),
            order: LayerOrder::new(implicit_id),
            next_id: 1, // 0 is reserved for implicit layer
            anonymous_count: 0,
        }
    }

    /// Get the next available layer ID
    fn allocate_id(&mut self) -> LayerId {
        let id = LayerId::new(self.next_id);
        self.next_id += 1;
        id
    }

    /// Declare a named layer (creates if doesn't exist)
    ///
    /// This corresponds to `@layer name;` or the first reference to a layer.
    /// Returns the layer ID.
    pub fn declare_layer(&mut self, name: &str) -> LayerId {
        if let Some(&id) = self.named_layers.get(name) {
            return id;
        }

        // Handle nested layer names (e.g., "framework.theme")
        if name.contains('.') {
            return self.declare_nested_layer(name);
        }

        let id = self.allocate_id();
        let layer = CascadeLayer::named(name, id);

        self.named_layers.insert(name.to_string(), id);
        self.layers.insert(id, layer);
        self.order.add_layer(id);

        id
    }

    /// Declare a nested layer (e.g., "framework.theme")
    fn declare_nested_layer(&mut self, full_name: &str) -> LayerId {
        let parts: Vec<&str> = full_name.split('.').collect();

        if parts.is_empty() {
            // Return implicit layer for empty name
            return self.order.implicit_layer();
        }

        // Ensure parent layers exist
        let mut current_path = String::new();

        for (i, part) in parts.iter().enumerate() {
            if i > 0 {
                current_path.push('.');
            }
            current_path.push_str(part);

            if !self.named_layers.contains_key(&current_path) {
                let id = self.allocate_id();
                let layer = CascadeLayer::named(&current_path, id);

                self.named_layers.insert(current_path.clone(), id);
                self.layers.insert(id, layer);
                self.order.add_layer(id);
            }
        }

        *self.named_layers.get(full_name).unwrap()
    }

    /// Create an anonymous layer
    ///
    /// This corresponds to `@layer { ... }` without a name.
    /// Anonymous layers are ordered by first occurrence.
    pub fn create_anonymous_layer(&mut self) -> LayerId {
        let id = self.allocate_id();
        let layer = CascadeLayer::anonymous(id);

        self.anonymous_count += 1;
        self.layers.insert(id, layer);
        self.order.add_layer(id);

        id
    }

    /// Declare multiple layers in order
    ///
    /// This corresponds to `@layer name1, name2, name3;`
    /// The order of names determines their priority (first = lowest).
    pub fn declare_layer_order(&mut self, names: &[&str]) -> Vec<LayerId> {
        let mut ids = Vec::with_capacity(names.len());

        for name in names {
            let id = self.declare_layer(name);
            ids.push(id);
        }

        // Establish the order
        self.order.declare_order(&ids);

        ids
    }

    /// Get a layer by name
    pub fn get_layer(&self, name: &str) -> Option<&CascadeLayer> {
        self.named_layers
            .get(name)
            .and_then(|&id| self.layers.get(&id))
    }

    /// Get a layer by name (mutable)
    pub fn get_layer_mut(&mut self, name: &str) -> Option<&mut CascadeLayer> {
        if let Some(&id) = self.named_layers.get(name) {
            self.layers.get_mut(&id)
        } else {
            None
        }
    }

    /// Get a layer by ID
    pub fn get_layer_by_id(&self, id: LayerId) -> Option<&CascadeLayer> {
        self.layers.get(&id)
    }

    /// Get a layer by ID (mutable)
    pub fn get_layer_by_id_mut(&mut self, id: LayerId) -> Option<&mut CascadeLayer> {
        self.layers.get_mut(&id)
    }

    /// Get the layer ID for a name
    pub fn get_layer_id(&self, name: &str) -> Option<LayerId> {
        self.named_layers.get(name).copied()
    }

    /// Get the implicit layer ID (for unlayered rules)
    pub fn implicit_layer_id(&self) -> LayerId {
        self.order.implicit_layer()
    }

    /// Get the layer order
    pub fn order(&self) -> &LayerOrder {
        &self.order
    }

    /// Get mutable access to layer order
    pub fn order_mut(&mut self) -> &mut LayerOrder {
        &mut self.order
    }

    /// Add a rule to a named layer
    pub fn add_rule_to_layer(&mut self, layer_name: &str, rule: CssRule) -> bool {
        // Ensure layer exists
        let id = self.declare_layer(layer_name);

        if let Some(layer) = self.layers.get_mut(&id) {
            layer.add_rule(rule);
            true
        } else {
            false
        }
    }

    /// Add a rule to an anonymous layer by ID
    pub fn add_rule_to_layer_by_id(&mut self, layer_id: LayerId, rule: CssRule) -> bool {
        if let Some(layer) = self.layers.get_mut(&layer_id) {
            layer.add_rule(rule);
            true
        } else {
            false
        }
    }

    /// Get all layer names in priority order (lowest first)
    pub fn layer_names_in_order(&self) -> Vec<Option<String>> {
        self.order
            .layers()
            .iter()
            .map(|&id| {
                self.layers
                    .get(&id)
                    .and_then(|l| l.name().map(String::from))
            })
            .collect()
    }

    /// Compare two layers by cascade priority
    pub fn compare_layers(&self, a: LayerId, b: LayerId) -> std::cmp::Ordering {
        self.order.compare(a, b)
    }

    /// Get the number of named layers
    pub fn named_layer_count(&self) -> usize {
        self.named_layers.len()
    }

    /// Get the number of anonymous layers
    pub fn anonymous_layer_count(&self) -> u32 {
        self.anonymous_count
    }

    /// Check if a layer name exists
    pub fn has_layer(&self, name: &str) -> bool {
        self.named_layers.contains_key(name)
    }

    /// Get all layer names
    pub fn layer_names(&self) -> Vec<&str> {
        self.named_layers.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for LayerRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Revert-Layer Support
// ============================================================================

/// Represents the revert-layer keyword value
///
/// When used, a property value reverts to the value from the previous
/// cascade layer that contributed to the element's styling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RevertLayer;

impl RevertLayer {
    /// Check if a string is the "revert-layer" keyword
    pub fn is_revert_layer(value: &str) -> bool {
        value.trim().eq_ignore_ascii_case("revert-layer")
    }
}

/// Represents different layer-related at-rules
#[derive(Debug, Clone, PartialEq)]
pub enum LayerAtRule {
    /// `@layer name;` - Layer statement (declaration without block)
    Statement(Vec<String>),
    /// `@layer name { ... }` - Layer block with rules
    Block {
        name: Option<String>,
        // Rules would go here in a full implementation
    },
    /// `@import url(...) layer(name)` - Import with layer
    Import {
        url: String,
        layer_name: Option<String>,
    },
}

impl LayerAtRule {
    /// Parse a layer statement like `@layer name1, name2;`
    pub fn parse_statement(input: &str) -> Option<Self> {
        let input = input.trim();

        // Remove @layer prefix if present
        let content = input
            .strip_prefix("@layer")
            .or_else(|| input.strip_prefix("layer"))?
            .trim();

        // Remove trailing semicolon
        let content = content.strip_suffix(';').unwrap_or(content).trim();

        if content.is_empty() {
            return None;
        }

        // Parse comma-separated layer names
        let names: Vec<String> = content
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        if names.is_empty() {
            return None;
        }

        Some(LayerAtRule::Statement(names))
    }

    /// Check if a string starts a layer block
    pub fn is_layer_block_start(input: &str) -> bool {
        let input = input.trim();
        (input.starts_with("@layer") || input.starts_with("layer")) && input.contains('{')
    }

    /// Parse the layer name from a block start like `@layer name {`
    pub fn parse_block_name(input: &str) -> Option<String> {
        let input = input.trim();

        let content = input
            .strip_prefix("@layer")
            .or_else(|| input.strip_prefix("layer"))?
            .trim();

        // Remove opening brace
        let name = content.strip_suffix('{')?.trim();

        if name.is_empty() {
            None // Anonymous layer
        } else {
            Some(name.to_string())
        }
    }

    /// Parse an import with layer like `@import url("...") layer(name);`
    pub fn parse_layered_import(input: &str) -> Option<Self> {
        let input = input.trim();

        if !input.starts_with("@import") {
            return None;
        }

        // Check for layer() function
        let layer_start = input.find("layer(")?;
        let layer_end = input[layer_start..].find(')')? + layer_start;

        let layer_content = input[layer_start + 6..layer_end].trim();

        let layer_name = if layer_content.is_empty() {
            None // Anonymous layer for import
        } else {
            Some(layer_content.to_string())
        };

        // Extract URL (simplified parsing)
        let url = if let Some(url_start) = input.find("url(") {
            let after_url = &input[url_start + 4..];
            let url_end = after_url.find(')')?;
            let url = after_url[..url_end].trim();
            // Remove quotes if present
            url.trim_matches('"').trim_matches('\'').to_string()
        } else {
            // Try to find quoted string directly after @import
            let after_import = input.strip_prefix("@import")?.trim();
            let url = if let Some(stripped) = after_import.strip_prefix('"') {
                let end = stripped.find('"')?;
                &stripped[..end]
            } else if let Some(stripped) = after_import.strip_prefix('\'') {
                let end = stripped.find('\'')?;
                &stripped[..end]
            } else {
                return None;
            };
            url.to_string()
        };

        Some(LayerAtRule::Import { url, layer_name })
    }
}

// ============================================================================
// Layer Name Validation
// ============================================================================

/// Validate a layer name
///
/// Layer names must be valid CSS identifiers and cannot be CSS-wide keywords.
pub fn is_valid_layer_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    // Check for CSS-wide keywords that are not allowed
    let lower = name.to_lowercase();
    if matches!(
        lower.as_str(),
        "initial" | "inherit" | "unset" | "revert" | "revert-layer"
    ) {
        return false;
    }

    // Must start with letter, underscore, or hyphen (not digit)
    let first = name.chars().next().unwrap();
    if !first.is_alphabetic() && first != '_' && first != '-' {
        return false;
    }

    // Can contain letters, digits, underscores, and hyphens
    // For nested layers, also allow dots
    name.chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '-' || c == '.')
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // LayerId Tests
    // ========================================================================

    #[test]
    fn test_layer_id_creation() {
        let id = LayerId::new(42);
        assert_eq!(id.value(), 42);
    }

    #[test]
    fn test_layer_id_equality() {
        let id1 = LayerId::new(1);
        let id2 = LayerId::new(1);
        let id3 = LayerId::new(2);

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_layer_id_display() {
        let id = LayerId::new(5);
        assert_eq!(format!("{}", id), "LayerId(5)");
    }

    // ========================================================================
    // CascadeLayer Tests
    // ========================================================================

    #[test]
    fn test_cascade_layer_named() {
        let id = LayerId::new(1);
        let layer = CascadeLayer::named("base", id);

        assert_eq!(layer.name(), Some("base"));
        assert!(!layer.is_anonymous());
        assert_eq!(layer.id(), id);
        assert!(layer.rules().is_empty());
        assert!(layer.sublayers().is_empty());
    }

    #[test]
    fn test_cascade_layer_anonymous() {
        let id = LayerId::new(1);
        let layer = CascadeLayer::anonymous(id);

        assert_eq!(layer.name(), None);
        assert!(layer.is_anonymous());
        assert_eq!(layer.id(), id);
    }

    #[test]
    fn test_cascade_layer_add_rule() {
        let id = LayerId::new(1);
        let mut layer = CascadeLayer::named("test", id);

        layer.add_rule(CssRule::new("div { color: red }"));
        layer.add_rule(CssRule::new("p { font-size: 16px }"));

        assert_eq!(layer.rules().len(), 2);
        assert_eq!(layer.rules()[0].css_text, "div { color: red }");
    }

    #[test]
    fn test_cascade_layer_sublayers() {
        let parent_id = LayerId::new(1);
        let child_id = LayerId::new(2);

        let mut parent = CascadeLayer::named("framework", parent_id);
        let child = CascadeLayer::named("theme", child_id);

        parent.add_sublayer(child);

        assert_eq!(parent.sublayers().len(), 1);
        assert_eq!(parent.sublayers()[0].name(), Some("theme"));
    }

    #[test]
    fn test_cascade_layer_find_sublayer() {
        let parent_id = LayerId::new(1);
        let child1_id = LayerId::new(2);
        let child2_id = LayerId::new(3);

        let mut parent = CascadeLayer::named("framework", parent_id);
        parent.add_sublayer(CascadeLayer::named("theme", child1_id));
        parent.add_sublayer(CascadeLayer::named("components", child2_id));

        let found = parent.find_sublayer("theme");
        assert!(found.is_some());
        assert_eq!(found.unwrap().name(), Some("theme"));

        let not_found = parent.find_sublayer("nonexistent");
        assert!(not_found.is_none());
    }

    #[test]
    fn test_cascade_layer_full_name() {
        let id = LayerId::new(1);
        let layer = CascadeLayer::named("theme", id);

        assert_eq!(layer.full_name(None), Some("theme".to_string()));
        assert_eq!(
            layer.full_name(Some("framework")),
            Some("framework.theme".to_string())
        );
    }

    // ========================================================================
    // LayerOrder Tests
    // ========================================================================

    #[test]
    fn test_layer_order_creation() {
        let implicit_id = LayerId::new(0);
        let order = LayerOrder::new(implicit_id);

        assert!(order.is_empty());
        assert_eq!(order.len(), 0);
        assert_eq!(order.implicit_layer(), implicit_id);
    }

    #[test]
    fn test_layer_order_add_layer() {
        let implicit_id = LayerId::new(0);
        let mut order = LayerOrder::new(implicit_id);

        let layer1 = LayerId::new(1);
        let layer2 = LayerId::new(2);

        order.add_layer(layer1);
        order.add_layer(layer2);

        assert_eq!(order.len(), 2);
        assert_eq!(order.layers(), &[layer1, layer2]);
    }

    #[test]
    fn test_layer_order_no_duplicate() {
        let implicit_id = LayerId::new(0);
        let mut order = LayerOrder::new(implicit_id);

        let layer1 = LayerId::new(1);
        order.add_layer(layer1);
        order.add_layer(layer1); // Duplicate

        assert_eq!(order.len(), 1);
    }

    #[test]
    fn test_layer_order_implicit_not_added() {
        let implicit_id = LayerId::new(0);
        let mut order = LayerOrder::new(implicit_id);

        order.add_layer(implicit_id);

        assert!(order.is_empty());
    }

    #[test]
    fn test_layer_order_priority() {
        let implicit_id = LayerId::new(0);
        let mut order = LayerOrder::new(implicit_id);

        let layer1 = LayerId::new(1);
        let layer2 = LayerId::new(2);
        let layer3 = LayerId::new(3);

        order.add_layer(layer1);
        order.add_layer(layer2);
        order.add_layer(layer3);

        // First added = lowest priority (0)
        assert_eq!(order.priority(layer1), Some(0));
        assert_eq!(order.priority(layer2), Some(1));
        assert_eq!(order.priority(layer3), Some(2));

        // Implicit layer has highest priority
        assert_eq!(order.priority(implicit_id), Some(4)); // len + 1 = 3 + 1
    }

    #[test]
    fn test_layer_order_compare() {
        let implicit_id = LayerId::new(0);
        let mut order = LayerOrder::new(implicit_id);

        let layer1 = LayerId::new(1);
        let layer2 = LayerId::new(2);

        order.add_layer(layer1);
        order.add_layer(layer2);

        // layer1 has lower priority than layer2
        assert_eq!(order.compare(layer1, layer2), std::cmp::Ordering::Less);
        assert_eq!(order.compare(layer2, layer1), std::cmp::Ordering::Greater);
        assert_eq!(order.compare(layer1, layer1), std::cmp::Ordering::Equal);

        // Both have lower priority than implicit
        assert_eq!(order.compare(layer1, implicit_id), std::cmp::Ordering::Less);
        assert_eq!(order.compare(layer2, implicit_id), std::cmp::Ordering::Less);
    }

    #[test]
    fn test_layer_order_declare_order() {
        let implicit_id = LayerId::new(0);
        let mut order = LayerOrder::new(implicit_id);

        let layer1 = LayerId::new(1);
        let layer2 = LayerId::new(2);
        let layer3 = LayerId::new(3);

        // Add in different order initially
        order.add_layer(layer3);
        order.add_layer(layer1);

        // Declare new order
        order.declare_order(&[layer1, layer2, layer3]);

        // Should be in declared order now
        assert_eq!(order.priority(layer1), Some(0));
        assert_eq!(order.priority(layer2), Some(1));
        assert_eq!(order.priority(layer3), Some(2));
    }

    #[test]
    fn test_layer_order_insert_at() {
        let implicit_id = LayerId::new(0);
        let mut order = LayerOrder::new(implicit_id);

        let layer1 = LayerId::new(1);
        let layer2 = LayerId::new(2);
        let layer3 = LayerId::new(3);

        order.add_layer(layer1);
        order.add_layer(layer3);

        // Insert layer2 between layer1 and layer3
        order.insert_at(1, layer2);

        assert_eq!(order.priority(layer1), Some(0));
        assert_eq!(order.priority(layer2), Some(1));
        assert_eq!(order.priority(layer3), Some(2));
    }

    // ========================================================================
    // LayerRegistry Tests
    // ========================================================================

    #[test]
    fn test_layer_registry_creation() {
        let registry = LayerRegistry::new();

        assert_eq!(registry.named_layer_count(), 0);
        assert_eq!(registry.anonymous_layer_count(), 0);
    }

    #[test]
    fn test_layer_registry_declare_layer() {
        let mut registry = LayerRegistry::new();

        let id1 = registry.declare_layer("base");
        let id2 = registry.declare_layer("components");
        let id3 = registry.declare_layer("base"); // Duplicate

        assert_eq!(id1, id3); // Same layer
        assert_ne!(id1, id2); // Different layers
        assert_eq!(registry.named_layer_count(), 2);
    }

    #[test]
    fn test_layer_registry_nested_layers() {
        let mut registry = LayerRegistry::new();

        let id = registry.declare_layer("framework.theme");

        // Should create both "framework" and "framework.theme"
        assert!(registry.has_layer("framework"));
        assert!(registry.has_layer("framework.theme"));
        assert_eq!(registry.get_layer_id("framework.theme"), Some(id));
    }

    #[test]
    fn test_layer_registry_anonymous_layer() {
        let mut registry = LayerRegistry::new();

        let id1 = registry.create_anonymous_layer();
        let id2 = registry.create_anonymous_layer();

        assert_ne!(id1, id2);
        assert_eq!(registry.anonymous_layer_count(), 2);

        let layer = registry.get_layer_by_id(id1).unwrap();
        assert!(layer.is_anonymous());
    }

    #[test]
    fn test_layer_registry_declare_layer_order() {
        let mut registry = LayerRegistry::new();

        let ids = registry.declare_layer_order(&["reset", "base", "components", "utilities"]);

        assert_eq!(ids.len(), 4);

        // Check ordering - first declared = lowest priority
        let order = registry.order();
        assert!(order.priority(ids[0]) < order.priority(ids[1]));
        assert!(order.priority(ids[1]) < order.priority(ids[2]));
        assert!(order.priority(ids[2]) < order.priority(ids[3]));
    }

    #[test]
    fn test_layer_registry_add_rule_to_layer() {
        let mut registry = LayerRegistry::new();

        registry.declare_layer("base");
        let result = registry.add_rule_to_layer("base", CssRule::new("div { color: red }"));

        assert!(result);

        let layer = registry.get_layer("base").unwrap();
        assert_eq!(layer.rules().len(), 1);
    }

    #[test]
    fn test_layer_registry_add_rule_creates_layer() {
        let mut registry = LayerRegistry::new();

        // Layer doesn't exist yet
        assert!(!registry.has_layer("new-layer"));

        // Adding rule should create it
        let result = registry.add_rule_to_layer("new-layer", CssRule::new("p { margin: 0 }"));

        assert!(result);
        assert!(registry.has_layer("new-layer"));
    }

    #[test]
    fn test_layer_registry_implicit_layer_priority() {
        let mut registry = LayerRegistry::new();

        registry.declare_layer("base");
        registry.declare_layer("components");

        let base_id = registry.get_layer_id("base").unwrap();
        let implicit_id = registry.implicit_layer_id();

        // Implicit layer should have highest priority
        let order = registry.order();
        assert!(order.priority(implicit_id) > order.priority(base_id));
    }

    #[test]
    fn test_layer_registry_layer_names_in_order() {
        let mut registry = LayerRegistry::new();

        registry.declare_layer_order(&["reset", "base", "utilities"]);

        let names = registry.layer_names_in_order();

        assert_eq!(names.len(), 3);
        assert_eq!(names[0], Some("reset".to_string()));
        assert_eq!(names[1], Some("base".to_string()));
        assert_eq!(names[2], Some("utilities".to_string()));
    }

    #[test]
    fn test_layer_registry_compare_layers() {
        let mut registry = LayerRegistry::new();

        let ids = registry.declare_layer_order(&["low", "high"]);

        // "low" should have lower priority than "high"
        assert_eq!(
            registry.compare_layers(ids[0], ids[1]),
            std::cmp::Ordering::Less
        );
    }

    // ========================================================================
    // RevertLayer Tests
    // ========================================================================

    #[test]
    fn test_revert_layer_detection() {
        assert!(RevertLayer::is_revert_layer("revert-layer"));
        assert!(RevertLayer::is_revert_layer("REVERT-LAYER"));
        assert!(RevertLayer::is_revert_layer("  revert-layer  "));
        assert!(!RevertLayer::is_revert_layer("revert"));
        assert!(!RevertLayer::is_revert_layer("layer"));
    }

    // ========================================================================
    // LayerAtRule Tests
    // ========================================================================

    #[test]
    fn test_parse_layer_statement() {
        let result = LayerAtRule::parse_statement("@layer reset, base, components;");

        assert!(result.is_some());
        if let Some(LayerAtRule::Statement(names)) = result {
            assert_eq!(names, vec!["reset", "base", "components"]);
        } else {
            panic!("Expected Statement");
        }
    }

    #[test]
    fn test_parse_layer_statement_single() {
        let result = LayerAtRule::parse_statement("@layer utilities;");

        if let Some(LayerAtRule::Statement(names)) = result {
            assert_eq!(names, vec!["utilities"]);
        } else {
            panic!("Expected Statement");
        }
    }

    #[test]
    fn test_parse_layer_statement_without_at() {
        let result = LayerAtRule::parse_statement("layer base;");

        if let Some(LayerAtRule::Statement(names)) = result {
            assert_eq!(names, vec!["base"]);
        } else {
            panic!("Expected Statement");
        }
    }

    #[test]
    fn test_parse_layer_statement_empty() {
        let result = LayerAtRule::parse_statement("@layer;");
        assert!(result.is_none());
    }

    #[test]
    fn test_is_layer_block_start() {
        assert!(LayerAtRule::is_layer_block_start("@layer base {"));
        assert!(LayerAtRule::is_layer_block_start("@layer {"));
        assert!(LayerAtRule::is_layer_block_start("layer utilities {"));
        assert!(!LayerAtRule::is_layer_block_start("@layer base;"));
        assert!(!LayerAtRule::is_layer_block_start("@media screen {"));
    }

    #[test]
    fn test_parse_block_name() {
        assert_eq!(
            LayerAtRule::parse_block_name("@layer base {"),
            Some("base".to_string())
        );
        assert_eq!(
            LayerAtRule::parse_block_name("@layer framework.theme {"),
            Some("framework.theme".to_string())
        );
        assert_eq!(LayerAtRule::parse_block_name("@layer {"), None); // Anonymous
    }

    #[test]
    fn test_parse_layered_import() {
        let result = LayerAtRule::parse_layered_import("@import url(\"style.css\") layer(base);");

        if let Some(LayerAtRule::Import { url, layer_name }) = result {
            assert_eq!(url, "style.css");
            assert_eq!(layer_name, Some("base".to_string()));
        } else {
            panic!("Expected Import");
        }
    }

    #[test]
    fn test_parse_layered_import_anonymous() {
        let result = LayerAtRule::parse_layered_import("@import url(\"reset.css\") layer();");

        if let Some(LayerAtRule::Import { url, layer_name }) = result {
            assert_eq!(url, "reset.css");
            assert_eq!(layer_name, None); // Anonymous layer
        } else {
            panic!("Expected Import");
        }
    }

    #[test]
    fn test_parse_layered_import_quoted_url() {
        let result = LayerAtRule::parse_layered_import("@import \"theme.css\" layer(theme);");

        if let Some(LayerAtRule::Import { url, layer_name }) = result {
            assert_eq!(url, "theme.css");
            assert_eq!(layer_name, Some("theme".to_string()));
        } else {
            panic!("Expected Import");
        }
    }

    // ========================================================================
    // Layer Name Validation Tests
    // ========================================================================

    #[test]
    fn test_valid_layer_names() {
        assert!(is_valid_layer_name("base"));
        assert!(is_valid_layer_name("_private"));
        assert!(is_valid_layer_name("-custom"));
        assert!(is_valid_layer_name("layer123"));
        assert!(is_valid_layer_name("framework.theme"));
        assert!(is_valid_layer_name("a.b.c.d"));
    }

    #[test]
    fn test_invalid_layer_names() {
        assert!(!is_valid_layer_name("")); // Empty
        assert!(!is_valid_layer_name("123start")); // Starts with digit
        assert!(!is_valid_layer_name("initial")); // CSS-wide keyword
        assert!(!is_valid_layer_name("inherit")); // CSS-wide keyword
        assert!(!is_valid_layer_name("unset")); // CSS-wide keyword
        assert!(!is_valid_layer_name("revert")); // CSS-wide keyword
        assert!(!is_valid_layer_name("revert-layer")); // CSS-wide keyword
    }

    // ========================================================================
    // Integration Tests
    // ========================================================================

    #[test]
    fn test_full_layer_workflow() {
        let mut registry = LayerRegistry::new();

        // Step 1: Declare layer order (establishes priority)
        registry.declare_layer_order(&["reset", "base", "components", "utilities"]);

        // Step 2: Add rules to layers
        registry.add_rule_to_layer("reset", CssRule::new("* { margin: 0; padding: 0; }"));
        registry.add_rule_to_layer("base", CssRule::new("body { font-family: sans-serif; }"));
        registry.add_rule_to_layer(
            "components",
            CssRule::new(".button { padding: 10px 20px; }"),
        );
        registry.add_rule_to_layer("utilities", CssRule::new(".hidden { display: none; }"));

        // Step 3: Verify order
        let reset_id = registry.get_layer_id("reset").unwrap();
        let utilities_id = registry.get_layer_id("utilities").unwrap();
        let implicit_id = registry.implicit_layer_id();

        // reset < utilities < implicit
        assert_eq!(
            registry.compare_layers(reset_id, utilities_id),
            std::cmp::Ordering::Less
        );
        assert_eq!(
            registry.compare_layers(utilities_id, implicit_id),
            std::cmp::Ordering::Less
        );
    }

    #[test]
    fn test_layer_priority_unlayered_beats_layered() {
        let mut registry = LayerRegistry::new();

        // Declare layers
        registry.declare_layer("base");
        registry.declare_layer("utilities");

        let base_id = registry.get_layer_id("base").unwrap();
        let utilities_id = registry.get_layer_id("utilities").unwrap();
        let implicit_id = registry.implicit_layer_id();

        // Unlayered rules (implicit layer) should beat all named layers
        assert!(registry.order().priority(implicit_id) > registry.order().priority(base_id));
        assert!(registry.order().priority(implicit_id) > registry.order().priority(utilities_id));
    }

    #[test]
    fn test_nested_layer_ordering() {
        let mut registry = LayerRegistry::new();

        // Declare nested layers
        registry.declare_layer("framework");
        registry.declare_layer("framework.reset");
        registry.declare_layer("framework.theme");

        // All should exist
        assert!(registry.has_layer("framework"));
        assert!(registry.has_layer("framework.reset"));
        assert!(registry.has_layer("framework.theme"));

        // Order should be framework < framework.reset < framework.theme
        let fw_id = registry.get_layer_id("framework").unwrap();
        let reset_id = registry.get_layer_id("framework.reset").unwrap();
        let theme_id = registry.get_layer_id("framework.theme").unwrap();

        assert!(registry.order().priority(fw_id) < registry.order().priority(reset_id));
        assert!(registry.order().priority(reset_id) < registry.order().priority(theme_id));
    }

    #[test]
    fn test_anonymous_layer_ordering() {
        let mut registry = LayerRegistry::new();

        // Create anonymous layers
        let anon1 = registry.create_anonymous_layer();
        registry.declare_layer("named");
        let anon2 = registry.create_anonymous_layer();

        // Anonymous layers are ordered by creation time
        // anon1 < named < anon2
        let named_id = registry.get_layer_id("named").unwrap();

        assert!(registry.order().priority(anon1) < registry.order().priority(named_id));
        assert!(registry.order().priority(named_id) < registry.order().priority(anon2));
    }
}
