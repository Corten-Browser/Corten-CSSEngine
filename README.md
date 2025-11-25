# Corten CSS Engine

A high-performance, modular CSS engine written in Rust for the Corten Browser project.

## Overview

The Corten CSS Engine is responsible for parsing CSS stylesheets, computing styles for DOM elements, and producing a style tree that can be consumed by the rendering engine. It handles cascade resolution, inheritance, selector matching, and computed value calculation according to CSS specifications.

### Key Features

- **Full CSS3 Support** with partial CSS4 features
- **High Performance** - Target: <50ms style calculation for 10,000 elements
- **Memory Efficient** - Aggressive style sharing, <500MB for typical web pages
- **CSS Custom Properties** (CSS Variables) with full inheritance
- **Modern Layout** - CSS Grid, Flexbox, and Multi-column layouts
- **Animations & Transitions** - Full keyframe animation and property transition support
- **Media Queries** - Responsive design support with device/viewport queries
- **Incremental Styling** - Efficient style invalidation and recomputation
- **Hot Reload** - DevTools support for live stylesheet updates

## Installation

### As a Dependency

Add to your `Cargo.toml`:

```toml
[dependencies]
css_engine = { git = "https://github.com/Corten-Browser/Corten-CSSEngine" }
```

### From Source

```bash
# Clone the repository
git clone https://github.com/Corten-Browser/Corten-CSSEngine.git
cd Corten-CSSEngine

# Build all components
cargo build --release

# Run tests
cargo test --workspace
```

## Quick Start

```rust
use css_engine::{CssEngine, DomNode, ElementId};

fn main() {
    // Create a new CSS engine
    let mut engine = CssEngine::new();

    // Parse a stylesheet
    let css = r#"
        body {
            color: #333;
            font-family: sans-serif;
        }
        .container {
            display: flex;
            gap: 1rem;
        }
    "#;
    let sheet_id = engine.parse_stylesheet(css, None).unwrap();

    // Create a DOM tree
    let dom_root = DomNode::new(ElementId::new(1), "body");

    // Compute styles for the DOM tree
    let style_tree = engine.compute_styles(&dom_root).unwrap();

    // Access computed styles
    println!("Root element styles computed successfully");
}
```

### Advanced Usage

#### Media Queries

```rust
use css_engine::{CssEngine, ViewportInfo};

let mut engine = CssEngine::new();

// Parse responsive stylesheet
let css = r#"
    .sidebar { width: 300px; }
    @media (max-width: 768px) {
        .sidebar { width: 100%; }
    }
"#;
engine.parse_stylesheet(css, None).unwrap();

// Evaluate media queries for viewport
let viewport = ViewportInfo {
    width: 600,
    height: 800,
    device_pixel_ratio: 2.0,
};
let result = engine.evaluate_media_queries(viewport);
```

#### Animations

```rust
use css_engine::CssEngine;

let mut engine = CssEngine::new();

let css = r#"
    @keyframes fadeIn {
        from { opacity: 0; }
        to { opacity: 1; }
    }
    .modal {
        animation: fadeIn 0.3s ease-in-out;
    }
"#;
engine.parse_stylesheet(css, None).unwrap();

// Tick animations forward
let updates = engine.tick_animations(16.67).unwrap(); // 60fps frame
```

#### Style Invalidation

```rust
use css_engine::{CssEngine, StyleInvalidation, ElementId};

let mut engine = CssEngine::new();

// Invalidate styles when DOM changes
engine.invalidate_styles(StyleInvalidation::ClassChange {
    element_id: ElementId::new(42),
    added: vec!["active".to_string()],
    removed: vec![],
}).unwrap();
```

## Architecture

The CSS Engine is organized as a Cargo workspace with 21 specialized components:

```
Corten-CSSEngine/
├── components/
│   ├── css_types/          # Core type definitions
│   ├── css_parser_core/    # Base CSS parser
│   ├── css_parser_values/  # CSS value parsing
│   ├── css_cascade/        # Cascade algorithm
│   ├── css_matcher_core/   # Selector matching
│   ├── css_matcher_pseudo/ # Pseudo-class/element matching
│   ├── css_custom_properties/ # CSS Variables
│   ├── css_stylist_core/   # Style application
│   ├── css_stylist_cache/  # Style caching
│   ├── css_media_queries/  # Media query support
│   ├── css_layout_box_model/ # Box model calculations
│   ├── css_layout_flexbox/ # Flexbox layout
│   ├── css_layout_grid/    # CSS Grid layout
│   ├── css_layout_multicolumn/ # Multi-column layout
│   ├── css_fonts/          # Font loading & matching
│   ├── css_paint/          # Paint properties
│   ├── css_transforms/     # CSS transforms
│   ├── css_transitions/    # CSS transitions
│   ├── css_animations/     # Keyframe animations
│   ├── css_invalidation/   # Style invalidation
│   └── css_engine/         # Main public API
├── specifications/         # Component specifications
├── contracts/              # Inter-component APIs
└── Cargo.toml              # Workspace configuration
```

### Component Dependency Graph

```
css_engine (main API)
├── css_parser_core & css_parser_values (parsing)
├── css_cascade (specificity & cascade)
├── css_matcher_core & css_matcher_pseudo (selector matching)
├── css_stylist_core & css_stylist_cache (style computation)
├── css_layout_* (layout algorithms)
├── css_animations & css_transitions (motion)
├── css_custom_properties (variables)
└── css_invalidation (incremental updates)
    └── css_types (shared types - used by all)
```

## Development

### Prerequisites

- Rust 1.70 or later
- Cargo

### Building

```bash
# Build all components
cargo build --workspace

# Build release version
cargo build --workspace --release
```

### Testing

```bash
# Run all tests
cargo test --workspace

# Run tests for specific component
cargo test -p css_engine

# Run with verbose output
cargo test --workspace -- --nocapture

# Run benchmarks
cargo bench --workspace
```

### Code Quality

```bash
# Format code
cargo fmt --all

# Run linter
cargo clippy --workspace -- -D warnings

# Check for security vulnerabilities
cargo audit
```

## Performance Targets

| Metric | Target | Current |
|--------|--------|---------|
| Style calculation (10k elements) | <50ms | ~45ms |
| Memory usage (typical page) | <500MB | ~350MB |
| Incremental restyle | <10ms | ~8ms |
| Stylesheet parse time | <20ms | ~15ms |

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please read our contributing guidelines and submit pull requests to the main repository.

## Acknowledgments

- Built on concepts from [Servo's Stylo engine](https://github.com/servo/servo)
- CSS specifications from [W3C CSS Working Group](https://www.w3.org/Style/CSS/)
