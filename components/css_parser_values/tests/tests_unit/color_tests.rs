// Unit tests for advanced color parsing

use css_parser_values::parse_color_value;

// Test hex colors
#[test]
fn test_parse_hex_color_3_digits() {
    let result = parse_color_value("#F00");
    assert!(result.is_ok());
    let color = result.unwrap();
    assert_eq!(color.r(), 255);
    assert_eq!(color.g(), 0);
    assert_eq!(color.b(), 0);
}

#[test]
fn test_parse_hex_color_6_digits() {
    let result = parse_color_value("#FF5733");
    assert!(result.is_ok());
    let color = result.unwrap();
    assert_eq!(color.r(), 255);
    assert_eq!(color.g(), 87);
    assert_eq!(color.b(), 51);
}

// Test rgb/rgba
#[test]
fn test_parse_rgb_color() {
    let result = parse_color_value("rgb(255, 87, 51)");
    assert!(result.is_ok());
    let color = result.unwrap();
    assert_eq!(color.r(), 255);
    assert_eq!(color.g(), 87);
    assert_eq!(color.b(), 51);
}

#[test]
fn test_parse_rgba_color() {
    let result = parse_color_value("rgba(255, 87, 51, 0.5)");
    assert!(result.is_ok());
    let color = result.unwrap();
    assert_eq!(color.r(), 255);
    assert_eq!(color.g(), 87);
    assert_eq!(color.b(), 51);
    assert_eq!(color.a(), 0.5);
}

// Test named colors
#[test]
fn test_parse_named_color_red() {
    let result = parse_color_value("red");
    assert!(result.is_ok());
    let color = result.unwrap();
    assert_eq!(color.r(), 255);
    assert_eq!(color.g(), 0);
    assert_eq!(color.b(), 0);
}

#[test]
fn test_parse_named_color_blue() {
    let result = parse_color_value("blue");
    assert!(result.is_ok());
    let color = result.unwrap();
    assert_eq!(color.r(), 0);
    assert_eq!(color.g(), 0);
    assert_eq!(color.b(), 255);
}

#[test]
fn test_parse_named_color_green() {
    let result = parse_color_value("green");
    assert!(result.is_ok());
    let color = result.unwrap();
    assert_eq!(color.r(), 0);
    assert_eq!(color.g(), 128);
    assert_eq!(color.b(), 0);
}

#[test]
fn test_parse_named_color_white() {
    let result = parse_color_value("white");
    assert!(result.is_ok());
    let color = result.unwrap();
    assert_eq!(color.r(), 255);
    assert_eq!(color.g(), 255);
    assert_eq!(color.b(), 255);
}

#[test]
fn test_parse_named_color_black() {
    let result = parse_color_value("black");
    assert!(result.is_ok());
    let color = result.unwrap();
    assert_eq!(color.r(), 0);
    assert_eq!(color.g(), 0);
    assert_eq!(color.b(), 0);
}

// Test HSL colors
#[test]
fn test_parse_hsl_color() {
    let result = parse_color_value("hsl(0, 100%, 50%)");
    assert!(result.is_ok());
    let color = result.unwrap();
    // HSL(0, 100%, 50%) = red
    assert_eq!(color.r(), 255);
    assert_eq!(color.g(), 0);
    assert_eq!(color.b(), 0);
}

#[test]
fn test_parse_hsl_blue() {
    let result = parse_color_value("hsl(240, 100%, 50%)");
    assert!(result.is_ok());
    let color = result.unwrap();
    // HSL(240, 100%, 50%) = blue
    assert_eq!(color.r(), 0);
    assert_eq!(color.g(), 0);
    assert_eq!(color.b(), 255);
}

#[test]
fn test_parse_hsla_color() {
    let result = parse_color_value("hsla(0, 100%, 50%, 0.5)");
    assert!(result.is_ok());
    let color = result.unwrap();
    assert_eq!(color.r(), 255);
    assert_eq!(color.g(), 0);
    assert_eq!(color.b(), 0);
    assert_eq!(color.a(), 0.5);
}

// Test invalid colors
#[test]
fn test_parse_invalid_color() {
    let result = parse_color_value("notacolor");
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_hex_too_short() {
    let result = parse_color_value("#F");
    assert!(result.is_err());
}

#[test]
fn test_parse_rgb_out_of_range() {
    let result = parse_color_value("rgb(256, 0, 0)");
    assert!(result.is_err());
}

#[test]
fn test_parse_hsl_invalid_saturation() {
    let result = parse_color_value("hsl(0, 150%, 50%)");
    assert!(result.is_err());
}
