use std::collections::HashSet;

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
        let mut set: HashSet<u64> = HashSet::new();
        for range in &lines {
            sum_invalid(range, 2, &mut set);
        }
        let total_sum_invalid: u64 = set.iter().sum();
        format!(
            "Parsed {} ranges, sum of invalid: {:?}",
            lines.len(),
            total_sum_invalid
        )
    }

    fn part2(&self, input: &str) -> String {
        let lines = parse_input(input).expect("Failed to parse input").1;
        let mut set: HashSet<u64> = HashSet::new();
        for range in &lines {
            for k in 2..=num_length(range.end) {
                sum_invalid(range, k, &mut set);
            }
        }
        let total_sum_invalid: u64 = set.iter().sum();
        format!(
            "Parsed {} ranges, sum of invalid: {:?}",
            lines.len(),
            total_sum_invalid
        )
    }
}
fn sum_invalid(range: &Range, k: u32, set: &mut HashSet<u64>) {
    // dbg!((range, k));
    let start_len = num_length(range.start);
    let end_len = num_length(range.end);

    // If either is even, divide that length by 2
    let n = if start_len % k == 0 || end_len % k == 0 {
        end_len / k
    } else {
        return;
    };
    // Find the number of
    let div = (0..k).into_iter().map(|i| 10u64.pow(i * n)).sum::<u64>();
    let mut a = (range.start + div - 1) / div;
    let mut b = range.end / div;
    if end_len % k != 0 {
        b = 10u64.pow(n) - 1;
    }
    if start_len % k != 0 {
        a = 10u64.pow(n - 1);
    }
    for i in a..=b {
        set.insert(i * div);
    }
}

fn num_length(mut n: u64) -> u32 {
    let mut length = 0;
    while n > 0 {
        n /= 10;
        length += 1;
    }
    length
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
