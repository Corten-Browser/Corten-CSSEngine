# CSS Transitions

CSS transition property parsing and value interpolation for smooth animations.

## Features

- Parse CSS `transition-property` values (all, none, specific properties)
- Parse CSS `transition-duration` values (time in seconds or milliseconds)
- Parse CSS `transition-timing-function` values (ease, linear, cubic-bezier, steps)
- Parse CSS `transition-delay` values (time in seconds or milliseconds)
- Parse shorthand `transition` property
- Interpolate property values over time
- Apply timing functions to control animation easing

## Usage

```rust
use css_transitions::{
    parse_transition_property, parse_transition_duration, parse_transition_timing_function,
    interpolate_value, TransitionProperty, PropertyValue, Duration
};

// Parse transition properties
let property = parse_transition_property("opacity").unwrap();
let duration = parse_transition_duration("300ms").unwrap();
let timing_fn = parse_transition_timing_function("ease-in-out").unwrap();

// Interpolate between two values
let start_value = PropertyValue::Number(0.0);
let end_value = PropertyValue::Number(1.0);
let progress = 0.5; // 50% through transition

let current = interpolate_value(&start_value, &end_value, progress, &timing_fn);
```

## Transition Properties Supported

### transition-property
- `all` - Transition all animatable properties
- `none` - No transitions
- `<property-name>` - Specific property (e.g., `opacity`, `transform`)
- Multiple properties: `opacity, transform, width`

### transition-duration
- `<time>` - Duration in seconds (e.g., `2s`) or milliseconds (e.g., `500ms`)
- Default: `0s` (instant)

### transition-timing-function
- `ease` - Slow start, fast middle, slow end
- `linear` - Constant speed
- `ease-in` - Slow start
- `ease-out` - Slow end
- `ease-in-out` - Slow start and end
- `cubic-bezier(x1, y1, x2, y2)` - Custom cubic bezier curve
- `steps(n, start|end)` - Step function with n steps

### transition-delay
- `<time>` - Delay before transition starts (e.g., `1s`, `200ms`)
- Default: `0s` (no delay)

## Interpolation

Supports interpolation for:
- Numbers (opacity, z-index, etc.)
- Lengths (width, height, margin, etc.)
- Colors (RGB, RGBA, hex)
- Transforms (translate, rotate, scale)
- Percentages
- Other animatable CSS values

## Timing Functions

### Cubic Bezier
- Control points define acceleration curve
- Common presets: ease, ease-in, ease-out, ease-in-out
- Custom curves: `cubic-bezier(0.4, 0.0, 0.2, 1.0)`

### Steps
- Discrete jumps instead of smooth interpolation
- `steps(4, start)` - 4 steps, jump at start
- `steps(4, end)` - 4 steps, jump at end

## Testing

```bash
# Run all tests
cargo test

# Run with coverage
cargo tarpaulin --out Html
```

## Documentation

```bash
cargo doc --open
```

## Quality Standards

- Test Coverage: ≥80% (target: 95%)
- All Tests Pass: 100% pass rate
- Linting: Zero clippy warnings
- Formatting: 100% formatted
