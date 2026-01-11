use crate::aoclib::runner::Runner;
use std::str::FromStr;

// #[derive(Debug)]
// struct Beam {
//     start: (u32, u32),
//     end: Option<(u32, u32)>,
// }

#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct AdventOfCode2025Day07 {
    start: (usize, usize),
    grid_height: usize,
    grid_width: usize,
    splitters: Vec<(usize, usize)>,
}

impl AdventOfCode2025Day07 {
    
    /// Finds the number of times a beam splits along with the number
    /// of different timelines thus solving both part 1 and part 2 at the same time.
    /// We exploit a count tracking map to both keep it fast and keep track of timelines.
    fn splits_timelines(&self) -> (u64, u64) {
        let mut split_count = 0;
        let mut beams = vec![0; self.grid_width];
        beams[self.start.0] = 1;
        

        for splitter in &self.splitters {
            let count = beams[splitter.0];
            if count > 0 {
                split_count += 1;
                beams[splitter.0] = 0;
                beams[splitter.0 - 1] += count;
                beams[splitter.0 + 1] += count;
            }
        }
        let count = beams.iter().sum();

        (split_count, count)
    }
}

impl FromStr for AdventOfCode2025Day07 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = (0, 0);
        let mut splitters: Vec<(usize, usize)> = vec![];
        let mut grid_height = 0;
        let mut grid_width = 0;
        for (y, line) in s.lines().enumerate() {
            grid_height += 1;
            for (x, c) in line.chars().enumerate() {
                match c {
                    'S' => start = (x, y),
                    '^' => splitters.push((x, y)),
                    _ => {}
                }
                grid_width = x + 1;
            }
        }

        Ok(Self {
            start,
            grid_height,
            grid_width,
            splitters,
        })
    }
}

impl Runner for AdventOfCode2025Day07 {
    type Output = u64;

    fn name(&self) -> (u32, u32) {
        (2025, 7)
    }

    /// Get the number of times the tachyon beam splits on the way to the end
    /// of the tachyon manifold.
    ///
    /// Fast implementation using bitmap to keep track of beam locations.
    fn part01(&self) -> Self::Output {
        self.splits_timelines().0
    }

    fn part02(&self) -> Self::Output {
        self.splits_timelines().1
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc2025::day07::AdventOfCode2025Day07;
    use crate::aoclib::runner::Runner;

    const TEST_INPUT: &str = include_str!("../../input/test/day07.input");

    #[test]
    fn test_name() {
        let day07 = AdventOfCode2025Day07::default();
        assert_eq!(day07.name(), (2025, 7));
    }

    #[test]
    fn test_from_str() {
        let day07 = TEST_INPUT.parse::<AdventOfCode2025Day07>().unwrap();
        assert_eq!(day07.start, (7, 0));
        assert_eq!(day07.splitters.len(), 22);
    }

    #[test]
    fn test_part01() {
        let day07 = TEST_INPUT.parse::<AdventOfCode2025Day07>().unwrap();
        assert_eq!(day07.part01(), 21);
    }

    #[test]
    fn test_part02() {
        let day07 = TEST_INPUT.parse::<AdventOfCode2025Day07>().unwrap();
        assert_eq!(day07.part02(), 40);
    }
}
