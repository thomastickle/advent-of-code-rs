pub mod aoc2025_day01;
pub mod aoc2025_day02;
pub mod aoc2025_day03;
pub mod aoc2025_day04;
pub mod aoc2025_day05;

pub trait Runner {
    fn name(&self) -> (u32, u32);
    fn parse(&mut self) -> Result<(), String>;
    fn part01(&self) -> String;
    fn part02(&self) -> String;

    fn input_path(&self) -> String {
        let (_, day) = self.name();
        format!("input/day{:02}.input", day)
    }

    fn run(&self) {
        let (_, day) = self.name();
        println!("    Day {:02}", day);

        let now = std::time::Instant::now();
        let p1 = self.part01();
        let d1 = now.elapsed();
        println!("        Part 01: {} ({:?})", p1, d1);

        let now = std::time::Instant::now();
        let p2 = self.part02();
        let d2 = now.elapsed();
        println!("        Part 02: {} ({:?})", p2, d2);
    }
}

pub fn read_lines(filename: &str) -> Vec<String> {
    std::fs::read_to_string(filename)
        .expect("Error reading file")
        .lines()
        .map(String::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_read_lines() {
        let mut path = std::env::temp_dir();
        path.push("test_input.txt");
        let content = "line1\nline2\nline3";
        let mut file = std::fs::File::create(&path).unwrap();
        file.write_all(content.as_bytes()).unwrap();

        let lines = read_lines(path.to_str().unwrap());
        assert_eq!(lines, vec!["line1", "line2", "line3"]);

        std::fs::remove_file(path).unwrap();
    }
}
