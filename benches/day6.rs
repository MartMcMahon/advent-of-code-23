use criterion::{black_box, criterion_group, criterion_main, Criterion};

use advent_of_code_23::day8;

fn criterion_benchmark_star1(c: &mut Criterion) {
    let input = day8::read_lines();
    let mut group = c.benchmark_group("day_08::star1");
    group.bench_function("day 8 star 1", |b| b.iter(|| day8::star1(input)));
}

fn criterion_benchmark_star2(c: &mut Criterion) {
    let input = day8::read_lines();
    let mut group = c.benchmark_group("day_08::star2");
    group.bench_function("day 8 star 2", |b| b.iter(|| day8::star2(input)));
}

criterion_group!(
    benches,
    criterion_benchmark_star1,
    criterion_benchmark_star2
);
criterion_main!(benches);
