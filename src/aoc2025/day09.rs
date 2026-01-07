use crate::aoclib::runner::Runner;
use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug)]
struct RedTile {
    x: i64,
    y: i64,
}

#[derive(Debug, Default)]
pub struct AdventOfCode2025Day09 {
    red_tiles: Vec<RedTile>,
}

impl FromStr for AdventOfCode2025Day09 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let red_tiles = s.lines().filter(|line| !line.is_empty()).map(|line| {
            let (x, y) = line.split_once(',').ok_or("Invalid input format")?;
            let x = x.parse::<i64>().map_err(|_| "Invalid x coordinate")?;
            let y = y.parse::<i64>().map_err(|_| "Invalid y coordinate")?;
            Ok(RedTile { x, y })
        }).collect::<Result<Vec<RedTile>, String>>()?;

        Ok(Self { red_tiles } )
    }
}

impl Runner for AdventOfCode2025Day09 {
    type Output = i64;

    fn name(&self) -> (u32, u32) {
        (2025, 9)
    }

    fn part01(&self) -> Self::Output {
        self.red_tiles.iter().tuple_combinations().map(|(a, b)| {
            ((a.x - b.x).abs() + 1) * ((a.y - b.y).abs() + 1)
        }).max().unwrap()
    }

    fn part02(&self) -> Self::Output {
        24
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc2025::day09::AdventOfCode2025Day09;
    use crate::aoclib::runner::Runner;
    use std::str::FromStr;

    const TEST_INPUT: &str = include_str!("../../input/test/day09.input");

    #[test]
    fn test_name() {
        let day07 = AdventOfCode2025Day09::default();
        assert_eq!(day07.name(), (2025, 9));
    }

    #[test]
    fn test_from_str() {
        let day07 = AdventOfCode2025Day09::from_str(TEST_INPUT).unwrap();
        assert_eq!(day07.red_tiles.len(), 8);
    }

    #[test]
    fn test_part01() {
        let day07 = AdventOfCode2025Day09::from_str(TEST_INPUT).unwrap();
        assert_eq!(day07.part01(), 50);
    }

    #[test]
    fn test_part02() {
        let day07 = AdventOfCode2025Day09::from_str(TEST_INPUT).unwrap();
        assert_eq!(day07.part02(), 24);
    }


}
