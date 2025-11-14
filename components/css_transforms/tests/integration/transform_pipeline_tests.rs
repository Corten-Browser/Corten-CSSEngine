//! Integration tests for complete transform pipeline

use css_transforms::*;

#[test]
fn test_complete_transform_pipeline() {
    // Parse transform
    let transform = parse_transform("translate(10px, 20px) rotate(45deg)").unwrap();

    // Parse origin
    let origin = parse_transform_origin("center center").unwrap();

    // Create reference box
    let rect = Rect {
        x: 0.0,
        y: 0.0,
        width: 100.0,
        height: 100.0,
    };

    // Compute matrix
    let mut matrix = compute_transform_matrix(&transform, &rect);

    // Apply origin
    apply_transform_origin(&mut matrix, &origin, &rect);

    // Matrix should be valid
    assert!(matrix.matrix[0][0].is_finite());
    assert!(matrix.matrix[1][1].is_finite());
}

#[test]
fn test_transform_computer_trait() {
    struct MyComputer;

    impl TransformComputer for MyComputer {
        fn compute_transform(
            &self,
            transform: &Transform,
            origin: &TransformOrigin,
            reference_box: &Rect,
        ) -> TransformMatrix {
            let mut matrix = compute_transform_matrix(transform, reference_box);
            apply_transform_origin(&mut matrix, origin, reference_box);
            matrix
        }
    }

    let computer = MyComputer;
    let transform = parse_transform("scale(2)").unwrap();
    let origin = parse_transform_origin("0px 0px").unwrap();
    let rect = Rect {
        x: 0.0,
        y: 0.0,
        width: 100.0,
        height: 100.0,
    };

    let matrix = computer.compute_transform(&transform, &origin, &rect);
    assert_eq!(matrix.matrix[0][0], 2.0);
}

#[test]
fn test_3d_transform_pipeline() {
    let transform = parse_transform("translate3d(10px, 20px, 30px) rotateX(45deg)").unwrap();
    let _origin = parse_transform_origin("center center 0px").unwrap();
    let rect = Rect {
        x: 0.0,
        y: 0.0,
        width: 100.0,
        height: 100.0,
    };

    let matrix = compute_transform_matrix(&transform, &rect);

    // 3D transform should modify z-axis
    assert!(matrix.matrix[2][3].abs() > 0.0 || matrix.matrix[2][2] != 1.0);
}

#[test]
fn test_complex_transform_composition() {
    let transform =
        parse_transform("perspective(500px) translate3d(10px, 20px, 30px) rotateY(45deg) scale(2)")
            .unwrap();

    let rect = Rect {
        x: 0.0,
        y: 0.0,
        width: 100.0,
        height: 100.0,
    };

    let matrix = compute_transform_matrix(&transform, &rect);

    // Complex transform should produce non-identity matrix
    assert_ne!(matrix.matrix[0][0], 1.0);
    assert!(matrix.matrix[0][0].is_finite());
}
