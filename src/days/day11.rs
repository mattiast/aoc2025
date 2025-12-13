use crate::Solution;
use nom::{
    IResult, Parser,
    bytes::complete::take,
    character::complete::{char, line_ending, space1},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
};

type NodeId = [u8; 3];
#[derive(Debug, Clone)]
struct Connection {
    node: NodeId,
    neighbors: Vec<NodeId>,
}

// Parse a 3-character identifier
fn parse_identifier(input: &str) -> IResult<&str, [u8; 3]> {
    map(take(3usize), |s: &str| s.as_bytes().try_into().unwrap()).parse(input)
}

// Parse a list of space-separated identifiers
fn parse_neighbor_list(input: &str) -> IResult<&str, Vec<[u8; 3]>> {
    separated_list1(space1, parse_identifier).parse(input)
}

// Parse a single line: "abc: def ghi jkl"
fn parse_connection(input: &str) -> IResult<&str, Connection> {
    map(
        separated_pair(parse_identifier, (char(':'), space1), parse_neighbor_list),
        |(node, neighbors)| Connection { node, neighbors },
    )
    .parse(input)
}

// Parse the full input
fn parse_input(input: &str) -> IResult<&str, Vec<Connection>> {
    separated_list1(line_ending, parse_connection).parse(input)
}

pub struct Day11;
impl Solution for Day11 {
    fn part1(&self, input: &str) -> String {
        let (_, connections) = parse_input(input).expect("Failed to parse input");
        format!("Parsed {} connections", connections.len())
    }

    fn part2(&self, _input: &str) -> String {
        "Part 2 TODO".to_string()
    }
}
