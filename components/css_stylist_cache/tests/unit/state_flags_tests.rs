//! Unit tests for StateFlags

use css_stylist_cache::StateFlags;

#[test]
fn test_state_flags_default() {
    let flags = StateFlags::default();

    assert!(!flags.hover());
    assert!(!flags.active());
    assert!(!flags.focus());
    assert!(!flags.visited());
}

#[test]
fn test_state_flags_new() {
    let flags = StateFlags::new(true, false, true, false);

    assert!(flags.hover());
    assert!(!flags.active());
    assert!(flags.focus());
    assert!(!flags.visited());
}

#[test]
fn test_state_flags_equality() {
    let flags1 = StateFlags::new(true, true, false, false);
    let flags2 = StateFlags::new(true, true, false, false);
    let flags3 = StateFlags::new(false, true, false, false);

    assert_eq!(flags1, flags2);
    assert_ne!(flags1, flags3);
}

#[test]
fn test_state_flags_hash_consistency() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let flags1 = StateFlags::new(true, false, true, false);
    let flags2 = StateFlags::new(true, false, true, false);

    let mut hasher1 = DefaultHasher::new();
    let mut hasher2 = DefaultHasher::new();

    flags1.hash(&mut hasher1);
    flags2.hash(&mut hasher2);

    assert_eq!(hasher1.finish(), hasher2.finish());
}

#[test]
fn test_state_flags_clone() {
    let flags = StateFlags::new(true, true, true, false);
    let cloned = flags.clone();

    assert_eq!(flags, cloned);
}
