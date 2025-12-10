use crate::Solution;

fn group_to_bitmask(group: &Vec<usize>) -> u16 {
    group.iter().fold(0, |acc, &n| acc | (1 << n))
}
fn pattern_to_bitmask(pattern: &Vec<bool>) -> u16 {
    pattern.iter().enumerate().fold(0, |acc, (i, &b)| {
        if b {
            acc | (1 << i)
        } else {
            acc
        }
    })
}

#[derive(Debug, Clone)]
struct Device {
    pattern: Vec<bool>, // true = on (#), false = off (.)
    groups: Vec<Vec<usize>>,
    joltages: Vec<u32>,
}

/// Given a pattern and a group of patterns, find the minimum
/// size of a subset of the group, whose XOR equals the pattern.
fn min_repr_pattern(pattern: u16, group: &Vec<u16>) -> u16 {
    let mut dp = vec![u16::MAX; 1 << 10];
    dp[0] = 0;

    for &g in group {
        for p in 0..dp.len() {
            let new_p = p ^ g as usize;
            if dp[p] != u16::MAX {
                dp[new_p] = dp[new_p].min(dp[p] + 1);
            }
        }
    }

    dp[pattern as usize]
}

pub struct Day10;

impl Solution for Day10 {
    fn part1(&self, input: &str) -> String {
        let devices = parse_input(input);
        let mut total = 0;
        for device in &devices {
            let g = device.groups.iter().map(|g| group_to_bitmask(g)).collect();
            let min_size = min_repr_pattern(pattern_to_bitmask(&device.pattern), &g);
            total += min_size as u32;
        }
        format!("Parsed {} devices, total min size: {}", devices.len(), total)
    }

    fn part2(&self, _input: &str) -> String {
        "Part 2 TODO".to_string()
    }
}

mod parser {
    use super::Device;
    use nom::{
        IResult, Parser,
        bytes::complete::take_while1,
        character::complete::{char, multispace0, u32 as nom_u32},
        combinator::map,
        multi::{many1, separated_list0, separated_list1},
        sequence::{delimited, preceded},
    };

    // Parse a pattern like [.##.] into Vec<bool>
    fn pattern(input: &str) -> IResult<&str, Vec<bool>> {
        delimited(
            char('['),
            map(take_while1(|c| c == '.' || c == '#'), |s: &str| {
                s.chars().map(|c| c == '#').collect()
            }),
            char(']'),
        )
        .parse(input)
    }

    // Parse a group like (1,2,3) into Vec<usize>
    fn group(input: &str) -> IResult<&str, Vec<usize>> {
        delimited(
            char('('),
            map(separated_list0(char(','), nom_u32), |nums| {
                nums.into_iter().map(|n| n as usize).collect()
            }),
            char(')'),
        )
        .parse(input)
    }

    // Parse joltages like {3,5,4,7} into Vec<u32>
    fn joltages(input: &str) -> IResult<&str, Vec<u32>> {
        delimited(char('{'), separated_list1(char(','), nom_u32), char('}')).parse(input)
    }

    // Parse a complete line
    pub fn device_line(input: &str) -> IResult<&str, Device> {
        let (input, pat) = pattern(input)?;
        let (input, _) = multispace0(input)?;
        let (input, grps) = many1(preceded(multispace0, group)).parse(input)?;
        let (input, _) = multispace0(input)?;
        let (input, jolts) = joltages(input)?;

        Ok((
            input,
            Device {
                pattern: pat,
                groups: grps,
                joltages: jolts,
            },
        ))
    }
}

fn parse_input(input: &str) -> Vec<Device> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter_map(|line| parser::device_line(line).ok().map(|(_, device)| device))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

    #[test]
    fn test_parse_sample() {
        let devices = parse_input(SAMPLE_INPUT);
        assert_eq!(devices.len(), 3);

        // First device: [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
        assert_eq!(devices[0].pattern, vec![false, true, true, false]);
        assert_eq!(devices[0].groups.len(), 6);
        assert_eq!(devices[0].groups[0], vec![3]);
        assert_eq!(devices[0].groups[1], vec![1, 3]);
        assert_eq!(devices[0].joltages, vec![3, 5, 4, 7]);

        // Second device: [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
        assert_eq!(devices[1].pattern, vec![false, false, false, true, false]);
        assert_eq!(devices[1].groups.len(), 5);
        assert_eq!(devices[1].groups[0], vec![0, 2, 3, 4]);
        assert_eq!(devices[1].joltages, vec![7, 5, 12, 7, 2]);

        // Third device: [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
        assert_eq!(
            devices[2].pattern,
            vec![false, true, true, true, false, true]
        );
        assert_eq!(devices[2].groups.len(), 4);
        assert_eq!(devices[2].groups[0], vec![0, 1, 2, 3, 4]);
        assert_eq!(devices[2].joltages, vec![10, 11, 11, 5, 10, 5]);
    }

    #[test]
    fn test_part1_sample() {
        let day10 = Day10;
        let result = day10.part1(SAMPLE_INPUT);
        // Just verify it runs without panicking
        assert_eq!(result, "Parsed 3 devices, total min size: 7");
    }
}
