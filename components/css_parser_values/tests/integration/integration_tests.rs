// Integration tests for css_parser_values

use css_parser_values::{
    parse_attribute_selector, parse_color_value, parse_function_value, parse_value, ValueKind,
};

#[test]
fn test_complete_attribute_selector_workflow() {
    // Test parsing a complete selector with all features
    let selector = parse_attribute_selector("[data-theme=\"dark\" i]").unwrap();
    assert_eq!(selector.name(), "data-theme");
    assert_eq!(selector.value(), Some("dark"));
}

#[test]
fn test_complete_color_parsing_workflow() {
    // Test all color formats
    let hex = parse_color_value("#FF5733").unwrap();
    let rgb = parse_color_value("rgb(255, 87, 51)").unwrap();
    let hsl = parse_color_value("hsl(10, 100%, 60%)").unwrap();
    let named = parse_color_value("red").unwrap();

    // All should be valid colors
    assert_eq!(hex.r(), 255);
    assert_eq!(rgb.r(), 255);
    assert!(hsl.r() > 250); // HSL approximation
    assert_eq!(named.r(), 255);
}

#[test]
fn test_function_value_parsing_workflow() {
    // Test various CSS functions
    let url = parse_function_value("url(\"image.png\")").unwrap();
    assert_eq!(url.name(), "url");

    let calc = parse_function_value("calc(100% - 50px)").unwrap();
    assert_eq!(calc.name(), "calc");

    let var = parse_function_value("var(--main-color, red)").unwrap();
    assert_eq!(var.name(), "var");
    assert_eq!(var.args().len(), 2);
}

#[test]
fn test_complex_value_type_detection() {
    // Test that parse_value correctly identifies different value types
    assert_eq!(
        parse_value("10px", "margin").unwrap().kind(),
        ValueKind::Length
    );
    assert_eq!(
        parse_value("50%", "width").unwrap().kind(),
        ValueKind::Percentage
    );
    assert_eq!(
        parse_value("42", "line-height").unwrap().kind(),
        ValueKind::Number
    );
    assert_eq!(
        parse_value("#FF0000", "color").unwrap().kind(),
        ValueKind::Color
    );
    assert_eq!(
        parse_value("url(\"test.png\")", "background").unwrap().kind(),
        ValueKind::Url
    );
    assert_eq!(
        parse_value("auto", "margin").unwrap().kind(),
        ValueKind::Keyword
    );
}

#[test]
fn test_error_handling_workflow() {
    // Test that errors are properly propagated
    assert!(parse_attribute_selector("notvalid").is_err());
    assert!(parse_color_value("notacolor").is_err());
    assert!(parse_function_value("notafunction").is_err());
    assert!(parse_value("", "color").is_err());
}

#[test]
fn test_real_world_css_values() {
    // Test parsing values that would appear in real CSS
    let gradient = parse_function_value("linear-gradient(to right, #FF0000, #0000FF)").unwrap();
    assert_eq!(gradient.name(), "linear-gradient");

    let box_shadow = parse_value("0 4px 6px rgba(0,0,0,0.1)", "box-shadow");
    // This would be parsed as a keyword since it has spaces
    assert!(box_shadow.is_ok());

    let transform = parse_function_value("rotate(45deg)").unwrap();
    assert_eq!(transform.name(), "rotate");
}
