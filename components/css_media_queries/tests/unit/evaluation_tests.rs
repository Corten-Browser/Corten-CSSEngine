//! Unit tests for media query evaluation

use css_media_queries::*;
use css_types::{Length, LengthUnit};

#[test]
fn test_match_media_type_screen() {
    let viewport = ViewportInfo::desktop();
    assert!(match_media_type(&MediaType::Screen, &viewport));
    assert!(match_media_type(&MediaType::All, &viewport));
}

#[test]
fn test_match_media_type_print() {
    let viewport = ViewportInfo::desktop();
    assert!(!match_media_type(&MediaType::Print, &viewport));
}

#[test]
fn test_evaluate_width_min_true() {
    let viewport = ViewportInfo::new(1920, 1080);
    let feature = MediaFeature::Width(Some(Length::new(768.0, LengthUnit::Px)));
    let result = evaluate_media_feature(&feature, &RangeType::Min, &viewport);
    assert!(result);
}

#[test]
fn test_evaluate_width_min_false() {
    let viewport = ViewportInfo::new(400, 600);
    let feature = MediaFeature::Width(Some(Length::new(768.0, LengthUnit::Px)));
    let result = evaluate_media_feature(&feature, &RangeType::Min, &viewport);
    assert!(!result);
}

#[test]
fn test_evaluate_width_max_true() {
    let viewport = ViewportInfo::new(400, 600);
    let feature = MediaFeature::Width(Some(Length::new(768.0, LengthUnit::Px)));
    let result = evaluate_media_feature(&feature, &RangeType::Max, &viewport);
    assert!(result);
}

#[test]
fn test_evaluate_width_max_false() {
    let viewport = ViewportInfo::new(1920, 1080);
    let feature = MediaFeature::Width(Some(Length::new(768.0, LengthUnit::Px)));
    let result = evaluate_media_feature(&feature, &RangeType::Max, &viewport);
    assert!(!result);
}

#[test]
fn test_evaluate_width_exact_true() {
    let viewport = ViewportInfo::new(768, 1024);
    let feature = MediaFeature::Width(Some(Length::new(768.0, LengthUnit::Px)));
    let result = evaluate_media_feature(&feature, &RangeType::Exact, &viewport);
    assert!(result);
}

#[test]
fn test_evaluate_width_exact_false() {
    let viewport = ViewportInfo::new(800, 1024);
    let feature = MediaFeature::Width(Some(Length::new(768.0, LengthUnit::Px)));
    let result = evaluate_media_feature(&feature, &RangeType::Exact, &viewport);
    assert!(!result);
}

#[test]
fn test_evaluate_height_min_true() {
    let viewport = ViewportInfo::new(1920, 1080);
    let feature = MediaFeature::Height(Some(Length::new(600.0, LengthUnit::Px)));
    let result = evaluate_media_feature(&feature, &RangeType::Min, &viewport);
    assert!(result);
}

#[test]
fn test_evaluate_height_max_true() {
    let viewport = ViewportInfo::new(1920, 1080);
    let feature = MediaFeature::Height(Some(Length::new(2000.0, LengthUnit::Px)));
    let result = evaluate_media_feature(&feature, &RangeType::Max, &viewport);
    assert!(result);
}

#[test]
fn test_evaluate_orientation_portrait_true() {
    let viewport = ViewportInfo::new(768, 1024);
    let feature = MediaFeature::Orientation(Orientation::Portrait);
    let result = evaluate_media_feature(&feature, &RangeType::Exact, &viewport);
    assert!(result);
}

#[test]
fn test_evaluate_orientation_portrait_false() {
    let viewport = ViewportInfo::new(1920, 1080);
    let feature = MediaFeature::Orientation(Orientation::Portrait);
    let result = evaluate_media_feature(&feature, &RangeType::Exact, &viewport);
    assert!(!result);
}

#[test]
fn test_evaluate_orientation_landscape_true() {
    let viewport = ViewportInfo::new(1920, 1080);
    let feature = MediaFeature::Orientation(Orientation::Landscape);
    let result = evaluate_media_feature(&feature, &RangeType::Exact, &viewport);
    assert!(result);
}

#[test]
fn test_evaluate_resolution_min_true() {
    let viewport = ViewportInfo {
        width: 1920,
        height: 1080,
        device_width: 1920,
        device_height: 1080,
        device_pixel_ratio: 2.0,
        orientation: Orientation::Landscape,
        color_bits: 24,
        monochrome_bits: 0,
        resolution_dpi: 192.0,
    };

    let feature = MediaFeature::Resolution(Resolution::new(96.0, ResolutionUnit::Dpi));
    let result = evaluate_media_feature(&feature, &RangeType::Min, &viewport);
    assert!(result);
}

#[test]
fn test_evaluate_color_scheme_dark() {
    // Note: This would require user preference in viewport
    // For now, test with default (assumes light)
    let viewport = ViewportInfo::desktop();
    let feature = MediaFeature::PrefersColorScheme(ColorScheme::Light);
    let result = evaluate_media_feature(&feature, &RangeType::Exact, &viewport);
    // Default should be light
    assert!(result);
}

#[test]
fn test_evaluate_aspect_ratio() {
    let viewport = ViewportInfo::new(1920, 1080); // 16:9
    let feature = MediaFeature::AspectRatio {
        numerator: 16,
        denominator: 9,
    };
    let result = evaluate_media_feature(&feature, &RangeType::Exact, &viewport);
    assert!(result);
}

#[test]
fn test_evaluate_color_feature_true() {
    let viewport = ViewportInfo::desktop();
    let feature = MediaFeature::Color(None); // Boolean - has color
    let result = evaluate_media_feature(&feature, &RangeType::Exact, &viewport);
    assert!(result);
}

#[test]
fn test_evaluate_color_bits_min_true() {
    let viewport = ViewportInfo::desktop(); // 24 bits
    let feature = MediaFeature::Color(Some(8));
    let result = evaluate_media_feature(&feature, &RangeType::Min, &viewport);
    assert!(result);
}

#[test]
fn test_evaluate_media_query_simple_screen() {
    let viewport = ViewportInfo::desktop();
    let query = MediaQuery::media_type(MediaType::Screen);
    let result = evaluate_media_query(&query, &viewport);
    assert!(result);
}

#[test]
fn test_evaluate_media_query_with_condition() {
    let viewport = ViewportInfo::new(1920, 1080);
    let condition = MediaCondition::Feature {
        feature: MediaFeature::Width(Some(Length::new(768.0, LengthUnit::Px))),
        range: RangeType::Min,
    };
    let query = MediaQuery::new(Some(MediaType::Screen), Some(condition), false);
    let result = evaluate_media_query(&query, &viewport);
    assert!(result);
}

#[test]
fn test_evaluate_media_query_negated() {
    let viewport = ViewportInfo::desktop();
    let query = MediaQuery::new(Some(MediaType::Print), None, true);
    let result = evaluate_media_query(&query, &viewport);
    assert!(result); // NOT print = true for screen
}

#[test]
fn test_evaluate_and_condition_true() {
    let viewport = ViewportInfo::new(1920, 1080);
    let left = MediaCondition::Feature {
        feature: MediaFeature::Width(Some(Length::new(768.0, LengthUnit::Px))),
        range: RangeType::Min,
    };
    let right = MediaCondition::Feature {
        feature: MediaFeature::Height(Some(Length::new(600.0, LengthUnit::Px))),
        range: RangeType::Min,
    };
    let condition = MediaCondition::And {
        left: Box::new(left),
        right: Box::new(right),
    };
    let query = MediaQuery::condition(condition);
    let result = evaluate_media_query(&query, &viewport);
    assert!(result);
}

#[test]
fn test_evaluate_and_condition_false() {
    let viewport = ViewportInfo::new(400, 600);
    let left = MediaCondition::Feature {
        feature: MediaFeature::Width(Some(Length::new(768.0, LengthUnit::Px))),
        range: RangeType::Min,
    };
    let right = MediaCondition::Feature {
        feature: MediaFeature::Height(Some(Length::new(600.0, LengthUnit::Px))),
        range: RangeType::Min,
    };
    let condition = MediaCondition::And {
        left: Box::new(left),
        right: Box::new(right),
    };
    let query = MediaQuery::condition(condition);
    let result = evaluate_media_query(&query, &viewport);
    assert!(!result); // Width fails
}

#[test]
fn test_evaluate_or_condition_true() {
    let viewport = ViewportInfo::new(400, 600);
    let left = MediaCondition::Feature {
        feature: MediaFeature::Width(Some(Length::new(768.0, LengthUnit::Px))),
        range: RangeType::Min,
    };
    let right = MediaCondition::Feature {
        feature: MediaFeature::Width(Some(Length::new(768.0, LengthUnit::Px))),
        range: RangeType::Max,
    };
    let condition = MediaCondition::Or {
        left: Box::new(left),
        right: Box::new(right),
    };
    let query = MediaQuery::condition(condition);
    let result = evaluate_media_query(&query, &viewport);
    assert!(result); // Right side is true
}

#[test]
fn test_evaluate_not_condition() {
    let viewport = ViewportInfo::new(400, 600);
    let inner = MediaCondition::Feature {
        feature: MediaFeature::Width(Some(Length::new(768.0, LengthUnit::Px))),
        range: RangeType::Min,
    };
    let condition = MediaCondition::Not {
        condition: Box::new(inner),
    };
    let query = MediaQuery::condition(condition);
    let result = evaluate_media_query(&query, &viewport);
    assert!(result); // NOT (400 >= 768) = NOT false = true
}

#[test]
fn test_media_query_evaluator_trait() {
    struct MyEvaluator;
    impl MediaQueryEvaluator for MyEvaluator {
        fn evaluate_list(&self, query_list: &MediaQueryList, viewport: &ViewportInfo) -> bool {
            query_list.queries.iter().any(|q| self.matches(q, viewport))
        }

        fn matches(&self, query: &MediaQuery, viewport: &ViewportInfo) -> bool {
            evaluate_media_query(query, viewport)
        }
    }

    let evaluator = MyEvaluator;
    let viewport = ViewportInfo::desktop();
    let list = MediaQueryList::new(vec![
        MediaQuery::media_type(MediaType::Print),
        MediaQuery::media_type(MediaType::Screen),
    ]);

    assert!(evaluator.evaluate_list(&list, &viewport));
}

#[test]
fn test_evaluate_responsive_mobile_breakpoint() {
    let viewport = ViewportInfo::mobile();
    let query = parse_media_query("(max-width: 767px)").unwrap();
    assert!(evaluate_media_query(&query, &viewport));
}

#[test]
fn test_evaluate_responsive_tablet_breakpoint() {
    let viewport = ViewportInfo::tablet();
    let query = parse_media_query("(min-width: 768px) and (max-width: 1023px)").unwrap();
    assert!(evaluate_media_query(&query, &viewport));
}

#[test]
fn test_evaluate_responsive_desktop_breakpoint() {
    let viewport = ViewportInfo::desktop();
    let query = parse_media_query("(min-width: 1024px)").unwrap();
    assert!(evaluate_media_query(&query, &viewport));
}

#[test]
fn test_evaluate_hover_none() {
    let viewport = ViewportInfo::mobile(); // Assume mobile has no hover
    let feature = MediaFeature::Hover(HoverCapability::None);
    let result = evaluate_media_feature(&feature, &RangeType::Exact, &viewport);
    assert!(result);
}

#[test]
fn test_evaluate_pointer_coarse() {
    let viewport = ViewportInfo::mobile(); // Touch device
    let feature = MediaFeature::Pointer(PointerCapability::Coarse);
    let result = evaluate_media_feature(&feature, &RangeType::Exact, &viewport);
    assert!(result);
}
