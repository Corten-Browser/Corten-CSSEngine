//! Transform and transform-origin parsing

use crate::{Angle, Transform, TransformFunction, TransformOrigin};
use css_parser_core::ParseError;
use css_types::{Length, LengthUnit};

/// Parse CSS transform property value
///
/// Parses a CSS transform string containing one or more transform functions.
///
/// # Examples
/// ```
/// use css_transforms::parse_transform;
///
/// let transform = parse_transform("translate(10px, 20px) rotate(45deg)").unwrap();
/// assert_eq!(transform.functions.len(), 2);
/// ```
///
/// # Errors
/// Returns `ParseError` if the input is not a valid CSS transform.
pub fn parse_transform(input: &str) -> Result<Transform, ParseError> {
    let input = input.trim();

    if input.is_empty() {
        return Err(ParseError::new(0, 0, "Empty transform string"));
    }

    let mut functions = Vec::new();
    let mut current_pos = 0;

    // Split by function calls
    while current_pos < input.len() {
        // Skip whitespace
        while current_pos < input.len() && input.chars().nth(current_pos).unwrap().is_whitespace() {
            current_pos += 1;
        }

        if current_pos >= input.len() {
            break;
        }

        // Find function name (up to opening paren)
        let start = current_pos;
        while current_pos < input.len() && input.chars().nth(current_pos).unwrap() != '(' {
            current_pos += 1;
        }

        if current_pos >= input.len() {
            return Err(ParseError::new(
                0,
                current_pos,
                "Expected opening parenthesis",
            ));
        }

        let func_name = &input[start..current_pos];
        current_pos += 1; // Skip '('

        // Find matching closing paren
        let args_start = current_pos;
        let mut paren_depth = 1;
        while current_pos < input.len() && paren_depth > 0 {
            match input.chars().nth(current_pos).unwrap() {
                '(' => paren_depth += 1,
                ')' => paren_depth -= 1,
                _ => {}
            }
            current_pos += 1;
        }

        if paren_depth != 0 {
            return Err(ParseError::new(0, current_pos, "Unmatched parenthesis"));
        }

        let args = &input[args_start..current_pos - 1];
        let function = parse_transform_function(func_name, args)?;
        functions.push(function);
    }

    if functions.is_empty() {
        return Err(ParseError::new(0, 0, "No transform functions found"));
    }

    Ok(Transform { functions })
}

fn parse_transform_function(name: &str, args: &str) -> Result<TransformFunction, ParseError> {
    let name = name.trim();
    let parts: Vec<&str> = args.split(',').map(|s| s.trim()).collect();

    match name {
        "translate" => {
            if parts.len() != 2 {
                return Err(ParseError::new(0, 0, "translate() requires 2 arguments"));
            }
            Ok(TransformFunction::Translate {
                x: parse_length(parts[0])?,
                y: parse_length(parts[1])?,
            })
        }
        "translateX" => {
            if parts.len() != 1 {
                return Err(ParseError::new(0, 0, "translateX() requires 1 argument"));
            }
            Ok(TransformFunction::TranslateX {
                value: parse_length(parts[0])?,
            })
        }
        "translateY" => {
            if parts.len() != 1 {
                return Err(ParseError::new(0, 0, "translateY() requires 1 argument"));
            }
            Ok(TransformFunction::TranslateY {
                value: parse_length(parts[0])?,
            })
        }
        "translateZ" => {
            if parts.len() != 1 {
                return Err(ParseError::new(0, 0, "translateZ() requires 1 argument"));
            }
            Ok(TransformFunction::TranslateZ {
                value: parse_length(parts[0])?,
            })
        }
        "translate3d" => {
            if parts.len() != 3 {
                return Err(ParseError::new(0, 0, "translate3d() requires 3 arguments"));
            }
            Ok(TransformFunction::Translate3d {
                x: parse_length(parts[0])?,
                y: parse_length(parts[1])?,
                z: parse_length(parts[2])?,
            })
        }
        "scale" => {
            if parts.len() == 1 {
                let value = parse_number(parts[0])?;
                Ok(TransformFunction::Scale { x: value, y: value })
            } else if parts.len() == 2 {
                Ok(TransformFunction::Scale {
                    x: parse_number(parts[0])?,
                    y: parse_number(parts[1])?,
                })
            } else {
                Err(ParseError::new(0, 0, "scale() requires 1 or 2 arguments"))
            }
        }
        "scaleX" => {
            if parts.len() != 1 {
                return Err(ParseError::new(0, 0, "scaleX() requires 1 argument"));
            }
            Ok(TransformFunction::ScaleX {
                value: parse_number(parts[0])?,
            })
        }
        "scaleY" => {
            if parts.len() != 1 {
                return Err(ParseError::new(0, 0, "scaleY() requires 1 argument"));
            }
            Ok(TransformFunction::ScaleY {
                value: parse_number(parts[0])?,
            })
        }
        "scaleZ" => {
            if parts.len() != 1 {
                return Err(ParseError::new(0, 0, "scaleZ() requires 1 argument"));
            }
            Ok(TransformFunction::ScaleZ {
                value: parse_number(parts[0])?,
            })
        }
        "scale3d" => {
            if parts.len() != 3 {
                return Err(ParseError::new(0, 0, "scale3d() requires 3 arguments"));
            }
            Ok(TransformFunction::Scale3d {
                x: parse_number(parts[0])?,
                y: parse_number(parts[1])?,
                z: parse_number(parts[2])?,
            })
        }
        "rotate" => {
            if parts.len() != 1 {
                return Err(ParseError::new(0, 0, "rotate() requires 1 argument"));
            }
            Ok(TransformFunction::Rotate {
                angle: parse_angle(parts[0])?,
            })
        }
        "rotateX" => {
            if parts.len() != 1 {
                return Err(ParseError::new(0, 0, "rotateX() requires 1 argument"));
            }
            Ok(TransformFunction::RotateX {
                angle: parse_angle(parts[0])?,
            })
        }
        "rotateY" => {
            if parts.len() != 1 {
                return Err(ParseError::new(0, 0, "rotateY() requires 1 argument"));
            }
            Ok(TransformFunction::RotateY {
                angle: parse_angle(parts[0])?,
            })
        }
        "rotateZ" => {
            if parts.len() != 1 {
                return Err(ParseError::new(0, 0, "rotateZ() requires 1 argument"));
            }
            Ok(TransformFunction::RotateZ {
                angle: parse_angle(parts[0])?,
            })
        }
        "rotate3d" => {
            if parts.len() != 4 {
                return Err(ParseError::new(0, 0, "rotate3d() requires 4 arguments"));
            }
            Ok(TransformFunction::Rotate3d {
                x: parse_number(parts[0])?,
                y: parse_number(parts[1])?,
                z: parse_number(parts[2])?,
                angle: parse_angle(parts[3])?,
            })
        }
        "skew" => {
            if parts.len() != 2 {
                return Err(ParseError::new(0, 0, "skew() requires 2 arguments"));
            }
            Ok(TransformFunction::Skew {
                x: parse_angle(parts[0])?,
                y: parse_angle(parts[1])?,
            })
        }
        "skewX" => {
            if parts.len() != 1 {
                return Err(ParseError::new(0, 0, "skewX() requires 1 argument"));
            }
            Ok(TransformFunction::SkewX {
                angle: parse_angle(parts[0])?,
            })
        }
        "skewY" => {
            if parts.len() != 1 {
                return Err(ParseError::new(0, 0, "skewY() requires 1 argument"));
            }
            Ok(TransformFunction::SkewY {
                angle: parse_angle(parts[0])?,
            })
        }
        "matrix" => {
            if parts.len() != 6 {
                return Err(ParseError::new(0, 0, "matrix() requires 6 arguments"));
            }
            Ok(TransformFunction::Matrix {
                a: parse_number(parts[0])?,
                b: parse_number(parts[1])?,
                c: parse_number(parts[2])?,
                d: parse_number(parts[3])?,
                tx: parse_number(parts[4])?,
                ty: parse_number(parts[5])?,
            })
        }
        "matrix3d" => {
            if parts.len() != 16 {
                return Err(ParseError::new(0, 0, "matrix3d() requires 16 arguments"));
            }
            let mut values = [0.0; 16];
            for (i, part) in parts.iter().enumerate() {
                values[i] = parse_number(part)?;
            }
            Ok(TransformFunction::Matrix3d { values })
        }
        "perspective" => {
            if parts.len() != 1 {
                return Err(ParseError::new(0, 0, "perspective() requires 1 argument"));
            }
            Ok(TransformFunction::Perspective {
                value: parse_length(parts[0])?,
            })
        }
        _ => Err(ParseError::new(
            0,
            0,
            format!("Unknown transform function: {}", name),
        )),
    }
}

/// Parse transform-origin property value
///
/// Parses a CSS transform-origin string, supporting keywords (left, right, top, bottom, center),
/// lengths, and percentages in 2D or 3D.
///
/// # Examples
/// ```
/// use css_transforms::parse_transform_origin;
///
/// let origin = parse_transform_origin("center center").unwrap();
/// assert_eq!(origin.x.value(), 50.0);
/// assert_eq!(origin.y.value(), 50.0);
/// ```
///
/// # Errors
/// Returns `ParseError` if the input is not a valid CSS transform-origin.
pub fn parse_transform_origin(input: &str) -> Result<TransformOrigin, ParseError> {
    let input = input.trim();

    if input.is_empty() {
        return Err(ParseError::new(0, 0, "Empty transform-origin string"));
    }

    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.is_empty() {
        return Err(ParseError::new(0, 0, "Empty transform-origin string"));
    }

    let x = parse_origin_component(parts[0], true)?;
    let y = if parts.len() > 1 {
        parse_origin_component(parts[1], false)?
    } else {
        // If only one value, y defaults to center
        Length::new(50.0, LengthUnit::Percent)
    };
    let z = if parts.len() > 2 {
        parse_length(parts[2])?
    } else {
        Length::new(0.0, LengthUnit::Px)
    };

    Ok(TransformOrigin { x, y, z })
}

fn parse_origin_component(input: &str, is_x: bool) -> Result<Length, ParseError> {
    match input {
        "left" if is_x => Ok(Length::new(0.0, LengthUnit::Percent)),
        "right" if is_x => Ok(Length::new(100.0, LengthUnit::Percent)),
        "top" if !is_x => Ok(Length::new(0.0, LengthUnit::Percent)),
        "bottom" if !is_x => Ok(Length::new(100.0, LengthUnit::Percent)),
        "center" => Ok(Length::new(50.0, LengthUnit::Percent)),
        _ => parse_length(input),
    }
}

fn parse_length(input: &str) -> Result<Length, ParseError> {
    use css_types::CssValue;
    Length::parse(input).map_err(|e| ParseError::new(0, 0, e.to_string()))
}

fn parse_angle(input: &str) -> Result<Angle, ParseError> {
    Angle::parse(input).map_err(|e| ParseError::new(0, 0, e.to_string()))
}

fn parse_number(input: &str) -> Result<f32, ParseError> {
    input
        .trim()
        .parse::<f32>()
        .map_err(|_| ParseError::new(0, 0, format!("Invalid number: {}", input)))
}
