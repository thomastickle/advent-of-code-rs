use criterion::{black_box, criterion_group, criterion_main, Criterion};
use adventofcode_rs::aoc2025_day02::{AdventOfCode2025Day02, Runner};

fn criterion_benchmark(c: &mut Criterion) {
    let mut aoc_day02 = AdventOfCode2025Day02::new();
    aoc_day02.parse().expect("Failed to parse input for benchmarking");

    let mut group = c.benchmark_group("Day 02");
    group.sample_size(100); // Back to default 100
    group.measurement_time(std::time::Duration::from_secs(40)); // Increased to allow 100 samples (each takes ~250ms)
    group.bench_function("Part 1", |b| b.iter(|| black_box(aoc_day02.part01())));
    group.bench_function("Part 2", |b| b.iter(|| black_box(aoc_day02.part02())));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
