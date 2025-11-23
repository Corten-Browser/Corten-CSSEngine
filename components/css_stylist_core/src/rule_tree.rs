//! Rule Tree Implementation for Style Computation
//!
//! The rule tree is a data structure that enables efficient style sharing
//! between elements with the same cascade of matching rules. Instead of
//! storing computed values for each element, we share nodes in the tree.
//!
//! # Design
//!
//! The rule tree is a DAG (directed acyclic graph) where:
//! - Each node represents a rule in the cascade
//! - Paths from root to leaf represent complete cascades
//! - Elements share path prefixes when they have the same rules
//!
//! # Example
//!
//! ```text
//! Root
//! ├── div { color: red }
//! │   ├── .foo { margin: 10px }
//! │   │   └── #bar { padding: 5px }  <- Element A uses this path
//! │   └── .baz { margin: 20px }      <- Element B uses this path
//! └── span { color: blue }
//! ```

use css_cascade::{ApplicableRule, PropertyId, PropertyValue};
use css_types::Specificity;
use std::collections::HashMap;

/// Unique identifier for a rule node
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RuleNodeId(u32);

impl RuleNodeId {
    /// Create a new rule node ID
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    /// Get the raw ID value
    pub fn raw(self) -> u32 {
        self.0
    }
}

/// Source information for a rule
#[derive(Debug, Clone, PartialEq)]
pub enum RuleSource {
    /// Rule from author stylesheet
    Author {
        /// Stylesheet index
        stylesheet_index: usize,
        /// Rule index within stylesheet
        rule_index: usize,
    },
    /// Rule from user agent stylesheet
    UserAgent,
    /// Rule from user stylesheet
    User,
    /// Inline style attribute
    Inline,
}

impl Default for RuleSource {
    fn default() -> Self {
        RuleSource::Author {
            stylesheet_index: 0,
            rule_index: 0,
        }
    }
}

/// A declaration (property: value pair)
#[derive(Debug, Clone, PartialEq)]
pub struct PropertyDeclaration {
    /// The property being set
    pub property: PropertyId,
    /// The value for the property
    pub value: PropertyValue,
    /// Whether this declaration has !important
    pub important: bool,
}

impl PropertyDeclaration {
    /// Create a new property declaration
    pub fn new(property: PropertyId, value: PropertyValue, important: bool) -> Self {
        Self {
            property,
            value,
            important,
        }
    }

    /// Create a non-important declaration
    pub fn normal(property: PropertyId, value: PropertyValue) -> Self {
        Self::new(property, value, false)
    }

    /// Create an important declaration
    pub fn important(property: PropertyId, value: PropertyValue) -> Self {
        Self::new(property, value, true)
    }
}

/// A node in the rule tree
///
/// Each node represents one rule in the cascade. The path from root
/// to any node represents the complete cascade of rules.
#[derive(Debug, Clone)]
pub struct RuleNode {
    /// Declarations from this rule
    pub declarations: Vec<PropertyDeclaration>,
    /// Specificity of the selector that matched
    pub specificity: Specificity,
    /// Source of this rule
    pub source: RuleSource,
    /// Parent node (toward root)
    parent: Option<RuleNodeId>,
    /// Child nodes
    children: Vec<RuleNodeId>,
    /// Depth in the tree (root = 0)
    depth: u32,
}

impl RuleNode {
    /// Create a new rule node
    fn new(
        declarations: Vec<PropertyDeclaration>,
        specificity: Specificity,
        source: RuleSource,
        parent: Option<RuleNodeId>,
        depth: u32,
    ) -> Self {
        Self {
            declarations,
            specificity,
            source,
            parent,
            children: Vec::new(),
            depth,
        }
    }

    /// Create a root node
    fn root() -> Self {
        Self::new(
            Vec::new(),
            Specificity::zero(),
            RuleSource::UserAgent,
            None,
            0,
        )
    }

    /// Get the parent node ID
    pub fn parent(&self) -> Option<RuleNodeId> {
        self.parent
    }

    /// Get the depth in the tree
    pub fn depth(&self) -> u32 {
        self.depth
    }

    /// Get the children node IDs
    pub fn children(&self) -> &[RuleNodeId] {
        &self.children
    }

    /// Check if this is a root node
    pub fn is_root(&self) -> bool {
        self.parent.is_none()
    }

    /// Check if this is a leaf node
    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
}

/// Computed style from a rule tree path
#[derive(Debug, Clone, Default)]
pub struct ComputedStyle {
    /// Property values indexed by property ID
    properties: HashMap<PropertyId, PropertyValue>,
}

impl ComputedStyle {
    /// Create a new empty computed style
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }

    /// Set a property value
    pub fn set(&mut self, property: PropertyId, value: PropertyValue) {
        self.properties.insert(property, value);
    }

    /// Get a property value
    pub fn get(&self, property: &PropertyId) -> Option<&PropertyValue> {
        self.properties.get(property)
    }

    /// Check if a property is set
    pub fn contains(&self, property: &PropertyId) -> bool {
        self.properties.contains_key(property)
    }

    /// Get all properties
    pub fn properties(&self) -> &HashMap<PropertyId, PropertyValue> {
        &self.properties
    }

    /// Get the number of properties
    pub fn len(&self) -> usize {
        self.properties.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.properties.is_empty()
    }
}

/// The rule tree structure
///
/// This is the main data structure for style sharing. It stores all
/// rule nodes and provides methods to insert rules and compute styles.
pub struct RuleTree {
    /// All nodes in the tree
    nodes: Vec<RuleNode>,
    /// The root node ID (always 0)
    root: RuleNodeId,
    /// Index for finding existing children (parent_id, declarations_hash) -> child_id
    child_index: HashMap<(RuleNodeId, u64), RuleNodeId>,
}

impl RuleTree {
    /// Create a new empty rule tree
    ///
    /// # Examples
    ///
    /// ```
    /// use css_stylist_core::rule_tree::RuleTree;
    ///
    /// let tree = RuleTree::new();
    /// assert_eq!(tree.node_count(), 1); // Root node
    /// ```
    pub fn new() -> Self {
        let root = RuleNode::root();
        Self {
            nodes: vec![root],
            root: RuleNodeId(0),
            child_index: HashMap::new(),
        }
    }

    /// Get the root node ID
    pub fn root(&self) -> RuleNodeId {
        self.root
    }

    /// Get a node by ID
    pub fn get(&self, id: RuleNodeId) -> Option<&RuleNode> {
        self.nodes.get(id.0 as usize)
    }

    /// Get a mutable node by ID
    fn get_mut(&mut self, id: RuleNodeId) -> Option<&mut RuleNode> {
        self.nodes.get_mut(id.0 as usize)
    }

    /// Get the total number of nodes
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Insert a rule into the tree
    ///
    /// This finds or creates a child node under the given parent with
    /// the specified declarations and specificity.
    ///
    /// # Arguments
    ///
    /// * `declarations` - The property declarations from the rule
    /// * `specificity` - The specificity of the selector
    ///
    /// # Returns
    ///
    /// The ID of the node representing this rule in the cascade
    ///
    /// # Examples
    ///
    /// ```
    /// use css_stylist_core::rule_tree::{RuleTree, PropertyDeclaration};
    /// use css_cascade::PropertyId;
    /// use css_cascade::PropertyValue;
    /// use css_types::Specificity;
    ///
    /// let mut tree = RuleTree::new();
    /// let decls = vec![
    ///     PropertyDeclaration::normal(PropertyId::Color, PropertyValue::Keyword("red".to_string())),
    /// ];
    /// let node_id = tree.insert_rule(decls, Specificity::new(0, 1, 0));
    /// ```
    pub fn insert_rule(
        &mut self,
        declarations: Vec<PropertyDeclaration>,
        specificity: Specificity,
    ) -> RuleNodeId {
        self.insert_rule_at(
            self.root,
            declarations,
            specificity,
            RuleSource::Author {
                stylesheet_index: 0,
                rule_index: 0,
            },
        )
    }

    /// Insert a rule at a specific parent node
    ///
    /// This is useful for building cascades incrementally.
    pub fn insert_rule_at(
        &mut self,
        parent: RuleNodeId,
        declarations: Vec<PropertyDeclaration>,
        specificity: Specificity,
        source: RuleSource,
    ) -> RuleNodeId {
        // Compute a simple hash of declarations for indexing
        let decl_hash = self.hash_declarations(&declarations);
        let key = (parent, decl_hash);

        // Check if we already have this child
        if let Some(&existing_id) = self.child_index.get(&key) {
            return existing_id;
        }

        // Get parent depth
        let parent_depth = self.get(parent).map(|n| n.depth).unwrap_or(0);

        // Create new node
        let new_id = RuleNodeId(self.nodes.len() as u32);
        let node = RuleNode::new(
            declarations,
            specificity,
            source,
            Some(parent),
            parent_depth + 1,
        );
        self.nodes.push(node);

        // Add to parent's children
        if let Some(parent_node) = self.get_mut(parent) {
            parent_node.children.push(new_id);
        }

        // Add to index
        self.child_index.insert(key, new_id);

        new_id
    }

    /// Get the path from root to a node
    ///
    /// Returns nodes in order from root to the specified node.
    ///
    /// # Examples
    ///
    /// ```
    /// use css_stylist_core::rule_tree::{RuleTree, PropertyDeclaration};
    /// use css_cascade::PropertyId;
    /// use css_cascade::PropertyValue;
    /// use css_types::Specificity;
    ///
    /// let mut tree = RuleTree::new();
    /// let decls1 = vec![PropertyDeclaration::normal(PropertyId::Color, PropertyValue::Keyword("red".to_string()))];
    /// let node1 = tree.insert_rule(decls1, Specificity::new(0, 1, 0));
    ///
    /// let path = tree.get_path(node1);
    /// assert_eq!(path.len(), 2); // Root + our node
    /// ```
    pub fn get_path(&self, node_id: RuleNodeId) -> Vec<&RuleNode> {
        let mut path = Vec::new();
        let mut current = Some(node_id);

        while let Some(id) = current {
            if let Some(node) = self.get(id) {
                path.push(node);
                current = node.parent;
            } else {
                break;
            }
        }

        // Reverse to get root-to-node order
        path.reverse();
        path
    }

    /// Compute the style for a node
    ///
    /// This walks from root to the node, applying declarations in order.
    /// Later declarations override earlier ones (cascade order).
    ///
    /// # Examples
    ///
    /// ```
    /// use css_stylist_core::rule_tree::{RuleTree, PropertyDeclaration};
    /// use css_cascade::PropertyId;
    /// use css_cascade::PropertyValue;
    /// use css_types::Specificity;
    ///
    /// let mut tree = RuleTree::new();
    /// let decls = vec![PropertyDeclaration::normal(PropertyId::Color, PropertyValue::Keyword("red".to_string()))];
    /// let node_id = tree.insert_rule(decls, Specificity::new(0, 1, 0));
    ///
    /// let style = tree.compute_style(node_id);
    /// assert!(style.get(&PropertyId::Color).is_some());
    /// ```
    pub fn compute_style(&self, node_id: RuleNodeId) -> ComputedStyle {
        let path = self.get_path(node_id);
        let mut style = ComputedStyle::new();

        // Collect all declarations, separating normal and important
        let mut normal_decls: Vec<(&PropertyDeclaration, usize)> = Vec::new();
        let mut important_decls: Vec<(&PropertyDeclaration, usize)> = Vec::new();

        for (order, node) in path.iter().enumerate() {
            for decl in &node.declarations {
                if decl.important {
                    important_decls.push((decl, order));
                } else {
                    normal_decls.push((decl, order));
                }
            }
        }

        // Apply normal declarations first
        for (decl, _) in normal_decls {
            style.set(decl.property, decl.value.clone());
        }

        // Then apply important declarations (they override normal)
        for (decl, _) in important_decls {
            style.set(decl.property, decl.value.clone());
        }

        style
    }

    /// Insert a chain of rules (for building a complete cascade)
    ///
    /// This inserts multiple rules in sequence, each becoming a child
    /// of the previous one.
    pub fn insert_rule_chain(
        &mut self,
        rules: Vec<(Vec<PropertyDeclaration>, Specificity, RuleSource)>,
    ) -> RuleNodeId {
        let mut current = self.root;

        for (declarations, specificity, source) in rules {
            current = self.insert_rule_at(current, declarations, specificity, source);
        }

        current
    }

    /// Simple hash function for declarations
    fn hash_declarations(&self, declarations: &[PropertyDeclaration]) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        for decl in declarations {
            // Hash property ID
            (decl.property as u8).hash(&mut hasher);
            // Hash importance
            decl.important.hash(&mut hasher);
            // Hash the value content for proper distinction
            Self::hash_property_value(&decl.value, &mut hasher);
        }
        hasher.finish()
    }

    /// Hash a PropertyValue including its content
    fn hash_property_value<H: std::hash::Hasher>(value: &PropertyValue, hasher: &mut H) {
        use std::hash::Hash;

        // First hash the discriminant
        std::mem::discriminant(value).hash(hasher);

        // Then hash the content
        match value {
            PropertyValue::Keyword(s) => s.hash(hasher),
            PropertyValue::Length(v, unit) => {
                v.to_bits().hash(hasher);
                unit.hash(hasher);
            }
            PropertyValue::Number(v) => v.to_bits().hash(hasher),
            PropertyValue::FontFamily(families) => families.hash(hasher),
            PropertyValue::Border {
                width,
                style,
                color,
            } => {
                width.to_bits().hash(hasher);
                style.hash(hasher);
                color.hash(hasher);
            }
            PropertyValue::Important(inner) => {
                Self::hash_property_value(inner, hasher);
            }
            PropertyValue::Inherit => {}
        }
    }

    /// Convert an ApplicableRule to PropertyDeclarations
    pub fn from_applicable_rule(rule: &ApplicableRule) -> Vec<PropertyDeclaration> {
        rule.rule
            .declarations
            .iter()
            .map(|(prop, val)| {
                let important = matches!(val, PropertyValue::Important(_));
                let value = if let PropertyValue::Important(inner) = val {
                    (**inner).clone()
                } else {
                    val.clone()
                };
                PropertyDeclaration::new(*prop, value, important)
            })
            .collect()
    }
}

impl Default for RuleTree {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use css_cascade::{Origin, StyleRule};

    fn make_color_decl(color: &str) -> PropertyDeclaration {
        PropertyDeclaration::normal(PropertyId::Color, PropertyValue::Keyword(color.to_string()))
    }

    fn make_margin_decl(value: f64) -> PropertyDeclaration {
        PropertyDeclaration::normal(
            PropertyId::Margin,
            PropertyValue::Length(value, "px".to_string()),
        )
    }

    #[test]
    fn test_rule_tree_new() {
        let tree = RuleTree::new();
        assert_eq!(tree.node_count(), 1);
        assert!(tree.get(tree.root()).unwrap().is_root());
    }

    #[test]
    fn test_rule_node_id() {
        let id = RuleNodeId::new(42);
        assert_eq!(id.raw(), 42);
    }

    #[test]
    fn test_property_declaration_normal() {
        let decl = PropertyDeclaration::normal(
            PropertyId::Color,
            PropertyValue::Keyword("red".to_string()),
        );
        assert!(!decl.important);
        assert_eq!(decl.property, PropertyId::Color);
    }

    #[test]
    fn test_property_declaration_important() {
        let decl = PropertyDeclaration::important(
            PropertyId::Color,
            PropertyValue::Keyword("red".to_string()),
        );
        assert!(decl.important);
    }

    #[test]
    fn test_insert_single_rule() {
        let mut tree = RuleTree::new();
        let decls = vec![make_color_decl("red")];
        let node_id = tree.insert_rule(decls, Specificity::new(0, 1, 0));

        assert_eq!(tree.node_count(), 2);
        let node = tree.get(node_id).unwrap();
        assert!(!node.is_root());
        assert!(node.is_leaf());
        assert_eq!(node.depth(), 1);
    }

    #[test]
    fn test_insert_rule_chain() {
        let mut tree = RuleTree::new();

        let rules = vec![
            (
                vec![make_color_decl("red")],
                Specificity::new(0, 0, 1),
                RuleSource::UserAgent,
            ),
            (
                vec![make_margin_decl(10.0)],
                Specificity::new(0, 1, 0),
                RuleSource::Author {
                    stylesheet_index: 0,
                    rule_index: 0,
                },
            ),
        ];

        let leaf_id = tree.insert_rule_chain(rules);
        assert_eq!(tree.node_count(), 3);

        let path = tree.get_path(leaf_id);
        assert_eq!(path.len(), 3); // Root + 2 rules
    }

    #[test]
    fn test_get_path() {
        let mut tree = RuleTree::new();
        let decls1 = vec![make_color_decl("red")];
        let node1 = tree.insert_rule(decls1, Specificity::new(0, 1, 0));

        let decls2 = vec![make_margin_decl(10.0)];
        let node2 = tree.insert_rule_at(
            node1,
            decls2,
            Specificity::new(0, 1, 0),
            RuleSource::default(),
        );

        let path = tree.get_path(node2);
        assert_eq!(path.len(), 3);
        assert!(path[0].is_root());
        assert!(!path[1].is_root());
        assert!(!path[2].is_root());
    }

    #[test]
    fn test_compute_style_single_rule() {
        let mut tree = RuleTree::new();
        let decls = vec![make_color_decl("red")];
        let node_id = tree.insert_rule(decls, Specificity::new(0, 1, 0));

        let style = tree.compute_style(node_id);
        assert!(style.get(&PropertyId::Color).is_some());
        assert_eq!(
            style.get(&PropertyId::Color),
            Some(&PropertyValue::Keyword("red".to_string()))
        );
    }

    #[test]
    fn test_compute_style_cascade_override() {
        let mut tree = RuleTree::new();

        // First rule: color red
        let decls1 = vec![make_color_decl("red")];
        let node1 = tree.insert_rule(decls1, Specificity::new(0, 0, 1));

        // Second rule: color blue (should override)
        let decls2 = vec![make_color_decl("blue")];
        let node2 = tree.insert_rule_at(
            node1,
            decls2,
            Specificity::new(0, 1, 0),
            RuleSource::default(),
        );

        let style = tree.compute_style(node2);
        assert_eq!(
            style.get(&PropertyId::Color),
            Some(&PropertyValue::Keyword("blue".to_string()))
        );
    }

    #[test]
    fn test_compute_style_important() {
        let mut tree = RuleTree::new();

        // First rule: color red with !important
        let decls1 = vec![PropertyDeclaration::important(
            PropertyId::Color,
            PropertyValue::Keyword("red".to_string()),
        )];
        let node1 = tree.insert_rule(decls1, Specificity::new(0, 0, 1));

        // Second rule: color blue (normal, should NOT override)
        let decls2 = vec![make_color_decl("blue")];
        let node2 = tree.insert_rule_at(
            node1,
            decls2,
            Specificity::new(0, 1, 0),
            RuleSource::default(),
        );

        let style = tree.compute_style(node2);
        // Important declarations win
        assert_eq!(
            style.get(&PropertyId::Color),
            Some(&PropertyValue::Keyword("red".to_string()))
        );
    }

    #[test]
    fn test_style_sharing() {
        let mut tree = RuleTree::new();

        // Insert same rule twice - should reuse node
        let decls1 = vec![make_color_decl("red")];
        let node1 = tree.insert_rule(decls1.clone(), Specificity::new(0, 1, 0));
        let node2 = tree.insert_rule(decls1, Specificity::new(0, 1, 0));

        assert_eq!(node1, node2);
        assert_eq!(tree.node_count(), 2); // Root + 1 shared node
    }

    #[test]
    fn test_computed_style_methods() {
        let mut style = ComputedStyle::new();
        assert!(style.is_empty());
        assert_eq!(style.len(), 0);

        style.set(PropertyId::Color, PropertyValue::Keyword("red".to_string()));
        assert!(!style.is_empty());
        assert_eq!(style.len(), 1);
        assert!(style.contains(&PropertyId::Color));
        assert!(!style.contains(&PropertyId::Margin));
    }

    #[test]
    fn test_rule_source_default() {
        let source = RuleSource::default();
        assert!(matches!(
            source,
            RuleSource::Author {
                stylesheet_index: 0,
                rule_index: 0
            }
        ));
    }

    #[test]
    fn test_rule_node_accessors() {
        let mut tree = RuleTree::new();
        let decls = vec![make_color_decl("red")];
        let node_id = tree.insert_rule(decls, Specificity::new(0, 1, 0));

        let root = tree.get(tree.root()).unwrap();
        assert_eq!(root.children().len(), 1);
        assert_eq!(root.children()[0], node_id);

        let node = tree.get(node_id).unwrap();
        assert_eq!(node.parent(), Some(tree.root()));
    }

    #[test]
    fn test_multiple_branches() {
        let mut tree = RuleTree::new();

        // Branch 1: red -> margin 10
        let decls1 = vec![make_color_decl("red")];
        let node1 = tree.insert_rule(decls1, Specificity::new(0, 1, 0));
        let decls2 = vec![make_margin_decl(10.0)];
        let leaf1 = tree.insert_rule_at(
            node1,
            decls2,
            Specificity::new(0, 1, 0),
            RuleSource::default(),
        );

        // Branch 2: red -> margin 20 (different branch)
        let decls3 = vec![make_margin_decl(20.0)];
        let leaf2 = tree.insert_rule_at(
            node1,
            decls3,
            Specificity::new(0, 1, 0),
            RuleSource::default(),
        );

        // Should share the "red" node
        assert_eq!(tree.node_count(), 4); // Root + red + margin10 + margin20

        // Different computed styles
        let style1 = tree.compute_style(leaf1);
        let style2 = tree.compute_style(leaf2);

        assert_eq!(
            style1.get(&PropertyId::Margin),
            Some(&PropertyValue::Length(10.0, "px".to_string()))
        );
        assert_eq!(
            style2.get(&PropertyId::Margin),
            Some(&PropertyValue::Length(20.0, "px".to_string()))
        );

        // Both should have red color
        assert_eq!(
            style1.get(&PropertyId::Color),
            Some(&PropertyValue::Keyword("red".to_string()))
        );
        assert_eq!(
            style2.get(&PropertyId::Color),
            Some(&PropertyValue::Keyword("red".to_string()))
        );
    }

    #[test]
    fn test_from_applicable_rule() {
        let rule = ApplicableRule {
            rule: StyleRule {
                declarations: vec![
                    (PropertyId::Color, PropertyValue::Keyword("red".to_string())),
                    (
                        PropertyId::Margin,
                        PropertyValue::Important(Box::new(PropertyValue::Length(
                            10.0,
                            "px".to_string(),
                        ))),
                    ),
                ],
            },
            specificity: Specificity::new(0, 1, 0),
            origin: Origin::Author,
            source_order: 0,
        };

        let decls = RuleTree::from_applicable_rule(&rule);
        assert_eq!(decls.len(), 2);
        assert!(!decls[0].important);
        assert!(decls[1].important);
    }
}
