//! Media query parsing implementation

use crate::*;
use css_parser_core::ParseError;
use css_types::{Length, LengthUnit};

/// Parse a complete media query from a string
pub fn parse_media_query(input: &str) -> Result<MediaQuery, ParseError> {
    let input = input.trim();

    if input.is_empty() {
        return Err(ParseError::new(0, 0, "Empty media query"));
    }

    // Check for NOT prefix
    let (negated, input) = if let Some(stripped) = input.strip_prefix("not ") {
        (true, stripped.trim())
    } else {
        (false, input)
    };

    // Check if starts with parenthesis (condition-only query)
    if input.starts_with('(') {
        let condition = parse_media_condition(input)?;
        return Ok(MediaQuery::new(None, Some(condition), negated));
    }

    // Try to parse media type
    if let Some(space_pos) = input.find(" and ") {
        // Media type with condition: "screen and (min-width: 768px)"
        let media_type_str = input[..space_pos].trim();
        let condition_str = input[space_pos + 5..].trim();

        let media_type = parse_media_type(media_type_str)?;
        let condition = parse_media_condition(condition_str)?;

        Ok(MediaQuery::new(Some(media_type), Some(condition), negated))
    } else {
        // Media type only: "screen"
        let media_type = parse_media_type(input)?;
        Ok(MediaQuery::new(Some(media_type), None, negated))
    }
}

/// Parse a comma-separated media query list
pub fn parse_media_query_list(input: &str) -> Result<MediaQueryList, ParseError> {
    let input = input.trim();

    if input.is_empty() {
        return Ok(MediaQueryList::empty());
    }

    let mut queries = Vec::new();
    let parts = input.split(',');

    for part in parts {
        let query = parse_media_query(part.trim())?;
        queries.push(query);
    }

    Ok(MediaQueryList::new(queries))
}

/// Parse a media type
fn parse_media_type(input: &str) -> Result<MediaType, ParseError> {
    match input {
        "all" => Ok(MediaType::All),
        "screen" => Ok(MediaType::Screen),
        "print" => Ok(MediaType::Print),
        "speech" => Ok(MediaType::Speech),
        _ => Err(ParseError::new(
            0,
            0,
            format!("Unknown media type: {}", input),
        )),
    }
}

/// Parse a media condition (with potential AND/OR operators)
fn parse_media_condition(input: &str) -> Result<MediaCondition, ParseError> {
    let input = input.trim();

    // Check for AND operator
    if let Some(and_pos) = find_operator(input, " and ") {
        let left_str = &input[..and_pos].trim();
        let right_str = &input[and_pos + 5..].trim();

        let left = parse_media_condition(left_str)?;
        let right = parse_media_condition(right_str)?;

        return Ok(MediaCondition::And {
            left: Box::new(left),
            right: Box::new(right),
        });
    }

    // Check for OR operator
    if let Some(or_pos) = find_operator(input, " or ") {
        let left_str = &input[..or_pos].trim();
        let right_str = &input[or_pos + 4..].trim();

        let left = parse_media_condition(left_str)?;
        let right = parse_media_condition(right_str)?;

        return Ok(MediaCondition::Or {
            left: Box::new(left),
            right: Box::new(right),
        });
    }

    // Check for NOT operator
    if let Some(stripped) = input.strip_prefix("not ") {
        let inner_str = stripped.trim();
        let inner = parse_media_condition(inner_str)?;
        return Ok(MediaCondition::Not {
            condition: Box::new(inner),
        });
    }

    // Single feature: (min-width: 768px)
    if !input.starts_with('(') || !input.ends_with(')') {
        return Err(ParseError::new(
            0,
            0,
            "Media condition must be wrapped in parentheses",
        ));
    }

    let content = &input[1..input.len() - 1].trim();
    parse_media_feature_with_range(content)
}

/// Find an operator at the top level (not inside parentheses)
fn find_operator(input: &str, operator: &str) -> Option<usize> {
    let mut depth = 0;
    let chars: Vec<char> = input.chars().collect();
    let op_chars: Vec<char> = operator.chars().collect();

    for i in 0..chars.len() {
        if chars[i] == '(' {
            depth += 1;
        } else if chars[i] == ')' {
            depth -= 1;
        } else if depth == 0 {
            // Check if operator starts here
            if i + op_chars.len() <= chars.len() {
                let slice: String = chars[i..i + op_chars.len()].iter().collect();
                if slice == operator {
                    return Some(i);
                }
            }
        }
    }

    None
}

/// Parse a media feature with range type
fn parse_media_feature_with_range(content: &str) -> Result<MediaCondition, ParseError> {
    let content = content.trim();

    // Check for colon (feature with value)
    if let Some(colon_pos) = content.find(':') {
        let feature_name = content[..colon_pos].trim();
        let value_str = content[colon_pos + 1..].trim();

        // Determine range type from feature name
        let (base_name, range) = if let Some(stripped) = feature_name.strip_prefix("min-") {
            (stripped, RangeType::Min)
        } else if let Some(stripped) = feature_name.strip_prefix("max-") {
            (stripped, RangeType::Max)
        } else {
            (feature_name, RangeType::Exact)
        };

        let feature = parse_media_feature(base_name, Some(value_str))?;
        Ok(MediaCondition::Feature { feature, range })
    } else {
        // Boolean feature (no value): (color)
        let feature = parse_media_feature(content, None)?;
        Ok(MediaCondition::Feature {
            feature,
            range: RangeType::Exact,
        })
    }
}

/// Parse a media feature
fn parse_media_feature(name: &str, value: Option<&str>) -> Result<MediaFeature, ParseError> {
    match name {
        "width" => {
            if let Some(val) = value {
                let length = parse_length(val)?;
                Ok(MediaFeature::Width(Some(length)))
            } else {
                Ok(MediaFeature::Width(None))
            }
        }
        "height" => {
            if let Some(val) = value {
                let length = parse_length(val)?;
                Ok(MediaFeature::Height(Some(length)))
            } else {
                Ok(MediaFeature::Height(None))
            }
        }
        "orientation" => {
            let val = value.ok_or_else(|| ParseError::new(0, 0, "orientation requires a value"))?;
            let orientation = match val {
                "portrait" => Orientation::Portrait,
                "landscape" => Orientation::Landscape,
                _ => {
                    return Err(ParseError::new(
                        0,
                        0,
                        format!("Unknown orientation: {}", val),
                    ))
                }
            };
            Ok(MediaFeature::Orientation(orientation))
        }
        "aspect-ratio" => {
            let val =
                value.ok_or_else(|| ParseError::new(0, 0, "aspect-ratio requires a value"))?;
            let parts: Vec<&str> = val.split('/').collect();
            if parts.len() != 2 {
                return Err(ParseError::new(0, 0, "aspect-ratio must be in format N/M"));
            }
            let numerator = parts[0]
                .trim()
                .parse::<u32>()
                .map_err(|_| ParseError::new(0, 0, "Invalid numerator"))?;
            let denominator = parts[1]
                .trim()
                .parse::<u32>()
                .map_err(|_| ParseError::new(0, 0, "Invalid denominator"))?;
            Ok(MediaFeature::AspectRatio {
                numerator,
                denominator,
            })
        }
        "resolution" => {
            let val = value.ok_or_else(|| ParseError::new(0, 0, "resolution requires a value"))?;
            let resolution = parse_resolution(val)?;
            Ok(MediaFeature::Resolution(resolution))
        }
        "color" => {
            if let Some(val) = value {
                let bits = val
                    .parse::<u32>()
                    .map_err(|_| ParseError::new(0, 0, "Invalid color bits"))?;
                Ok(MediaFeature::Color(Some(bits)))
            } else {
                Ok(MediaFeature::Color(None))
            }
        }
        "color-index" => {
            if let Some(val) = value {
                let index = val
                    .parse::<u32>()
                    .map_err(|_| ParseError::new(0, 0, "Invalid color index"))?;
                Ok(MediaFeature::ColorIndex(Some(index)))
            } else {
                Ok(MediaFeature::ColorIndex(None))
            }
        }
        "monochrome" => {
            if let Some(val) = value {
                let bits = val
                    .parse::<u32>()
                    .map_err(|_| ParseError::new(0, 0, "Invalid monochrome bits"))?;
                Ok(MediaFeature::Monochrome(Some(bits)))
            } else {
                Ok(MediaFeature::Monochrome(None))
            }
        }
        "grid" => {
            let val = value.ok_or_else(|| ParseError::new(0, 0, "grid requires a value"))?;
            let grid = val == "1" || val == "true";
            Ok(MediaFeature::Grid(grid))
        }
        "scan" => {
            let val = value.ok_or_else(|| ParseError::new(0, 0, "scan requires a value"))?;
            let scan = match val {
                "interlace" => Scan::Interlace,
                "progressive" => Scan::Progressive,
                _ => return Err(ParseError::new(0, 0, format!("Unknown scan type: {}", val))),
            };
            Ok(MediaFeature::Scan(scan))
        }
        "update" => {
            let val = value.ok_or_else(|| ParseError::new(0, 0, "update requires a value"))?;
            let update = match val {
                "none" => Update::None,
                "slow" => Update::Slow,
                "fast" => Update::Fast,
                _ => {
                    return Err(ParseError::new(
                        0,
                        0,
                        format!("Unknown update type: {}", val),
                    ))
                }
            };
            Ok(MediaFeature::Update(update))
        }
        "hover" => {
            let val = value.ok_or_else(|| ParseError::new(0, 0, "hover requires a value"))?;
            let hover = match val {
                "none" => HoverCapability::None,
                "hover" => HoverCapability::Hover,
                _ => {
                    return Err(ParseError::new(
                        0,
                        0,
                        format!("Unknown hover type: {}", val),
                    ))
                }
            };
            Ok(MediaFeature::Hover(hover))
        }
        "pointer" => {
            let val = value.ok_or_else(|| ParseError::new(0, 0, "pointer requires a value"))?;
            let pointer = match val {
                "none" => PointerCapability::None,
                "coarse" => PointerCapability::Coarse,
                "fine" => PointerCapability::Fine,
                _ => {
                    return Err(ParseError::new(
                        0,
                        0,
                        format!("Unknown pointer type: {}", val),
                    ))
                }
            };
            Ok(MediaFeature::Pointer(pointer))
        }
        "prefers-color-scheme" => {
            let val = value
                .ok_or_else(|| ParseError::new(0, 0, "prefers-color-scheme requires a value"))?;
            let scheme = match val {
                "light" => ColorScheme::Light,
                "dark" => ColorScheme::Dark,
                _ => {
                    return Err(ParseError::new(
                        0,
                        0,
                        format!("Unknown color scheme: {}", val),
                    ))
                }
            };
            Ok(MediaFeature::PrefersColorScheme(scheme))
        }
        "prefers-reduced-motion" => {
            let val = value
                .ok_or_else(|| ParseError::new(0, 0, "prefers-reduced-motion requires a value"))?;
            let motion = match val {
                "no-preference" => ReducedMotion::NoPreference,
                "reduce" => ReducedMotion::Reduce,
                _ => {
                    return Err(ParseError::new(
                        0,
                        0,
                        format!("Unknown reduced motion: {}", val),
                    ))
                }
            };
            Ok(MediaFeature::PrefersReducedMotion(motion))
        }
        "prefers-contrast" => {
            let val =
                value.ok_or_else(|| ParseError::new(0, 0, "prefers-contrast requires a value"))?;
            let contrast = match val {
                "no-preference" => Contrast::NoPreference,
                "more" => Contrast::More,
                "less" => Contrast::Less,
                _ => return Err(ParseError::new(0, 0, format!("Unknown contrast: {}", val))),
            };
            Ok(MediaFeature::PrefersContrast(contrast))
        }
        _ => Err(ParseError::new(
            0,
            0,
            format!("Unknown media feature: {}", name),
        )),
    }
}

/// Parse a CSS length value
fn parse_length(input: &str) -> Result<Length, ParseError> {
    let input = input.trim();

    // Find where the number ends and unit begins
    let mut num_end = 0;
    for (i, ch) in input.chars().enumerate() {
        if ch.is_ascii_digit() || ch == '.' || ch == '-' {
            num_end = i + 1;
        } else {
            break;
        }
    }

    if num_end == 0 {
        return Err(ParseError::new(0, 0, "Length must start with a number"));
    }

    let value_str = &input[..num_end];
    let unit_str = &input[num_end..];

    let value = value_str
        .parse::<f32>()
        .map_err(|_| ParseError::new(0, 0, "Invalid number"))?;

    let unit = match unit_str {
        "px" => LengthUnit::Px,
        "em" => LengthUnit::Em,
        "rem" => LengthUnit::Rem,
        "%" => LengthUnit::Percent,
        "vw" => LengthUnit::Vw,
        "vh" => LengthUnit::Vh,
        _ => return Err(ParseError::new(0, 0, format!("Unknown unit: {}", unit_str))),
    };

    Ok(Length::new(value, unit))
}

/// Parse a resolution value
fn parse_resolution(input: &str) -> Result<Resolution, ParseError> {
    let input = input.trim();

    // Find where the number ends and unit begins
    let mut num_end = 0;
    for (i, ch) in input.chars().enumerate() {
        if ch.is_ascii_digit() || ch == '.' {
            num_end = i + 1;
        } else {
            break;
        }
    }

    if num_end == 0 {
        return Err(ParseError::new(0, 0, "Resolution must start with a number"));
    }

    let value_str = &input[..num_end];
    let unit_str = &input[num_end..];

    let value = value_str
        .parse::<f32>()
        .map_err(|_| ParseError::new(0, 0, "Invalid resolution value"))?;

    let unit = match unit_str {
        "dpi" => ResolutionUnit::Dpi,
        "dpcm" => ResolutionUnit::Dpcm,
        "dppx" | "x" => ResolutionUnit::Dppx,
        _ => {
            return Err(ParseError::new(
                0,
                0,
                format!("Unknown resolution unit: {}", unit_str),
            ))
        }
    };

    Ok(Resolution::new(value, unit))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_media_type() {
        let result = parse_media_query("screen");
        assert!(result.is_ok());
        let query = result.unwrap();
        assert_eq!(query.media_type, Some(MediaType::Screen));
    }

    #[test]
    fn test_parse_min_width_feature() {
        let result = parse_media_query("(min-width: 768px)");
        assert!(result.is_ok());
        let query = result.unwrap();
        assert!(query.condition.is_some());
    }

    #[test]
    fn test_find_operator_simple() {
        let input = "(min-width: 768px) and (max-width: 1024px)";
        let pos = find_operator(input, " and ");
        assert_eq!(pos, Some(18));
    }

    #[test]
    fn test_find_operator_nested() {
        let input = "((a and b)) and (c)";
        let pos = find_operator(input, " and ");
        assert_eq!(pos, Some(11));
    }
}
