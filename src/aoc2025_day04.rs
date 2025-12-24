use crate::Runner;
use std::collections::HashSet;

#[allow(dead_code)]
struct PaperRoll {
    x: u32,
    y: u32,
}

impl PaperRoll {}

#[derive(Debug, Default)]
pub struct AdventOfCode2025Day04 {
    #[allow(dead_code)]
    rolls: HashSet<(i32, i32)>,
}

impl AdventOfCode2025Day04 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for AdventOfCode2025Day04 {
    fn name(&self) -> (u32, u32) {
        (25, 4)
    }

    fn parse(&mut self) -> Result<(), String> {
        let _lines: Vec<String> = crate::read_lines("input/day04.input");
        Ok(())
    }

    fn part01(&self) -> String {
        "todo".to_string()
    }
    fn part02(&self) -> String {
        "todo".to_string()
    }

    fn run(&self) {
        let (_, day) = self.name();
        println!("    Day {:02}", day);
        println!("        Part 01: {}", self.part01());
        println!("        Part 02: {}", self.part02());
    }
}

#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    const TEST_INPUT: &str = "..@@.@@@@.\n\
        @@@.@.@.@@\n\
        @@@@@.@.@@\n\
        @.@@@@..@.\n\
        @@.@@@@.@@\n\
        .@@@@@@@.@\n\
        .@.@.@.@@@\n\
        @.@@@.@@@@\n\
        .@@@@@@@@.\n\
        @.@.@@@.@.\n";
}
