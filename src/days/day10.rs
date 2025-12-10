use crate::Solution;

pub struct Day10;

#[derive(Debug, Clone)]
struct Device {
    pattern: Vec<bool>, // true = on (#), false = off (.)
    groups: Vec<Vec<usize>>,
    joltages: Vec<u32>,
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

impl Solution for Day10 {
    fn part1(&self, input: &str) -> String {
        let devices = parse_input(input);
        format!("Parsed {} devices", devices.len())
    }

    fn part2(&self, _input: &str) -> String {
        "Part 2 TODO".to_string()
    }
}
