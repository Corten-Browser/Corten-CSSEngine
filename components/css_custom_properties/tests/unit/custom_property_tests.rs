use css_custom_properties::*;

#[test]
fn test_parse_custom_property_simple() {
    let result = parse_custom_property("--primary-color: #FF5733");
    assert!(result.is_ok());
    let prop = result.unwrap();
    assert_eq!(prop.name(), "--primary-color");
    assert_eq!(prop.value(), "#FF5733");
    assert!(prop.inherited()); // Custom properties inherit by default
}

#[test]
fn test_parse_custom_property_with_whitespace() {
    let result = parse_custom_property("  --margin-top  :  20px  ");
    assert!(result.is_ok());
    let prop = result.unwrap();
    assert_eq!(prop.name(), "--margin-top");
    assert_eq!(prop.value(), "20px");
}

#[test]
fn test_parse_custom_property_complex_value() {
    let result = parse_custom_property("--box-shadow: 0 2px 4px rgba(0,0,0,0.1)");
    assert!(result.is_ok());
    let prop = result.unwrap();
    assert_eq!(prop.name(), "--box-shadow");
    assert_eq!(prop.value(), "0 2px 4px rgba(0,0,0,0.1)");
}

#[test]
fn test_parse_custom_property_invalid_name() {
    // Custom property must start with --
    let result = parse_custom_property("primary-color: red");
    assert!(result.is_err());
}

#[test]
fn test_parse_custom_property_missing_colon() {
    let result = parse_custom_property("--primary-color #FF5733");
    assert!(result.is_err());
}

#[test]
fn test_custom_property_creation() {
    let prop = CustomProperty::new("--color", "blue", true);
    assert_eq!(prop.name(), "--color");
    assert_eq!(prop.value(), "blue");
    assert!(prop.inherited());
}
