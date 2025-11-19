//! Advent of Code 2024 – Day 14
//!
//! This module solves **both parts** of the “Restroom Robots” puzzle.
//!
//! - **Part 1:** Simulate robots for 100 seconds and compute the “safety factor”
//!   (the product of robot counts in each quadrant).
//! - **Part 2:** Find the earliest second when the robots form a Christmas tree
//!   pattern, i.e. the tightest bounding box (lowest area) on the toroidal grid.
//!
//! The simulation assumes a *wrap-around grid* (toroidal world).  
//! Positions use modular arithmetic via `rem_euclid` so robots “wrap” cleanly.

use crate::utils;
use anyhow::Result;

/// A single robot with position and velocity on a toroidal grid.
#[derive(Debug, Clone)]
struct Robot {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl Robot {
    /// Creates a new `Robot` from initial position and velocity.
    fn new(x: i32, y: i32, vx: i32, vy: i32) -> Self {
        Self { x, y, vx, vy }
    }

    /// Advances the robot by one second, wrapping around grid edges.
    ///
    /// This uses `rem_euclid` to ensure negative coordinates wrap correctly
    /// (so `-1` on width 10 becomes `9`, not `-1`).
    // fn step(&mut self, width: i32, height: i32) {
    //     self.x = (self.x + self.vx).rem_euclid(width);
    //     self.y = (self.y + self.vy).rem_euclid(height);
    // }

    /// Computes robot’s position after `t` seconds *without looping*.
    ///
    /// The direct formula `(x + vx * t) mod width` is much faster than
    /// stepping one tick at a time—handy for scanning thousands of steps.
    fn pos_at(&self, t: i32, width: i32, height: i32) -> (i32, i32) {
        let nx = ((self.x as i64) + (self.vx as i64) * (t as i64))
            .rem_euclid(width as i64) as i32;
        let ny = ((self.y as i64) + (self.vy as i64) * (t as i64))
            .rem_euclid(height as i64) as i32;
        (nx, ny)
    }

    /// Returns the robot’s quadrant index (0–3) at time `t`,
    /// or `None` if it sits exactly on the center lines.
    fn get_quadrant_at(&self, t: i32, width: i32, height: i32) -> Option<usize> {
        let (x, y) = self.pos_at(t, width, height);
        let mid_x = width / 2;
        let mid_y = height / 2;
        if x == mid_x || y == mid_y {
            return None; // center-line robots don’t count
        }
        match (x < mid_x, y < mid_y) {
            (true,  true)  => Some(0), // top-left
            (false, true)  => Some(1), // top-right
            (true,  false) => Some(2), // bottom-left
            (false, false) => Some(3), // bottom-right
        }
    }
}

/* ─────────────────────────── Parsing & utilities ─────────────────────────── */

/// Parses input lines like `p=18,60 v=90,-17` into a vector of `Robot`s.
fn parse_robots(input: &str) -> Vec<Robot> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let mut parts = line.split_whitespace();
            let pos = parts.next().unwrap().strip_prefix("p=").unwrap();
            let vel = parts.next().unwrap().strip_prefix("v=").unwrap();
            let mut p = pos.split(',').map(|s| s.parse::<i32>().unwrap());
            let mut v = vel.split(',').map(|s| s.parse::<i32>().unwrap());
            Robot::new(p.next().unwrap(), p.next().unwrap(), v.next().unwrap(), v.next().unwrap())
        })
        .collect()
}

/// Computes every robot’s position at time `t`.
fn positions_at_time(robots: &[Robot], t: i32, w: i32, h: i32) -> Vec<(i32, i32)> {
    robots.iter().map(|r| r.pos_at(t, w, h)).collect()
}

/// Returns the area of a minimal bounding box covering all given points.
fn bbox_area(points: &[(i32, i32)]) -> i64 {
    let (minx, maxx, miny, maxy) = points.iter().fold(
        (i32::MAX, i32::MIN, i32::MAX, i32::MIN),
        |(mnx, mxx, mny, mxy), &(x, y)| (mnx.min(x), mxx.max(x), mny.min(y), mxy.max(y)),
    );
    let w = (maxx - minx + 1) as i64;
    let h = (maxy - miny + 1) as i64;
    w * h
}

/// Least common multiple, used to find the grid’s repeat period.
fn lcm(a: i32, b: i32) -> i32 {
    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let t = a % b;
            a = b;
            b = t;
        }
        a.abs()
    }
    (a / gcd(a, b)) * b
}

/* ──────────────────────────────── Part 1 ───────────────────────────────── */

/// Computes the **safety factor** after `t` seconds (product of quadrant counts).
fn safety_factor_at_t(robots: &[Robot], t: i32, w: i32, h: i32) -> i32 {
    let mut q = [0; 4];
    for r in robots {
        if let Some(idx) = r.get_quadrant_at(t, w, h) {
            q[idx] += 1;
        }
    }
    q.iter().product()
}

/* ──────────────────────────────── Part 2 ───────────────────────────────── */

/// Searches one full torus period for the smallest bounding-box area,
/// returning `(time, area)`.  The earliest minimum is considered the
/// moment the “Christmas tree” appears.
fn find_tree_time(robots: &[Robot], w: i32, h: i32) -> (i32, i64) {
    let period = lcm(w, h);              // world repeats every LCM(width,height)
    let mut best_t = 0;
    let mut best_area = i64::MAX;
    let mut best_unique = false;

    for t in 0..period {
        let pts = positions_at_time(robots, t, w, h);
        let area = bbox_area(&pts);

        // Favor frames where no two robots overlap—tree looks “crisp”.
        use std::collections::HashSet;
        let unique = {
            let set: HashSet<(i32, i32)> = pts.iter().copied().collect();
            set.len() == robots.len()
        };

        let better = area < best_area || (area == best_area && (unique && !best_unique));
        if better {
            best_t = t;
            best_area = area;
            best_unique = unique;
        }
    }
    (best_t, best_area)
}

/// (Optional) Produces an ASCII rendering of robot positions at `t`.
fn _render_at_t(robots: &[Robot], t: i32, w: i32, h: i32) -> String {
    use std::collections::HashSet;
    let pts = positions_at_time(robots, t, w, h);
    let (minx, maxx, miny, maxy) = pts.iter().fold(
        (i32::MAX, i32::MIN, i32::MAX, i32::MIN),
        |(mnx, mxx, mny, mxy), &(x, y)| (mnx.min(x), mxx.max(x), mny.min(y), mxy.max(y)),
    );
    let set: HashSet<(i32, i32)> = pts.into_iter().collect();
    let mut out = String::new();
    for y in miny..=maxy {
        for x in minx..=maxx {
            out.push(if set.contains(&(x, y)) { '#' } else { '.' });
        }
        out.push('\n');
    }
    out
}

/* ─────────────────────────────── Entry Point ───────────────────────────── */

/// Main solver: loads input, runs both parts, and prints results.
///
/// Output format:
/// ```text
/// Part 1: <safety_factor>
/// Part 2: <seconds_until_tree>
/// ```
pub fn solve() -> Result<()> {
    // Read puzzle input (adjust path to your environment if needed).
    let input = utils::load_input(2024, 14)?;
    let robots = parse_robots(&input);

    // Puzzle's grid dimensions.
    let (width, height) = (101, 103);

    // Solutions:
    // Part 1
    let safety = safety_factor_at_t(&robots, 100, width, height);
    println!("Part 1: {}", safety);

    // Part 2
    let (tree_t, _area) = find_tree_time(&robots, width, height);
    println!("Part 2: {}", tree_t);

    //Uncomment below to visualize the tree (disabled for performance).
    // println!("{}", _render_at_t(&robots, tree_t, width, height));

    Ok(())
}


/* ─────────────────────────────── Unit Tests ────────────────────────────── */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1_matches_spec() {
        // Sample from the AoC problem statement.
        let example = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;

        let robots = parse_robots(example);
        assert_eq!(safety_factor_at_t(&robots, 100, 11, 7), 12);
    }

    // #[test]
    // fn pos_at_equivalence_with_step() {
    //     // Ensure direct math equals repeated stepping.
    //     let r = Robot::new(2, 4, 2, -3);
    //     for t in 0..40 {
    //         let mut sim = r.clone();
    //         for _ in 0..t { sim.step(11, 7); }
    //         assert_eq!(r.pos_at(t, 11, 7), (sim.x, sim.y));
    //     }
    // }
}
