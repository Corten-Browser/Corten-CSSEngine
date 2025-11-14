use css_custom_properties::*;
use css_types::{Length, LengthUnit};

#[test]
fn test_parse_calc_simple_addition() {
    let result = parse_calc_expression("calc(10px + 20px)");
    assert!(result.is_ok());
}

#[test]
fn test_parse_calc_simple_subtraction() {
    let result = parse_calc_expression("calc(100% - 20px)");
    assert!(result.is_ok());
}

#[test]
fn test_parse_calc_multiplication() {
    let result = parse_calc_expression("calc(10px * 2)");
    assert!(result.is_ok());
}

#[test]
fn test_parse_calc_division() {
    let result = parse_calc_expression("calc(100px / 4)");
    assert!(result.is_ok());
}

#[test]
fn test_parse_calc_with_whitespace() {
    let result = parse_calc_expression("calc(  100%  -  20px  )");
    assert!(result.is_ok());
}

#[test]
fn test_parse_calc_nested() {
    let result = parse_calc_expression("calc((100% - 20px) / 2)");
    assert!(result.is_ok());
}

#[test]
fn test_parse_calc_mixed_units() {
    let result = parse_calc_expression("calc(100% - 20px)");
    assert!(result.is_ok());
}

#[test]
fn test_parse_calc_percentage() {
    let result = parse_calc_expression("calc(50% + 10px)");
    assert!(result.is_ok());
}

#[test]
fn test_parse_calc_number() {
    let result = parse_calc_expression("calc(2 * 10px)");
    assert!(result.is_ok());
}

#[test]
fn test_parse_calc_invalid_syntax() {
    // Missing calc prefix
    let result = parse_calc_expression("(10px + 20px)");
    assert!(result.is_err());

    // Missing parentheses
    let result = parse_calc_expression("calc 10px + 20px");
    assert!(result.is_err());

    // Invalid operator
    let result = parse_calc_expression("calc(10px % 20px)");
    assert!(result.is_err());
}

#[test]
fn test_calc_value_creation() {
    let val = CalcValue::Number(2.0);
    assert!(matches!(val, CalcValue::Number(2.0)));

    let val = CalcValue::Length(Length::new(10.0, LengthUnit::Px));
    assert!(matches!(val, CalcValue::Length(_)));

    let val = CalcValue::Percentage(50.0);
    assert!(matches!(val, CalcValue::Percentage(50.0)));
}

#[test]
fn test_calc_expression_creation() {
    let expr = CalcExpression::Value(CalcValue::Number(10.0));
    assert!(matches!(expr, CalcExpression::Value(_)));

    let left = Box::new(CalcExpression::Value(CalcValue::Number(10.0)));
    let right = Box::new(CalcExpression::Value(CalcValue::Number(20.0)));
    let expr = CalcExpression::Add(left, right);
    assert!(matches!(expr, CalcExpression::Add(_, _)));
}

#[test]
fn test_evaluate_calc_simple() {
    // calc(10px + 20px) = 30px
    let left = Box::new(CalcExpression::Value(CalcValue::Length(Length::new(
        10.0,
        LengthUnit::Px,
    ))));
    let right = Box::new(CalcExpression::Value(CalcValue::Length(Length::new(
        20.0,
        LengthUnit::Px,
    ))));
    let expr = CalcExpression::Add(left, right);

    let context = CalcContext::new(100.0, 16.0); // viewport_width=100, font_size=16
    let result = expr.evaluate(&context);
    assert!((result - 30.0).abs() < 0.01);
}

#[test]
fn test_evaluate_calc_multiplication() {
    // calc(10px * 2) = 20px
    let value = Box::new(CalcExpression::Value(CalcValue::Length(Length::new(
        10.0,
        LengthUnit::Px,
    ))));
    let expr = CalcExpression::Multiply(value, 2.0);

    let context = CalcContext::new(100.0, 16.0);
    let result = expr.evaluate(&context);
    assert!((result - 20.0).abs() < 0.01);
}

#[test]
fn test_evaluate_calc_division() {
    // calc(100px / 4) = 25px
    let value = Box::new(CalcExpression::Value(CalcValue::Length(Length::new(
        100.0,
        LengthUnit::Px,
    ))));
    let expr = CalcExpression::Divide(value, 4.0);

    let context = CalcContext::new(100.0, 16.0);
    let result = expr.evaluate(&context);
    assert!((result - 25.0).abs() < 0.01);
}

#[test]
fn test_evaluate_calc_percentage() {
    // calc(50% + 10px) with viewport_width=100 = 50 + 10 = 60px
    let left = Box::new(CalcExpression::Value(CalcValue::Percentage(50.0)));
    let right = Box::new(CalcExpression::Value(CalcValue::Length(Length::new(
        10.0,
        LengthUnit::Px,
    ))));
    let expr = CalcExpression::Add(left, right);

    let context = CalcContext::new(100.0, 16.0);
    let result = expr.evaluate(&context);
    assert!((result - 60.0).abs() < 0.01);
}
