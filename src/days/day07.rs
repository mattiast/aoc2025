use crate::Solution;
use std::collections::HashSet;

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
    fn part1(&self, _input: &str) -> String {
        "Part 1 TODO".to_string()
    }

    fn part2(&self, _input: &str) -> String {
        "Part 2 TODO".to_string()
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

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
}
