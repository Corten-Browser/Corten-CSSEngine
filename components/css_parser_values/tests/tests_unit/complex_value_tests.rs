// Unit tests for ComplexValue parsing

use css_parser_values::{parse_value, ValueKind};

#[test]
fn test_parse_number_value() {
    let result = parse_value("42", "line-height");
    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value.kind(), ValueKind::Number);
}

#[test]
fn test_parse_percentage_value() {
    let result = parse_value("50%", "width");
    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value.kind(), ValueKind::Percentage);
}

#[test]
fn test_parse_length_value() {
    let result = parse_value("10px", "margin");
    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value.kind(), ValueKind::Length);
}

#[test]
fn test_parse_color_value() {
    let result = parse_value("#FF0000", "color");
    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value.kind(), ValueKind::Color);
}

#[test]
fn test_parse_string_value() {
    let result = parse_value("\"hello world\"", "content");
    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value.kind(), ValueKind::String);
}

#[test]
fn test_parse_url_value() {
    let result = parse_value("url(\"image.png\")", "background-image");
    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value.kind(), ValueKind::Url);
}

#[test]
fn test_parse_function_value() {
    let result = parse_value("calc(100% - 50px)", "width");
    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value.kind(), ValueKind::Function);
}

#[test]
fn test_parse_keyword_value() {
    let result = parse_value("auto", "margin");
    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value.kind(), ValueKind::Keyword);
}

#[test]
fn test_parse_inherit_keyword() {
    let result = parse_value("inherit", "color");
    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value.kind(), ValueKind::Keyword);
}

#[test]
fn test_parse_initial_keyword() {
    let result = parse_value("initial", "margin");
    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value.kind(), ValueKind::Keyword);
}

#[test]
fn test_parse_invalid_value() {
    let result = parse_value("", "color");
    assert!(result.is_err());
}
