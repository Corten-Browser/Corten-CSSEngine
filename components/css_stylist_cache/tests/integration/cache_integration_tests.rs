//! Integration tests for caching scenarios

use css_matcher_core::ElementLike;
use css_stylist_cache::{
    can_share_style, compute_style_key, StateFlags, StyleCache, StyleCacheManager, StyleKey,
    StyleSharing,
};
use css_stylist_core::ComputedValues;
use servo_arc::Arc;

// Mock element for testing
#[derive(Clone)]
struct TestElement {
    tag: String,
    id: Option<String>,
    classes: Vec<String>,
}

impl ElementLike for TestElement {
    fn tag_name(&self) -> &str {
        &self.tag
    }

    fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    fn classes(&self) -> &[String] {
        &self.classes
    }

    fn parent(&self) -> Option<&Self> {
        None
    }

    fn previous_sibling(&self) -> Option<&Self> {
        None
    }
}

#[test]
fn test_cache_workflow() {
    let mut cache = StyleCache::new();

    // Create test elements
    let elem1 = TestElement {
        tag: "div".to_string(),
        id: None,
        classes: vec!["content".to_string()],
    };

    let elem2 = TestElement {
        tag: "div".to_string(),
        id: None,
        classes: vec!["content".to_string()],
    };

    // Compute keys
    let key1 = compute_style_key(&elem1);
    let key2 = compute_style_key(&elem2);

    // Same elements should have same key
    assert_eq!(key1, key2);

    // First access - miss
    assert!(cache.get_cached_style_mut(&key1).is_none());
    assert_eq!(cache.misses(), 1);

    // Cache a value
    let values = ComputedValues::default();
    cache.cache_style(key1.clone(), values.clone());

    // Second access - hit
    let cached = cache.get_cached_style_mut(&key1);
    assert!(cached.is_some());
    assert_eq!(*cached.unwrap(), values);

    // Check hit count
    assert_eq!(cache.hits(), 1);
}

#[test]
fn test_style_sharing_workflow() {
    let mut sharing = StyleSharing::new();

    let elem1 = TestElement {
        tag: "div".to_string(),
        id: None,
        classes: vec!["button".to_string()],
    };

    let elem2 = TestElement {
        tag: "div".to_string(),
        id: None,
        classes: vec!["button".to_string()],
    };

    let elem3 = TestElement {
        tag: "span".to_string(),
        id: None,
        classes: vec!["button".to_string()],
    };

    // Elements can share if same tag and classes
    assert!(can_share_style(&elem1, &elem2));
    assert!(!can_share_style(&elem1, &elem3));

    // Add elem1 as candidate
    let values = Arc::new(ComputedValues::default());
    sharing.add_candidate(&elem1, values.clone());

    // elem2 can share with elem1
    let shared = sharing.find_candidate(&elem2);
    assert!(shared.is_some());
    assert_eq!(*shared.unwrap(), *values);

    // elem3 cannot share (different tag)
    let not_shared = sharing.find_candidate(&elem3);
    assert!(not_shared.is_none());
}

#[test]
fn test_cache_invalidation() {
    let mut cache = StyleCache::new();
    let flags = StateFlags::default();
    let key = StyleKey::new(100, None, flags);
    let values = ComputedValues::default();

    // Cache a value
    cache.cache_style(key.clone(), values);
    assert!(cache.get_cached_style(&key).is_some());

    // Invalidate cache
    cache.invalidate_cache(None);

    // Value should be gone
    assert!(cache.get_cached_style(&key).is_none());
}

#[test]
fn test_state_dependent_caching() {
    let mut cache = StyleCache::new();

    let flags_normal = StateFlags::new(false, false, false, false);
    let flags_hover = StateFlags::new(true, false, false, false);

    let key_normal = StyleKey::new(100, None, flags_normal);
    let key_hover = StyleKey::new(100, None, flags_hover);

    // Keys should be different even with same selector hash
    assert_ne!(key_normal, key_hover);

    // Cache different values for different states
    let values_normal = ComputedValues::default();
    let mut values_hover = ComputedValues::default();
    values_hover.display = css_stylist_core::Display::Block;

    cache.cache_style(key_normal.clone(), values_normal.clone());
    cache.cache_style(key_hover.clone(), values_hover.clone());

    // Retrieve correct value based on state
    let normal_cached = cache.get_cached_style(&key_normal);
    let hover_cached = cache.get_cached_style(&key_hover);

    assert!(normal_cached.is_some());
    assert!(hover_cached.is_some());
    assert_eq!(normal_cached.unwrap().display, values_normal.display);
    assert_eq!(hover_cached.unwrap().display, values_hover.display);
}

#[test]
fn test_cache_hit_rate_tracking() {
    let mut cache = StyleCache::new();
    let flags = StateFlags::default();

    // Create 3 different keys
    let keys: Vec<StyleKey> = (0..3)
        .map(|i| StyleKey::new(i * 100, None, flags.clone()))
        .collect();

    // Cache values for first 2 keys
    for key in &keys[0..2] {
        cache.cache_style(key.clone(), ComputedValues::default());
    }

    // Access pattern: hit, hit, miss, hit, miss, miss
    let _ = cache.get_cached_style_mut(&keys[0]); // hit
    let _ = cache.get_cached_style_mut(&keys[1]); // hit
    let _ = cache.get_cached_style_mut(&keys[2]); // miss
    let _ = cache.get_cached_style_mut(&keys[0]); // hit
    let _ = cache.get_cached_style_mut(&keys[2]); // miss
    let _ = cache.get_cached_style_mut(&keys[2]); // miss

    assert_eq!(cache.hits(), 3);
    assert_eq!(cache.misses(), 3);
    assert_eq!(cache.hit_rate(), 0.5); // 3/6 = 0.5
}

#[test]
fn test_parent_hash_in_cache_key() {
    let flags = StateFlags::default();

    let key_no_parent = StyleKey::new(100, None, flags.clone());
    let key_with_parent = StyleKey::new(100, Some(200), flags);

    // Should be different keys
    assert_ne!(key_no_parent, key_with_parent);
}
