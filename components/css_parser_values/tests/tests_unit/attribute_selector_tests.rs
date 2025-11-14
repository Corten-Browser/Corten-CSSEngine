// Unit tests for AttributeSelector parsing

use css_parser_values::{
    parse_attribute_selector, AttributeOperator, CaseSensitivity,
};

#[test]
fn test_parse_attribute_exists() {
    let result = parse_attribute_selector("[attr]");
    assert!(result.is_ok());
    let selector = result.unwrap();
    assert_eq!(selector.name(), "attr");
    assert_eq!(selector.operator(), AttributeOperator::Exists);
    assert_eq!(selector.value(), None);
}

#[test]
fn test_parse_attribute_equals() {
    let result = parse_attribute_selector("[attr=value]");
    assert!(result.is_ok());
    let selector = result.unwrap();
    assert_eq!(selector.name(), "attr");
    assert_eq!(selector.operator(), AttributeOperator::Equals);
    assert_eq!(selector.value(), Some("value"));
}

#[test]
fn test_parse_attribute_equals_with_quotes() {
    let result = parse_attribute_selector("[attr=\"value\"]");
    assert!(result.is_ok());
    let selector = result.unwrap();
    assert_eq!(selector.name(), "attr");
    assert_eq!(selector.operator(), AttributeOperator::Equals);
    assert_eq!(selector.value(), Some("value"));
}

#[test]
fn test_parse_attribute_includes() {
    let result = parse_attribute_selector("[attr~=value]");
    assert!(result.is_ok());
    let selector = result.unwrap();
    assert_eq!(selector.name(), "attr");
    assert_eq!(selector.operator(), AttributeOperator::Includes);
    assert_eq!(selector.value(), Some("value"));
}

#[test]
fn test_parse_attribute_dash_match() {
    let result = parse_attribute_selector("[attr|=value]");
    assert!(result.is_ok());
    let selector = result.unwrap();
    assert_eq!(selector.name(), "attr");
    assert_eq!(selector.operator(), AttributeOperator::DashMatch);
    assert_eq!(selector.value(), Some("value"));
}

#[test]
fn test_parse_attribute_prefix() {
    let result = parse_attribute_selector("[attr^=value]");
    assert!(result.is_ok());
    let selector = result.unwrap();
    assert_eq!(selector.name(), "attr");
    assert_eq!(selector.operator(), AttributeOperator::Prefix);
    assert_eq!(selector.value(), Some("value"));
}

#[test]
fn test_parse_attribute_suffix() {
    let result = parse_attribute_selector("[attr$=value]");
    assert!(result.is_ok());
    let selector = result.unwrap();
    assert_eq!(selector.name(), "attr");
    assert_eq!(selector.operator(), AttributeOperator::Suffix);
    assert_eq!(selector.value(), Some("value"));
}

#[test]
fn test_parse_attribute_substring() {
    let result = parse_attribute_selector("[attr*=value]");
    assert!(result.is_ok());
    let selector = result.unwrap();
    assert_eq!(selector.name(), "attr");
    assert_eq!(selector.operator(), AttributeOperator::Substring);
    assert_eq!(selector.value(), Some("value"));
}

#[test]
fn test_parse_attribute_with_namespace() {
    let result = parse_attribute_selector("[ns|attr=value]");
    assert!(result.is_ok());
    let selector = result.unwrap();
    assert_eq!(selector.name(), "attr");
    assert_eq!(selector.namespace(), Some("ns"));
    assert_eq!(selector.operator(), AttributeOperator::Equals);
}

#[test]
fn test_parse_attribute_case_insensitive() {
    let result = parse_attribute_selector("[attr=value i]");
    assert!(result.is_ok());
    let selector = result.unwrap();
    assert_eq!(selector.name(), "attr");
    assert_eq!(
        selector.case_sensitivity(),
        CaseSensitivity::AsciiCaseInsensitive
    );
}

#[test]
fn test_parse_attribute_case_sensitive() {
    let result = parse_attribute_selector("[attr=value s]");
    assert!(result.is_ok());
    let selector = result.unwrap();
    assert_eq!(selector.case_sensitivity(), CaseSensitivity::CaseSensitive);
}

#[test]
fn test_parse_attribute_invalid_no_brackets() {
    let result = parse_attribute_selector("attr");
    assert!(result.is_err());
}

#[test]
fn test_parse_attribute_invalid_empty() {
    let result = parse_attribute_selector("[]");
    assert!(result.is_err());
}

#[test]
fn test_parse_attribute_with_whitespace() {
    let result = parse_attribute_selector("[ attr = value ]");
    assert!(result.is_ok());
    let selector = result.unwrap();
    assert_eq!(selector.name(), "attr");
    assert_eq!(selector.value(), Some("value"));
}
