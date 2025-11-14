//! Tests for Keyframe and Keyframes types

use css_animations::*;
use std::collections::HashMap;

#[test]
fn test_keyframe_creation() {
    let mut properties = HashMap::new();
    properties.insert("opacity".to_string(), "0.5".to_string());
    properties.insert("transform".to_string(), "translateX(100px)".to_string());

    let keyframe = Keyframe {
        offset: 0.5,
        properties,
    };

    assert_eq!(keyframe.offset, 0.5);
    assert_eq!(keyframe.properties.len(), 2);
    assert_eq!(keyframe.properties.get("opacity"), Some(&"0.5".to_string()));
    assert_eq!(
        keyframe.properties.get("transform"),
        Some(&"translateX(100px)".to_string())
    );
}

#[test]
fn test_keyframe_at_start() {
    let mut properties = HashMap::new();
    properties.insert("opacity".to_string(), "0".to_string());

    let keyframe = Keyframe {
        offset: 0.0,
        properties,
    };

    assert_eq!(keyframe.offset, 0.0);
}

#[test]
fn test_keyframe_at_end() {
    let mut properties = HashMap::new();
    properties.insert("opacity".to_string(), "1".to_string());

    let keyframe = Keyframe {
        offset: 1.0,
        properties,
    };

    assert_eq!(keyframe.offset, 1.0);
}

#[test]
fn test_keyframes_creation() {
    let mut properties_0 = HashMap::new();
    properties_0.insert("opacity".to_string(), "0".to_string());

    let mut properties_100 = HashMap::new();
    properties_100.insert("opacity".to_string(), "1".to_string());

    let keyframes = Keyframes {
        name: "fadeIn".to_string(),
        keyframes: vec![
            Keyframe {
                offset: 0.0,
                properties: properties_0,
            },
            Keyframe {
                offset: 1.0,
                properties: properties_100,
            },
        ],
    };

    assert_eq!(keyframes.name, "fadeIn");
    assert_eq!(keyframes.keyframes.len(), 2);
    assert_eq!(keyframes.keyframes[0].offset, 0.0);
    assert_eq!(keyframes.keyframes[1].offset, 1.0);
}

#[test]
fn test_keyframes_with_intermediate_keyframe() {
    let mut props_0 = HashMap::new();
    props_0.insert("opacity".to_string(), "0".to_string());

    let mut props_50 = HashMap::new();
    props_50.insert("opacity".to_string(), "0.5".to_string());

    let mut props_100 = HashMap::new();
    props_100.insert("opacity".to_string(), "1".to_string());

    let keyframes = Keyframes {
        name: "fadeIn".to_string(),
        keyframes: vec![
            Keyframe {
                offset: 0.0,
                properties: props_0,
            },
            Keyframe {
                offset: 0.5,
                properties: props_50,
            },
            Keyframe {
                offset: 1.0,
                properties: props_100,
            },
        ],
    };

    assert_eq!(keyframes.keyframes.len(), 3);
    assert_eq!(keyframes.keyframes[0].offset, 0.0);
    assert_eq!(keyframes.keyframes[1].offset, 0.5);
    assert_eq!(keyframes.keyframes[2].offset, 1.0);
}
