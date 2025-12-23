pub mod aoc2025_day01;
pub mod aoc2025_day02;

pub trait Runner {
    fn name(&self) -> (u32, u32);
    fn parse(&mut self) -> Result<(), String>;
    fn part01(&self) -> String;
    fn part02(&self) -> String;

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
