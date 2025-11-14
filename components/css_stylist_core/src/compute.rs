//! Style computation functions
//!
//! This module provides functions for computing styles:
//! - Cascade resolution
//! - Inheritance
//! - Unit resolution

use crate::types::{ComputedValues, StyleContext};
use css_types::{Length, LengthUnit};

/// Resolve a length value to pixels
///
/// Converts relative length units (em, rem, %) to absolute pixel values
/// based on the style context.
///
/// # Arguments
/// * `length` - The length value to resolve
/// * `context` - Style context with parent values and viewport info
///
/// # Examples
/// ```
/// use css_stylist_core::compute::resolve_length;
/// use css_stylist_core::types::StyleContext;
/// use css_types::{Length, LengthUnit};
///
/// let context = StyleContext::default();
/// let length = Length::new(10.0, LengthUnit::Px);
/// let resolved = resolve_length(&length, &context);
/// assert_eq!(resolved, 10.0);
/// ```
pub fn resolve_length(length: &Length, context: &StyleContext) -> f32 {
    match length.unit() {
        LengthUnit::Px => length.value(),
        LengthUnit::Percent => {
            // For now, resolve percentage relative to viewport width
            // In real implementation, this depends on the property
            context.viewport_width * length.value() / 100.0
        }
        LengthUnit::Em => {
            // Resolve em relative to parent font size
            let parent_font_size = context
                .parent_values
                .as_ref()
                .map(|v| v.font_size.value())
                .unwrap_or(16.0);
            length.value() * parent_font_size
        }
        LengthUnit::Rem => {
            // Resolve rem relative to root font size
            length.value() * context.root_font_size
        }
        LengthUnit::Vw => {
            // Viewport width percentage
            context.viewport_width * length.value() / 100.0
        }
        LengthUnit::Vh => {
            // Viewport height percentage
            context.viewport_height * length.value() / 100.0
        }
    }
}

/// Apply inheritance to computed values
///
/// Inherits inherited properties from parent, uses initial values for
/// non-inherited properties.
///
/// # Arguments
/// * `parent` - Parent element's computed values
///
/// # Examples
/// ```
/// use css_stylist_core::compute::apply_inheritance;
/// use css_stylist_core::types::ComputedValues;
/// use css_types::Color;
///
/// let mut parent = ComputedValues::default();
/// parent.color = Color::rgb(255, 0, 0);
///
/// let child = apply_inheritance(&parent);
/// assert_eq!(child.color, Color::rgb(255, 0, 0));
/// ```
pub fn apply_inheritance(parent: &ComputedValues) -> ComputedValues {
    ComputedValues::inherit_from(parent)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::StyleContext;
    use css_types::{Color, LengthUnit};
    use servo_arc::Arc;

    #[test]
    fn test_resolve_length_px() {
        let context = StyleContext::default();
        let length = Length::new(10.0, LengthUnit::Px);

        let resolved = resolve_length(&length, &context);
        assert_eq!(resolved, 10.0);
    }

    #[test]
    fn test_resolve_length_percent() {
        let context = StyleContext::new(None, 1000.0, 800.0, 16.0);
        let length = Length::new(50.0, LengthUnit::Percent);

        let resolved = resolve_length(&length, &context);
        assert_eq!(resolved, 500.0); // 50% of 1000px viewport width
    }

    #[test]
    fn test_resolve_length_em() {
        let mut parent = ComputedValues::default();
        parent.font_size = Length::new(20.0, LengthUnit::Px);
        let context = StyleContext::new(Some(Arc::new(parent)), 1000.0, 800.0, 16.0);

        let length = Length::new(2.0, LengthUnit::Em);
        let resolved = resolve_length(&length, &context);
        assert_eq!(resolved, 40.0); // 2em * 20px
    }

    #[test]
    fn test_resolve_length_rem() {
        let context = StyleContext::new(None, 1000.0, 800.0, 16.0);
        let length = Length::new(1.5, LengthUnit::Rem);

        let resolved = resolve_length(&length, &context);
        assert_eq!(resolved, 24.0); // 1.5rem * 16px
    }

    #[test]
    fn test_resolve_length_vw() {
        let context = StyleContext::new(None, 1000.0, 800.0, 16.0);
        let length = Length::new(10.0, LengthUnit::Vw);

        let resolved = resolve_length(&length, &context);
        assert_eq!(resolved, 100.0); // 10vw of 1000px viewport
    }

    #[test]
    fn test_resolve_length_vh() {
        let context = StyleContext::new(None, 1000.0, 800.0, 16.0);
        let length = Length::new(10.0, LengthUnit::Vh);

        let resolved = resolve_length(&length, &context);
        assert_eq!(resolved, 80.0); // 10vh of 800px viewport
    }

    #[test]
    fn test_apply_inheritance() {
        let mut parent = ComputedValues::default();
        parent.color = Color::rgb(255, 100, 50);
        parent.font_size = Length::new(18.0, LengthUnit::Px);

        let child = apply_inheritance(&parent);

        // Inherited properties
        assert_eq!(child.color, Color::rgb(255, 100, 50));
        assert_eq!(child.font_size.value(), 18.0);

        // Non-inherited properties use initial values
        assert_eq!(child.margin_top.value(), 0.0);
    }
}
