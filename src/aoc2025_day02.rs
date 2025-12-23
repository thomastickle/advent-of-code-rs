use std::str::FromStr;

#[derive(Debug, PartialEq, Default)]
pub struct Range<T> {
    pub start: T,
    pub end: T,
}

impl<T> Range<T>
where
    T: std::ops::AddAssign + Default + Copy + std::fmt::Display,
    std::ops::RangeInclusive<T>: Iterator<Item=T>,
{
    pub fn sum_of_invalids(&self) -> T {
        (self.start..=self.end)
            .filter(|&i| {
                let s = i.to_string();
                let len = s.len();
                len % 2 == 0 && s[..len / 2] == s[len / 2..]
            })
            .fold(T::default(), |mut acc, i| {
                acc += i;
                acc
            })
    }

    pub fn sum_of_multi_invalids(&self) -> T {
        (self.start..=self.end)
            .filter(|&i| {
                let s = i.to_string();
                let len = s.len();
                (1..=len / 2).any(|size| {
                    len % size == 0 && s.as_bytes().chunks(size).all(|chunk| chunk == &s.as_bytes()[..size])
                })
            })
            .fold(T::default(), |mut acc, i| {
                acc += i;
                acc
            })
    }
}

impl<T> FromStr for Range<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Display,
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 2 {
            return Err(format!(
                "Invalid range format: '{}'. Expected 'start-end'",
                s
            ));
        }

        let start = parts[0]
            .trim()
            .parse::<T>()
            .map_err(|e| format!("Failed to parse start value: {}", e))?;
        let end = parts[1]
            .trim()
            .parse::<T>()
            .map_err(|e| format!("Failed to parse end value: {}", e))?;

        Ok(Range { start, end })
    }
}

pub trait Runner {
    fn name(&self) -> (u32, u32);
    fn parse(&mut self) -> Result<(), String>;
    fn run(&self);
}

#[derive(Debug, Default)]
pub struct AdventOfCode2025Day02 {
    ranges: Vec<Range<i64>>,
}

impl AdventOfCode2025Day02 {
    pub fn new() -> Self {
        Self::default()
    }

    fn part01(&self) -> i64 {
        self.ranges.iter().map(|r| r.sum_of_invalids()).sum()
    }

    fn part02(&self) -> i64 {
        self.ranges.iter().map(|r| r.sum_of_multi_invalids()).sum()
    }
}

impl Runner for AdventOfCode2025Day02 {
    fn name(&self) -> (u32, u32) {
        (2025, 2)
    }

    fn parse(&mut self) -> Result<(), String> {
        let content = std::fs::read_to_string("input\\day02.input")
            .map_err(|e| format!("Failed to read input file: {}", e))?;
        let parsed: AdventOfCode2025Day02 = content.trim().parse()?;
        self.ranges = parsed.ranges;
        Ok(())
    }

    fn run(&self) {
        let (year, day) = self.name();
        let now = std::time::Instant::now();
        let part1_result = self.part01();
        let part1_duration = now.elapsed();
        println!(
            "Advent of Code {} Day {:02} Part 1: {} ({:?})",
            year, day, part1_result, part1_duration
        );

        let now = std::time::Instant::now();
        let part2_result = self.part02();
        let part2_duration = now.elapsed();
        println!(
            "Advent of Code {} Day {:02} Part 2: {} ({:?})",
            year, day, part2_result, part2_duration
        );
    }
}

impl FromStr for AdventOfCode2025Day02 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ranges = s
            .split(',')
            .map(|range_str| range_str.trim().parse::<Range<i64>>())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(AdventOfCode2025Day02 { ranges })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
                              1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
                              824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_range_from_str() {
        assert_eq!(
            "1-2".parse::<Range<i32>>().unwrap(),
            Range { start: 1, end: 2 }
        );
        assert_eq!(
            " 10 - 20 ".parse::<Range<u32>>().unwrap(),
            Range { start: 10, end: 20 }
        );
    }

    #[test]
    fn test_range_sum_of_invalids() {
        let first_range_str = TEST_INPUT.split(',').next().unwrap();
        let range: Range<i32> = first_range_str.parse().unwrap();
        assert_eq!(range.sum_of_invalids(), 33);
    }

    #[test]
    fn test_range_from_str_invalid() {
        assert!("1-a".parse::<Range<i32>>().is_err());
        assert!("1".parse::<Range<i32>>().is_err());
        assert!("1-2-3".parse::<Range<i32>>().is_err());
        assert!("".parse::<Range<i32>>().is_err());
    }

    #[test]
    fn test_day02_part01() {
        let aoc_day02: AdventOfCode2025Day02 = TEST_INPUT.parse().unwrap();
        assert_eq!(aoc_day02.part01(), 1227775554_i64);
    }

    #[test]
    fn test_day02_part02() {
        let aoc_day02: AdventOfCode2025Day02 = TEST_INPUT.parse().unwrap();
        assert_eq!(aoc_day02.part02(), 4174379265_i64);
    }

    #[test]
    fn test_day02_name() {
        let aoc_day02 = AdventOfCode2025Day02::default();
        assert_eq!(aoc_day02.name(), (2025, 2));
    }

    #[test]
    fn test_day02_parse() {
        let mut aoc_day02 = AdventOfCode2025Day02::default();
        aoc_day02.parse().unwrap();
        assert!(!aoc_day02.ranges.is_empty());
        // Verify the first range matches what's in input/day02.input
        assert_eq!(
            aoc_day02.ranges[0],
            Range {
                start: 7777742220_i64,
                end: 7777814718_i64
            }
        );
    }
}
