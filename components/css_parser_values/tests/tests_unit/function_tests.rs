// Unit tests for CSS function parsing

use css_parser_values::parse_function_value;

#[test]
fn test_parse_url_function() {
    let result = parse_function_value("url(\"https://example.com/image.png\")");
    assert!(result.is_ok());
    let func = result.unwrap();
    assert_eq!(func.name(), "url");
    assert_eq!(func.args().len(), 1);
    assert_eq!(func.args()[0], "https://example.com/image.png");
}

#[test]
fn test_parse_url_function_without_quotes() {
    let result = parse_function_value("url(https://example.com/image.png)");
    assert!(result.is_ok());
    let func = result.unwrap();
    assert_eq!(func.name(), "url");
    assert_eq!(func.args()[0], "https://example.com/image.png");
}

#[test]
fn test_parse_calc_function() {
    let result = parse_function_value("calc(100% - 50px)");
    assert!(result.is_ok());
    let func = result.unwrap();
    assert_eq!(func.name(), "calc");
    assert_eq!(func.args().len(), 1);
    assert_eq!(func.args()[0], "100% - 50px");
}

#[test]
fn test_parse_var_function() {
    let result = parse_function_value("var(--main-color)");
    assert!(result.is_ok());
    let func = result.unwrap();
    assert_eq!(func.name(), "var");
    assert_eq!(func.args()[0], "--main-color");
}

#[test]
fn test_parse_var_function_with_fallback() {
    let result = parse_function_value("var(--main-color, red)");
    assert!(result.is_ok());
    let func = result.unwrap();
    assert_eq!(func.name(), "var");
    assert_eq!(func.args().len(), 2);
    assert_eq!(func.args()[0], "--main-color");
    assert_eq!(func.args()[1], "red");
}

#[test]
fn test_parse_linear_gradient() {
    let result = parse_function_value("linear-gradient(to right, red, blue)");
    assert!(result.is_ok());
    let func = result.unwrap();
    assert_eq!(func.name(), "linear-gradient");
    assert!(func.args().len() >= 2);
}

#[test]
fn test_parse_radial_gradient() {
    let result = parse_function_value("radial-gradient(circle, red, blue)");
    assert!(result.is_ok());
    let func = result.unwrap();
    assert_eq!(func.name(), "radial-gradient");
    assert!(func.args().len() >= 2);
}

#[test]
fn test_parse_rgb_function() {
    let result = parse_function_value("rgb(255, 0, 0)");
    assert!(result.is_ok());
    let func = result.unwrap();
    assert_eq!(func.name(), "rgb");
    assert_eq!(func.args().len(), 3);
}

#[test]
fn test_parse_rgba_function() {
    let result = parse_function_value("rgba(255, 0, 0, 0.5)");
    assert!(result.is_ok());
    let func = result.unwrap();
    assert_eq!(func.name(), "rgba");
    assert_eq!(func.args().len(), 4);
}

#[test]
fn test_parse_function_invalid_no_parentheses() {
    let result = parse_function_value("notafunction");
    assert!(result.is_err());
}

#[test]
fn test_parse_function_invalid_missing_closing_paren() {
    let result = parse_function_value("calc(100%");
    assert!(result.is_err());
}

#[test]
fn test_parse_function_empty_args() {
    let result = parse_function_value("func()");
    assert!(result.is_ok());
    let func = result.unwrap();
    assert_eq!(func.name(), "func");
    assert!(func.args().is_empty() || func.args()[0].is_empty());
}
