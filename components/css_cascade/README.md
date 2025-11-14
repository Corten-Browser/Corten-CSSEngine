# css_cascade

**Type**: core
**Tech Stack**: Rust 2021
**Estimated Size**: ~10000 lines
**Version**: 0.1.0

## Responsibility

CSS cascade algorithm implementation including:
- Specificity calculation for CSS selectors
- Cascade resolution (origin, specificity, source order)
- Property inheritance
- !important declaration handling

## Dependencies

- `css_types` - Shared type definitions (Specificity, etc.)

## Implementation

### Public API

- **CascadeResolver**: Main resolver struct
  - `new()` - Create a new cascade resolver
  - `resolve(&[ApplicableRule])` - Resolve cascade for a set of rules
  - `compute_specificity(&Selector)` - Calculate selector specificity
  - `apply_inheritance(&ComputedValues, &mut ComputedValues)` - Apply property inheritance

### Types

- **ApplicableRule**: Rule with specificity, origin, and source order
- **Origin**: Rule origin enum (UserAgent, User, Author)
- **CascadeResult**: Result of cascade resolution with properties map
- **ComputedValues**: Container for computed property values
- **Selector**: Selector representation (Type, Class, Id, Compound, etc.)
- **PropertyId**: Property identifiers
- **PropertyValue**: Property value variants

## Structure

```
css_cascade/
├── src/
│   ├── lib.rs           # Public API and exports
│   ├── types.rs         # Type definitions
│   └── resolver.rs      # Cascade resolution logic
├── tests/
│   ├── unit/            # Unit tests
│   │   ├── specificity_tests.rs
│   │   ├── cascade_tests.rs
│   │   └── inheritance_tests.rs
│   └── integration_tests.rs
├── Cargo.toml           # Rust dependencies
├── CLAUDE.md            # Agent instructions
└── README.md            # This file
```

## Development

This component is developed using Test-Driven Development (TDD):

1. Write failing test
2. Implement minimal code to pass test
3. Refactor while keeping tests passing
4. Commit with TDD pattern

### Running Tests

```bash
# Run all tests
cargo test

# Run with coverage
cargo tarpaulin --out Html

# Run benchmarks
cargo bench
```

### Quality Standards

- Test coverage: ≥80%
- All tests passing: 100%
- Linting: cargo clippy
- Formatting: cargo fmt

## Integration

This component is part of the Corten CSS Engine orchestration system.

**Through Orchestrator**: The orchestrator launches agents to work on this component using the Task tool.

**Direct Work**: 
```bash
cd components/css_cascade
# Work directly in this directory
```

See CLAUDE.md for detailed development instructions and quality standards.
