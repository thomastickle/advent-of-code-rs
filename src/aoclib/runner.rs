use std::str::FromStr;

pub trait Runner: FromStr {
    type Output;

    /// Get the expected input path for input file of a particular Runner.
    fn input_path(&self) -> String {
        let (_, day) = self.name();
        format!("input/day{:02}.input", day)
    }

    fn name(&self) -> (u32, u32);
    fn new() -> Self;
    fn parse(&self, input: &str) -> Self;
    fn part01(&self) -> Self::Output;
    fn part02(&self) -> Self::Output;
}
