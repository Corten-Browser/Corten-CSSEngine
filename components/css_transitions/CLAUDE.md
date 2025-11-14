# css_transitions Component

## ⚠️ VERSION CONTROL RESTRICTIONS
- ❌ NEVER change project version to 1.0.0
- ❌ NEVER declare system "production ready"
- ✅ Complete your component work with high quality

**Type**: feature
**Tech Stack**: Rust 2021
**Project**: Corten CSS Engine v0.1.0

You are building ONLY the `css_transitions` component.

## Component Overview

### Purpose
CSS transitions for smooth property value changes (transition-property, transition-duration, transition-timing-function, transition-delay).

### Contract
Read `../../contracts/css_transitions.yaml` for complete API specification.

### Dependencies
- `css_types` (from components/css_types/)
- `css_parser_core` (from components/css_parser_core/)
- `css_animations` (from components/css_animations/)

### Boundaries
- ✅ Work ONLY in `components/css_transitions/`
- ✅ Import other components' public APIs
- ✅ Read contracts: `../../contracts/css_transitions.yaml`
- ❌ DO NOT modify other components' files

## MANDATORY: Test-Driven Development (TDD)

1. **RED**: Write failing test first
2. **GREEN**: Write minimal code to pass
3. **REFACTOR**: Improve while keeping tests passing

### Commit Pattern
```bash
git commit -m "[css_transitions] test: add transition parsing tests"
git commit -m "[css_transitions] feat: implement transition parsing (makes tests pass)"
git commit -m "[css_transitions] feat: implement value interpolation (makes tests pass)"
```

## Quality Standards

- **Test Coverage**: ≥80%
- **All Tests Pass**: 100% pass rate
- **Linting**: `cargo clippy` - zero warnings
- **Formatting**: `cargo fmt` - 100% formatted
- **Documentation**: All public APIs documented

## Implementation Requirements

### Core Features (from contract)
1. Transition property parsing (all, none, specific, multiple)
2. Duration parsing (s, ms)
3. Timing function parsing (ease, linear, cubic-bezier, steps)
4. Delay parsing
5. Value interpolation (length, color, number, percentage)
6. Timing function evaluation
7. Transition state management

### Test Requirements
- Unit tests for all parsing functions
- Interpolation tests for all value types
- Timing function evaluation tests
- Transition lifecycle tests (start → tick → complete)
- Performance targets: <10μs interpolate, <5μs timing eval

## Development Workflow

1. Read contract: `../../contracts/css_transitions.yaml`
2. Create types matching contract
3. Write tests (RED)
4. Implement (GREEN)
5. Refactor
6. Verify: `cargo test` (100% pass), `cargo clippy` (zero warnings)
7. Commit with component prefix

## Cargo.toml

```toml
[package]
name = "css-transitions"
version = "0.1.0"
edition = "2021"

[dependencies]
css-types = { path = "../css_types" }
css-parser-core = { path = "../css_parser_core" }
css-animations = { path = "../css_animations" }
```

Start implementing now. Follow TDD strictly.
