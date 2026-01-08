use adventofcode_rs::aoc2025;
use adventofcode_rs::aoclib::runner::AocDay;

fn main() {
    let days: Vec<Box<dyn AocDay>> = vec![
        Box::new(aoc2025::day01::AdventOfCode2025Day01::default()),
        Box::new(aoc2025::day02::AdventOfCode2025Day02::default()),
        Box::new(aoc2025::day03::AdventOfCode2025Day03::default()),
        Box::new(aoc2025::day04::AdventOfCode2025Day04::default()),
        Box::new(aoc2025::day05::AdventOfCode2025Day05::default()),
        Box::new(aoc2025::day06::AdventOfCode2025Day06::default()),
        Box::new(aoc2025::day07::AdventOfCode2025Day07::default()),
        Box::new(aoc2025::day09::AdventOfCode2025Day09::default()),
    ];

    for day in days {
        println!("{}", day.run_day());
    }
}
