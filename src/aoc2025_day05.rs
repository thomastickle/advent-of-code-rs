use crate::Runner;
use rayon::prelude::*;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct FreshIngredientRange {
    start: usize,
    end: usize,
}

impl FreshIngredientRange {
    fn contains(&self, value: usize) -> bool {
        self.start <= value && value <= self.end
    }
}

#[derive(Debug, Default)]
pub struct AdventOfCode2025Day05 {
    fresh_ingredient_ranges: Vec<FreshIngredientRange>,
    ingredients: HashSet<usize>,
}

impl AdventOfCode2025Day05 {
    pub fn new() -> Self {
        Self::default()
    }

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

    pub fn get_fresh_ingredients_count(&self) -> usize {
        self.ingredients
            .par_iter()
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
            .count()
    }

    pub fn get_total_fresh_slots_count(&self) -> usize {
        // Sum the sizes of disjoint ranges
        self.fresh_ingredient_ranges
            .iter()
            .map(|r| r.end - r.start + 1)
            .sum()
    }
}

impl FromStr for AdventOfCode2025Day05 {
    type Err = String;
    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        let mut fresh_ingredient_ranges = Vec::new();
        let mut ingredients = HashSet::new();
        let mut blank_line_seen = false;
        for line in _s.lines() {
            let line = line.trim();
            if line.is_empty() {
                blank_line_seen = true;
                continue;
            }

            if !blank_line_seen {
                let parts: Vec<&str> = line.split('-').collect();
                if parts.len() != 2 {
                    return Err("Invalid range format".to_string());
                }
                let start = parts[0]
                    .parse::<usize>()
                    .map_err(|_| "Invalid start value".to_string())?;
                let end = parts[1]
                    .parse::<usize>()
                    .map_err(|_| "Invalid end value".to_string())?;
                fresh_ingredient_ranges.push(FreshIngredientRange { start, end });
            } else {
                let ingredient = line
                    .parse::<usize>()
                    .map_err(|_| "Invalid ingredient value".to_string())?;
                ingredients.insert(ingredient);
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
    fn name(&self) -> (u32, u32) {
        (2025, 5)
    }

    fn parse(&mut self) -> Result<(), String> {
        let content = std::fs::read_to_string(self.input_path())
            .map_err(|e| format!("Failed to read input file: {}", e))?;
        let parsed: AdventOfCode2025Day05 = content.parse()?;
        self.fresh_ingredient_ranges = parsed.fresh_ingredient_ranges;
        self.ingredients = parsed.ingredients;
        Ok(())
    }

    fn part01(&self) -> String {
        self.get_fresh_ingredients_count().to_string()
    }

    fn part02(&self) -> String {
        self.get_total_fresh_slots_count().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3-5\n\
        10-14\n\
        16-20\n\
        12-18\n\
        \n\
        1\n\
        5\n\
        8\n\
        11\n\
        17\n\
        32\n";

    #[test]
    fn test_part01() {
        let day05 = TEST_INPUT.parse::<AdventOfCode2025Day05>().unwrap();
        assert_eq!(day05.part01(), "3");
    }

    #[test]
    fn test_part02() {
        let day05 = TEST_INPUT.parse::<AdventOfCode2025Day05>().unwrap();
        assert_eq!(day05.part02(), "14");
    }
}
