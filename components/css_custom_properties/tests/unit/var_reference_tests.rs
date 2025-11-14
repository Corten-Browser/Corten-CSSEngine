use css_custom_properties::*;

#[test]
fn test_parse_var_reference_simple() {
    let result = parse_var_reference("var(--primary-color)");
    assert!(result.is_ok());
    let var_ref = result.unwrap();
    assert_eq!(var_ref.name(), "--primary-color");
    assert!(var_ref.fallback().is_none());
}

#[test]
fn test_parse_var_reference_with_fallback() {
    let result = parse_var_reference("var(--primary-color, #FF5733)");
    assert!(result.is_ok());
    let var_ref = result.unwrap();
    assert_eq!(var_ref.name(), "--primary-color");
    assert_eq!(var_ref.fallback(), Some("#FF5733"));
}

#[test]
fn test_parse_var_reference_with_whitespace() {
    let result = parse_var_reference("var(  --margin  ,  20px  )");
    assert!(result.is_ok());
    let var_ref = result.unwrap();
    assert_eq!(var_ref.name(), "--margin");
    assert_eq!(var_ref.fallback(), Some("20px"));
}

#[test]
fn test_parse_var_reference_nested_fallback() {
    // Fallback can contain another var()
    let result = parse_var_reference("var(--color-1, var(--color-2, blue))");
    assert!(result.is_ok());
    let var_ref = result.unwrap();
    assert_eq!(var_ref.name(), "--color-1");
    assert_eq!(var_ref.fallback(), Some("var(--color-2, blue)"));
}

#[test]
fn test_parse_var_reference_invalid_syntax() {
    // Missing var prefix
    let result = parse_var_reference("(--color)");
    assert!(result.is_err());

    // Missing parentheses
    let result = parse_var_reference("var --color");
    assert!(result.is_err());

    // Missing closing paren
    let result = parse_var_reference("var(--color");
    assert!(result.is_err());
}

#[test]
fn test_variable_reference_creation() {
    let var_ref = VariableReference::new("--color");
    assert_eq!(var_ref.name(), "--color");
    assert!(var_ref.fallback().is_none());

    let var_ref = VariableReference::with_fallback("--size", "10px");
    assert_eq!(var_ref.name(), "--size");
    assert_eq!(var_ref.fallback(), Some("10px"));
}
