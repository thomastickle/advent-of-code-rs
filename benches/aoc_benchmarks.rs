use adventofcode_rs::aoc2025_day01::AdventOfCode2025Day01;
use adventofcode_rs::aoc2025_day02::AdventOfCode2025Day02;
use adventofcode_rs::aoc2025_day03::AdventOfCode2025Day03;
use adventofcode_rs::Runner;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    // Day 01
    let mut aoc_day01 = AdventOfCode2025Day01::default();
    aoc_day01.parse().expect("Failed to parse input for Day 01 benchmarking");

    let mut group01 = c.benchmark_group("Day 01");
    group01.sample_size(100);
    group01.bench_function("Part 1", |b| b.iter(|| black_box(aoc_day01.part01())));
    group01.bench_function("Part 2", |b| b.iter(|| black_box(aoc_day01.part02())));
    group01.finish();

    // Day 02
    let mut aoc_day02 = AdventOfCode2025Day02::new();
    aoc_day02.parse().expect("Failed to parse input for Day 02 benchmarking");

    let mut group02 = c.benchmark_group("Day 02");
    group02.sample_size(100);
    group02.measurement_time(std::time::Duration::from_secs(40));
    group02.bench_function("Part 1", |b| b.iter(|| black_box(aoc_day02.part01())));
    group02.bench_function("Part 2", |b| b.iter(|| black_box(aoc_day02.part02())));
    group02.finish();

    // Day 03
    let mut aoc_day03 = AdventOfCode2025Day03::default();
    aoc_day03.parse().expect("Failed to parse input for Day 03 benchmarking");

    let mut group03 = c.benchmark_group("Day 03");
    group03.sample_size(100);
    group03.bench_function("Part 1", |b| b.iter(|| black_box(aoc_day03.part01())));
    group03.bench_function("Part 2", |b| b.iter(|| black_box(aoc_day03.part02())));
    group03.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
