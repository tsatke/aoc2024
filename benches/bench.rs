use aoc2024::day01::{part1, part2};
use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark(c: &mut Criterion) {
    c.benchmark_group("Day 01")
        .bench_function("Part 1", |b| b.iter(part1))
        .bench_function("Part 2", |b| b.iter(part2));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
