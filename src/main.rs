use crate::aoclib::runner::Runner;

mod aoclib;
mod aoc2025;

fn main() {
    let x = aoc2025::day01::AdventOfCode2025Day01::new();
    println!("{}", x.run());
}
