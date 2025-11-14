//! CSS Animations - Animation and @keyframes support
//!
//! This module provides CSS animation functionality including:
//! - Keyframe definitions and animations
//! - Timing functions (ease, linear, cubic-bezier, steps)
//! - Animation properties (duration, delay, iteration, direction, fill-mode)
//! - Animation engine for computing animated values

use std::collections::HashMap;

// ============================================================================
// Basic Enums
// ============================================================================

/// Step timing position (for steps() timing function)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepPosition {
    /// Jump happens at start of interval
    Start,
    /// Jump happens at end of interval
    End,
}

/// Animation iteration count
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IterationCount {
    /// Finite number of iterations
    Count(f32),
    /// Infinite iterations
    Infinite,
}

/// Animation playback direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationDirection {
    /// Play forward each iteration
    Normal,
    /// Play backward each iteration
    Reverse,
    /// Alternate between forward and backward
    Alternate,
    /// Alternate starting with reverse
    AlternateReverse,
}

/// Animation fill mode - how styles apply before/after animation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FillMode {
    /// No fill - element reverts to original state
    None,
    /// Keep final keyframe styles after animation
    Forwards,
    /// Apply first keyframe styles during delay
    Backwards,
    /// Apply both forwards and backwards
    Both,
}

/// Animation play state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayState {
    /// Animation is running
    Running,
    /// Animation is paused
    Paused,
}

/// Timing/easing function for animations
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimingFunction {
    /// CSS ease timing (cubic-bezier(0.25, 0.1, 0.25, 1.0))
    Ease,
    /// Linear timing (no easing)
    Linear,
    /// Ease in (slow start)
    EaseIn,
    /// Ease out (slow end)
    EaseOut,
    /// Ease in and out (slow start and end)
    EaseInOut,
    /// Custom cubic bezier curve
    CubicBezier(f32, f32, f32, f32),
    /// Step function with number of steps and position
    Steps(i32, StepPosition),
}

// ============================================================================
// Keyframe Types
// ============================================================================

/// Single keyframe in an animation
#[derive(Debug, Clone, PartialEq)]
pub struct Keyframe {
    /// Offset in animation timeline (0.0 = 0%, 1.0 = 100%)
    pub offset: f32,
    /// CSS properties and their values at this keyframe
    pub properties: HashMap<String, String>,
}

/// Named keyframes definition (@keyframes rule)
#[derive(Debug, Clone, PartialEq)]
pub struct Keyframes {
    /// Name of the keyframes animation
    pub name: String,
    /// List of keyframes (should be sorted by offset)
    pub keyframes: Vec<Keyframe>,
}

// ============================================================================
// Animation Type
// ============================================================================

/// Animation applied to an element
#[derive(Debug, Clone, PartialEq)]
pub struct Animation {
    /// Name of the keyframes to use
    pub name: String,
    /// Animation duration in seconds
    pub duration: f32,
    /// Timing/easing function
    pub timing_function: TimingFunction,
    /// Delay before animation starts (seconds)
    pub delay: f32,
    /// Number of iterations
    pub iteration_count: IterationCount,
    /// Playback direction
    pub direction: AnimationDirection,
    /// Fill mode
    pub fill_mode: FillMode,
    /// Current play state
    pub play_state: PlayState,
}

// ============================================================================
// Animation Engine Trait
// ============================================================================

/// Element identifier type
pub type ElementId = u64;

/// Animation update result from tick
#[derive(Debug, Clone, PartialEq)]
pub struct AnimationUpdate {
    /// Element being animated
    pub element_id: ElementId,
    /// Animation name
    pub animation_name: String,
    /// Updated property name
    pub property: String,
    /// New property value
    pub value: String,
}

/// Animation engine trait for computing and applying animations
pub trait AnimationEngine {
    /// Update all animations at the given timestamp
    ///
    /// # Arguments
    /// * `timestamp_ms` - Current timestamp in milliseconds
    ///
    /// # Returns
    /// Vector of property updates to apply
    fn tick(&mut self, timestamp_ms: f64) -> Vec<AnimationUpdate>;

    /// Add an animation to an element
    ///
    /// # Arguments
    /// * `element_id` - Element to animate
    /// * `animation` - Animation definition
    fn add_animation(&mut self, element_id: ElementId, animation: Animation);

    /// Pause a specific animation on an element
    ///
    /// # Arguments
    /// * `element_id` - Element with the animation
    /// * `animation_name` - Name of animation to pause
    fn pause_animation(&mut self, element_id: ElementId, animation_name: &str);

    /// Resume a paused animation on an element
    ///
    /// # Arguments
    /// * `element_id` - Element with the animation
    /// * `animation_name` - Name of animation to resume
    fn resume_animation(&mut self, element_id: ElementId, animation_name: &str);
}

// ============================================================================
// Helper Functions
// ============================================================================

impl TimingFunction {
    /// Apply timing function to linear progress (0.0 to 1.0)
    ///
    /// # Arguments
    /// * `t` - Linear progress from 0.0 to 1.0
    ///
    /// # Returns
    /// Eased progress value from 0.0 to 1.0
    pub fn apply(&self, t: f32) -> f32 {
        match self {
            TimingFunction::Linear => t,
            TimingFunction::Ease => cubic_bezier(t, 0.25, 0.1, 0.25, 1.0),
            TimingFunction::EaseIn => cubic_bezier(t, 0.42, 0.0, 1.0, 1.0),
            TimingFunction::EaseOut => cubic_bezier(t, 0.0, 0.0, 0.58, 1.0),
            TimingFunction::EaseInOut => cubic_bezier(t, 0.42, 0.0, 0.58, 1.0),
            TimingFunction::CubicBezier(x1, y1, x2, y2) => cubic_bezier(t, *x1, *y1, *x2, *y2),
            TimingFunction::Steps(steps, position) => {
                let steps_f = *steps as f32;
                match position {
                    StepPosition::Start => {
                        if t >= 1.0 {
                            1.0
                        } else {
                            // Jump happens at start of interval
                            let step = (t * steps_f).ceil().max(1.0);
                            (step / steps_f).min(1.0)
                        }
                    }
                    StepPosition::End => {
                        if t >= 1.0 {
                            1.0
                        } else {
                            // Jump happens at end of interval
                            ((t * steps_f).floor() / steps_f).min(1.0)
                        }
                    }
                }
            }
        }
    }
}

/// Cubic bezier curve evaluation (simplified implementation)
///
/// This is a simplified cubic bezier for timing functions.
/// Production implementation would use Newton-Raphson or binary search.
fn cubic_bezier(t: f32, _x1: f32, y1: f32, _x2: f32, y2: f32) -> f32 {
    // Simplified cubic bezier - use t directly for x
    // In production, we'd solve for t given x using Newton-Raphson
    let t2 = t * t;
    let t3 = t2 * t;
    let one_minus_t = 1.0 - t;
    let one_minus_t2 = one_minus_t * one_minus_t;

    // Cubic bezier curve: B(t) = (1-t)³P₀ + 3(1-t)²tP₁ + 3(1-t)t²P₂ + t³P₃
    // P₀ = (0, 0), P₃ = (1, 1), P₁ = (x1, y1), P₂ = (x2, y2)
    // Note: Simplified - not using x1, x2 (would be needed for proper bezier solver)
    3.0 * one_minus_t2 * t * y1 + 3.0 * one_minus_t * t2 * y2 + t3
}

/// Interpolate between two numeric values
///
/// # Arguments
/// * `from` - Start value
/// * `to` - End value
/// * `progress` - Interpolation progress (0.0 to 1.0)
///
/// # Returns
/// Interpolated value
pub fn interpolate_f32(from: f32, to: f32, progress: f32) -> f32 {
    from + (to - from) * progress
}

/// Find keyframes surrounding a given offset
///
/// # Arguments
/// * `keyframes` - Sorted list of keyframes
/// * `offset` - Target offset (0.0 to 1.0)
///
/// # Returns
/// Tuple of (before_keyframe, after_keyframe, local_progress)
pub fn find_surrounding_keyframes(
    keyframes: &[Keyframe],
    offset: f32,
) -> Option<(&Keyframe, &Keyframe, f32)> {
    if keyframes.is_empty() {
        return None;
    }

    // Find the keyframes before and after the offset
    let mut before_idx = 0;
    let mut after_idx = keyframes.len() - 1;

    for (i, kf) in keyframes.iter().enumerate() {
        if kf.offset <= offset {
            before_idx = i;
        }
        if kf.offset >= offset {
            after_idx = i;
            break;
        }
    }

    // If we're exactly on a keyframe, return it twice with 0 progress
    if keyframes[before_idx].offset == offset {
        return Some((&keyframes[before_idx], &keyframes[before_idx], 0.0));
    }

    // If before and after are the same, we're past the last keyframe
    if before_idx == after_idx {
        return Some((&keyframes[before_idx], &keyframes[after_idx], 1.0));
    }

    let before = &keyframes[before_idx];
    let after = &keyframes[after_idx];

    // Calculate local progress between the two keyframes
    let range = after.offset - before.offset;
    let local_offset = offset - before.offset;
    let local_progress = if range > 0.0 {
        local_offset / range
    } else {
        0.0
    };

    Some((before, after, local_progress))
}

// ============================================================================
// Basic Animation Engine Implementation
// ============================================================================

/// Internal state for a running animation
#[derive(Debug, Clone)]
struct AnimationState {
    element_id: ElementId,
    animation: Animation,
    start_time: f64,
}

/// Basic animation engine implementation
#[derive(Debug, Default)]
pub struct BasicAnimationEngine {
    animations: Vec<AnimationState>,
    keyframes_registry: HashMap<String, Keyframes>,
}

impl BasicAnimationEngine {
    /// Create a new animation engine
    pub fn new() -> Self {
        Self {
            animations: Vec::new(),
            keyframes_registry: HashMap::new(),
        }
    }

    /// Register keyframes definition
    ///
    /// # Arguments
    /// * `keyframes` - Keyframes to register
    pub fn register_keyframes(&mut self, keyframes: Keyframes) {
        self.keyframes_registry
            .insert(keyframes.name.clone(), keyframes);
    }

    /// Get registered keyframes by name
    pub fn get_keyframes(&self, name: &str) -> Option<&Keyframes> {
        self.keyframes_registry.get(name)
    }

    /// Calculate animation progress at given timestamp
    fn calculate_progress(&self, state: &AnimationState, timestamp_ms: f64) -> Option<f32> {
        if state.animation.play_state == PlayState::Paused {
            return None;
        }

        let elapsed = (timestamp_ms - state.start_time) / 1000.0; // Convert to seconds
        let delay = state.animation.delay as f64;

        // Animation hasn't started yet (still in delay period)
        if elapsed < delay {
            if state.animation.fill_mode == FillMode::Backwards
                || state.animation.fill_mode == FillMode::Both
            {
                return Some(0.0);
            }
            return None;
        }

        let time_since_start = elapsed - delay;
        let duration = state.animation.duration as f64;

        // Calculate which iteration we're in
        let raw_progress = time_since_start / duration;

        // Check if animation is complete
        let is_complete = match state.animation.iteration_count {
            IterationCount::Count(count) => raw_progress >= count as f64,
            IterationCount::Infinite => false,
        };

        if is_complete {
            // Animation finished
            if state.animation.fill_mode == FillMode::Forwards
                || state.animation.fill_mode == FillMode::Both
            {
                return Some(1.0);
            }
            return None;
        }

        // Get progress within current iteration
        let iteration_progress = (raw_progress % 1.0) as f32;

        // Apply direction
        let directed_progress = match state.animation.direction {
            AnimationDirection::Normal => iteration_progress,
            AnimationDirection::Reverse => 1.0 - iteration_progress,
            AnimationDirection::Alternate => {
                let iteration = raw_progress.floor() as i32;
                if iteration % 2 == 0 {
                    iteration_progress
                } else {
                    1.0 - iteration_progress
                }
            }
            AnimationDirection::AlternateReverse => {
                let iteration = raw_progress.floor() as i32;
                if iteration % 2 == 0 {
                    1.0 - iteration_progress
                } else {
                    iteration_progress
                }
            }
        };

        Some(directed_progress)
    }
}

impl AnimationEngine for BasicAnimationEngine {
    fn tick(&mut self, timestamp_ms: f64) -> Vec<AnimationUpdate> {
        let mut updates = Vec::new();

        for state in &self.animations {
            // Calculate current progress
            let progress = match self.calculate_progress(state, timestamp_ms) {
                Some(p) => p,
                None => continue,
            };

            // Get keyframes for this animation
            let keyframes = match self.keyframes_registry.get(&state.animation.name) {
                Some(kf) => kf,
                None => continue,
            };

            // Apply timing function
            let eased_progress = state.animation.timing_function.apply(progress);

            // Find surrounding keyframes
            let (before, after, local_progress) =
                match find_surrounding_keyframes(&keyframes.keyframes, eased_progress) {
                    Some(result) => result,
                    None => continue,
                };

            // For each property, interpolate and create update
            // First, collect all properties from both keyframes
            let mut properties = std::collections::HashSet::new();
            for key in before.properties.keys() {
                properties.insert(key.clone());
            }
            for key in after.properties.keys() {
                properties.insert(key.clone());
            }

            for property in properties {
                let value = if local_progress == 0.0 {
                    // Exactly on a keyframe
                    before.properties.get(&property).cloned()
                } else {
                    // Need to interpolate (simplified - just use 'after' value for non-numeric)
                    after.properties.get(&property).cloned()
                };

                if let Some(val) = value {
                    updates.push(AnimationUpdate {
                        element_id: state.element_id,
                        animation_name: state.animation.name.clone(),
                        property: property.clone(),
                        value: val,
                    });
                }
            }
        }

        updates
    }

    fn add_animation(&mut self, element_id: ElementId, animation: Animation) {
        // Remove any existing animation with the same name on this element
        self.animations.retain(|state| {
            state.element_id != element_id || state.animation.name != animation.name
        });

        // Add new animation (start time is set when first tick is called)
        // For now, use 0.0 as start time - in a real implementation,
        // this would be set to the current timestamp
        self.animations.push(AnimationState {
            element_id,
            animation,
            start_time: 0.0,
        });
    }

    fn pause_animation(&mut self, element_id: ElementId, animation_name: &str) {
        for state in &mut self.animations {
            if state.element_id == element_id && state.animation.name == animation_name {
                state.animation.play_state = PlayState::Paused;
            }
        }
    }

    fn resume_animation(&mut self, element_id: ElementId, animation_name: &str) {
        for state in &mut self.animations {
            if state.element_id == element_id && state.animation.name == animation_name {
                state.animation.play_state = PlayState::Running;
            }
        }
    }
}
