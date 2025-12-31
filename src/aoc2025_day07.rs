use crate::Runner;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Beam {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct AdventOfCode2025Day07 {
    start: (usize, usize),
    splitters: Vec<(usize, usize)>,
}

impl FromStr for AdventOfCode2025Day07 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = None;
        let mut splitters = Vec::new();

        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    'S' => start = Some((x, y)),
                    '^' => splitters.push((x, y)),
                    _ => {}
                }
            }
        }

        let start = start.ok_or("No start element 'S' found")?;

        println!("Start: {:?}", start);
        println!("Splitter: {:?}", splitters);

        Ok(AdventOfCode2025Day07 { start, splitters })
    }
}

impl Runner for AdventOfCode2025Day07 {
    fn name(&self) -> (u32, u32) {
        (2025, 7)
    }

    fn parse(&mut self) -> Result<(), String> {
        todo!()
    }

    fn part01(&self) -> String {
        let mut splits = 0;
        let mut queue = vec![Beam { x: self.start.0, y: self.start.1 }];
        let mut seen_beams = HashSet::new();
        let mut hit_splitters = HashSet::new();

        while let Some(beam) = queue.pop() {
            if !seen_beams.insert(beam.clone()) {
                continue;
            }

            let hit = self.splitters.iter()
                .filter(|(sx, sy)| *sx == beam.x && *sy > beam.y)
                .min_by_key(|(_, sy)| sy);

            if let Some(&splitter) = hit {
                if hit_splitters.insert(splitter) {
                    splits += 1;
                    queue.push(Beam { x: splitter.0.saturating_sub(1), y: splitter.1 });
                    queue.push(Beam { x: splitter.0 + 1, y: splitter.1 });
                }
            }
        }

        format!("{}", splits)
    }

    fn part02(&self) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc2025_day07::AdventOfCode2025Day07;
    use crate::Runner;
    use std::str::FromStr;

    static TEST_INPUT: &str = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#;

    #[test]
    fn test_day07() {
        let _day07 = AdventOfCode2025Day07::from_str(TEST_INPUT).unwrap();
        //assert_eq!(day07.part01(), "10");
    }

    #[test]
    fn test_day07_part01() {
        let day07 = AdventOfCode2025Day07::from_str(TEST_INPUT).unwrap();
        assert_eq!(day07.part01(), "21");
    }
}
