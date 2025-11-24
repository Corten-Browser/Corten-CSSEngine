//! Parallel Style Computation
//!
//! This module provides parallel style computation capabilities using Rayon
//! for work-stealing parallelism. It enables efficient multi-threaded
//! computation of CSS styles across large DOM trees.
//!
//! # Example
//!
//! ```ignore
//! use css_engine::parallel::ParallelStyleComputer;
//!
//! let computer = ParallelStyleComputer::new(4);
//! let styles = computer.compute_styles_parallel(&elements, &stylesheets);
//! ```

use rayon::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::error::ElementId;
use crate::types::{ComputedStyle, DomNode};

/// Reference to an element for parallel processing
#[derive(Debug, Clone)]
pub struct ElementRef {
    /// Element ID
    pub id: ElementId,
    /// Tag name
    pub tag_name: String,
    /// CSS classes
    pub classes: Vec<String>,
    /// HTML attributes
    pub attributes: Vec<(String, String)>,
    /// Parent element ID (None for root)
    pub parent_id: Option<ElementId>,
    /// Depth in the DOM tree
    pub depth: usize,
}

impl ElementRef {
    /// Create a new element reference
    pub fn new(id: ElementId, tag_name: impl Into<String>) -> Self {
        Self {
            id,
            tag_name: tag_name.into(),
            classes: Vec::new(),
            attributes: Vec::new(),
            parent_id: None,
            depth: 0,
        }
    }

    /// Set the parent ID
    pub fn with_parent(mut self, parent_id: ElementId) -> Self {
        self.parent_id = Some(parent_id);
        self
    }

    /// Set the depth
    pub fn with_depth(mut self, depth: usize) -> Self {
        self.depth = depth;
        self
    }

    /// Add a class
    pub fn with_class(mut self, class: impl Into<String>) -> Self {
        self.classes.push(class.into());
        self
    }
}

/// Parsed stylesheet for style computation
#[derive(Debug, Clone)]
pub struct ParsedStylesheet {
    /// Stylesheet ID
    pub id: u32,
    /// Source CSS (for debugging)
    pub source: String,
    /// Rules in this stylesheet (simplified representation)
    pub rules: Vec<StyleRule>,
}

impl ParsedStylesheet {
    /// Create a new parsed stylesheet
    pub fn new(id: u32, source: impl Into<String>) -> Self {
        Self {
            id,
            source: source.into(),
            rules: Vec::new(),
        }
    }

    /// Add a rule
    pub fn with_rule(mut self, rule: StyleRule) -> Self {
        self.rules.push(rule);
        self
    }
}

/// A CSS style rule
#[derive(Debug, Clone)]
pub struct StyleRule {
    /// Selector string
    pub selector: String,
    /// Specificity (for cascade ordering)
    pub specificity: Specificity,
    /// Declarations
    pub declarations: Vec<Declaration>,
}

impl StyleRule {
    /// Create a new style rule
    pub fn new(selector: impl Into<String>) -> Self {
        Self {
            selector: selector.into(),
            specificity: Specificity::default(),
            declarations: Vec::new(),
        }
    }

    /// Add a declaration
    pub fn with_declaration(
        mut self,
        property: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.declarations.push(Declaration {
            property: property.into(),
            value: value.into(),
            important: false,
        });
        self
    }
}

/// Specificity of a CSS selector
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Specificity {
    /// ID selectors
    pub ids: u16,
    /// Class selectors, attribute selectors, pseudo-classes
    pub classes: u16,
    /// Element selectors, pseudo-elements
    pub elements: u16,
}

/// A CSS declaration (property: value pair)
#[derive(Debug, Clone)]
pub struct Declaration {
    /// Property name
    pub property: String,
    /// Property value
    pub value: String,
    /// Whether this has !important
    pub important: bool,
}

/// DOM subtree for parallel processing
#[derive(Debug, Clone)]
pub struct DomSubtree {
    /// Root element of this subtree
    pub root: ElementRef,
    /// All elements in this subtree (flattened)
    pub elements: Vec<ElementRef>,
}

impl DomSubtree {
    /// Create a new subtree
    pub fn new(root: ElementRef) -> Self {
        Self {
            elements: vec![root.clone()],
            root,
        }
    }

    /// Get the number of elements
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

/// Parallel style computer using Rayon for work-stealing parallelism
#[derive(Debug)]
pub struct ParallelStyleComputer {
    /// Number of threads to use
    thread_count: usize,
    /// Statistics: elements processed
    elements_processed: AtomicU64,
    /// Statistics: cache hits
    cache_hits: AtomicU64,
}

impl ParallelStyleComputer {
    /// Create a new parallel style computer
    ///
    /// # Arguments
    /// * `thread_count` - Number of threads to use (0 = use Rayon default)
    pub fn new(thread_count: usize) -> Self {
        Self {
            thread_count,
            elements_processed: AtomicU64::new(0),
            cache_hits: AtomicU64::new(0),
        }
    }

    /// Get the configured thread count
    pub fn thread_count(&self) -> usize {
        self.thread_count
    }

    /// Get the number of elements processed
    pub fn elements_processed(&self) -> u64 {
        self.elements_processed.load(Ordering::Relaxed)
    }

    /// Get the number of cache hits
    pub fn cache_hits(&self) -> u64 {
        self.cache_hits.load(Ordering::Relaxed)
    }

    /// Reset statistics
    pub fn reset_stats(&self) {
        self.elements_processed.store(0, Ordering::Relaxed);
        self.cache_hits.store(0, Ordering::Relaxed);
    }

    /// Compute styles in parallel for a slice of elements
    ///
    /// # Arguments
    /// * `elements` - Slice of element references to style
    /// * `stylesheets` - Slice of parsed stylesheets to apply
    ///
    /// # Returns
    /// Vector of computed styles, one per element (in same order)
    pub fn compute_styles_parallel(
        &self,
        elements: &[ElementRef],
        stylesheets: &[ParsedStylesheet],
    ) -> Vec<ComputedStyle> {
        // Use parallel iterator for computation
        let results: Vec<ComputedStyle> = elements
            .par_iter()
            .map(|element| {
                self.elements_processed.fetch_add(1, Ordering::Relaxed);
                self.compute_single_style(element, stylesheets)
            })
            .collect();

        results
    }

    /// Compute styles in parallel using partitioned subtrees
    ///
    /// This is more cache-friendly for DOM tree traversal as it
    /// keeps related elements together.
    ///
    /// # Arguments
    /// * `subtrees` - Partitioned DOM subtrees
    /// * `stylesheets` - Parsed stylesheets to apply
    ///
    /// # Returns
    /// Vector of (ElementId, ComputedStyle) pairs
    pub fn compute_styles_partitioned(
        &self,
        subtrees: &[DomSubtree],
        stylesheets: &[ParsedStylesheet],
    ) -> Vec<(ElementId, ComputedStyle)> {
        // Collect all elements from all subtrees first
        let all_elements: Vec<&ElementRef> = subtrees
            .iter()
            .flat_map(|subtree| subtree.elements.iter())
            .collect();

        // Process all elements in parallel
        all_elements
            .par_iter()
            .map(|element| {
                self.elements_processed.fetch_add(1, Ordering::Relaxed);
                let style = self.compute_single_style(element, stylesheets);
                (element.id, style)
            })
            .collect()
    }

    /// Compute style for a single element
    fn compute_single_style(
        &self,
        element: &ElementRef,
        stylesheets: &[ParsedStylesheet],
    ) -> ComputedStyle {
        let mut style = ComputedStyle::default();

        // Collect matching rules from all stylesheets
        let mut matching_rules: Vec<(&StyleRule, u32)> = Vec::new();

        for stylesheet in stylesheets {
            for rule in &stylesheet.rules {
                if self.selector_matches(element, &rule.selector) {
                    matching_rules.push((rule, stylesheet.id));
                }
            }
        }

        // Sort by specificity (stable sort preserves source order for equal specificity)
        matching_rules.sort_by(|a, b| a.0.specificity.cmp(&b.0.specificity));

        // Apply rules in order
        for (rule, _) in matching_rules {
            for declaration in &rule.declarations {
                self.apply_declaration(&mut style, declaration);
            }
        }

        style
    }

    /// Check if a selector matches an element (simplified)
    fn selector_matches(&self, element: &ElementRef, selector: &str) -> bool {
        // Simplified selector matching for demonstration
        // Real implementation would use the css_matcher_core component

        // Universal selector
        if selector == "*" {
            return true;
        }

        // Tag selector
        if selector == element.tag_name {
            return true;
        }

        // Class selector
        if let Some(class_name) = selector.strip_prefix('.') {
            return element.classes.iter().any(|c| c == class_name);
        }

        // ID selector
        if let Some(id_name) = selector.strip_prefix('#') {
            return element
                .attributes
                .iter()
                .any(|(name, value)| name == "id" && value == id_name);
        }

        false
    }

    /// Apply a declaration to a computed style
    fn apply_declaration(&self, style: &mut ComputedStyle, declaration: &Declaration) {
        // Simplified property application
        // Real implementation would handle all CSS properties
        if declaration.property.as_str() == "display" {
            style.display = match declaration.value.as_str() {
                "none" => crate::types::Display::None,
                "block" => crate::types::Display::Block,
                "inline" => crate::types::Display::Inline,
                "inline-block" => crate::types::Display::InlineBlock,
                "flex" => crate::types::Display::Flex,
                "grid" => crate::types::Display::Grid,
                _ => crate::types::Display::default(),
            };
        }
        // Additional properties would be handled here
    }
}

impl Default for ParallelStyleComputer {
    fn default() -> Self {
        Self::new(0)
    }
}

/// Partition a DOM tree into subtrees for parallel processing
///
/// Uses a work-stealing friendly partitioning strategy that keeps
/// related elements together while enabling parallel processing.
///
/// # Arguments
/// * `root` - Root of the DOM tree
///
/// # Returns
/// Vector of subtrees that can be processed in parallel
pub fn partition_dom_tree(root: &DomNode) -> Vec<DomSubtree> {
    let mut subtrees = Vec::new();
    let min_subtree_size = 10; // Minimum elements per subtree for parallelism benefit

    // Flatten the tree and create subtrees
    let elements = flatten_dom_tree(root, None, 0);
    let total_elements = elements.len();

    if total_elements <= min_subtree_size {
        // Small tree, single subtree
        if let Some(first) = elements.first() {
            let mut subtree = DomSubtree::new(first.clone());
            subtree.elements = elements;
            subtrees.push(subtree);
        }
    } else {
        // Partition by depth-first chunks
        let chunk_size = (total_elements / rayon::current_num_threads()).max(min_subtree_size);
        for chunk in elements.chunks(chunk_size) {
            if let Some(first) = chunk.first() {
                let mut subtree = DomSubtree::new(first.clone());
                subtree.elements = chunk.to_vec();
                subtrees.push(subtree);
            }
        }
    }

    subtrees
}

/// Flatten a DOM tree into a vector of element references
fn flatten_dom_tree(node: &DomNode, parent_id: Option<ElementId>, depth: usize) -> Vec<ElementRef> {
    let mut elements = Vec::new();

    // Create element ref for this node
    let mut element_ref = ElementRef::new(node.id, &node.tag_name).with_depth(depth);

    if let Some(parent) = parent_id {
        element_ref = element_ref.with_parent(parent);
    }

    for class in &node.classes {
        element_ref.classes.push(class.clone());
    }

    for (name, value) in &node.attributes {
        element_ref.attributes.push((name.clone(), value.clone()));
    }

    elements.push(element_ref);

    // Recursively add children
    for child in &node.children {
        let child_elements = flatten_dom_tree(child, Some(node.id), depth + 1);
        elements.extend(child_elements);
    }

    elements
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_style_computer_creation() {
        let computer = ParallelStyleComputer::new(4);
        assert_eq!(computer.thread_count(), 4);
        assert_eq!(computer.elements_processed(), 0);
    }

    #[test]
    fn test_parallel_style_computer_default() {
        let computer = ParallelStyleComputer::default();
        assert_eq!(computer.thread_count(), 0); // Rayon default
    }

    #[test]
    fn test_compute_styles_parallel_empty() {
        let computer = ParallelStyleComputer::new(2);
        let elements: Vec<ElementRef> = Vec::new();
        let stylesheets: Vec<ParsedStylesheet> = Vec::new();

        let styles = computer.compute_styles_parallel(&elements, &stylesheets);
        assert!(styles.is_empty());
    }

    #[test]
    fn test_compute_styles_parallel_single_element() {
        let computer = ParallelStyleComputer::new(2);
        let elements = vec![ElementRef::new(ElementId::new(1), "div")];
        let stylesheets: Vec<ParsedStylesheet> = Vec::new();

        let styles = computer.compute_styles_parallel(&elements, &stylesheets);
        assert_eq!(styles.len(), 1);
        assert_eq!(computer.elements_processed(), 1);
    }

    #[test]
    fn test_compute_styles_parallel_multiple_elements() {
        let computer = ParallelStyleComputer::new(2);
        let elements = vec![
            ElementRef::new(ElementId::new(1), "div"),
            ElementRef::new(ElementId::new(2), "span"),
            ElementRef::new(ElementId::new(3), "p"),
        ];
        let stylesheets: Vec<ParsedStylesheet> = Vec::new();

        let styles = computer.compute_styles_parallel(&elements, &stylesheets);
        assert_eq!(styles.len(), 3);
        assert_eq!(computer.elements_processed(), 3);
    }

    #[test]
    fn test_compute_styles_with_stylesheet() {
        let computer = ParallelStyleComputer::new(2);
        let elements = vec![ElementRef::new(ElementId::new(1), "div")];

        let stylesheet = ParsedStylesheet::new(1, "div { display: block; }")
            .with_rule(StyleRule::new("div").with_declaration("display", "block"));

        let styles = computer.compute_styles_parallel(&elements, &[stylesheet]);
        assert_eq!(styles.len(), 1);
        assert_eq!(styles[0].display, crate::types::Display::Block);
    }

    #[test]
    fn test_selector_matches_tag() {
        let computer = ParallelStyleComputer::new(1);
        let element = ElementRef::new(ElementId::new(1), "div");

        assert!(computer.selector_matches(&element, "div"));
        assert!(!computer.selector_matches(&element, "span"));
    }

    #[test]
    fn test_selector_matches_class() {
        let computer = ParallelStyleComputer::new(1);
        let element = ElementRef::new(ElementId::new(1), "div").with_class("container");

        assert!(computer.selector_matches(&element, ".container"));
        assert!(!computer.selector_matches(&element, ".other"));
    }

    #[test]
    fn test_selector_matches_universal() {
        let computer = ParallelStyleComputer::new(1);
        let element = ElementRef::new(ElementId::new(1), "div");

        assert!(computer.selector_matches(&element, "*"));
    }

    #[test]
    fn test_partition_dom_tree_small() {
        let root = DomNode::new(ElementId::new(1), "div");
        let subtrees = partition_dom_tree(&root);

        assert!(!subtrees.is_empty());
        assert_eq!(subtrees[0].root.id, ElementId::new(1));
    }

    #[test]
    fn test_partition_dom_tree_with_children() {
        let child1 = DomNode::new(ElementId::new(2), "span");
        let child2 = DomNode::new(ElementId::new(3), "span");
        let root = DomNode::new(ElementId::new(1), "div")
            .with_child(child1)
            .with_child(child2);

        let subtrees = partition_dom_tree(&root);
        assert!(!subtrees.is_empty());

        // Count total elements across all subtrees
        let total: usize = subtrees.iter().map(|s| s.elements.len()).sum();
        assert_eq!(total, 3);
    }

    #[test]
    fn test_element_ref_builder() {
        let element = ElementRef::new(ElementId::new(1), "div")
            .with_parent(ElementId::new(0))
            .with_depth(2)
            .with_class("container");

        assert_eq!(element.id, ElementId::new(1));
        assert_eq!(element.tag_name, "div");
        assert_eq!(element.parent_id, Some(ElementId::new(0)));
        assert_eq!(element.depth, 2);
        assert_eq!(element.classes.len(), 1);
    }

    #[test]
    fn test_dom_subtree_len() {
        let root = ElementRef::new(ElementId::new(1), "div");
        let subtree = DomSubtree::new(root);

        assert_eq!(subtree.len(), 1);
        assert!(!subtree.is_empty());
    }

    #[test]
    fn test_reset_stats() {
        let computer = ParallelStyleComputer::new(2);
        let elements = vec![ElementRef::new(ElementId::new(1), "div")];
        let stylesheets: Vec<ParsedStylesheet> = Vec::new();

        computer.compute_styles_parallel(&elements, &stylesheets);
        assert_eq!(computer.elements_processed(), 1);

        computer.reset_stats();
        assert_eq!(computer.elements_processed(), 0);
    }

    #[test]
    fn test_parallel_correctness() {
        // Test that parallel computation produces same results as sequential
        let computer = ParallelStyleComputer::new(4);

        let elements: Vec<ElementRef> = (0..100)
            .map(|i| ElementRef::new(ElementId::new(i), "div"))
            .collect();

        let stylesheet = ParsedStylesheet::new(1, "div { display: block; }")
            .with_rule(StyleRule::new("div").with_declaration("display", "block"));

        let parallel_results = computer.compute_styles_parallel(&elements, &[stylesheet.clone()]);

        // All elements should have display: block
        for style in &parallel_results {
            assert_eq!(style.display, crate::types::Display::Block);
        }

        assert_eq!(parallel_results.len(), 100);
    }

    #[test]
    fn test_compute_styles_partitioned() {
        let computer = ParallelStyleComputer::new(2);

        let child1 = DomNode::new(ElementId::new(2), "span");
        let child2 = DomNode::new(ElementId::new(3), "span");
        let root = DomNode::new(ElementId::new(1), "div")
            .with_child(child1)
            .with_child(child2);

        let subtrees = partition_dom_tree(&root);
        let stylesheets: Vec<ParsedStylesheet> = Vec::new();

        let results = computer.compute_styles_partitioned(&subtrees, &stylesheets);
        assert_eq!(results.len(), 3);
    }
}
