use aoc2024::{day01, day02, day03, day04, day05, day06, day07, day08, day09};
use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark(c: &mut Criterion) {
    let mut g = c.benchmark_group("AoC 2024");
    g.bench_function("Day 1 Part 1", |b| b.iter(day01::part1));
    g.bench_function("Day 1 Part 2", |b| b.iter(day01::part2));
    g.bench_function("Day 2 Part 1", |b| b.iter(day02::part1));
    g.bench_function("Day 2 Part 2", |b| b.iter(day02::part2));
    g.bench_function("Day 3 Part 1", |b| b.iter(day03::part1));
    g.bench_function("Day 3 Part 2", |b| b.iter(day03::part2));
    g.bench_function("Day 4 Part 1", |b| b.iter(day04::part1));
    g.bench_function("Day 4 Part 2", |b| b.iter(day04::part2));
    g.bench_function("Day 5 Part 1", |b| b.iter(day05::part1));
    g.bench_function("Day 5 Part 2", |b| b.iter(day05::part2));
    g.bench_function("Day 6 Part 1", |b| b.iter(day06::part1));
    g.bench_function("Day 6 Part 2", |b| b.iter(day06::part2));
    g.bench_function("Day 7 Part 1", |b| b.iter(day07::part1));
    g.bench_function("Day 7 Part 2", |b| b.iter(day07::part2));
    g.bench_function("Day 8 Part 1", |b| b.iter(day08::part1));
    g.bench_function("Day 8 Part 2", |b| b.iter(day08::part2));
    g.bench_function("Day 9 Part 1", |b| b.iter(day09::part1));
    g.bench_function("Day 9 Part 2", |b| b.iter(day09::part2));
    g.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
