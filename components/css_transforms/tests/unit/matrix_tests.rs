//! Unit tests for transform matrix computation

use css_transforms::*;
use css_types::Length;

#[test]
fn test_identity_matrix() {
    let matrix = TransformMatrix::identity();
    assert_eq!(matrix.matrix[0][0], 1.0);
    assert_eq!(matrix.matrix[1][1], 1.0);
    assert_eq!(matrix.matrix[2][2], 1.0);
    assert_eq!(matrix.matrix[3][3], 1.0);
    assert_eq!(matrix.matrix[0][1], 0.0);
}

#[test]
fn test_compute_translate_matrix() {
    let transform = Transform {
        functions: vec![TransformFunction::Translate {
            x: Length::new(10.0, css_types::LengthUnit::Px),
            y: Length::new(20.0, css_types::LengthUnit::Px),
        }],
    };

    let rect = Rect {
        x: 0.0,
        y: 0.0,
        width: 100.0,
        height: 100.0,
    };

    let matrix = compute_transform_matrix(&transform, &rect);

    // Translation values should be in [0][3] and [1][3]
    assert_eq!(matrix.matrix[0][3], 10.0);
    assert_eq!(matrix.matrix[1][3], 20.0);
}

#[test]
fn test_compute_scale_matrix() {
    let transform = Transform {
        functions: vec![TransformFunction::Scale { x: 2.0, y: 3.0 }],
    };

    let rect = Rect {
        x: 0.0,
        y: 0.0,
        width: 100.0,
        height: 100.0,
    };

    let matrix = compute_transform_matrix(&transform, &rect);

    // Scale values should be on diagonal
    assert_eq!(matrix.matrix[0][0], 2.0);
    assert_eq!(matrix.matrix[1][1], 3.0);
}

#[test]
fn test_compute_rotate_matrix() {
    let transform = Transform {
        functions: vec![TransformFunction::Rotate {
            angle: Angle::new(90.0, AngleUnit::Deg),
        }],
    };

    let rect = Rect {
        x: 0.0,
        y: 0.0,
        width: 100.0,
        height: 100.0,
    };

    let matrix = compute_transform_matrix(&transform, &rect);

    // 90 degree rotation should swap axes
    // cos(90°) ≈ 0, sin(90°) ≈ 1
    assert!((matrix.matrix[0][0] - 0.0).abs() < 0.0001);
    assert!((matrix.matrix[0][1] - -1.0).abs() < 0.0001);
    assert!((matrix.matrix[1][0] - 1.0).abs() < 0.0001);
    assert!((matrix.matrix[1][1] - 0.0).abs() < 0.0001);
}

#[test]
fn test_matrix_multiplication() {
    let m1 = TransformMatrix::identity();
    let m2 = TransformMatrix::identity();
    let result = m1.multiply(&m2);

    // Identity × Identity = Identity
    assert_eq!(result.matrix[0][0], 1.0);
    assert_eq!(result.matrix[1][1], 1.0);
    assert_eq!(result.matrix[2][2], 1.0);
    assert_eq!(result.matrix[3][3], 1.0);
}

#[test]
fn test_transform_composition() {
    let transform = Transform {
        functions: vec![
            TransformFunction::Scale { x: 2.0, y: 2.0 },
            TransformFunction::Rotate {
                angle: Angle::new(45.0, AngleUnit::Deg),
            },
        ],
    };

    let rect = Rect {
        x: 0.0,
        y: 0.0,
        width: 100.0,
        height: 100.0,
    };

    let matrix = compute_transform_matrix(&transform, &rect);

    // Should compose scale and rotation
    // Result matrix should not be identity
    assert_ne!(matrix.matrix[0][0], 1.0);
}

#[test]
fn test_apply_transform_origin() {
    let mut matrix = TransformMatrix::identity();
    let origin = TransformOrigin {
        x: Length::new(50.0, css_types::LengthUnit::Percent),
        y: Length::new(50.0, css_types::LengthUnit::Percent),
        z: Length::new(0.0, css_types::LengthUnit::Px),
    };
    let rect = Rect {
        x: 0.0,
        y: 0.0,
        width: 100.0,
        height: 100.0,
    };

    apply_transform_origin(&mut matrix, &origin, &rect);

    // Should still be valid after applying origin
    assert!(matrix.matrix[0][0].is_finite());
}
