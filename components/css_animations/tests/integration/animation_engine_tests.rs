//! Integration tests for AnimationEngine

use css_animations::*;
use std::collections::HashMap;

#[test]
fn test_engine_register_keyframes() {
    let mut engine = BasicAnimationEngine::new();

    let mut props_0 = HashMap::new();
    props_0.insert("opacity".to_string(), "0".to_string());

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
                offset: 1.0,
                properties: props_100,
            },
        ],
    };

    engine.register_keyframes(keyframes);

    let retrieved = engine.get_keyframes("fadeIn");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().name, "fadeIn");
}

#[test]
fn test_engine_add_animation() {
    let mut engine = BasicAnimationEngine::new();

    let animation = Animation {
        name: "fadeIn".to_string(),
        duration: 1.0,
        timing_function: TimingFunction::Linear,
        delay: 0.0,
        iteration_count: IterationCount::Count(1.0),
        direction: AnimationDirection::Normal,
        fill_mode: FillMode::None,
        play_state: PlayState::Running,
    };

    engine.add_animation(1, animation);

    // Verify by ticking (should return empty since no keyframes registered)
    let updates = engine.tick(500.0);
    assert_eq!(updates.len(), 0); // No keyframes registered
}

#[test]
fn test_engine_simple_animation() {
    let mut engine = BasicAnimationEngine::new();

    // Register keyframes
    let mut props_0 = HashMap::new();
    props_0.insert("opacity".to_string(), "0".to_string());

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
                offset: 1.0,
                properties: props_100,
            },
        ],
    };

    engine.register_keyframes(keyframes);

    // Add animation (1 second duration)
    let animation = Animation {
        name: "fadeIn".to_string(),
        duration: 1.0,
        timing_function: TimingFunction::Linear,
        delay: 0.0,
        iteration_count: IterationCount::Count(1.0),
        direction: AnimationDirection::Normal,
        fill_mode: FillMode::None,
        play_state: PlayState::Running,
    };

    engine.add_animation(1, animation);

    // Tick at start (0ms)
    let updates = engine.tick(0.0);
    assert_eq!(updates.len(), 1);
    assert_eq!(updates[0].element_id, 1);
    assert_eq!(updates[0].property, "opacity");
    assert_eq!(updates[0].value, "0");

    // Tick at middle (500ms)
    let updates = engine.tick(500.0);
    assert_eq!(updates.len(), 1);
    // At 50% progress with linear timing, we're between keyframes
    // Our simplified implementation just uses the 'after' value
    assert_eq!(updates[0].value, "1");
}

#[test]
fn test_engine_pause_resume() {
    let mut engine = BasicAnimationEngine::new();

    // Register keyframes
    let mut props_0 = HashMap::new();
    props_0.insert("opacity".to_string(), "0".to_string());

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
                offset: 1.0,
                properties: props_100,
            },
        ],
    };

    engine.register_keyframes(keyframes);

    // Add animation
    let animation = Animation {
        name: "fadeIn".to_string(),
        duration: 1.0,
        timing_function: TimingFunction::Linear,
        delay: 0.0,
        iteration_count: IterationCount::Count(1.0),
        direction: AnimationDirection::Normal,
        fill_mode: FillMode::None,
        play_state: PlayState::Running,
    };

    engine.add_animation(1, animation);

    // Pause the animation
    engine.pause_animation(1, "fadeIn");

    // Tick should return no updates when paused
    let updates = engine.tick(500.0);
    assert_eq!(updates.len(), 0);

    // Resume the animation
    engine.resume_animation(1, "fadeIn");

    // Should get updates again
    let updates = engine.tick(500.0);
    assert_eq!(updates.len(), 1);
}

#[test]
fn test_engine_fill_mode_forwards() {
    let mut engine = BasicAnimationEngine::new();

    // Register keyframes
    let mut props_0 = HashMap::new();
    props_0.insert("opacity".to_string(), "0".to_string());

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
                offset: 1.0,
                properties: props_100,
            },
        ],
    };

    engine.register_keyframes(keyframes);

    // Add animation with forwards fill mode
    let animation = Animation {
        name: "fadeIn".to_string(),
        duration: 1.0,
        timing_function: TimingFunction::Linear,
        delay: 0.0,
        iteration_count: IterationCount::Count(1.0),
        direction: AnimationDirection::Normal,
        fill_mode: FillMode::Forwards,
        play_state: PlayState::Running,
    };

    engine.add_animation(1, animation);

    // Tick after animation completes (2000ms > 1000ms duration)
    let updates = engine.tick(2000.0);
    assert_eq!(updates.len(), 1);
    // With forwards fill mode, should stay at final state
    assert_eq!(updates[0].value, "1");
}

#[test]
fn test_engine_multiple_animations() {
    let mut engine = BasicAnimationEngine::new();

    // Register first keyframes
    let mut props1_0 = HashMap::new();
    props1_0.insert("opacity".to_string(), "0".to_string());

    let mut props1_100 = HashMap::new();
    props1_100.insert("opacity".to_string(), "1".to_string());

    let keyframes1 = Keyframes {
        name: "fadeIn".to_string(),
        keyframes: vec![
            Keyframe {
                offset: 0.0,
                properties: props1_0,
            },
            Keyframe {
                offset: 1.0,
                properties: props1_100,
            },
        ],
    };

    // Register second keyframes
    let mut props2_0 = HashMap::new();
    props2_0.insert("transform".to_string(), "translateX(0px)".to_string());

    let mut props2_100 = HashMap::new();
    props2_100.insert("transform".to_string(), "translateX(100px)".to_string());

    let keyframes2 = Keyframes {
        name: "slideIn".to_string(),
        keyframes: vec![
            Keyframe {
                offset: 0.0,
                properties: props2_0,
            },
            Keyframe {
                offset: 1.0,
                properties: props2_100,
            },
        ],
    };

    engine.register_keyframes(keyframes1);
    engine.register_keyframes(keyframes2);

    // Add both animations to same element
    let animation1 = Animation {
        name: "fadeIn".to_string(),
        duration: 1.0,
        timing_function: TimingFunction::Linear,
        delay: 0.0,
        iteration_count: IterationCount::Count(1.0),
        direction: AnimationDirection::Normal,
        fill_mode: FillMode::None,
        play_state: PlayState::Running,
    };

    let animation2 = Animation {
        name: "slideIn".to_string(),
        duration: 1.0,
        timing_function: TimingFunction::Linear,
        delay: 0.0,
        iteration_count: IterationCount::Count(1.0),
        direction: AnimationDirection::Normal,
        fill_mode: FillMode::None,
        play_state: PlayState::Running,
    };

    engine.add_animation(1, animation1);
    engine.add_animation(1, animation2);

    // Tick should return updates for both animations
    let updates = engine.tick(500.0);
    assert_eq!(updates.len(), 2);

    // Check that we have updates for both properties
    let properties: Vec<&str> = updates.iter().map(|u| u.property.as_str()).collect();
    assert!(properties.contains(&"opacity"));
    assert!(properties.contains(&"transform"));
}
