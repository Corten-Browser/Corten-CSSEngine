use css_custom_properties::*;
use css_types::{Length, LengthUnit};
use std::collections::HashMap;

// Real-world resolver implementation for testing
#[derive(Default)]
struct PropertyStore {
    properties: HashMap<String, String>,
}

impl CustomPropertyResolver for PropertyStore {
    fn set_custom_property(&mut self, name: &str, value: &str) {
        self.properties.insert(name.to_string(), value.to_string());
    }

    fn get_custom_property(&self, name: &str) -> Option<String> {
        self.properties.get(name).cloned()
    }

    fn resolve_var(&self, var_ref: &VariableReference) -> String {
        self.get_custom_property(var_ref.name())
            .or_else(|| var_ref.fallback().map(|s| s.to_string()))
            .unwrap_or_else(|| "initial".to_string())
    }

    fn evaluate_calc(&self, expr: &CalcExpression, context: &CalcContext) -> f32 {
        expr.evaluate(context)
    }
}

#[test]
fn test_full_custom_property_workflow() {
    // Parse a custom property definition
    let prop_def = "--primary-color: #FF5733";
    let prop = parse_custom_property(prop_def).unwrap();

    // Store it in resolver
    let mut store = PropertyStore::default();
    store.set_custom_property(prop.name(), prop.value());

    // Parse a var reference
    let var_ref = parse_var_reference("var(--primary-color)").unwrap();

    // Resolve it
    let value = store.resolve_var(&var_ref);
    assert_eq!(value, "#FF5733");
}

#[test]
fn test_var_with_fallback_chain() {
    let store = PropertyStore::default();

    // Parse var with fallback
    let var_ref = parse_var_reference("var(--undefined, var(--also-undefined, blue))").unwrap();

    // The fallback should be the entire nested var()
    assert_eq!(var_ref.fallback(), Some("var(--also-undefined, blue)"));
}

#[test]
fn test_calc_with_mixed_units() {
    // calc(100% - 20px) with 200px viewport width should be 180px
    let expr_str = "calc(100% - 20px)";
    let expr = parse_calc_expression(expr_str).unwrap();

    let context = CalcContext::new(200.0, 16.0);
    let result = expr.evaluate(&context);

    // 100% of 200px = 200px, minus 20px = 180px
    assert!((result - 180.0).abs() < 0.01);
}

#[test]
fn test_calc_complex_expression() {
    // calc((100% - 40px) / 2)
    let expr_str = "calc((100% - 40px) / 2)";
    let expr = parse_calc_expression(expr_str).unwrap();

    let context = CalcContext::new(200.0, 16.0);
    let result = expr.evaluate(&context);

    // (100% of 200px - 40px) / 2 = (200 - 40) / 2 = 80px
    assert!((result - 80.0).abs() < 0.01);
}

#[test]
fn test_calc_with_em_units() {
    // calc(2em + 10px) with 16px font size
    let left = Box::new(CalcExpression::Value(CalcValue::Length(Length::new(
        2.0,
        LengthUnit::Em,
    ))));
    let right = Box::new(CalcExpression::Value(CalcValue::Length(Length::new(
        10.0,
        LengthUnit::Px,
    ))));
    let expr = CalcExpression::Add(left, right);

    let context = CalcContext::new(100.0, 16.0);
    let result = expr.evaluate(&context);

    // 2em * 16px/em + 10px = 32px + 10px = 42px
    assert!((result - 42.0).abs() < 0.01);
}

#[test]
fn test_property_inheritance() {
    let mut store = PropertyStore::default();

    // Set multiple inherited properties
    store.set_custom_property("--spacing", "10px");
    store.set_custom_property("--primary-color", "#007bff");
    store.set_custom_property("--font-size", "16px");

    // All should be retrievable
    assert_eq!(
        store.get_custom_property("--spacing"),
        Some("10px".to_string())
    );
    assert_eq!(
        store.get_custom_property("--primary-color"),
        Some("#007bff".to_string())
    );
    assert_eq!(
        store.get_custom_property("--font-size"),
        Some("16px".to_string())
    );
}

#[test]
fn test_multiple_custom_properties_and_resolution() {
    let mut store = PropertyStore::default();

    // Define a theme with multiple properties
    let properties = vec![
        "--primary-color: #007bff",
        "--secondary-color: #6c757d",
        "--spacing-small: 5px",
        "--spacing-medium: 10px",
        "--spacing-large: 20px",
        "--font-size-base: 16px",
        "--font-size-large: 20px",
    ];

    // Parse and store all properties
    for prop_str in properties {
        let prop = parse_custom_property(prop_str).unwrap();
        store.set_custom_property(prop.name(), prop.value());
    }

    // Resolve various var references
    let color = store.resolve_var(&VariableReference::new("--primary-color"));
    assert_eq!(color, "#007bff");

    let spacing = store.resolve_var(&VariableReference::new("--spacing-medium"));
    assert_eq!(spacing, "10px");

    let font = store.resolve_var(&VariableReference::new("--font-size-large"));
    assert_eq!(font, "20px");

    // Test undefined with fallback
    let undefined = store.resolve_var(&VariableReference::with_fallback("--undefined", "red"));
    assert_eq!(undefined, "red");
}

#[test]
fn test_calc_division_by_zero() {
    // calc(100px / 0) should return 0 to avoid panics
    let value = Box::new(CalcExpression::Value(CalcValue::Length(Length::new(
        100.0,
        LengthUnit::Px,
    ))));
    let expr = CalcExpression::Divide(value, 0.0);

    let context = CalcContext::new(100.0, 16.0);
    let result = expr.evaluate(&context);
    assert_eq!(result, 0.0);
}

#[test]
fn test_parse_and_evaluate_percentage_calc() {
    // calc(50% + 25%) with 100px viewport
    let left = Box::new(CalcExpression::Value(CalcValue::Percentage(50.0)));
    let right = Box::new(CalcExpression::Value(CalcValue::Percentage(25.0)));
    let expr = CalcExpression::Add(left, right);

    let context = CalcContext::new(100.0, 16.0);
    let result = expr.evaluate(&context);

    // 50% of 100px + 25% of 100px = 50px + 25px = 75px
    assert!((result - 75.0).abs() < 0.01);
}

#[test]
fn test_real_world_layout_calc() {
    // Common CSS pattern: calc(100vw - 2 * 20px) for side margins
    // Parsing: calc(100% - 40px) where 100% represents viewport width

    let expr_str = "calc(100% - 40px)";
    let expr = parse_calc_expression(expr_str).unwrap();

    // Viewport width is 1024px
    let context = CalcContext::new(1024.0, 16.0);
    let result = expr.evaluate(&context);

    // 1024px - 40px = 984px
    assert!((result - 984.0).abs() < 0.01);
}

#[test]
fn test_custom_property_update() {
    let mut store = PropertyStore::default();

    // Set initial value
    store.set_custom_property("--color", "red");
    assert_eq!(
        store.get_custom_property("--color"),
        Some("red".to_string())
    );

    // Update to new value
    store.set_custom_property("--color", "blue");
    assert_eq!(
        store.get_custom_property("--color"),
        Some("blue".to_string())
    );

    // Verify old value is gone
    let var_ref = VariableReference::new("--color");
    assert_eq!(store.resolve_var(&var_ref), "blue");
}

#[test]
fn test_parse_complex_value_with_multiple_parts() {
    // Custom properties can have complex values like box-shadow
    let prop_str = "--box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1), 0 2px 4px rgba(0, 0, 0, 0.06)";
    let prop = parse_custom_property(prop_str).unwrap();

    assert_eq!(prop.name(), "--box-shadow");
    assert_eq!(
        prop.value(),
        "0 4px 6px rgba(0, 0, 0, 0.1), 0 2px 4px rgba(0, 0, 0, 0.06)"
    );
}
