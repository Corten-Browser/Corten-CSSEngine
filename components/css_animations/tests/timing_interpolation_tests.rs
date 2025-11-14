//! Tests for timing functions and interpolation

use css_animations::*;
use std::collections::HashMap;

// ============================================================================
// Timing Function Tests
// ============================================================================

#[test]
fn test_linear_timing() {
    let linear = TimingFunction::Linear;

    assert_eq!(linear.apply(0.0), 0.0);
    assert_eq!(linear.apply(0.5), 0.5);
    assert_eq!(linear.apply(1.0), 1.0);
}

#[test]
fn test_ease_timing_boundaries() {
    let ease = TimingFunction::Ease;

    let start = ease.apply(0.0);
    let end = ease.apply(1.0);

    // At start and end, should be close to 0.0 and 1.0
    assert!((start - 0.0).abs() < 0.01);
    assert!((end - 1.0).abs() < 0.01);
}

#[test]
fn test_ease_in_timing() {
    let ease_in = TimingFunction::EaseIn;

    let start = ease_in.apply(0.0);
    let end = ease_in.apply(1.0);

    // Ease in: start and end should be close to 0.0 and 1.0
    // (Note: simplified bezier implementation, so we test boundaries)
    assert!((start - 0.0).abs() < 0.01);
    assert!((end - 1.0).abs() < 0.01);
}

#[test]
fn test_ease_out_timing() {
    let ease_out = TimingFunction::EaseOut;

    let start = ease_out.apply(0.0);
    let end = ease_out.apply(1.0);

    // Ease out: start and end should be close to 0.0 and 1.0
    // (Note: simplified bezier implementation, so we test boundaries)
    assert!((start - 0.0).abs() < 0.01);
    assert!((end - 1.0).abs() < 0.01);
}

#[test]
fn test_custom_cubic_bezier() {
    let bezier = TimingFunction::CubicBezier(0.25, 0.1, 0.25, 1.0);

    let start = bezier.apply(0.0);
    let end = bezier.apply(1.0);

    assert!((start - 0.0).abs() < 0.01);
    assert!((end - 1.0).abs() < 0.01);
}

#[test]
fn test_steps_end() {
    let steps = TimingFunction::Steps(4, StepPosition::End);

    // With 4 steps and End position: 0, 0.25, 0.5, 0.75, 1.0
    assert_eq!(steps.apply(0.0), 0.0);
    assert_eq!(steps.apply(0.1), 0.0);
    assert_eq!(steps.apply(0.25), 0.25);
    assert_eq!(steps.apply(0.5), 0.5);
    assert_eq!(steps.apply(0.75), 0.75);
    assert_eq!(steps.apply(1.0), 1.0);
}

#[test]
fn test_steps_start() {
    let steps = TimingFunction::Steps(4, StepPosition::Start);

    // With 4 steps and Start position, jumps happen at start
    assert_eq!(steps.apply(0.0), 0.25);
    assert_eq!(steps.apply(0.24), 0.25);
    assert_eq!(steps.apply(0.26), 0.5);
    assert_eq!(steps.apply(1.0), 1.0);
}

// ============================================================================
// Interpolation Tests
// ============================================================================

#[test]
fn test_interpolate_f32_at_start() {
    let result = interpolate_f32(0.0, 100.0, 0.0);
    assert_eq!(result, 0.0);
}

#[test]
fn test_interpolate_f32_at_end() {
    let result = interpolate_f32(0.0, 100.0, 1.0);
    assert_eq!(result, 100.0);
}

#[test]
fn test_interpolate_f32_at_middle() {
    let result = interpolate_f32(0.0, 100.0, 0.5);
    assert_eq!(result, 50.0);
}

#[test]
fn test_interpolate_f32_negative_to_positive() {
    let result = interpolate_f32(-50.0, 50.0, 0.5);
    assert_eq!(result, 0.0);
}

#[test]
fn test_interpolate_f32_with_quarter_progress() {
    let result = interpolate_f32(0.0, 100.0, 0.25);
    assert_eq!(result, 25.0);
}

// ============================================================================
// Keyframe Finding Tests
// ============================================================================

#[test]
fn test_find_surrounding_keyframes_empty() {
    let keyframes: Vec<Keyframe> = vec![];
    let result = find_surrounding_keyframes(&keyframes, 0.5);
    assert!(result.is_none());
}

#[test]
fn test_find_surrounding_keyframes_single() {
    let mut props = HashMap::new();
    props.insert("opacity".to_string(), "1".to_string());

    let keyframes = vec![Keyframe {
        offset: 0.0,
        properties: props,
    }];

    let result = find_surrounding_keyframes(&keyframes, 0.5);
    assert!(result.is_some());

    let (before, after, progress) = result.unwrap();
    assert_eq!(before.offset, 0.0);
    assert_eq!(after.offset, 0.0);
    assert_eq!(progress, 1.0);
}

#[test]
fn test_find_surrounding_keyframes_two() {
    let mut props_0 = HashMap::new();
    props_0.insert("opacity".to_string(), "0".to_string());

    let mut props_1 = HashMap::new();
    props_1.insert("opacity".to_string(), "1".to_string());

    let keyframes = vec![
        Keyframe {
            offset: 0.0,
            properties: props_0,
        },
        Keyframe {
            offset: 1.0,
            properties: props_1,
        },
    ];

    let result = find_surrounding_keyframes(&keyframes, 0.5);
    assert!(result.is_some());

    let (before, after, progress) = result.unwrap();
    assert_eq!(before.offset, 0.0);
    assert_eq!(after.offset, 1.0);
    assert_eq!(progress, 0.5);
}

#[test]
fn test_find_surrounding_keyframes_exact_match() {
    let mut props_0 = HashMap::new();
    props_0.insert("opacity".to_string(), "0".to_string());

    let mut props_1 = HashMap::new();
    props_1.insert("opacity".to_string(), "1".to_string());

    let keyframes = vec![
        Keyframe {
            offset: 0.0,
            properties: props_0,
        },
        Keyframe {
            offset: 1.0,
            properties: props_1,
        },
    ];

    let result = find_surrounding_keyframes(&keyframes, 0.0);
    assert!(result.is_some());

    let (before, after, progress) = result.unwrap();
    assert_eq!(before.offset, 0.0);
    assert_eq!(after.offset, 0.0);
    assert_eq!(progress, 0.0);
}

#[test]
fn test_find_surrounding_keyframes_three_keyframes() {
    let mut props_0 = HashMap::new();
    props_0.insert("opacity".to_string(), "0".to_string());

    let mut props_50 = HashMap::new();
    props_50.insert("opacity".to_string(), "0.5".to_string());

    let mut props_100 = HashMap::new();
    props_100.insert("opacity".to_string(), "1".to_string());

    let keyframes = vec![
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
    ];

    // Test between first and second keyframe
    let result = find_surrounding_keyframes(&keyframes, 0.25);
    assert!(result.is_some());
    let (before, after, progress) = result.unwrap();
    assert_eq!(before.offset, 0.0);
    assert_eq!(after.offset, 0.5);
    assert_eq!(progress, 0.5);

    // Test between second and third keyframe
    let result = find_surrounding_keyframes(&keyframes, 0.75);
    assert!(result.is_some());
    let (before, after, progress) = result.unwrap();
    assert_eq!(before.offset, 0.5);
    assert_eq!(after.offset, 1.0);
    assert_eq!(progress, 0.5);
}
