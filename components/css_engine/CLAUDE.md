# css_engine Component

## ⚠️ VERSION CONTROL RESTRICTIONS
- ❌ NEVER change project version to 1.0.0
- ❌ NEVER declare system "production ready"
- ✅ Complete your component work with high quality

**Type**: integration (8000 lines estimated)
**Tech Stack**: Rust 2021
**Project**: Corten CSS Engine v0.1.0

You are building ONLY the `css_engine` component.

---

## Component Overview

### Purpose
Main CSS Engine - public API, orchestration

### Dependencies
- `css_types` (from components/css_types/)
- `css_parser_core` (from components/css_parser_core/)
- `css_parser_values` (from components/css_parser_values/)
- `css_cascade` (from components/css_cascade/)
- `css_matcher_core` (from components/css_matcher_core/)
- `css_matcher_pseudo` (from components/css_matcher_pseudo/)
- `css_stylist_core` (from components/css_stylist_core/)
- `css_stylist_cache` (from components/css_stylist_cache/)
- `css_invalidation` (from components/css_invalidation/)
- `css_layout_box_model` (from components/css_layout_box_model/)
- `css_layout_flexbox` (from components/css_layout_flexbox/)
- `css_layout_grid` (from components/css_layout_grid/)
- `css_animations` (from components/css_animations/)
- `css_custom_properties` (from components/css_custom_properties/)

### Boundaries
- ✅ Work ONLY in `components/css_engine/`
- ✅ Import other components' public APIs
- ✅ Read specification: `../../css-engine-specification.md`
- ✅ Read contracts: `../../contracts/`
- ❌ DO NOT modify other components' files
- ❌ DO NOT access private implementation details

---

## MANDATORY: Test-Driven Development (TDD)

### TDD Workflow (NON-NEGOTIABLE)

1. **RED**: Write failing test first
2. **GREEN**: Write minimal code to pass
3. **REFACTOR**: Improve while keeping tests passing

### TDD in Rust
```rust
// 1. RED - Write failing test
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_color_hex() {
        let result = parse_color("#FF5733");
        assert_eq!(result.unwrap(), Color::Rgb(255, 87, 51));
    }
}

// Run: cargo test (should FAIL - proves test works)

// 2. GREEN - Minimal implementation
pub fn parse_color(input: &str) -> Result<Color, ParseError> {
    // Implement just enough to pass
    if input.starts_with('#') {
        // Parse hex color
    }
    Ok(Color::Rgb(255, 87, 51))
}

// Run: cargo test (should PASS)

// 3. REFACTOR - Improve implementation
pub fn parse_color(input: &str) -> Result<Color, ParseError> {
    if !input.starts_with('#') {
        return Err(ParseError::InvalidFormat);
    }
    // Proper hex parsing with validation
    parse_hex_color(&input[1..])
}
```

### Commit Pattern
```bash
git commit -m "[css_engine] test: add color parsing tests"
git commit -m "[css_engine] feat: implement color parsing (makes tests pass)"
git commit -m "[css_engine] refactor: extract hex validation logic"
```

---

## Quality Standards

### Code Requirements
- **Test Coverage**: ≥80% (target: 95%)
- **All Tests Pass**: 100% pass rate (ZERO failures)
- **Linting**: `cargo clippy` - zero warnings
- **Formatting**: `cargo fmt` - 100% formatted
- **Complexity**: Keep functions focused and simple
- **Documentation**: All public APIs must have doc comments

### Rust-Specific Standards
```rust
// ✅ GOOD: Clear doc comments
/// Parse a CSS color value from string.
///
/// Supports hex colors (#RGB, #RRGGBB), named colors (red, blue),
/// and rgb()/rgba() functions.
///
/// # Examples
/// ```
/// let color = parse_color("#FF5733")?;
/// assert_eq!(color, Color::Rgb(255, 87, 51));
/// ```
///
/// # Errors
/// Returns `ParseError::InvalidFormat` if input is not valid CSS color.
pub fn parse_color(input: &str) -> Result<Color, ParseError> {
    // Implementation
}

// ✅ GOOD: Descriptive error types
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid color format: {0}")]
    InvalidFormat(String),
    
    #[error("Color value out of range: {0}")]
    OutOfRange(u32),
}

// ✅ GOOD: Use Result for fallible operations
pub fn parse_length(input: &str) -> Result<Length, ParseError> {
    // Implementation that can fail
}

// ❌ BAD: Panic in library code
pub fn parse_length(input: &str) -> Length {
    input.parse().unwrap()  // DON'T DO THIS
}
```

---

## Testing Standards

### Test Organization
```
tests/
├── unit/              # Unit tests (fast, isolated)
│   ├── parser_tests.rs
│   ├── types_tests.rs
│   └── utils_tests.rs
├── integration/       # Integration tests (cross-module)
│   └── api_tests.rs
└── fixtures/          # Test data
    └── test_stylesheets.css
```

### Test Quality
```rust
// ✅ GOOD: Specific, clear test
#[test]
fn test_parse_rgb_color_with_valid_input() {
    let result = parse_color("rgb(255, 87, 51)");
    assert_eq!(result.unwrap(), Color::Rgb(255, 87, 51));
}

#[test]
fn test_parse_rgb_color_rejects_invalid_values() {
    let result = parse_color("rgb(256, 0, 0)");
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ParseError::OutOfRange(_)));
}

// ❌ BAD: Vague test
#[test]
fn test_colors() {
    assert!(parse_color("red").is_ok());
}
```

### Benchmarks
```rust
// benches/parsing_bench.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_color_parsing(c: &mut Criterion) {
    c.bench_function("parse hex color", |b| {
        b.iter(|| parse_color(black_box("#FF5733")))
    });
}

criterion_group!(benches, benchmark_color_parsing);
criterion_main!(benches);
```

---

## Development Commands

### Testing
```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_parse_color

# Run with coverage (requires tarpaulin)
cargo tarpaulin --out Html

# Run benchmarks
cargo bench
```

### Quality Checks
```bash
# Linting
cargo clippy -- -D warnings

# Formatting
cargo fmt --check  # Check only
cargo fmt          # Apply formatting

# Documentation
cargo doc --open

# Build
cargo build --release
```

---

## Size Management

### Token Limits
- **Optimal**: <70,000 tokens (~7,000 lines)
- **Warning**: 70,000-90,000 tokens
- **STOP**: >110,000 tokens - alert orchestrator

### Check Size
```bash
find src/ -name "*.rs" | xargs wc -l
# If > 8,000 lines: approaching limits
# If > 10,000 lines: alert orchestrator
```

---

## Git Workflow

### Committing
```bash
# Stage only your component
git add components/css_engine/

# Commit with prefix
git commit -m "[css_engine] feat: implement selector matching"

# Or use retry wrapper
python ../../orchestration/git_retry.py "css_engine" "feat: implement selector matching"
```

---

## Definition of Done

A feature is complete when:
- [ ] Tests written FIRST (TDD verified in git history)
- [ ] All tests pass (100% pass rate)
- [ ] Coverage ≥80%
- [ ] `cargo clippy` passes (zero warnings)
- [ ] `cargo fmt` applied
- [ ] All public APIs have doc comments
- [ ] README updated if needed
- [ ] Committed to git with component prefix

---

## Breaking Changes Policy

**VERSION**: 0.1.0 (pre-release)
**POLICY**: Breaking changes ENCOURAGED

- ✅ Break and improve code freely
- ✅ Remove deprecated code immediately
- ✅ Refactor to better patterns
- ❌ DON'T maintain backwards compatibility during 0.x.x

---

## Architecture Principles

### SOLID Principles
- **Single Responsibility**: Each struct/function does ONE thing
- **Open/Closed**: Use traits for extensibility
- **Liskov Substitution**: Trait implementations must be substitutable
- **Interface Segregation**: Small, focused traits
- **Dependency Inversion**: Depend on traits, not concrete types

### Rust Best Practices
- Use `Result` for fallible operations (not `panic!`)
- Use `Option` for nullable values
- Prefer borrowing over cloning (use `&` references)
- Use `enum` for state/variants
- Implement `Display` and `Debug` for public types
- Use `thiserror` for error types

---

## Specification Reference

Read the full CSS Engine specification:
- `../../css-engine-specification.md`

Your component implements section(s) related to Main CSS Engine - public API, orchestration.

---

## Questions?

If you need to:
- Modify API contracts
- Access another component's private code
- Split this component
- Get requirement clarification

**Ask the orchestrator - do not proceed without approval.**

---

**Remember**: Quality is non-negotiable. TDD is mandatory. All tests must pass before completion.
