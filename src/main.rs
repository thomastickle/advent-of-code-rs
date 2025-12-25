use adventofcode_rs::aoc2025_day01::AdventOfCode2025Day01;
use adventofcode_rs::aoc2025_day02::AdventOfCode2025Day02;
use adventofcode_rs::aoc2025_day03::AdventOfCode2025Day03;
use adventofcode_rs::aoc2025_day04::AdventOfCode2025Day04;
use adventofcode_rs::aoc2025_day05::AdventOfCode2025Day05;
use adventofcode_rs::Runner;
use std::env;

fn main() {
    println!("Advent of Code 2025");

    let mut runners = get_runners();
    let args: Vec<String> = env::args().collect();
    let to_run = parse_args(&args, &runners);

    for index in to_run {
        let runner = &mut runners[index];
        let (_, day) = runner.name();
        if let Err(e) = runner.parse() {
            eprintln!("    Error parsing Day {:02} input: {}", day, e);
        } else {
            runner.run();
        }
    }
}

fn get_runners() -> Vec<Box<dyn Runner>> {
    vec![
        Box::new(AdventOfCode2025Day01::new()),
        Box::new(AdventOfCode2025Day02::new()),
        Box::new(AdventOfCode2025Day03::new()),
        Box::new(AdventOfCode2025Day04::new()),
        Box::new(AdventOfCode2025Day05::new()),
    ]
}

fn parse_args(args: &[String], runners: &[Box<dyn Runner>]) -> Vec<usize> {
    let mut to_run: Vec<usize> = Vec::new();

    if args.len() > 1 {
        let arg = &args[1];
        if arg == "all" {
            for i in 0..runners.len() {
                to_run.push(i);
            }
        } else if let Some(days_str) = arg.strip_prefix("day=") {
            let target_days: Vec<u32> = days_str
                .split(',')
                .filter_map(|s| s.parse::<u32>().ok())
                .collect();

            for day in target_days {
                for (i, runner) in runners.iter().enumerate() {
                    let (_, d) = runner.name();
                    if d == day {
                        to_run.push(i);
                    }
                }
            }
        }
    } else if !runners.is_empty() {
        to_run.push(runners.len() - 1);
    }

    to_run
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockRunner {
        day: u32,
    }

    impl Runner for MockRunner {
        fn name(&self) -> (u32, u32) {
            (2025, self.day)
        }
        fn parse(&mut self) -> Result<(), String> {
            Ok(())
        }
        fn part01(&self) -> String {
            "".to_string()
        }
        fn part02(&self) -> String {
            "".to_string()
        }
    }

    fn get_mock_runners() -> Vec<Box<dyn Runner>> {
        vec![
            Box::new(MockRunner { day: 1 }),
            Box::new(MockRunner { day: 2 }),
            Box::new(MockRunner { day: 3 }),
        ]
    }

    #[test]
    fn test_get_runners() {
        let runners = get_runners();
        assert!(!runners.is_empty());
        // Verify names are somewhat expected (e.g., day 1 is there)
        let (_, day) = runners[0].name();
        assert_eq!(day, 1);
    }

    #[test]
    fn test_parse_args_default() {
        let runners = get_mock_runners();
        let args = vec!["binary_name".to_string()];
        let to_run = parse_args(&args, &runners);
        assert_eq!(to_run, vec![2]); // Last runner index
    }

    #[test]
    fn test_parse_args_all() {
        let runners = get_mock_runners();
        let args = vec!["binary_name".to_string(), "all".to_string()];
        let to_run = parse_args(&args, &runners);
        assert_eq!(to_run, vec![0, 1, 2]);
    }

    #[test]
    fn test_parse_args_day_single() {
        let runners = get_mock_runners();
        let args = vec!["binary_name".to_string(), "day=2".to_string()];
        let to_run = parse_args(&args, &runners);
        assert_eq!(to_run, vec![1]);
    }

    #[test]
    fn test_parse_args_day_multiple() {
        let runners = get_mock_runners();
        let args = vec!["binary_name".to_string(), "day=1,3".to_string()];
        let to_run = parse_args(&args, &runners);
        assert_eq!(to_run, vec![0, 2]);
    }

    #[test]
    fn test_parse_args_day_invalid() {
        let runners = get_mock_runners();
        let args = vec!["binary_name".to_string(), "day=invalid,4".to_string()];
        let to_run = parse_args(&args, &runners);
        assert!(to_run.is_empty());
    }

    #[test]
    fn test_parse_args_empty_runners() {
        let runners: Vec<Box<dyn Runner>> = Vec::new();
        let args = vec!["binary_name".to_string()];
        let to_run = parse_args(&args, &runners);
        assert!(to_run.is_empty());
    }
}
