use crate::Runner;
use std::str::FromStr;

#[derive(Debug, Default)]
pub struct BatteryPack {
    pub battery: Vec<u64>,
}

impl BatteryPack {
    
}

impl BatteryPack {
    /// Returns the maximum joltage that can be formed by selecting exactly `k` digits
    /// from the battery pack while maintaining their original relative order.
    ///
    /// # Arguments
    /// * `k` - The number of digits (cells) to turn on in the battery pack.
    pub fn max_joltage(&self, k: usize) -> u64 {
        let mut result = Vec::with_capacity(k);
        let to_remove = self.battery.len() - k;
        let mut removed = 0;

        for &digit in &self.battery {
            while removed < to_remove && !result.is_empty() && *result.last().unwrap() < digit {
                result.pop();
                removed += 1;
            }
            if result.len() < k {
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
            .filter(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect();
        Ok(BatteryPack { battery: cells })
    }
}

#[derive(Debug, Default)]
pub struct AdventOfCode2025Day03 {
    pub battery_packs: Vec<BatteryPack>,
}

impl AdventOfCode2025Day03 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for AdventOfCode2025Day03 {
    fn name(&self) -> (u32, u32) {
        (2025, 3)
    }

    fn parse(&mut self) -> Result<(), String> {
        let content = std::fs::read_to_string("input/day03.input")
            .map_err(|e| format!("Failed to read input file: {}", e))?;
        let parsed: AdventOfCode2025Day03 = content.parse()?;
        self.battery_packs = parsed.battery_packs;
        Ok(())
    }

    fn part01(&self) -> String {
        self.battery_packs.iter().map(|bp| bp.max_joltage(2)).sum::<u64>().to_string()
    }

    fn part02(&self) -> String {
        self.battery_packs.iter().map(|bp| bp.max_joltage(12)).sum::<u64>().to_string()
    }
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


#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "987654321111111\n\
                              811111111111119\n\
                              234234234234278\n\
                              818181911112111";

    #[test]
    fn test_day03_from_str() {
        let day03: AdventOfCode2025Day03 = TEST_INPUT.parse().unwrap();
        // There are 4 lines in TEST_INPUT.
        assert_eq!(day03.battery_packs.len(), 4);
        
        // The first battery pack should have 15 digits.
        let first_pack = &day03.battery_packs[0];
        assert_eq!(first_pack.battery.len(), 15);
        assert_eq!(first_pack.battery[0], 9);
        assert_eq!(first_pack.battery[14], 1);

        // The second battery pack should also be parsed correctly.
        let second_pack = &day03.battery_packs[1];
        assert_eq!(second_pack.battery[0], 8);
        assert_eq!(second_pack.battery[14], 9);
    }

    #[test]
    fn test_day03_max_2_cell_in_order_joltage() {
        let day03: AdventOfCode2025Day03 = TEST_INPUT.parse().unwrap();
        let expected_joltages: [u64; 4] = [98, 89, 78, 92];
        
        for (i, &expected) in expected_joltages.iter().enumerate() {
            assert_eq!(
                day03.battery_packs[i].max_joltage(2),
                expected,
                "Battery pack at index {} failed",
                i
            );
        }
    }

    #[test]
    fn test_day03_max_12_cell_in_order_joltage() {
        let day03: AdventOfCode2025Day03 = TEST_INPUT.parse().unwrap();
        let expected_joltages: [u64; 4] = [987654321111, 811111111119, 434234234278, 888911112111];

        for (i, &expected) in expected_joltages.iter().enumerate() {
            assert_eq!(
                day03.battery_packs[i].max_joltage(12),
                expected,
                "Battery pack at index {} failed",
                i
            );
        }
    }
}
