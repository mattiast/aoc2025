use crate::Solution;
use nom::{
    IResult, Parser,
    bytes::complete::take,
    character::complete::{char, line_ending, space1},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
};
use std::collections::HashMap;

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

struct PathSolver {
    // Graph: node -> list of neighbors
    connections: HashMap<NodeId, Vec<NodeId>>,
    // Cache: (from, to) -> number of paths
    cache: HashMap<(NodeId, NodeId), u64>,
}

impl PathSolver {
    fn new(connections_list: Vec<Connection>) -> Self {
        let mut connections = HashMap::new();
        for conn in connections_list {
            connections.insert(conn.node, conn.neighbors);
        }
        Self {
            connections,
            cache: HashMap::new(),
        }
    }

    fn num_paths(&mut self, a: NodeId, b: NodeId) -> u64 {
        // Base case
        if a == b {
            return 1;
        }

        // Check cache
        if let Some(&count) = self.cache.get(&(a, b)) {
            return count;
        }

        // Recursive case: collect neighbors first to avoid borrow issues
        let neighbors = self.connections.get(&a).cloned();
        let result = if let Some(neighbors) = neighbors {
            neighbors
                .iter()
                .map(|&neighbor| self.num_paths(neighbor, b))
                .sum()
        } else {
            0 // Dead end: no outgoing edges
        };

        self.cache.insert((a, b), result);
        result
    }
}

pub struct Day11;
impl Solution for Day11 {
    fn part1(&self, input: &str) -> String {
        let (_, connections) = parse_input(input).expect("Failed to parse input");
        let mut solver = PathSolver::new(connections);

        let result = solver.num_paths(*b"you", *b"out");

        format!("Number of distinct paths from 'you' to 'out': {}", result)
    }

    fn part2(&self, input: &str) -> String {
        let (_, connections) = parse_input(input).expect("Failed to parse input");
        let mut solver = PathSolver::new(connections);

        let y2d = solver.num_paths(*b"svr", *b"dac");
        let d2f = solver.num_paths(*b"dac", *b"fft");
        let f2o = solver.num_paths(*b"fft", *b"out");
        let y2f = solver.num_paths(*b"svr", *b"fft");
        let f2d = solver.num_paths(*b"fft", *b"dac");
        let d2o = solver.num_paths(*b"dac", *b"out");
        let result = y2d * d2f * f2o + y2f * f2d * d2o;

        format!("Paths going through 'dac' and 'fft': {}", result)
    }
}
