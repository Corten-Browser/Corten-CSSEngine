//! CSS Transform Parsing and Computation
//!
//! This module provides types and functions for parsing and computing CSS transforms,
//! including 2D and 3D transformations.

use css_types::{CssError, Length};
use std::f32::consts::PI;

mod matrix;
mod parsing;

pub use matrix::*;
pub use parsing::*;

// ============================================================================
// Angle Type
// ============================================================================

/// CSS angle units
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AngleUnit {
    /// Degrees
    Deg,
    /// Radians
    Rad,
    /// Gradians
    Grad,
    /// Turns
    Turn,
}

/// CSS angle value with unit
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Angle {
    value: f32,
    unit: AngleUnit,
}

impl Angle {
    /// Create a new angle
    pub fn new(value: f32, unit: AngleUnit) -> Self {
        Self { value, unit }
    }

    /// Get the numeric value
    pub fn value(&self) -> f32 {
        self.value
    }

    /// Get the unit
    pub fn unit(&self) -> AngleUnit {
        self.unit
    }

    /// Convert angle to radians
    pub fn to_radians(&self) -> f32 {
        match self.unit {
            AngleUnit::Deg => self.value * PI / 180.0,
            AngleUnit::Rad => self.value,
            AngleUnit::Grad => self.value * PI / 200.0,
            AngleUnit::Turn => self.value * 2.0 * PI,
        }
    }

    /// Parse an angle from string (e.g., "45deg", "1.5rad")
    pub fn parse(input: &str) -> Result<Self, CssError> {
        let input = input.trim();

        if input.is_empty() {
            return Err(CssError::ParseError("Empty angle string".to_string()));
        }

        // Find where the number ends and the unit begins
        let mut num_end = 0;
        for (i, ch) in input.chars().enumerate() {
            if ch.is_ascii_digit() || ch == '.' || ch == '-' || ch == '+' {
                num_end = i + 1;
            } else {
                break;
            }
        }

        if num_end == 0 {
            return Err(CssError::ParseError(
                "Angle must start with a number".to_string(),
            ));
        }

        let value_str = &input[..num_end];
        let unit_str = &input[num_end..];

        if unit_str.is_empty() {
            return Err(CssError::ParseError("Angle must have a unit".to_string()));
        }

        let value = value_str
            .parse::<f32>()
            .map_err(|_| CssError::ParseError("Invalid number".to_string()))?;

        let unit = match unit_str {
            "deg" => AngleUnit::Deg,
            "rad" => AngleUnit::Rad,
            "grad" => AngleUnit::Grad,
            "turn" => AngleUnit::Turn,
            _ => {
                return Err(CssError::ParseError(format!(
                    "Unknown angle unit: {}",
                    unit_str
                )))
            }
        };

        Ok(Self::new(value, unit))
    }
}

// ============================================================================
// Rect Type (for reference box)
// ============================================================================

/// Rectangle defining a reference box for transform computation
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

// ============================================================================
// Transform Types
// ============================================================================

/// Individual transform function
#[derive(Debug, Clone, PartialEq)]
pub enum TransformFunction {
    Translate {
        x: Length,
        y: Length,
    },
    TranslateX {
        value: Length,
    },
    TranslateY {
        value: Length,
    },
    TranslateZ {
        value: Length,
    },
    Translate3d {
        x: Length,
        y: Length,
        z: Length,
    },
    Scale {
        x: f32,
        y: f32,
    },
    ScaleX {
        value: f32,
    },
    ScaleY {
        value: f32,
    },
    ScaleZ {
        value: f32,
    },
    Scale3d {
        x: f32,
        y: f32,
        z: f32,
    },
    Rotate {
        angle: Angle,
    },
    RotateX {
        angle: Angle,
    },
    RotateY {
        angle: Angle,
    },
    RotateZ {
        angle: Angle,
    },
    Rotate3d {
        x: f32,
        y: f32,
        z: f32,
        angle: Angle,
    },
    Skew {
        x: Angle,
        y: Angle,
    },
    SkewX {
        angle: Angle,
    },
    SkewY {
        angle: Angle,
    },
    Matrix {
        a: f32,
        b: f32,
        c: f32,
        d: f32,
        tx: f32,
        ty: f32,
    },
    Matrix3d {
        values: [f32; 16],
    },
    Perspective {
        value: Length,
    },
}

/// Complete transform value (list of functions)
#[derive(Debug, Clone, PartialEq)]
pub struct Transform {
    pub functions: Vec<TransformFunction>,
}

/// Transform origin point
#[derive(Debug, Clone, PartialEq)]
pub struct TransformOrigin {
    pub x: Length,
    pub y: Length,
    pub z: Length,
}

/// Transform style (preserve-3d or flat)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransformStyle {
    Flat,
    Preserve3d,
}

/// Computed 4x4 transformation matrix
#[derive(Debug, Clone, PartialEq)]
pub struct TransformMatrix {
    pub matrix: [[f32; 4]; 4],
}

impl TransformMatrix {
    /// Create an identity matrix
    pub fn identity() -> Self {
        Self {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Multiply two matrices
    pub fn multiply(&self, other: &TransformMatrix) -> TransformMatrix {
        let mut result = TransformMatrix::identity();

        for i in 0..4 {
            for j in 0..4 {
                result.matrix[i][j] = 0.0;
                for k in 0..4 {
                    result.matrix[i][j] += self.matrix[i][k] * other.matrix[k][j];
                }
            }
        }

        result
    }
}

// ============================================================================
// Transform Computer Trait
// ============================================================================

/// Transform computation interface
pub trait TransformComputer {
    /// Compute a transformation matrix from a transform, origin, and reference box
    fn compute_transform(
        &self,
        transform: &Transform,
        origin: &TransformOrigin,
        reference_box: &Rect,
    ) -> TransformMatrix;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_angle_to_radians() {
        let angle = Angle::new(180.0, AngleUnit::Deg);
        assert!((angle.to_radians() - PI).abs() < 0.0001);
    }

    #[test]
    fn test_angle_parse() {
        let angle = Angle::parse("45deg").unwrap();
        assert_eq!(angle.value(), 45.0);
        assert_eq!(angle.unit(), AngleUnit::Deg);
    }

    #[test]
    fn test_identity_matrix() {
        let matrix = TransformMatrix::identity();
        assert_eq!(matrix.matrix[0][0], 1.0);
        assert_eq!(matrix.matrix[1][1], 1.0);
        assert_eq!(matrix.matrix[2][2], 1.0);
        assert_eq!(matrix.matrix[3][3], 1.0);
    }
}
