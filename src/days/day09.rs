use crate::Solution;
use nom::{
    IResult, Parser,
    character::complete::{char, i64 as nom_i64, line_ending},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
};

// Tiles are grid squares - the input coordinates representing unit squares on a grid
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Tile {
    x: i64,
    y: i64,
}

// Points are exact corner coordinates where polygon edges meet (between tiles)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy)]
struct Rect {
    left: i64,
    right: i64,
    bottom: i64,
    top: i64,
}

impl Rect {
    fn from_tiles(t1: Tile, t2: Tile) -> Self {
        Rect {
            left: t1.x.min(t2.x),
            right: t1.x.max(t2.x) + 1,
            bottom: t1.y.min(t2.y),
            top: t1.y.max(t2.y) + 1,
        }
    }

    fn area(&self) -> i64 {
        (self.right - self.left) * (self.top - self.bottom)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy)]
struct Edge {
    p1: Point,
    p2: Point,
}

// Parsing functions
fn parse_tile(input: &str) -> IResult<&str, Tile> {
    map(separated_pair(nom_i64, char(','), nom_i64), |(x, y)| Tile {
        x,
        y,
    })
    .parse(input)
}

fn parse_input(input: &str) -> Vec<Tile> {
    let result: IResult<&str, Vec<Tile>> =
        separated_list1(line_ending, parse_tile).parse(input.trim());
    match result {
        Ok((_, tiles)) => tiles,
        Err(_) => vec![],
    }
}

// Part 1: Maximum rectangle from any two tiles
fn solve_part1(tiles: &[Tile]) -> i64 {
    let mut max_area = 0;

    for i in 0..tiles.len() {
        for j in (i + 1)..tiles.len() {
            let rect = Rect::from_tiles(tiles[i], tiles[j]);
            max_area = max_area.max(rect.area());
        }
    }

    max_area
}

// Part 2: Helper functions

// Determine direction from tile t1 to tile t2
fn direction(t1: Tile, t2: Tile) -> Direction {
    let dx = t2.x - t1.x;
    let dy = t2.y - t1.y;

    // Assert that tiles are adjacent (differ in exactly one coordinate)
    assert!(
        (dx == 0) as i64 + (dy == 0) as i64 == 1,
        "Tiles must be adjacent: {:?} -> {:?}",
        t1,
        t2
    );

    if dx < 0 {
        Direction::West
    } else if dx > 0 {
        Direction::East
    } else if dy < 0 {
        Direction::North
    } else {
        Direction::South
    }
}

// Compute corner Point from a Tile and the directions before and after
fn get_corner(d0: Direction, tile: Tile, d2: Direction) -> Point {
    let x = tile.x;
    let y = tile.y;

    match (d0, d2) {
        // Convex corners
        (Direction::North, Direction::East) => Point { x, y },
        (Direction::East, Direction::South) => Point { x: x + 1, y },
        (Direction::South, Direction::West) => Point { x: x + 1, y: y + 1 },
        (Direction::West, Direction::North) => Point { x, y: y + 1 },

        // Concave corners
        (Direction::South, Direction::East) => Point { x: x + 1, y },
        (Direction::East, Direction::North) => Point { x, y },
        (Direction::North, Direction::West) => Point { x, y: y + 1 },
        (Direction::West, Direction::South) => Point { x: x + 1, y: y + 1 },

        _ => panic!("Invalid direction combination: {:?} -> {:?}", d0, d2),
    }
}

// Convert tiles to corner Points
fn compute_corners(tiles: &[Tile]) -> Vec<Point> {
    let n = tiles.len();
    let mut corners = Vec::with_capacity(n);

    for i in 0..n {
        let p0 = tiles[(i + n - 1) % n];
        let p1 = tiles[i];
        let p2 = tiles[(i + 1) % n];

        let d0 = direction(p0, p1);
        let d2 = direction(p1, p2);

        corners.push(get_corner(d0, p1, d2));
    }

    corners
}

// Extract vertical and horizontal edges from corner Points
fn extract_edges(corners: &[Point]) -> (Vec<Edge>, Vec<Edge>) {
    // TODO: This assumes that the first edge is vertical. Could be made more robust.
    let n = corners.len();
    let mut verticals = Vec::with_capacity(n / 2);
    let mut horizontals = Vec::with_capacity(n / 2);

    // Based on the Python code pattern:
    // verticals = [(corners[2*i], corners[2*i+1]) for i in range(len(corners)//2)]
    // horizontals = [(corners[2*i], corners[2*i-1]) for i in range(len(corners)//2)]

    for i in 0..(n / 2) {
        verticals.push(Edge {
            p1: corners[2 * i],
            p2: corners[2 * i + 1],
        });

        let prev_idx = if 2 * i == 0 { n - 1 } else { 2 * i - 1 };
        horizontals.push(Edge {
            p1: corners[2 * i],
            p2: corners[prev_idx],
        });
    }

    (verticals, horizontals)
}

// Check if rect intersects vertical edge
fn touch_vertical(rect: &Rect, edge: &Edge) -> bool {
    let x = edge.p1.x;
    let mut y1 = edge.p1.y;
    let mut y2 = edge.p2.y;

    if y1 > y2 {
        std::mem::swap(&mut y1, &mut y2);
    }

    // Check if x is strictly between left and right
    if !(rect.left < x && x < rect.right) {
        return false;
    }

    // Check if y-intervals overlap
    rect.bottom.max(y1) < rect.top.min(y2)
}

// Check if rect intersects horizontal edge
fn touch_horizontal(rect: &Rect, edge: &Edge) -> bool {
    let y = edge.p1.y;
    let mut x1 = edge.p1.x;
    let mut x2 = edge.p2.x;

    if x1 > x2 {
        std::mem::swap(&mut x1, &mut x2);
    }

    // Check if y is strictly between bottom and top
    if !(rect.bottom < y && y < rect.top) {
        return false;
    }

    // Check if x-intervals overlap
    rect.left.max(x1) < rect.right.min(x2)
}

// Part 2: Maximum rectangle not intersecting carpet boundary
fn solve_part2(tiles: &[Tile]) -> i64 {
    let corners = compute_corners(tiles);
    let (verticals, horizontals) = extract_edges(&corners);

    let mut max_area = 0;

    for i in 0..tiles.len() {
        for j in (i + 1)..tiles.len() {
            let rect = Rect::from_tiles(tiles[i], tiles[j]);

            // Check if rect intersects any edge
            let intersects = verticals.iter().any(|e| touch_vertical(&rect, e))
                || horizontals.iter().any(|e| touch_horizontal(&rect, e));

            if !intersects {
                max_area = max_area.max(rect.area());
            }
        }
    }

    max_area
}

pub struct Day09;

impl Solution for Day09 {
    fn part1(&self, input: &str) -> String {
        let tiles = parse_input(input);
        let max_area = solve_part1(&tiles);
        format!("Maximum rectangle area: {}", max_area)
    }

    fn part2(&self, input: &str) -> String {
        let tiles = parse_input(input);
        let max_area = solve_part2(&tiles);
        format!("Maximum non-intersecting rectangle area: {}", max_area)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
    #[test]
    fn test_parse_tile() {
        let (_, tile) = parse_tile("5,10").unwrap();
        assert_eq!(tile.x, 5);
        assert_eq!(tile.y, 10);
    }

    #[test]
    fn test_parse_input() {
        let tiles = parse_input(SAMPLE_INPUT);
        assert_eq!(tiles.len(), 8);
        assert_eq!(tiles[0].x, 7);
        assert_eq!(tiles[0].y, 1);
        assert_eq!(tiles[7].x, 7);
        assert_eq!(tiles[7].y, 3);
    }

    #[test]
    fn test_rect_from_tiles() {
        let t1 = Tile { x: 1, y: 2 };
        let t2 = Tile { x: 5, y: 7 };
        let rect = Rect::from_tiles(t1, t2);

        assert_eq!(rect.left, 1);
        assert_eq!(rect.right, 6);
        assert_eq!(rect.bottom, 2);
        assert_eq!(rect.top, 8);
    }

    #[test]
    fn test_rect_area() {
        let rect = Rect {
            left: 1,
            right: 6,
            bottom: 2,
            top: 8,
        };
        assert_eq!(rect.area(), 30);
    }

    #[test]
    fn test_direction() {
        let t1 = Tile { x: 5, y: 3 };
        let t2 = Tile { x: 6, y: 3 };
        assert_eq!(direction(t1, t2), Direction::East);

        let t3 = Tile { x: 5, y: 2 };
        assert_eq!(direction(t1, t3), Direction::North);

        let t4 = Tile { x: 4, y: 3 };
        assert_eq!(direction(t1, t4), Direction::West);

        let t5 = Tile { x: 5, y: 4 };
        assert_eq!(direction(t1, t5), Direction::South);
    }

    #[test]
    fn test_get_corner() {
        let tile = Tile { x: 5, y: 3 };

        // Convex corner: North -> East
        let corner = get_corner(Direction::North, tile, Direction::East);
        assert_eq!(corner.x, 5);
        assert_eq!(corner.y, 3);

        // Convex corner: East -> South
        let corner = get_corner(Direction::East, tile, Direction::South);
        assert_eq!(corner.x, 6);
        assert_eq!(corner.y, 3);
    }

    #[test]
    fn test_compute_corners() {
        let tiles = parse_input(SAMPLE_INPUT);
        let corners = compute_corners(&tiles);
        assert_eq!(corners.len(), tiles.len());
    }

    #[test]
    fn test_part1() {
        let solution = Day09;
        let result = solution.part1(SAMPLE_INPUT);
        assert!(result.contains("Maximum rectangle area: 50"));
    }

    #[test]
    fn test_part2() {
        let solution = Day09;
        let input: &str = "7,3
7,1
11,1
11,7
9,7
9,5
2,5
2,3";
        let result = solution.part2(input);
        assert_eq!(result, "Maximum non-intersecting rectangle area: 24");
    }
}
