use crate::Runner;
use std::ops::Mul;
use std::str::FromStr;

#[derive(Debug, Default)]
pub struct BatteryPack {
    pub battery: Vec<i32>,
}

impl BatteryPack {
    pub fn max_joltage(&self) -> i32 {
        let mut max_joltage = 0;
        for i in 0..self.battery.len() {
            let first = self.battery[i].mul(10);
            for j in i+1..self.battery.len() {
                let second = self.battery[j];
                let joltage = first + second;
                if joltage > max_joltage {
                    max_joltage = joltage;
                }
            }
        }
        max_joltage
    }
}

impl FromStr for BatteryPack {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let first_line = s.lines().next().unwrap_or("");
        let cells: Vec<i32> = first_line
            .chars()
            .filter(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();
        Ok(BatteryPack { battery: cells })
    }
}

#[derive(Debug, Default)]
pub struct AdventOfCode2025Day03 {
    pub battery_packs: Vec<BatteryPack>,
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
        self.battery_packs.iter().map(|bp| bp.max_joltage()).sum::<i32>().to_string()
    }

    fn part02(&self) -> String {
        "0".to_string()
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
    fn test_day03_max_joltage() {
        let day03: AdventOfCode2025Day03 = TEST_INPUT.parse().unwrap();
        let expected_joltages = [98, 89, 78, 92];
        
        for (i, &expected) in expected_joltages.iter().enumerate() {
            assert_eq!(
                day03.battery_packs[i].max_joltage(),
                expected,
                "Battery pack at index {} failed",
                i
            );
        }
    }
}
