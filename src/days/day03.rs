use crate::Solution;
use nom::{
    IResult, Parser,
    character::complete::{digit1, newline},
    combinator::map,
    multi::separated_list1,
};

pub struct Day03;

type Grid = Vec<Vec<u8>>;

fn parse_digit(c: char) -> u8 {
    c.to_digit(10).unwrap() as u8
}

fn parse_line(input: &str) -> IResult<&str, Vec<u8>> {
    map(digit1, |s: &str| s.chars().map(parse_digit).collect()).parse(input)
}

fn parse_input(input: &str) -> IResult<&str, Grid> {
    separated_list1(newline, parse_line).parse(input)
}

impl Solution for Day03 {
    fn part1(&self, input: &str) -> anyhow::Result<String> {
        let (_, grid) = parse_input(input).expect("Failed to parse input");
        let mut joltage = 0u64;
        for line in &grid {
            joltage += find_joltage::<2>(line);
        }
        Ok(format!(
            "Parsed grid: {} rows x {} cols, total joltage {}",
            grid.len(),
            grid[0].len(),
            joltage
        ))
    }

    fn part2(&self, input: &str) -> anyhow::Result<String> {
        let (_, grid) = parse_input(input).expect("Failed to parse input");
        let mut total_joltage = 0u64;
        let m = grid.len();
        let n = grid[0].len();
        for line in &grid {
            total_joltage += find_joltage::<12>(line);
        }
        Ok(format!(
            "Parsed grid: {} rows x {} cols, total joltage {}",
            m, n, total_joltage
        ))
    }
}

fn find_joltage<const N: usize>(line: &[u8]) -> u64 {
    let n = line.len();
    let mut noi: [u8; N] = line[n - N..].try_into().unwrap();
    for &digit in line[..n - N].iter().rev() {
        let mut newnoi = noi;
        if digit >= noi[0] {
            newnoi[0] = digit;
        } else {
            continue;
        }
        for i in 1..N {
            if noi[i - 1] >= noi[i] {
                newnoi[i] = noi[i - 1];
            } else {
                break;
            }
        }
        noi = newnoi;
    }
    let joltage = {
        let mut acc = 0u64;
        for &digit in &noi[..] {
            acc = acc * 10 + digit as u64;
        }
        acc
    };
    joltage
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        const TEST_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";
        let output = Day03.part1(TEST_INPUT);
        assert!(output.contains("357"), "Unexpected output: {}", output);
    }
    #[test]
    fn test_part2_sample() {
        const TEST_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";
        let output = Day03.part2(TEST_INPUT);
        assert!(
            output.contains("3121910778619"),
            "Unexpected output: {}",
            output
        );
    }
    #[test]
    fn test_parse_line() {
        let input = "987654321111111";
        let (_, digits) = parse_line(input).unwrap();
        assert_eq!(digits.len(), 15);
        assert_eq!(digits[0], 9);
        assert_eq!(digits[1], 8);
        assert_eq!(digits[2], 7);
        assert_eq!(digits[14], 1);
    }

    #[test]
    fn test_parse_input() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        let (_, grid) = parse_input(input).unwrap();
        assert_eq!(grid.len(), 4);
        assert_eq!(grid[0].len(), 15);
        assert_eq!(grid[0][0], 9);
        assert_eq!(grid[1][0], 8);
        assert_eq!(grid[2][0], 2);
        assert_eq!(grid[3][0], 8);
    }

    #[test]
    fn test_parse_digit() {
        assert_eq!(parse_digit('0'), 0);
        assert_eq!(parse_digit('5'), 5);
        assert_eq!(parse_digit('9'), 9);
    }
}
