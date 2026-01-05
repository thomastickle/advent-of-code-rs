use adventofcode_rs::aoc2025::day01::AdventOfCode2025Day01;
use adventofcode_rs::aoc2025::day02::AdventOfCode2025Day02;
use adventofcode_rs::aoc2025::day03::AdventOfCode2025Day03;
use adventofcode_rs::aoc2025::day04::AdventOfCode2025Day04;
use adventofcode_rs::aoc2025::day05::AdventOfCode2025Day05;
pub(crate) use adventofcode_rs::aoc2025::day06::AdventOfCode2025Day06;
pub(crate) use adventofcode_rs::aoclib::runner::Runner;
use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;
use std::hint::black_box;
use std::str::FromStr;

fn benchmark_day<T: Runner>(c: &mut Criterion, day_num: u32)
where
    <T as FromStr>::Err: std::fmt::Debug
{
    let mut group = c.benchmark_group(format!("Day {:02}", day_num));
    let input_path = format!("input/day{:02}.input", day_num);
    let input = fs::read_to_string(&input_path).expect("Could not read input file");

    // Benchmark Parsing
    group.bench_function("parse", |b| {
        b.iter(|| T::from_str(black_box(&input)).unwrap())
    });

    let parsed = T::from_str(&input).unwrap();

    // Benchmark Part 1
    group.bench_function("part01", |b| {
        b.iter(|| black_box(parsed.part01()))
    });

    // Benchmark Part 2
    group.bench_function("part02", |b| {
        b.iter(|| black_box(parsed.part02()))
    });

    group.finish();
}

fn bench_all_days(c: &mut Criterion) {
    // Add each day here
    benchmark_day::<AdventOfCode2025Day01>(c, 1);
    benchmark_day::<AdventOfCode2025Day02>(c, 2);
    benchmark_day::<AdventOfCode2025Day03>(c, 3);
    benchmark_day::<AdventOfCode2025Day04>(c, 4);
    benchmark_day::<AdventOfCode2025Day05>(c, 5);
    benchmark_day::<AdventOfCode2025Day06>(c, 6);
}

criterion_group!(benches, bench_all_days);
criterion_main!(benches);
