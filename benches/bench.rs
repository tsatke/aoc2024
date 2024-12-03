use aoc2024::{solution_01_1, solution_01_2};
use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark(c: &mut Criterion) {
    c.benchmark_group("Day 01")
        .bench_function("Solution 1.1", |b| b.iter(solution_01_1))
        .bench_function("Solution 1.2", |b| b.iter(solution_01_2));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
