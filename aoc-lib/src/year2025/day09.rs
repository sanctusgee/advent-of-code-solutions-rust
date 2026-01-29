// file: year2025/day09.rs

// Advent of Code
// Day 09: Movie Thetatre
//
// https://adventofcode.com/2025/day/9

use anyhow::{anyhow, Result};
use crate::utils;

pub fn solve() -> Result<()> {
    let input = utils::load_input(2025, 9)?;

    let part1 = solve_part1(&input)?;
    let part2 = solve_part2(&input)?;

    println!("Day 9 / Year 2025");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn parse(line: &str) -> Result<Self> {
        let line = line.trim();
        if line.is_empty() {
            return Err(anyhow!("empty line"));
        }

        let (xs, ys) = line
            .split_once(',')
            .ok_or_else(|| anyhow!("invalid point (missing comma): {line}"))?;

        let x: i64 = xs.trim().parse()?;
        let y: i64 = ys.trim().parse()?;

        Ok(Self { x, y })
    }
}

#[derive(Debug, Clone, Copy)]
struct Rect {
    xmin: i64,
    xmax: i64,
    ymin: i64,
    ymax: i64,
}

impl Rect {
    fn from_opposite(a: Point, b: Point) -> Self {
        let (xmin, xmax) = if a.x <= b.x { (a.x, b.x) } else { (b.x, a.x) };
        let (ymin, ymax) = if a.y <= b.y { (a.y, b.y) } else { (b.y, a.y) };
        Self { xmin, xmax, ymin, ymax }
    }

    fn area_tiles(&self) -> i64 {
        (self.xmax - self.xmin).abs().saturating_add(1) * (self.ymax - self.ymin).abs().saturating_add(1)
    }

    fn corners(&self) -> [Point; 4] {
        [
            Point { x: self.xmin, y: self.ymin },
            Point { x: self.xmin, y: self.ymax },
            Point { x: self.xmax, y: self.ymin },
            Point { x: self.xmax, y: self.ymax },
        ]
    }
}

#[derive(Debug, Clone, Copy)]
struct Edge {
    a: Point,
    b: Point,
}

impl Edge {
    fn new(a: Point, b: Point) -> Result<Self> {
        if a.x != b.x && a.y != b.y {
            return Err(anyhow!("non-axis-aligned edge: {:?} -> {:?}", a, b));
        }
        Ok(Self { a, b })
    }

    fn is_vertical(&self) -> bool {
        self.a.x == self.b.x
    }

    fn x_const(&self) -> i64 {
        self.a.x
    }

    fn y_const(&self) -> i64 {
        self.a.y
    }

    fn y_min_max(&self) -> (i64, i64) {
        if self.a.y <= self.b.y { (self.a.y, self.b.y) } else { (self.b.y, self.a.y) }
    }

    fn x_min_max(&self) -> (i64, i64) {
        if self.a.x <= self.b.x { (self.a.x, self.b.x) } else { (self.b.x, self.a.x) }
    }

    // True if this polygon boundary edge has any point strictly inside the rectangle interior.
    // If yes, the rectangle cannot be fully contained in a simple filled polygon region.
    fn intersects_rect_interior(&self, r: Rect) -> bool {
        if self.is_vertical() {
            let x = self.x_const();
            if !(r.xmin < x && x < r.xmax) {
                return false;
            }
            let (y1, y2) = self.y_min_max();
            let lo = y1.max(r.ymin);
            let hi = y2.min(r.ymax);
            lo < hi && r.ymin < hi && lo < r.ymax
        } else {
            let y = self.y_const();
            if !(r.ymin < y && y < r.ymax) {
                return false;
            }
            let (x1, x2) = self.x_min_max();
            let lo = x1.max(r.xmin);
            let hi = x2.min(r.xmax);
            lo < hi && r.xmin < hi && lo < r.xmax
        }
    }
}

fn point_on_edge(p: Point, e: Edge) -> bool {
    if e.is_vertical() {
        if p.x != e.x_const() {
            return false;
        }
        let (y1, y2) = e.y_min_max();
        y1 <= p.y && p.y <= y2
    } else {
        if p.y != e.y_const() {
            return false;
        }
        let (x1, x2) = e.x_min_max();
        x1 <= p.x && p.x <= x2
    }
}

// Ray casting to +X, treating boundary as inside.
// Uses half-open rule on vertical edges to avoid double counting at vertices.
fn point_in_or_on_polygon(p: Point, edges: &[Edge]) -> bool {
    if edges.iter().any(|&e| point_on_edge(p, e)) {
        return true;
    }

    let mut crossings: u32 = 0;

    for &e in edges {
        if !e.is_vertical() {
            continue;
        }

        let x = e.x_const();
        if x <= p.x {
            continue;
        }

        let (y1, y2) = e.y_min_max();
        // half-open: include y1, exclude y2
        if y1 <= p.y && p.y < y2 {
            crossings += 1;
        }
    }

    (crossings & 1) == 1
}

fn rect_fully_inside_polygon(r: Rect, edges: &[Edge]) -> bool {
    for c in r.corners() {
        if !point_in_or_on_polygon(c, edges) {
            return false;
        }
    }

    // Boundary cannot pass through rectangle interior.
    for &e in edges {
        if e.intersects_rect_interior(r) {
            return false;
        }
    }

    true
}

fn parse_points_in_order(input: &str) -> Result<Vec<Point>> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(Point::parse)
        .collect::<Result<Vec<_>>>()
}

fn build_edges(points: &[Point]) -> Result<Vec<Edge>> {
    if points.len() < 2 {
        return Ok(vec![]);
    }

    let mut edges = Vec::with_capacity(points.len());
    for i in 0..points.len() {
        let a = points[i];
        let b = points[(i + 1) % points.len()];
        edges.push(Edge::new(a, b)?);
    }
    Ok(edges)
}

fn solve_part1(input: &str) -> Result<impl std::fmt::Display> {
    let points = parse_points_in_order(input)?;
    if points.len() < 2 {
        return Ok(0_i64);
    }

    let mut best: i64 = 0;

    for i in 0..points.len() - 1 {
        let a = points[i];
        for j in (i + 1)..points.len() {
            let rect = Rect::from_opposite(a, points[j]);
            let area = rect.area_tiles();
            if area > best {
                best = area;
            }
        }
    }

    Ok(best)
}

fn solve_part2(input: &str) -> Result<impl std::fmt::Display> {
    let points = parse_points_in_order(input)?;
    if points.len() < 2 {
        return Ok(0_i64);
    }

    let edges = build_edges(&points)?;

    let mut best: i64 = 0;

    for i in 0..points.len() - 1 {
        let a = points[i];
        for j in (i + 1)..points.len() {
            let rect = Rect::from_opposite(a, points[j]);
            let area = rect.area_tiles();

            if area <= best {
                continue;
            }

            if rect_fully_inside_polygon(rect, &edges) {
                best = area;
            }
        }
    }

    Ok(best)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1_is_50() {
        let input = r#"
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
"#;

        assert_eq!(solve_part1(input).unwrap().to_string(), "50");
    }

    #[test]
    fn example_part2_is_24() {
        let input = r#"
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
"#;

        assert_eq!(solve_part2(input).unwrap().to_string(), "24");
    }
}
