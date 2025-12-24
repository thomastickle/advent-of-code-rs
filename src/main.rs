use adventofcode_rs::aoc2025_day01::AdventOfCode2025Day01;
use adventofcode_rs::aoc2025_day02::AdventOfCode2025Day02;
use adventofcode_rs::aoc2025_day03::AdventOfCode2025Day03;
use adventofcode_rs::aoc2025_day04::AdventOfCode2025Day04;
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
