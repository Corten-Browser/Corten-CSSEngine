//! Unit tests for transform-origin parsing

use css_transforms::*;

#[test]
fn test_parse_transform_origin_center() {
    let result = parse_transform_origin("center center");
    assert!(result.is_ok());
    let origin = result.unwrap();
    // Center is 50%
    assert_eq!(origin.x.value(), 50.0);
    assert_eq!(origin.y.value(), 50.0);
    assert_eq!(origin.z.value(), 0.0);
}

#[test]
fn test_parse_transform_origin_px() {
    let result = parse_transform_origin("10px 20px");
    assert!(result.is_ok());
    let origin = result.unwrap();
    assert_eq!(origin.x.value(), 10.0);
    assert_eq!(origin.y.value(), 20.0);
    assert_eq!(origin.z.value(), 0.0);
}

#[test]
fn test_parse_transform_origin_3d() {
    let result = parse_transform_origin("10px 20px 30px");
    assert!(result.is_ok());
    let origin = result.unwrap();
    assert_eq!(origin.x.value(), 10.0);
    assert_eq!(origin.y.value(), 20.0);
    assert_eq!(origin.z.value(), 30.0);
}

#[test]
fn test_parse_transform_origin_top_left() {
    let result = parse_transform_origin("left top");
    assert!(result.is_ok());
    let origin = result.unwrap();
    assert_eq!(origin.x.value(), 0.0);
    assert_eq!(origin.y.value(), 0.0);
}

#[test]
fn test_parse_transform_origin_bottom_right() {
    let result = parse_transform_origin("right bottom");
    assert!(result.is_ok());
    let origin = result.unwrap();
    assert_eq!(origin.x.value(), 100.0);
    assert_eq!(origin.y.value(), 100.0);
}

#[test]
fn test_parse_transform_origin_percentage() {
    let result = parse_transform_origin("25% 75%");
    assert!(result.is_ok());
    let origin = result.unwrap();
    assert_eq!(origin.x.value(), 25.0);
    assert_eq!(origin.y.value(), 75.0);
}

#[test]
fn test_parse_transform_origin_single_value() {
    let result = parse_transform_origin("center");
    assert!(result.is_ok());
    let origin = result.unwrap();
    assert_eq!(origin.x.value(), 50.0);
    assert_eq!(origin.y.value(), 50.0);
}
