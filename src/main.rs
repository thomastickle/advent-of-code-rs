use crate::aoclib::runner::AocDay;

mod aoclib;
mod aoc2025;

fn main() {
    let days: Vec<Box<dyn AocDay>> = vec![
        Box::new(aoc2025::day01::AdventOfCode2025Day01::default()),
        Box::new(aoc2025::day02::AdventOfCode2025Day02::default()),
        Box::new(aoc2025::day03::AdventOfCode2025Day03::default()),
        Box::new(aoc2025::day04::AdventOfCode2025Day04::default()),
        Box::new(aoc2025::day05::AdventOfCode2025Day05::default()),
    ];

    for day in days {
        println!("{}", day.run_day());
    }
}
