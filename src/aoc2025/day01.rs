use crate::aoc2025::day01::Direction::{Left, Right};
use crate::aoclib::runner::Runner;
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Direction {
    Left(i32),
    Right(i32),
}

#[derive(Debug)]
struct AdventOfCode2025Day01 {
    directions: Vec<Direction>,
}

impl AdventOfCode2025Day01 {
    pub const DIAL_START_POSITION: i32 = 50;
}

impl FromStr for AdventOfCode2025Day01 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let directions: Result<Vec<Direction>, String> = s
            .lines()
            .map(|line| {
                let value = line[1..]
                    .parse::<i32>()
                    .map_err(|_| format!("Invalid number in line: {}", line))?;

                match &line[0..1] {
                    "L" => Ok(Left(value)),
                    "R" => Ok(Right(value)),
                    _ => Err(format!("Invalid direction in line: {}", line)),
                }
            })
            .collect();

        Ok(AdventOfCode2025Day01 {
            directions: directions?,
        })
    }
}

impl Runner for AdventOfCode2025Day01 {
    type Output = i32;

    fn name(&self) -> (u32, u32) {
        (2025, 1)
    }

    fn new() -> Self {
        AdventOfCode2025Day01 { directions: vec![] }
    }

    fn parse(&self, input: &str) -> Self {
        input
            .parse::<AdventOfCode2025Day01>()
            .unwrap_or_else(|e| panic!("Failed to parse input: {}", e))
    }

    fn part01(&self) -> i32 {
        self.directions
            .iter()
            .fold((0, Self::DIAL_START_POSITION), |acc, direction| {
                let position = match direction {
                    Left(steps) => (acc.1 - steps).rem_euclid(100),
                    Right(steps) => (acc.1 + steps).rem_euclid(100),
                };

                match position {
                    0 => (acc.0 + 1, position),
                    _ => (acc.0, position),
                }
            })
            .0
    }

    fn part02(&self) -> i32 {
        self.directions
            .iter()
            .scan(50i32, |abs_dial, rotation| {
                let start = *abs_dial;
                match rotation {
                    Left(amount) => *abs_dial -= amount,
                    Right(amount) => *abs_dial += amount,
                }
                let end = *abs_dial;
                if end > start {
                    Some(end.div_euclid(100) - start.div_euclid(100))
                } else if end < start {
                    Some((-end).div_euclid(100) - (-start).div_euclid(100))
                } else {
                    Some(0)
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc2025::day01::{AdventOfCode2025Day01, Direction};
    use crate::aoclib::runner::Runner;

    const TEST_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn direction() {
        let direction = Direction::Left(1);
        assert_eq!(direction, Direction::Left(1))
    }

    #[test]
    fn test_parse() {
        let day01 = AdventOfCode2025Day01::new();
        let parsed = day01.parse(TEST_INPUT);
        assert_eq!(10, parsed.directions.len());
    }

    #[test]
    fn test_from_str() {
        let day01 = TEST_INPUT.parse::<AdventOfCode2025Day01>().unwrap();
        assert_eq!(10, day01.directions.len());
    }

    #[test]
    fn test_part01() {
        let day01 = TEST_INPUT.parse::<AdventOfCode2025Day01>().unwrap();
        assert_eq!(3, day01.part01());
    }

    #[test]
    fn test_part02() {
        let day01 = TEST_INPUT.parse::<AdventOfCode2025Day01>().unwrap();
        assert_eq!(6, day01.part02());
    }

    #[test]
    fn name() {
        assert_eq!((2025, 1), AdventOfCode2025Day01::new().name());
    }
}
