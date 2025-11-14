// Benchmarks for CSS parser values

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use css_parser_values::{parse_attribute_selector, parse_color_value, parse_function_value, parse_value};

fn benchmark_attribute_selector(c: &mut Criterion) {
    c.bench_function("parse attribute exists", |b| {
        b.iter(|| parse_attribute_selector(black_box("[href]")))
    });

    c.bench_function("parse attribute equals", |b| {
        b.iter(|| parse_attribute_selector(black_box("[href=\"https://example.com\"]")))
    });

    c.bench_function("parse attribute prefix", |b| {
        b.iter(|| parse_attribute_selector(black_box("[href^=\"https\"]")))
    });
}

fn benchmark_color_parsing(c: &mut Criterion) {
    c.bench_function("parse hex color 6 digits", |b| {
        b.iter(|| parse_color_value(black_box("#FF5733")))
    });

    c.bench_function("parse hex color 3 digits", |b| {
        b.iter(|| parse_color_value(black_box("#F00")))
    });

    c.bench_function("parse rgb color", |b| {
        b.iter(|| parse_color_value(black_box("rgb(255, 87, 51)")))
    });

    c.bench_function("parse rgba color", |b| {
        b.iter(|| parse_color_value(black_box("rgba(255, 87, 51, 0.5)")))
    });

    c.bench_function("parse hsl color", |b| {
        b.iter(|| parse_color_value(black_box("hsl(0, 100%, 50%)")))
    });

    c.bench_function("parse named color", |b| {
        b.iter(|| parse_color_value(black_box("red")))
    });
}

fn benchmark_function_parsing(c: &mut Criterion) {
    c.bench_function("parse url function", |b| {
        b.iter(|| parse_function_value(black_box("url(\"https://example.com/image.png\")")))
    });

    c.bench_function("parse calc function", |b| {
        b.iter(|| parse_function_value(black_box("calc(100% - 50px)")))
    });

    c.bench_function("parse var function", |b| {
        b.iter(|| parse_function_value(black_box("var(--main-color)")))
    });

    c.bench_function("parse linear-gradient", |b| {
        b.iter(|| parse_function_value(black_box("linear-gradient(to right, red, blue)")))
    });
}

fn benchmark_value_parsing(c: &mut Criterion) {
    c.bench_function("parse length value", |b| {
        b.iter(|| parse_value(black_box("10px"), black_box("margin")))
    });

    c.bench_function("parse percentage value", |b| {
        b.iter(|| parse_value(black_box("50%"), black_box("width")))
    });

    c.bench_function("parse number value", |b| {
        b.iter(|| parse_value(black_box("42"), black_box("line-height")))
    });

    c.bench_function("parse color value", |b| {
        b.iter(|| parse_value(black_box("#FF0000"), black_box("color")))
    });

    c.bench_function("parse keyword value", |b| {
        b.iter(|| parse_value(black_box("auto"), black_box("margin")))
    });
}

criterion_group!(
    benches,
    benchmark_attribute_selector,
    benchmark_color_parsing,
    benchmark_function_parsing,
    benchmark_value_parsing
);
criterion_main!(benches);
