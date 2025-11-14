# css_media_queries Component

## ⚠️ VERSION CONTROL RESTRICTIONS
- ❌ NEVER change project version to 1.0.0
- ❌ NEVER declare system "production ready"
- ✅ Complete your component work with high quality

**Type**: core
**Tech Stack**: Rust 2021
**Project**: Corten CSS Engine v0.1.0

You are building ONLY the `css_media_queries` component.

## Component Overview

### Purpose
CSS media query parsing, evaluation, and viewport management (responsive design breakpoints, device queries, user preferences).

### Contract
Read `../../contracts/css_media_queries.yaml` for complete API specification.

### Dependencies
- `css_types` (from components/css_types/)
- `css_parser_core` (from components/css_parser_core/)

### Boundaries
- ✅ Work ONLY in `components/css_media_queries/`
- ✅ Import other components' public APIs
- ✅ Read contracts: `../../contracts/css_media_queries.yaml`
- ❌ DO NOT modify other components' files

## MANDATORY: Test-Driven Development (TDD)

1. **RED**: Write failing test first
2. **GREEN**: Write minimal code to pass
3. **REFACTOR**: Improve while keeping tests passing

### Commit Pattern
```bash
git commit -m "[css_media_queries] test: add media query parsing tests"
git commit -m "[css_media_queries] feat: implement media query parsing (makes tests pass)"
git commit -m "[css_media_queries] feat: implement query evaluation (makes tests pass)"
```

## Quality Standards

- **Test Coverage**: ≥80%
- **All Tests Pass**: 100% pass rate
- **Linting**: `cargo clippy` - zero warnings
- **Formatting**: `cargo fmt` - 100% formatted
- **Documentation**: All public APIs documented

## Implementation Requirements

### Core Features (from contract)
1. Media type parsing (all, screen, print, speech)
2. Media feature parsing (width, height, orientation, resolution, etc.)
3. User preference features (prefers-color-scheme, prefers-reduced-motion, prefers-contrast)
4. Range queries (min-width, max-width, min-height, max-height)
5. Logical operators (and, or, not)
6. Media query list parsing
7. Viewport information management
8. Media query evaluation

### Test Requirements
- Unit tests for all media types
- Tests for all media features
- Range query tests
- Logical operator tests
- Evaluation tests (true/false)
- Real-world responsive breakpoint tests
- Performance targets: <500μs parse, <50μs evaluate

## Development Workflow

1. Read contract: `../../contracts/css_media_queries.yaml`
2. Create types matching contract
3. Write tests (RED)
4. Implement (GREEN)
5. Refactor
6. Verify: `cargo test` (100% pass), `cargo clippy` (zero warnings)
7. Commit with component prefix

## Cargo.toml

```toml
[package]
name = "css-media-queries"
version = "0.1.0"
edition = "2021"

[dependencies]
css-types = { path = "../css_types" }
css-parser-core = { path = "../css_parser_core" }
```

Start implementing now. Follow TDD strictly.
