//! Unit tests for basic animation types

use css_animations::*;

#[test]
fn test_step_position_variants() {
    let start = StepPosition::Start;
    let end = StepPosition::End;

    assert!(matches!(start, StepPosition::Start));
    assert!(matches!(end, StepPosition::End));
}

#[test]
fn test_iteration_count_finite() {
    let count = IterationCount::Count(3.0);

    if let IterationCount::Count(n) = count {
        assert_eq!(n, 3.0);
    } else {
        panic!("Expected Count variant");
    }
}

#[test]
fn test_iteration_count_infinite() {
    let infinite = IterationCount::Infinite;
    assert!(matches!(infinite, IterationCount::Infinite));
}

#[test]
fn test_animation_direction_variants() {
    let normal = AnimationDirection::Normal;
    let reverse = AnimationDirection::Reverse;
    let alternate = AnimationDirection::Alternate;
    let alt_reverse = AnimationDirection::AlternateReverse;

    assert!(matches!(normal, AnimationDirection::Normal));
    assert!(matches!(reverse, AnimationDirection::Reverse));
    assert!(matches!(alternate, AnimationDirection::Alternate));
    assert!(matches!(alt_reverse, AnimationDirection::AlternateReverse));
}

#[test]
fn test_fill_mode_variants() {
    let none = FillMode::None;
    let forwards = FillMode::Forwards;
    let backwards = FillMode::Backwards;
    let both = FillMode::Both;

    assert!(matches!(none, FillMode::None));
    assert!(matches!(forwards, FillMode::Forwards));
    assert!(matches!(backwards, FillMode::Backwards));
    assert!(matches!(both, FillMode::Both));
}

#[test]
fn test_play_state_variants() {
    let running = PlayState::Running;
    let paused = PlayState::Paused;

    assert!(matches!(running, PlayState::Running));
    assert!(matches!(paused, PlayState::Paused));
}

#[test]
fn test_timing_function_ease() {
    let ease = TimingFunction::Ease;
    assert!(matches!(ease, TimingFunction::Ease));
}

#[test]
fn test_timing_function_linear() {
    let linear = TimingFunction::Linear;
    assert!(matches!(linear, TimingFunction::Linear));
}

#[test]
fn test_timing_function_ease_in() {
    let ease_in = TimingFunction::EaseIn;
    assert!(matches!(ease_in, TimingFunction::EaseIn));
}

#[test]
fn test_timing_function_ease_out() {
    let ease_out = TimingFunction::EaseOut;
    assert!(matches!(ease_out, TimingFunction::EaseOut));
}

#[test]
fn test_timing_function_ease_in_out() {
    let ease_in_out = TimingFunction::EaseInOut;
    assert!(matches!(ease_in_out, TimingFunction::EaseInOut));
}

#[test]
fn test_timing_function_cubic_bezier() {
    let bezier = TimingFunction::CubicBezier(0.25, 0.1, 0.25, 1.0);

    if let TimingFunction::CubicBezier(x1, y1, x2, y2) = bezier {
        assert_eq!(x1, 0.25);
        assert_eq!(y1, 0.1);
        assert_eq!(x2, 0.25);
        assert_eq!(y2, 1.0);
    } else {
        panic!("Expected CubicBezier variant");
    }
}

#[test]
fn test_timing_function_steps() {
    let steps = TimingFunction::Steps(4, StepPosition::End);

    if let TimingFunction::Steps(n, pos) = steps {
        assert_eq!(n, 4);
        assert!(matches!(pos, StepPosition::End));
    } else {
        panic!("Expected Steps variant");
    }
}
