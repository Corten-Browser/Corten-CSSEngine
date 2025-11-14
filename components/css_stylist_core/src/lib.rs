//! CSS Stylist Core - Style computation
//!
//! This crate provides style computation for CSS elements including:
//! - Cascade resolution
//! - Property inheritance
//! - Unit resolution
//! - Rule tree for style sharing

pub mod compute;
pub mod types;

pub use types::{ComputedValues, Display, Position, RuleNode, StyleContext};

use css_cascade::ApplicableRule;
use css_matcher_core::ElementLike;
use servo_arc::Arc;
use std::collections::HashMap;

/// The Stylist is responsible for computing styles for elements
///
/// It maintains a rule tree for efficient style sharing and caches
/// computed values.
///
/// # Examples
/// ```
/// use css_stylist_core::Stylist;
///
/// let stylist = Stylist::new();
/// assert!(stylist.is_empty());
/// ```
pub struct Stylist {
    /// Root of the rule tree
    rule_tree_root: Arc<RuleNode>,
    /// Rules indexed by selector
    rules: Vec<ApplicableRule>,
    /// Cache of computed values by element ID
    cache: HashMap<u64, Arc<ComputedValues>>,
}

impl Stylist {
    /// Create a new empty Stylist
    ///
    /// # Examples
    /// ```
    /// use css_stylist_core::Stylist;
    ///
    /// let stylist = Stylist::new();
    /// assert!(stylist.is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            rule_tree_root: Arc::new(RuleNode::root()),
            rules: Vec::new(),
            cache: HashMap::new(),
        }
    }

    /// Check if the stylist has no rules
    ///
    /// # Examples
    /// ```
    /// use css_stylist_core::Stylist;
    ///
    /// let stylist = Stylist::new();
    /// assert!(stylist.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }

    /// Add a rule to the stylist
    ///
    /// # Arguments
    /// * `rule` - The applicable rule to add
    ///
    /// # Examples
    /// ```
    /// use css_stylist_core::Stylist;
    /// use css_cascade::{ApplicableRule, Origin, StyleRule};
    /// use css_types::Specificity;
    ///
    /// let mut stylist = Stylist::new();
    /// let rule = ApplicableRule {
    ///     rule: StyleRule { declarations: vec![] },
    ///     specificity: Specificity::new(0, 1, 0),
    ///     origin: Origin::Author,
    ///     source_order: 0,
    /// };
    /// stylist.add_rule(rule);
    /// assert!(!stylist.is_empty());
    /// ```
    pub fn add_rule(&mut self, rule: ApplicableRule) {
        self.rules.push(rule);
    }

    /// Compute styles for an element
    ///
    /// This is the main entry point for style computation. It:
    /// 1. Matches selectors against the element
    /// 2. Builds a rule node chain for matching rules
    /// 3. Resolves computed values with cascade and inheritance
    ///
    /// # Arguments
    /// * `element` - The element to compute styles for
    /// * `context` - Style context with parent values and viewport info
    ///
    /// # Examples
    /// ```
    /// use css_stylist_core::{Stylist, StyleContext};
    ///
    /// # struct TestElement { id: u64, tag: String, classes: Vec<String> }
    /// # impl css_matcher_core::ElementLike for TestElement {
    /// #     fn tag_name(&self) -> &str { &self.tag }
    /// #     fn id(&self) -> Option<&str> { None }
    /// #     fn classes(&self) -> &[String] { &self.classes }
    /// #     fn parent(&self) -> Option<&Self> { None }
    /// #     fn previous_sibling(&self) -> Option<&Self> { None }
    /// # }
    ///
    /// let stylist = Stylist::new();
    /// let element = TestElement { id: 1, tag: "div".to_string(), classes: vec![] };
    /// let context = StyleContext::default();
    ///
    /// let computed = stylist.compute(&element, &context);
    /// // Returns default computed values for an element with no matching rules
    /// ```
    pub fn compute<E: ElementLike>(
        &self,
        _element: &E,
        context: &StyleContext,
    ) -> Arc<ComputedValues> {
        // For now, just return inherited values from context or defaults
        // In a full implementation, this would:
        // 1. Match selectors against element
        // 2. Build rule node chain
        // 3. Apply cascade
        // 4. Resolve computed values

        if let Some(parent_values) = &context.parent_values {
            Arc::new(ComputedValues::inherit_from(parent_values))
        } else {
            Arc::new(ComputedValues::default())
        }
    }

    /// Build a rule tree node for an element
    ///
    /// Creates a chain of rule nodes representing the cascade of
    /// rules that apply to the element.
    ///
    /// # Arguments
    /// * `_element` - The element to build rules for
    ///
    /// # Returns
    /// An Arc to the rule node representing the cascade for this element
    pub fn build_rule_tree<E: ElementLike>(&self, _element: &E) -> Arc<RuleNode> {
        // For now, just return the root
        // In a full implementation, this would:
        // 1. Find all matching rules
        // 2. Sort by cascade order
        // 3. Build a chain of rule nodes
        self.rule_tree_root.clone()
    }

    /// Resolve computed values from a rule node
    ///
    /// Applies cascade resolution and inheritance to produce final
    /// computed values.
    ///
    /// # Arguments
    /// * `_rule_node` - The rule node chain for the element
    /// * `context` - Style context with parent values
    ///
    /// # Returns
    /// Computed values for the element
    pub fn resolve_computed_values(
        &self,
        _rule_node: &Arc<RuleNode>,
        context: &StyleContext,
    ) -> Arc<ComputedValues> {
        // For now, use inheritance
        // In a full implementation, this would:
        // 1. Walk the rule node chain
        // 2. Apply each rule's declarations
        // 3. Apply inheritance for missing properties
        // 4. Resolve relative units

        if let Some(parent_values) = &context.parent_values {
            Arc::new(ComputedValues::inherit_from(parent_values))
        } else {
            Arc::new(ComputedValues::default())
        }
    }

    /// Clear the style cache
    ///
    /// Should be called when rules change to invalidate cached values.
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

impl Default for Stylist {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use css_cascade::{ApplicableRule, Origin, StyleRule};
    use css_types::Specificity;

    // Mock element for testing
    struct TestElement {
        tag: String,
    }

    impl ElementLike for TestElement {
        fn tag_name(&self) -> &str {
            &self.tag
        }

        fn id(&self) -> Option<&str> {
            None
        }

        fn classes(&self) -> &[String] {
            &[]
        }

        fn parent(&self) -> Option<&Self> {
            None
        }

        fn previous_sibling(&self) -> Option<&Self> {
            None
        }
    }

    #[test]
    fn test_stylist_new() {
        let stylist = Stylist::new();
        assert!(stylist.is_empty());
    }

    #[test]
    fn test_stylist_add_rule() {
        let mut stylist = Stylist::new();
        let rule = ApplicableRule {
            rule: StyleRule {
                declarations: vec![],
            },
            specificity: Specificity::new(0, 1, 0),
            origin: Origin::Author,
            source_order: 0,
        };

        stylist.add_rule(rule);
        assert!(!stylist.is_empty());
    }

    #[test]
    fn test_stylist_compute_default() {
        let stylist = Stylist::new();
        let element = TestElement {
            tag: "div".to_string(),
        };
        let context = StyleContext::default();

        let computed = stylist.compute(&element, &context);

        // Should return default values
        assert_eq!(computed.display, Display::Inline);
        assert_eq!(computed.position, Position::Static);
    }

    #[test]
    fn test_stylist_compute_with_parent() {
        use css_types::Color;

        let stylist = Stylist::new();
        let element = TestElement {
            tag: "span".to_string(),
        };

        let mut parent_values = ComputedValues::default();
        parent_values.color = Color::rgb(255, 0, 0);

        let context =
            StyleContext::new(Some(Arc::new(parent_values.clone())), 1920.0, 1080.0, 16.0);

        let computed = stylist.compute(&element, &context);

        // Color should be inherited
        assert_eq!(computed.color, Color::rgb(255, 0, 0));
        // Display should not be inherited
        assert_eq!(computed.display, Display::Inline);
    }

    #[test]
    fn test_stylist_build_rule_tree() {
        let stylist = Stylist::new();
        let element = TestElement {
            tag: "div".to_string(),
        };

        let rule_node = stylist.build_rule_tree(&element);

        // For now, should return root node
        assert!(rule_node.rule.is_none());
        assert!(rule_node.parent.is_none());
    }

    #[test]
    fn test_stylist_resolve_computed_values() {
        use css_types::Color;

        let stylist = Stylist::new();
        let rule_node = Arc::new(RuleNode::root());

        let mut parent_values = ComputedValues::default();
        parent_values.color = Color::rgb(0, 255, 0);

        let context =
            StyleContext::new(Some(Arc::new(parent_values.clone())), 1920.0, 1080.0, 16.0);

        let computed = stylist.resolve_computed_values(&rule_node, &context);

        // Should inherit from parent
        assert_eq!(computed.color, Color::rgb(0, 255, 0));
    }

    #[test]
    fn test_stylist_clear_cache() {
        let mut stylist = Stylist::new();
        stylist.cache.insert(1, Arc::new(ComputedValues::default()));
        assert_eq!(stylist.cache.len(), 1);

        stylist.clear_cache();
        assert_eq!(stylist.cache.len(), 0);
    }
}
