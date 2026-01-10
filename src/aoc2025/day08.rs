use crate::aoclib::runner::Runner;
use std::collections::HashSet;

use disjoint_sets::UnionFind;
use glam::I64Vec3;
use itertools::Itertools;
use std::str::FromStr;

type JunctionBox = I64Vec3;

#[derive(Debug, Default)]
pub struct AdventOfCode2025Day08 {
    junction_boxes: Vec<JunctionBox>,
}

impl FromStr for AdventOfCode2025Day08 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let junction_boxes = s
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.split(',')
                    .map(|coord| {
                        coord
                            .trim()
                            .parse::<i64>()
                            .map_err(|e| format!("Failed to parse '{}': {}", coord, e))
                    })
                    .collect_tuple::<(
                        Result<i64, String>,
                        Result<i64, String>,
                        Result<i64, String>,
                    )>()
                    .map(|(x, y, z)| Ok(JunctionBox::new(  x?, y?, z?)))
                    .unwrap_or_else(|| Err(format!("Line '{}' does not have 3 coordinates", line)))
            })
            .collect::<Result<Vec<JunctionBox>, String>>()?;
        Ok(AdventOfCode2025Day08 { junction_boxes })
    }
}

impl AdventOfCode2025Day08 {

    fn connect_all(&self) -> (usize, usize) {
        let pairs = asc_pair_distances(&self.junction_boxes);
        let total_boxes = self.junction_boxes.len();
        let mut uf = UnionFind::new(total_boxes);
        let mut num_components = total_boxes;
        let mut last_joined = (0, 0);

        for &(_, idx_a, idx_b) in pairs.iter() {
            if uf.find(idx_a) != uf.find(idx_b) {
                uf.union(idx_a, idx_b);
                last_joined = (idx_a, idx_b);
                num_components -= 1;

                if num_components == 1 {
                    return last_joined;
                }
            }
        }
        last_joined
    }

    fn group_circuits(&self, n: usize) -> Vec<HashSet<usize>> {
        let pairs = asc_pair_distances(&self.junction_boxes);
        let mut uf = UnionFind::new(self.junction_boxes.len());

        for &(_, idx_a, idx_b) in pairs.iter().take(n) {
            uf.union(idx_a, idx_b);
        }

        // Grouping indices by their representative (root) in the Union-Find structure
        let mut groups: std::collections::HashMap<usize, HashSet<usize>> = std::collections::HashMap::new();
        for i in 0..self.junction_boxes.len() {
            groups.entry(uf.find(i)).or_default().insert(i);
        }

        groups.into_values().filter(|g| g.len() > 1).collect()
    }
}

impl Runner for AdventOfCode2025Day08 {
    type Output = u64;

    fn name(&self) -> (u32, u32) {
        (2025, 8)
    }

    fn part01(&self) -> Self::Output {
        let amount = if self.junction_boxes.len() > 20usize { 1000usize } else { 10usize };
        let circuits = self.group_circuits(amount);

        circuits
            .iter()
            .map(|c| c.len() as u64)
            .sorted_by(|a, b| b.cmp(a)) // Sort descending
            .take(3)
            .product()
    }

    fn part02(&self) -> Self::Output {
        let (a, b) = self.connect_all();

        (self.junction_boxes[a].x * self.junction_boxes[b].x) as u64
    }
}

fn distance(a: &JunctionBox, b: &JunctionBox) -> i64 {
    (a - b).length_squared()
}


/// Calculate all pairwise distances between junction boxes
///
fn asc_pair_distances(junction_boxes: &Vec<JunctionBox>) -> Vec<(i64, usize, usize)> {
    junction_boxes
        .iter()
        .enumerate()
        .tuple_combinations()
        .map(|((idx_a, a), (idx_b, b))| {
            let distance = distance(a, b);
            (distance, idx_a, idx_b)
        })
        .sorted_by_key(|&(distance, _, _)| distance)
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use crate::aoc2025::day08::AdventOfCode2025Day08;
    use crate::aoclib::runner::Runner;
    use std::str::FromStr;

    const TEST_INPUT: &str = include_str!("../../input/test/day08.input");

    #[test]
    fn test_name() {
        let day08 = AdventOfCode2025Day08::default();
        assert_eq!(day08.name(), (2025, 8));
    }

    #[test]
    fn test_from_str() {
        let day08 = AdventOfCode2025Day08::from_str(TEST_INPUT).unwrap();
        assert_eq!(day08.junction_boxes.len(), 20);
    }

    #[test]
    fn test_part01() {
        let day08 = AdventOfCode2025Day08::from_str(TEST_INPUT).unwrap();
        assert_eq!(day08.part01(), 40);
    }

    #[test]
    fn test_part02() {
        let day08 = AdventOfCode2025Day08::from_str(TEST_INPUT).unwrap();
        assert_eq!(day08.part02(), 25272);
    }
}
