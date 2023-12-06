use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day6::*;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day 6 star 1", |b| b.iter(|| day6::star2(black_box())));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
