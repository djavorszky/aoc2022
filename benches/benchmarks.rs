use aoc::day6;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn day6_benchmark(c: &mut Criterion) {
    let input = include_str!("../src/input/day6.txt");

    c.bench_function("day 6 task 1", |b| b.iter(|| day6::task1(black_box(input))));
    c.bench_function("day 6 task 2", |b| b.iter(|| day6::task1(black_box(input))));
}

criterion_group!(benches, day6_benchmark);
criterion_main!(benches);
