# CSS Transforms

CSS transform property parsing and computation (2D and 3D transforms).

## Features

- Parse CSS `transform` property values
- Parse CSS `transform-origin` property values
- Compute 4x4 transformation matrices
- Support for all CSS transform functions:
  - 2D: `translate()`, `rotate()`, `scale()`, `skew()`, `matrix()`
  - 3D: `translate3d()`, `rotateX()`, `rotateY()`, `rotateZ()`, `rotate3d()`, `scale3d()`, `matrix3d()`, `perspective()`
- Transform composition (multiple functions)
- Transform origin application

## Usage

```rust
use css_transforms::{parse_transform, parse_transform_origin, compute_transform_matrix, apply_transform_origin, Rect};

// Parse a transform
let transform = parse_transform("translate(10px, 20px) rotate(45deg)").unwrap();

// Parse a transform origin
let origin = parse_transform_origin("center center").unwrap();

// Compute the transformation matrix
let rect = Rect { x: 0.0, y: 0.0, width: 100.0, height: 100.0 };
let mut matrix = compute_transform_matrix(&transform, &rect);

// Apply the transform origin
apply_transform_origin(&mut matrix, &origin, &rect);
```

## Transform Functions Supported

### 2D Transforms
- `translate(x, y)` - Translate in 2D
- `translateX(x)` - Translate on X axis
- `translateY(y)` - Translate on Y axis
- `scale(x, y)` - Scale in 2D
- `scale(n)` - Uniform scale
- `scaleX(x)` - Scale on X axis
- `scaleY(y)` - Scale on Y axis
- `rotate(angle)` - Rotate in 2D
- `skew(x, y)` - Skew in 2D
- `skewX(angle)` - Skew on X axis
- `skewY(angle)` - Skew on Y axis
- `matrix(a, b, c, d, tx, ty)` - 2D matrix

### 3D Transforms
- `translate3d(x, y, z)` - Translate in 3D
- `translateZ(z)` - Translate on Z axis
- `scale3d(x, y, z)` - Scale in 3D
- `scaleZ(z)` - Scale on Z axis
- `rotateX(angle)` - Rotate around X axis
- `rotateY(angle)` - Rotate around Y axis
- `rotateZ(angle)` - Rotate around Z axis
- `rotate3d(x, y, z, angle)` - Rotate around arbitrary axis
- `matrix3d(...)` - 3D matrix (16 values)
- `perspective(d)` - Apply perspective

## Transform Origin

Supports keywords and values:
- Keywords: `left`, `right`, `top`, `bottom`, `center`
- Lengths: `10px`, `2em`, etc.
- Percentages: `50%`, `25%`, etc.
- 2D: `center center`
- 3D: `center center 0px`

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
