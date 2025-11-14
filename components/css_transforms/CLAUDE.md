# css_transforms Component

## ⚠️ VERSION CONTROL RESTRICTIONS
- ❌ NEVER change project version to 1.0.0
- ❌ NEVER declare system "production ready"
- ✅ Complete your component work with high quality

**Type**: feature
**Tech Stack**: Rust 2021
**Project**: Corten CSS Engine v0.1.0

You are building ONLY the `css_transforms` component.

## Component Overview

### Purpose
CSS transform property parsing and computation (2D and 3D transforms including translate, rotate, scale, skew, matrix, perspective).

### Contract
Read `../../contracts/css_transforms.yaml` for complete API specification.

### Dependencies
- `css_types` (from components/css_types/)
- `css_parser_core` (from components/css_parser_core/)

### Boundaries
- ✅ Work ONLY in `components/css_transforms/`
- ✅ Import other components' public APIs
- ✅ Read specification: `../../css-engine-specification.md`
- ✅ Read contracts: `../../contracts/css_transforms.yaml`
- ❌ DO NOT modify other components' files

## MANDATORY: Test-Driven Development (TDD)

1. **RED**: Write failing test first
2. **GREEN**: Write minimal code to pass
3. **REFACTOR**: Improve while keeping tests passing

### Commit Pattern
```bash
git commit -m "[css_transforms] test: add transform parsing tests"
git commit -m "[css_transforms] feat: implement transform parsing (makes tests pass)"
git commit -m "[css_transforms] refactor: extract matrix computation"
```

## Quality Standards

- **Test Coverage**: ≥80% (target: 95%)
- **All Tests Pass**: 100% pass rate (ZERO failures)
- **Linting**: `cargo clippy` - zero warnings
- **Formatting**: `cargo fmt` - 100% formatted
- **Documentation**: All public APIs must have doc comments

## Implementation Requirements

### Core Features (from contract)
1. Transform function parsing (translate, rotate, scale, skew, matrix)
2. 3D transform functions (translate3d, rotate3d, scale3d, perspective)
3. Transform origin parsing
4. Transform matrix computation (4x4 matrices)
5. Transform composition (multiple functions)

### Test Requirements
- Unit tests for ALL transform functions
- Matrix computation verification
- 3D transform correctness
- Transform origin application
- Performance targets: <100μs parse, <200μs compute

## Development Workflow

1. Read contract: `../../contracts/css_transforms.yaml`
2. Create types matching contract
3. Write tests for each function (RED)
4. Implement to pass tests (GREEN)
5. Refactor for quality (REFACTOR)
6. Run `cargo test` - must show 100% pass
7. Run `cargo clippy` - must show zero warnings
8. Commit work with component prefix

## Cargo.toml

```toml
[package]
name = "css-transforms"
version = "0.1.0"
edition = "2021"

[dependencies]
css-types = { path = "../css_types" }
css-parser-core = { path = "../css_parser_core" }

[dev-dependencies]
```

Start implementing now. Follow TDD strictly.
