use crate::aoclib::runner::{AocDay, Runner};

mod aoclib;
mod aoc2025;

fn main() {
    let days: Vec<Box<dyn AocDay>> = vec![
        Box::new(aoc2025::day01::AdventOfCode2025Day01::new()),
        Box::new(aoc2025::day02::AdventOfCode2025Day02::new()),
    ];

    for day in days {
        println!("{}", day.run_day());
    }
}
