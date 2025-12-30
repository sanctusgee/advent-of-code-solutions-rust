// file: year2025/day08.rs

// Advent of Code
// Day 08: Playground
//
// https://adventofcode.com/2025/day/8

// see day08_README.md for implementation details
//

use anyhow::{bail, Context, Result};
use crate::utils;

// 3D position of a junction box.
// Small, Copy-friendly, no heap involvement.
#[derive(Clone, Copy, Debug)]
struct Point3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Point3 {
    // Parse a single `x,y,z` line.
    // Explicit field handling keeps errors precise.
    fn parse(line: &str) -> Result<Self> {
        let mut it = line.split(',');
		// the error checking is overkill for AoC, but I am choosing to exercise good Rust muscle
        let x = it.next().context("missing x")?.trim().parse().context("bad x")?;
        let y = it.next().context("missing y")?.trim().parse().context("bad y")?;
        let z = it.next().context("missing z")?.trim().parse().context("bad z")?;
        if it.next().is_some() {
            bail!("too many fields");
        }
        Ok(Self { x, y, z })
    }

    // Squared distance avoids sqrt and preserves ordering.
	// Hint from
	// https://www.reddit.com/r/adventofcode/comments/1pr5oq5/first_time_and_want_to_learn_more/
	// >>> "One simple optimisation that most people spotted early on is to not bother with
	// 		the square root when calculating the Euclidian distance, the order is presered
	// 		if you don't bother and you keep everything to using integers. "
	// >>>
    #[inline]
    fn dist2(self, other: Self) -> i64 {
        let dx = (other.x - self.x) as i64;
        let dy = (other.y - self.y) as i64;
        let dz = (other.z - self.z) as i64;
        dx * dx + dy * dy + dz * dz
    }
}

// Edge between two points, weighted by squared distance.
// Struct beats tuple soup for readability.
#[derive(Clone, Copy, Debug)]
struct Edge {
    w: i64,
    i: usize,
    j: usize,
}

// Disjoint Set Union (Union-Find).
// Tracks connected components efficiently during Kruskal.
struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    groups: usize, // number of current components
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            groups: n,
        }
    }

    // Number of remaining connected components.
    fn groups(&self) -> usize {
        self.groups
    }

    // Path-halving find.
    // Slightly faster than full compression, still correct.
    fn find(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            let next = self.parent[x];
            self.parent[x] = self.parent[next];
            x = next;
        }
        x
    }

    // Union by size.
    // Returns true only when a merge actually happens.
    fn union(&mut self, a: usize, b: usize) -> bool {
        let (mut ra, mut rb) = (self.find(a), self.find(b));
        if ra == rb {
            return false;
        }
        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }
        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
        self.groups -= 1;
        true
    }

    // Compute component sizes by counting true roots.
    fn component_sizes(&mut self) -> Vec<usize> {
        let n = self.parent.len();
        let mut counts = vec![0usize; n];
        for i in 0..n {
            let r = self.find(i);
            counts[r] += 1;
        }
        counts.into_iter().filter(|&c| c > 0).collect()
    }
}

// Parse entire input into points.
// Fails early on malformed input.
fn parse_points(input: &str) -> Result<Vec<Point3>> {
    let points: Vec<Point3> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .enumerate()
        .map(|(i, line)| Point3::parse(line).with_context(|| format!("line {}", i + 1)))
        .collect::<Result<_>>()?;

    if points.len() < 2 {
        bail!("need at least 2 points");
    }
    Ok(points)
}

// Build all possible edges (O(n²)).
// Acceptable for AoC constraints.
fn build_edges(points: &[Point3]) -> Vec<Edge> {
    let n = points.len();
    let mut edges = Vec::with_capacity(n * (n - 1) / 2);
    for i in 0..n {
        for j in (i + 1)..n {
            edges.push(Edge {
                w: points[i].dist2(points[j]),
                i,
                j,
            });
        }
    }
    edges
}

// Multiply the three largest component sizes.
// Single-pass selection avoids sorting.
fn top3_product(sizes: &[usize]) -> Result<u64> {
    if sizes.len() < 3 {
        bail!("need at least 3 components");
    }
    let (mut a, mut b, mut c) = (0usize, 0usize, 0usize);
    for &s in sizes {
        if s >= a {
            c = b;
            b = a;
            a = s;
        } else if s >= b {
            c = b;
            b = s;
        } else if s > c {
            c = s;
        }
    }
    Ok((a as u64) * (b as u64) * (c as u64))
}

// Defines when the Kruskal scan should stop.
enum StopRule {
    // Stop after K successful unions (Part 1).
    AfterEdgeAttempts(usize),
    // Stop when the graph becomes fully connected (Part 2).
    UntilSingleCircuit,
}

// Result of a Kruskal run.
// Captures both final UF state and last successful edge.
struct RunResult {
    uf: UnionFind,
    last_success: Option<(usize, usize)>,
}

// Core Kruskal runner.
// Consumes edges in ascending order and applies unions until the stop rule fires.
// Shared by both parts to avoid duplication.
fn kruskal_run(points_len: usize, edges: &mut [Edge], stop: StopRule) -> RunResult {
    edges.sort_unstable_by_key(|e| e.w);

    let mut uf = UnionFind::new(points_len);
    let mut last_success = None;

    match stop {
        StopRule::AfterEdgeAttempts(k) => {
            for e in edges.iter().take(k) {
                if uf.union(e.i, e.j) {
                    last_success = Some((e.i, e.j));
                }
            }
        }
        StopRule::UntilSingleCircuit => {
            for e in edges.iter() {
                if uf.union(e.i, e.j) {
                    last_success = Some((e.i, e.j));
                    if uf.groups() == 1 {
                        break;
                    }
                }
            }
        }
    }

    RunResult { uf, last_success }
}


// Keep only the K smallest edges without sorting everything.
// Uses `select_nth_unstable` for O(n) partitioning.
fn take_k_smallest_edges(mut edges: Vec<Edge>, k: usize) -> Vec<Edge> {
    if k == 0 || edges.is_empty() {
        return Vec::new();
    }
    if k < edges.len() {
        edges.select_nth_unstable_by_key(k, |e| e.w);
        edges.truncate(k);
    }
    edges
}

// Part 1:
// Attempt the 1000 closest pairs, then compute top-3 circuit sizes.
pub fn solve_part1(input: &str) -> Result<u64> {
    const K_CLOSEST_PAIRS: usize = 1000;

    let points = parse_points(input)?;
    let mut edges = build_edges(&points);
    edges = take_k_smallest_edges(edges, K_CLOSEST_PAIRS);

    let mut res = kruskal_run(
        points.len(),
        &mut edges,
        StopRule::AfterEdgeAttempts(K_CLOSEST_PAIRS),
    );

    top3_product(&res.uf.component_sizes())
}

// Part 2:
// Continue Kruskal until everything is connected.
// Return product of X coordinates of the final connecting edge.
pub fn solve_part2(input: &str) -> Result<u64> {
    let points = parse_points(input)?;
    let mut edges = build_edges(&points);

    let res = kruskal_run(points.len(), &mut edges, StopRule::UntilSingleCircuit);

    let (i, j) = res.last_success.context("no successful union occurred")?;

    Ok((points[i].x as i64 * points[j].x as i64) as u64)
}


pub fn solve() -> Result<()> {
    let input = utils::load_input(2025, 8)?;

    let part1 = solve_part1(&input)?;
    let part2 = solve_part2(&input)?;

    println!("Day 8 / Year 2025");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
162,817,812
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
425,690,689
"#;

    #[test]
    fn part1_example_after_10_shortest_connections_is_40() {
        let points = parse_points(SAMPLE).unwrap();
        let mut edges = build_edges(&points);

        // "After making the ten shortest connections"
        // Interpreted as 10 successful unions (connections that actually merge circuits).
        let mut res = kruskal_run(points.len(), &mut edges, StopRule::AfterEdgeAttempts(10));
        let ans = top3_product(&res.uf.component_sizes()).unwrap();
        assert_eq!(ans, 40);
    }

    #[test]
    fn part2_example_last_x_product_is_25272() {
        let points = parse_points(SAMPLE).unwrap();
        let mut edges = build_edges(&points);

        let res = kruskal_run(points.len(), &mut edges, StopRule::UntilSingleCircuit);

        let (i, j) = res.last_success.unwrap();
        let ans = (points[i].x as i64 * points[j].x as i64) as u64;

        assert_eq!(ans, 25272);
    }
}