use crate::Solution;
use nom::{
    IResult, Parser,
    character::complete::{char, newline, u64 as nom_u64},
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
};

pub struct Day05;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range {
    pub start: u64,
    pub end: u64,
}

impl Range {
    pub fn contains(&self, number: u64) -> bool {
        number >= self.start && number <= self.end
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Input {
    pub ranges: Vec<Range>,
    pub numbers: Vec<u64>,
}

fn merge_ranges(sorted_ranges: &[Range]) -> Vec<Range> {
    let mut iterator = sorted_ranges.iter();

    let mut merged_ranges = Vec::new();
    let mut current_range = match iterator.next() {
        Some(r) => *r,
        None => return merged_ranges,
    };
    for &range in iterator {
        if range.start <= current_range.end + 1 {
            current_range.end = current_range.end.max(range.end);
        } else {
            merged_ranges.push(current_range);
            current_range = range;
        }
    }
    merged_ranges.push(current_range);
    merged_ranges
}

impl Solution for Day05 {
    fn part1(&self, input: &str) -> anyhow::Result<String> {
        let (_, mut data) = parse_input_complete(input).expect("Failed to parse input");
        data.ranges.sort_by_key(|r| r.start);

        let merged_ranges = merge_ranges(&data.ranges);

        let mut num_fresh = 0;
        for &number in &data.numbers {
            let search_res = merged_ranges.binary_search_by(|r| {
                if r.contains(number) {
                    std::cmp::Ordering::Equal
                } else if number < r.start {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Less
                }
            });
            if search_res.is_ok() {
                num_fresh += 1;
            }
        }

        Ok(format!(
            "Parsed {} ranges and {} numbers, {} numbers are within at least one range.",
            data.ranges.len(),
            data.numbers.len(),
            num_fresh
        ))
    }

    fn part2(&self, input: &str) -> anyhow::Result<String> {
        let (_, mut data) = parse_input_complete(input).expect("Failed to parse input");
        data.ranges.sort_by_key(|r| r.start);

        let merged_ranges = merge_ranges(&data.ranges);
        let total_covered: u64 = merged_ranges.iter().map(|r| r.end - r.start + 1).sum();

        Ok(format!(
            "After merging, there are {} ranges covering a total of {} numbers.",
            merged_ranges.len(),
            total_covered
        ))
    }
}

fn parse_range(input: &str) -> IResult<&str, Range> {
    let (input, (start, end)) = separated_pair(nom_u64, char('-'), nom_u64).parse(input)?;
    Ok((input, Range { start, end }))
}

fn parse_ranges(input: &str) -> IResult<&str, Vec<Range>> {
    separated_list1(newline, parse_range).parse(input)
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(newline, nom_u64).parse(input)
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    let (input, ranges) = terminated(parse_ranges, many1(newline)).parse(input)?;
    let (input, numbers) = parse_numbers.parse(input)?;
    Ok((input, Input { ranges, numbers }))
}

fn parse_input_complete(input: &str) -> IResult<&str, Input> {
    terminated(parse_input, many1(newline)).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    const SAMPLE_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

    #[test]
    fn test_parse_range() {
        let (_, range) = parse_range("3-5").unwrap();
        assert_eq!(range.start, 3);
        assert_eq!(range.end, 5);
    }

    #[test]
    fn test_parse_ranges() {
        let input = "3-5\n10-14\n16-20";
        let (_, ranges) = parse_ranges(input).unwrap();
        assert_eq!(ranges.len(), 3);
        assert_eq!(ranges[0], Range { start: 3, end: 5 });
        assert_eq!(ranges[1], Range { start: 10, end: 14 });
        assert_eq!(ranges[2], Range { start: 16, end: 20 });
    }

    #[test]
    fn test_parse_numbers() {
        let input = "1\n5\n8\n11";
        let (_, numbers) = parse_numbers(input).unwrap();
        assert_eq!(numbers.len(), 4);
        assert_eq!(numbers, vec![1, 5, 8, 11]);
    }

    #[test]
    fn test_parse_input() {
        let (_, data) = parse_input_complete(SAMPLE_INPUT).unwrap();
        assert_eq!(data.ranges.len(), 4);
        assert_eq!(data.numbers.len(), 6);

        assert_eq!(data.ranges[0], Range { start: 3, end: 5 });
        assert_eq!(data.ranges[3], Range { start: 12, end: 18 });

        assert_eq!(data.numbers[0], 1);
        assert_eq!(data.numbers[5], 32);
    }

    #[test]
    fn test_part1_sample() {
        let output = Day05.part1(SAMPLE_INPUT);
        let re = Regex::new(r"\<3\>").unwrap();
        assert!(
            re.is_match(&output),
            "Output did not match expected pattern: {}",
            output
        );
    }

    #[test]
    fn test_part2_sample() {
        let output = Day05.part2(SAMPLE_INPUT);
        let re = Regex::new(r"\<14\>").unwrap();
        assert!(
            re.is_match(&output),
            "Output did not match expected pattern: {}",
            output
        );
    }
}
