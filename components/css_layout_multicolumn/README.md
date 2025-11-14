# CSS Multi-Column Layout

CSS multi-column layout property parsing and computation.

## Features

- Parse CSS `column-count` property values (auto, integer)
- Parse CSS `column-width` property values (auto, length)
- Parse CSS `column-gap` property values (normal, length)
- Parse CSS `column-rule` property values (width, style, color)
- Compute actual column count and width from constraints
- Content balancing across columns
- Column span handling

## Usage

```rust
use css_layout_multicolumn::{
    parse_column_count, parse_column_width, parse_column_gap,
    compute_column_layout, MultiColumnLayout, ComputedColumns
};

// Parse column properties
let count = parse_column_count("3").unwrap();
let width = parse_column_width("200px").unwrap();
let gap = parse_column_gap("20px").unwrap();

// Create multi-column configuration
let config = MultiColumnLayout {
    column_count: count,
    column_width: width,
    column_gap: gap,
    column_rule_width: None,
    column_rule_style: None,
    column_rule_color: None,
};

// Compute actual column layout
let available_width = 800.0;
let computed = compute_column_layout(&config, available_width);

println!("Actual columns: {}", computed.actual_count);
println!("Column width: {}", computed.actual_width);
```

## Column Properties Supported

### column-count
- `auto` - Automatic column count
- `<integer>` - Specific column count (e.g., `3`, `4`)

### column-width
- `auto` - Automatic column width
- `<length>` - Specific column width (e.g., `200px`, `10em`)

### column-gap
- `normal` - Normal gap (1em)
- `<length>` - Specific gap (e.g., `20px`, `1rem`)

### column-rule
- `<column-rule-width>` - Border width (e.g., `thin`, `1px`)
- `<column-rule-style>` - Border style (e.g., `solid`, `dashed`)
- `<column-rule-color>` - Border color (e.g., `red`, `#ff0000`)

## Column Layout Algorithm

The algorithm determines actual columns based on:
1. Available width
2. Column count constraint
3. Column width constraint
4. Column gap
5. Content balancing requirements

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
