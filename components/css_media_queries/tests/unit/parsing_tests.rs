//! Unit tests for media query parsing

use css_media_queries::*;
use css_types::LengthUnit;

#[test]
fn test_parse_media_type_screen() {
    let result = parse_media_query("screen");
    assert!(result.is_ok());
    let query = result.unwrap();
    assert_eq!(query.media_type, Some(MediaType::Screen));
    assert!(query.condition.is_none());
    assert!(!query.negated);
}

#[test]
fn test_parse_media_type_print() {
    let result = parse_media_query("print");
    assert!(result.is_ok());
    let query = result.unwrap();
    assert_eq!(query.media_type, Some(MediaType::Print));
}

#[test]
fn test_parse_media_type_all() {
    let result = parse_media_query("all");
    assert!(result.is_ok());
    let query = result.unwrap();
    assert_eq!(query.media_type, Some(MediaType::All));
}

#[test]
fn test_parse_media_type_speech() {
    let result = parse_media_query("speech");
    assert!(result.is_ok());
    let query = result.unwrap();
    assert_eq!(query.media_type, Some(MediaType::Speech));
}

#[test]
fn test_parse_min_width() {
    let result = parse_media_query("(min-width: 768px)");
    assert!(result.is_ok());
    let query = result.unwrap();
    assert!(query.media_type.is_none());
    assert!(query.condition.is_some());

    if let Some(MediaCondition::Feature { feature, range }) = query.condition {
        assert_eq!(range, RangeType::Min);
        if let MediaFeature::Width(Some(length)) = feature {
            assert_eq!(length.value(), 768.0);
            assert_eq!(length.unit(), LengthUnit::Px);
        } else {
            panic!("Expected Width feature");
        }
    } else {
        panic!("Expected Feature condition");
    }
}

#[test]
fn test_parse_max_width() {
    let result = parse_media_query("(max-width: 480px)");
    assert!(result.is_ok());
    let query = result.unwrap();

    if let Some(MediaCondition::Feature { feature, range }) = query.condition {
        assert_eq!(range, RangeType::Max);
        if let MediaFeature::Width(Some(length)) = feature {
            assert_eq!(length.value(), 480.0);
        } else {
            panic!("Expected Width feature");
        }
    }
}

#[test]
fn test_parse_exact_width() {
    let result = parse_media_query("(width: 1024px)");
    assert!(result.is_ok());
    let query = result.unwrap();

    if let Some(MediaCondition::Feature { feature, range }) = query.condition {
        assert_eq!(range, RangeType::Exact);
        if let MediaFeature::Width(Some(length)) = feature {
            assert_eq!(length.value(), 1024.0);
        } else {
            panic!("Expected Width feature");
        }
    }
}

#[test]
fn test_parse_min_height() {
    let result = parse_media_query("(min-height: 600px)");
    assert!(result.is_ok());
    let query = result.unwrap();

    if let Some(MediaCondition::Feature { feature, range }) = query.condition {
        assert_eq!(range, RangeType::Min);
        if let MediaFeature::Height(Some(_)) = feature {
            // Success
        } else {
            panic!("Expected Height feature");
        }
    }
}

#[test]
fn test_parse_orientation_portrait() {
    let result = parse_media_query("(orientation: portrait)");
    assert!(result.is_ok());
    let query = result.unwrap();

    if let Some(MediaCondition::Feature { feature, range }) = query.condition {
        assert_eq!(range, RangeType::Exact);
        if let MediaFeature::Orientation(Orientation::Portrait) = feature {
            // Success
        } else {
            panic!("Expected Portrait orientation");
        }
    }
}

#[test]
fn test_parse_orientation_landscape() {
    let result = parse_media_query("(orientation: landscape)");
    assert!(result.is_ok());
    let query = result.unwrap();

    if let Some(MediaCondition::Feature { feature, .. }) = query.condition {
        if let MediaFeature::Orientation(Orientation::Landscape) = feature {
            // Success
        } else {
            panic!("Expected Landscape orientation");
        }
    }
}

#[test]
fn test_parse_prefers_color_scheme_dark() {
    let result = parse_media_query("(prefers-color-scheme: dark)");
    assert!(result.is_ok());
    let query = result.unwrap();

    if let Some(MediaCondition::Feature { feature, .. }) = query.condition {
        if let MediaFeature::PrefersColorScheme(ColorScheme::Dark) = feature {
            // Success
        } else {
            panic!("Expected Dark color scheme");
        }
    }
}

#[test]
fn test_parse_prefers_reduced_motion() {
    let result = parse_media_query("(prefers-reduced-motion: reduce)");
    assert!(result.is_ok());
    let query = result.unwrap();

    if let Some(MediaCondition::Feature { feature, .. }) = query.condition {
        if let MediaFeature::PrefersReducedMotion(ReducedMotion::Reduce) = feature {
            // Success
        } else {
            panic!("Expected Reduce motion");
        }
    }
}

#[test]
fn test_parse_screen_and_min_width() {
    let result = parse_media_query("screen and (min-width: 768px)");
    assert!(result.is_ok());
    let query = result.unwrap();
    assert_eq!(query.media_type, Some(MediaType::Screen));
    assert!(query.condition.is_some());
}

#[test]
fn test_parse_and_operator() {
    let result = parse_media_query("(min-width: 768px) and (max-width: 1024px)");
    assert!(result.is_ok());
    let query = result.unwrap();

    if let Some(MediaCondition::And { .. }) = query.condition {
        // Success
    } else {
        panic!("Expected And condition");
    }
}

#[test]
fn test_parse_not_screen() {
    let result = parse_media_query("not screen");
    assert!(result.is_ok());
    let query = result.unwrap();
    assert!(query.negated || matches!(query.condition, Some(MediaCondition::Not { .. })));
}

#[test]
fn test_parse_media_query_list_two_queries() {
    let result = parse_media_query_list("screen, print");
    assert!(result.is_ok());
    let list = result.unwrap();
    assert_eq!(list.queries.len(), 2);
    assert_eq!(list.queries[0].media_type, Some(MediaType::Screen));
    assert_eq!(list.queries[1].media_type, Some(MediaType::Print));
}

#[test]
fn test_parse_media_query_list_with_features() {
    let result = parse_media_query_list("screen and (min-width: 768px), (max-width: 480px)");
    assert!(result.is_ok());
    let list = result.unwrap();
    assert_eq!(list.queries.len(), 2);
}

#[test]
fn test_parse_empty_string() {
    let result = parse_media_query("");
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_media_type() {
    let result = parse_media_query("invalid");
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_feature() {
    let result = parse_media_query("(invalid-feature: 100px)");
    assert!(result.is_err());
}

#[test]
fn test_parse_missing_parenthesis() {
    let result = parse_media_query("min-width: 768px");
    assert!(result.is_err());
}

#[test]
fn test_parse_resolution_dpi() {
    let result = parse_media_query("(min-resolution: 192dpi)");
    assert!(result.is_ok());
    let query = result.unwrap();

    if let Some(MediaCondition::Feature { feature, range }) = query.condition {
        assert_eq!(range, RangeType::Min);
        if let MediaFeature::Resolution(_) = feature {
            // Success
        } else {
            panic!("Expected Resolution feature");
        }
    }
}

#[test]
fn test_parse_resolution_dppx() {
    let result = parse_media_query("(min-resolution: 2dppx)");
    assert!(result.is_ok());
}

#[test]
fn test_parse_aspect_ratio() {
    let result = parse_media_query("(aspect-ratio: 16/9)");
    assert!(result.is_ok());
    let query = result.unwrap();

    if let Some(MediaCondition::Feature { feature, .. }) = query.condition {
        if let MediaFeature::AspectRatio {
            numerator,
            denominator,
        } = feature
        {
            assert_eq!(numerator, 16);
            assert_eq!(denominator, 9);
        } else {
            panic!("Expected AspectRatio feature");
        }
    }
}

#[test]
fn test_parse_color() {
    let result = parse_media_query("(color)");
    assert!(result.is_ok());
    let query = result.unwrap();

    if let Some(MediaCondition::Feature { feature, .. }) = query.condition {
        if let MediaFeature::Color(None) = feature {
            // Success - boolean feature
        } else {
            panic!("Expected Color feature");
        }
    }
}

#[test]
fn test_parse_color_with_value() {
    let result = parse_media_query("(min-color: 8)");
    assert!(result.is_ok());
    let query = result.unwrap();

    if let Some(MediaCondition::Feature { feature, range }) = query.condition {
        assert_eq!(range, RangeType::Min);
        if let MediaFeature::Color(Some(bits)) = feature {
            assert_eq!(bits, 8);
        } else {
            panic!("Expected Color feature with value");
        }
    }
}

#[test]
fn test_parse_hover() {
    let result = parse_media_query("(hover: hover)");
    assert!(result.is_ok());
    let query = result.unwrap();

    if let Some(MediaCondition::Feature { feature, .. }) = query.condition {
        if let MediaFeature::Hover(HoverCapability::Hover) = feature {
            // Success
        } else {
            panic!("Expected Hover feature");
        }
    }
}

#[test]
fn test_parse_pointer_fine() {
    let result = parse_media_query("(pointer: fine)");
    assert!(result.is_ok());
    let query = result.unwrap();

    if let Some(MediaCondition::Feature { feature, .. }) = query.condition {
        if let MediaFeature::Pointer(PointerCapability::Fine) = feature {
            // Success
        } else {
            panic!("Expected Pointer feature");
        }
    }
}
