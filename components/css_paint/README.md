# CSS Paint API (Houdini)

CSS Paint API implementation for custom paint worklets in the Corten CSS Engine.

## Overview

This component provides the framework for CSS Paint API (CSS Houdini Paint Worklets), enabling programmatic drawing for CSS backgrounds, borders, and other properties through the `paint()` CSS function.

## Features

- **PaintWorklet trait** - Define custom paint worklets
- **PaintContext** - 2D drawing API (Canvas-like)
- **PaintWorkletRegistry** - Register and execute worklets
- **paint() parser** - Parse CSS paint() function calls
- **Built-in worklets** - Example implementations (checkerboard, circle, rough-background)

## Usage

### Defining a Paint Worklet

```rust
use css_paint::{PaintWorklet, PaintContext, PaintSize, PaintValue, PaintArgumentType};
use css_types::Color;

struct GradientWorklet;

impl PaintWorklet for GradientWorklet {
    fn name(&self) -> &str {
        "custom-gradient"
    }

    fn input_properties(&self) -> Vec<String> {
        vec!["--gradient-color".to_string()]
    }

    fn input_arguments(&self) -> Vec<PaintArgumentType> {
        vec![PaintArgumentType::Color, PaintArgumentType::Color]
    }

    fn paint(&self, ctx: &mut PaintContext, size: PaintSize, args: &[PaintValue]) {
        let color1 = args.first().and_then(|a| a.as_color()).cloned()
            .unwrap_or_else(|| Color::rgb(255, 0, 0));
        let color2 = args.get(1).and_then(|a| a.as_color()).cloned()
            .unwrap_or_else(|| Color::rgb(0, 0, 255));

        // Draw gradient (simplified)
        ctx.set_fill_color(color1);
        ctx.fill_rect(0.0, 0.0, size.width, size.height / 2.0);
        ctx.set_fill_color(color2);
        ctx.fill_rect(0.0, size.height / 2.0, size.width, size.height / 2.0);
    }
}
```

### Registering Worklets

```rust
use css_paint::PaintWorkletRegistry;
use std::sync::Arc;

let mut registry = PaintWorkletRegistry::new();
registry.register(Arc::new(GradientWorklet)).unwrap();

// Or use built-in worklets
let registry = PaintWorkletRegistry::with_builtins();
```

### Parsing paint() Functions

```rust
use css_paint::parse_paint_function;

let parsed = parse_paint_function("paint(checkerboard, 20, #ff0000, #0000ff)").unwrap();
assert_eq!(parsed.name, "checkerboard");
assert_eq!(parsed.raw_arguments, vec!["20", "#ff0000", "#0000ff"]);
```

### Executing Paint Operations

```rust
use css_paint::{PaintWorkletRegistry, PaintSize, PaintValue};
use css_types::Color;

let registry = PaintWorkletRegistry::with_builtins();
let size = PaintSize::new(200.0, 100.0);
let args = vec![
    PaintValue::Number(20.0),
    PaintValue::Color(Color::rgb(255, 255, 255)),
    PaintValue::Color(Color::rgb(0, 0, 0)),
];

let ctx = registry.execute("checkerboard", size, &args).unwrap();
let operations = ctx.operations();
// Render operations using your graphics backend
```

## Paint Operations

The `PaintContext` provides Canvas-like 2D drawing operations:

- **Rectangles**: `fill_rect`, `stroke_rect`, `clear_rect`
- **Paths**: `begin_path`, `move_to`, `line_to`, `arc`, `bezier_curve_to`, `close_path`
- **Drawing**: `fill`, `stroke`, `clip`
- **Styles**: `set_fill_color`, `set_stroke_color`, `set_line_width`
- **Transforms**: `translate`, `rotate`, `scale`, `transform`
- **State**: `save`, `restore`

## Built-in Worklets

- **checkerboard** - Checkerboard pattern with configurable size and colors
- **circle** - Simple filled circle
- **rough-background** - Hand-drawn style border effect

## Argument Types

Worklets can accept various argument types:

- `Number` - Numeric values (e.g., `10`, `3.14`)
- `Length` - CSS lengths (e.g., `10px`, `2em`)
- `Color` - CSS colors (e.g., `#ff0000`, `rgb(255, 0, 0)`)
- `Percentage` - Percentages (e.g., `50%`)
- `Angle` - Angles (e.g., `45deg`, `0.5turn`)
- `Ident` - Identifiers/strings
- `Image` - Image URLs

## Note

This is a framework/structure implementation. Actual JavaScript worklet execution and integration with a graphics rendering backend are out of scope for this component.
