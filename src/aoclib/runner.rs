use std::str::FromStr;


pub trait Runner: FromStr {
    type Output: std::fmt::Display;

    /// Get the expected input path for the input file of a particular Runner.
    fn input_path(&self) -> String {
        let (_, day) = self.name();
        format!("input/day{:02}.input", day)
    }

    fn name(&self) -> (u32, u32);
    fn parse(&self, input: &str) -> Self;
    fn part01(&self) -> Self::Output;
    fn part02(&self) -> Self::Output;

    fn run(&self) -> String {
        let start_time = std::time::Instant::now();
        let input_path = self.input_path();
        let input = std::fs::read_to_string(&input_path)
            .unwrap_or_else(|e| panic!("Failed to read input file {}: {}", input_path, e));
        let parsed = self.parse(&input);
        let parse_duration = start_time.elapsed();

        let part1_result = parsed.part01();
        let part1_duration = start_time.elapsed() - parse_duration;
        let part2_result = parsed.part02();
        let part2_duration = start_time.elapsed() - parse_duration - part1_duration;

        format!(
            "Day {:02} Results:\n\tParse: [{:?}]\n\tPart 01: {} [{:?}]\n\tPart 02: {} [{:?}]",
            self.name().1,
            parse_duration,
            part1_result,
            part1_duration,
            part2_result,
            part2_duration
        )
    }
}

pub trait AocDay {
    fn run_day(&self) -> String;
}

impl <T:Runner> AocDay for T {
    fn run_day(&self) -> String {
        self.run()
    }

}
