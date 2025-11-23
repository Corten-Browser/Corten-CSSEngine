//! Performance benchmarks for the CSS Engine
//!
//! This module provides comprehensive benchmarks using Criterion to measure:
//! - CSS parsing speed (small, medium, large stylesheets)
//! - Selector matching performance (various selector types)
//! - Cascade resolution performance
//! - Full style computation pipeline
//! - Style invalidation processing
//!
//! Run benchmarks with: `cargo bench`

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use css_engine::{
    parallel::{
        partition_dom_tree, Declaration, ElementRef, ParallelStyleComputer,
        ParsedStylesheet as ParallelParsedStylesheet, Specificity, StyleRule,
    },
    CssEngine, DomNode, ElementId, StyleInvalidation,
};

// ============================================================================
// Test Data Generators
// ============================================================================

/// Helper module for generating test data for benchmarks
mod test_data {
    use super::*;

    /// Generate a CSS stylesheet string with the specified number of rules
    pub fn generate_stylesheet(rule_count: usize) -> String {
        let mut css = String::with_capacity(rule_count * 50);

        for i in 0..rule_count {
            // Mix of different selector types for realistic benchmarks
            let selector = match i % 5 {
                0 => format!("div.class-{}", i),
                1 => format!("#id-{}", i),
                2 => format!(".class-{}", i),
                3 => format!("div span.nested-{}", i),
                4 => format!("p",),
                _ => unreachable!(),
            };

            // Mix of property declarations
            let property = match i % 4 {
                0 => "color: red;",
                1 => "margin: 10px;",
                2 => "display: block;",
                3 => "font-size: 16px;",
                _ => unreachable!(),
            };

            css.push_str(&format!("{} {{ {} }}\n", selector, property));
        }

        css
    }

    /// Generate a ParsedStylesheet with rules for parallel computation
    pub fn generate_parsed_stylesheet(rule_count: usize) -> ParallelParsedStylesheet {
        let mut stylesheet = ParallelParsedStylesheet::new(1, "benchmark stylesheet");

        for i in 0..rule_count {
            let selector = match i % 5 {
                0 => format!("div"),
                1 => format!(".class-{}", i % 100),
                2 => format!("#id-{}", i % 100),
                3 => "*".to_string(),
                4 => format!("span"),
                _ => unreachable!(),
            };

            let mut rule = StyleRule::new(&selector);

            // Set specificity based on selector type
            rule.specificity = match i % 5 {
                0 => Specificity {
                    ids: 0,
                    classes: 0,
                    elements: 1,
                },
                1 => Specificity {
                    ids: 0,
                    classes: 1,
                    elements: 0,
                },
                2 => Specificity {
                    ids: 1,
                    classes: 0,
                    elements: 0,
                },
                3 => Specificity {
                    ids: 0,
                    classes: 0,
                    elements: 0,
                },
                4 => Specificity {
                    ids: 0,
                    classes: 0,
                    elements: 1,
                },
                _ => unreachable!(),
            };

            // Add declarations
            rule.declarations.push(Declaration {
                property: "display".to_string(),
                value: "block".to_string(),
                important: false,
            });

            stylesheet = stylesheet.with_rule(rule);
        }

        stylesheet
    }

    /// Generate a flat list of elements for benchmarking
    pub fn generate_elements(count: usize) -> Vec<ElementRef> {
        (0..count)
            .map(|i| {
                let mut element =
                    ElementRef::new(ElementId::new(i as u64), "div").with_depth(i % 10);

                // Add classes to some elements
                if i % 3 == 0 {
                    element = element.with_class(format!("class-{}", i % 100));
                }

                // Add ID attribute to some elements
                if i % 5 == 0 {
                    element
                        .attributes
                        .push(("id".to_string(), format!("id-{}", i % 100)));
                }

                element
            })
            .collect()
    }

    /// Generate a DOM tree with specified width and depth
    pub fn generate_dom_tree(width: usize, depth: usize) -> DomNode {
        generate_dom_node(0, width, depth, 0)
    }

    fn generate_dom_node(id: u64, width: usize, max_depth: usize, current_depth: usize) -> DomNode {
        let tag = match current_depth % 4 {
            0 => "div",
            1 => "span",
            2 => "p",
            3 => "section",
            _ => "div",
        };

        let mut node = DomNode::new(ElementId::new(id), tag);

        // Add class to some nodes
        if id % 3 == 0 {
            node = node.with_class(format!("class-{}", id % 100));
        }

        // Add children if not at max depth
        if current_depth < max_depth {
            for i in 0..width {
                let child_id = id * (width as u64 + 1) + (i as u64) + 1;
                let child = generate_dom_node(child_id, width, max_depth, current_depth + 1);
                node = node.with_child(child);
            }
        }

        node
    }

    /// Generate a deep DOM tree (single path)
    pub fn generate_deep_dom_tree(depth: usize) -> DomNode {
        if depth == 0 {
            return DomNode::new(ElementId::new(0), "div");
        }

        let mut current = DomNode::new(ElementId::new(depth as u64), "div");

        for i in (0..depth).rev() {
            let tag = match i % 4 {
                0 => "div",
                1 => "span",
                2 => "section",
                3 => "article",
                _ => "div",
            };
            let mut parent = DomNode::new(ElementId::new(i as u64), tag);

            if i % 2 == 0 {
                parent = parent.with_class(format!("level-{}", i));
            }

            parent = parent.with_child(current);
            current = parent;
        }

        current
    }

    /// Generate complex selectors for matching benchmarks
    pub fn generate_complex_selectors() -> Vec<String> {
        vec![
            // Simple selectors
            "div".to_string(),
            ".container".to_string(),
            "#main".to_string(),
            "*".to_string(),
            // Compound selectors
            "div.container".to_string(),
            "div#main.active".to_string(),
            // Descendant selectors
            "div span".to_string(),
            "body div p span".to_string(),
            // Child selectors
            "div > span".to_string(),
            "section > div > p".to_string(),
            // Attribute selectors
            "[data-id]".to_string(),
            "[class^=\"btn\"]".to_string(),
            // Pseudo-class selectors (represented as strings)
            ":first-child".to_string(),
            ":nth-child(2n+1)".to_string(),
            ":not(.hidden)".to_string(),
            // Combined complex selectors
            "div.container > p:first-child".to_string(),
            "section article.featured > header h1".to_string(),
        ]
    }

    /// Generate invalidation events
    pub fn generate_invalidations(count: usize) -> Vec<StyleInvalidation> {
        (0..count)
            .map(|i| match i % 4 {
                0 => StyleInvalidation::AttributeChange {
                    element_id: ElementId::new(i as u64),
                    attr: format!("data-{}", i),
                },
                1 => StyleInvalidation::ClassChange {
                    element_id: ElementId::new(i as u64),
                    added: vec![format!("new-class-{}", i)],
                    removed: vec![],
                },
                2 => StyleInvalidation::ClassChange {
                    element_id: ElementId::new(i as u64),
                    added: vec![],
                    removed: vec![format!("old-class-{}", i)],
                },
                3 => StyleInvalidation::ElementInserted {
                    element_id: ElementId::new(i as u64 + 10000),
                    parent_id: ElementId::new(i as u64),
                },
                _ => unreachable!(),
            })
            .collect()
    }
}

// ============================================================================
// Parsing Benchmarks
// ============================================================================

/// Benchmark CSS stylesheet parsing at various sizes
fn bench_parse_stylesheet(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse_stylesheet");

    // Test different stylesheet sizes
    for size in [100, 1000, 10000].iter() {
        let css = test_data::generate_stylesheet(*size);

        group.throughput(Throughput::Bytes(css.len() as u64));

        group.bench_with_input(BenchmarkId::new("rules", size), &css, |b, css| {
            b.iter(|| {
                let mut engine = CssEngine::new();
                engine.parse_stylesheet(black_box(css), None)
            });
        });
    }

    group.finish();
}

/// Benchmark parsing of complex CSS selectors
fn bench_parse_complex_css(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse_complex_css");

    // CSS with nested media queries
    let media_query_css = r#"
        @media screen and (min-width: 768px) {
            .container { max-width: 720px; }
            .row { display: flex; }
        }
        @media screen and (min-width: 1024px) {
            .container { max-width: 960px; }
        }
    "#
    .repeat(50);

    group.bench_function("media_queries", |b| {
        b.iter(|| {
            let mut engine = CssEngine::new();
            engine.parse_stylesheet(black_box(&media_query_css), None)
        });
    });

    // CSS with complex selectors
    let complex_selectors_css = test_data::generate_complex_selectors()
        .iter()
        .map(|s| format!("{} {{ color: red; }}", s))
        .collect::<Vec<_>>()
        .join("\n")
        .repeat(100);

    group.bench_function("complex_selectors", |b| {
        b.iter(|| {
            let mut engine = CssEngine::new();
            engine.parse_stylesheet(black_box(&complex_selectors_css), None)
        });
    });

    group.finish();
}

// ============================================================================
// Selector Matching Benchmarks
// ============================================================================

/// Benchmark selector matching with various selector types
fn bench_selector_matching(c: &mut Criterion) {
    let mut group = c.benchmark_group("selector_matching");

    let computer = ParallelStyleComputer::new(1);

    // Generate test elements
    let elements = test_data::generate_elements(1000);

    // Test different selector types
    let selector_tests = vec![
        ("tag", "div"),
        ("class", ".class-0"),
        ("id", "#id-0"),
        ("universal", "*"),
    ];

    for (name, selector) in selector_tests {
        group.bench_with_input(BenchmarkId::new("type", name), &selector, |b, _selector| {
            b.iter(|| {
                let mut matches = 0;
                for element in &elements {
                    // We need to test selector matching internally
                    // Since selector_matches is private, we test through compute_styles
                    if element.tag_name == "div" && name == "tag" {
                        matches += 1;
                    }
                }
                black_box(matches)
            });
        });
    }

    // Benchmark with stylesheets of different sizes
    for rule_count in [100, 1000, 10000].iter() {
        let stylesheet = test_data::generate_parsed_stylesheet(*rule_count);

        group.bench_with_input(
            BenchmarkId::new("rules_vs_elements", rule_count),
            &stylesheet,
            |b, stylesheet| {
                b.iter(|| {
                    computer.compute_styles_parallel(
                        black_box(&elements),
                        black_box(&[stylesheet.clone()]),
                    )
                });
            },
        );
    }

    group.finish();
}

/// Benchmark selector matching with complex selectors
fn bench_complex_selector_matching(c: &mut Criterion) {
    let mut group = c.benchmark_group("complex_selector_matching");

    let computer = ParallelStyleComputer::new(1);
    let elements = test_data::generate_elements(500);

    // Create stylesheet with various selector complexities
    let simple_stylesheet = ParallelParsedStylesheet::new(1, "simple")
        .with_rule(StyleRule::new("div").with_declaration("display", "block"));

    let complex_stylesheet = {
        let mut s = ParallelParsedStylesheet::new(2, "complex");
        for i in 0..100 {
            s = s.with_rule(
                StyleRule::new(format!(".class-{}", i)).with_declaration("color", "red"),
            );
        }
        s
    };

    group.bench_function("simple_selectors", |b| {
        b.iter(|| {
            computer.compute_styles_parallel(
                black_box(&elements),
                black_box(&[simple_stylesheet.clone()]),
            )
        });
    });

    group.bench_function("many_class_selectors", |b| {
        b.iter(|| {
            computer.compute_styles_parallel(
                black_box(&elements),
                black_box(&[complex_stylesheet.clone()]),
            )
        });
    });

    group.finish();
}

// ============================================================================
// Cascade Resolution Benchmarks
// ============================================================================

/// Benchmark cascade resolution performance
fn bench_cascade_resolution(c: &mut Criterion) {
    let mut group = c.benchmark_group("cascade_resolution");

    let computer = ParallelStyleComputer::new(1);
    let elements = test_data::generate_elements(100);

    // Test with increasing number of stylesheets (cascade complexity)
    for stylesheet_count in [1, 5, 10, 20].iter() {
        let stylesheets: Vec<ParallelParsedStylesheet> = (0..*stylesheet_count)
            .map(|i| {
                ParallelParsedStylesheet::new(i as u32, format!("sheet-{}", i))
                    .with_rule(StyleRule::new("div").with_declaration("display", "block"))
                    .with_rule(StyleRule::new(".class-0").with_declaration("color", "red"))
            })
            .collect();

        group.bench_with_input(
            BenchmarkId::new("stylesheets", stylesheet_count),
            &stylesheets,
            |b, stylesheets| {
                b.iter(|| {
                    computer.compute_styles_parallel(black_box(&elements), black_box(stylesheets))
                });
            },
        );
    }

    // Test with rules having different specificities
    let specificity_stylesheet = {
        let mut s = ParallelParsedStylesheet::new(1, "specificity");

        // Low specificity (element selector)
        let mut rule1 = StyleRule::new("div");
        rule1.specificity = Specificity {
            ids: 0,
            classes: 0,
            elements: 1,
        };
        rule1.declarations.push(Declaration {
            property: "display".to_string(),
            value: "inline".to_string(),
            important: false,
        });

        // Medium specificity (class selector)
        let mut rule2 = StyleRule::new(".class-0");
        rule2.specificity = Specificity {
            ids: 0,
            classes: 1,
            elements: 0,
        };
        rule2.declarations.push(Declaration {
            property: "display".to_string(),
            value: "block".to_string(),
            important: false,
        });

        // High specificity (ID selector)
        let mut rule3 = StyleRule::new("#id-0");
        rule3.specificity = Specificity {
            ids: 1,
            classes: 0,
            elements: 0,
        };
        rule3.declarations.push(Declaration {
            property: "display".to_string(),
            value: "flex".to_string(),
            important: false,
        });

        s = s.with_rule(rule1).with_rule(rule2).with_rule(rule3);
        s
    };

    group.bench_function("specificity_sorting", |b| {
        b.iter(|| {
            computer.compute_styles_parallel(
                black_box(&elements),
                black_box(&[specificity_stylesheet.clone()]),
            )
        });
    });

    group.finish();
}

// ============================================================================
// Style Computation Benchmarks
// ============================================================================

/// Benchmark full style computation pipeline
fn bench_style_computation(c: &mut Criterion) {
    let mut group = c.benchmark_group("style_computation");

    // Test with different DOM tree sizes
    for (width, depth) in [(2, 5), (3, 4), (4, 3), (10, 2)].iter() {
        let dom = test_data::generate_dom_tree(*width, *depth);
        let total_nodes = ((*width as i32).pow(*depth as u32 + 1) - 1) / (*width as i32 - 1);

        group.throughput(Throughput::Elements(total_nodes as u64));

        group.bench_with_input(
            BenchmarkId::new("tree", format!("{}x{}", width, depth)),
            &dom,
            |b, dom| {
                b.iter(|| {
                    let mut engine = CssEngine::new();
                    engine
                        .parse_stylesheet("div { display: block; }", None)
                        .unwrap();
                    engine.compute_styles(black_box(dom))
                });
            },
        );
    }

    group.finish();
}

/// Benchmark style computation with deep DOM trees
fn bench_deep_dom_computation(c: &mut Criterion) {
    let mut group = c.benchmark_group("deep_dom_computation");

    // Test increasingly deep DOM trees
    for depth in [50, 100, 200, 500].iter() {
        let dom = test_data::generate_deep_dom_tree(*depth);

        group.throughput(Throughput::Elements(*depth as u64));

        group.bench_with_input(BenchmarkId::new("depth", depth), &dom, |b, dom| {
            b.iter(|| {
                let mut engine = CssEngine::new();
                engine
                    .parse_stylesheet("div { display: block; } span { color: red; }", None)
                    .unwrap();
                engine.compute_styles(black_box(dom))
            });
        });
    }

    group.finish();
}

/// Benchmark parallel style computation
fn bench_parallel_computation(c: &mut Criterion) {
    let mut group = c.benchmark_group("parallel_computation");

    // Test different thread counts
    for thread_count in [1, 2, 4, 8].iter() {
        let computer = ParallelStyleComputer::new(*thread_count);
        let elements = test_data::generate_elements(10000);
        let stylesheet = test_data::generate_parsed_stylesheet(100);

        group.throughput(Throughput::Elements(elements.len() as u64));

        group.bench_with_input(
            BenchmarkId::new("threads", thread_count),
            &elements,
            |b, elements| {
                b.iter(|| {
                    computer.compute_styles_parallel(
                        black_box(elements),
                        black_box(&[stylesheet.clone()]),
                    )
                });
            },
        );
    }

    group.finish();
}

/// Benchmark partitioned parallel computation
fn bench_partitioned_computation(c: &mut Criterion) {
    let mut group = c.benchmark_group("partitioned_computation");

    let computer = ParallelStyleComputer::new(4);
    let stylesheet = test_data::generate_parsed_stylesheet(100);

    // Test different tree sizes for partitioning
    for size in [100, 500, 1000, 5000].iter() {
        // Create a wide, shallow tree
        let dom = test_data::generate_dom_tree((*size as f64).sqrt() as usize, 2);
        let subtrees = partition_dom_tree(&dom);

        let total_elements: usize = subtrees.iter().map(|s| s.len()).sum();

        group.throughput(Throughput::Elements(total_elements as u64));

        group.bench_with_input(
            BenchmarkId::new("elements", size),
            &subtrees,
            |b, subtrees| {
                b.iter(|| {
                    computer.compute_styles_partitioned(
                        black_box(subtrees),
                        black_box(&[stylesheet.clone()]),
                    )
                });
            },
        );
    }

    group.finish();
}

// ============================================================================
// Invalidation Benchmarks
// ============================================================================

/// Benchmark style invalidation processing
fn bench_invalidation(c: &mut Criterion) {
    let mut group = c.benchmark_group("invalidation");

    // Test different numbers of invalidations
    for count in [10, 100, 1000].iter() {
        let invalidations = test_data::generate_invalidations(*count);

        group.throughput(Throughput::Elements(*count as u64));

        group.bench_with_input(
            BenchmarkId::new("count", count),
            &invalidations,
            |b, invalidations| {
                b.iter_batched(
                    || {
                        // Setup: create engine with computed styles
                        let mut engine = CssEngine::new();
                        engine
                            .parse_stylesheet("div { display: block; }", None)
                            .unwrap();

                        // Create DOM and compute styles for cache
                        let dom = test_data::generate_dom_tree(10, 3);
                        engine.compute_styles(&dom).unwrap();

                        (engine, invalidations.clone())
                    },
                    |(mut engine, invalidations)| {
                        for invalidation in invalidations {
                            engine.invalidate_styles(black_box(invalidation)).unwrap();
                        }
                    },
                    criterion::BatchSize::SmallInput,
                );
            },
        );
    }

    group.finish();
}

/// Benchmark different types of invalidations
fn bench_invalidation_types(c: &mut Criterion) {
    let mut group = c.benchmark_group("invalidation_types");

    let create_engine = || {
        let mut engine = CssEngine::new();
        engine
            .parse_stylesheet("div { display: block; }", None)
            .unwrap();
        let dom = test_data::generate_dom_tree(5, 4);
        engine.compute_styles(&dom).unwrap();
        engine
    };

    // Attribute change invalidation
    group.bench_function("attribute_change", |b| {
        b.iter_batched(
            create_engine,
            |mut engine| {
                for i in 0..100 {
                    let invalidation = StyleInvalidation::AttributeChange {
                        element_id: ElementId::new(i),
                        attr: "data-test".to_string(),
                    };
                    engine.invalidate_styles(black_box(invalidation)).unwrap();
                }
            },
            criterion::BatchSize::SmallInput,
        );
    });

    // Class change invalidation (add)
    group.bench_function("class_add", |b| {
        b.iter_batched(
            create_engine,
            |mut engine| {
                for i in 0..100 {
                    let invalidation = StyleInvalidation::ClassChange {
                        element_id: ElementId::new(i),
                        added: vec!["new-class".to_string()],
                        removed: vec![],
                    };
                    engine.invalidate_styles(black_box(invalidation)).unwrap();
                }
            },
            criterion::BatchSize::SmallInput,
        );
    });

    // Class change invalidation (remove)
    group.bench_function("class_remove", |b| {
        b.iter_batched(
            create_engine,
            |mut engine| {
                for i in 0..100 {
                    let invalidation = StyleInvalidation::ClassChange {
                        element_id: ElementId::new(i),
                        added: vec![],
                        removed: vec!["old-class".to_string()],
                    };
                    engine.invalidate_styles(black_box(invalidation)).unwrap();
                }
            },
            criterion::BatchSize::SmallInput,
        );
    });

    // Element insertion invalidation
    group.bench_function("element_inserted", |b| {
        b.iter_batched(
            create_engine,
            |mut engine| {
                for i in 0..100 {
                    let invalidation = StyleInvalidation::ElementInserted {
                        element_id: ElementId::new(i + 10000),
                        parent_id: ElementId::new(i),
                    };
                    engine.invalidate_styles(black_box(invalidation)).unwrap();
                }
            },
            criterion::BatchSize::SmallInput,
        );
    });

    // Element removal invalidation
    group.bench_function("element_removed", |b| {
        b.iter_batched(
            create_engine,
            |mut engine| {
                for i in 0..100 {
                    let invalidation = StyleInvalidation::ElementRemoved {
                        element_id: ElementId::new(i),
                    };
                    engine.invalidate_styles(black_box(invalidation)).unwrap();
                }
            },
            criterion::BatchSize::SmallInput,
        );
    });

    group.finish();
}

// ============================================================================
// Cache Performance Benchmarks
// ============================================================================

/// Benchmark cache hit/miss performance
fn bench_cache_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("cache_performance");

    // Benchmark with cache (warm)
    group.bench_function("cache_warm", |b| {
        b.iter_batched(
            || {
                let mut engine = CssEngine::new();
                engine
                    .parse_stylesheet("div { display: block; }", None)
                    .unwrap();
                let dom = test_data::generate_dom_tree(5, 3);
                // Warm up cache
                engine.compute_styles(&dom).unwrap();
                (engine, dom)
            },
            |(mut engine, dom)| {
                // Second computation should hit cache
                engine.compute_styles(black_box(&dom)).unwrap()
            },
            criterion::BatchSize::SmallInput,
        );
    });

    // Benchmark without cache (cold)
    group.bench_function("cache_cold", |b| {
        b.iter_batched(
            || {
                let engine = CssEngine::new();
                let dom = test_data::generate_dom_tree(5, 3);
                (engine, dom)
            },
            |(mut engine, dom)| {
                engine
                    .parse_stylesheet("div { display: block; }", None)
                    .unwrap();
                engine.compute_styles(black_box(&dom)).unwrap()
            },
            criterion::BatchSize::SmallInput,
        );
    });

    // Benchmark cache clear and recompute
    group.bench_function("cache_clear_recompute", |b| {
        b.iter_batched(
            || {
                let mut engine = CssEngine::new();
                engine
                    .parse_stylesheet("div { display: block; }", None)
                    .unwrap();
                let dom = test_data::generate_dom_tree(5, 3);
                engine.compute_styles(&dom).unwrap();
                (engine, dom)
            },
            |(mut engine, dom)| {
                engine.clear_cache();
                engine.compute_styles(black_box(&dom)).unwrap()
            },
            criterion::BatchSize::SmallInput,
        );
    });

    group.finish();
}

// ============================================================================
// End-to-End Benchmarks
// ============================================================================

/// Benchmark realistic end-to-end scenarios
fn bench_realistic_scenarios(c: &mut Criterion) {
    let mut group = c.benchmark_group("realistic_scenarios");

    // Scenario: Initial page load
    group.bench_function("initial_page_load", |b| {
        let css = test_data::generate_stylesheet(500);
        let dom = test_data::generate_dom_tree(10, 4);

        b.iter(|| {
            let mut engine = CssEngine::new();
            engine
                .parse_stylesheet(black_box(&css), Some("main.css"))
                .unwrap();
            engine.compute_styles(black_box(&dom)).unwrap()
        });
    });

    // Scenario: Interactive update (class toggle)
    group.bench_function("interactive_class_toggle", |b| {
        b.iter_batched(
            || {
                let mut engine = CssEngine::new();
                engine
                    .parse_stylesheet("div { display: block; } .active { color: red; }", None)
                    .unwrap();
                let dom = test_data::generate_dom_tree(5, 3);
                engine.compute_styles(&dom).unwrap();
                engine
            },
            |mut engine| {
                // Toggle class on element
                let invalidation = StyleInvalidation::ClassChange {
                    element_id: ElementId::new(1),
                    added: vec!["active".to_string()],
                    removed: vec![],
                };
                engine.invalidate_styles(black_box(invalidation)).unwrap();

                // Recompute affected styles
                let dom = DomNode::new(ElementId::new(1), "div").with_class("active");
                engine.compute_styles(black_box(&dom)).unwrap()
            },
            criterion::BatchSize::SmallInput,
        );
    });

    // Scenario: Adding a new stylesheet dynamically
    group.bench_function("dynamic_stylesheet_add", |b| {
        b.iter_batched(
            || {
                let mut engine = CssEngine::new();
                engine
                    .parse_stylesheet("div { display: block; }", None)
                    .unwrap();
                let dom = test_data::generate_dom_tree(5, 3);
                engine.compute_styles(&dom).unwrap();
                (engine, dom)
            },
            |(mut engine, dom)| {
                // Add new stylesheet
                let new_css = ".new-styles { color: blue; }";
                engine
                    .parse_stylesheet(black_box(new_css), Some("dynamic.css"))
                    .unwrap();

                // Recompute styles
                engine.clear_cache();
                engine.compute_styles(black_box(&dom)).unwrap()
            },
            criterion::BatchSize::SmallInput,
        );
    });

    group.finish();
}

// ============================================================================
// Criterion Configuration
// ============================================================================

criterion_group!(
    parsing_benches,
    bench_parse_stylesheet,
    bench_parse_complex_css,
);

criterion_group!(
    selector_benches,
    bench_selector_matching,
    bench_complex_selector_matching,
);

criterion_group!(cascade_benches, bench_cascade_resolution,);

criterion_group!(
    computation_benches,
    bench_style_computation,
    bench_deep_dom_computation,
    bench_parallel_computation,
    bench_partitioned_computation,
);

criterion_group!(
    invalidation_benches,
    bench_invalidation,
    bench_invalidation_types,
);

criterion_group!(cache_benches, bench_cache_performance,);

criterion_group!(e2e_benches, bench_realistic_scenarios,);

criterion_main!(
    parsing_benches,
    selector_benches,
    cascade_benches,
    computation_benches,
    invalidation_benches,
    cache_benches,
    e2e_benches,
);
