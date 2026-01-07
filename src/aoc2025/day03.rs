use crate::aoclib::runner::Runner;
use std::str::FromStr;

#[derive(Debug, Default)]
#[derive(PartialEq)]
pub struct BatteryPack {
    pub battery: Vec<u64>,
}

impl BatteryPack {
    /// Returns the maximum joltage that can be formed by selecting exactly `k` digits
    /// from the battery pack while maintaining their original relative order.
    ///
    /// # Arguments
    /// * `max_cells` - The number of digits (cells) to turn on in the battery pack.
    pub fn max_joltage(&self, max_cells: usize) -> u64 {
        let mut result = Vec::with_capacity(max_cells);
        let to_remove = self.battery.len() - max_cells;
        let mut removed = 0;

        for &digit in &self.battery { // Dereference so we don't have to dereference in the loop.
            while removed < to_remove && !result.is_empty() && *result.last().unwrap() < digit {
                result.pop();
                removed += 1;
            }
            if result.len() < max_cells {
                result.push(digit);
            } else {
                removed += 1;
            }
        }

        result.iter().fold(0, |acc, &d| acc * 10 + d)
    }
}

impl FromStr for BatteryPack {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let first_line = s.lines().next().unwrap_or("");
        let cells: Vec<u64> = first_line
            .chars()
            .map(|c| {
                c.to_digit(10).map(|d| d as u64).ok_or_else(|| format!("Invalid digit: '{}'", c))
            }).collect::<Result<Vec<_>, _>>()?;
        Ok(BatteryPack { battery: cells })
    }
}

#[derive(Debug, Default)]
pub struct AdventOfCode2025Day03 {
    battery_packs: Vec<BatteryPack>,
}

impl FromStr for AdventOfCode2025Day03 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let battery_packs = s
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|line| line.parse::<BatteryPack>())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(AdventOfCode2025Day03 { battery_packs })
    }
}

impl Runner for AdventOfCode2025Day03 {
    type Output = u64;

    fn name(&self) -> (u32, u32) {
        (2025, 3)
    }

    fn part01(&self) -> Self::Output {
        self.battery_packs.iter().map(|bp| bp.max_joltage(2)).sum::<u64>()
    }

    fn part02(&self) -> Self::Output {
        self.battery_packs.iter().map(|bp| bp.max_joltage(12)).sum::<u64>()
    }
}


#[cfg(test)]
mod tests {
    use crate::aoc2025::day03::{AdventOfCode2025Day03, BatteryPack};
    use crate::aoclib::runner::Runner;
    use std::str::FromStr;

    const TEST_INPUT: &str = include_str!("../../input/test/day03.input");

    #[test]
    fn test_battery_pack_from_str() {
        let input = "1234567890";
        let expected = BatteryPack { battery: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0] };
        let result = BatteryPack::from_str(input).unwrap();
        assert_eq!(result, expected);

        // Test the failure case
        let input = "1234AB1234";
        let result = BatteryPack::from_str(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_battery_pack_max_joltage() {
        let pack = BatteryPack { battery: vec![9,8,6,4,3,2,1,1,1,1,1,1,1] };
        assert_eq!(98, pack.max_joltage(2));

        let pack = BatteryPack { battery: vec![9,8,7,6,5,4,3,2,1,1,1,1,1,1,1] };
        assert_eq!(987654321111, pack.max_joltage(12));
    }

    #[test]
    fn test_name() {
        let x = AdventOfCode2025Day03::default().name();
        assert_eq!((2025, 3), x);
    }

    #[test]
    fn test_from_str() {
        let day03 = TEST_INPUT.parse::<AdventOfCode2025Day03>().unwrap();
        assert_eq!(4, day03.battery_packs.len());
        assert_eq!(15, day03.battery_packs[0].battery.len());
    }

    #[test]
    fn test_part01() {
        let day03 = TEST_INPUT.parse::<AdventOfCode2025Day03>().unwrap();
        assert_eq!(357, day03.part01());
    }

    #[test]
    fn test_part02() {
        let day03 = TEST_INPUT.parse::<AdventOfCode2025Day03>().unwrap();
        assert_eq!(3121910778619, day03.part02());
    }
}
