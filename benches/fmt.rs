use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use wasmfmt::{wat, Options};

fn fmt_i32(c: &mut Criterion) {
    let id = BenchmarkId::new("fmt", "i32");
    let input = include_str!("../tests/data/input/i32.wat");
    c.bench_with_input(id, &input, |b, i| b.iter(|| wat::fmt(i, Options::default())));
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = fmt_i32
}
criterion_main!(benches);
