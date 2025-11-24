//! Bloom Filter for Fast Selector Rejection
//!
//! A bloom filter is a probabilistic data structure that can quickly determine
//! if an element DEFINITELY DOES NOT match a selector. It may have false positives
//! (saying something might match when it doesn't), but never false negatives.
//!
//! # Use Case
//!
//! When matching selectors against elements, we can first check the bloom filter.
//! If it says "definitely not a match", we can skip the expensive full matching.
//! This is especially useful for:
//! - Descendant selectors (`.foo .bar`) - we can skip checking ancestors
//! - Complex selectors with many components
//!
//! # Design
//!
//! This implementation uses a 256-bit filter (4 x u64) which provides:
//! - Fast operations (bitwise operations on u64)
//! - Reasonable false positive rate for typical CSS (< 5% at ~20 elements)
//! - Can be passed up the DOM tree for ancestor matching
//!
//! # Algorithm
//!
//! We use two hash functions derived from a single hash (double hashing):
//! - h1(x) = hash(x) & 0xFF (lower 8 bits)
//! - h2(x) = (hash(x) >> 8) & 0xFF (next 8 bits)
//!
//! For each hash, we set/check the corresponding bit in the 256-bit array.

use css_matcher_core::{Component, Selector};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// A 256-bit bloom filter for selector matching optimization
///
/// The filter tracks IDs, classes, and tag names that have been added.
/// When checking a selector, if ANY required component is definitely not
/// present, we can skip the full selector matching.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SelectorBloomFilter {
    /// 256 bits stored as 4 u64 values
    bits: [u64; 4],
}

impl SelectorBloomFilter {
    /// Create a new empty bloom filter
    ///
    /// # Examples
    ///
    /// ```
    /// use css_stylist_core::bloom_filter::SelectorBloomFilter;
    ///
    /// let filter = SelectorBloomFilter::new();
    /// assert!(filter.is_empty());
    /// ```
    pub fn new() -> Self {
        Self { bits: [0; 4] }
    }

    /// Check if the filter is empty (no items added)
    pub fn is_empty(&self) -> bool {
        self.bits == [0; 4]
    }

    /// Clear the filter
    pub fn clear(&mut self) {
        self.bits = [0; 4];
    }

    /// Add an ID to the filter
    ///
    /// # Examples
    ///
    /// ```
    /// use css_stylist_core::bloom_filter::SelectorBloomFilter;
    ///
    /// let mut filter = SelectorBloomFilter::new();
    /// filter.add_id("header");
    /// assert!(!filter.is_empty());
    /// ```
    pub fn add_id(&mut self, id: &str) {
        self.add_hash(Self::hash_id(id));
    }

    /// Add a class to the filter
    ///
    /// # Examples
    ///
    /// ```
    /// use css_stylist_core::bloom_filter::SelectorBloomFilter;
    ///
    /// let mut filter = SelectorBloomFilter::new();
    /// filter.add_class("container");
    /// filter.add_class("primary");
    /// ```
    pub fn add_class(&mut self, class: &str) {
        self.add_hash(Self::hash_class(class));
    }

    /// Add a tag name to the filter
    ///
    /// # Examples
    ///
    /// ```
    /// use css_stylist_core::bloom_filter::SelectorBloomFilter;
    ///
    /// let mut filter = SelectorBloomFilter::new();
    /// filter.add_tag("div");
    /// filter.add_tag("span");
    /// ```
    pub fn add_tag(&mut self, tag: &str) {
        self.add_hash(Self::hash_tag(tag));
    }

    /// Check if a selector might match based on the filter
    ///
    /// Returns `true` if the selector MIGHT match (needs full check).
    /// Returns `false` if the selector DEFINITELY DOES NOT match.
    ///
    /// # Examples
    ///
    /// ```
    /// use css_stylist_core::bloom_filter::SelectorBloomFilter;
    /// use css_matcher_core::{Selector, Component};
    ///
    /// let mut filter = SelectorBloomFilter::new();
    /// filter.add_class("foo");
    /// filter.add_tag("div");
    ///
    /// // This might match (class is present)
    /// let selector1 = Selector::with_components(vec![Component::Class("foo".to_string())]);
    /// assert!(filter.might_match(&selector1));
    ///
    /// // This definitely does not match (class not present)
    /// let selector2 = Selector::with_components(vec![Component::Class("bar".to_string())]);
    /// assert!(!filter.might_match(&selector2));
    /// ```
    pub fn might_match(&self, selector: &Selector) -> bool {
        for component in &selector.components {
            match component {
                Component::Id(id) => {
                    if !self.might_contain_id(id) {
                        return false;
                    }
                }
                Component::Class(class) => {
                    if !self.might_contain_class(class) {
                        return false;
                    }
                }
                Component::Tag(tag) => {
                    if !self.might_contain_tag(tag) {
                        return false;
                    }
                }
                Component::Universal => {
                    // Universal always matches, continue checking
                }
            }
        }
        true
    }

    /// Check if a selector definitely does not match
    ///
    /// This is the complement of `might_match`. Returns `true` if we can
    /// definitively say the selector will not match.
    ///
    /// # Examples
    ///
    /// ```
    /// use css_stylist_core::bloom_filter::SelectorBloomFilter;
    /// use css_matcher_core::{Selector, Component};
    ///
    /// let mut filter = SelectorBloomFilter::new();
    /// filter.add_class("foo");
    ///
    /// let selector = Selector::with_components(vec![Component::Class("bar".to_string())]);
    /// assert!(filter.definitely_not_match(&selector));
    /// ```
    pub fn definitely_not_match(&self, selector: &Selector) -> bool {
        !self.might_match(selector)
    }

    /// Check if an ID might be present
    pub fn might_contain_id(&self, id: &str) -> bool {
        self.might_contain_hash(Self::hash_id(id))
    }

    /// Check if a class might be present
    pub fn might_contain_class(&self, class: &str) -> bool {
        self.might_contain_hash(Self::hash_class(class))
    }

    /// Check if a tag might be present
    pub fn might_contain_tag(&self, tag: &str) -> bool {
        self.might_contain_hash(Self::hash_tag(tag))
    }

    /// Merge another filter into this one (union)
    ///
    /// This is useful for building ancestor filters when walking up the DOM.
    ///
    /// # Examples
    ///
    /// ```
    /// use css_stylist_core::bloom_filter::SelectorBloomFilter;
    ///
    /// let mut filter1 = SelectorBloomFilter::new();
    /// filter1.add_class("foo");
    ///
    /// let mut filter2 = SelectorBloomFilter::new();
    /// filter2.add_class("bar");
    ///
    /// filter1.merge(&filter2);
    /// assert!(filter1.might_contain_class("foo"));
    /// assert!(filter1.might_contain_class("bar"));
    /// ```
    pub fn merge(&mut self, other: &SelectorBloomFilter) {
        for i in 0..4 {
            self.bits[i] |= other.bits[i];
        }
    }

    /// Create a filter that is the union of two filters
    pub fn union(&self, other: &SelectorBloomFilter) -> SelectorBloomFilter {
        let mut result = *self;
        result.merge(other);
        result
    }

    /// Get the number of bits set (for debugging/statistics)
    pub fn popcount(&self) -> u32 {
        self.bits.iter().map(|b| b.count_ones()).sum()
    }

    /// Get the approximate fill ratio (0.0 to 1.0)
    pub fn fill_ratio(&self) -> f64 {
        self.popcount() as f64 / 256.0
    }

    // Internal methods

    fn add_hash(&mut self, hash: u64) {
        let (h1, h2) = Self::double_hash(hash);
        self.set_bit(h1);
        self.set_bit(h2);
    }

    fn might_contain_hash(&self, hash: u64) -> bool {
        let (h1, h2) = Self::double_hash(hash);
        self.get_bit(h1) && self.get_bit(h2)
    }

    fn set_bit(&mut self, bit: u8) {
        let index = (bit / 64) as usize;
        let offset = bit % 64;
        self.bits[index] |= 1 << offset;
    }

    fn get_bit(&self, bit: u8) -> bool {
        let index = (bit / 64) as usize;
        let offset = bit % 64;
        (self.bits[index] & (1 << offset)) != 0
    }

    /// Double hashing: derive two hash values from one
    fn double_hash(hash: u64) -> (u8, u8) {
        let h1 = (hash & 0xFF) as u8;
        let h2 = ((hash >> 8) & 0xFF) as u8;
        (h1, h2)
    }

    /// Hash an ID (with type prefix to avoid collisions between types)
    fn hash_id(id: &str) -> u64 {
        Self::hash_with_prefix(b"id:", id)
    }

    /// Hash a class
    fn hash_class(class: &str) -> u64 {
        Self::hash_with_prefix(b"class:", class)
    }

    /// Hash a tag
    fn hash_tag(tag: &str) -> u64 {
        Self::hash_with_prefix(b"tag:", tag)
    }

    fn hash_with_prefix(prefix: &[u8], value: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        prefix.hash(&mut hasher);
        value.hash(&mut hasher);
        hasher.finish()
    }
}

/// Builder for constructing bloom filters from element hierarchies
///
/// This builder helps construct ancestor bloom filters by collecting
/// all IDs, classes, and tags from ancestor elements.
#[derive(Debug, Clone, Default)]
pub struct BloomFilterBuilder {
    filter: SelectorBloomFilter,
    element_count: usize,
}

impl BloomFilterBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            filter: SelectorBloomFilter::new(),
            element_count: 0,
        }
    }

    /// Add an element's attributes to the filter
    pub fn add_element(&mut self, id: Option<&str>, classes: &[String], tag: &str) {
        if let Some(id) = id {
            self.filter.add_id(id);
        }
        for class in classes {
            self.filter.add_class(class);
        }
        self.filter.add_tag(tag);
        self.element_count += 1;
    }

    /// Get the current element count
    pub fn element_count(&self) -> usize {
        self.element_count
    }

    /// Build the final filter
    pub fn build(self) -> SelectorBloomFilter {
        self.filter
    }

    /// Get a reference to the current filter
    pub fn filter(&self) -> &SelectorBloomFilter {
        &self.filter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_filter_is_empty() {
        let filter = SelectorBloomFilter::new();
        assert!(filter.is_empty());
        assert_eq!(filter.popcount(), 0);
    }

    #[test]
    fn test_add_id() {
        let mut filter = SelectorBloomFilter::new();
        filter.add_id("header");

        assert!(!filter.is_empty());
        assert!(filter.might_contain_id("header"));
    }

    #[test]
    fn test_add_class() {
        let mut filter = SelectorBloomFilter::new();
        filter.add_class("container");

        assert!(filter.might_contain_class("container"));
        // Should not contain other classes (with high probability)
        // Note: This could theoretically fail due to hash collision
    }

    #[test]
    fn test_add_tag() {
        let mut filter = SelectorBloomFilter::new();
        filter.add_tag("div");

        assert!(filter.might_contain_tag("div"));
    }

    #[test]
    fn test_clear() {
        let mut filter = SelectorBloomFilter::new();
        filter.add_class("foo");
        assert!(!filter.is_empty());

        filter.clear();
        assert!(filter.is_empty());
    }

    #[test]
    fn test_might_match_class_selector() {
        let mut filter = SelectorBloomFilter::new();
        filter.add_class("foo");
        filter.add_class("bar");
        filter.add_tag("div");

        // Should match selector for class "foo"
        let selector1 = Selector::with_components(vec![Component::Class("foo".to_string())]);
        assert!(filter.might_match(&selector1));

        // Should not match selector for class "baz" (not added)
        let selector2 = Selector::with_components(vec![Component::Class("baz".to_string())]);
        assert!(filter.definitely_not_match(&selector2));
    }

    #[test]
    fn test_might_match_compound_selector() {
        let mut filter = SelectorBloomFilter::new();
        filter.add_class("foo");
        filter.add_tag("div");

        // Should match div.foo (both present)
        let selector1 = Selector::with_components(vec![
            Component::Tag("div".to_string()),
            Component::Class("foo".to_string()),
        ]);
        assert!(filter.might_match(&selector1));

        // Should not match span.foo (span not present)
        let selector2 = Selector::with_components(vec![
            Component::Tag("span".to_string()),
            Component::Class("foo".to_string()),
        ]);
        assert!(filter.definitely_not_match(&selector2));
    }

    #[test]
    fn test_might_match_id_selector() {
        let mut filter = SelectorBloomFilter::new();
        filter.add_id("header");

        let selector1 = Selector::with_components(vec![Component::Id("header".to_string())]);
        assert!(filter.might_match(&selector1));

        let selector2 = Selector::with_components(vec![Component::Id("footer".to_string())]);
        assert!(filter.definitely_not_match(&selector2));
    }

    #[test]
    fn test_universal_always_matches() {
        let filter = SelectorBloomFilter::new();
        let selector = Selector::with_components(vec![Component::Universal]);
        assert!(filter.might_match(&selector));
    }

    #[test]
    fn test_merge() {
        let mut filter1 = SelectorBloomFilter::new();
        filter1.add_class("foo");

        let mut filter2 = SelectorBloomFilter::new();
        filter2.add_class("bar");

        filter1.merge(&filter2);

        assert!(filter1.might_contain_class("foo"));
        assert!(filter1.might_contain_class("bar"));
    }

    #[test]
    fn test_union() {
        let mut filter1 = SelectorBloomFilter::new();
        filter1.add_class("foo");

        let mut filter2 = SelectorBloomFilter::new();
        filter2.add_class("bar");

        let union = filter1.union(&filter2);

        assert!(union.might_contain_class("foo"));
        assert!(union.might_contain_class("bar"));

        // Original filters unchanged
        assert!(!filter1.might_contain_class("bar"));
        assert!(!filter2.might_contain_class("foo"));
    }

    #[test]
    fn test_fill_ratio() {
        let mut filter = SelectorBloomFilter::new();
        assert_eq!(filter.fill_ratio(), 0.0);

        filter.add_class("foo");
        assert!(filter.fill_ratio() > 0.0);
        assert!(filter.fill_ratio() < 0.1); // Should be small with just one item
    }

    #[test]
    fn test_default() {
        let filter = SelectorBloomFilter::default();
        assert!(filter.is_empty());
    }

    #[test]
    fn test_copy() {
        let mut filter1 = SelectorBloomFilter::new();
        filter1.add_class("foo");

        let filter2 = filter1;
        assert!(filter2.might_contain_class("foo"));
    }

    #[test]
    fn test_builder_new() {
        let builder = BloomFilterBuilder::new();
        assert_eq!(builder.element_count(), 0);
        assert!(builder.filter().is_empty());
    }

    #[test]
    fn test_builder_add_element() {
        let mut builder = BloomFilterBuilder::new();
        builder.add_element(
            Some("header"),
            &["container".to_string(), "primary".to_string()],
            "div",
        );

        assert_eq!(builder.element_count(), 1);
        let filter = builder.filter();
        assert!(filter.might_contain_id("header"));
        assert!(filter.might_contain_class("container"));
        assert!(filter.might_contain_class("primary"));
        assert!(filter.might_contain_tag("div"));
    }

    #[test]
    fn test_builder_multiple_elements() {
        let mut builder = BloomFilterBuilder::new();
        builder.add_element(None, &["foo".to_string()], "div");
        builder.add_element(Some("main"), &["bar".to_string()], "section");

        assert_eq!(builder.element_count(), 2);

        let filter = builder.build();
        assert!(filter.might_contain_class("foo"));
        assert!(filter.might_contain_class("bar"));
        assert!(filter.might_contain_tag("div"));
        assert!(filter.might_contain_tag("section"));
        assert!(filter.might_contain_id("main"));
    }

    #[test]
    fn test_no_false_negatives() {
        // This is the critical property of bloom filters:
        // If we add something, it must always be found
        let mut filter = SelectorBloomFilter::new();

        let items = vec![
            ("id1", "class1", "div"),
            ("id2", "class2", "span"),
            ("id3", "class3", "p"),
            ("id4", "class4", "a"),
            ("id5", "class5", "section"),
        ];

        for (id, class, tag) in &items {
            filter.add_id(id);
            filter.add_class(class);
            filter.add_tag(tag);
        }

        for (id, class, tag) in &items {
            assert!(filter.might_contain_id(id), "ID {} should be found", id);
            assert!(
                filter.might_contain_class(class),
                "Class {} should be found",
                class
            );
            assert!(filter.might_contain_tag(tag), "Tag {} should be found", tag);
        }
    }

    #[test]
    fn test_different_types_dont_collide() {
        // Adding "foo" as an ID should not match "foo" as a class
        let mut filter = SelectorBloomFilter::new();
        filter.add_id("foo");

        // Due to type prefixes, class "foo" should not be present
        // Note: With a small filter, there's a small chance of collision
        // This test verifies the prefix mechanism works
        let id_hash = SelectorBloomFilter::hash_id("foo");
        let class_hash = SelectorBloomFilter::hash_class("foo");
        assert_ne!(
            id_hash, class_hash,
            "Different types should have different hashes"
        );
    }
}
