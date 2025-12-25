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

    /// Counts how many rolls have fewer than 4 neighbors.
    /// This is used for Part 1 to identify which rolls would be removed in the first pass
    /// without actually modifying the collection.
    fn count_removable_rolls(&self) -> i32 {
        const MAX_NEIGHBORS: i32 = 4;
        let mut count = 0;
        for (r_x, r_y) in self.rolls.iter() {
            let mut neighbor_count = 0;
            for (d_x, d_y) in Self::DIRECTIONS.iter() {
                let key = (r_x + d_x, r_y + d_y);
                if self.rolls.contains(&key) {
                    neighbor_count += 1;
                }
            }
            if neighbor_count < MAX_NEIGHBORS {
                count += 1;
            }
        }
        count
    }

    /// Removes rolls that have fewer than 4 neighbors iteratively until the collection stabilizes.
    ///
    /// # Arguments
    /// * `rolls` - The mutable collection of roll coordinates.
    fn remove_rolls(rolls: &mut HashSet<(i32, i32)>) -> i32 {
        const MAX_NEIGHBORS: i32 = 4;
        let mut removed_count = 0;

        // Iterative removal until stable
        let mut to_check: Vec<(i32, i32)> = rolls.iter().cloned().collect();
        
        while !to_check.is_empty() {
            let mut next_to_check = HashSet::new();
            let mut removable = Vec::new();

            for roll in to_check {
                let mut neighbor_count = 0;
                for (d_x, d_y) in Self::DIRECTIONS.iter() {
                    if rolls.contains(&(roll.0 + d_x, roll.1 + d_y)) {
                        neighbor_count += 1;
                    }
                }
                if neighbor_count < MAX_NEIGHBORS {
                    removable.push(roll);
                }
            }

            if removable.is_empty() {
                break;
            }

            for roll in removable {
                if rolls.remove(&roll) {
                    removed_count += 1;
                    // When a roll is removed, its neighbors need to be re-checked
                    for (d_x, d_y) in Self::DIRECTIONS.iter() {
                        let neighbor = (roll.0 + d_x, roll.1 + d_y);
                        if rolls.contains(&neighbor) {
                            next_to_check.insert(neighbor);
                        }
                    }
                }
            }
            to_check = next_to_check.into_iter().collect();
        }

        removed_count
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
        self.count_removable_rolls().to_string()
    }
    fn part02(&self) -> String {
        let mut rolls_clone = self.rolls.clone();
        Self::remove_rolls(&mut rolls_clone).to_string()
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
        let aoc_day04: AdventOfCode2025Day04 = TEST_INPUT.parse().unwrap();
        let removed = aoc_day04.count_removable_rolls();
        assert_eq!(removed, 13);
    }

    #[test]
    fn test_remove_rolls_all() {
        let mut aoc_day04: AdventOfCode2025Day04 = TEST_INPUT.parse().unwrap();
        let removed = AdventOfCode2025Day04::remove_rolls(&mut aoc_day04.rolls);
        assert_eq!(removed, 43);
    }
}
