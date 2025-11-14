//! Unit tests for invalidation types

use css_invalidation::{
    InvalidationType, InvalidationScope, Invalidation, InvalidationSet,
    InvalidationTracker, InvalidationEngine, ElementId,
};
use std::collections::HashSet;

// ============================================================================
// InvalidationType Tests
// ============================================================================

#[test]
fn test_invalidation_type_variants_exist() {
    // Test that all variants can be created
    let _ = InvalidationType::Full;
    let _ = InvalidationType::Subtree;
    let _ = InvalidationType::Element;
    let _ = InvalidationType::Attribute;
    let _ = InvalidationType::Class;
    let _ = InvalidationType::State;
}

#[test]
fn test_invalidation_type_equality() {
    assert_eq!(InvalidationType::Full, InvalidationType::Full);
    assert_eq!(InvalidationType::Subtree, InvalidationType::Subtree);
    assert_ne!(InvalidationType::Full, InvalidationType::Subtree);
}

#[test]
fn test_invalidation_type_debug() {
    let inv_type = InvalidationType::Full;
    let debug_str = format!("{:?}", inv_type);
    assert!(debug_str.contains("Full"));
}

// ============================================================================
// InvalidationScope Tests
// ============================================================================

#[test]
fn test_invalidation_scope_global() {
    let scope = InvalidationScope::Global;
    assert_eq!(scope, InvalidationScope::Global);
}

#[test]
fn test_invalidation_scope_selector() {
    let scope = InvalidationScope::Selector(".button".to_string());
    match scope {
        InvalidationScope::Selector(s) => assert_eq!(s, ".button"),
        _ => panic!("Expected Selector variant"),
    }
}

#[test]
fn test_invalidation_scope_element() {
    let element_id = ElementId::new(42);
    let scope = InvalidationScope::Element(element_id);
    match scope {
        InvalidationScope::Element(id) => assert_eq!(id, element_id),
        _ => panic!("Expected Element variant"),
    }
}

#[test]
fn test_invalidation_scope_subtree() {
    let element_id = ElementId::new(42);
    let scope = InvalidationScope::Subtree(element_id);
    match scope {
        InvalidationScope::Subtree(id) => assert_eq!(id, element_id),
        _ => panic!("Expected Subtree variant"),
    }
}

// ============================================================================
// ElementId Tests
// ============================================================================

#[test]
fn test_element_id_creation() {
    let id = ElementId::new(42);
    assert_eq!(id.value(), 42);
}

#[test]
fn test_element_id_equality() {
    let id1 = ElementId::new(42);
    let id2 = ElementId::new(42);
    let id3 = ElementId::new(43);

    assert_eq!(id1, id2);
    assert_ne!(id1, id3);
}

#[test]
fn test_element_id_hash() {
    let mut set = HashSet::new();
    set.insert(ElementId::new(42));
    set.insert(ElementId::new(42)); // Duplicate
    set.insert(ElementId::new(43));

    assert_eq!(set.len(), 2); // Should only have 2 unique IDs
}

// ============================================================================
// Invalidation Tests
// ============================================================================

#[test]
fn test_invalidation_creation() {
    let invalidation = Invalidation::new(
        InvalidationType::Full,
        InvalidationScope::Global,
        1000,
    );

    assert_eq!(invalidation.invalidation_type(), &InvalidationType::Full);
    assert_eq!(invalidation.scope(), &InvalidationScope::Global);
    assert_eq!(invalidation.timestamp(), 1000);
}

#[test]
fn test_invalidation_with_element_scope() {
    let element_id = ElementId::new(42);
    let invalidation = Invalidation::new(
        InvalidationType::Attribute,
        InvalidationScope::Element(element_id),
        2000,
    );

    assert_eq!(invalidation.invalidation_type(), &InvalidationType::Attribute);
    match invalidation.scope() {
        InvalidationScope::Element(id) => assert_eq!(*id, element_id),
        _ => panic!("Expected Element scope"),
    }
}

// ============================================================================
// InvalidationSet Tests
// ============================================================================

#[test]
fn test_invalidation_set_creation() {
    let set = InvalidationSet::new();
    assert_eq!(set.invalidations().len(), 0);
    assert_eq!(set.affected_elements().len(), 0);
}

#[test]
fn test_invalidation_set_with_data() {
    let invalidation = Invalidation::new(
        InvalidationType::Class,
        InvalidationScope::Global,
        1000,
    );

    let mut affected = HashSet::new();
    affected.insert(ElementId::new(1));
    affected.insert(ElementId::new(2));

    let set = InvalidationSet::with_data(vec![invalidation], affected.clone());

    assert_eq!(set.invalidations().len(), 1);
    assert_eq!(set.affected_elements().len(), 2);
    assert!(set.affected_elements().contains(&ElementId::new(1)));
}

#[test]
fn test_invalidation_set_add_invalidation() {
    let mut set = InvalidationSet::new();
    let invalidation = Invalidation::new(
        InvalidationType::State,
        InvalidationScope::Global,
        1000,
    );

    set.add_invalidation(invalidation);
    assert_eq!(set.invalidations().len(), 1);
}

#[test]
fn test_invalidation_set_add_affected_element() {
    let mut set = InvalidationSet::new();
    let element_id = ElementId::new(42);

    set.add_affected_element(element_id);
    assert_eq!(set.affected_elements().len(), 1);
    assert!(set.affected_elements().contains(&element_id));
}

// ============================================================================
// InvalidationTracker Tests
// ============================================================================

#[test]
fn test_invalidation_tracker_creation() {
    let tracker = InvalidationTracker::new();
    assert_eq!(tracker.dirty_elements().len(), 0);
    assert_eq!(tracker.dirty_subtrees().len(), 0);
    assert_eq!(tracker.pending_invalidations().len(), 0);
}

#[test]
fn test_invalidation_tracker_mark_dirty() {
    let mut tracker = InvalidationTracker::new();
    let element_id = ElementId::new(42);

    tracker.mark_dirty(element_id, InvalidationType::Attribute);

    assert!(tracker.is_dirty(element_id));
    assert_eq!(tracker.dirty_elements().len(), 1);
}

#[test]
fn test_invalidation_tracker_mark_subtree_dirty() {
    let mut tracker = InvalidationTracker::new();
    let element_id = ElementId::new(42);

    tracker.mark_dirty(element_id, InvalidationType::Subtree);

    assert!(tracker.is_subtree_dirty(element_id));
    assert_eq!(tracker.dirty_subtrees().len(), 1);
}

#[test]
fn test_invalidation_tracker_clear_dirty() {
    let mut tracker = InvalidationTracker::new();
    let element_id = ElementId::new(42);

    tracker.mark_dirty(element_id, InvalidationType::Element);
    assert!(tracker.is_dirty(element_id));

    tracker.clear_dirty(element_id);
    assert!(!tracker.is_dirty(element_id));
}

#[test]
fn test_invalidation_tracker_add_pending() {
    let mut tracker = InvalidationTracker::new();
    let invalidation = Invalidation::new(
        InvalidationType::Class,
        InvalidationScope::Global,
        1000,
    );

    tracker.add_pending_invalidation(invalidation);
    assert_eq!(tracker.pending_invalidations().len(), 1);
}
