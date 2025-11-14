//! Public types for the CSS Engine

use crate::error::ElementId;

/// Computed style tree produced by the CSS engine
#[derive(Debug, Clone)]
pub struct StyleTree {
    /// Root node of the style tree
    pub root: StyleNode,
}

/// Individual styled node in the tree
#[derive(Debug, Clone)]
pub struct StyleNode {
    /// Element ID this node corresponds to
    pub element_id: ElementId,
    /// Computed style for this element
    pub computed_style: ComputedStyle,
    /// Child style nodes
    pub children: Vec<StyleNode>,
}

/// Computed CSS properties for an element
#[derive(Debug, Clone, Default)]
pub struct ComputedStyle {
    /// Display property
    pub display: Display,
    /// Color property
    pub color: Color,
    /// Background color
    pub background_color: Color,
    /// Width
    pub width: Length,
    /// Height
    pub height: Length,
    // Additional properties will be added as implementation progresses
}

/// Display property values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Display {
    #[default]
    Inline,
    Block,
    InlineBlock,
    Flex,
    Grid,
    None,
}

/// Color representation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    /// Create a new color with full opacity
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b, a: 255 }
    }

    /// Create a new color with alpha channel
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }

    /// Black color
    pub fn black() -> Self {
        Color::rgb(0, 0, 0)
    }

    /// White color
    pub fn white() -> Self {
        Color::rgb(255, 255, 255)
    }

    /// Transparent color
    pub fn transparent() -> Self {
        Color::rgba(0, 0, 0, 0)
    }
}

/// Length values
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Length {
    /// Pixels
    Px(f32),
    /// Percentage
    Percent(f32),
    /// Em units
    Em(f32),
    /// Rem units
    Rem(f32),
    /// Auto
    #[default]
    Auto,
}

/// Style invalidation types for incremental recomputation
#[derive(Debug, Clone)]
pub enum StyleInvalidation {
    /// Element attribute changed
    AttributeChange { element_id: ElementId, attr: String },
    /// Element class list changed
    ClassChange {
        element_id: ElementId,
        added: Vec<String>,
        removed: Vec<String>,
    },
    /// Element was inserted
    ElementInserted {
        element_id: ElementId,
        parent_id: ElementId,
    },
    /// Element was removed
    ElementRemoved { element_id: ElementId },
}

/// Simple DOM node representation for testing
#[derive(Debug, Clone)]
pub struct DomNode {
    /// Element ID
    pub id: ElementId,
    /// Tag name
    pub tag_name: String,
    /// Classes
    pub classes: Vec<String>,
    /// Attributes
    pub attributes: Vec<(String, String)>,
    /// Child nodes
    pub children: Vec<DomNode>,
}

impl DomNode {
    /// Create a new DOM node
    pub fn new(id: ElementId, tag_name: impl Into<String>) -> Self {
        DomNode {
            id,
            tag_name: tag_name.into(),
            classes: Vec::new(),
            attributes: Vec::new(),
            children: Vec::new(),
        }
    }

    /// Add a class to this node
    pub fn with_class(mut self, class: impl Into<String>) -> Self {
        self.classes.push(class.into());
        self
    }

    /// Add an attribute to this node
    pub fn with_attribute(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.push((name.into(), value.into()));
        self
    }

    /// Add a child node
    pub fn with_child(mut self, child: DomNode) -> Self {
        self.children.push(child);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_creation() {
        let color = Color::rgb(255, 128, 64);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 64);
        assert_eq!(color.a, 255);
    }

    #[test]
    fn test_color_rgba() {
        let color = Color::rgba(255, 128, 64, 128);
        assert_eq!(color.a, 128);
    }

    #[test]
    fn test_length_px() {
        let length = Length::Px(16.0);
        assert!(matches!(length, Length::Px(16.0)));
    }

    #[test]
    fn test_dom_node_builder() {
        let node = DomNode::new(ElementId::new(1), "div")
            .with_class("container")
            .with_attribute("id", "main");

        assert_eq!(node.tag_name, "div");
        assert_eq!(node.classes.len(), 1);
        assert_eq!(node.attributes.len(), 1);
    }
}
