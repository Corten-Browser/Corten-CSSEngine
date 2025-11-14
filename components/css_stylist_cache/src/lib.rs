//! CSS Stylist Cache - Style sharing and caching optimizations
//!
//! This crate provides efficient caching and style sharing for CSS style computation:
//! - StyleCache: Cache computed styles to avoid redundant computation
//! - StyleSharing: Share styles between similar elements
//! - StateFlags: Track element states for cache invalidation
//!
//! # Performance Targets
//! - Cache lookup: < 1us
//! - Sharing candidate search: < 10us
//! - Cache hit rate target: > 60%
//! - Memory overhead: < 50MB for 10,000 elements

use css_matcher_core::ElementLike;
use css_stylist_core::ComputedValues;
use servo_arc::Arc;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

// ============================================================================
// StateFlags - Element state tracking
// ============================================================================

/// Element state flags for caching
///
/// Tracks pseudo-class states that affect style computation and caching.
///
/// # Examples
/// ```
/// use css_stylist_cache::StateFlags;
///
/// let flags = StateFlags::new(true, false, false, false);
/// assert!(flags.hover());
/// assert!(!flags.active());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StateFlags {
    hover: bool,
    active: bool,
    focus: bool,
    visited: bool,
}

impl StateFlags {
    /// Create new state flags
    ///
    /// # Arguments
    /// * `hover` - Element is in hover state
    /// * `active` - Element is in active state
    /// * `focus` - Element is in focus state
    /// * `visited` - Element is a visited link
    ///
    /// # Examples
    /// ```
    /// use css_stylist_cache::StateFlags;
    ///
    /// let flags = StateFlags::new(true, false, true, false);
    /// assert!(flags.hover());
    /// assert!(flags.focus());
    /// ```
    pub fn new(hover: bool, active: bool, focus: bool, visited: bool) -> Self {
        Self {
            hover,
            active,
            focus,
            visited,
        }
    }

    /// Check if element is in hover state
    pub fn hover(&self) -> bool {
        self.hover
    }

    /// Check if element is in active state
    pub fn active(&self) -> bool {
        self.active
    }

    /// Check if element is in focus state
    pub fn focus(&self) -> bool {
        self.focus
    }

    /// Check if element is a visited link
    pub fn visited(&self) -> bool {
        self.visited
    }
}

impl Default for StateFlags {
    /// Create default state flags (all false)
    ///
    /// # Examples
    /// ```
    /// use css_stylist_cache::StateFlags;
    ///
    /// let flags = StateFlags::default();
    /// assert!(!flags.hover());
    /// assert!(!flags.active());
    /// ```
    fn default() -> Self {
        Self::new(false, false, false, false)
    }
}

// ============================================================================
// StyleKey - Cache key for style lookup
// ============================================================================

/// Key for style cache lookup
///
/// Combines selector hash, parent hash, and element state to uniquely
/// identify cached styles.
///
/// # Examples
/// ```
/// use css_stylist_cache::{StateFlags, StyleKey};
///
/// let flags = StateFlags::default();
/// let key = StyleKey::new(12345, Some(67890), flags);
/// assert_eq!(key.selector_hash(), 12345);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StyleKey {
    selector_hash: u64,
    parent_hash: Option<u64>,
    state_flags: StateFlags,
}

impl StyleKey {
    /// Create a new style key
    ///
    /// # Arguments
    /// * `selector_hash` - Hash of matching selectors
    /// * `parent_hash` - Hash of parent element (for inheritance)
    /// * `state_flags` - Element state flags
    ///
    /// # Examples
    /// ```
    /// use css_stylist_cache::{StateFlags, StyleKey};
    ///
    /// let flags = StateFlags::new(true, false, false, false);
    /// let key = StyleKey::new(100, None, flags);
    /// ```
    pub fn new(selector_hash: u64, parent_hash: Option<u64>, state_flags: StateFlags) -> Self {
        Self {
            selector_hash,
            parent_hash,
            state_flags,
        }
    }

    /// Get the selector hash
    pub fn selector_hash(&self) -> u64 {
        self.selector_hash
    }

    /// Get the parent hash
    pub fn parent_hash(&self) -> Option<u64> {
        self.parent_hash
    }

    /// Get the state flags
    pub fn state_flags(&self) -> &StateFlags {
        &self.state_flags
    }
}

// ============================================================================
// SharingKey - Key for style sharing lookup
// ============================================================================

/// Key for style sharing lookup
///
/// Used to find elements that can share computed styles.
///
/// # Examples
/// ```
/// use css_stylist_cache::SharingKey;
///
/// let key = SharingKey::new("div".to_string(), 12345, None);
/// assert_eq!(key.element_name(), "div");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SharingKey {
    element_name: String,
    class_hash: u64,
    parent_hash: Option<u64>,
}

impl SharingKey {
    /// Create a new sharing key
    ///
    /// # Arguments
    /// * `element_name` - Element tag name
    /// * `class_hash` - Hash of element classes
    /// * `parent_hash` - Hash of parent element
    ///
    /// # Examples
    /// ```
    /// use css_stylist_cache::SharingKey;
    ///
    /// let key = SharingKey::new("div".to_string(), 12345, Some(67890));
    /// ```
    pub fn new(element_name: String, class_hash: u64, parent_hash: Option<u64>) -> Self {
        Self {
            element_name,
            class_hash,
            parent_hash,
        }
    }

    /// Get the element name
    pub fn element_name(&self) -> &str {
        &self.element_name
    }

    /// Get the class hash
    pub fn class_hash(&self) -> u64 {
        self.class_hash
    }

    /// Get the parent hash
    pub fn parent_hash(&self) -> Option<u64> {
        self.parent_hash
    }
}

// ============================================================================
// StyleCache - Cache for computed styles
// ============================================================================

/// Cache for computed styles
///
/// Stores computed styles by StyleKey and tracks cache hit/miss statistics.
///
/// # Examples
/// ```
/// use css_stylist_cache::{StateFlags, StyleCache, StyleCacheManager, StyleKey};
/// use css_stylist_core::ComputedValues;
///
/// let mut cache = StyleCache::new();
/// let flags = StateFlags::default();
/// let key = StyleKey::new(123, None, flags);
/// let values = ComputedValues::default();
///
/// cache.cache_style(key.clone(), values);
/// let result = cache.get_cached_style(&key);
/// assert!(result.is_some());
/// ```
pub struct StyleCache {
    cache: HashMap<StyleKey, ComputedValues>,
    hits: u64,
    misses: u64,
}

impl StyleCache {
    /// Create a new empty style cache
    ///
    /// # Examples
    /// ```
    /// use css_stylist_cache::StyleCache;
    ///
    /// let cache = StyleCache::new();
    /// assert_eq!(cache.hits(), 0);
    /// assert_eq!(cache.misses(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            hits: 0,
            misses: 0,
        }
    }

    /// Get the number of cache hits
    pub fn hits(&self) -> u64 {
        self.hits
    }

    /// Get the number of cache misses
    pub fn misses(&self) -> u64 {
        self.misses
    }

    /// Calculate cache hit rate
    ///
    /// Returns the percentage of cache hits (0.0 to 1.0).
    ///
    /// # Examples
    /// ```
    /// use css_stylist_cache::{StateFlags, StyleCache, StyleCacheManager, StyleKey};
    /// use css_stylist_core::ComputedValues;
    ///
    /// let mut cache = StyleCache::new();
    /// let flags = StateFlags::default();
    /// let key = StyleKey::new(123, None, flags);
    /// cache.cache_style(key.clone(), ComputedValues::default());
    /// let _ = cache.get_cached_style_mut(&key); // hit (mutable version tracks stats)
    ///
    /// let hit_rate = cache.hit_rate();
    /// assert!(hit_rate > 0.0);
    /// ```
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }

    /// Clear cache statistics
    ///
    /// Resets hit and miss counters to zero.
    pub fn clear_stats(&mut self) {
        self.hits = 0;
        self.misses = 0;
    }
}

impl Default for StyleCache {
    fn default() -> Self {
        Self::new()
    }
}

impl StyleCacheManager for StyleCache {
    fn get_cached_style(&self, key: &StyleKey) -> Option<&ComputedValues> {
        let result = self.cache.get(key);

        // Update stats (requires interior mutability in real implementation)
        // For now, we'll track stats via mutable reference in cache_style
        // In production, use Cell or RefCell for interior mutability

        result
    }

    fn cache_style(&mut self, key: StyleKey, style: ComputedValues) {
        self.cache.insert(key, style);
    }

    fn find_sharing_candidate(&self, _element: &impl ElementLike) -> Option<Arc<ComputedValues>> {
        // Basic implementation - could be optimized with StyleSharing
        None
    }

    fn invalidate_cache(&mut self, _selector: Option<&str>) {
        // For now, invalidate all
        self.cache.clear();
    }
}

// Need a wrapper to track stats properly
impl StyleCache {
    /// Get cached style with statistics tracking
    ///
    /// This is a mutable version that updates hit/miss counters.
    pub fn get_cached_style_mut(&mut self, key: &StyleKey) -> Option<&ComputedValues> {
        if let Some(value) = self.cache.get(key) {
            self.hits += 1;
            Some(value)
        } else {
            self.misses += 1;
            None
        }
    }
}

// ============================================================================
// StyleSharing - Style sharing between similar elements
// ============================================================================

/// Style sharing between similar elements
///
/// Maintains a list of sharing candidates to avoid recomputing styles
/// for elements with the same characteristics.
///
/// # Examples
/// ```
/// use css_stylist_cache::StyleSharing;
///
/// let sharing = StyleSharing::new();
/// assert_eq!(sharing.candidate_count(), 0);
/// ```
pub struct StyleSharing {
    shared_styles: HashMap<SharingKey, Arc<ComputedValues>>,
    sharing_candidates: Vec<(SharingKey, Arc<ComputedValues>)>,
}

impl StyleSharing {
    /// Create a new style sharing manager
    ///
    /// # Examples
    /// ```
    /// use css_stylist_cache::StyleSharing;
    ///
    /// let sharing = StyleSharing::new();
    /// ```
    pub fn new() -> Self {
        Self {
            shared_styles: HashMap::new(),
            sharing_candidates: Vec::new(),
        }
    }

    /// Add a sharing candidate
    ///
    /// # Arguments
    /// * `element` - Element to add as candidate
    /// * `style` - Computed style for the element
    pub fn add_candidate(&mut self, element: &impl ElementLike, style: Arc<ComputedValues>) {
        let key = compute_sharing_key(element);
        self.sharing_candidates.push((key.clone(), style.clone()));
        self.shared_styles.insert(key, style);
    }

    /// Find a sharing candidate for an element
    ///
    /// # Arguments
    /// * `element` - Element to find candidate for
    ///
    /// # Returns
    /// Shared computed values if a matching candidate is found
    pub fn find_candidate(&self, element: &impl ElementLike) -> Option<Arc<ComputedValues>> {
        let key = compute_sharing_key(element);
        self.shared_styles.get(&key).cloned()
    }

    /// Get the number of sharing candidates
    pub fn candidate_count(&self) -> usize {
        self.sharing_candidates.len()
    }

    /// Clear all sharing candidates
    pub fn clear(&mut self) {
        self.shared_styles.clear();
        self.sharing_candidates.clear();
    }
}

impl Default for StyleSharing {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// StyleCacheManager Trait
// ============================================================================

/// Trait for managing style caching and sharing
///
/// Provides methods for retrieving cached styles, caching new styles,
/// finding sharing candidates, and invalidating the cache.
pub trait StyleCacheManager {
    /// Retrieve cached style if available
    ///
    /// # Arguments
    /// * `key` - Style key to lookup
    ///
    /// # Returns
    /// Cached computed values if found
    fn get_cached_style(&self, key: &StyleKey) -> Option<&ComputedValues>;

    /// Store computed style in cache
    ///
    /// # Arguments
    /// * `key` - Style key for the cached value
    /// * `style` - Computed values to cache
    fn cache_style(&mut self, key: StyleKey, style: ComputedValues);

    /// Find element that can share style
    ///
    /// # Arguments
    /// * `element` - Element to find sharing candidate for
    ///
    /// # Returns
    /// Shared computed values if a suitable candidate is found
    fn find_sharing_candidate(&self, element: &impl ElementLike) -> Option<Arc<ComputedValues>>;

    /// Invalidate cached styles
    ///
    /// # Arguments
    /// * `selector` - Optional selector to invalidate (None = invalidate all)
    fn invalidate_cache(&mut self, selector: Option<&str>);
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Determine if two elements can share styles
///
/// Elements can share styles if they have the same tag name and classes,
/// and neither has an ID (IDs make elements unique).
///
/// # Arguments
/// * `elem1` - First element
/// * `elem2` - Second element
///
/// # Returns
/// true if elements can share styles
///
/// # Examples
/// ```
/// use css_stylist_cache::can_share_style;
/// use css_matcher_core::ElementLike;
///
/// # struct MockElement { tag: String, id: Option<String>, classes: Vec<String> }
/// # impl ElementLike for MockElement {
/// #     fn tag_name(&self) -> &str { &self.tag }
/// #     fn id(&self) -> Option<&str> { self.id.as_deref() }
/// #     fn classes(&self) -> &[String] { &self.classes }
/// #     fn parent(&self) -> Option<&Self> { None }
/// #     fn previous_sibling(&self) -> Option<&Self> { None }
/// # }
///
/// let elem1 = MockElement {
///     tag: "div".to_string(),
///     id: None,
///     classes: vec!["foo".to_string()],
/// };
/// let elem2 = MockElement {
///     tag: "div".to_string(),
///     id: None,
///     classes: vec!["foo".to_string()],
/// };
///
/// assert!(can_share_style(&elem1, &elem2));
/// ```
pub fn can_share_style(elem1: &impl ElementLike, elem2: &impl ElementLike) -> bool {
    // Elements with IDs cannot share styles (IDs should be unique)
    if elem1.id().is_some() || elem2.id().is_some() {
        return false;
    }

    // Must have same tag name
    if elem1.tag_name() != elem2.tag_name() {
        return false;
    }

    // Must have same classes
    if elem1.classes() != elem2.classes() {
        return false;
    }

    true
}

/// Compute cache key for element
///
/// Creates a StyleKey based on the element's characteristics.
///
/// # Arguments
/// * `element` - Element to compute key for
///
/// # Returns
/// StyleKey for caching
///
/// # Examples
/// ```
/// use css_stylist_cache::compute_style_key;
/// use css_matcher_core::ElementLike;
///
/// # struct MockElement { tag: String, classes: Vec<String> }
/// # impl ElementLike for MockElement {
/// #     fn tag_name(&self) -> &str { &self.tag }
/// #     fn id(&self) -> Option<&str> { None }
/// #     fn classes(&self) -> &[String] { &self.classes }
/// #     fn parent(&self) -> Option<&Self> { None }
/// #     fn previous_sibling(&self) -> Option<&Self> { None }
/// # }
///
/// let elem = MockElement {
///     tag: "div".to_string(),
///     classes: vec!["foo".to_string()],
/// };
///
/// let key = compute_style_key(&elem);
/// assert_ne!(key.selector_hash(), 0);
/// ```
pub fn compute_style_key(element: &impl ElementLike) -> StyleKey {
    use std::collections::hash_map::DefaultHasher;

    let mut hasher = DefaultHasher::new();

    // Hash element tag
    element.tag_name().hash(&mut hasher);

    // Hash element ID if present
    if let Some(id) = element.id() {
        id.hash(&mut hasher);
    }

    // Hash classes
    for class in element.classes() {
        class.hash(&mut hasher);
    }

    let selector_hash = hasher.finish();

    // For now, no parent hash (would require parent access)
    let parent_hash = None;

    // Default state flags (no pseudo-class state)
    let state_flags = StateFlags::default();

    StyleKey::new(selector_hash, parent_hash, state_flags)
}

/// Compute sharing key for element
///
/// Internal helper to create a SharingKey from an element.
fn compute_sharing_key(element: &impl ElementLike) -> SharingKey {
    use std::collections::hash_map::DefaultHasher;

    let mut hasher = DefaultHasher::new();

    // Hash classes
    for class in element.classes() {
        class.hash(&mut hasher);
    }

    let class_hash = hasher.finish();
    let element_name = element.tag_name().to_string();

    // For now, no parent hash
    let parent_hash = None;

    SharingKey::new(element_name, class_hash, parent_hash)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_flags_basic() {
        let flags = StateFlags::new(true, false, true, false);
        assert!(flags.hover());
        assert!(!flags.active());
        assert!(flags.focus());
        assert!(!flags.visited());
    }

    #[test]
    fn test_style_key_basic() {
        let flags = StateFlags::default();
        let key = StyleKey::new(123, None, flags);
        assert_eq!(key.selector_hash(), 123);
    }

    #[test]
    fn test_sharing_key_basic() {
        let key = SharingKey::new("div".to_string(), 456, None);
        assert_eq!(key.element_name(), "div");
        assert_eq!(key.class_hash(), 456);
    }

    #[test]
    fn test_style_cache_basic() {
        let cache = StyleCache::new();
        assert_eq!(cache.hits(), 0);
        assert_eq!(cache.misses(), 0);
    }
}
