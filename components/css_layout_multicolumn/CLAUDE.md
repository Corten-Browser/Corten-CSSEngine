# css_layout_multicolumn Component

## ⚠️ VERSION CONTROL RESTRICTIONS
- ❌ NEVER change project version to 1.0.0
- ❌ NEVER declare system "production ready"
- ✅ Complete your component work with high quality

**Type**: feature
**Tech Stack**: Rust 2021
**Project**: Corten CSS Engine v0.1.0

You are building ONLY the `css_layout_multicolumn` component.

## Component Overview

### Purpose
CSS multi-column layout computation (column-count, column-width, column-gap, column-rule, content balancing).

### Contract
Read `../../contracts/css_layout_multicolumn.yaml` for complete API specification.

### Dependencies
- `css_types` (from components/css_types/)
- `css_parser_core` (from components/css_parser_core/)

### Boundaries
- ✅ Work ONLY in `components/css_layout_multicolumn/`
- ✅ Import other components' public APIs
- ✅ Read contracts: `../../contracts/css_layout_multicolumn.yaml`
- ❌ DO NOT modify other components' files

## MANDATORY: Test-Driven Development (TDD)

1. **RED**: Write failing test first
2. **GREEN**: Write minimal code to pass
3. **REFACTOR**: Improve while keeping tests passing

### Commit Pattern
```bash
git commit -m "[css_layout_multicolumn] test: add column layout tests"
git commit -m "[css_layout_multicolumn] feat: implement column computation (makes tests pass)"
```

## Quality Standards

- **Test Coverage**: ≥80%
- **All Tests Pass**: 100% pass rate
- **Linting**: `cargo clippy` - zero warnings
- **Formatting**: `cargo fmt` - 100% formatted
- **Documentation**: All public APIs documented

## Implementation Requirements

### Core Features (from contract)
1. Column count parsing (auto, integer)
2. Column width parsing (auto, length)
3. Column gap parsing (normal, length)
4. Column rule parsing (width, style, color)
5. Column layout computation (determine actual columns)
6. Content balancing across columns

### Test Requirements
- Unit tests for all parsing functions
- Column layout computation tests
- Auto count/width calculation tests
- Performance target: <100μs compute

## Development Workflow

1. Read contract: `../../contracts/css_layout_multicolumn.yaml`
2. Create types matching contract
3. Write tests (RED)
4. Implement (GREEN)
5. Refactor
6. Verify: `cargo test` (100% pass), `cargo clippy` (zero warnings)
7. Commit with component prefix

## Cargo.toml

```toml
[package]
name = "css-layout-multicolumn"
version = "0.1.0"
edition = "2021"

[dependencies]
css-types = { path = "../css_types" }
css-parser-core = { path = "../css_parser_core" }
```

Start implementing now. Follow TDD strictly.
