//! Tests for Animation type

use css_animations::*;

#[test]
fn test_animation_creation() {
    let animation = Animation {
        name: "fadeIn".to_string(),
        duration: 2.0,
        timing_function: TimingFunction::Ease,
        delay: 0.0,
        iteration_count: IterationCount::Count(1.0),
        direction: AnimationDirection::Normal,
        fill_mode: FillMode::None,
        play_state: PlayState::Running,
    };

    assert_eq!(animation.name, "fadeIn");
    assert_eq!(animation.duration, 2.0);
    assert_eq!(animation.timing_function, TimingFunction::Ease);
    assert_eq!(animation.delay, 0.0);
    assert_eq!(animation.iteration_count, IterationCount::Count(1.0));
    assert_eq!(animation.direction, AnimationDirection::Normal);
    assert_eq!(animation.fill_mode, FillMode::None);
    assert_eq!(animation.play_state, PlayState::Running);
}

#[test]
fn test_animation_with_delay() {
    let animation = Animation {
        name: "slideIn".to_string(),
        duration: 1.0,
        timing_function: TimingFunction::Linear,
        delay: 0.5,
        iteration_count: IterationCount::Count(1.0),
        direction: AnimationDirection::Normal,
        fill_mode: FillMode::None,
        play_state: PlayState::Running,
    };

    assert_eq!(animation.delay, 0.5);
}

#[test]
fn test_animation_infinite_iterations() {
    let animation = Animation {
        name: "spin".to_string(),
        duration: 1.0,
        timing_function: TimingFunction::Linear,
        delay: 0.0,
        iteration_count: IterationCount::Infinite,
        direction: AnimationDirection::Normal,
        fill_mode: FillMode::None,
        play_state: PlayState::Running,
    };

    assert_eq!(animation.iteration_count, IterationCount::Infinite);
}

#[test]
fn test_animation_with_custom_timing() {
    let animation = Animation {
        name: "bounce".to_string(),
        duration: 1.0,
        timing_function: TimingFunction::CubicBezier(0.68, -0.55, 0.265, 1.55),
        delay: 0.0,
        iteration_count: IterationCount::Count(1.0),
        direction: AnimationDirection::Normal,
        fill_mode: FillMode::None,
        play_state: PlayState::Running,
    };

    if let TimingFunction::CubicBezier(x1, y1, x2, y2) = animation.timing_function {
        assert_eq!(x1, 0.68);
        assert_eq!(y1, -0.55);
        assert_eq!(x2, 0.265);
        assert_eq!(y2, 1.55);
    } else {
        panic!("Expected CubicBezier timing function");
    }
}

#[test]
fn test_animation_alternate_direction() {
    let animation = Animation {
        name: "pulse".to_string(),
        duration: 1.0,
        timing_function: TimingFunction::Ease,
        delay: 0.0,
        iteration_count: IterationCount::Infinite,
        direction: AnimationDirection::Alternate,
        fill_mode: FillMode::None,
        play_state: PlayState::Running,
    };

    assert_eq!(animation.direction, AnimationDirection::Alternate);
}

#[test]
fn test_animation_fill_mode_forwards() {
    let animation = Animation {
        name: "fadeOut".to_string(),
        duration: 1.0,
        timing_function: TimingFunction::Ease,
        delay: 0.0,
        iteration_count: IterationCount::Count(1.0),
        direction: AnimationDirection::Normal,
        fill_mode: FillMode::Forwards,
        play_state: PlayState::Running,
    };

    assert_eq!(animation.fill_mode, FillMode::Forwards);
}

#[test]
fn test_animation_paused() {
    let animation = Animation {
        name: "slideIn".to_string(),
        duration: 1.0,
        timing_function: TimingFunction::Ease,
        delay: 0.0,
        iteration_count: IterationCount::Count(1.0),
        direction: AnimationDirection::Normal,
        fill_mode: FillMode::None,
        play_state: PlayState::Paused,
    };

    assert_eq!(animation.play_state, PlayState::Paused);
}
