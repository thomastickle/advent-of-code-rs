use crate::aoclib::runner::Runner;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct FreshIngredientRange {
    start: u64,
    end: u64,
}

impl FromStr for FreshIngredientRange {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 2 {
            return Err("Invalid range format".to_string());
        }
        let start = parts[0]
            .parse::<u64>()
            .map_err(|_| "Invalid start value".to_string())?;
        let end = parts[1]
            .parse::<u64>()
            .map_err(|_| "Invalid end value".to_string())?;
        Ok(Self { start, end })
    }
}

impl FreshIngredientRange {
    fn contains(&self, value: u64) -> bool {
        self.start <= value && value <= self.end
    }
}

#[derive(Debug, Default)]
pub struct AdventOfCode2025Day05 {
    fresh_ingredient_ranges: Vec<FreshIngredientRange>,
    ingredients: Vec<u64>,
}

impl AdventOfCode2025Day05 {
    fn merge_ranges(ranges: &mut Vec<FreshIngredientRange>) {
        if ranges.is_empty() {
            return;
        }

        // Sort ranges by start position
        ranges.sort_by_key(|r| r.start);

        // Merge overlapping ranges
        let mut merged: Vec<FreshIngredientRange> = Vec::new();
        let mut current = ranges[0].clone();

        for next in ranges.iter().skip(1) {
            if next.start <= current.end {
                // Overlap or adjacent, extend current range
                current.end = current.end.max(next.end);
            } else {
                // Disjoint, push current and start new one
                merged.push(current);
                current = next.clone();
            }
        }
        merged.push(current);
        *ranges = merged;
    }
}

impl FromStr for AdventOfCode2025Day05 {
    type Err = String;
    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        let mut fresh_ingredient_ranges = Vec::new();
        let mut ingredients = Vec::new();
        let mut blank_line_seen = false;
        for line in _s.lines() {
            let line = line.trim();
            if line.is_empty() {
                blank_line_seen = true;
                continue;
            }

            if !blank_line_seen {
                let fresh_ingredient_range = line.parse::<FreshIngredientRange>().unwrap();
                fresh_ingredient_ranges.push(fresh_ingredient_range);
            } else {
                let ingredient = line
                    .parse::<u64>()
                    .map_err(|_| "Invalid ingredient value".to_string())?;
                ingredients.push(ingredient);
            }
        }

        Self::merge_ranges(&mut fresh_ingredient_ranges);

        Ok(Self {
            fresh_ingredient_ranges,
            ingredients,
        })
    }
}

impl Runner for AdventOfCode2025Day05 {
    type Output = u64;

    fn name(&self) -> (u32, u32) {
        (2025, 5)
    }

    fn part01(&self) -> Self::Output {
        self.ingredients
            .iter()
            .filter(|&&ingredient| {
                // Binary search for a range that might contain the ingredient
                self.fresh_ingredient_ranges
                    .binary_search_by(|range| {
                        if range.contains(ingredient) {
                            std::cmp::Ordering::Equal
                        } else if ingredient < range.start {
                            std::cmp::Ordering::Greater
                        } else {
                            std::cmp::Ordering::Less
                        }
                    })
                    .is_ok()
            })
            .count() as u64
    }

    /// Sum up all the fresh ingredient ranges to get the total number of
    /// fresh ingredients.
    fn part02(&self) -> Self::Output {
        self.fresh_ingredient_ranges
            .iter()
            .map(|r| r.end - r.start + 1)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc2025::day05::AdventOfCode2025Day05;
    use crate::aoclib::runner::Runner;

    const TEST_INPUT: &str = include_str!("../../input/test/day05.input");

    #[test]
    fn test_name() {
        let day05 = AdventOfCode2025Day05::default();
        assert_eq!((2025, 5), day05.name());
    }

    #[test]
    fn test_part01() {
        let day05 = TEST_INPUT.parse::<AdventOfCode2025Day05>().unwrap();
        assert_eq!(3, day05.part01());
    }

    #[test]
    fn test_part02() {
        let day05 = TEST_INPUT.parse::<AdventOfCode2025Day05>().unwrap();
        assert_eq!(14, day05.part02());
    }
}
