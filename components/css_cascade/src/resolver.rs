use crate::types::{
    ApplicableRule, CascadeResult, ComputedValues, PropertyId, PropertyValue, Selector,
};
use css_types::Specificity;

/// CSS cascade resolver
pub struct CascadeResolver;

impl CascadeResolver {
    /// Create a new cascade resolver
    pub fn new() -> Self {
        Self
    }

    /// Resolve CSS cascade for a set of rules
    ///
    /// Algorithm:
    /// 1. Sort rules by origin (UserAgent < User < Author)
    /// 2. Within same origin, sort by specificity
    /// 3. Within same specificity, sort by source order
    /// 4. Apply !important rules with reversed origin priority
    /// 5. Merge all properties, later rules override earlier ones for same property
    pub fn resolve(&self, rules: &[ApplicableRule]) -> CascadeResult {
        let mut result = CascadeResult::new();

        if rules.is_empty() {
            return result;
        }

        // Separate normal and important declarations
        let mut normal_rules: Vec<_> = Vec::new();
        let mut important_rules: Vec<_> = Vec::new();

        for applicable_rule in rules {
            let mut normal_decls = Vec::new();
            let mut important_decls = Vec::new();

            for (prop_id, prop_value) in &applicable_rule.rule.declarations {
                match prop_value {
                    PropertyValue::Important(inner_value) => {
                        important_decls.push((*prop_id, (**inner_value).clone()));
                    }
                    _ => {
                        normal_decls.push((*prop_id, prop_value.clone()));
                    }
                }
            }

            if !normal_decls.is_empty() {
                normal_rules.push((
                    applicable_rule.origin,
                    applicable_rule.specificity,
                    applicable_rule.source_order,
                    normal_decls,
                ));
            }

            if !important_decls.is_empty() {
                important_rules.push((
                    applicable_rule.origin,
                    applicable_rule.specificity,
                    applicable_rule.source_order,
                    important_decls,
                ));
            }
        }

        // Sort normal rules by cascade order
        normal_rules.sort_by(|a, b| {
            // Compare origin first
            match a.0.cmp(&b.0) {
                std::cmp::Ordering::Equal => {
                    // Then specificity
                    match a.1.cmp(&b.1) {
                        std::cmp::Ordering::Equal => {
                            // Finally source order
                            a.2.cmp(&b.2)
                        }
                        other => other,
                    }
                }
                other => other,
            }
        });

        // Sort important rules (reversed origin priority)
        important_rules.sort_by(|a, b| {
            // Compare origin first (REVERSED for !important)
            match b.0.cmp(&a.0) {
                std::cmp::Ordering::Equal => {
                    // Then specificity
                    match a.1.cmp(&b.1) {
                        std::cmp::Ordering::Equal => {
                            // Finally source order
                            a.2.cmp(&b.2)
                        }
                        other => other,
                    }
                }
                other => other,
            }
        });

        // Apply normal rules (in order, so later rules override earlier ones)
        for (_origin, _specificity, _source_order, declarations) in normal_rules {
            for (prop_id, prop_value) in declarations {
                result.properties.insert(prop_id, prop_value);
            }
        }

        // Apply important rules (these override normal rules)
        for (_origin, _specificity, _source_order, declarations) in important_rules {
            for (prop_id, prop_value) in declarations {
                result
                    .properties
                    .insert(prop_id, PropertyValue::Important(Box::new(prop_value)));
            }
        }

        result
    }

    /// Calculate selector specificity
    ///
    /// Specificity is calculated as (a, b, c):
    /// - a: number of ID selectors
    /// - b: number of class selectors, attribute selectors, and pseudo-classes
    /// - c: number of type selectors and pseudo-elements
    ///
    /// Universal selector (*) has specificity (0, 0, 0)
    pub fn compute_specificity(selector: &Selector) -> Specificity {
        match selector {
            Selector::Universal => Specificity::zero(),

            Selector::Type(_) => Specificity::new(0, 0, 1),

            Selector::Class(_) => Specificity::new(0, 1, 0),

            Selector::Id(_) => Specificity::new(1, 0, 0),

            Selector::Attribute { .. } => Specificity::new(0, 1, 0),

            Selector::PseudoClass(_) => Specificity::new(0, 1, 0),

            Selector::PseudoElement(_) => Specificity::new(0, 0, 1),

            Selector::Compound(selectors) => {
                // Sum up all the specificities
                selectors.iter().map(Self::compute_specificity).fold(
                    Specificity::zero(),
                    |acc, spec| {
                        Specificity::new(
                            acc.id_selectors() + spec.id_selectors(),
                            acc.class_selectors() + spec.class_selectors(),
                            acc.type_selectors() + spec.type_selectors(),
                        )
                    },
                )
            }

            Selector::Descendant(left, right)
            | Selector::Child(left, right)
            | Selector::AdjacentSibling(left, right) => {
                // Add specificities of both parts
                let left_spec = Self::compute_specificity(left);
                let right_spec = Self::compute_specificity(right);
                Specificity::new(
                    left_spec.id_selectors() + right_spec.id_selectors(),
                    left_spec.class_selectors() + right_spec.class_selectors(),
                    left_spec.type_selectors() + right_spec.type_selectors(),
                )
            }
        }
    }

    /// Apply property inheritance from parent to child
    ///
    /// Inherited properties (if not explicitly set on child):
    /// - color
    /// - font-family
    /// - font-size
    /// - line-height
    /// - text-align
    ///
    /// Non-inherited properties:
    /// - margin
    /// - padding
    /// - border
    /// - width
    /// - height
    /// - display
    ///
    /// Special case: PropertyValue::Inherit forces inheritance for any property
    pub fn apply_inheritance(parent: &ComputedValues, child: &mut ComputedValues) {
        // List of inherited properties
        let inherited_properties = [
            PropertyId::Color,
            PropertyId::FontFamily,
            PropertyId::FontSize,
            PropertyId::LineHeight,
            PropertyId::TextAlign,
        ];

        // Apply inherited properties
        for &prop_id in &inherited_properties {
            // Only inherit if child doesn't already have this property set
            if !child.contains_key(&prop_id) {
                if let Some(parent_value) = parent.get(&prop_id) {
                    child.set(prop_id, parent_value.clone());
                }
            }
        }

        // Handle explicit 'inherit' keyword for any property
        let all_properties = [
            PropertyId::Color,
            PropertyId::FontFamily,
            PropertyId::FontSize,
            PropertyId::LineHeight,
            PropertyId::TextAlign,
            PropertyId::Margin,
            PropertyId::Padding,
            PropertyId::Border,
            PropertyId::Width,
            PropertyId::Height,
            PropertyId::Display,
        ];

        for &prop_id in &all_properties {
            if let Some(PropertyValue::Inherit) = child.get(&prop_id) {
                // Replace 'inherit' keyword with parent's value
                if let Some(parent_value) = parent.get(&prop_id) {
                    child.set(prop_id, parent_value.clone());
                } else {
                    // If parent doesn't have it, use initial value
                    // For now, just remove the inherit keyword
                    // (In a full implementation, we'd set initial values)
                }
            }
        }
    }
}

impl Default for CascadeResolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolver_creation() {
        let resolver = CascadeResolver::new();
        let empty_result = resolver.resolve(&[]);
        assert!(empty_result.properties.is_empty());
    }

    #[test]
    fn test_specificity_calculation_type() {
        let selector = Selector::Type("div".to_string());
        let spec = CascadeResolver::compute_specificity(&selector);
        assert_eq!(spec, Specificity::new(0, 0, 1));
    }

    #[test]
    fn test_specificity_calculation_class() {
        let selector = Selector::Class("button".to_string());
        let spec = CascadeResolver::compute_specificity(&selector);
        assert_eq!(spec, Specificity::new(0, 1, 0));
    }

    #[test]
    fn test_specificity_calculation_id() {
        let selector = Selector::Id("header".to_string());
        let spec = CascadeResolver::compute_specificity(&selector);
        assert_eq!(spec, Specificity::new(1, 0, 0));
    }
}
