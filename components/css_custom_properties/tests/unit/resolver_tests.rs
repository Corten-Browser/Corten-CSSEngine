use css_custom_properties::*;

// Simple resolver implementation for testing
#[derive(Default)]
struct TestResolver {
    properties: std::collections::HashMap<String, String>,
}

impl CustomPropertyResolver for TestResolver {
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
fn test_resolver_set_and_get() {
    let mut resolver = TestResolver::default();
    resolver.set_custom_property("--primary-color", "#FF5733");

    let value = resolver.get_custom_property("--primary-color");
    assert_eq!(value, Some("#FF5733".to_string()));
}

#[test]
fn test_resolver_get_nonexistent() {
    let resolver = TestResolver::default();
    let value = resolver.get_custom_property("--nonexistent");
    assert!(value.is_none());
}

#[test]
fn test_resolve_var_simple() {
    let mut resolver = TestResolver::default();
    resolver.set_custom_property("--color", "blue");

    let var_ref = VariableReference::new("--color");
    let result = resolver.resolve_var(&var_ref);
    assert_eq!(result, "blue");
}

#[test]
fn test_resolve_var_with_fallback() {
    let resolver = TestResolver::default();
    // Property doesn't exist, should use fallback
    let var_ref = VariableReference::with_fallback("--color", "red");
    let result = resolver.resolve_var(&var_ref);
    assert_eq!(result, "red");
}

#[test]
fn test_resolve_var_no_fallback() {
    let resolver = TestResolver::default();
    // Property doesn't exist, no fallback, should return "initial"
    let var_ref = VariableReference::new("--color");
    let result = resolver.resolve_var(&var_ref);
    assert_eq!(result, "initial");
}

#[test]
fn test_evaluate_calc_via_resolver() {
    let resolver = TestResolver::default();

    let left = Box::new(CalcExpression::Value(CalcValue::Length(
        css_types::Length::new(10.0, css_types::LengthUnit::Px),
    )));
    let right = Box::new(CalcExpression::Value(CalcValue::Length(
        css_types::Length::new(20.0, css_types::LengthUnit::Px),
    )));
    let expr = CalcExpression::Add(left, right);

    let context = CalcContext::new(100.0, 16.0);
    let result = resolver.evaluate_calc(&expr, &context);
    assert!((result - 30.0).abs() < 0.01);
}

#[test]
fn test_multiple_properties() {
    let mut resolver = TestResolver::default();
    resolver.set_custom_property("--color", "red");
    resolver.set_custom_property("--size", "20px");
    resolver.set_custom_property("--margin", "10px");

    assert_eq!(
        resolver.get_custom_property("--color"),
        Some("red".to_string())
    );
    assert_eq!(
        resolver.get_custom_property("--size"),
        Some("20px".to_string())
    );
    assert_eq!(
        resolver.get_custom_property("--margin"),
        Some("10px".to_string())
    );
}

#[test]
fn test_property_override() {
    let mut resolver = TestResolver::default();
    resolver.set_custom_property("--color", "red");
    assert_eq!(
        resolver.get_custom_property("--color"),
        Some("red".to_string())
    );

    // Override with new value
    resolver.set_custom_property("--color", "blue");
    assert_eq!(
        resolver.get_custom_property("--color"),
        Some("blue".to_string())
    );
}
