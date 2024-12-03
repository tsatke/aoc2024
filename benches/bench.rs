use aoc2024::{day01, day02};
use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark(c: &mut Criterion) {
    let mut g = c.benchmark_group("AoC 2024");
    g.bench_function("Day 1 Part 1", |b| b.iter(day01::part1));
    g.bench_function("Day 1 Part 2", |b| b.iter(day01::part2));
    g.bench_function("Day 2 Part 1", |b| b.iter(day02::part1));
    g.bench_function("Day 2 Part 2", |b| b.iter(day02::part2));
    g.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
