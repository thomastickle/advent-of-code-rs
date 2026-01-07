use crate::aoclib::runner::Runner;
use std::str::FromStr;

#[derive(Debug, Default)]
pub struct AdventOfCode2025Day04 {
    rolls: Vec<bool>,
    width: i32,
    height: i32,
    active_coords: Vec<(i32, i32)>,
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

    const MAX_NEIGHBORS: i32 = 4;

    fn count_removable_rolls(&self) -> i32 {
        self.active_coords
            .iter()
            .filter(|&&(x, y)| {
                let mut neighbor_count = 0;
                for (dx, dy) in Self::DIRECTIONS.iter() {
                    let nx = x + dx;
                    let ny = y + dy;

                    if nx >= 0
                        && nx < self.width
                        && ny >= 0
                        && ny < self.height
                        && self.rolls[(ny * self.width + nx) as usize]
                    {
                        neighbor_count += 1;
                        if neighbor_count >= Self::MAX_NEIGHBORS {
                            return false;
                        }
                    }
                }
                true
            })
            .count() as i32
    }

    fn remove_rolls(grid: &mut [bool], width: i32, height: i32, active: &[(i32, i32)]) -> i32 {
        let mut removed_count = 0;
        let mut to_check: Vec<(i32, i32)> = active.to_vec();
        // Track which cells are already in the next queue to avoid duplicates
        let mut is_queued = vec![false; (width * height) as usize];

        while !to_check.is_empty() {
            let mut removable = Vec::new();
            let mut next_to_check = Vec::new();

            for (x, y) in to_check {
                let idx = (y * width + x) as usize;
                if !grid[idx] {
                    continue;
                }

                let mut neighbor_count = 0;
                for (dx, dy) in Self::DIRECTIONS.iter() {
                    let nx = x + dx;
                    let ny = y + dy;
                    if nx >= 0
                        && nx < width
                        && ny >= 0
                        && ny < height
                        && grid[(ny * width + nx) as usize]
                    {
                        neighbor_count += 1;
                        if neighbor_count >= Self::MAX_NEIGHBORS {
                            break;
                        }
                    }
                }

                if neighbor_count < Self::MAX_NEIGHBORS {
                    removable.push((x, y));
                }
            }

            if removable.is_empty() {
                break;
            }

            for (x, y) in removable {
                let idx = (y * width + x) as usize;
                if grid[idx] {
                    grid[idx] = false;
                    removed_count += 1;

                    for (dx, dy) in Self::DIRECTIONS.iter() {
                        let nx = x + dx;
                        let ny = y + dy;
                        if nx >= 0 && nx < width && ny >= 0 && ny < height {
                            let n_idx = (ny * width + nx) as usize;
                            // Only add to queue if it's a roll AND not already queued
                            if grid[n_idx] && !is_queued[n_idx] {
                                is_queued[n_idx] = true;
                                next_to_check.push((nx, ny));
                            }
                        }
                    }
                }
            }

            // Clear the dirty bits for the next round
            for &(cx, cy) in &next_to_check {
                is_queued[(cy * width + cx) as usize] = false;
            }
            to_check = next_to_check;
        }
        removed_count
    }
}

impl FromStr for AdventOfCode2025Day04 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        let height = lines.len() as i32;
        let width = lines.first().map_or(0, |l| l.len()) as i32;
        let mut rolls = vec![false; (width * height) as usize];
        let mut active_coords = Vec::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '@' {
                    rolls[y * width as usize + x] = true;
                    active_coords.push((x as i32, y as i32));
                }
            }
        }
        Ok(AdventOfCode2025Day04 {
            rolls,
            width,
            height,
            active_coords,
        })
    }
}

impl Runner for AdventOfCode2025Day04 {
    type Output = i32;

    fn name(&self) -> (u32, u32) {
        (2025, 4)
    }

    fn part01(&self) -> Self::Output {
        self.count_removable_rolls()
    }

    fn part02(&self) -> Self::Output {
        let mut rolls_clone = self.rolls.clone();

        Self::remove_rolls(
            &mut rolls_clone,
            self.width,
            self.height,
            &self.active_coords,
        )
    }
}

#[cfg(test)]
pub mod tests {
    use crate::aoc2025::day04::AdventOfCode2025Day04;
    use crate::aoclib::runner::Runner;

    const TEST_INPUT: &str = include_str!("../../input/test/day04.input");

    #[test]
    fn test_day04_from_str() {
        let day04: AdventOfCode2025Day04 = TEST_INPUT.parse().unwrap();

        // Verify dimensions
        assert_eq!(day04.width, 10);
        assert_eq!(day04.height, 10);

        // Helper to check the Vec<bool> grid
        let is_set = |x: i32, y: i32| day04.rolls[(y * day04.width + x) as usize];

        // Check specific characters from TEST_INPUT
        // "..@@.@@@@." -> First line
        assert!(!is_set(0, 0)); // '.'
        assert!(!is_set(1, 0)); // '.'
        assert!(is_set(2, 0)); // '@'
        assert!(is_set(3, 0)); // '@'
        assert!(!is_set(4, 0)); // '.'

        // Check a bit of the second line "@@@.@.@.@@"
        assert!(is_set(0, 1)); // '@'
        assert!(is_set(1, 1)); // '@'
        assert!(is_set(2, 1)); // '@'
        assert!(!is_set(3, 1)); // '.'

        // Verify active_coords contains the expected number of '@' symbols
        // Count '@' in TEST_INPUT to be sure (it's 71)
        let expected_count = TEST_INPUT.chars().filter(|&c| c == '@').count();
        assert_eq!(day04.active_coords.len(), expected_count);

        // Verify active_coords contains a known point
        assert!(day04.active_coords.contains(&(2, 0)));
    }

    #[test]
    fn test_day04_part01() {
        let day04: AdventOfCode2025Day04 = TEST_INPUT.parse().unwrap();
        assert_eq!(13, day04.part01());
    }

    #[test]
    fn test_day04_part02() {
        let day04: AdventOfCode2025Day04 = TEST_INPUT.parse().unwrap();
        assert_eq!(43, day04.part02());
    }
}
