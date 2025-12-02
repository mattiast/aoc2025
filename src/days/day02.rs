use crate::Solution;
use nom::{
    IResult, Parser,
    character::complete::{char, u64 as parse_u64},
    multi::separated_list0,
    sequence::separated_pair,
};

pub struct Day02;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range {
    pub start: u64,
    pub end: u64,
}

fn parse_range(input: &str) -> IResult<&str, Range> {
    let (input, (start, end)) = separated_pair(parse_u64, char('-'), parse_u64).parse(input)?;
    Ok((input, Range { start, end }))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Range>> {
    separated_list0(char(','), parse_range).parse(input)
}

impl Solution for Day02 {
    fn part1(&self, input: &str) -> String {
        let (_, lines) = parse_input(input).expect("Failed to parse input");
        // TODO: Implement part 1 logic
        format!(
            "Parsed {} ranges",
            lines.len(),
        )
    }

    fn part2(&self, input: &str) -> String {
        let lines = parse_input(input).expect("Failed to parse input").1;
        // TODO: Implement part 2 logic
        format!("Parsed {} lines", lines.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_range() {
        let (_, range) = parse_range("851786270-851907437").unwrap();
        assert_eq!(range.start, 851786270);
        assert_eq!(range.end, 851907437);
    }

    #[test]
    fn test_parse_line() {
        let input = "851786270-851907437,27-47,577-1044";
        let (_, ranges) = parse_input(input).unwrap();
        assert_eq!(ranges.len(), 3);
        assert_eq!(ranges[0].start, 851786270);
        assert_eq!(ranges[0].end, 851907437);
        assert_eq!(ranges[1].start, 27);
        assert_eq!(ranges[1].end, 47);
    }

    #[test]
    fn test_parse_input() {
        let input = "851786270-851907437,27-47,2-17\n";
        let lines = parse_input(input).unwrap().1;
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[2].start, 2);
        assert_eq!(lines[2].end, 17);
    }
}
