use crate::Solution;
use nom::{
    IResult, Parser,
    branch::alt,
    character::complete::{char, u32 as parse_u32},
    combinator::map,
};

pub struct Day01;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Instruction {
    direction: Direction,
    distance: u32,
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    alt((
        map(char('L'), |_| Direction::Left),
        map(char('R'), |_| Direction::Right),
    ))
    .parse(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, direction) = parse_direction(input)?;
    let (input, distance) = parse_u32(input)?;
    Ok((
        input,
        Instruction {
            direction,
            distance,
        },
    ))
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            parse_instruction(line)
                .expect("Failed to parse instruction")
                .1
        })
        .collect()
}

impl Solution for Day01 {
    fn part1(&self, input: &str) -> String {
        let instructions = parse_input(input);
        let mut x = 50;
        let mut zeros = 0;
        for inst in &instructions {
            match inst.direction {
                Direction::Left => x -= inst.distance as i32,
                Direction::Right => x += inst.distance as i32,
            }
            x %= 100;
            if x == 0 {
                zeros += 1;
            }
        }
        format!("Points at 0 {} times", zeros)
    }

    fn part2(&self, input: &str) -> String {
        let instructions = parse_input(input);
        let mut x = 50i32;
        let mut zeros = 0;
        for inst in &instructions {
            // If we go right, starting from 34.
            // We go by zero >= 1 time  if dist >= 66
            // We go by zero >= 2 times if dist >= 166

            // If we go left, starting from 34
            // We go by zero >= 1 time  if dist >= 34
            // We go by zero >= 2 times if dist >= 134
            match inst {
                &Instruction {
                    direction: Direction::Right,
                    distance,
                } => {
                    zeros += (x + distance as i32) / 100;
                    x += distance as i32;
                }
                &Instruction {
                    direction: Direction::Left,
                    distance,
                } => {
                    let from_zero = if x == 0 { 0 } else { 100 - x };
                    zeros += (distance as i32 + from_zero) / 100;
                    x -= distance as i32;
                }
            }
            x = x.rem_euclid(100);
        }
        format!("Pass by 0 {} times", zeros)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instruction() {
        let (_, inst) = parse_instruction("R27").unwrap();
        assert_eq!(inst.direction, Direction::Right);
        assert_eq!(inst.distance, 27);

        let (_, inst) = parse_instruction("L5").unwrap();
        assert_eq!(inst.direction, Direction::Left);
        assert_eq!(inst.distance, 5);
    }

    #[test]
    fn test_parse_input() {
        let input = "R27\nL5\nR99\n";
        let instructions = parse_input(input);
        assert_eq!(instructions.len(), 3);
        assert_eq!(instructions[0].distance, 27);
        assert_eq!(instructions[1].direction, Direction::Left);
    }
}
