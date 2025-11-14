//! Integration tests for InvalidationEngine trait and helper functions

use css_invalidation::{
    compute_affected_elements, should_invalidate_subtree, DomTree, ElementId, Invalidation,
    InvalidationEngine, InvalidationScope, InvalidationTracker, InvalidationType,
};

// ============================================================================
// Helper Functions Tests
// ============================================================================

#[test]
fn test_should_invalidate_subtree_for_subtree_type() {
    assert!(should_invalidate_subtree(&InvalidationType::Subtree));
}

#[test]
fn test_should_invalidate_subtree_for_full_type() {
    assert!(should_invalidate_subtree(&InvalidationType::Full));
}

#[test]
fn test_should_not_invalidate_subtree_for_element_type() {
    assert!(!should_invalidate_subtree(&InvalidationType::Element));
}

#[test]
fn test_should_not_invalidate_subtree_for_attribute_type() {
    assert!(!should_invalidate_subtree(&InvalidationType::Attribute));
}

#[test]
fn test_should_not_invalidate_subtree_for_class_type() {
    assert!(!should_invalidate_subtree(&InvalidationType::Class));
}

#[test]
fn test_should_not_invalidate_subtree_for_state_type() {
    assert!(!should_invalidate_subtree(&InvalidationType::State));
}

#[test]
fn test_compute_affected_elements_global_scope() {
    let dom = DomTree::with_elements(vec![
        ElementId::new(1),
        ElementId::new(2),
        ElementId::new(3),
    ]);

    let invalidation = Invalidation::new(InvalidationType::Full, InvalidationScope::Global, 1000);

    let affected = compute_affected_elements(&invalidation, &dom);
    assert_eq!(affected.len(), 3);
    assert!(affected.contains(&ElementId::new(1)));
    assert!(affected.contains(&ElementId::new(2)));
    assert!(affected.contains(&ElementId::new(3)));
}

#[test]
fn test_compute_affected_elements_element_scope() {
    let dom = DomTree::with_elements(vec![
        ElementId::new(1),
        ElementId::new(2),
        ElementId::new(3),
    ]);

    let target_id = ElementId::new(2);
    let invalidation = Invalidation::new(
        InvalidationType::Attribute,
        InvalidationScope::Element(target_id),
        1000,
    );

    let affected = compute_affected_elements(&invalidation, &dom);
    assert_eq!(affected.len(), 1);
    assert!(affected.contains(&target_id));
}

#[test]
fn test_compute_affected_elements_subtree_scope() {
    let mut dom = DomTree::new();
    let root = ElementId::new(1);
    let child1 = ElementId::new(2);
    let child2 = ElementId::new(3);
    let grandchild = ElementId::new(4);

    dom.add_element(root);
    dom.add_element(child1);
    dom.add_element(child2);
    dom.add_element(grandchild);
    dom.set_parent(child1, root);
    dom.set_parent(child2, root);
    dom.set_parent(grandchild, child1);

    let invalidation = Invalidation::new(
        InvalidationType::Subtree,
        InvalidationScope::Subtree(root),
        1000,
    );

    let affected = compute_affected_elements(&invalidation, &dom);
    // Should include root and all descendants
    assert_eq!(affected.len(), 4);
    assert!(affected.contains(&root));
    assert!(affected.contains(&child1));
    assert!(affected.contains(&child2));
    assert!(affected.contains(&grandchild));
}

#[test]
fn test_compute_affected_elements_selector_scope() {
    let mut dom = DomTree::new();
    let elem1 = ElementId::new(1);
    let elem2 = ElementId::new(2);
    let elem3 = ElementId::new(3);

    dom.add_element(elem1);
    dom.add_element(elem2);
    dom.add_element(elem3);
    dom.add_class(elem1, "button");
    dom.add_class(elem3, "button");

    let invalidation = Invalidation::new(
        InvalidationType::Class,
        InvalidationScope::Selector(".button".to_string()),
        1000,
    );

    let affected = compute_affected_elements(&invalidation, &dom);
    // Should only include elements matching .button selector
    assert_eq!(affected.len(), 2);
    assert!(affected.contains(&elem1));
    assert!(affected.contains(&elem3));
    assert!(!affected.contains(&elem2));
}

// ============================================================================
// InvalidationEngine Trait Tests
// ============================================================================

#[test]
fn test_invalidation_engine_invalidate() {
    let mut tracker = InvalidationTracker::new();
    let invalidation = Invalidation::new(InvalidationType::Class, InvalidationScope::Global, 1000);

    tracker.invalidate(invalidation.clone());
    assert_eq!(tracker.pending_invalidations().len(), 1);
}

#[test]
fn test_invalidation_engine_mark_dirty() {
    let mut tracker = InvalidationTracker::new();
    let element_id = ElementId::new(42);

    tracker.mark_dirty(element_id, InvalidationType::Attribute);
    assert!(tracker.is_dirty(element_id));
}

#[test]
fn test_invalidation_engine_is_dirty() {
    let mut tracker = InvalidationTracker::new();
    let element_id = ElementId::new(42);

    assert!(!tracker.is_dirty(element_id));

    tracker.mark_dirty(element_id, InvalidationType::Element);
    assert!(tracker.is_dirty(element_id));
}

#[test]
fn test_invalidation_engine_clear_dirty() {
    let mut tracker = InvalidationTracker::new();
    let element_id = ElementId::new(42);

    tracker.mark_dirty(element_id, InvalidationType::Element);
    assert!(tracker.is_dirty(element_id));

    tracker.clear_dirty(element_id);
    assert!(!tracker.is_dirty(element_id));
}

#[test]
fn test_invalidation_engine_process_invalidations_empty() {
    let mut tracker = InvalidationTracker::new();
    let dom = DomTree::new();

    let result = tracker.process_invalidations(&dom);
    assert_eq!(result.invalidations().len(), 0);
    assert_eq!(result.affected_elements().len(), 0);
}

#[test]
fn test_invalidation_engine_process_invalidations_with_pending() {
    let mut tracker = InvalidationTracker::new();
    let mut dom = DomTree::new();

    let elem1 = ElementId::new(1);
    let elem2 = ElementId::new(2);
    dom.add_element(elem1);
    dom.add_element(elem2);

    let invalidation = Invalidation::new(InvalidationType::Full, InvalidationScope::Global, 1000);

    tracker.invalidate(invalidation);

    let result = tracker.process_invalidations(&dom);
    assert_eq!(result.invalidations().len(), 1);
    assert_eq!(result.affected_elements().len(), 2);

    // Pending invalidations should be cleared after processing
    assert_eq!(tracker.pending_invalidations().len(), 0);
}

#[test]
fn test_invalidation_engine_coalescing_multiple_invalidations() {
    let mut tracker = InvalidationTracker::new();
    let element_id = ElementId::new(42);

    // Mark same element dirty multiple times
    tracker.mark_dirty(element_id, InvalidationType::Attribute);
    tracker.mark_dirty(element_id, InvalidationType::Class);
    tracker.mark_dirty(element_id, InvalidationType::State);

    // Should still only be marked as dirty once
    assert_eq!(tracker.dirty_elements().len(), 1);
}

#[test]
fn test_invalidation_processing_order() {
    let mut tracker = InvalidationTracker::new();
    let mut dom = DomTree::new();

    let elem1 = ElementId::new(1);
    dom.add_element(elem1);

    // Add invalidations with different timestamps
    let inv1 = Invalidation::new(
        InvalidationType::Attribute,
        InvalidationScope::Element(elem1),
        1000,
    );
    let inv2 = Invalidation::new(
        InvalidationType::Class,
        InvalidationScope::Element(elem1),
        2000,
    );
    let inv3 = Invalidation::new(
        InvalidationType::State,
        InvalidationScope::Element(elem1),
        1500,
    );

    tracker.invalidate(inv1);
    tracker.invalidate(inv2);
    tracker.invalidate(inv3);

    let result = tracker.process_invalidations(&dom);
    let invalidations = result.invalidations();

    // Should be processed in timestamp order
    assert_eq!(invalidations[0].timestamp(), 1000);
    assert_eq!(invalidations[1].timestamp(), 1500);
    assert_eq!(invalidations[2].timestamp(), 2000);
}

// ============================================================================
// Additional Contract Tests
// ============================================================================

#[test]
fn test_full_invalidation_marks_all_elements_dirty() {
    let mut tracker = InvalidationTracker::new();
    let mut dom = DomTree::new();

    let elem1 = ElementId::new(1);
    let elem2 = ElementId::new(2);
    let elem3 = ElementId::new(3);
    dom.add_element(elem1);
    dom.add_element(elem2);
    dom.add_element(elem3);

    let invalidation = Invalidation::new(InvalidationType::Full, InvalidationScope::Global, 1000);

    tracker.invalidate(invalidation);
    tracker.process_invalidations(&dom);

    // All elements should be marked dirty
    assert!(tracker.is_dirty(elem1));
    assert!(tracker.is_dirty(elem2));
    assert!(tracker.is_dirty(elem3));
}

#[test]
fn test_subtree_invalidation_marks_descendants() {
    let mut tracker = InvalidationTracker::new();
    let mut dom = DomTree::new();

    let root = ElementId::new(1);
    let child = ElementId::new(2);
    let grandchild = ElementId::new(3);

    dom.add_element(root);
    dom.add_element(child);
    dom.add_element(grandchild);
    dom.set_parent(child, root);
    dom.set_parent(grandchild, child);

    let invalidation = Invalidation::new(
        InvalidationType::Subtree,
        InvalidationScope::Subtree(root),
        1000,
    );

    tracker.invalidate(invalidation);
    tracker.process_invalidations(&dom);

    // All elements in subtree should be marked dirty
    assert!(tracker.is_dirty(root));
    assert!(tracker.is_dirty(child));
    assert!(tracker.is_dirty(grandchild));
}

#[test]
fn test_element_invalidation_marks_single_element() {
    let mut tracker = InvalidationTracker::new();
    let mut dom = DomTree::new();

    let elem1 = ElementId::new(1);
    let elem2 = ElementId::new(2);
    dom.add_element(elem1);
    dom.add_element(elem2);

    let invalidation = Invalidation::new(
        InvalidationType::Element,
        InvalidationScope::Element(elem1),
        1000,
    );

    tracker.invalidate(invalidation);
    tracker.process_invalidations(&dom);

    // Only the specified element should be marked dirty
    assert!(tracker.is_dirty(elem1));
    assert!(!tracker.is_dirty(elem2));
}

#[test]
fn test_dirty_flag_clearing_after_restyle() {
    let mut tracker = InvalidationTracker::new();
    let element_id = ElementId::new(42);

    tracker.mark_dirty(element_id, InvalidationType::Attribute);
    assert!(tracker.is_dirty(element_id));

    // Simulate restyle completion
    tracker.clear_dirty(element_id);
    assert!(!tracker.is_dirty(element_id));
}
