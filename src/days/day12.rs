use crate::Solution;
use nom::{
    IResult, Parser,
    character::complete::{char, line_ending, space1, u8 as nom_u8, u32 as nom_u32},
    combinator::map,
    multi::{count, separated_list1},
    sequence::{separated_pair, terminated},
};

pub struct Day12;

#[derive(Debug, Clone)]
struct Figure {
    id: u8,
    pattern: [[bool; 3]; 3], // true = '#', false = '.'
}

#[derive(Debug, Clone)]
struct DimensionEntry {
    width: u32,
    height: u32,
    numbers: [u32; 6],
}

#[derive(Debug)]
struct Input {
    figures: [Figure; 6],
    entries: Vec<DimensionEntry>,
}

// Parse a single line of a 3x3 pattern
fn parse_pattern_line(input: &str) -> IResult<&str, [bool; 3]> {
    use nom::branch::alt;
    let parse_cell = alt((map(char('#'), |_| true), map(char('.'), |_| false)));

    let (input, chars) = count(parse_cell, 3).parse(input)?;
    Ok((input, [chars[0], chars[1], chars[2]]))
}

// Parse a complete 3x3 figure
fn parse_figure(input: &str) -> IResult<&str, Figure> {
    let (input, id) = nom_u8.parse(input)?;
    let (input, _) = (char(':'), line_ending).parse(input)?;
    let (input, line1) = terminated(parse_pattern_line, line_ending).parse(input)?;
    let (input, line2) = terminated(parse_pattern_line, line_ending).parse(input)?;
    let (input, line3) = terminated(parse_pattern_line, line_ending).parse(input)?;
    // Skip blank line after figure
    let (input, _) = line_ending.parse(input)?;

    Ok((
        input,
        Figure {
            id,
            pattern: [line1, line2, line3],
        },
    ))
}

// Parse a dimension entry like "39x43: 23 41 27 30 29 31"
fn parse_dimension_entry(input: &str) -> IResult<&str, DimensionEntry> {
    let (input, (width, height)) = separated_pair(nom_u32, char('x'), nom_u32).parse(input)?;
    let (input, _) = (char(':'), space1).parse(input)?;
    let (input, numbers_vec) = separated_list1(space1, nom_u32).parse(input)?;

    let numbers: [u32; 6] = numbers_vec.try_into().map_err(|_| {
        nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Count))
    })?;

    Ok((
        input,
        DimensionEntry {
            width,
            height,
            numbers,
        },
    ))
}

// Parse all figures (0-5)
fn parse_figures(input: &str) -> IResult<&str, [Figure; 6]> {
    let (input, (f1, f2, f3, f4, f5, f6)) = (
        parse_figure,
        parse_figure,
        parse_figure,
        parse_figure,
        parse_figure,
        parse_figure,
    )
        .parse(input)?;
    Ok((input, [f1, f2, f3, f4, f5, f6]))
}

// Parse all dimension entries
fn parse_entries(input: &str) -> IResult<&str, Vec<DimensionEntry>> {
    separated_list1(line_ending, parse_dimension_entry).parse(input)
}

// Parse the complete input
fn parse_input(input: &str) -> Result<Input, String> {
    let (input, figures) =
        parse_figures(input).map_err(|e| format!("Failed to parse figures: {:?}", e))?;

    // No need for extra line_ending here since figures already consume the blank line after them

    let (_input, entries) =
        parse_entries(input).map_err(|e| format!("Failed to parse entries: {:?}", e))?;

    Ok(Input { figures, entries })
}

impl Solution for Day12 {
    fn part1(&self, input: &str) -> String {
        let parsed = parse_input(input).unwrap();
        format!(
            "Parsed {} figures and {} entries",
            parsed.figures.len(),
            parsed.entries.len()
        )
    }

    fn part2(&self, _input: &str) -> String {
        "Part 2 TODO".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_figure() {
        let input = "0:\n###\n#.#\n#.#\n\n";
        let result = parse_figure(input);
        assert!(result.is_ok());
        let (_, figure) = result.unwrap();
        assert_eq!(figure.id, 0);
        assert_eq!(figure.pattern[0], [true, true, true]);
        assert_eq!(figure.pattern[1], [true, false, true]);
        assert_eq!(figure.pattern[2], [true, false, true]);
    }

    #[test]
    fn test_parse_dimension_entry() {
        let input = "39x43: 23 41 27 30 29 31";
        let result = parse_dimension_entry(input);
        assert!(result.is_ok());
        let (_, entry) = result.unwrap();
        assert_eq!(entry.width, 39);
        assert_eq!(entry.height, 43);
        assert_eq!(entry.numbers, [23, 41, 27, 30, 29, 31]);
    }
}
