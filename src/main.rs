mod aoc2025_day02;

use aoc2025_day02::{AdventOfCode2025Day02, Runner};

fn main() {
    let mut aoc_day02 = AdventOfCode2025Day02::new();
    if let Err(e) = aoc_day02.parse() {
        eprintln!("Error parsing input: {}", e);
        return;
    }
    aoc_day02.run();
}
