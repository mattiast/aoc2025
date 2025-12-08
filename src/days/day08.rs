use crate::Solution;
use nom::{
    IResult, Parser,
    character::complete::{char, line_ending, u64 as nom_u64},
    multi::separated_list1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point3D {
    pub x: u64,
    pub y: u64,
    pub z: u64,
}

impl Point3D {
    pub fn dist2(&self, other: &Point3D) -> u64 {
        let dx = self.x as i64 - other.x as i64;
        let dy = self.y as i64 - other.y as i64;
        let dz = self.z as i64 - other.z as i64;
        (dx * dx + dy * dy + dz * dz) as u64
    }
}

fn parse_point(input: &str) -> IResult<&str, Point3D> {
    (nom_u64, char(','), nom_u64, char(','), nom_u64)
        .map(|(x, _, y, _, z)| Point3D { x, y, z })
        .parse(input)
}

fn parse_points(input: &str) -> IResult<&str, Vec<Point3D>> {
    separated_list1(line_ending, parse_point).parse(input)
}

pub struct Day08;

impl Solution for Day08 {
    fn part1(&self, input: &str) -> String {
        let (_, points) = parse_points(input).unwrap();
        let n = points.len();
        let mut uf = UnionFind::new(n);

        let mut dists = Vec::with_capacity(n * (n - 1) / 2);
        for i in 0..n {
            for j in (i + 1)..n {
                let dist2 = points[i].dist2(&points[j]);
                dists.push((dist2, i, j));
            }
        }
        dists.sort_unstable_by_key(|&(dist2, _, _)| dist2);

        for (_, i, j) in &dists[..1000] {
            uf.union(*i, *j);
        }

        let mut component_sizes = uf.connected_component_sizes();
        component_sizes.sort_unstable_by_key(|&size| std::cmp::Reverse(size));
        let prod = component_sizes.iter().take(3).product::<usize>();

        format!("{:?}, product top 3 {}", &component_sizes, prod)
    }

    fn part2(&self, input: &str) -> String {
        let (_, points) = parse_points(input).unwrap();
        let n = points.len();
        let mut uf = UnionFind::new(n);

        let mut dists = Vec::with_capacity(n * (n - 1) / 2);
        for i in 0..n {
            for j in (i + 1)..n {
                let dist2 = points[i].dist2(&points[j]);
                dists.push((dist2, i, j));
            }
        }
        dists.sort_unstable_by_key(|&(dist2, _, _)| dist2);

        let mut components = n;
        for (_, i, j) in &dists {
            let components_reduced = uf.union(*i, *j);
            if components_reduced {
                // This could of course be part of the UnionFind struct
                components -= 1;
            }
            if components == 1 {
                let p1 = points[*i];
                let p2 = points[*j];
                let xprod = p1.x * p2.x;
                return format!(
                    "All points connected by connecting {:?} and {:?}, x prod {}",
                    p1, p2, xprod
                );
            }
        }
        format!("Could not connect all points")
    }
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    /// Unites the sets that contain x and y.
    /// Returns true if a union was performed, false if x and y were already in the same set.
    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x != root_y {
            if self.size[root_x] < self.size[root_y] {
                self.parent[root_x] = root_y;
                self.size[root_y] += self.size[root_x];
            } else {
                self.parent[root_y] = root_x;
                self.size[root_x] += self.size[root_y];
            }
            true
        } else {
            false
        }
    }

    fn connected_component_sizes(&mut self) -> Vec<usize> {
        let mut roots = std::collections::HashSet::new();
        for i in 0..self.parent.len() {
            roots.insert(self.find(i));
        }
        roots.into_iter().map(|r| self.size[r]).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
    #[test]
    fn test_parse_points() {
        let (_, points) = parse_points(INPUT).unwrap();
        assert_eq!(points.len(), 20);
        assert_eq!(
            points[0],
            Point3D {
                x: 162,
                y: 817,
                z: 812
            }
        );
        assert_eq!(
            points[1],
            Point3D {
                x: 57,
                y: 618,
                z: 57
            }
        );
        assert_eq!(
            points[19],
            Point3D {
                x: 425,
                y: 690,
                z: 689
            }
        );
    }

    #[test]
    fn test_part1() {
        let solution = Day08;
        let result = solution.part1(INPUT);
        assert_eq!(result, "4,3,2,11");
    }
}
