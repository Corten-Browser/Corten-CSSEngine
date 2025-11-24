//! Benchmarks for CSS Stylist Core optimizations
//!
//! This benchmark suite tests the performance of:
//! - Rule Tree operations (insertion, path traversal, style computation)
//! - Bloom Filter operations (add, check, merge)
//! - String Interning operations (intern, lookup)

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use css_cascade::{PropertyId, PropertyValue};
use css_matcher_core::{Component, Selector};
use css_stylist_core::bloom_filter::{BloomFilterBuilder, SelectorBloomFilter};
use css_stylist_core::interning::{ClassNameInterner, StringInterner};
use css_stylist_core::rule_tree::{PropertyDeclaration, RuleSource, RuleTree};
use css_types::Specificity;

// ============================================================================
// Rule Tree Benchmarks
// ============================================================================

fn bench_rule_tree_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("rule_tree_insert");

    for size in [10, 100, 1000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let mut tree = RuleTree::new();
                for i in 0..size {
                    let decls = vec![PropertyDeclaration::normal(
                        PropertyId::Color,
                        PropertyValue::Keyword(format!("color{}", i)),
                    )];
                    tree.insert_rule(decls, Specificity::new(0, 1, 0));
                }
                black_box(tree)
            });
        });
    }

    group.finish();
}

fn bench_rule_tree_chain_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("rule_tree_chain_insert");

    for depth in [5, 10, 20].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(depth), depth, |b, &depth| {
            b.iter(|| {
                let mut tree = RuleTree::new();
                let rules: Vec<_> = (0..depth)
                    .map(|i| {
                        (
                            vec![PropertyDeclaration::normal(
                                PropertyId::Color,
                                PropertyValue::Keyword(format!("color{}", i)),
                            )],
                            Specificity::new(0, i as u32, 0),
                            RuleSource::default(),
                        )
                    })
                    .collect();
                tree.insert_rule_chain(rules)
            });
        });
    }

    group.finish();
}

fn bench_rule_tree_get_path(c: &mut Criterion) {
    let mut group = c.benchmark_group("rule_tree_get_path");

    for depth in [5, 10, 20].iter() {
        // Setup: create a tree with a chain of rules
        let mut tree = RuleTree::new();
        let rules: Vec<_> = (0..*depth)
            .map(|i| {
                (
                    vec![PropertyDeclaration::normal(
                        PropertyId::Color,
                        PropertyValue::Keyword(format!("color{}", i)),
                    )],
                    Specificity::new(0, i as u32, 0),
                    RuleSource::default(),
                )
            })
            .collect();
        let leaf_id = tree.insert_rule_chain(rules);

        group.bench_with_input(BenchmarkId::from_parameter(depth), depth, |b, _| {
            b.iter(|| black_box(tree.get_path(leaf_id)));
        });
    }

    group.finish();
}

fn bench_rule_tree_compute_style(c: &mut Criterion) {
    let mut group = c.benchmark_group("rule_tree_compute_style");

    for depth in [5, 10, 20].iter() {
        // Setup
        let mut tree = RuleTree::new();
        let rules: Vec<_> = (0..*depth)
            .map(|i| {
                (
                    vec![PropertyDeclaration::normal(
                        PropertyId::Color,
                        PropertyValue::Keyword(format!("color{}", i)),
                    )],
                    Specificity::new(0, i as u32, 0),
                    RuleSource::default(),
                )
            })
            .collect();
        let leaf_id = tree.insert_rule_chain(rules);

        group.bench_with_input(BenchmarkId::from_parameter(depth), depth, |b, _| {
            b.iter(|| black_box(tree.compute_style(leaf_id)));
        });
    }

    group.finish();
}

fn bench_rule_tree_sharing(c: &mut Criterion) {
    // Benchmark the efficiency of sharing common prefixes
    c.bench_function("rule_tree_sharing_1000_elements", |b| {
        b.iter(|| {
            let mut tree = RuleTree::new();

            // Simulate 1000 elements that share common ancestor rules
            // Each element has: reset -> body -> container -> specific
            let reset = tree.insert_rule(
                vec![PropertyDeclaration::normal(
                    PropertyId::Margin,
                    PropertyValue::Length(0.0, "px".to_string()),
                )],
                Specificity::new(0, 0, 1),
            );

            let body = tree.insert_rule_at(
                reset,
                vec![PropertyDeclaration::normal(
                    PropertyId::FontSize,
                    PropertyValue::Length(16.0, "px".to_string()),
                )],
                Specificity::new(0, 0, 1),
                RuleSource::default(),
            );

            let container = tree.insert_rule_at(
                body,
                vec![PropertyDeclaration::normal(
                    PropertyId::Width,
                    PropertyValue::Length(1200.0, "px".to_string()),
                )],
                Specificity::new(0, 1, 0),
                RuleSource::default(),
            );

            // Add 1000 different specific rules
            for i in 0..1000 {
                tree.insert_rule_at(
                    container,
                    vec![PropertyDeclaration::normal(
                        PropertyId::Color,
                        PropertyValue::Keyword(format!("color{}", i)),
                    )],
                    Specificity::new(0, 1, 0),
                    RuleSource::default(),
                );
            }

            black_box(tree.node_count())
        });
    });
}

// ============================================================================
// Bloom Filter Benchmarks
// ============================================================================

fn bench_bloom_filter_add(c: &mut Criterion) {
    let mut group = c.benchmark_group("bloom_filter_add");

    for count in [10, 100, 1000].iter() {
        group.throughput(Throughput::Elements(*count as u64));
        group.bench_with_input(BenchmarkId::from_parameter(count), count, |b, &count| {
            b.iter(|| {
                let mut filter = SelectorBloomFilter::new();
                for i in 0..count {
                    filter.add_class(&format!("class{}", i));
                }
                black_box(filter)
            });
        });
    }

    group.finish();
}

fn bench_bloom_filter_check(c: &mut Criterion) {
    // Setup: create a filter with 100 items
    let mut filter = SelectorBloomFilter::new();
    for i in 0..100 {
        filter.add_class(&format!("class{}", i));
    }

    c.bench_function("bloom_filter_check_present", |b| {
        b.iter(|| {
            // Check items that ARE in the filter
            for i in 0..100 {
                black_box(filter.might_contain_class(&format!("class{}", i)));
            }
        });
    });

    c.bench_function("bloom_filter_check_absent", |b| {
        b.iter(|| {
            // Check items that are NOT in the filter
            for i in 100..200 {
                black_box(filter.might_contain_class(&format!("class{}", i)));
            }
        });
    });
}

fn bench_bloom_filter_selector_match(c: &mut Criterion) {
    // Setup
    let mut filter = SelectorBloomFilter::new();
    for i in 0..50 {
        filter.add_class(&format!("class{}", i));
        filter.add_tag(&format!("tag{}", i));
    }
    filter.add_id("main");

    let matching_selector = Selector::with_components(vec![
        Component::Tag("tag10".to_string()),
        Component::Class("class20".to_string()),
    ]);

    let non_matching_selector = Selector::with_components(vec![
        Component::Tag("nonexistent".to_string()),
        Component::Class("class20".to_string()),
    ]);

    c.bench_function("bloom_filter_might_match_true", |b| {
        b.iter(|| black_box(filter.might_match(&matching_selector)));
    });

    c.bench_function("bloom_filter_might_match_false", |b| {
        b.iter(|| black_box(filter.might_match(&non_matching_selector)));
    });
}

fn bench_bloom_filter_merge(c: &mut Criterion) {
    let mut group = c.benchmark_group("bloom_filter_merge");

    for count in [10, 100].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(count), count, |b, &count| {
            // Setup: create multiple filters
            let filters: Vec<_> = (0..count)
                .map(|i| {
                    let mut f = SelectorBloomFilter::new();
                    f.add_class(&format!("class{}", i));
                    f
                })
                .collect();

            b.iter(|| {
                let mut result = SelectorBloomFilter::new();
                for f in &filters {
                    result.merge(f);
                }
                black_box(result)
            });
        });
    }

    group.finish();
}

fn bench_bloom_filter_builder(c: &mut Criterion) {
    c.bench_function("bloom_filter_builder_100_elements", |b| {
        b.iter(|| {
            let mut builder = BloomFilterBuilder::new();
            for i in 0..100 {
                builder.add_element(
                    Some(&format!("id{}", i)),
                    &[format!("class{}", i), format!("common{}", i % 10)],
                    &format!("tag{}", i % 20),
                );
            }
            black_box(builder.build())
        });
    });
}

// ============================================================================
// String Interning Benchmarks
// ============================================================================

fn bench_string_interner_intern(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_interner_intern");

    for count in [100, 1000, 10000].iter() {
        group.throughput(Throughput::Elements(*count as u64));
        group.bench_with_input(BenchmarkId::from_parameter(count), count, |b, &count| {
            b.iter(|| {
                let mut interner = StringInterner::new();
                for i in 0..count {
                    interner.intern(&format!("string{}", i));
                }
                black_box(interner.len())
            });
        });
    }

    group.finish();
}

fn bench_string_interner_lookup_existing(c: &mut Criterion) {
    // Setup: pre-intern 1000 strings
    let mut interner = StringInterner::new();
    let strings: Vec<_> = (0..1000).map(|i| format!("string{}", i)).collect();
    for s in &strings {
        interner.intern(s);
    }

    c.bench_function("string_interner_lookup_1000", |b| {
        b.iter(|| {
            for s in &strings {
                black_box(interner.get_id(s));
            }
        });
    });
}

fn bench_string_interner_intern_existing(c: &mut Criterion) {
    // Setup: pre-intern 1000 strings
    let mut interner = StringInterner::new();
    let strings: Vec<_> = (0..1000).map(|i| format!("string{}", i)).collect();
    for s in &strings {
        interner.intern(s);
    }

    c.bench_function("string_interner_reuse_1000", |b| {
        b.iter(|| {
            // Re-interning should be fast (just lookup)
            for s in &strings {
                black_box(interner.intern(s));
            }
        });
    });
}

fn bench_string_interner_get(c: &mut Criterion) {
    // Setup
    let mut interner = StringInterner::new();
    let ids: Vec<_> = (0..1000)
        .map(|i| interner.intern(&format!("string{}", i)))
        .collect();

    c.bench_function("string_interner_get_1000", |b| {
        b.iter(|| {
            for &id in &ids {
                black_box(interner.get(id));
            }
        });
    });
}

fn bench_class_name_interner(c: &mut Criterion) {
    c.bench_function("class_name_interner_1000_classes", |b| {
        b.iter(|| {
            let mut interner = ClassNameInterner::new();
            for i in 0..1000 {
                let id = interner.intern(&format!("class{}", i));
                black_box(id);
            }
            black_box(interner.len())
        });
    });
}

fn bench_string_comparison_vs_id(c: &mut Criterion) {
    // Compare string comparison vs interned ID comparison
    let strings: Vec<_> = (0..1000).map(|i| format!("string{}", i)).collect();

    let mut interner = StringInterner::new();
    let ids: Vec<_> = strings.iter().map(|s| interner.intern(s)).collect();

    c.bench_function("string_comparison_1000", |b| {
        b.iter(|| {
            let mut count = 0;
            for i in 0..strings.len() {
                for j in 0..strings.len() {
                    if strings[i] == strings[j] {
                        count += 1;
                    }
                }
            }
            black_box(count)
        });
    });

    c.bench_function("id_comparison_1000", |b| {
        b.iter(|| {
            let mut count = 0;
            for i in 0..ids.len() {
                for j in 0..ids.len() {
                    if ids[i] == ids[j] {
                        count += 1;
                    }
                }
            }
            black_box(count)
        });
    });
}

fn bench_with_css_keywords(c: &mut Criterion) {
    c.bench_function("string_interner_with_css_keywords", |b| {
        b.iter(|| {
            let interner = StringInterner::with_css_keywords();
            black_box(interner.len())
        });
    });
}

// ============================================================================
// Criterion Groups
// ============================================================================

criterion_group!(
    rule_tree_benches,
    bench_rule_tree_insert,
    bench_rule_tree_chain_insert,
    bench_rule_tree_get_path,
    bench_rule_tree_compute_style,
    bench_rule_tree_sharing,
);

criterion_group!(
    bloom_filter_benches,
    bench_bloom_filter_add,
    bench_bloom_filter_check,
    bench_bloom_filter_selector_match,
    bench_bloom_filter_merge,
    bench_bloom_filter_builder,
);

criterion_group!(
    string_interner_benches,
    bench_string_interner_intern,
    bench_string_interner_lookup_existing,
    bench_string_interner_intern_existing,
    bench_string_interner_get,
    bench_class_name_interner,
    bench_string_comparison_vs_id,
    bench_with_css_keywords,
);

criterion_main!(
    rule_tree_benches,
    bloom_filter_benches,
    string_interner_benches
);
