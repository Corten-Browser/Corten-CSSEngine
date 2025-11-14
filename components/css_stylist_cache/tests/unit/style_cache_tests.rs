//! Unit tests for StyleCache

use css_stylist_cache::{StateFlags, StyleCache, StyleCacheManager, StyleKey};
use css_stylist_core::ComputedValues;

#[test]
fn test_style_cache_new() {
    let cache = StyleCache::new();

    assert_eq!(cache.hits(), 0);
    assert_eq!(cache.misses(), 0);
}

#[test]
fn test_style_cache_get_cached_style_miss() {
    let cache = StyleCache::new();
    let flags = StateFlags::default();
    let key = StyleKey::new(123, None, flags);

    let result = cache.get_cached_style(&key);

    assert!(result.is_none());
}

#[test]
fn test_style_cache_cache_and_retrieve() {
    let mut cache = StyleCache::new();
    let flags = StateFlags::default();
    let key = StyleKey::new(123, None, flags);
    let values = ComputedValues::default();

    cache.cache_style(key.clone(), values.clone());

    let result = cache.get_cached_style(&key);
    assert!(result.is_some());
    assert_eq!(*result.unwrap(), values);
}

#[test]
fn test_style_cache_hit_tracking() {
    let mut cache = StyleCache::new();
    let flags = StateFlags::default();
    let key = StyleKey::new(123, None, flags);
    let values = ComputedValues::default();

    // Initial state
    assert_eq!(cache.hits(), 0);
    assert_eq!(cache.misses(), 0);

    // First access - cache the value
    cache.cache_style(key.clone(), values);

    // Second access - hit (use mutable version for stats tracking)
    let _ = cache.get_cached_style_mut(&key);
    assert_eq!(cache.hits(), 1);
    assert_eq!(cache.misses(), 0);
}

#[test]
fn test_style_cache_miss_tracking() {
    let mut cache = StyleCache::new();
    let flags = StateFlags::default();
    let key = StyleKey::new(123, None, flags);

    // Access non-existent key (use mutable version for stats tracking)
    let result = cache.get_cached_style_mut(&key);

    assert!(result.is_none());
    assert_eq!(cache.hits(), 0);
    assert_eq!(cache.misses(), 1);
}

#[test]
fn test_style_cache_multiple_keys() {
    let mut cache = StyleCache::new();
    let flags1 = StateFlags::new(true, false, false, false);
    let flags2 = StateFlags::new(false, true, false, false);

    let key1 = StyleKey::new(100, None, flags1);
    let key2 = StyleKey::new(200, None, flags2);

    let values1 = ComputedValues::default();
    let mut values2 = ComputedValues::default();
    values2.display = css_stylist_core::Display::Block;

    cache.cache_style(key1.clone(), values1.clone());
    cache.cache_style(key2.clone(), values2.clone());

    let result1 = cache.get_cached_style(&key1);
    let result2 = cache.get_cached_style(&key2);

    assert!(result1.is_some());
    assert!(result2.is_some());
    assert_eq!(result1.unwrap().display, css_stylist_core::Display::Inline);
    assert_eq!(result2.unwrap().display, css_stylist_core::Display::Block);
}

#[test]
fn test_style_cache_invalidate_all() {
    let mut cache = StyleCache::new();
    let flags = StateFlags::default();
    let key = StyleKey::new(123, None, flags);
    let values = ComputedValues::default();

    cache.cache_style(key.clone(), values);

    // Invalidate all
    cache.invalidate_cache(None);

    let result = cache.get_cached_style(&key);
    assert!(result.is_none());
}

#[test]
fn test_style_cache_hit_rate() {
    let mut cache = StyleCache::new();
    let flags = StateFlags::default();
    let key = StyleKey::new(123, None, flags);
    let values = ComputedValues::default();

    cache.cache_style(key.clone(), values);

    // 1 hit (use mutable version for stats tracking)
    let _ = cache.get_cached_style_mut(&key);

    // 2 misses (use mutable version for stats tracking)
    let key2 = StyleKey::new(456, None, flags.clone());
    let _ = cache.get_cached_style_mut(&key2);
    let _ = cache.get_cached_style_mut(&key2);

    assert_eq!(cache.hits(), 1);
    assert_eq!(cache.misses(), 2);

    let hit_rate = cache.hit_rate();
    assert!((hit_rate - 0.333).abs() < 0.01); // 1/3 ≈ 0.333
}

#[test]
fn test_style_cache_hit_rate_no_accesses() {
    let cache = StyleCache::new();
    assert_eq!(cache.hit_rate(), 0.0);
}

#[test]
fn test_style_cache_clear_stats() {
    let mut cache = StyleCache::new();
    let flags = StateFlags::default();
    let key = StyleKey::new(123, None, flags);
    let values = ComputedValues::default();

    cache.cache_style(key.clone(), values);
    let _ = cache.get_cached_style_mut(&key); // use mutable version for stats tracking

    assert_eq!(cache.hits(), 1);

    cache.clear_stats();

    assert_eq!(cache.hits(), 0);
    assert_eq!(cache.misses(), 0);
}
