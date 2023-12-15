use advent_of_code_23::day15;
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark_star1(c: &mut Criterion) {
    let input = day15::read_input("input/15.txt");
    c.bench_function("day15::star1", |b| b.iter(|| day15::star1(input.clone())));
}

fn criterion_benchmark_star2(c: &mut Criterion) {
    let input = day15::read_input("input/15.txt");
    c.bench_function("day15::star2", |b| b.iter(|| day15::star2(input.clone())));
}

criterion_group!(
    benches,
    criterion_benchmark_star1,
    criterion_benchmark_star2
);
criterion_main!(benches);
