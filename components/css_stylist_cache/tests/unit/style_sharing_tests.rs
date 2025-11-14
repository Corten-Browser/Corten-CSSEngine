//! Unit tests for StyleSharing and can_share_style

use css_matcher_core::ElementLike;
use css_stylist_cache::{can_share_style, compute_style_key, StateFlags, StyleSharing};
use css_stylist_core::ComputedValues;
use servo_arc::Arc;

// Mock element for testing
#[derive(Clone)]
struct MockElement {
    tag: String,
    id: Option<String>,
    classes: Vec<String>,
}

impl ElementLike for MockElement {
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
fn test_can_share_style_same_tag_no_classes() {
    let elem1 = MockElement {
        tag: "div".to_string(),
        id: None,
        classes: vec![],
    };
    let elem2 = MockElement {
        tag: "div".to_string(),
        id: None,
        classes: vec![],
    };

    assert!(can_share_style(&elem1, &elem2));
}

#[test]
fn test_can_share_style_different_tags() {
    let elem1 = MockElement {
        tag: "div".to_string(),
        id: None,
        classes: vec![],
    };
    let elem2 = MockElement {
        tag: "span".to_string(),
        id: None,
        classes: vec![],
    };

    assert!(!can_share_style(&elem1, &elem2));
}

#[test]
fn test_can_share_style_same_classes() {
    let elem1 = MockElement {
        tag: "div".to_string(),
        id: None,
        classes: vec!["foo".to_string(), "bar".to_string()],
    };
    let elem2 = MockElement {
        tag: "div".to_string(),
        id: None,
        classes: vec!["foo".to_string(), "bar".to_string()],
    };

    assert!(can_share_style(&elem1, &elem2));
}

#[test]
fn test_can_share_style_different_classes() {
    let elem1 = MockElement {
        tag: "div".to_string(),
        id: None,
        classes: vec!["foo".to_string()],
    };
    let elem2 = MockElement {
        tag: "div".to_string(),
        id: None,
        classes: vec!["bar".to_string()],
    };

    assert!(!can_share_style(&elem1, &elem2));
}

#[test]
fn test_can_share_style_with_id_not_sharable() {
    let elem1 = MockElement {
        tag: "div".to_string(),
        id: Some("unique".to_string()),
        classes: vec![],
    };
    let elem2 = MockElement {
        tag: "div".to_string(),
        id: Some("unique".to_string()),
        classes: vec![],
    };

    // Elements with IDs should not share styles due to uniqueness
    assert!(!can_share_style(&elem1, &elem2));
}

#[test]
fn test_compute_style_key_basic() {
    let elem = MockElement {
        tag: "div".to_string(),
        id: None,
        classes: vec!["foo".to_string()],
    };

    let key = compute_style_key(&elem);

    // Key should have a non-zero selector hash
    assert_ne!(key.selector_hash(), 0);
    assert_eq!(key.parent_hash(), None);
    assert_eq!(
        key.state_flags(),
        &StateFlags::new(false, false, false, false)
    );
}

#[test]
fn test_compute_style_key_consistency() {
    let elem = MockElement {
        tag: "div".to_string(),
        id: None,
        classes: vec!["foo".to_string()],
    };

    let key1 = compute_style_key(&elem);
    let key2 = compute_style_key(&elem);

    assert_eq!(key1, key2);
}

#[test]
fn test_style_sharing_new() {
    let sharing = StyleSharing::new();

    assert_eq!(sharing.candidate_count(), 0);
}

#[test]
fn test_style_sharing_add_candidate() {
    let mut sharing = StyleSharing::new();
    let elem = MockElement {
        tag: "div".to_string(),
        id: None,
        classes: vec![],
    };
    let values = Arc::new(ComputedValues::default());

    sharing.add_candidate(&elem, values.clone());

    assert_eq!(sharing.candidate_count(), 1);
}

#[test]
fn test_style_sharing_find_candidate() {
    let mut sharing = StyleSharing::new();

    let elem1 = MockElement {
        tag: "div".to_string(),
        id: None,
        classes: vec!["foo".to_string()],
    };
    let elem2 = MockElement {
        tag: "div".to_string(),
        id: None,
        classes: vec!["foo".to_string()],
    };

    let values = Arc::new(ComputedValues::default());
    sharing.add_candidate(&elem1, values.clone());

    let result = sharing.find_candidate(&elem2);

    assert!(result.is_some());
    assert_eq!(*result.unwrap(), *values);
}

#[test]
fn test_style_sharing_no_match() {
    let mut sharing = StyleSharing::new();

    let elem1 = MockElement {
        tag: "div".to_string(),
        id: None,
        classes: vec!["foo".to_string()],
    };
    let elem2 = MockElement {
        tag: "span".to_string(),
        id: None,
        classes: vec!["foo".to_string()],
    };

    let values = Arc::new(ComputedValues::default());
    sharing.add_candidate(&elem1, values);

    let result = sharing.find_candidate(&elem2);

    assert!(result.is_none());
}

#[test]
fn test_style_sharing_clear() {
    let mut sharing = StyleSharing::new();

    let elem = MockElement {
        tag: "div".to_string(),
        id: None,
        classes: vec![],
    };
    let values = Arc::new(ComputedValues::default());

    sharing.add_candidate(&elem, values);
    assert_eq!(sharing.candidate_count(), 1);

    sharing.clear();
    assert_eq!(sharing.candidate_count(), 0);
}
