# CSS Media Queries

CSS media query parsing and evaluation for responsive design.

## Features

- Parse CSS `@media` rules and media queries
- Evaluate media queries against viewport information
- Support for all standard media features
- Range syntax support (e.g., `width >= 768px`)
- Logical operators: `and`, `or`, `not`
- Media types: `screen`, `print`, `all`
- Dynamic viewport updates

## Usage

```rust
use css_media_queries::{
    parse_media_query, evaluate_media_query, MediaQuery, ViewportInfo, Orientation
};

// Parse a media query
let query = parse_media_query("(min-width: 768px) and (orientation: landscape)").unwrap();

// Create viewport information
let viewport = ViewportInfo {
    width: 1024,
    height: 768,
    device_pixel_ratio: 2.0,
    orientation: Orientation::Landscape,
    prefers_color_scheme: ColorScheme::Light,
    prefers_reduced_motion: ReducedMotion::NoPreference,
};

// Evaluate the query
let matches = evaluate_media_query(&query, &viewport);
println!("Media query matches: {}", matches); // true
```

## Media Features Supported

### Viewport Dimensions
- `width` - Viewport width
- `min-width` - Minimum viewport width
- `max-width` - Maximum viewport width
- `height` - Viewport height
- `min-height` - Minimum viewport height
- `max-height` - Maximum viewport height

### Device Characteristics
- `orientation` - `portrait` or `landscape`
- `aspect-ratio` - Width-to-height ratio
- `device-pixel-ratio` - Device pixel density
- `resolution` - Display resolution (dpi, dpcm)

### User Preferences
- `prefers-color-scheme` - `light`, `dark`, or `no-preference`
- `prefers-reduced-motion` - `reduce` or `no-preference`
- `prefers-contrast` - `more`, `less`, or `no-preference`

### Media Types
- `screen` - Computer screens, tablets, phones
- `print` - Printers and print preview
- `all` - All devices (default)

## Range Syntax

Modern range syntax:
```css
/* Old syntax */
@media (min-width: 768px) and (max-width: 1024px) { }

/* New range syntax */
@media (768px <= width <= 1024px) { }
@media (width >= 768px) { }
```

## Logical Operators

### AND
```css
@media (min-width: 768px) and (orientation: landscape) { }
```

### OR (comma-separated)
```css
@media (min-width: 768px), (orientation: landscape) { }
```

### NOT
```css
@media not (min-width: 768px) { }
```

## Common Media Queries

### Mobile First
```css
/* Base styles */
@media (min-width: 768px) { /* Tablet */ }
@media (min-width: 1024px) { /* Desktop */ }
```

### Dark Mode
```css
@media (prefers-color-scheme: dark) { }
```

### Reduced Motion
```css
@media (prefers-reduced-motion: reduce) { }
```

### High DPI Displays
```css
@media (min-resolution: 2dppx) { }
```

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
