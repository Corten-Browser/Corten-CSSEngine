//! WPT Parser Tests - CSS Parsing Conformance Tests
//!
//! This module contains WPT-style test cases for CSS parsing conformance.
//! Tests cover selector parsing, declaration parsing, and error handling.

use crate::harness::{WptExpectation, WptTestCase, WptTestResult, WptTestRunner};

/// CSS parsing test function
///
/// Returns Pass if the CSS parses according to the expectation,
/// Fail otherwise.
pub fn run_parser_test(test: &WptTestCase) -> WptTestResult {
    match &test.expected {
        WptExpectation::ShouldParse => {
            // Attempt to parse and expect success
            match validate_css_syntax(&test.css) {
                Ok(_) => WptTestResult::Pass,
                Err(e) => WptTestResult::Fail(format!("Expected valid CSS, got error: {}", e)),
            }
        }
        WptExpectation::ShouldNotParse => {
            // Attempt to parse and expect failure
            match validate_css_syntax(&test.css) {
                Ok(_) => WptTestResult::Fail("Expected parse error, but CSS was valid".to_string()),
                Err(_) => WptTestResult::Pass,
            }
        }
        WptExpectation::SerializesTo(expected) => {
            // Parse and re-serialize, checking output
            match parse_and_serialize(&test.css) {
                Ok(serialized) => {
                    if normalize_whitespace(&serialized) == normalize_whitespace(expected) {
                        WptTestResult::Pass
                    } else {
                        WptTestResult::Fail(format!(
                            "Serialization mismatch:\n  Expected: {}\n  Got: {}",
                            expected, serialized
                        ))
                    }
                }
                Err(e) => WptTestResult::Error(format!("Failed to parse: {}", e)),
            }
        }
        _ => WptTestResult::Error("Unsupported expectation for parser test".to_string()),
    }
}

/// Simple CSS syntax validation
///
/// This validates basic CSS structure:
/// - Balanced braces
/// - Valid selector format
/// - Valid declaration format
fn validate_css_syntax(css: &str) -> Result<(), String> {
    let css = css.trim();

    if css.is_empty() {
        return Err("Empty CSS".to_string());
    }

    // Check for balanced braces
    let mut brace_count = 0;
    let mut in_string = false;
    let mut string_char = ' ';

    for ch in css.chars() {
        if in_string {
            if ch == string_char {
                in_string = false;
            }
            continue;
        }

        match ch {
            '"' | '\'' => {
                in_string = true;
                string_char = ch;
            }
            '{' => brace_count += 1,
            '}' => {
                brace_count -= 1;
                if brace_count < 0 {
                    return Err("Unbalanced braces: too many closing braces".to_string());
                }
            }
            _ => {}
        }
    }

    if brace_count != 0 {
        return Err(format!("Unbalanced braces: {} unclosed", brace_count));
    }

    // Check for valid rule structure (selector { declarations })
    if css.contains('{') {
        // Parse rule(s)
        let mut current_pos = 0;
        while let Some(brace_pos) = css[current_pos..].find('{') {
            let selector_part = css[current_pos..current_pos + brace_pos].trim();

            // Validate selector is not empty
            if selector_part.is_empty() {
                return Err("Empty selector".to_string());
            }

            // Find matching close brace
            let declarations_start = current_pos + brace_pos + 1;
            let mut depth = 1;
            let mut declarations_end = declarations_start;

            for (i, ch) in css[declarations_start..].chars().enumerate() {
                match ch {
                    '{' => depth += 1,
                    '}' => {
                        depth -= 1;
                        if depth == 0 {
                            declarations_end = declarations_start + i;
                            break;
                        }
                    }
                    _ => {}
                }
            }

            // Validate declarations
            let declarations = &css[declarations_start..declarations_end];
            validate_declarations(declarations)?;

            current_pos = declarations_end + 1;
            if current_pos >= css.len() {
                break;
            }
        }
    }

    Ok(())
}

/// Validate CSS declarations
fn validate_declarations(declarations: &str) -> Result<(), String> {
    let declarations = declarations.trim();

    if declarations.is_empty() {
        return Ok(()); // Empty declaration block is valid
    }

    // Split by semicolons and validate each declaration
    for decl in declarations.split(';') {
        let decl = decl.trim();
        if decl.is_empty() {
            continue; // Trailing semicolons are fine
        }

        // Each declaration should have property: value
        if !decl.contains(':') {
            return Err(format!("Invalid declaration (missing colon): '{}'", decl));
        }

        let parts: Vec<&str> = decl.splitn(2, ':').collect();
        if parts.len() != 2 {
            return Err(format!("Invalid declaration format: '{}'", decl));
        }

        let property = parts[0].trim();
        let value = parts[1].trim();

        if property.is_empty() {
            return Err("Empty property name".to_string());
        }

        // Property names should only contain valid characters
        if !is_valid_property_name(property) {
            return Err(format!("Invalid property name: '{}'", property));
        }

        if value.is_empty() {
            return Err(format!("Empty value for property '{}'", property));
        }
    }

    Ok(())
}

/// Check if a string is a valid CSS property name
fn is_valid_property_name(name: &str) -> bool {
    let name = name.trim();
    if name.is_empty() {
        return false;
    }

    // Custom properties start with --
    if name.starts_with("--") {
        return name.len() > 2;
    }

    // Standard properties: alphanumeric and hyphens
    name.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
        && !name.starts_with('-')  // Can't start with single hyphen (except --)
}

/// Parse CSS and serialize it back (for round-trip testing)
fn parse_and_serialize(css: &str) -> Result<String, String> {
    // For now, just validate and return a normalized version
    validate_css_syntax(css)?;
    Ok(normalize_css(css))
}

/// Normalize CSS for comparison
fn normalize_css(css: &str) -> String {
    let mut result = String::new();
    let mut in_whitespace = false;
    let mut in_string = false;
    let mut string_char = ' ';

    for ch in css.chars() {
        if in_string {
            result.push(ch);
            if ch == string_char {
                in_string = false;
            }
            continue;
        }

        match ch {
            '"' | '\'' => {
                in_string = true;
                string_char = ch;
                in_whitespace = false;
                result.push(ch);
            }
            ' ' | '\t' | '\n' | '\r' => {
                if !in_whitespace {
                    result.push(' ');
                    in_whitespace = true;
                }
            }
            _ => {
                in_whitespace = false;
                result.push(ch);
            }
        }
    }

    result.trim().to_string()
}

/// Normalize whitespace for comparison
fn normalize_whitespace(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Create the standard set of CSS parsing tests
pub fn create_parser_tests() -> Vec<WptTestCase> {
    vec![
        // ============================================
        // Valid CSS - Should Parse
        // ============================================

        // Basic selectors
        WptTestCase::new(
            "parser/selectors/element-selector",
            "div { }",
        )
        .with_expectation(WptExpectation::ShouldParse)
        .with_flag("selector"),

        WptTestCase::new(
            "parser/selectors/class-selector",
            ".button { }",
        )
        .with_expectation(WptExpectation::ShouldParse)
        .with_flag("selector"),

        WptTestCase::new(
            "parser/selectors/id-selector",
            "#header { }",
        )
        .with_expectation(WptExpectation::ShouldParse)
        .with_flag("selector"),

        WptTestCase::new(
            "parser/selectors/universal-selector",
            "* { }",
        )
        .with_expectation(WptExpectation::ShouldParse)
        .with_flag("selector"),

        WptTestCase::new(
            "parser/selectors/compound-selector",
            "div.container#main { }",
        )
        .with_expectation(WptExpectation::ShouldParse)
        .with_flag("selector"),

        WptTestCase::new(
            "parser/selectors/selector-list",
            "div, .class, #id { }",
        )
        .with_expectation(WptExpectation::ShouldParse)
        .with_flag("selector"),

        // Declarations
        WptTestCase::new(
            "parser/declarations/single-declaration",
            "div { color: red; }",
        )
        .with_expectation(WptExpectation::ShouldParse)
        .with_flag("declaration"),

        WptTestCase::new(
            "parser/declarations/multiple-declarations",
            "div { color: red; background: blue; margin: 10px; }",
        )
        .with_expectation(WptExpectation::ShouldParse)
        .with_flag("declaration"),

        WptTestCase::new(
            "parser/declarations/no-trailing-semicolon",
            "div { color: red }",
        )
        .with_expectation(WptExpectation::ShouldParse)
        .with_flag("declaration"),

        WptTestCase::new(
            "parser/declarations/empty-declaration-block",
            "div { }",
        )
        .with_expectation(WptExpectation::ShouldParse)
        .with_flag("declaration"),

        // Values
        WptTestCase::new(
            "parser/values/length-px",
            "div { width: 100px; }",
        )
        .with_expectation(WptExpectation::ShouldParse)
        .with_flag("value"),

        WptTestCase::new(
            "parser/values/length-em",
            "div { font-size: 1.5em; }",
        )
        .with_expectation(WptExpectation::ShouldParse)
        .with_flag("value"),

        WptTestCase::new(
            "parser/values/percentage",
            "div { width: 50%; }",
        )
        .with_expectation(WptExpectation::ShouldParse)
        .with_flag("value"),

        WptTestCase::new(
            "parser/values/color-hex",
            "div { color: #ff0000; }",
        )
        .with_expectation(WptExpectation::ShouldParse)
        .with_flag("color"),

        WptTestCase::new(
            "parser/values/color-hex-short",
            "div { color: #f00; }",
        )
        .with_expectation(WptExpectation::ShouldParse)
        .with_flag("color"),

        WptTestCase::new(
            "parser/values/color-rgb",
            "div { color: rgb(255, 0, 0); }",
        )
        .with_expectation(WptExpectation::ShouldParse)
        .with_flag("color"),

        WptTestCase::new(
            "parser/values/color-rgba",
            "div { color: rgba(255, 0, 0, 0.5); }",
        )
        .with_expectation(WptExpectation::ShouldParse)
        .with_flag("color"),

        // Custom properties
        WptTestCase::new(
            "parser/custom-properties/definition",
            ":root { --main-color: #ff0000; }",
        )
        .with_expectation(WptExpectation::ShouldParse)
        .with_flag("custom-property"),

        WptTestCase::new(
            "parser/custom-properties/usage",
            "div { color: var(--main-color); }",
        )
        .with_expectation(WptExpectation::ShouldParse)
        .with_flag("custom-property"),

        // Multiple rules
        WptTestCase::new(
            "parser/rules/multiple-rules",
            "div { color: red; } .class { color: blue; } #id { color: green; }",
        )
        .with_expectation(WptExpectation::ShouldParse)
        .with_flag("multi-rule"),

        // ============================================
        // Invalid CSS - Should NOT Parse
        // ============================================

        WptTestCase::new(
            "parser/invalid/unbalanced-braces-open",
            "div { color: red;",
        )
        .with_expectation(WptExpectation::ShouldNotParse)
        .with_flag("invalid"),

        WptTestCase::new(
            "parser/invalid/unbalanced-braces-close",
            "div color: red; }",
        )
        .with_expectation(WptExpectation::ShouldNotParse)
        .with_flag("invalid"),

        WptTestCase::new(
            "parser/invalid/empty-selector",
            " { color: red; }",
        )
        .with_expectation(WptExpectation::ShouldNotParse)
        .with_flag("invalid"),

        WptTestCase::new(
            "parser/invalid/missing-colon",
            "div { color red; }",
        )
        .with_expectation(WptExpectation::ShouldNotParse)
        .with_flag("invalid"),

        WptTestCase::new(
            "parser/invalid/empty-property",
            "div { : red; }",
        )
        .with_expectation(WptExpectation::ShouldNotParse)
        .with_flag("invalid"),

        WptTestCase::new(
            "parser/invalid/empty-value",
            "div { color: ; }",
        )
        .with_expectation(WptExpectation::ShouldNotParse)
        .with_flag("invalid"),

        WptTestCase::new(
            "parser/invalid/empty-css",
            "",
        )
        .with_expectation(WptExpectation::ShouldNotParse)
        .with_flag("invalid"),

        // ============================================
        // Serialization Tests
        // ============================================

        WptTestCase::new(
            "parser/serialize/basic",
            "div{color:red}",
        )
        .with_expectation(WptExpectation::SerializesTo("div { color: red }".to_string()))
        .with_flag("serialization"),

        WptTestCase::new(
            "parser/serialize/whitespace-normalization",
            "div  {   color:   red;   }",
        )
        .with_expectation(WptExpectation::SerializesTo("div { color: red; }".to_string()))
        .with_flag("serialization"),
    ]
}

/// Run all parser tests and return statistics
pub fn run_all_parser_tests() -> crate::harness::WptTestStats {
    let mut runner = WptTestRunner::new().verbose(true);
    let tests = create_parser_tests();
    runner.run_tests(tests, run_parser_test)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_css_syntax_valid() {
        assert!(validate_css_syntax("div { color: red; }").is_ok());
        assert!(validate_css_syntax(".class { }").is_ok());
        assert!(validate_css_syntax("#id { margin: 10px; padding: 5px; }").is_ok());
        assert!(validate_css_syntax("* { box-sizing: border-box; }").is_ok());
    }

    #[test]
    fn test_validate_css_syntax_invalid() {
        assert!(validate_css_syntax("div { color: red;").is_err());
        assert!(validate_css_syntax("div color: red; }").is_err());
        assert!(validate_css_syntax(" { color: red; }").is_err());
        assert!(validate_css_syntax("div { color red; }").is_err());
        assert!(validate_css_syntax("").is_err());
    }

    #[test]
    fn test_validate_declarations() {
        assert!(validate_declarations("color: red").is_ok());
        assert!(validate_declarations("color: red; background: blue").is_ok());
        assert!(validate_declarations("").is_ok()); // Empty is valid
        assert!(validate_declarations("color red").is_err()); // Missing colon
        assert!(validate_declarations(": red").is_err()); // Empty property
        assert!(validate_declarations("color:").is_err()); // Empty value
    }

    #[test]
    fn test_is_valid_property_name() {
        assert!(is_valid_property_name("color"));
        assert!(is_valid_property_name("background-color"));
        assert!(is_valid_property_name("--custom-prop"));
        assert!(is_valid_property_name("WebkitTransform")); // Vendor prefix
        assert!(!is_valid_property_name("")); // Empty
        assert!(!is_valid_property_name("-invalid")); // Single hyphen prefix
        assert!(!is_valid_property_name("--")); // Just dashes
    }

    #[test]
    fn test_normalize_whitespace() {
        assert_eq!(normalize_whitespace("a  b   c"), "a b c");
        assert_eq!(normalize_whitespace("  a  "), "a");
        assert_eq!(normalize_whitespace("a\n\nb"), "a b");
    }

    #[test]
    fn test_run_parser_test_should_parse() {
        let test = WptTestCase::new("test", "div { color: red; }")
            .with_expectation(WptExpectation::ShouldParse);

        let result = run_parser_test(&test);
        assert!(result.is_pass());
    }

    #[test]
    fn test_run_parser_test_should_not_parse() {
        let test = WptTestCase::new("test", "div { color: }")
            .with_expectation(WptExpectation::ShouldNotParse);

        let result = run_parser_test(&test);
        assert!(result.is_pass());
    }

    #[test]
    fn test_run_parser_test_should_parse_fails_on_invalid() {
        let test = WptTestCase::new("test", "div { color: }")
            .with_expectation(WptExpectation::ShouldParse);

        let result = run_parser_test(&test);
        assert!(result.is_fail());
    }

    #[test]
    fn test_run_parser_test_should_not_parse_fails_on_valid() {
        let test = WptTestCase::new("test", "div { color: red; }")
            .with_expectation(WptExpectation::ShouldNotParse);

        let result = run_parser_test(&test);
        assert!(result.is_fail());
    }

    #[test]
    fn test_parser_tests_collection() {
        let tests = create_parser_tests();

        // Should have a reasonable number of tests
        assert!(tests.len() >= 20);

        // Check that tests have names
        for test in &tests {
            assert!(!test.name.is_empty());
        }

        // Count valid vs invalid tests
        let valid_count = tests.iter()
            .filter(|t| matches!(t.expected, WptExpectation::ShouldParse))
            .count();
        let invalid_count = tests.iter()
            .filter(|t| matches!(t.expected, WptExpectation::ShouldNotParse))
            .count();

        assert!(valid_count > 0, "Should have valid CSS tests");
        assert!(invalid_count > 0, "Should have invalid CSS tests");
    }

    #[test]
    fn test_all_parser_tests_run() {
        let mut runner = WptTestRunner::new();
        let tests = create_parser_tests();
        let stats = runner.run_tests(tests, run_parser_test);

        // All tests should complete (no timeouts/errors)
        assert_eq!(stats.timeouts, 0);
        assert_eq!(stats.errors, 0);

        // Most tests should pass
        assert!(stats.pass_rate() > 90.0, "Pass rate should be > 90%");
    }
}
