use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use advent_of_code_23::day6;

fn criterion_benchmark_star1(c: &mut Criterion) {
    let input = day6::read_lines();

    let mut group = c.benchmark_group("day_06::star1");
    group.bench_with_input("day 6 star 1", &input, |b, (times, distances)| {
        b.iter(|| day6::star1(times.clone(), distances.clone()))
    });
}

fn criterion_benchmark_star2(c: &mut Criterion) {
    let mut group = c.benchmark_group("day_06::star2");
    let (time, distance) = day6::read_lines2();
    group.bench_function("day 6 star 2", |b| b.iter(|| day6::star2(time, distance)));
}

criterion_group!(
    benches,
    criterion_benchmark_star1,
    criterion_benchmark_star2
);
criterion_main!(benches);
