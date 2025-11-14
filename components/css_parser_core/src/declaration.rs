//! Property declaration parsing for CSS2.1

use crate::{ParseError, PropertyDeclaration, PropertyValue};
use css_types::{Color, Length, LengthUnit};

/// Parse a block of declarations (inside braces)
pub fn parse_declarations(input: &str) -> Result<Vec<PropertyDeclaration>, ParseError> {
    let input = input.trim();

    if input.is_empty() {
        return Ok(Vec::new());
    }

    let mut declarations = Vec::new();

    // Split by semicolon
    for decl_text in input.split(';') {
        let decl_text = decl_text.trim();
        if decl_text.is_empty() {
            continue;
        }

        let declaration = parse_single_declaration(decl_text)?;
        declarations.push(declaration);
    }

    Ok(declarations)
}

/// Parse a single property declaration
fn parse_single_declaration(input: &str) -> Result<PropertyDeclaration, ParseError> {
    // Split by colon
    let parts: Vec<&str> = input.splitn(2, ':').collect();

    if parts.len() != 2 {
        return Err(ParseError::new(1, 1, "Invalid declaration: missing colon"));
    }

    let property = parts[0].trim();
    let value_text = parts[1].trim();

    if property.is_empty() {
        return Err(ParseError::new(1, 1, "Empty property name"));
    }

    if value_text.is_empty() {
        return Err(ParseError::new(1, 1, "Empty property value"));
    }

    // Check for !important
    let (value_text, important) = if value_text.ends_with("!important") {
        let val = value_text.trim_end_matches("!important").trim();
        (val, true)
    } else {
        (value_text, false)
    };

    // Parse the value based on property type
    let value = parse_property_value(property, value_text)?;

    Ok(PropertyDeclaration {
        name: property.to_string(),
        value,
        important,
    })
}

/// Parse a property value based on property name
fn parse_property_value(property: &str, value: &str) -> Result<PropertyValue, ParseError> {
    let value = value.trim();

    // Try to parse as color for color properties
    if property == "color" || property == "background-color" {
        if let Ok(color) = parse_color(value) {
            return Ok(PropertyValue::Color(color));
        }
    }

    // Try to parse as length for size properties
    if is_length_property(property) {
        if let Ok(length_value) = parse_length_value(value) {
            return Ok(length_value);
        }
    }

    // Default to keyword or string
    Ok(PropertyValue::Keyword(value.to_string()))
}

/// Check if a property expects length values
fn is_length_property(property: &str) -> bool {
    matches!(
        property,
        "width"
            | "height"
            | "margin"
            | "padding"
            | "top"
            | "left"
            | "right"
            | "bottom"
            | "margin-top"
            | "margin-right"
            | "margin-bottom"
            | "margin-left"
            | "padding-top"
            | "padding-right"
            | "padding-bottom"
            | "padding-left"
            | "font-size"
            | "line-height"
            | "border-width"
    )
}

/// Parse a CSS color value
fn parse_color(value: &str) -> Result<Color, ParseError> {
    let value = value.trim();

    // Named colors
    match value.to_lowercase().as_str() {
        "red" => return Ok(Color::rgb(255, 0, 0)),
        "green" => return Ok(Color::rgb(0, 128, 0)),
        "blue" => return Ok(Color::rgb(0, 0, 255)),
        "white" => return Ok(Color::rgb(255, 255, 255)),
        "black" => return Ok(Color::rgb(0, 0, 0)),
        _ => {}
    }

    // Hex colors
    if let Some(hex) = value.strip_prefix('#') {
        return parse_hex_color(hex);
    }

    // RGB/RGBA function
    if value.starts_with("rgb(") || value.starts_with("rgba(") {
        return parse_rgb_color(value);
    }

    Err(ParseError::new(
        1,
        1,
        format!("Invalid color value: {}", value),
    ))
}

/// Parse hex color (#RGB or #RRGGBB)
fn parse_hex_color(hex: &str) -> Result<Color, ParseError> {
    let hex = hex.trim();

    let (r, g, b) = match hex.len() {
        3 => {
            // #RGB format
            let r = u8::from_str_radix(&hex[0..1].repeat(2), 16)
                .map_err(|_| ParseError::new(1, 1, "Invalid hex digit"))?;
            let g = u8::from_str_radix(&hex[1..2].repeat(2), 16)
                .map_err(|_| ParseError::new(1, 1, "Invalid hex digit"))?;
            let b = u8::from_str_radix(&hex[2..3].repeat(2), 16)
                .map_err(|_| ParseError::new(1, 1, "Invalid hex digit"))?;
            (r, g, b)
        }
        6 => {
            // #RRGGBB format
            let r = u8::from_str_radix(&hex[0..2], 16)
                .map_err(|_| ParseError::new(1, 1, "Invalid hex digit"))?;
            let g = u8::from_str_radix(&hex[2..4], 16)
                .map_err(|_| ParseError::new(1, 1, "Invalid hex digit"))?;
            let b = u8::from_str_radix(&hex[4..6], 16)
                .map_err(|_| ParseError::new(1, 1, "Invalid hex digit"))?;
            (r, g, b)
        }
        _ => return Err(ParseError::new(1, 1, "Invalid hex color length")),
    };

    Ok(Color::rgb(r, g, b))
}

/// Parse rgb() or rgba() color function
fn parse_rgb_color(value: &str) -> Result<Color, ParseError> {
    let is_rgba = value.starts_with("rgba(");
    let start = if is_rgba { 5 } else { 4 };
    let end = value.len() - 1;

    if !value.ends_with(')') {
        return Err(ParseError::new(1, 1, "Missing closing parenthesis"));
    }

    let args = &value[start..end];
    let parts: Vec<&str> = args.split(',').map(|s| s.trim()).collect();

    if (is_rgba && parts.len() != 4) || (!is_rgba && parts.len() != 3) {
        return Err(ParseError::new(1, 1, "Invalid number of arguments"));
    }

    let r = parts[0]
        .parse::<u8>()
        .map_err(|_| ParseError::new(1, 1, "Invalid red value"))?;
    let g = parts[1]
        .parse::<u8>()
        .map_err(|_| ParseError::new(1, 1, "Invalid green value"))?;
    let b = parts[2]
        .parse::<u8>()
        .map_err(|_| ParseError::new(1, 1, "Invalid blue value"))?;

    if is_rgba {
        let a = parts[3]
            .parse::<f32>()
            .map_err(|_| ParseError::new(1, 1, "Invalid alpha value"))?;
        Ok(Color::rgba(r, g, b, a))
    } else {
        Ok(Color::rgb(r, g, b))
    }
}

/// Parse a CSS length value (returns PropertyValue to handle auto)
fn parse_length_value(value: &str) -> Result<PropertyValue, ParseError> {
    let value = value.trim();

    // Special keyword values
    if value == "auto" {
        return Ok(PropertyValue::Keyword("auto".to_string()));
    }

    // Extract number and unit
    let mut num_end = 0;
    for (i, ch) in value.char_indices() {
        if !ch.is_numeric() && ch != '.' && ch != '-' {
            num_end = i;
            break;
        }
    }

    if num_end == 0 {
        num_end = value.len();
    }

    let num_str = &value[..num_end];
    let unit_str = &value[num_end..];

    let num = num_str
        .parse::<f32>()
        .map_err(|_| ParseError::new(1, 1, format!("Invalid number: {}", num_str)))?;

    let unit = match unit_str {
        "px" | "" => LengthUnit::Px,
        "em" => LengthUnit::Em,
        "rem" => LengthUnit::Rem,
        "%" => LengthUnit::Percent,
        "vw" => LengthUnit::Vw,
        "vh" => LengthUnit::Vh,
        _ => return Err(ParseError::new(1, 1, format!("Unknown unit: {}", unit_str))),
    };

    Ok(PropertyValue::Length(Length::new(num, unit)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_color_red() {
        let color = parse_color("red").unwrap();
        assert_eq!(color, Color::rgb(255, 0, 0));
    }

    #[test]
    fn test_parse_hex_color_short() {
        let color = parse_hex_color("F00").unwrap();
        assert_eq!(color, Color::rgb(255, 0, 0));
    }

    #[test]
    fn test_parse_hex_color_long() {
        let color = parse_hex_color("FF5733").unwrap();
        assert_eq!(color, Color::rgb(255, 87, 51));
    }

    #[test]
    fn test_parse_rgb_color() {
        let color = parse_rgb_color("rgb(255, 87, 51)").unwrap();
        assert_eq!(color, Color::rgb(255, 87, 51));
    }

    #[test]
    fn test_parse_length_px() {
        let value = parse_length_value("10px").unwrap();
        assert_eq!(
            value,
            PropertyValue::Length(Length::new(10.0, LengthUnit::Px))
        );
    }

    #[test]
    fn test_parse_length_em() {
        let value = parse_length_value("1.5em").unwrap();
        assert_eq!(
            value,
            PropertyValue::Length(Length::new(1.5, LengthUnit::Em))
        );
    }

    #[test]
    fn test_parse_single_declaration() {
        let decl = parse_single_declaration("color: red").unwrap();
        assert_eq!(decl.name, "color");
        assert_eq!(decl.value, PropertyValue::Color(Color::rgb(255, 0, 0)));
        assert!(!decl.important);
    }

    #[test]
    fn test_parse_declaration_important() {
        let decl = parse_single_declaration("color: red !important").unwrap();
        assert_eq!(decl.name, "color");
        assert!(decl.important);
    }

    #[test]
    fn test_parse_declarations_multiple() {
        let decls = parse_declarations("color: red; margin: 10px").unwrap();
        assert_eq!(decls.len(), 2);
        assert_eq!(decls[0].name, "color");
        assert_eq!(decls[1].name, "margin");
    }
}
