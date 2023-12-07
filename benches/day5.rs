use advent_of_code_23::day5;
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark_star1(c: &mut Criterion) {
    let input = day5::read_lines()
        .unwrap()
        .map(|s| s.unwrap())
        .collect::<Vec<String>>();

    let mut group = c.benchmark_group("day_05::star1");

    group.bench_function("day 5 star 1", |b| b.iter(|| day5::star1(input.clone())));
}

// fn criterion_benchmark_star2(c: &mut Criterion) {
//     let mut group = c.benchmark_group("day_06::star2");
//     let input = day5::read_lines();
//     group.bench_function("day 5 star 2", |b| b.iter(|| day5::star1(input)));
// }

criterion_group!(
    benches,
    criterion_benchmark_star1,
    // criterion_benchmark_star2
);
criterion_main!(benches);
