//! Types for style computation
//!
//! This module defines the core types used in style computation:
//! - ComputedValues: Computed CSS properties for an element
//! - RuleNode: Node in the rule tree for style sharing
//! - StyleContext: Context for style computation

use css_cascade::ApplicableRule;
use css_types::{Color, Length, LengthUnit};
use servo_arc::Arc;

/// CSS Display property
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Display {
    /// Block display
    Block,
    /// Inline display
    Inline,
    /// Inline block
    InlineBlock,
    /// None (hidden)
    None,
    /// Flex
    Flex,
}

/// CSS Position property
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Position {
    /// Static positioning
    Static,
    /// Relative positioning
    Relative,
    /// Absolute positioning
    Absolute,
    /// Fixed positioning
    Fixed,
}

/// Computed style values for an element
///
/// Contains the final computed values for all CSS properties after
/// cascade, inheritance, and unit resolution.
#[derive(Debug, Clone, PartialEq)]
pub struct ComputedValues {
    // Box model properties
    /// Display property
    pub display: Display,
    /// Position property
    pub position: Position,
    /// Width property
    pub width: Length,
    /// Height property
    pub height: Length,

    // Margin properties
    /// Margin top
    pub margin_top: Length,
    /// Margin right
    pub margin_right: Length,
    /// Margin bottom
    pub margin_bottom: Length,
    /// Margin left
    pub margin_left: Length,

    // Padding properties
    /// Padding top
    pub padding_top: Length,
    /// Padding right
    pub padding_right: Length,
    /// Padding bottom
    pub padding_bottom: Length,
    /// Padding left
    pub padding_left: Length,

    // Text properties
    /// Text color
    pub color: Color,
    /// Font size
    pub font_size: Length,
}

impl Default for ComputedValues {
    /// Create default computed values
    ///
    /// Returns computed values with initial CSS values per CSS2.1 spec.
    ///
    /// # Examples
    /// ```
    /// use css_stylist_core::types::{ComputedValues, Display};
    ///
    /// let values = ComputedValues::default();
    /// assert_eq!(values.display, Display::Inline);
    /// ```
    fn default() -> Self {
        Self {
            display: Display::Inline,
            position: Position::Static,
            width: Length::new(0.0, LengthUnit::Px), // Auto is represented as 0px for now
            height: Length::new(0.0, LengthUnit::Px),
            margin_top: Length::new(0.0, LengthUnit::Px),
            margin_right: Length::new(0.0, LengthUnit::Px),
            margin_bottom: Length::new(0.0, LengthUnit::Px),
            margin_left: Length::new(0.0, LengthUnit::Px),
            padding_top: Length::new(0.0, LengthUnit::Px),
            padding_right: Length::new(0.0, LengthUnit::Px),
            padding_bottom: Length::new(0.0, LengthUnit::Px),
            padding_left: Length::new(0.0, LengthUnit::Px),
            color: Color::rgb(0, 0, 0),
            font_size: Length::new(16.0, LengthUnit::Px),
        }
    }
}

impl ComputedValues {
    /// Inherit properties from parent
    ///
    /// Creates computed values by inheriting inherited properties from parent
    /// and using initial values for non-inherited properties.
    ///
    /// # Arguments
    /// * `parent` - Parent element's computed values
    ///
    /// # Examples
    /// ```
    /// use css_stylist_core::types::{ComputedValues, Display};
    /// use css_types::{Color, Length, LengthUnit};
    ///
    /// let parent = ComputedValues::default();
    /// let child = ComputedValues::inherit_from(&parent);
    ///
    /// // Color is inherited
    /// assert_eq!(child.color, parent.color);
    /// // Display is not inherited
    /// assert_eq!(child.display, Display::Inline);
    /// ```
    pub fn inherit_from(parent: &ComputedValues) -> Self {
        Self {
            // Non-inherited properties get initial values
            display: Display::Inline,
            position: Position::Static,
            width: Length::new(0.0, LengthUnit::Px),
            height: Length::new(0.0, LengthUnit::Px),
            margin_top: Length::new(0.0, LengthUnit::Px),
            margin_right: Length::new(0.0, LengthUnit::Px),
            margin_bottom: Length::new(0.0, LengthUnit::Px),
            margin_left: Length::new(0.0, LengthUnit::Px),
            padding_top: Length::new(0.0, LengthUnit::Px),
            padding_right: Length::new(0.0, LengthUnit::Px),
            padding_bottom: Length::new(0.0, LengthUnit::Px),
            padding_left: Length::new(0.0, LengthUnit::Px),

            // Inherited properties come from parent
            color: parent.color,
            font_size: parent.font_size,
        }
    }
}

/// Node in the rule tree
///
/// Rule tree is used for style sharing - multiple elements with the same
/// matching rules can share the same rule node.
#[derive(Debug, Clone)]
pub struct RuleNode {
    /// Applicable rule for this node
    pub rule: Option<ApplicableRule>,
    /// Parent node in the tree
    pub parent: Option<Arc<RuleNode>>,
    /// Cached computed values for this rule combination
    pub cached_values: Option<Arc<ComputedValues>>,
}

impl RuleNode {
    /// Create a new root rule node
    ///
    /// # Examples
    /// ```
    /// use css_stylist_core::types::RuleNode;
    ///
    /// let root = RuleNode::root();
    /// assert!(root.rule.is_none());
    /// assert!(root.parent.is_none());
    /// ```
    pub fn root() -> Self {
        Self {
            rule: None,
            parent: None,
            cached_values: None,
        }
    }

    /// Create a new rule node with a parent
    ///
    /// # Arguments
    /// * `rule` - The applicable rule for this node
    /// * `parent` - The parent node
    ///
    /// # Examples
    /// ```
    /// use css_stylist_core::types::RuleNode;
    /// use css_cascade::{ApplicableRule, Origin, StyleRule};
    /// use css_types::Specificity;
    /// use servo_arc::Arc;
    ///
    /// let root = Arc::new(RuleNode::root());
    /// let rule = ApplicableRule {
    ///     rule: StyleRule { declarations: vec![] },
    ///     specificity: Specificity::new(0, 1, 0),
    ///     origin: Origin::Author,
    ///     source_order: 0,
    /// };
    /// let node = RuleNode::new(rule, Some(root));
    /// assert!(node.rule.is_some());
    /// ```
    pub fn new(rule: ApplicableRule, parent: Option<Arc<RuleNode>>) -> Self {
        Self {
            rule: Some(rule),
            parent,
            cached_values: None,
        }
    }
}

/// Context for style computation
///
/// Provides context needed for computing styles including parent styles
/// and viewport information.
#[derive(Debug, Clone)]
pub struct StyleContext {
    /// Parent element's computed values (for inheritance)
    pub parent_values: Option<Arc<ComputedValues>>,
    /// Viewport width in pixels
    pub viewport_width: f32,
    /// Viewport height in pixels
    pub viewport_height: f32,
    /// Root font size for rem units
    pub root_font_size: f32,
}

impl StyleContext {
    /// Create a new style context
    ///
    /// # Arguments
    /// * `parent_values` - Parent's computed values (None for root)
    /// * `viewport_width` - Viewport width in pixels
    /// * `viewport_height` - Viewport height in pixels
    /// * `root_font_size` - Root font size in pixels
    ///
    /// # Examples
    /// ```
    /// use css_stylist_core::types::StyleContext;
    ///
    /// let context = StyleContext::new(None, 1920.0, 1080.0, 16.0);
    /// assert_eq!(context.viewport_width, 1920.0);
    /// assert_eq!(context.root_font_size, 16.0);
    /// ```
    pub fn new(
        parent_values: Option<Arc<ComputedValues>>,
        viewport_width: f32,
        viewport_height: f32,
        root_font_size: f32,
    ) -> Self {
        Self {
            parent_values,
            viewport_width,
            viewport_height,
            root_font_size,
        }
    }
}

impl Default for StyleContext {
    /// Create a default style context for testing
    ///
    /// Uses standard defaults: 1920x1080 viewport, 16px root font size.
    ///
    /// # Examples
    /// ```
    /// use css_stylist_core::types::StyleContext;
    ///
    /// let context = StyleContext::default();
    /// assert_eq!(context.viewport_width, 1920.0);
    /// ```
    fn default() -> Self {
        Self::new(None, 1920.0, 1080.0, 16.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use css_types::{Color, LengthUnit, Specificity};

    #[test]
    fn test_computed_values_default() {
        let values = ComputedValues::default();

        assert_eq!(values.display, Display::Inline);
        assert_eq!(values.position, Position::Static);
        assert_eq!(values.width.value(), 0.0);
        assert_eq!(values.width.unit(), LengthUnit::Px);
        assert_eq!(values.margin_top.value(), 0.0);
        assert_eq!(values.color, Color::rgb(0, 0, 0));
        assert_eq!(values.font_size.value(), 16.0);
    }

    #[test]
    fn test_computed_values_inherit_from() {
        let mut parent = ComputedValues::default();
        parent.color = Color::rgb(255, 0, 0);
        parent.font_size = Length::new(20.0, LengthUnit::Px);
        parent.display = Display::Block;

        let child = ComputedValues::inherit_from(&parent);

        // Inherited properties
        assert_eq!(child.color, Color::rgb(255, 0, 0));
        assert_eq!(child.font_size.value(), 20.0);

        // Non-inherited properties use initial values
        assert_eq!(child.display, Display::Inline);
        assert_eq!(child.width.value(), 0.0);
    }

    #[test]
    fn test_rule_node_root() {
        let root = RuleNode::root();

        assert!(root.rule.is_none());
        assert!(root.parent.is_none());
        assert!(root.cached_values.is_none());
    }

    #[test]
    fn test_rule_node_new() {
        use css_cascade::{ApplicableRule, Origin, StyleRule};

        let root = Arc::new(RuleNode::root());
        let rule = ApplicableRule {
            rule: StyleRule {
                declarations: vec![],
            },
            specificity: Specificity::new(0, 1, 0),
            origin: Origin::Author,
            source_order: 0,
        };

        let node = RuleNode::new(rule, Some(root.clone()));

        assert!(node.rule.is_some());
        assert!(node.parent.is_some());
        assert!(node.cached_values.is_none());
    }

    #[test]
    fn test_style_context_new() {
        let values = Arc::new(ComputedValues::default());
        let context = StyleContext::new(Some(values.clone()), 1920.0, 1080.0, 16.0);

        assert!(context.parent_values.is_some());
        assert_eq!(context.viewport_width, 1920.0);
        assert_eq!(context.viewport_height, 1080.0);
        assert_eq!(context.root_font_size, 16.0);
    }

    #[test]
    fn test_style_context_default() {
        let context = StyleContext::default();

        assert!(context.parent_values.is_none());
        assert_eq!(context.viewport_width, 1920.0);
        assert_eq!(context.viewport_height, 1080.0);
        assert_eq!(context.root_font_size, 16.0);
    }
}
