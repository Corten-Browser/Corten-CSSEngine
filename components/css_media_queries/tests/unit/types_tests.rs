//! Unit tests for media query types

use css_media_queries::*;

#[test]
fn test_media_type_all() {
    let media_type = MediaType::All;
    assert_eq!(format!("{:?}", media_type), "All");
}

#[test]
fn test_media_type_screen() {
    let media_type = MediaType::Screen;
    assert_eq!(format!("{:?}", media_type), "Screen");
}

#[test]
fn test_media_type_print() {
    let media_type = MediaType::Print;
    assert_eq!(format!("{:?}", media_type), "Print");
}

#[test]
fn test_media_type_speech() {
    let media_type = MediaType::Speech;
    assert_eq!(format!("{:?}", media_type), "Speech");
}

#[test]
fn test_orientation_portrait() {
    let orientation = Orientation::Portrait;
    assert_eq!(format!("{:?}", orientation), "Portrait");
}

#[test]
fn test_orientation_landscape() {
    let orientation = Orientation::Landscape;
    assert_eq!(format!("{:?}", orientation), "Landscape");
}

#[test]
fn test_resolution_dpi() {
    let resolution = Resolution {
        value: 96.0,
        unit: ResolutionUnit::Dpi,
    };
    assert_eq!(resolution.value, 96.0);
    assert_eq!(resolution.unit, ResolutionUnit::Dpi);
}

#[test]
fn test_resolution_dpcm() {
    let resolution = Resolution {
        value: 38.0,
        unit: ResolutionUnit::Dpcm,
    };
    assert_eq!(resolution.value, 38.0);
    assert_eq!(resolution.unit, ResolutionUnit::Dpcm);
}

#[test]
fn test_resolution_dppx() {
    let resolution = Resolution {
        value: 2.0,
        unit: ResolutionUnit::Dppx,
    };
    assert_eq!(resolution.value, 2.0);
    assert_eq!(resolution.unit, ResolutionUnit::Dppx);
}

#[test]
fn test_range_type_exact() {
    let range = RangeType::Exact;
    assert_eq!(format!("{:?}", range), "Exact");
}

#[test]
fn test_range_type_min() {
    let range = RangeType::Min;
    assert_eq!(format!("{:?}", range), "Min");
}

#[test]
fn test_range_type_max() {
    let range = RangeType::Max;
    assert_eq!(format!("{:?}", range), "Max");
}

#[test]
fn test_color_scheme_light() {
    let scheme = ColorScheme::Light;
    assert_eq!(format!("{:?}", scheme), "Light");
}

#[test]
fn test_color_scheme_dark() {
    let scheme = ColorScheme::Dark;
    assert_eq!(format!("{:?}", scheme), "Dark");
}

#[test]
fn test_reduced_motion_no_preference() {
    let motion = ReducedMotion::NoPreference;
    assert_eq!(format!("{:?}", motion), "NoPreference");
}

#[test]
fn test_reduced_motion_reduce() {
    let motion = ReducedMotion::Reduce;
    assert_eq!(format!("{:?}", motion), "Reduce");
}

#[test]
fn test_contrast_no_preference() {
    let contrast = Contrast::NoPreference;
    assert_eq!(format!("{:?}", contrast), "NoPreference");
}

#[test]
fn test_contrast_more() {
    let contrast = Contrast::More;
    assert_eq!(format!("{:?}", contrast), "More");
}

#[test]
fn test_contrast_less() {
    let contrast = Contrast::Less;
    assert_eq!(format!("{:?}", contrast), "Less");
}

#[test]
fn test_viewport_info_creation() {
    let viewport = ViewportInfo {
        width: 1920,
        height: 1080,
        device_width: 1920,
        device_height: 1080,
        device_pixel_ratio: 1.0,
        orientation: Orientation::Landscape,
        color_bits: 24,
        monochrome_bits: 0,
        resolution_dpi: 96.0,
    };

    assert_eq!(viewport.width, 1920);
    assert_eq!(viewport.height, 1080);
    assert_eq!(viewport.orientation, Orientation::Landscape);
}

#[test]
fn test_media_query_simple() {
    let query = MediaQuery {
        media_type: Some(MediaType::Screen),
        condition: None,
        negated: false,
    };

    assert_eq!(query.media_type, Some(MediaType::Screen));
    assert!(query.condition.is_none());
    assert!(!query.negated);
}

#[test]
fn test_media_query_with_condition() {
    use css_types::{Length, LengthUnit};

    let condition = MediaCondition::Feature {
        feature: MediaFeature::Width(Some(Length::new(768.0, LengthUnit::Px))),
        range: RangeType::Min,
    };

    let query = MediaQuery {
        media_type: Some(MediaType::Screen),
        condition: Some(condition),
        negated: false,
    };

    assert_eq!(query.media_type, Some(MediaType::Screen));
    assert!(query.condition.is_some());
}

#[test]
fn test_media_query_list() {
    let query1 = MediaQuery {
        media_type: Some(MediaType::Screen),
        condition: None,
        negated: false,
    };

    let query2 = MediaQuery {
        media_type: Some(MediaType::Print),
        condition: None,
        negated: false,
    };

    let list = MediaQueryList {
        queries: vec![query1, query2],
    };

    assert_eq!(list.queries.len(), 2);
}
