//! Unit tests for StyleKey

use css_stylist_cache::{StateFlags, StyleKey};

#[test]
fn test_style_key_new() {
    let flags = StateFlags::new(true, false, false, false);
    let key = StyleKey::new(12345, Some(67890), flags);

    assert_eq!(key.selector_hash(), 12345);
    assert_eq!(key.parent_hash(), Some(67890));
    assert_eq!(key.state_flags(), &flags);
}

#[test]
fn test_style_key_no_parent() {
    let flags = StateFlags::default();
    let key = StyleKey::new(12345, None, flags);

    assert_eq!(key.selector_hash(), 12345);
    assert_eq!(key.parent_hash(), None);
}

#[test]
fn test_style_key_equality() {
    let flags1 = StateFlags::new(true, false, false, false);
    let flags2 = StateFlags::new(true, false, false, false);
    let flags3 = StateFlags::new(false, false, false, false);

    let key1 = StyleKey::new(100, Some(200), flags1);
    let key2 = StyleKey::new(100, Some(200), flags2);
    let key3 = StyleKey::new(100, Some(200), flags3);

    assert_eq!(key1, key2);
    assert_ne!(key1, key3);
}

#[test]
fn test_style_key_hash_consistency() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let flags = StateFlags::new(true, false, false, false);
    let key1 = StyleKey::new(100, Some(200), flags.clone());
    let key2 = StyleKey::new(100, Some(200), flags);

    let mut hasher1 = DefaultHasher::new();
    let mut hasher2 = DefaultHasher::new();

    key1.hash(&mut hasher1);
    key2.hash(&mut hasher2);

    assert_eq!(hasher1.finish(), hasher2.finish());
}

#[test]
fn test_style_key_different_hashes() {
    let flags = StateFlags::default();
    let key1 = StyleKey::new(100, Some(200), flags.clone());
    let key2 = StyleKey::new(101, Some(200), flags.clone());
    let key3 = StyleKey::new(100, Some(201), flags);

    assert_ne!(key1, key2);
    assert_ne!(key1, key3);
}
