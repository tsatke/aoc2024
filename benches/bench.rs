use aoc2024::{day01, day02};
use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark(c: &mut Criterion) {
    c.benchmark_group("Day 01")
        .bench_function("Part 1", |b| b.iter(day01::part1))
        .bench_function("Part 2", |b| b.iter(day01::part2));
    c.benchmark_group("Day 02")
        .bench_function("Part 1", |b| b.iter(day02::part1))
        .bench_function("Part 2", |b| b.iter(day02::part2));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
