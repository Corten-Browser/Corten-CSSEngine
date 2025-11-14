//! Media query evaluation implementation

use crate::*;
use css_types::LengthUnit;

/// Trait for media query evaluation
pub trait MediaQueryEvaluator {
    /// Evaluate a media query list (returns true if ANY query matches)
    fn evaluate_list(&self, query_list: &MediaQueryList, viewport: &ViewportInfo) -> bool;

    /// Check if a single media query matches
    fn matches(&self, query: &MediaQuery, viewport: &ViewportInfo) -> bool;
}

/// Evaluate a complete media query against a viewport
pub fn evaluate_media_query(query: &MediaQuery, viewport: &ViewportInfo) -> bool {
    // Check media type
    let type_matches = if let Some(media_type) = &query.media_type {
        match_media_type(media_type, viewport)
    } else {
        true // No media type specified = matches all
    };

    // Check condition
    let condition_matches = if let Some(condition) = &query.condition {
        evaluate_condition(condition, viewport)
    } else {
        true // No condition = always matches
    };

    // Combine results with negation
    let result = type_matches && condition_matches;
    if query.negated {
        !result
    } else {
        result
    }
}

/// Evaluate a media condition (AND/OR/NOT/Feature)
fn evaluate_condition(condition: &MediaCondition, viewport: &ViewportInfo) -> bool {
    match condition {
        MediaCondition::Feature { feature, range } => {
            evaluate_media_feature(feature, range, viewport)
        }
        MediaCondition::And { left, right } => {
            evaluate_condition(left, viewport) && evaluate_condition(right, viewport)
        }
        MediaCondition::Or { left, right } => {
            evaluate_condition(left, viewport) || evaluate_condition(right, viewport)
        }
        MediaCondition::Not { condition } => !evaluate_condition(condition, viewport),
    }
}

/// Evaluate a single media feature against a viewport
pub fn evaluate_media_feature(
    feature: &MediaFeature,
    range: &RangeType,
    viewport: &ViewportInfo,
) -> bool {
    match feature {
        MediaFeature::Width(length_opt) => {
            if let Some(length) = length_opt {
                let target_px = length_to_px(length, viewport);
                compare_value(viewport.width as f32, target_px, range)
            } else {
                // Boolean feature - true if has width
                viewport.width > 0
            }
        }
        MediaFeature::Height(length_opt) => {
            if let Some(length) = length_opt {
                let target_px = length_to_px(length, viewport);
                compare_value(viewport.height as f32, target_px, range)
            } else {
                // Boolean feature - true if has height
                viewport.height > 0
            }
        }
        MediaFeature::Orientation(target_orientation) => {
            viewport.orientation == *target_orientation
        }
        MediaFeature::AspectRatio {
            numerator,
            denominator,
        } => {
            let viewport_ratio = viewport.width as f32 / viewport.height as f32;
            let target_ratio = *numerator as f32 / *denominator as f32;
            // Allow small floating point error
            (viewport_ratio - target_ratio).abs() < 0.01
        }
        MediaFeature::Resolution(resolution) => {
            let target_dpi = resolution.to_dpi();
            compare_value(viewport.resolution_dpi, target_dpi, range)
        }
        MediaFeature::Color(bits_opt) => {
            if let Some(bits) = bits_opt {
                compare_value(viewport.color_bits as f32, *bits as f32, range)
            } else {
                // Boolean feature - true if has color
                viewport.color_bits > 0
            }
        }
        MediaFeature::ColorIndex(index_opt) => {
            if let Some(_index) = index_opt {
                // Color index support not implemented in viewport
                false
            } else {
                // Boolean feature
                false
            }
        }
        MediaFeature::Monochrome(bits_opt) => {
            if let Some(bits) = bits_opt {
                compare_value(viewport.monochrome_bits as f32, *bits as f32, range)
            } else {
                // Boolean feature - true if monochrome
                viewport.monochrome_bits > 0
            }
        }
        MediaFeature::Grid(grid) => {
            // Assume all viewports are not grid devices
            !*grid
        }
        MediaFeature::Scan(_scan) => {
            // Scan type not implemented in viewport
            true // Default to true for progressive
        }
        MediaFeature::Update(_update) => {
            // Update frequency not implemented in viewport
            true // Default to true for fast
        }
        MediaFeature::Hover(hover) => {
            // Determine hover capability based on device
            let has_hover = viewport.width >= 1024; // Desktop-like devices have hover
            match hover {
                HoverCapability::None => !has_hover,
                HoverCapability::Hover => has_hover,
            }
        }
        MediaFeature::Pointer(pointer) => {
            // Determine pointer type based on device
            if viewport.width < 768 {
                // Mobile devices have coarse pointer (touch)
                *pointer == PointerCapability::Coarse
            } else {
                // Desktop devices have fine pointer (mouse)
                *pointer == PointerCapability::Fine
            }
        }
        MediaFeature::PrefersColorScheme(scheme) => {
            // Default to light color scheme
            // In a real implementation, this would check system preferences
            *scheme == ColorScheme::Light
        }
        MediaFeature::PrefersReducedMotion(motion) => {
            // Default to no preference
            // In a real implementation, this would check system preferences
            *motion == ReducedMotion::NoPreference
        }
        MediaFeature::PrefersContrast(contrast) => {
            // Default to no preference
            // In a real implementation, this would check system preferences
            *contrast == Contrast::NoPreference
        }
    }
}

/// Check if a media type matches the current viewport
pub fn match_media_type(media_type: &MediaType, _viewport: &ViewportInfo) -> bool {
    match media_type {
        MediaType::All => true,
        MediaType::Screen => true,  // Assume screen device
        MediaType::Print => false,  // Not in print mode
        MediaType::Speech => false, // Not a speech synthesizer
    }
}

/// Compare a value with a target based on range type
fn compare_value(value: f32, target: f32, range: &RangeType) -> bool {
    match range {
        RangeType::Exact => (value - target).abs() < 0.1, // Allow small floating point error
        RangeType::Min => value >= target,
        RangeType::Max => value <= target,
    }
}

/// Convert a length to pixels based on viewport
fn length_to_px(length: &Length, viewport: &ViewportInfo) -> f32 {
    match length.unit() {
        LengthUnit::Px => length.value(),
        LengthUnit::Em => length.value() * 16.0, // Assume 16px base font size
        LengthUnit::Rem => length.value() * 16.0, // Assume 16px root font size
        LengthUnit::Percent => {
            // Percentage of viewport width for media queries
            (length.value() / 100.0) * viewport.width as f32
        }
        LengthUnit::Vw => (length.value() / 100.0) * viewport.width as f32,
        LengthUnit::Vh => (length.value() / 100.0) * viewport.height as f32,
    }
}

/// Default implementation of MediaQueryEvaluator
pub struct DefaultEvaluator;

impl MediaQueryEvaluator for DefaultEvaluator {
    fn evaluate_list(&self, query_list: &MediaQueryList, viewport: &ViewportInfo) -> bool {
        // Returns true if ANY query in the list matches
        query_list
            .queries
            .iter()
            .any(|query| self.matches(query, viewport))
    }

    fn matches(&self, query: &MediaQuery, viewport: &ViewportInfo) -> bool {
        evaluate_media_query(query, viewport)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use css_types::Length;

    #[test]
    fn test_evaluate_width_min() {
        let viewport = ViewportInfo::new(1920, 1080);
        let feature = MediaFeature::Width(Some(Length::new(768.0, LengthUnit::Px)));
        let result = evaluate_media_feature(&feature, &RangeType::Min, &viewport);
        assert!(result);
    }

    #[test]
    fn test_evaluate_orientation() {
        let viewport = ViewportInfo::new(1920, 1080); // Landscape
        let feature = MediaFeature::Orientation(Orientation::Landscape);
        let result = evaluate_media_feature(&feature, &RangeType::Exact, &viewport);
        assert!(result);
    }

    #[test]
    fn test_match_media_type_screen() {
        let viewport = ViewportInfo::desktop();
        assert!(match_media_type(&MediaType::Screen, &viewport));
    }

    #[test]
    fn test_compare_value_exact() {
        assert!(compare_value(768.0, 768.0, &RangeType::Exact));
        assert!(!compare_value(800.0, 768.0, &RangeType::Exact));
    }

    #[test]
    fn test_compare_value_min() {
        assert!(compare_value(1920.0, 768.0, &RangeType::Min));
        assert!(!compare_value(400.0, 768.0, &RangeType::Min));
    }

    #[test]
    fn test_compare_value_max() {
        assert!(compare_value(400.0, 768.0, &RangeType::Max));
        assert!(!compare_value(1920.0, 768.0, &RangeType::Max));
    }

    #[test]
    fn test_length_to_px() {
        let viewport = ViewportInfo::new(1920, 1080);
        let length = Length::new(50.0, LengthUnit::Vw);
        let px = length_to_px(&length, &viewport);
        assert_eq!(px, 960.0);
    }
}
