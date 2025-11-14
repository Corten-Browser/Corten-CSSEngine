//! CSS Invalidation - Incremental style invalidation for performance
//!
//! This module provides the invalidation system for tracking which elements
//! need their styles recomputed after changes to the DOM or stylesheets.
//!
//! # Key Concepts
//!
//! - **InvalidationType**: What kind of change occurred
//! - **InvalidationScope**: Which elements are affected
//! - **Invalidation**: A single invalidation event
//! - **InvalidationTracker**: Tracks dirty elements
//! - **InvalidationEngine**: Trait for processing invalidations

use std::collections::{HashMap, HashSet};

// ============================================================================
// Element ID
// ============================================================================

/// Unique identifier for a DOM element
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ElementId(u64);

impl ElementId {
    /// Create a new element ID
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    /// Get the numeric value of the element ID
    pub fn value(&self) -> u64 {
        self.0
    }
}

// ============================================================================
// DOM Tree (Simplified)
// ============================================================================

/// Simplified DOM tree structure for invalidation processing
#[derive(Debug, Clone)]
pub struct DomTree {
    elements: HashSet<ElementId>,
    parent_map: HashMap<ElementId, ElementId>,
    children_map: HashMap<ElementId, Vec<ElementId>>,
    class_map: HashMap<ElementId, HashSet<String>>,
}

impl DomTree {
    /// Create a new empty DOM tree
    pub fn new() -> Self {
        Self {
            elements: HashSet::new(),
            parent_map: HashMap::new(),
            children_map: HashMap::new(),
            class_map: HashMap::new(),
        }
    }

    /// Create a DOM tree with the given elements
    pub fn with_elements(elements: Vec<ElementId>) -> Self {
        let mut tree = Self::new();
        for elem in elements {
            tree.add_element(elem);
        }
        tree
    }

    /// Add an element to the tree
    pub fn add_element(&mut self, element: ElementId) {
        self.elements.insert(element);
    }

    /// Set the parent of an element
    pub fn set_parent(&mut self, child: ElementId, parent: ElementId) {
        self.parent_map.insert(child, parent);
        self.children_map.entry(parent).or_default().push(child);
    }

    /// Add a class to an element
    pub fn add_class(&mut self, element: ElementId, class: &str) {
        self.class_map
            .entry(element)
            .or_default()
            .insert(class.to_string());
    }

    /// Get all elements in the tree
    pub fn elements(&self) -> &HashSet<ElementId> {
        &self.elements
    }

    /// Get all descendants of an element (including the element itself)
    pub fn get_descendants(&self, element: ElementId) -> HashSet<ElementId> {
        let mut result = HashSet::new();
        result.insert(element);
        self.collect_descendants(element, &mut result);
        result
    }

    fn collect_descendants(&self, element: ElementId, result: &mut HashSet<ElementId>) {
        if let Some(children) = self.children_map.get(&element) {
            for child in children {
                result.insert(*child);
                self.collect_descendants(*child, result);
            }
        }
    }

    /// Check if an element has a specific class
    pub fn has_class(&self, element: ElementId, class: &str) -> bool {
        self.class_map
            .get(&element)
            .map(|classes| classes.contains(class))
            .unwrap_or(false)
    }

    /// Get elements matching a simple class selector (e.g., ".button")
    pub fn get_elements_by_selector(&self, selector: &str) -> HashSet<ElementId> {
        if let Some(class_name) = selector.strip_prefix('.') {
            // Simple class selector
            self.elements
                .iter()
                .filter(|elem| self.has_class(**elem, class_name))
                .copied()
                .collect()
        } else {
            // For simplicity, return empty set for non-class selectors
            HashSet::new()
        }
    }
}

impl Default for DomTree {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// InvalidationType
// ============================================================================

/// Type of style invalidation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InvalidationType {
    /// Full restyle required for all elements
    Full,
    /// Subtree restyle required
    Subtree,
    /// Single element restyle
    Element,
    /// Attribute change invalidation
    Attribute,
    /// Class change invalidation
    Class,
    /// State change invalidation (e.g., :hover, :focus)
    State,
}

// ============================================================================
// InvalidationScope
// ============================================================================

/// Scope of invalidation - which elements are affected
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidationScope {
    /// All elements in the document
    Global,
    /// Elements matching a specific selector
    Selector(String),
    /// A specific element
    Element(ElementId),
    /// An element and all its descendants
    Subtree(ElementId),
}

// ============================================================================
// Invalidation
// ============================================================================

/// A single invalidation event
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Invalidation {
    invalidation_type: InvalidationType,
    scope: InvalidationScope,
    timestamp: u64,
}

impl Invalidation {
    /// Create a new invalidation
    pub fn new(
        invalidation_type: InvalidationType,
        scope: InvalidationScope,
        timestamp: u64,
    ) -> Self {
        Self {
            invalidation_type,
            scope,
            timestamp,
        }
    }

    /// Get the invalidation type
    pub fn invalidation_type(&self) -> &InvalidationType {
        &self.invalidation_type
    }

    /// Get the invalidation scope
    pub fn scope(&self) -> &InvalidationScope {
        &self.scope
    }

    /// Get the timestamp
    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }
}

// ============================================================================
// InvalidationSet
// ============================================================================

/// Set of invalidations to process
#[derive(Debug, Clone)]
pub struct InvalidationSet {
    invalidations: Vec<Invalidation>,
    affected_elements: HashSet<ElementId>,
}

impl InvalidationSet {
    /// Create a new empty invalidation set
    pub fn new() -> Self {
        Self {
            invalidations: Vec::new(),
            affected_elements: HashSet::new(),
        }
    }

    /// Create an invalidation set with data
    pub fn with_data(
        invalidations: Vec<Invalidation>,
        affected_elements: HashSet<ElementId>,
    ) -> Self {
        Self {
            invalidations,
            affected_elements,
        }
    }

    /// Get the invalidations
    pub fn invalidations(&self) -> &[Invalidation] {
        &self.invalidations
    }

    /// Get the affected elements
    pub fn affected_elements(&self) -> &HashSet<ElementId> {
        &self.affected_elements
    }

    /// Add an invalidation to the set
    pub fn add_invalidation(&mut self, invalidation: Invalidation) {
        self.invalidations.push(invalidation);
    }

    /// Add an affected element
    pub fn add_affected_element(&mut self, element: ElementId) {
        self.affected_elements.insert(element);
    }
}

impl Default for InvalidationSet {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// InvalidationTracker
// ============================================================================

/// Tracks which elements need restyling
#[derive(Debug, Clone)]
pub struct InvalidationTracker {
    dirty_elements: HashSet<ElementId>,
    dirty_subtrees: HashSet<ElementId>,
    pending_invalidations: Vec<Invalidation>,
}

impl InvalidationTracker {
    /// Create a new invalidation tracker
    pub fn new() -> Self {
        Self {
            dirty_elements: HashSet::new(),
            dirty_subtrees: HashSet::new(),
            pending_invalidations: Vec::new(),
        }
    }

    /// Get dirty elements
    pub fn dirty_elements(&self) -> &HashSet<ElementId> {
        &self.dirty_elements
    }

    /// Get dirty subtrees
    pub fn dirty_subtrees(&self) -> &HashSet<ElementId> {
        &self.dirty_subtrees
    }

    /// Get pending invalidations
    pub fn pending_invalidations(&self) -> &[Invalidation] {
        &self.pending_invalidations
    }

    /// Check if a subtree is dirty
    pub fn is_subtree_dirty(&self, element: ElementId) -> bool {
        self.dirty_subtrees.contains(&element)
    }

    /// Add a pending invalidation
    pub fn add_pending_invalidation(&mut self, invalidation: Invalidation) {
        self.pending_invalidations.push(invalidation);
    }
}

impl Default for InvalidationTracker {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// InvalidationEngine Trait
// ============================================================================

/// Trait for managing incremental style invalidation
pub trait InvalidationEngine {
    /// Record an invalidation event
    fn invalidate(&mut self, invalidation: Invalidation);

    /// Process pending invalidations and return affected elements
    fn process_invalidations(&mut self, dom: &DomTree) -> InvalidationSet;

    /// Mark an element as needing restyle
    fn mark_dirty(&mut self, element_id: ElementId, invalidation_type: InvalidationType);

    /// Check if an element needs restyle
    fn is_dirty(&self, element_id: ElementId) -> bool;

    /// Clear the dirty flag for an element after restyle
    fn clear_dirty(&mut self, element_id: ElementId);
}

// ============================================================================
// InvalidationEngine Implementation for InvalidationTracker
// ============================================================================

impl InvalidationEngine for InvalidationTracker {
    fn invalidate(&mut self, invalidation: Invalidation) {
        self.pending_invalidations.push(invalidation);
    }

    fn process_invalidations(&mut self, dom: &DomTree) -> InvalidationSet {
        // Sort invalidations by timestamp
        self.pending_invalidations.sort_by_key(|inv| inv.timestamp);

        let mut result = InvalidationSet::new();
        let mut all_affected = HashSet::new();

        // Collect invalidations to process (to avoid borrow checker issues)
        let invalidations: Vec<_> = self.pending_invalidations.drain(..).collect();

        // Process each invalidation
        for invalidation in invalidations {
            // Compute affected elements
            let affected = compute_affected_elements(&invalidation, dom);

            // Mark elements as dirty
            for element in &affected {
                self.mark_dirty(*element, *invalidation.invalidation_type());
            }

            // Add to result
            all_affected.extend(affected);
            result.add_invalidation(invalidation);
        }

        // Set affected elements in result
        for element in all_affected {
            result.add_affected_element(element);
        }

        result
    }

    fn mark_dirty(&mut self, element_id: ElementId, invalidation_type: InvalidationType) {
        // Always mark element as dirty
        self.dirty_elements.insert(element_id);

        // Additionally mark subtree if needed
        if should_invalidate_subtree(&invalidation_type) {
            self.dirty_subtrees.insert(element_id);
        }
    }

    fn is_dirty(&self, element_id: ElementId) -> bool {
        self.dirty_elements.contains(&element_id)
    }

    fn clear_dirty(&mut self, element_id: ElementId) {
        self.dirty_elements.remove(&element_id);
        self.dirty_subtrees.remove(&element_id);
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Compute which elements are affected by an invalidation
pub fn compute_affected_elements(invalidation: &Invalidation, dom: &DomTree) -> HashSet<ElementId> {
    match invalidation.scope() {
        InvalidationScope::Global => {
            // All elements are affected
            dom.elements().clone()
        }
        InvalidationScope::Element(element_id) => {
            // Only the specific element is affected
            let mut result = HashSet::new();
            result.insert(*element_id);
            result
        }
        InvalidationScope::Subtree(element_id) => {
            // Element and all its descendants are affected
            dom.get_descendants(*element_id)
        }
        InvalidationScope::Selector(selector) => {
            // Elements matching the selector are affected
            dom.get_elements_by_selector(selector)
        }
    }
}

/// Determine if a subtree invalidation is needed
pub fn should_invalidate_subtree(invalidation_type: &InvalidationType) -> bool {
    matches!(
        invalidation_type,
        InvalidationType::Full | InvalidationType::Subtree
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_id_creation() {
        let id = ElementId::new(42);
        assert_eq!(id.value(), 42);
    }

    #[test]
    fn test_invalidation_type_equality() {
        assert_eq!(InvalidationType::Full, InvalidationType::Full);
        assert_ne!(InvalidationType::Full, InvalidationType::Element);
    }

    #[test]
    fn test_basic_invalidation() {
        let invalidation =
            Invalidation::new(InvalidationType::Class, InvalidationScope::Global, 1000);
        assert_eq!(invalidation.invalidation_type(), &InvalidationType::Class);
        assert_eq!(invalidation.timestamp(), 1000);
    }

    #[test]
    fn test_invalidation_tracker_mark_dirty() {
        let mut tracker = InvalidationTracker::new();
        let element_id = ElementId::new(42);

        tracker.mark_dirty(element_id, InvalidationType::Attribute);
        assert!(tracker.is_dirty(element_id));
    }

    #[test]
    fn test_should_invalidate_subtree() {
        assert!(should_invalidate_subtree(&InvalidationType::Full));
        assert!(should_invalidate_subtree(&InvalidationType::Subtree));
        assert!(!should_invalidate_subtree(&InvalidationType::Element));
    }
}
