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

fn parse_pattern(s: &str) -> Vec<bool> {
    s.chars().map(|c| c == '#').collect()
}

fn parse_group(s: &str) -> Vec<usize> {
    if s.is_empty() {
        return vec![];
    }
    s.split(',').filter_map(|n| n.parse().ok()).collect()
}

fn parse_line(line: &str) -> Option<Device> {
    // Extract pattern from [...]
    let pattern_start = line.find('[')?;
    let pattern_end = line.find(']')?;
    let pattern = parse_pattern(&line[pattern_start + 1..pattern_end]);

    // Extract everything after the pattern
    let rest = &line[pattern_end + 1..].trim();

    // Find the joltages section {...}
    let joltages_start = rest.rfind('{')?;
    let joltages_end = rest.rfind('}')?;
    let joltages: Vec<u32> = rest[joltages_start + 1..joltages_end]
        .split(',')
        .filter_map(|n| n.trim().parse().ok())
        .collect();

    // Extract groups from (...)
    let groups_section = &rest[..joltages_start].trim();
    let mut groups = Vec::new();

    let mut in_group = false;
    let mut current_group = String::new();

    for ch in groups_section.chars() {
        match ch {
            '(' => {
                in_group = true;
                current_group.clear();
            }
            ')' => {
                if in_group {
                    groups.push(parse_group(&current_group));
                    in_group = false;
                }
            }
            _ if in_group => {
                current_group.push(ch);
            }
            _ => {}
        }
    }

    Some(Device {
        pattern,
        groups,
        joltages,
    })
}

fn parse_input(input: &str) -> Vec<Device> {
    input.lines().filter_map(|line| parse_line(line)).collect()
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
