use crate::Runner;
use std::collections::HashSet;
use std::str::FromStr;


#[derive(Debug, Default)]
pub struct AdventOfCode2025Day04 {
    rolls: HashSet<(i32, i32)>,
}

impl AdventOfCode2025Day04 {
    const DIRECTIONS: [(i32, i32); 8] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    pub fn new() -> Self {
        Self::default()
    }

    fn get_removable_rolls(&self) -> Vec<(i32, i32)> {
        const MAX_NEIGHBORS: i32 = 4;
        let mut removed_rolls: Vec<(i32, i32)> = Vec::new();
        for (r_x, r_y) in self.rolls.iter() {
            let mut neighbor_count = 0;
            for (d_x, d_y) in Self::DIRECTIONS.iter() {
                let key = (r_x + d_x, r_y + d_y);
                if self.rolls.contains(&key) {
                    neighbor_count += 1;
                }
            }
            if neighbor_count < MAX_NEIGHBORS {
                removed_rolls.push((*r_x, *r_y));
            }
        }
        removed_rolls
    }

    fn remove_rolls(&mut self, all_possible: bool) -> i32 {

        let mut removed_rolls_count = 0;
        let mut finished = false;

        while !finished {
            let removable_rolls = self.get_removable_rolls();
            removed_rolls_count += removable_rolls.len() as i32;

            if removable_rolls.is_empty() || !all_possible {
                finished = true;
            }

            for roll in removable_rolls {
                self.rolls.remove(&roll);
            }
        }

        removed_rolls_count
    }
}

impl FromStr for AdventOfCode2025Day04 {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rolls = HashSet::new();
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '@' {
                    rolls.insert((x as i32, y as i32));
                }
            }
        }
        Ok(AdventOfCode2025Day04 { rolls })
    }
}

impl Runner for AdventOfCode2025Day04 {
    fn name(&self) -> (u32, u32) {
        (25, 4)
    }

    fn parse(&mut self) -> Result<(), String> {
        let content = std::fs::read_to_string("input/day04.input")
            .map_err(|e| format!("Failed to read input file: {}", e))?;
        let parsed: AdventOfCode2025Day04 = content.parse()?;
        self.rolls = parsed.rolls;
        Ok(())
    }

    fn part01(&self) -> String {
        let mut clone = AdventOfCode2025Day04 {
            rolls: self.rolls.clone(),
        };
        clone.remove_rolls(false).to_string()
    }
    fn part02(&self) -> String {
        let mut clone = AdventOfCode2025Day04 {
            rolls: self.rolls.clone(),
        };
        clone.remove_rolls(true).to_string()
    }

    fn run(&self) {
        let (_, day) = self.name();
        println!("    Day {:02}", day);
        println!("        Part 01: {}", self.part01());
        println!("        Part 02: {}", self.part02());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "..@@.@@@@.\n\
        @@@.@.@.@@\n\
        @@@@@.@.@@\n\
        @.@@@@..@.\n\
        @@.@@@@.@@\n\
        .@@@@@@@.@\n\
        .@.@.@.@@@\n\
        @.@@@.@@@@\n\
        .@@@@@@@@.\n\
        @.@.@@@.@.\n";

    #[test]
    fn test_day04_from_str() {
        let aoc_day04: AdventOfCode2025Day04 = TEST_INPUT.parse().unwrap();
        // Check some specific coordinates based on TEST_INPUT
        // "..@@.@@@@." -> (2,0), (3,0), (5,0), (6,0), (7,0), (8,0) are '@'
        assert!(aoc_day04.rolls.contains(&(2, 0)));
        assert!(aoc_day04.rolls.contains(&(3, 0)));
        assert!(!aoc_day04.rolls.contains(&(0, 0)));
        assert!(!aoc_day04.rolls.contains(&(1, 0)));
        assert!(aoc_day04.rolls.contains(&(2, 1))); // "@@@.@.@.@@" -> (0,1), (1,1), (2,1)...
    }

    #[test]
    fn test_remove_rolls() {
        let mut aoc_day04: AdventOfCode2025Day04 = TEST_INPUT.parse().unwrap();
        let removed = aoc_day04.remove_rolls(false);
        assert_eq!(removed, 13);
    }

    #[test]
    fn test_remove_rolls_all() {
        let mut aoc_day04: AdventOfCode2025Day04 = TEST_INPUT.parse().unwrap();
        let removed = aoc_day04.remove_rolls(true);
        assert_eq!(removed, 43);
    }
}
