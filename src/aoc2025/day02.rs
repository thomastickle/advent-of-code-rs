use crate::aoclib::runner::Runner;
use std::collections::BTreeSet;
use std::str::FromStr;

#[derive(Debug, PartialEq, Default)]
pub struct Range {
    pub start: u64,
    pub end: u64,
}

impl Range {

    /// AOC Day 02 Part 01
    ///
    /// Sum up all the invalid numbers which are designated as repeating values of the upper and
    /// half of the number in the range.
    ///
    /// Note: this method uses strings comparison and walking the entire range, which is slower than
    /// a numeric approach.
    #[allow(dead_code)]
    pub fn sum_of_invalids(&self) -> u64 {
        (self.start..=self.end)
            .filter(|&i| {
                let s = i.to_string();
                let len = s.len();
                len % 2 == 0 && s[..len / 2] == s[len / 2..]
            })
            .fold(u64::default(), |mut acc, i| {
                acc += i;
                acc
            })
    }

    /// AOC Day 02 Part 01
    ///
    /// Sum up all the invalid numbers which are designated as repeating values of the upper and
    /// half of the number in the range.
    ///
    /// Note: this method uses strings comparison and walking the entire range, which is slower than
    /// a numeric approach.
    pub fn sum_of_invalids_fast(&self) -> u64 {
        let mut sum = 0;

        for number_length in 1..=9 {
            let multiplier = 10u64.pow(number_length) + 1;

            // Calculate the minimum bound on the prefix that we would find dupes.
            let min_prefix = 10u64.pow(number_length - 1);

            let start_prefix = min_prefix.max(self.start.div_ceil( multiplier));
            let end_prefix = 10u64.pow(number_length) - 1;

            for prefix in start_prefix..=end_prefix {
                let value = prefix * multiplier;
                if value > self.end {
                    break;
                }
                sum += value;
            }
        }
        sum
    }

    /// AOC Day 02 Part 02
    ///
    /// Sum up all the invalid numbers which are designated as all repeating values up until the upper
    /// and lower half of the number in the range.
    ///
    /// Note: this method uses strings comparison and walking the entire range, which is slower than
    /// a numeric approach.
    #[allow(dead_code)]
    pub fn sum_of_multi_invalids(&self) -> u64 {
        (self.start..=self.end)
            .filter(|&i| {
                let s = i.to_string();
                let len = s.len();
                (1..=len / 2).any(|size| {
                    len % size == 0
                        && s.as_bytes()
                            .chunks(size)
                            .all(|chunk| chunk == &s.as_bytes()[..size])
                })
            })
            .fold(u64::default(), |mut acc, i| {
                acc += i;
                acc
            })
    }

    pub fn sum_of_multi_invalids_fast(&self) -> u64 {
        let mut invalid_numbers = BTreeSet::new();

        for l in 1..=9 {
            let p_base = 10u64.pow(l);
            let min_p = 10u64.pow(l - 1);
            let max_p = 10u64.pow(l) - 1;

            for k in 2..=20 {
                let mut multiplier = 0u64;
                let mut overflowed = false;
                for i in 0..k {
                    if let Some(pow) = p_base.checked_pow(i) {
                        multiplier = multiplier.saturating_add(pow);
                    } else {
                        overflowed = true;
                        break;
                    }
                }

                if overflowed || multiplier == 0 {
                    break;
                }

                // Calculate the first prefix p such that p * multiplier >= self.start
                let start_prefix = min_p.max(self.start.div_ceil(multiplier));

                for p in start_prefix..=max_p {
                    let val = match p.checked_mul(multiplier) {
                        Some(v) => v,
                        None => break,
                    };

                    if val > self.end {
                        break;
                    }

                    invalid_numbers.insert(val);
                }
            }
        }

        invalid_numbers.iter().sum()
    }
}

impl FromStr for Range {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start_str, end_str) = s
            .split_once('-')
            .ok_or_else(|| format!("Invalid range format: '{}'. Expected 'start-end'", s))?;

        let start = start_str
            .trim()
            .parse::<u64>()
            .map_err(|e| format!("Failed to parse start value: {}", e))?;
        let end = end_str
            .trim()
            .parse::<u64>()
            .map_err(|e| format!("Failed to parse end value: {}", e))?;

        Ok(Range { start, end })
    }
}

#[derive(Debug, Default)]
pub struct AdventOfCode2025Day02 {
    ranges: Vec<Range>,
}

impl FromStr for AdventOfCode2025Day02 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ranges = s
            .split(',')
            .filter(|s| !s.trim().is_empty())
            .map(|range_str| range_str.trim().parse::<Range>())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(AdventOfCode2025Day02 { ranges })
    }
}

impl Runner for AdventOfCode2025Day02 {
    type Output = u64;

    fn name(&self) -> (u32, u32) {
        (2025, 2)
    }

    fn part01(&self) -> Self::Output {
        self.ranges
            .iter()
            .map(|r| r.sum_of_invalids_fast())
            .sum()
    }

    fn part02(&self) -> Self::Output {
        self.ranges
            .iter()
            .map(|r| r.sum_of_multi_invalids_fast())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc2025::day02::{AdventOfCode2025Day02, Range};
    use crate::aoclib::runner::Runner;

    const TEST_INPUT: &str = include_str!("../../input/test/day02.input");

    #[test]
    fn test_range_from_str() {
        assert_eq!("1-2".parse::<Range>().unwrap(), Range { start: 1, end: 2 });
        assert_eq!(
            " 10 - 20 ".parse::<Range>().unwrap(),
            Range { start: 10, end: 20 }
        );

        assert!("1-a".parse::<Range>().is_err());
        assert!("1".parse::<Range>().is_err());
        assert!("1-2-3".parse::<Range>().is_err());
        assert!("".parse::<Range>().is_err());
    }

    #[test]
    fn test_range_sum_of_invalids() {
        let first_range_str = TEST_INPUT.split(',').next().unwrap();
        let range: Range = first_range_str.parse().unwrap();
        assert_eq!(range.sum_of_invalids(), 33);
    }

    #[test]
    fn test_name() {
        let day02 = AdventOfCode2025Day02::default();
        assert_eq!((2025, 2), day02.name());
    }

    #[test]
    fn test_part01() {
        let day02 = TEST_INPUT.parse::<AdventOfCode2025Day02>().unwrap();
        assert_eq!(1227775554, day02.part01());
    }

    #[test]
    fn test_part02() {
        let day02 = TEST_INPUT.parse::<AdventOfCode2025Day02>().unwrap();
        assert_eq!(4174379265, day02.part02());
    }
}
