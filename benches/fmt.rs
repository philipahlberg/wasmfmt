use criterion::{BenchmarkId, criterion_group, criterion_main, Criterion};
use wasmfmt::fmt;

fn fmt_add_desugar(c: &mut Criterion) {
    let id = BenchmarkId::new("fmt", "add_desugar");
    let input = include_str!("../tests/data/input/add_desugar.wat");
    c.bench_with_input(id, &input, |b, i| {
        b.iter(|| fmt(i))
    });
}

fn fmt_add_sugar(c: &mut Criterion) {
    let id = BenchmarkId::new("fmt", "add_sugar");
    let input = include_str!("../tests/data/input/add_sugar.wat");
    c.bench_with_input(id, &input, |b, i| {
        b.iter(|| fmt(i))
    });
}

fn fmt_fac_desugar(c: &mut Criterion) {
    let id = BenchmarkId::new("fmt", "fac_desugar");
    let input = include_str!("../tests/data/input/fac_desugar.wat");
    c.bench_with_input(id, &input, |b, i| {
        b.iter(|| fmt(i))
    });
}

fn fmt_fac_sugar(c: &mut Criterion) {
    let id = BenchmarkId::new("fmt", "fac_sugar");
    let input = include_str!("../tests/data/input/fac_sugar.wat");
    c.bench_with_input(id, &input, |b, i| {
        b.iter(|| fmt(i))
    });
}

criterion_group!{
    name = benches;
    config = Criterion::default();
    targets = fmt_add_desugar, fmt_add_sugar, fmt_fac_desugar, fmt_fac_sugar
}
criterion_main!(benches);
