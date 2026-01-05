use crate::aoclib::runner::Runner;
use std::cmp::PartialEq;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Operation {
    Add,
    Multiply,
    NoOp,
}

#[derive(Debug)]
struct Problem {
    values: Vec<Vec<u8>>,
    operation: Operation,
}

impl Problem {
    /// Calculates the result for Part 1 by reading values horizontally.
    ///
    /// For each row in the problem's grid, it parses the sequence of digits into a single number
    /// (ignoring padding). It then applies the problem's [`Operation`] to all resulting numbers.
    pub fn solve(&self) -> i64 {
        let parsed_numbers: Vec<i64> = self
            .values
            .iter()
            .map(|digits| self.parse_digits_ignoring_padding(digits))
            .collect();

        match self.operation {
            Operation::Add => parsed_numbers.iter().sum(),
            Operation::Multiply => parsed_numbers.iter().product(),
            _ => 0
        }
    }

    /// Calculates the result for Part 2 by reading values vertically (transposed).
    ///
    /// This treats each column of the grid as a sequence of digits forming a single number.
    /// After parsing one number per column (ignoring vertical padding), it applies the
    /// problem's [`Operation`] to the results.
    pub fn solve_transposed(&self) -> i64 {
        if self.values.is_empty() { return 0; }

        let num_columns = self.values[0].len();
        let mut parsed_numbers = Vec::new();

        for col in 0..num_columns {
            // Collect digits from this column across all rows
            let column_digits: Vec<u8> = self.values.iter()
                                             .map(|row| row[col])
                                             .collect();

            parsed_numbers.push(self.parse_digits_ignoring_padding(&column_digits));
        }

        match self.operation {
            Operation::Add => parsed_numbers.iter().sum(),
            Operation::Multiply => parsed_numbers.iter().product(),
            _ => 0
        }
    }

    /// Converts a slice of digits into a single integer, ignoring leading and trailing zeros.
    ///
    /// Padding zeros (0) are treated as null space. This function finds the first and last
    /// non-zero digits and collapses everything in between into a base-10 number.
    ///
    /// # Example
    /// `[0, 0, 3, 8, 7, 0]` becomes `387`.
    fn parse_digits_ignoring_padding(&self, digits: &[u8]) -> i64 {
        let first = digits.iter().position(|&d| d != 0);
        let last = digits.iter().rposition(|&d| d != 0);

        match (first, last) {
            (Some(start), Some(end)) => digits[start..=end]
                .iter()
                .fold(0i64, |acc, &d| acc * 10 + d as i64),
            _ => 0,
        }
    }
}

#[derive(Debug, Default)]
pub struct AdventOfCode2025Day06 {
    problems: Vec<Problem>,
}

impl FromStr for AdventOfCode2025Day06 {
    type Err = ();

    /// Parses the input string into a series of `Problem` structs.
    ///
    /// The input is expected to consist of a multi-line grid of digits, followed by a final line
    /// containing operators (`+` or `*`). The position of each operator on the last line
    /// determines the horizontal "slice" of the grid that belongs to that specific problem.
    ///
    /// # Example Input format:
    /// ```text
    /// 123 456
    /// 789 012
    /// +   *
    /// ```
    /// This would create two problems:
    /// 1. An 'Add' problem using the first 4 columns.
    /// 2. A 'Multiply' problem using columns 5 through the end.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        let last_line = lines.last().unwrap();
        let mut problems: Vec<Problem> = vec![];
        let mut start = 0;
        let mut seen_start = false;
        let mut previous_op = Operation::NoOp;
        for (idx, c) in last_line.chars().enumerate() {
            match c {
                '+' | '*' => {
                    if !seen_start {
                        seen_start = true;
                        start = idx;
                        previous_op = if c == '+' {
                            Operation::Add
                        } else {
                            Operation::Multiply
                        };
                        continue;
                    }

                    let problem_elements: Vec<Vec<u8>> = lines[0..lines.len() - 1]
                        .iter()
                        .map(|line| {
                            line[start..idx - 1]
                                .chars()
                                .map(|c| {
                                    if c.is_ascii_digit() {
                                        c.to_digit(10).unwrap() as u8
                                    } else {
                                        0
                                    }
                                })
                                .collect()
                        })
                        .collect();

                    let problem = Problem {
                        values: problem_elements,
                        operation: previous_op,
                    };
                    problems.push(problem);

                    start = idx;
                    previous_op = if c == '+' {
                        Operation::Add
                    } else {
                        Operation::Multiply
                    };
                }
                _ => continue,
            }
        }

        let problem_elements: Vec<Vec<u8>> = lines[0..lines.len() - 1]
            .iter()
            .map(|line| {
                line[start..]
                    .chars()
                    .map(|c| {
                        if c.is_ascii_digit() {
                            c.to_digit(10).unwrap() as u8
                        } else {
                            0
                        }
                    })
                    .collect()
            })
            .collect();

        let problem = Problem {
            values: problem_elements,
            operation: previous_op,
        };
        problems.push(problem);

        Ok(Self { problems })
    }
}

impl Runner for AdventOfCode2025Day06 {
    type Output = i64;

    fn name(&self) -> (u32, u32) {
        (2025, 6)
    }

    fn part01(&self) -> Self::Output {
        self.problems.iter().map(|p| p.solve()).sum()
    }

    fn part02(&self) -> Self::Output {
        self.problems.iter().map(|p| p.solve_transposed()).sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc2025::day06::AdventOfCode2025Day06;
    use crate::aoclib::runner::Runner;
    use std::str::FromStr;

    const TEST_INPUT: &str = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";

    #[test]
    fn test_name() {
        assert_eq!((2025, 06), AdventOfCode2025Day06::default().name())
    }

    #[test]
    fn test_from_str() {
        let day06 = AdventOfCode2025Day06::from_str(TEST_INPUT).unwrap();
        assert_eq!(4, day06.problems.len())
    }

    #[test]
    fn test_part01() {
        let day06 = AdventOfCode2025Day06::from_str(TEST_INPUT).unwrap();
        assert_eq!(4277556, day06.part01())
    }

    #[test]
    fn test_part02() {
        let day06 = AdventOfCode2025Day06::from_str(TEST_INPUT).unwrap();
        assert_eq!(3263827, day06.part02())
    }
}
