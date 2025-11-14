//! Transform matrix computation

use crate::{Rect, Transform, TransformFunction, TransformMatrix, TransformOrigin};
use css_types::LengthUnit;

/// Compute 4x4 matrix from transform list
///
/// Computes a single 4x4 transformation matrix by composing all transform functions
/// in the given transform. The reference box is used to resolve percentage values.
///
/// # Examples
/// ```
/// use css_transforms::{parse_transform, compute_transform_matrix, Rect};
///
/// let transform = parse_transform("translate(10px, 20px)").unwrap();
/// let rect = Rect { x: 0.0, y: 0.0, width: 100.0, height: 100.0 };
/// let matrix = compute_transform_matrix(&transform, &rect);
/// assert_eq!(matrix.matrix[0][3], 10.0);
/// ```
pub fn compute_transform_matrix(transform: &Transform, reference_box: &Rect) -> TransformMatrix {
    let mut result = TransformMatrix::identity();

    // Apply each transform function in order (left to right composition)
    for func in &transform.functions {
        let func_matrix = compute_function_matrix(func, reference_box);
        result = result.multiply(&func_matrix);
    }

    result
}

fn compute_function_matrix(func: &TransformFunction, reference_box: &Rect) -> TransformMatrix {
    match func {
        TransformFunction::Translate { x, y } => {
            let tx = resolve_length(x, reference_box.width);
            let ty = resolve_length(y, reference_box.height);
            translation_matrix(tx, ty, 0.0)
        }
        TransformFunction::TranslateX { value } => {
            let tx = resolve_length(value, reference_box.width);
            translation_matrix(tx, 0.0, 0.0)
        }
        TransformFunction::TranslateY { value } => {
            let ty = resolve_length(value, reference_box.height);
            translation_matrix(0.0, ty, 0.0)
        }
        TransformFunction::TranslateZ { value } => {
            let tz = resolve_length(value, 0.0);
            translation_matrix(0.0, 0.0, tz)
        }
        TransformFunction::Translate3d { x, y, z } => {
            let tx = resolve_length(x, reference_box.width);
            let ty = resolve_length(y, reference_box.height);
            let tz = resolve_length(z, 0.0);
            translation_matrix(tx, ty, tz)
        }
        TransformFunction::Scale { x, y } => scale_matrix(*x, *y, 1.0),
        TransformFunction::ScaleX { value } => scale_matrix(*value, 1.0, 1.0),
        TransformFunction::ScaleY { value } => scale_matrix(1.0, *value, 1.0),
        TransformFunction::ScaleZ { value } => scale_matrix(1.0, 1.0, *value),
        TransformFunction::Scale3d { x, y, z } => scale_matrix(*x, *y, *z),
        TransformFunction::Rotate { angle } => rotation_z_matrix(angle.to_radians()),
        TransformFunction::RotateX { angle } => rotation_x_matrix(angle.to_radians()),
        TransformFunction::RotateY { angle } => rotation_y_matrix(angle.to_radians()),
        TransformFunction::RotateZ { angle } => rotation_z_matrix(angle.to_radians()),
        TransformFunction::Rotate3d { x, y, z, angle } => {
            rotation_3d_matrix(*x, *y, *z, angle.to_radians())
        }
        TransformFunction::Skew { x, y } => skew_matrix(x.to_radians(), y.to_radians()),
        TransformFunction::SkewX { angle } => skew_x_matrix(angle.to_radians()),
        TransformFunction::SkewY { angle } => skew_y_matrix(angle.to_radians()),
        TransformFunction::Matrix { a, b, c, d, tx, ty } => matrix_2d(*a, *b, *c, *d, *tx, *ty),
        TransformFunction::Matrix3d { values } => matrix_3d(*values),
        TransformFunction::Perspective { value } => {
            let d = resolve_length(value, 0.0);
            perspective_matrix(d)
        }
    }
}

/// Apply transform origin to transformation matrix
///
/// Modifies the given matrix to apply the transformation relative to the specified origin point.
/// This is equivalent to: translate(origin) * matrix * translate(-origin)
///
/// # Examples
/// ```
/// use css_transforms::{parse_transform_origin, TransformMatrix, apply_transform_origin, Rect};
///
/// let mut matrix = TransformMatrix::identity();
/// let origin = parse_transform_origin("50% 50%").unwrap();
/// let rect = Rect { x: 0.0, y: 0.0, width: 100.0, height: 100.0 };
/// apply_transform_origin(&mut matrix, &origin, &rect);
/// ```
pub fn apply_transform_origin(
    matrix: &mut TransformMatrix,
    origin: &TransformOrigin,
    reference_box: &Rect,
) {
    let ox = resolve_length(&origin.x, reference_box.width);
    let oy = resolve_length(&origin.y, reference_box.height);
    let oz = resolve_length(&origin.z, 0.0);

    // Transform origin is applied as: translate(origin) * transform * translate(-origin)
    let translate_to_origin = translation_matrix(-ox, -oy, -oz);
    let translate_back = translation_matrix(ox, oy, oz);

    // Compose: translate_back * matrix * translate_to_origin
    let temp = matrix.multiply(&translate_to_origin);
    *matrix = translate_back.multiply(&temp);
}

fn resolve_length(length: &css_types::Length, reference: f32) -> f32 {
    match length.unit() {
        LengthUnit::Px => length.value(),
        LengthUnit::Percent => length.value() * reference / 100.0,
        LengthUnit::Em | LengthUnit::Rem => length.value() * 16.0, // Assume 16px base
        LengthUnit::Vw | LengthUnit::Vh => length.value() * 10.0,  // Simplified
    }
}

fn translation_matrix(tx: f32, ty: f32, tz: f32) -> TransformMatrix {
    TransformMatrix {
        matrix: [
            [1.0, 0.0, 0.0, tx],
            [0.0, 1.0, 0.0, ty],
            [0.0, 0.0, 1.0, tz],
            [0.0, 0.0, 0.0, 1.0],
        ],
    }
}

fn scale_matrix(sx: f32, sy: f32, sz: f32) -> TransformMatrix {
    TransformMatrix {
        matrix: [
            [sx, 0.0, 0.0, 0.0],
            [0.0, sy, 0.0, 0.0],
            [0.0, 0.0, sz, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    }
}

fn rotation_x_matrix(angle_rad: f32) -> TransformMatrix {
    let cos_a = angle_rad.cos();
    let sin_a = angle_rad.sin();

    TransformMatrix {
        matrix: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, cos_a, -sin_a, 0.0],
            [0.0, sin_a, cos_a, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    }
}

fn rotation_y_matrix(angle_rad: f32) -> TransformMatrix {
    let cos_a = angle_rad.cos();
    let sin_a = angle_rad.sin();

    TransformMatrix {
        matrix: [
            [cos_a, 0.0, sin_a, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-sin_a, 0.0, cos_a, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    }
}

fn rotation_z_matrix(angle_rad: f32) -> TransformMatrix {
    let cos_a = angle_rad.cos();
    let sin_a = angle_rad.sin();

    TransformMatrix {
        matrix: [
            [cos_a, -sin_a, 0.0, 0.0],
            [sin_a, cos_a, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    }
}

fn rotation_3d_matrix(x: f32, y: f32, z: f32, angle_rad: f32) -> TransformMatrix {
    // Normalize the axis vector
    let length = (x * x + y * y + z * z).sqrt();
    if length == 0.0 {
        return TransformMatrix::identity();
    }

    let x = x / length;
    let y = y / length;
    let z = z / length;

    let cos_a = angle_rad.cos();
    let sin_a = angle_rad.sin();
    let one_minus_cos = 1.0 - cos_a;

    TransformMatrix {
        matrix: [
            [
                cos_a + x * x * one_minus_cos,
                x * y * one_minus_cos - z * sin_a,
                x * z * one_minus_cos + y * sin_a,
                0.0,
            ],
            [
                y * x * one_minus_cos + z * sin_a,
                cos_a + y * y * one_minus_cos,
                y * z * one_minus_cos - x * sin_a,
                0.0,
            ],
            [
                z * x * one_minus_cos - y * sin_a,
                z * y * one_minus_cos + x * sin_a,
                cos_a + z * z * one_minus_cos,
                0.0,
            ],
            [0.0, 0.0, 0.0, 1.0],
        ],
    }
}

fn skew_matrix(angle_x_rad: f32, angle_y_rad: f32) -> TransformMatrix {
    let tan_x = angle_x_rad.tan();
    let tan_y = angle_y_rad.tan();

    TransformMatrix {
        matrix: [
            [1.0, tan_x, 0.0, 0.0],
            [tan_y, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    }
}

fn skew_x_matrix(angle_rad: f32) -> TransformMatrix {
    skew_matrix(angle_rad, 0.0)
}

fn skew_y_matrix(angle_rad: f32) -> TransformMatrix {
    skew_matrix(0.0, angle_rad)
}

fn matrix_2d(a: f32, b: f32, c: f32, d: f32, tx: f32, ty: f32) -> TransformMatrix {
    TransformMatrix {
        matrix: [
            [a, c, 0.0, tx],
            [b, d, 0.0, ty],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    }
}

fn matrix_3d(values: [f32; 16]) -> TransformMatrix {
    TransformMatrix {
        matrix: [
            [values[0], values[4], values[8], values[12]],
            [values[1], values[5], values[9], values[13]],
            [values[2], values[6], values[10], values[14]],
            [values[3], values[7], values[11], values[15]],
        ],
    }
}

fn perspective_matrix(distance: f32) -> TransformMatrix {
    if distance == 0.0 {
        return TransformMatrix::identity();
    }

    TransformMatrix {
        matrix: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, -1.0 / distance, 1.0],
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn test_translation_matrix() {
        let matrix = translation_matrix(10.0, 20.0, 30.0);
        assert_eq!(matrix.matrix[0][3], 10.0);
        assert_eq!(matrix.matrix[1][3], 20.0);
        assert_eq!(matrix.matrix[2][3], 30.0);
    }

    #[test]
    fn test_scale_matrix() {
        let matrix = scale_matrix(2.0, 3.0, 4.0);
        assert_eq!(matrix.matrix[0][0], 2.0);
        assert_eq!(matrix.matrix[1][1], 3.0);
        assert_eq!(matrix.matrix[2][2], 4.0);
    }

    #[test]
    fn test_rotation_z_90deg() {
        let matrix = rotation_z_matrix(PI / 2.0);
        assert!((matrix.matrix[0][0] - 0.0).abs() < 0.0001);
        assert!((matrix.matrix[0][1] - -1.0).abs() < 0.0001);
        assert!((matrix.matrix[1][0] - 1.0).abs() < 0.0001);
        assert!((matrix.matrix[1][1] - 0.0).abs() < 0.0001);
    }
}
