//! Unit tests for transform function parsing

use css_transforms::*;

#[test]
fn test_parse_translate_2d() {
    let result = parse_transform("translate(10px, 20px)");
    assert!(result.is_ok());
    let transform = result.unwrap();
    assert_eq!(transform.functions.len(), 1);

    match &transform.functions[0] {
        TransformFunction::Translate { x, y } => {
            assert_eq!(x.value(), 10.0);
            assert_eq!(y.value(), 20.0);
        }
        _ => panic!("Expected Translate variant"),
    }
}

#[test]
fn test_parse_translate_x() {
    let result = parse_transform("translateX(10px)");
    assert!(result.is_ok());
    let transform = result.unwrap();
    assert_eq!(transform.functions.len(), 1);

    match &transform.functions[0] {
        TransformFunction::TranslateX { value } => {
            assert_eq!(value.value(), 10.0);
        }
        _ => panic!("Expected TranslateX variant"),
    }
}

#[test]
fn test_parse_translate_y() {
    let result = parse_transform("translateY(20px)");
    assert!(result.is_ok());
    let transform = result.unwrap();
    assert_eq!(transform.functions.len(), 1);

    match &transform.functions[0] {
        TransformFunction::TranslateY { value } => {
            assert_eq!(value.value(), 20.0);
        }
        _ => panic!("Expected TranslateY variant"),
    }
}

#[test]
fn test_parse_rotate() {
    let result = parse_transform("rotate(45deg)");
    assert!(result.is_ok());
    let transform = result.unwrap();
    assert_eq!(transform.functions.len(), 1);

    match &transform.functions[0] {
        TransformFunction::Rotate { angle } => {
            assert_eq!(angle.value(), 45.0);
        }
        _ => panic!("Expected Rotate variant"),
    }
}

#[test]
fn test_parse_scale() {
    let result = parse_transform("scale(2)");
    assert!(result.is_ok());
    let transform = result.unwrap();
    assert_eq!(transform.functions.len(), 1);

    match &transform.functions[0] {
        TransformFunction::Scale { x, y } => {
            assert_eq!(*x, 2.0);
            assert_eq!(*y, 2.0);
        }
        _ => panic!("Expected Scale variant"),
    }
}

#[test]
fn test_parse_scale_xy() {
    let result = parse_transform("scale(2, 3)");
    assert!(result.is_ok());
    let transform = result.unwrap();
    assert_eq!(transform.functions.len(), 1);

    match &transform.functions[0] {
        TransformFunction::Scale { x, y } => {
            assert_eq!(*x, 2.0);
            assert_eq!(*y, 3.0);
        }
        _ => panic!("Expected Scale variant"),
    }
}

#[test]
fn test_parse_multiple_transforms() {
    let result = parse_transform("scale(2) rotate(45deg)");
    assert!(result.is_ok());
    let transform = result.unwrap();
    assert_eq!(transform.functions.len(), 2);
}

#[test]
fn test_parse_skew() {
    let result = parse_transform("skew(10deg, 20deg)");
    assert!(result.is_ok());
    let transform = result.unwrap();
    assert_eq!(transform.functions.len(), 1);

    match &transform.functions[0] {
        TransformFunction::Skew { x, y } => {
            assert_eq!(x.value(), 10.0);
            assert_eq!(y.value(), 20.0);
        }
        _ => panic!("Expected Skew variant"),
    }
}

#[test]
fn test_parse_matrix() {
    let result = parse_transform("matrix(1, 0, 0, 1, 10, 20)");
    assert!(result.is_ok());
    let transform = result.unwrap();
    assert_eq!(transform.functions.len(), 1);

    match &transform.functions[0] {
        TransformFunction::Matrix { a, b, c, d, tx, ty } => {
            assert_eq!(*a, 1.0);
            assert_eq!(*b, 0.0);
            assert_eq!(*c, 0.0);
            assert_eq!(*d, 1.0);
            assert_eq!(*tx, 10.0);
            assert_eq!(*ty, 20.0);
        }
        _ => panic!("Expected Matrix variant"),
    }
}

#[test]
fn test_parse_translate3d() {
    let result = parse_transform("translate3d(10px, 20px, 30px)");
    assert!(result.is_ok());
    let transform = result.unwrap();
    assert_eq!(transform.functions.len(), 1);

    match &transform.functions[0] {
        TransformFunction::Translate3d { x, y, z } => {
            assert_eq!(x.value(), 10.0);
            assert_eq!(y.value(), 20.0);
            assert_eq!(z.value(), 30.0);
        }
        _ => panic!("Expected Translate3d variant"),
    }
}

#[test]
fn test_parse_perspective() {
    let result = parse_transform("perspective(500px)");
    assert!(result.is_ok());
    let transform = result.unwrap();
    assert_eq!(transform.functions.len(), 1);

    match &transform.functions[0] {
        TransformFunction::Perspective { value } => {
            assert_eq!(value.value(), 500.0);
        }
        _ => panic!("Expected Perspective variant"),
    }
}

#[test]
fn test_parse_invalid_transform() {
    let result = parse_transform("invalid(10px)");
    assert!(result.is_err());
}

#[test]
fn test_parse_empty_transform() {
    let result = parse_transform("");
    assert!(result.is_err());
}
