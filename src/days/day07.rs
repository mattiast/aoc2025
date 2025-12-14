use crate::Solution;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub start: Point,
    pub splitters: HashSet<Point>,
}

fn parse_input(input: &str) -> Grid {
    let lines: Vec<&str> = input.lines().filter(|line| !line.is_empty()).collect();
    let height = lines.len();
    let width = lines.first().map(|l| l.len()).unwrap_or(0);

    let mut start = Point { row: 0, col: 0 };
    let mut splitters = HashSet::new();

    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            match ch {
                'S' => start = Point { row, col },
                '^' => {
                    splitters.insert(Point { row, col });
                }
                _ => {}
            }
        }
    }

    Grid {
        width,
        height,
        start,
        splitters,
    }
}

pub struct Day07;

impl Solution for Day07 {
    fn part1(&self, input: &str) -> anyhow::Result<String> {
        let grid = parse_input(input);

        let mut row = grid.start.row;
        let mut colset: HashSet<usize> = [grid.start.col].iter().cloned().collect();

        let mut num_splittings = 0;
        while row < grid.height - 1 {
            row += 1;
            let mut new_colset = HashSet::new();
            for &col in &colset {
                if grid.splitters.contains(&Point { row, col }) {
                    num_splittings += 1;
                    new_colset.insert(col - 1);
                    new_colset.insert(col + 1);
                } else {
                    new_colset.insert(col);
                }
            }
            colset = new_colset;
        }

        Ok(format!(
            "Grid size: {}x{}, Start: ({}, {}), Splitters: {} => Splittings: {}",
            grid.width,
            grid.height,
            grid.start.row,
            grid.start.col,
            grid.splitters.len(),
            num_splittings
        ))
    }

    fn part2(&self, input: &str) -> anyhow::Result<String> {
        let grid = parse_input(input);

        let mut row = grid.start.row;
        let mut colset: HashMap<usize, u64> = [(grid.start.col, 1)].iter().cloned().collect();

        while row < grid.height - 1 {
            row += 1;
            let mut new_colset = HashMap::new();
            for (&col, &n) in &colset {
                if grid.splitters.contains(&Point { row, col }) {
                    *new_colset.entry(col - 1).or_insert(0) += n;
                    *new_colset.entry(col + 1).or_insert(0) += n;
                } else {
                    *new_colset.entry(col).or_insert(0) += n;
                }
            }
            colset = new_colset;
        }

        Ok(format!(
            "Grid size: {}x{}, Start: ({}, {}), Timelines: {}",
            grid.width,
            grid.height,
            grid.start.row,
            grid.start.col,
            colset.values().sum::<u64>(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#".......S.......
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
...............
"#;
    #[test]
    fn test_parse_input() {
        let grid = parse_input(TEST_INPUT);

        assert_eq!(grid.width, 15);
        assert_eq!(grid.height, 16);
        assert_eq!(grid.start, Point { row: 0, col: 7 });
        assert_eq!(grid.splitters.len(), 22);

        // Check a few specific splitter positions
        assert!(grid.splitters.contains(&Point { row: 2, col: 7 }));
        assert!(grid.splitters.contains(&Point { row: 4, col: 6 }));
        assert!(grid.splitters.contains(&Point { row: 4, col: 8 }));
    }

    #[test]
    fn test_part1() {
        let solution = Day07;
        let result = solution.part1(TEST_INPUT).unwrap();
        assert!(result.contains("Splittings: 21"));
    }

    #[test]
    fn test_part2() {
        let solution = Day07;
        let result = solution.part2(TEST_INPUT).unwrap();
        assert!(result.contains("Timelines: 40"));
    }
}
