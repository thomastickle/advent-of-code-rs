use adventofcode_rs::aoc2025_day01::AdventOfCode2025Day01;
use adventofcode_rs::aoc2025_day02::AdventOfCode2025Day02;
use adventofcode_rs::aoc2025_day03::AdventOfCode2025Day03;
use adventofcode_rs::Runner;

fn main() {
    println!("Advent of Code 2025");

    let mut aoc_day01 = AdventOfCode2025Day01::default();
    if let Err(e) = aoc_day01.parse() {
        eprintln!("    Error parsing Day 01 input: {}", e);
    } else {
        aoc_day01.run();
    }

    let mut aoc_day02 = AdventOfCode2025Day02::new();
    if let Err(e) = aoc_day02.parse() {
        eprintln!("    Error parsing Day 02 input: {}", e);
    } else {
        aoc_day02.run();
    }

    let mut aoc_day03 = AdventOfCode2025Day03::default();
    if let Err(e) = aoc_day03.parse() {
        eprintln!("    Error parsing Day 03 input: {}", e);
    } else {
        aoc_day03.run();
    }
}
