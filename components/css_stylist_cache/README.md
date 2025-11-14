# css_stylist_cache

**Type**: feature
**Tech Stack**: Rust 2021, lru crate
**Estimated Size**: ~6000 lines
**Version**: 0.1.0

## Responsibility

Style sharing and caching - LRU cache, bloom filter optimization

## Dependencies

- `css_types`
- `css_stylist_core`

## Structure

```
css_stylist_cache/
├── src/
│   └── lib.rs           # Public API
├── tests/
│   ├── unit/            # Unit tests
│   └── integration/     # Integration tests
├── benches/             # Performance benchmarks
├── Cargo.toml           # Rust dependencies
├── component.yaml       # Component manifest
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
cd components/css_stylist_cache
# Work directly in this directory
```

See CLAUDE.md for detailed development instructions and quality standards.
