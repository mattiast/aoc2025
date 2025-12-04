use crate::Solution;
use nom::{
    IResult, Parser,
    branch::alt,
    character::complete::{char, newline},
    combinator::value,
    multi::{many1, separated_list1},
    sequence::terminated,
};

pub struct Day04;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    PaperRoll,
}

impl Cell {
    pub fn is_paper_roll(&self) -> bool {
        matches!(self, Cell::PaperRoll)
    }
}

pub type Grid = Vec<Vec<Cell>>;

fn parse_cell(input: &str) -> IResult<&str, Cell> {
    alt((
        value(Cell::Empty, char('.')),
        value(Cell::PaperRoll, char('@')),
    ))
    .parse(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<Cell>> {
    many1(parse_cell).parse(input)
}

fn parse_input(input: &str) -> IResult<&str, Grid> {
    separated_list1(newline, parse_line).parse(input)
}

fn parse_input_complete(input: &str) -> IResult<&str, Grid> {
    terminated(parse_input, many1(newline)).parse(input)
}

impl Solution for Day04 {
    fn part1(&self, input: &str) -> String {
        let (_, grid) = parse_input_complete(input)
            .or_else(|_| parse_input(input))
            .expect("Failed to parse input");

        let rows = grid.len();
        let cols = grid.first().map(|r| r.len()).unwrap_or(0);

        let mut num_reachable_paper_rolls = 0;
        for i in 0..rows {
            for j in 0..cols {
                let num_neighbor_rolls = neighbors(&grid, i, j)
                    .iter()
                    .filter(|&&(nr, nc)| grid[nr][nc].is_paper_roll())
                    .count();
                if num_neighbor_rolls < 4 && grid[i][j].is_paper_roll() {
                    num_reachable_paper_rolls += 1;
                }
            }

        }

        format!("Parsed grid: {} rows x {} cols. Reachable: {}", rows, cols, num_reachable_paper_rolls)
    }

    fn part2(&self, input: &str) -> String {
        let (_, _grid) = parse_input_complete(input)
            .or_else(|_| parse_input(input))
            .expect("Failed to parse input");
        "Part 2 TODO".to_string()
    }
}

fn neighbors(grid: &Grid, row: usize, col: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    let rows = grid.len() as isize;
    let cols = grid.first().map(|r| r.len() as isize).unwrap_or(0);

    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (dr, dc) in directions.iter() {
        let new_row = row as isize + dr;
        let new_col = col as isize + dc;

        if new_row >= 0 && new_row < rows && new_col >= 0 && new_col < cols {
            result.push((new_row as usize, new_col as usize));
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

    #[test]
    fn test_part1_sample() {
        let output = Day04.part1(SAMPLE_INPUT);
        assert!(
            output.contains("13"),
            "Expected output to contain '13', got: {}",
            output
        );
    }

    #[test]
    fn test_parse_cell() {
        assert_eq!(parse_cell("."), Ok(("", Cell::Empty)));
        assert_eq!(parse_cell("@"), Ok(("", Cell::PaperRoll)));
    }

    #[test]
    fn test_parse_line() {
        let input = "..@@.@@@@.";
        let (_, cells) = parse_line(input).unwrap();
        assert_eq!(cells.len(), 10);
        assert_eq!(cells[0], Cell::Empty);
        assert_eq!(cells[1], Cell::Empty);
        assert_eq!(cells[2], Cell::PaperRoll);
        assert_eq!(cells[3], Cell::PaperRoll);
        assert_eq!(cells[4], Cell::Empty);
    }

    #[test]
    fn test_parse_input() {
        let (_, grid) = parse_input_complete(SAMPLE_INPUT).unwrap();
        assert_eq!(grid.len(), 10, "Expected 10 rows");
        assert_eq!(grid[0].len(), 10, "Expected 10 columns");

        // Check first row: ..@@.@@@@.
        assert_eq!(grid[0][0], Cell::Empty);
        assert_eq!(grid[0][1], Cell::Empty);
        assert_eq!(grid[0][2], Cell::PaperRoll);
        assert_eq!(grid[0][3], Cell::PaperRoll);

        // Check second row: @@@.@.@.@@
        assert_eq!(grid[1][0], Cell::PaperRoll);
        assert_eq!(grid[1][3], Cell::Empty);
    }

    #[test]
    fn test_count_paper_rolls() {
        let (_, grid) = parse_input_complete(SAMPLE_INPUT).unwrap();
        let paper_rolls = grid
            .iter()
            .flatten()
            .filter(|&&cell| cell == Cell::PaperRoll)
            .count();

        // Count '@' in sample: 71 paper rolls
        assert_eq!(paper_rolls, 71);
    }

    #[test]
    fn test_grid_dimensions() {
        let (_, grid) = parse_input_complete(SAMPLE_INPUT).unwrap();
        assert_eq!(grid.len(), 10);
        for row in &grid {
            assert_eq!(row.len(), 10, "All rows should have 10 columns");
        }
    }
}
