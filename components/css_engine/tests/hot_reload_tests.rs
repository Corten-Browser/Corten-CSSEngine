//! Integration tests for CSS Engine Hot Reload functionality
//!
//! These tests verify the hot-reload system works correctly for DevTools
//! live editing scenarios.

use css_engine::{
    HotReloadConfig, HotReloadManager, ParsedRuleSnapshot, StyleSheetId, StylesheetDiff,
};

// ============================================================================
// Stylesheet Replacement Tests
// ============================================================================

#[test]
fn test_full_stylesheet_replacement() {
    let mut manager = HotReloadManager::new();

    // Register initial stylesheet
    let id = manager.register_stylesheet(
        r#"
        body { color: red; margin: 0; }
        .container { width: 100%; padding: 20px; }
        "#,
    );

    // Replace with entirely different stylesheet
    let diff = manager
        .update_stylesheet(
            &id,
            r#"
        html { font-size: 16px; }
        .wrapper { display: flex; }
        "#,
        )
        .unwrap();

    // Should detect major changes
    assert!(diff.has_changes());
    assert!(diff.removed_rules.len() >= 2); // body and .container removed
    assert!(diff.added_rules.len() >= 2); // html and .wrapper added
}

#[test]
fn test_stylesheet_replacement_preserves_version() {
    let mut manager = HotReloadManager::new();
    let id = manager.register_stylesheet("body { color: red; }");

    // Multiple updates
    manager
        .update_stylesheet(&id, "body { color: blue; }")
        .unwrap();
    manager
        .update_stylesheet(&id, "body { color: green; }")
        .unwrap();
    manager
        .update_stylesheet(&id, "body { color: yellow; }")
        .unwrap();

    let stylesheet = manager.get_stylesheet(&id).unwrap();
    assert_eq!(stylesheet.version, 4); // Initial (1) + 3 updates
}

#[test]
fn test_stylesheet_replacement_with_source_url() {
    let mut manager = HotReloadManager::new();
    let id = manager.register_stylesheet_with_url("body { color: red; }", Some("styles/main.css"));

    // Update should preserve source URL
    manager
        .update_stylesheet(&id, "body { color: blue; }")
        .unwrap();

    let stylesheet = manager.get_stylesheet(&id).unwrap();
    assert_eq!(stylesheet.source_url.as_deref(), Some("styles/main.css"));
}

// ============================================================================
// Partial Rule Update Tests
// ============================================================================

#[test]
fn test_single_property_value_change() {
    let mut manager = HotReloadManager::new();
    let id = manager.register_stylesheet("body { color: red; font-size: 16px; margin: 0; }");

    // Change only one property
    let diff = manager
        .update_stylesheet(&id, "body { color: blue; font-size: 16px; margin: 0; }")
        .unwrap();

    assert_eq!(diff.modified_rules.len(), 1);
    assert!(diff.added_rules.is_empty());
    assert!(diff.removed_rules.is_empty());

    let modification = &diff.modified_rules[0];
    assert_eq!(modification.selector, "body");
    assert_eq!(modification.modified_declarations.len(), 1);
    assert_eq!(modification.modified_declarations[0].property, "color");
    assert_eq!(modification.modified_declarations[0].old_value, "red");
    assert_eq!(modification.modified_declarations[0].new_value, "blue");
}

#[test]
fn test_add_property_to_existing_rule() {
    let mut manager = HotReloadManager::new();
    let id = manager.register_stylesheet("body { color: red; }");

    // Add a new property
    let diff = manager
        .update_stylesheet(&id, "body { color: red; margin: 10px; }")
        .unwrap();

    assert_eq!(diff.modified_rules.len(), 1);
    assert_eq!(diff.modified_rules[0].added_declarations.len(), 1);
    assert_eq!(diff.modified_rules[0].added_declarations[0].0, "margin");
}

#[test]
fn test_remove_property_from_existing_rule() {
    let mut manager = HotReloadManager::new();
    let id = manager.register_stylesheet("body { color: red; margin: 10px; padding: 5px; }");

    // Remove a property
    let diff = manager
        .update_stylesheet(&id, "body { color: red; padding: 5px; }")
        .unwrap();

    assert_eq!(diff.modified_rules.len(), 1);
    assert_eq!(diff.modified_rules[0].removed_declarations.len(), 1);
    assert_eq!(diff.modified_rules[0].removed_declarations[0].0, "margin");
}

#[test]
fn test_multiple_property_changes_in_one_rule() {
    let mut manager = HotReloadManager::new();
    let id = manager.register_stylesheet("body { color: red; margin: 10px; padding: 5px; }");

    let diff = manager
        .update_stylesheet(&id, "body { color: blue; margin: 20px; border: 1px; }")
        .unwrap();

    let modification = &diff.modified_rules[0];
    assert_eq!(modification.modified_declarations.len(), 2); // color and margin changed
    assert_eq!(modification.added_declarations.len(), 1); // border added
    assert_eq!(modification.removed_declarations.len(), 1); // padding removed
}

#[test]
fn test_change_important_flag() {
    let mut manager = HotReloadManager::new();
    let id = manager.register_stylesheet("body { color: red; }");

    let diff = manager
        .update_stylesheet(&id, "body { color: red !important; }")
        .unwrap();

    assert_eq!(diff.modified_rules.len(), 1);
    let decl_mod = &diff.modified_rules[0].modified_declarations[0];
    assert!(!decl_mod.old_important);
    assert!(decl_mod.new_important);
}

// ============================================================================
// Invalidation After Update Tests
// ============================================================================

#[test]
fn test_invalidation_identifies_changed_selectors() {
    let mut manager = HotReloadManager::new();
    let id = manager.register_stylesheet(
        r#"
        body { color: red; }
        .container { width: 100%; }
        "#,
    );

    let diff = manager
        .update_stylesheet(
            &id,
            r#"
        body { color: blue; }
        .container { width: 50%; }
        "#,
        )
        .unwrap();

    // Both selectors should be marked as changed
    assert!(diff.changed_selectors.contains("body"));
    assert!(diff.changed_selectors.contains(".container"));
}

#[test]
fn test_invalidation_on_rule_add() {
    let mut manager = HotReloadManager::new();
    let id = manager.register_stylesheet("body { color: red; }");

    let diff = manager
        .update_stylesheet(&id, "body { color: red; }\n.new-class { display: flex; }")
        .unwrap();

    assert!(diff.changed_selectors.contains(".new-class"));
}

#[test]
fn test_invalidation_on_rule_remove() {
    let mut manager = HotReloadManager::new();
    let id = manager.register_stylesheet("body { color: red; }\n.old-class { display: block; }");

    let diff = manager
        .update_stylesheet(&id, "body { color: red; }")
        .unwrap();

    assert!(diff.changed_selectors.contains(".old-class"));
}

#[test]
fn test_no_invalidation_when_no_changes() {
    let mut manager = HotReloadManager::new();
    let css = "body { color: red; margin: 10px; }";
    let id = manager.register_stylesheet(css);

    // Update with identical content
    let diff = manager.update_stylesheet(&id, css).unwrap();

    assert!(!diff.has_changes());
    assert!(diff.changed_selectors.is_empty());
}

// ============================================================================
// Error Handling Tests
// ============================================================================

#[test]
fn test_update_nonexistent_stylesheet_fails() {
    let mut manager = HotReloadManager::new();
    let fake_id = StyleSheetId::new(999);

    let result = manager.update_stylesheet(&fake_id, "body { color: red; }");
    assert!(result.is_err());
}

#[test]
fn test_malformed_css_unclosed_brace() {
    let mut manager = HotReloadManager::new();
    let id = manager.register_stylesheet("body { color: red; }");

    // Unclosed brace
    let result = manager.update_stylesheet(&id, "body { color: blue;");
    assert!(result.is_err());
}

#[test]
fn test_malformed_css_extra_closing_brace() {
    let mut manager = HotReloadManager::new();
    let id = manager.register_stylesheet("body { color: red; }");

    // Extra closing brace
    let result = manager.update_stylesheet(&id, "body { color: blue; } }");
    assert!(result.is_err());
}

#[test]
fn test_malformed_css_nested_braces_valid() {
    let mut manager = HotReloadManager::new();
    let id = manager.register_stylesheet("body { color: red; }");

    // This is technically valid CSS (media query)
    let result = manager.update_stylesheet(&id, "@media screen { body { color: blue; } }");
    // Should not error on balanced braces
    assert!(result.is_ok());
}

#[test]
fn test_validation_can_be_disabled() {
    let config = HotReloadConfig {
        validate_before_apply: false,
        ..Default::default()
    };
    let mut manager = HotReloadManager::with_config(config);
    let id = manager.register_stylesheet("body { color: red; }");

    // With validation disabled, malformed CSS is accepted
    let result = manager.update_stylesheet(&id, "body { color: blue;");
    assert!(result.is_ok());
}

// ============================================================================
// Undo/Redo Tests
// ============================================================================

#[test]
fn test_undo_restores_previous_state() {
    let mut manager = HotReloadManager::new();
    let id = manager.register_stylesheet("body { color: red; }");

    manager
        .update_stylesheet(&id, "body { color: blue; }")
        .unwrap();

    // Verify current state
    assert!(manager.get_source(&id).unwrap().contains("blue"));

    // Undo
    manager.undo(&id).unwrap();

    // Should be back to red
    assert!(manager.get_source(&id).unwrap().contains("red"));
}

#[test]
fn test_redo_restores_undone_state() {
    let mut manager = HotReloadManager::new();
    let id = manager.register_stylesheet("body { color: red; }");

    manager
        .update_stylesheet(&id, "body { color: blue; }")
        .unwrap();
    manager.undo(&id).unwrap();

    // Redo
    manager.redo(&id).unwrap();

    // Should be back to blue
    assert!(manager.get_source(&id).unwrap().contains("blue"));
}

#[test]
fn test_undo_chain() {
    let mut manager = HotReloadManager::new();
    let id = manager.register_stylesheet("body { color: red; }");

    manager
        .update_stylesheet(&id, "body { color: blue; }")
        .unwrap();
    manager
        .update_stylesheet(&id, "body { color: green; }")
        .unwrap();
    manager
        .update_stylesheet(&id, "body { color: yellow; }")
        .unwrap();

    // Undo three times
    manager.undo(&id).unwrap();
    assert!(manager.get_source(&id).unwrap().contains("green"));

    manager.undo(&id).unwrap();
    assert!(manager.get_source(&id).unwrap().contains("blue"));

    manager.undo(&id).unwrap();
    assert!(manager.get_source(&id).unwrap().contains("red"));
}

#[test]
fn test_new_change_clears_redo_history() {
    let mut manager = HotReloadManager::new();
    let id = manager.register_stylesheet("body { color: red; }");

    manager
        .update_stylesheet(&id, "body { color: blue; }")
        .unwrap();
    manager.undo(&id).unwrap();

    // Make a new change
    manager
        .update_stylesheet(&id, "body { color: green; }")
        .unwrap();

    // Redo should no longer be available
    assert!(!manager.can_redo(&id));
}

#[test]
fn test_undo_returns_diff() {
    let mut manager = HotReloadManager::new();
    let id = manager.register_stylesheet("body { color: red; }");

    manager
        .update_stylesheet(&id, "body { color: blue; margin: 10px; }")
        .unwrap();

    let diff = manager.undo(&id).unwrap();

    // Diff should show the reverse changes
    assert!(diff.has_changes());
}

// ============================================================================
// Complex Selector Tests
// ============================================================================

#[test]
fn test_complex_selectors_class_combinations() {
    let mut manager = HotReloadManager::new();
    let id = manager.register_stylesheet(".a.b.c { color: red; }");

    let diff = manager
        .update_stylesheet(&id, ".a.b.c { color: blue; }")
        .unwrap();

    assert_eq!(diff.modified_rules.len(), 1);
    assert_eq!(diff.modified_rules[0].selector, ".a.b.c");
}

#[test]
fn test_complex_selectors_descendant() {
    let mut manager = HotReloadManager::new();
    let id = manager.register_stylesheet(".container .item { display: flex; }");

    let diff = manager
        .update_stylesheet(&id, ".container .item { display: grid; }")
        .unwrap();

    assert!(diff.has_changes());
    assert!(diff.changed_selectors.contains(".container .item"));
}

#[test]
fn test_complex_selectors_child_combinator() {
    let mut manager = HotReloadManager::new();
    let id = manager.register_stylesheet("ul > li { list-style: none; }");

    let diff = manager
        .update_stylesheet(&id, "ul > li { list-style: disc; }")
        .unwrap();

    assert_eq!(diff.modified_rules.len(), 1);
}

#[test]
fn test_complex_selectors_pseudo_classes() {
    let mut manager = HotReloadManager::new();
    let id = manager.register_stylesheet("a:hover { color: red; }");

    let diff = manager
        .update_stylesheet(&id, "a:hover { color: blue; }")
        .unwrap();

    assert_eq!(diff.modified_rules.len(), 1);
}

#[test]
fn test_complex_selectors_pseudo_elements() {
    let mut manager = HotReloadManager::new();
    let id = manager.register_stylesheet("p::before { content: ''; }");

    let diff = manager
        .update_stylesheet(&id, "p::before { content: '>'; }")
        .unwrap();

    assert_eq!(diff.modified_rules.len(), 1);
}

#[test]
fn test_complex_selectors_attribute() {
    let mut manager = HotReloadManager::new();
    let id = manager.register_stylesheet("[data-state=\"active\"] { background: blue; }");

    let diff = manager
        .update_stylesheet(&id, "[data-state=\"active\"] { background: green; }")
        .unwrap();

    assert_eq!(diff.modified_rules.len(), 1);
}

// ============================================================================
// Import Tracking Tests
// ============================================================================

#[test]
fn test_detect_import_added() {
    let mut manager = HotReloadManager::new();
    let id = manager.register_stylesheet("body { color: red; }");

    let diff = manager
        .update_stylesheet(&id, "@import 'base.css';\nbody { color: red; }")
        .unwrap();

    assert!(diff.imports_changed);
}

#[test]
fn test_detect_import_removed() {
    let mut manager = HotReloadManager::new();
    let id = manager.register_stylesheet("@import 'base.css';\nbody { color: red; }");

    let diff = manager
        .update_stylesheet(&id, "body { color: red; }")
        .unwrap();

    assert!(diff.imports_changed);
}

#[test]
fn test_detect_import_changed() {
    let mut manager = HotReloadManager::new();
    let id = manager.register_stylesheet("@import 'old.css';\nbody { color: red; }");

    let diff = manager
        .update_stylesheet(&id, "@import 'new.css';\nbody { color: red; }")
        .unwrap();

    assert!(diff.imports_changed);
}

#[test]
fn test_no_import_change_detected() {
    let mut manager = HotReloadManager::new();
    let id = manager.register_stylesheet("@import 'base.css';\nbody { color: red; }");

    let diff = manager
        .update_stylesheet(&id, "@import 'base.css';\nbody { color: blue; }")
        .unwrap();

    assert!(!diff.imports_changed);
}

// ============================================================================
// Enable/Disable Tests
// ============================================================================

#[test]
fn test_disable_stylesheet() {
    let mut manager = HotReloadManager::new();
    let id = manager.register_stylesheet("body { color: red; }");

    assert!(manager.is_enabled(&id));

    manager.set_enabled(&id, false).unwrap();
    assert!(!manager.is_enabled(&id));
}

#[test]
fn test_reenable_stylesheet() {
    let mut manager = HotReloadManager::new();
    let id = manager.register_stylesheet("body { color: red; }");

    manager.set_enabled(&id, false).unwrap();
    manager.set_enabled(&id, true).unwrap();

    assert!(manager.is_enabled(&id));
}

#[test]
fn test_update_disabled_stylesheet() {
    let mut manager = HotReloadManager::new();
    let id = manager.register_stylesheet("body { color: red; }");

    manager.set_enabled(&id, false).unwrap();

    // Updates should still work on disabled stylesheets
    let result = manager.update_stylesheet(&id, "body { color: blue; }");
    assert!(result.is_ok());

    // Should remain disabled after update
    assert!(!manager.is_enabled(&id));
}

// ============================================================================
// Multiple Stylesheet Tests
// ============================================================================

#[test]
fn test_multiple_stylesheets_independent() {
    let mut manager = HotReloadManager::new();

    let id1 = manager.register_stylesheet("body { color: red; }");
    let id2 = manager.register_stylesheet("div { margin: 10px; }");

    // Update one shouldn't affect the other
    manager
        .update_stylesheet(&id1, "body { color: blue; }")
        .unwrap();

    assert!(manager.get_source(&id1).unwrap().contains("blue"));
    assert!(manager.get_source(&id2).unwrap().contains("margin"));
}

#[test]
fn test_multiple_stylesheets_separate_undo_history() {
    let mut manager = HotReloadManager::new();

    let id1 = manager.register_stylesheet("body { color: red; }");
    let id2 = manager.register_stylesheet("div { margin: 10px; }");

    manager
        .update_stylesheet(&id1, "body { color: blue; }")
        .unwrap();
    manager
        .update_stylesheet(&id2, "div { margin: 20px; }")
        .unwrap();

    // Undo on id1 shouldn't affect id2
    manager.undo(&id1).unwrap();

    assert!(manager.get_source(&id1).unwrap().contains("red"));
    assert!(manager.get_source(&id2).unwrap().contains("20px"));
}

#[test]
fn test_remove_stylesheet_clears_history() {
    let mut manager = HotReloadManager::new();
    let id = manager.register_stylesheet("body { color: red; }");

    manager
        .update_stylesheet(&id, "body { color: blue; }")
        .unwrap();

    manager.remove_stylesheet(&id);

    // Re-register and check there's no undo history
    let new_id = manager.register_stylesheet("body { color: green; }");

    // New stylesheet should have no undo history
    assert!(!manager.can_undo(&new_id));
}

// ============================================================================
// Performance and Edge Case Tests
// ============================================================================

#[test]
fn test_large_stylesheet_update() {
    let mut manager = HotReloadManager::new();

    // Create a large stylesheet
    let mut css = String::new();
    for i in 0..100 {
        css.push_str(&format!(".class{} {{ color: red; margin: {}px; }}\n", i, i));
    }

    let id = manager.register_stylesheet(&css);

    // Update it
    let updated_css = css.replace("color: red", "color: blue");
    let diff = manager.update_stylesheet(&id, &updated_css).unwrap();

    // Should have 100 modified rules
    assert_eq!(diff.modified_rules.len(), 100);
}

#[test]
fn test_empty_rule() {
    let mut manager = HotReloadManager::new();
    let id = manager.register_stylesheet("body { }");

    let diff = manager
        .update_stylesheet(&id, "body { color: red; }")
        .unwrap();

    // Should detect the property addition
    assert!(diff.has_changes());
}

#[test]
fn test_whitespace_normalization() {
    let mut manager = HotReloadManager::new();
    let id = manager.register_stylesheet("body   {   color:   red;   }");

    // Same content with different whitespace
    let _diff = manager
        .update_stylesheet(&id, "body { color: red; }")
        .unwrap();

    // Selectors should be normalized, but content change is detected
    // (since source string differs)
    let stylesheet = manager.get_stylesheet(&id).unwrap();
    assert_eq!(stylesheet.rules[0].selector, "body");
}

#[test]
fn test_case_sensitivity_properties() {
    let mut manager = HotReloadManager::new();
    let id = manager.register_stylesheet("body { COLOR: red; }");

    let _diff = manager
        .update_stylesheet(&id, "body { color: red; }")
        .unwrap();

    // Properties are case-insensitive in CSS, so this might or might not be a change
    // depending on implementation
    let stylesheet = manager.get_stylesheet(&id).unwrap();
    // Our implementation lowercases properties
    assert!(stylesheet.rules[0]
        .declarations
        .iter()
        .any(|(p, _, _)| p == "color"));
}

// ============================================================================
// Diff Utility Tests
// ============================================================================

#[test]
fn test_diff_affected_rule_count() {
    let mut manager = HotReloadManager::new();
    let id = manager.register_stylesheet(
        r#"
        body { color: red; }
        div { margin: 10px; }
        span { padding: 5px; }
        "#,
    );

    let diff = manager
        .update_stylesheet(
            &id,
            r#"
        body { color: blue; }
        p { font-size: 14px; }
        "#,
        )
        .unwrap();

    // 1 modified (body), 2 removed (div, span), 1 added (p)
    assert_eq!(diff.affected_rule_count(), 4);
}

#[test]
fn test_diff_merge() {
    let mut diff1 = StylesheetDiff::new();
    diff1.added_rules.push(ParsedRuleSnapshot::new("body"));

    let mut diff2 = StylesheetDiff::new();
    diff2.removed_rules.push(ParsedRuleSnapshot::new("div"));
    diff2.new_version = 5;

    diff1.merge(diff2);

    assert_eq!(diff1.added_rules.len(), 1);
    assert_eq!(diff1.removed_rules.len(), 1);
    assert_eq!(diff1.new_version, 5);
}
