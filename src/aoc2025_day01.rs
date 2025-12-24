use crate::aoc2025_day01::Rotation::{Left, Right};
use crate::Runner;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Rotation {
    Left(i32),
    Right(i32),
}

impl FromStr for Rotation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = s.chars().next().ok_or("Empty rotation string")?;
        let amount = s[1..]
            .parse::<i32>()
            .map_err(|e| format!("Failed to parse rotation amount: {}", e))?;

        match direction {
            'L' => Ok(Left(amount)),
            'R' => Ok(Right(amount)),
            _ => Err(format!("Invalid direction: {}", direction)),
        }
    }
}

#[derive(Debug, Default)]
pub struct AdventOfCode2025Day01 {
    pub rotations: Vec<Rotation>,
}

impl AdventOfCode2025Day01 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn part01(&self) -> i32 {
        const DIAL_SIZE: i32 = 100;
        self.rotations
            .iter()
            .scan(50, |dial, rotation| {
                match rotation {
                    Left(amount) => *dial = (*dial - amount).rem_euclid(DIAL_SIZE),
                    Right(amount) => *dial = (*dial + amount).rem_euclid(DIAL_SIZE),
                }
                Some(*dial)
            })
            .filter(|&dial| dial == 0)
            .count() as i32
    }

    pub fn part02(&self) -> i32 {
        self.rotations
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

impl Runner for AdventOfCode2025Day01 {
    fn name(&self) -> (u32, u32) {
        (2025, 1)
    }

    fn parse(&mut self) -> Result<(), String> {
        let content = std::fs::read_to_string("input/day01.input")
            .map_err(|e| format!("Failed to read input file: {}", e))?;
        let parsed: AdventOfCode2025Day01 = content.trim().parse()?;
        self.rotations = parsed.rotations;
        Ok(())
    }

    fn part01(&self) -> String {
        self.part01().to_string()
    }

    fn part02(&self) -> String {
        self.part02().to_string()
    }
}

impl FromStr for AdventOfCode2025Day01 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rotations = s
            .split([',', '\n', '\r'])
            .map(|part| part.trim())
            .filter(|part| !part.is_empty())
            .map(|part| part.parse::<Rotation>())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(AdventOfCode2025Day01 { rotations })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";

    #[test]
    fn test_rotation_from_str() {
        assert_eq!("L1".parse::<Rotation>().unwrap(), Rotation::Left(1));
        assert_eq!("R123".parse::<Rotation>().unwrap(), Rotation::Right(123));
        assert!("X1".parse::<Rotation>().is_err());
        assert!("L".parse::<Rotation>().is_err());
        assert!("".parse::<Rotation>().is_err());
    }

    #[test]
    fn test_day01_from_str() {
        let input = "L1, R2, L3";
        let day01: AdventOfCode2025Day01 = input.parse().unwrap();
        assert_eq!(day01.rotations.len(), 3);
        assert_eq!(day01.rotations[0], Rotation::Left(1));
        assert_eq!(day01.rotations[1], Rotation::Right(2));
        assert_eq!(day01.rotations[2], Rotation::Left(3));
    }

    #[test]
    fn test_day01_part01() {
        let day01: AdventOfCode2025Day01 = TEST_INPUT.parse().unwrap();
        assert_eq!(day01.part01(), 3);
    }

    #[test]
    fn test_day01_part02() {
        let day01: AdventOfCode2025Day01 = TEST_INPUT.parse().unwrap();
        assert_eq!(day01.part02(), 6);
    }
}
