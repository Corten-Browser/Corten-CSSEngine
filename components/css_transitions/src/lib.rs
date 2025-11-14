//! CSS Transitions - Smooth property value changes
//!
//! This module provides CSS transition functionality including:
//! - Transition property parsing (all, none, specific, multiple)
//! - Duration and delay parsing (s, ms)
//! - Timing function parsing (ease, linear, cubic-bezier, steps)
//! - Value interpolation (length, color, number, percentage)
//! - Transition state management

use css_animations::StepPosition;
use css_types::{Color, CssError, Length};

// Re-export StepPosition from css_animations
pub use css_animations::StepPosition as AnimationStepPosition;

// ============================================================================
// Basic Types
// ============================================================================

/// Property to transition
#[derive(Debug, Clone, PartialEq)]
pub enum TransitionProperty {
    /// Transition all properties
    All,
    /// No transition
    None,
    /// Specific property by name
    Property(String),
    /// Multiple properties
    Multiple(Vec<String>),
}

/// Transition duration
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TransitionDuration {
    /// Duration in seconds
    pub duration: f64,
}

/// Timing function for transitions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransitionTimingFunction {
    /// CSS ease timing
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
    CubicBezier { x1: f64, y1: f64, x2: f64, y2: f64 },
    /// Step function
    Steps { count: u32, position: StepPosition },
}

/// Transition delay
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TransitionDelay {
    /// Delay in seconds
    pub delay: f64,
}

/// Complete transition specification
#[derive(Debug, Clone, PartialEq)]
pub struct Transition {
    /// Property to transition
    pub property: TransitionProperty,
    /// Transition duration
    pub duration: TransitionDuration,
    /// Timing function
    pub timing_function: TransitionTimingFunction,
    /// Transition delay
    pub delay: TransitionDelay,
}

/// Placeholder Transform type (to be implemented in css_types later)
#[derive(Debug, Clone, PartialEq)]
pub struct Transform {
    // Simplified placeholder
    pub value: String,
}

/// Generic property value for transitions
#[derive(Debug, Clone, PartialEq)]
pub enum PropertyValue {
    /// Length value
    Length(Length),
    /// Color value
    Color(Color),
    /// Numeric value
    Number(f64),
    /// Percentage value
    Percentage(f32),
    /// Transform value
    Transform(Transform),
}

/// Active transition state
#[derive(Debug, Clone, PartialEq)]
pub struct TransitionState {
    /// Property being transitioned
    pub property: String,
    /// Starting value
    pub start_value: PropertyValue,
    /// Ending value
    pub end_value: PropertyValue,
    /// Start time in seconds
    pub start_time: f64,
    /// Duration in seconds
    pub duration: f64,
    /// Timing function
    pub timing_function: TransitionTimingFunction,
}

// ============================================================================
// Parsing Functions
// ============================================================================

/// Parse transition-property value
///
/// # Examples
/// ```
/// use css_transitions::{parse_transition_property, TransitionProperty};
///
/// let prop = parse_transition_property("all").unwrap();
/// assert_eq!(prop, TransitionProperty::All);
///
/// let prop = parse_transition_property("opacity").unwrap();
/// assert_eq!(prop, TransitionProperty::Property("opacity".to_string()));
/// ```
pub fn parse_transition_property(input: &str) -> Result<TransitionProperty, CssError> {
    let input = input.trim();

    if input.is_empty() {
        return Err(CssError::ParseError(
            "Empty transition property".to_string(),
        ));
    }

    match input {
        "all" => Ok(TransitionProperty::All),
        "none" => Ok(TransitionProperty::None),
        _ => {
            // Check for multiple properties (comma-separated)
            if input.contains(',') {
                let properties: Vec<String> = input
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();

                if properties.is_empty() {
                    return Err(CssError::ParseError(
                        "No valid properties found".to_string(),
                    ));
                }

                Ok(TransitionProperty::Multiple(properties))
            } else {
                Ok(TransitionProperty::Property(input.to_string()))
            }
        }
    }
}

/// Parse transition-duration value
///
/// # Examples
/// ```
/// use css_transitions::{parse_transition_duration, TransitionDuration};
///
/// let duration = parse_transition_duration("0.3s").unwrap();
/// assert_eq!(duration.duration, 0.3);
///
/// let duration = parse_transition_duration("300ms").unwrap();
/// assert_eq!(duration.duration, 0.3);
/// ```
pub fn parse_transition_duration(input: &str) -> Result<TransitionDuration, CssError> {
    let input = input.trim();

    if input.is_empty() {
        return Err(CssError::ParseError("Empty duration".to_string()));
    }

    // Check for milliseconds first (more specific)
    if let Some(ms_value) = input.strip_suffix("ms") {
        let ms_value = ms_value.trim();
        let duration_ms = ms_value
            .parse::<f64>()
            .map_err(|_| CssError::ParseError("Invalid duration value".to_string()))?;

        if duration_ms < 0.0 {
            return Err(CssError::InvalidValue(
                "Duration cannot be negative".to_string(),
            ));
        }

        Ok(TransitionDuration {
            duration: duration_ms / 1000.0,
        })
    } else if let Some(s_value) = input.strip_suffix('s') {
        // Seconds
        let s_value = s_value.trim();

        let duration = s_value
            .parse::<f64>()
            .map_err(|_| CssError::ParseError("Invalid duration value".to_string()))?;

        if duration < 0.0 {
            return Err(CssError::InvalidValue(
                "Duration cannot be negative".to_string(),
            ));
        }

        Ok(TransitionDuration { duration })
    } else {
        Err(CssError::ParseError(
            "Duration must end with 's' or 'ms'".to_string(),
        ))
    }
}

/// Parse transition-timing-function value
///
/// # Examples
/// ```
/// use css_transitions::{parse_transition_timing_function, TransitionTimingFunction};
///
/// let timing = parse_transition_timing_function("ease").unwrap();
/// assert_eq!(timing, TransitionTimingFunction::Ease);
/// ```
pub fn parse_transition_timing_function(input: &str) -> Result<TransitionTimingFunction, CssError> {
    let input = input.trim();

    if input.is_empty() {
        return Err(CssError::ParseError("Empty timing function".to_string()));
    }

    match input {
        "ease" => Ok(TransitionTimingFunction::Ease),
        "linear" => Ok(TransitionTimingFunction::Linear),
        "ease-in" => Ok(TransitionTimingFunction::EaseIn),
        "ease-out" => Ok(TransitionTimingFunction::EaseOut),
        "ease-in-out" => Ok(TransitionTimingFunction::EaseInOut),
        _ => {
            // Try to parse cubic-bezier or steps
            if input.starts_with("cubic-bezier(") && input.ends_with(')') {
                parse_cubic_bezier(input)
            } else if input.starts_with("steps(") && input.ends_with(')') {
                parse_steps(input)
            } else {
                Err(CssError::ParseError(format!(
                    "Unknown timing function: {}",
                    input
                )))
            }
        }
    }
}

/// Parse cubic-bezier timing function
fn parse_cubic_bezier(input: &str) -> Result<TransitionTimingFunction, CssError> {
    let content = &input[13..input.len() - 1]; // Remove "cubic-bezier(" and ")"
    let parts: Vec<&str> = content.split(',').map(|s| s.trim()).collect();

    if parts.len() != 4 {
        return Err(CssError::ParseError(
            "cubic-bezier requires 4 values".to_string(),
        ));
    }

    let x1 = parts[0]
        .parse::<f64>()
        .map_err(|_| CssError::ParseError("Invalid x1 value".to_string()))?;
    let y1 = parts[1]
        .parse::<f64>()
        .map_err(|_| CssError::ParseError("Invalid y1 value".to_string()))?;
    let x2 = parts[2]
        .parse::<f64>()
        .map_err(|_| CssError::ParseError("Invalid x2 value".to_string()))?;
    let y2 = parts[3]
        .parse::<f64>()
        .map_err(|_| CssError::ParseError("Invalid y2 value".to_string()))?;

    // x values must be in [0, 1]
    if !(0.0..=1.0).contains(&x1) || !(0.0..=1.0).contains(&x2) {
        return Err(CssError::InvalidValue(
            "cubic-bezier x values must be in range [0, 1]".to_string(),
        ));
    }

    Ok(TransitionTimingFunction::CubicBezier { x1, y1, x2, y2 })
}

/// Parse steps timing function
fn parse_steps(input: &str) -> Result<TransitionTimingFunction, CssError> {
    let content = &input[6..input.len() - 1]; // Remove "steps(" and ")"
    let parts: Vec<&str> = content.split(',').map(|s| s.trim()).collect();

    if parts.is_empty() || parts.len() > 2 {
        return Err(CssError::ParseError(
            "steps requires 1 or 2 values".to_string(),
        ));
    }

    let count = parts[0]
        .parse::<u32>()
        .map_err(|_| CssError::ParseError("Invalid step count".to_string()))?;

    if count == 0 {
        return Err(CssError::InvalidValue("Step count must be > 0".to_string()));
    }

    let position = if parts.len() == 2 {
        match parts[1] {
            "start" => StepPosition::Start,
            "end" => StepPosition::End,
            _ => {
                return Err(CssError::ParseError(format!(
                    "Invalid step position: {}",
                    parts[1]
                )))
            }
        }
    } else {
        StepPosition::End // Default
    };

    Ok(TransitionTimingFunction::Steps { count, position })
}

/// Parse transition-delay value
///
/// # Examples
/// ```
/// use css_transitions::{parse_transition_delay, TransitionDelay};
///
/// let delay = parse_transition_delay("0.5s").unwrap();
/// assert_eq!(delay.delay, 0.5);
/// ```
pub fn parse_transition_delay(input: &str) -> Result<TransitionDelay, CssError> {
    // Same parsing logic as duration, but allow negative values
    let input = input.trim();

    if input.is_empty() {
        return Err(CssError::ParseError("Empty delay".to_string()));
    }

    // Check for milliseconds first (more specific)
    if let Some(ms_value) = input.strip_suffix("ms") {
        let ms_value = ms_value.trim();
        let delay_ms = ms_value
            .parse::<f64>()
            .map_err(|_| CssError::ParseError("Invalid delay value".to_string()))?;

        Ok(TransitionDelay {
            delay: delay_ms / 1000.0,
        })
    } else if let Some(s_value) = input.strip_suffix('s') {
        let s_value = s_value.trim();

        let delay = s_value
            .parse::<f64>()
            .map_err(|_| CssError::ParseError("Invalid delay value".to_string()))?;

        Ok(TransitionDelay { delay })
    } else {
        Err(CssError::ParseError(
            "Delay must end with 's' or 'ms'".to_string(),
        ))
    }
}

/// Parse transition shorthand property
///
/// # Examples
/// ```
/// use css_transitions::parse_transition;
///
/// let transition = parse_transition("opacity 0.3s ease").unwrap();
/// ```
pub fn parse_transition(input: &str) -> Result<Transition, CssError> {
    let input = input.trim();

    if input.is_empty() {
        return Err(CssError::ParseError("Empty transition".to_string()));
    }

    // Extract timing functions first (they may contain spaces)
    let (parts, timing_function) = extract_timing_function(input)?;

    if parts.is_empty() {
        return Err(CssError::ParseError("Empty transition".to_string()));
    }

    // Parse remaining components
    let mut property = None;
    let mut duration = None;
    let mut delay = None;

    for part in parts {
        // Try to parse as duration/delay (must have s or ms)
        if part.ends_with('s') || part.ends_with("ms") {
            if duration.is_none() {
                duration = Some(parse_transition_duration(&part)?);
            } else if delay.is_none() {
                delay = Some(parse_transition_delay(&part)?);
            } else {
                return Err(CssError::ParseError(
                    "Too many time values in transition".to_string(),
                ));
            }
        }
        // Try to parse as timing function keyword
        else if matches!(
            part.as_str(),
            "ease" | "linear" | "ease-in" | "ease-out" | "ease-in-out"
        ) {
            // Already handled by extract_timing_function
            continue;
        }
        // Otherwise, it's a property name
        else {
            if property.is_some() {
                return Err(CssError::ParseError(
                    "Multiple properties in transition shorthand".to_string(),
                ));
            }
            property = Some(parse_transition_property(&part)?);
        }
    }

    Ok(Transition {
        property: property.unwrap_or(TransitionProperty::All),
        duration: duration.ok_or_else(|| {
            CssError::ParseError("Duration is required in transition".to_string())
        })?,
        timing_function: timing_function.unwrap_or(TransitionTimingFunction::Ease),
        delay: delay.unwrap_or(TransitionDelay { delay: 0.0 }),
    })
}

/// Extract timing function from transition string, handling functions with spaces
fn extract_timing_function(
    input: &str,
) -> Result<(Vec<String>, Option<TransitionTimingFunction>), CssError> {
    let mut timing_function = None;
    let mut remaining_parts = Vec::new();
    let mut current_token = String::new();
    let mut in_function = false;

    for ch in input.chars() {
        if ch == '(' {
            in_function = true;
            current_token.push(ch);
        } else if ch == ')' {
            in_function = false;
            current_token.push(ch);
            // Parse the function
            if current_token.starts_with("cubic-bezier(") || current_token.starts_with("steps(") {
                timing_function = Some(parse_transition_timing_function(&current_token)?);
                current_token.clear();
            }
        } else if ch.is_whitespace() && !in_function {
            if !current_token.is_empty() {
                remaining_parts.push(current_token.clone());
                current_token.clear();
            }
        } else {
            current_token.push(ch);
        }
    }

    if !current_token.is_empty() {
        remaining_parts.push(current_token);
    }

    Ok((remaining_parts, timing_function))
}

// ============================================================================
// Value Interpolation
// ============================================================================

/// Interpolate between two property values
///
/// # Examples
/// ```
/// use css_transitions::{interpolate_value, PropertyValue, TransitionTimingFunction};
/// use css_types::Length;
///
/// let start = PropertyValue::Number(0.0);
/// let end = PropertyValue::Number(100.0);
/// let result = interpolate_value(&start, &end, 0.5, &TransitionTimingFunction::Linear);
/// ```
pub fn interpolate_value(
    start: &PropertyValue,
    end: &PropertyValue,
    progress: f64,
    timing_function: &TransitionTimingFunction,
) -> PropertyValue {
    // Apply timing function to progress
    let eased_progress = evaluate_timing_function(timing_function, progress);

    match (start, end) {
        (PropertyValue::Number(s), PropertyValue::Number(e)) => {
            PropertyValue::Number(s + (e - s) * eased_progress)
        }
        (PropertyValue::Percentage(s), PropertyValue::Percentage(e)) => {
            PropertyValue::Percentage(s + (e - s) * eased_progress as f32)
        }
        (PropertyValue::Length(s), PropertyValue::Length(e)) => {
            // Interpolate lengths (assuming same unit)
            PropertyValue::Length(interpolate_length(s, e, eased_progress))
        }
        (PropertyValue::Color(s), PropertyValue::Color(e)) => {
            PropertyValue::Color(interpolate_color(s, e, eased_progress))
        }
        // If types don't match, return end value (discrete transition)
        _ => end.clone(),
    }
}

/// Interpolate between two lengths
fn interpolate_length(start: &Length, end: &Length, progress: f64) -> Length {
    // For simplicity, just interpolate the value
    // In a real implementation, we'd need to handle unit conversion
    let start_val = start.value();
    let end_val = end.value();
    let interpolated = start_val + (end_val - start_val) * progress as f32;

    Length::new(interpolated, start.unit())
}

/// Interpolate between two colors
fn interpolate_color(start: &Color, end: &Color, progress: f64) -> Color {
    let r = (start.r() as f64 + (end.r() as f64 - start.r() as f64) * progress) as u8;
    let g = (start.g() as f64 + (end.g() as f64 - start.g() as f64) * progress) as u8;
    let b = (start.b() as f64 + (end.b() as f64 - start.b() as f64) * progress) as u8;
    let a = start.a() + (end.a() - start.a()) * progress as f32;

    Color::rgba(r, g, b, a)
}

// ============================================================================
// Timing Function Evaluation
// ============================================================================

/// Evaluate timing function at given progress
///
/// # Examples
/// ```
/// use css_transitions::{evaluate_timing_function, TransitionTimingFunction};
///
/// let result = evaluate_timing_function(&TransitionTimingFunction::Linear, 0.5);
/// assert_eq!(result, 0.5);
/// ```
pub fn evaluate_timing_function(timing_function: &TransitionTimingFunction, progress: f64) -> f64 {
    // Clamp progress to [0, 1]
    let progress = progress.clamp(0.0, 1.0);

    match timing_function {
        TransitionTimingFunction::Linear => progress,
        TransitionTimingFunction::Ease => {
            // cubic-bezier(0.25, 0.1, 0.25, 1.0)
            evaluate_cubic_bezier(0.25, 0.1, 0.25, 1.0, progress)
        }
        TransitionTimingFunction::EaseIn => {
            // cubic-bezier(0.42, 0, 1.0, 1.0)
            evaluate_cubic_bezier(0.42, 0.0, 1.0, 1.0, progress)
        }
        TransitionTimingFunction::EaseOut => {
            // cubic-bezier(0, 0, 0.58, 1.0)
            evaluate_cubic_bezier(0.0, 0.0, 0.58, 1.0, progress)
        }
        TransitionTimingFunction::EaseInOut => {
            // cubic-bezier(0.42, 0, 0.58, 1.0)
            evaluate_cubic_bezier(0.42, 0.0, 0.58, 1.0, progress)
        }
        TransitionTimingFunction::CubicBezier { x1, y1, x2, y2 } => {
            evaluate_cubic_bezier(*x1, *y1, *x2, *y2, progress)
        }
        TransitionTimingFunction::Steps { count, position } => {
            evaluate_steps(*count, *position, progress)
        }
    }
}

/// Evaluate cubic bezier curve
fn evaluate_cubic_bezier(x1: f64, y1: f64, x2: f64, y2: f64, t: f64) -> f64 {
    // Simplified cubic bezier evaluation using Newton's method
    // For production, use a more robust algorithm

    // Binary search for t value that gives us the desired x coordinate
    let mut lower = 0.0;
    let mut upper = 1.0;
    let mut current_t = t;

    for _ in 0..10 {
        // 10 iterations should be enough
        let current_x = cubic_bezier_x(x1, x2, current_t);
        if (current_x - t).abs() < 0.001 {
            break;
        }

        if current_x < t {
            lower = current_t;
        } else {
            upper = current_t;
        }
        current_t = (lower + upper) / 2.0;
    }

    // Calculate y for the found t
    cubic_bezier_y(y1, y2, current_t)
}

/// Calculate x coordinate of cubic bezier at t
fn cubic_bezier_x(x1: f64, x2: f64, t: f64) -> f64 {
    // Cubic bezier: B(t) = (1-t)³ * P0 + 3(1-t)² * t * P1 + 3(1-t) * t² * P2 + t³ * P3
    // P0 = (0, 0), P1 = (x1, y1), P2 = (x2, y2), P3 = (1, 1)
    3.0 * (1.0 - t) * (1.0 - t) * t * x1 + 3.0 * (1.0 - t) * t * t * x2 + t * t * t
}

/// Calculate y coordinate of cubic bezier at t
fn cubic_bezier_y(y1: f64, y2: f64, t: f64) -> f64 {
    3.0 * (1.0 - t) * (1.0 - t) * t * y1 + 3.0 * (1.0 - t) * t * t * y2 + t * t * t
}

/// Evaluate steps timing function
fn evaluate_steps(count: u32, position: StepPosition, progress: f64) -> f64 {
    if progress >= 1.0 {
        return 1.0;
    }
    if progress <= 0.0 {
        return 0.0;
    }

    let steps = count as f64;
    match position {
        StepPosition::Start => ((progress * steps).ceil() / steps).min(1.0),
        StepPosition::End => {
            // For "end", boundaries belong to the previous interval
            // Subtract tiny epsilon to handle exact boundary cases
            let adjusted = (progress * steps - 1e-10).max(0.0);
            (adjusted.floor() / steps).min(1.0)
        }
    }
}

// ============================================================================
// Transition Engine Trait
// ============================================================================

/// Transition management interface
pub trait TransitionEngine {
    /// Start a new transition
    fn start_transition(
        &self,
        property: &str,
        start_value: PropertyValue,
        end_value: PropertyValue,
        transition: &Transition,
        current_time: f64,
    ) -> TransitionState;

    /// Tick a transition and get current value
    fn tick_transition(&self, state: &TransitionState, current_time: f64) -> Option<PropertyValue>;

    /// Check if transition is complete
    fn is_transition_complete(&self, state: &TransitionState, current_time: f64) -> bool;
}

/// Default implementation of transition engine
pub struct DefaultTransitionEngine;

impl TransitionEngine for DefaultTransitionEngine {
    fn start_transition(
        &self,
        property: &str,
        start_value: PropertyValue,
        end_value: PropertyValue,
        transition: &Transition,
        current_time: f64,
    ) -> TransitionState {
        TransitionState {
            property: property.to_string(),
            start_value,
            end_value,
            start_time: current_time + transition.delay.delay,
            duration: transition.duration.duration,
            timing_function: transition.timing_function,
        }
    }

    fn tick_transition(&self, state: &TransitionState, current_time: f64) -> Option<PropertyValue> {
        // Check if we're before the start time (still in delay)
        if current_time < state.start_time {
            return Some(state.start_value.clone());
        }

        // Check if transition is complete
        if current_time >= state.start_time + state.duration {
            return Some(state.end_value.clone());
        }

        // Calculate progress
        let elapsed = current_time - state.start_time;
        let progress = elapsed / state.duration;

        // Interpolate value
        Some(interpolate_value(
            &state.start_value,
            &state.end_value,
            progress,
            &state.timing_function,
        ))
    }

    fn is_transition_complete(&self, state: &TransitionState, current_time: f64) -> bool {
        current_time >= state.start_time + state.duration
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // Transition Property Parsing Tests
    // ========================================================================

    #[test]
    fn test_parse_transition_property_all() {
        let result = parse_transition_property("all").unwrap();
        assert_eq!(result, TransitionProperty::All);
    }

    #[test]
    fn test_parse_transition_property_none() {
        let result = parse_transition_property("none").unwrap();
        assert_eq!(result, TransitionProperty::None);
    }

    #[test]
    fn test_parse_transition_property_single() {
        let result = parse_transition_property("opacity").unwrap();
        assert_eq!(result, TransitionProperty::Property("opacity".to_string()));
    }

    #[test]
    fn test_parse_transition_property_multiple() {
        let result = parse_transition_property("opacity, transform").unwrap();
        assert_eq!(
            result,
            TransitionProperty::Multiple(vec!["opacity".to_string(), "transform".to_string()])
        );
    }

    #[test]
    fn test_parse_transition_property_empty() {
        let result = parse_transition_property("");
        assert!(result.is_err());
    }

    // ========================================================================
    // Duration Parsing Tests
    // ========================================================================

    #[test]
    fn test_parse_duration_seconds() {
        let result = parse_transition_duration("0.3s").unwrap();
        assert_eq!(result.duration, 0.3);
    }

    #[test]
    fn test_parse_duration_milliseconds() {
        let result = parse_transition_duration("300ms").unwrap();
        assert_eq!(result.duration, 0.3);
    }

    #[test]
    fn test_parse_duration_zero() {
        let result = parse_transition_duration("0s").unwrap();
        assert_eq!(result.duration, 0.0);
    }

    #[test]
    fn test_parse_duration_negative() {
        let result = parse_transition_duration("-1s");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_duration_no_unit() {
        let result = parse_transition_duration("0.3");
        assert!(result.is_err());
    }

    // ========================================================================
    // Timing Function Parsing Tests
    // ========================================================================

    #[test]
    fn test_parse_timing_function_ease() {
        let result = parse_transition_timing_function("ease").unwrap();
        assert_eq!(result, TransitionTimingFunction::Ease);
    }

    #[test]
    fn test_parse_timing_function_linear() {
        let result = parse_transition_timing_function("linear").unwrap();
        assert_eq!(result, TransitionTimingFunction::Linear);
    }

    #[test]
    fn test_parse_timing_function_cubic_bezier() {
        let result = parse_transition_timing_function("cubic-bezier(0.4, 0, 0.2, 1)").unwrap();
        assert_eq!(
            result,
            TransitionTimingFunction::CubicBezier {
                x1: 0.4,
                y1: 0.0,
                x2: 0.2,
                y2: 1.0
            }
        );
    }

    #[test]
    fn test_parse_timing_function_steps() {
        let result = parse_transition_timing_function("steps(4, end)").unwrap();
        assert_eq!(
            result,
            TransitionTimingFunction::Steps {
                count: 4,
                position: StepPosition::End
            }
        );
    }

    #[test]
    fn test_parse_timing_function_steps_default() {
        let result = parse_transition_timing_function("steps(4)").unwrap();
        assert_eq!(
            result,
            TransitionTimingFunction::Steps {
                count: 4,
                position: StepPosition::End
            }
        );
    }

    // ========================================================================
    // Delay Parsing Tests
    // ========================================================================

    #[test]
    fn test_parse_delay_seconds() {
        let result = parse_transition_delay("0.5s").unwrap();
        assert_eq!(result.delay, 0.5);
    }

    #[test]
    fn test_parse_delay_milliseconds() {
        let result = parse_transition_delay("500ms").unwrap();
        assert_eq!(result.delay, 0.5);
    }

    #[test]
    fn test_parse_delay_negative() {
        let result = parse_transition_delay("-1s").unwrap();
        assert_eq!(result.delay, -1.0);
    }

    // ========================================================================
    // Transition Shorthand Parsing Tests
    // ========================================================================

    #[test]
    fn test_parse_transition_simple() {
        let result = parse_transition("opacity 0.3s ease").unwrap();
        assert_eq!(
            result.property,
            TransitionProperty::Property("opacity".to_string())
        );
        assert_eq!(result.duration.duration, 0.3);
        assert_eq!(result.timing_function, TransitionTimingFunction::Ease);
        assert_eq!(result.delay.delay, 0.0);
    }

    #[test]
    fn test_parse_transition_with_delay() {
        let result = parse_transition("all 1s cubic-bezier(0.4, 0, 0.2, 1) 0.5s").unwrap();
        assert_eq!(result.property, TransitionProperty::All);
        assert_eq!(result.duration.duration, 1.0);
        assert_eq!(
            result.timing_function,
            TransitionTimingFunction::CubicBezier {
                x1: 0.4,
                y1: 0.0,
                x2: 0.2,
                y2: 1.0
            }
        );
        assert_eq!(result.delay.delay, 0.5);
    }

    #[test]
    fn test_parse_transition_minimal() {
        let result = parse_transition("0.3s").unwrap();
        assert_eq!(result.property, TransitionProperty::All);
        assert_eq!(result.duration.duration, 0.3);
    }

    // ========================================================================
    // Value Interpolation Tests
    // ========================================================================

    #[test]
    fn test_interpolate_number() {
        let start = PropertyValue::Number(0.0);
        let end = PropertyValue::Number(100.0);
        let result = interpolate_value(&start, &end, 0.5, &TransitionTimingFunction::Linear);

        match result {
            PropertyValue::Number(val) => assert!((val - 50.0).abs() < 0.01),
            _ => panic!("Expected Number"),
        }
    }

    #[test]
    fn test_interpolate_percentage() {
        let start = PropertyValue::Percentage(0.0);
        let end = PropertyValue::Percentage(100.0);
        let result = interpolate_value(&start, &end, 0.5, &TransitionTimingFunction::Linear);

        match result {
            PropertyValue::Percentage(val) => assert!((val - 50.0).abs() < 0.01),
            _ => panic!("Expected Percentage"),
        }
    }

    #[test]
    fn test_interpolate_color() {
        let start = PropertyValue::Color(Color::rgb(0, 0, 0));
        let end = PropertyValue::Color(Color::rgb(255, 255, 255));
        let result = interpolate_value(&start, &end, 0.5, &TransitionTimingFunction::Linear);

        match result {
            PropertyValue::Color(color) => {
                // Should be approximately halfway
                assert!((color.r() as i32 - 127).abs() <= 1);
                assert!((color.g() as i32 - 127).abs() <= 1);
                assert!((color.b() as i32 - 127).abs() <= 1);
            }
            _ => panic!("Expected Color"),
        }
    }

    // ========================================================================
    // Timing Function Evaluation Tests
    // ========================================================================

    #[test]
    fn test_evaluate_linear() {
        let result = evaluate_timing_function(&TransitionTimingFunction::Linear, 0.5);
        assert_eq!(result, 0.5);
    }

    #[test]
    fn test_evaluate_linear_bounds() {
        assert_eq!(
            evaluate_timing_function(&TransitionTimingFunction::Linear, 0.0),
            0.0
        );
        assert_eq!(
            evaluate_timing_function(&TransitionTimingFunction::Linear, 1.0),
            1.0
        );
    }

    #[test]
    fn test_evaluate_ease() {
        let result = evaluate_timing_function(&TransitionTimingFunction::Ease, 0.5);
        // Ease curve should be > 0.5 at midpoint (accelerates then decelerates)
        assert!(result > 0.5);
        assert!(result < 1.0);
    }

    #[test]
    fn test_evaluate_steps() {
        let timing = TransitionTimingFunction::Steps {
            count: 4,
            position: StepPosition::End,
        };

        assert_eq!(evaluate_timing_function(&timing, 0.0), 0.0);
        assert_eq!(evaluate_timing_function(&timing, 0.1), 0.0);
        assert_eq!(evaluate_timing_function(&timing, 0.25), 0.0);
        assert_eq!(evaluate_timing_function(&timing, 0.26), 0.25);
        assert_eq!(evaluate_timing_function(&timing, 0.5), 0.25);
        assert_eq!(evaluate_timing_function(&timing, 0.51), 0.5);
        assert_eq!(evaluate_timing_function(&timing, 1.0), 1.0);
    }

    // ========================================================================
    // TransitionEngine Tests
    // ========================================================================

    #[test]
    fn test_start_transition() {
        let engine = DefaultTransitionEngine;
        let transition = Transition {
            property: TransitionProperty::Property("opacity".to_string()),
            duration: TransitionDuration { duration: 1.0 },
            timing_function: TransitionTimingFunction::Linear,
            delay: TransitionDelay { delay: 0.0 },
        };

        let state = engine.start_transition(
            "opacity",
            PropertyValue::Number(0.0),
            PropertyValue::Number(1.0),
            &transition,
            0.0,
        );

        assert_eq!(state.property, "opacity");
        assert_eq!(state.start_time, 0.0);
        assert_eq!(state.duration, 1.0);
    }

    #[test]
    fn test_tick_transition_during() {
        let engine = DefaultTransitionEngine;
        let state = TransitionState {
            property: "opacity".to_string(),
            start_value: PropertyValue::Number(0.0),
            end_value: PropertyValue::Number(1.0),
            start_time: 0.0,
            duration: 1.0,
            timing_function: TransitionTimingFunction::Linear,
        };

        let value = engine.tick_transition(&state, 0.5).unwrap();
        match value {
            PropertyValue::Number(val) => assert!((val - 0.5).abs() < 0.01),
            _ => panic!("Expected Number"),
        }
    }

    #[test]
    fn test_tick_transition_complete() {
        let engine = DefaultTransitionEngine;
        let state = TransitionState {
            property: "opacity".to_string(),
            start_value: PropertyValue::Number(0.0),
            end_value: PropertyValue::Number(1.0),
            start_time: 0.0,
            duration: 1.0,
            timing_function: TransitionTimingFunction::Linear,
        };

        let value = engine.tick_transition(&state, 1.5).unwrap();
        match value {
            PropertyValue::Number(val) => assert_eq!(val, 1.0),
            _ => panic!("Expected Number"),
        }
    }

    #[test]
    fn test_is_transition_complete() {
        let engine = DefaultTransitionEngine;
        let state = TransitionState {
            property: "opacity".to_string(),
            start_value: PropertyValue::Number(0.0),
            end_value: PropertyValue::Number(1.0),
            start_time: 0.0,
            duration: 1.0,
            timing_function: TransitionTimingFunction::Linear,
        };

        assert!(!engine.is_transition_complete(&state, 0.5));
        assert!(engine.is_transition_complete(&state, 1.0));
        assert!(engine.is_transition_complete(&state, 1.5));
    }

    #[test]
    fn test_transition_with_delay() {
        let engine = DefaultTransitionEngine;
        let transition = Transition {
            property: TransitionProperty::Property("opacity".to_string()),
            duration: TransitionDuration { duration: 1.0 },
            timing_function: TransitionTimingFunction::Linear,
            delay: TransitionDelay { delay: 0.5 },
        };

        let state = engine.start_transition(
            "opacity",
            PropertyValue::Number(0.0),
            PropertyValue::Number(1.0),
            &transition,
            0.0,
        );

        // During delay, should return start value
        let value = engine.tick_transition(&state, 0.25).unwrap();
        match value {
            PropertyValue::Number(val) => assert_eq!(val, 0.0),
            _ => panic!("Expected Number"),
        }

        // After delay, should interpolate
        let value = engine.tick_transition(&state, 1.0).unwrap();
        match value {
            PropertyValue::Number(val) => assert!((val - 0.5).abs() < 0.01),
            _ => panic!("Expected Number"),
        }
    }
}
