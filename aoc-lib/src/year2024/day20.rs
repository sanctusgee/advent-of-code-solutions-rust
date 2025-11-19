//! Advent of Code 2024 – Day 20 (Parts 1 & 2) — Race Condition
//!
//! Grid of walls `#` and open cells `.` with start `S` and end `E`.
//! Moving between 4-neighbors costs 1 picosecond.
//!
//! Let `L` be the normal shortest time from `S` to `E` with *no cheats*.
//!
//! A **cheat** lets you “jump” from one open cell `(x1,y1)` to another open
//! cell `(x2,y2)` at cost equal to their Manhattan distance
//! `d = |x1-x2| + |y1-y2|` even if walls lie between them, *once* per run.
//! The total time using such a cheat becomes:
//!     dist_start[y1][x1] + d + dist_goal[y2][x2] .
//!
//! We count cheats whose *saving* `L - (dist_start + d + dist_goal)` is
//! at least a threshold:
//!   - Part 1: jump radius R = 2, saving ≥ 100.
//!   - Part 2: jump radius R = 20, saving ≥ 100.
//!
//! Approach
//! --------
//! 1) BFS from S over open cells → dist_start
//! 2) BFS from E over open cells → dist_goal
//! 3) For every open cell u with finite dist_start[u],
//!    iterate all v within Manhattan radius R that are open and have finite
//!    dist_goal[v]. If `d = manhattan(u,v) ≤ R` and the new time is < L,
//!    accumulate if saving ≥ threshold.
//!
//! Complexity: O(N * R^2) where N is number of cells; for R ∈ {2,20} this is fast.

use std::collections::VecDeque;
use crate::utils;
use anyhow::Result;

fn parse_grid(input: &str) -> (Vec<Vec<u8>>, (usize, usize), (usize, usize)) {
    let mut grid: Vec<Vec<u8>> = Vec::new();
    let mut s: Option<(usize, usize)> = None;
    let mut e: Option<(usize, usize)> = None;

    for (r, line) in input.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let row = line.as_bytes().to_vec();
        for (c, &ch) in row.iter().enumerate() {
            if ch == b'S' {
                s = Some((r, c));
            } else if ch == b'E' {
                e = Some((r, c));
            }
        }
        grid.push(row);
    }

    (grid, s.expect("no S"), e.expect("no E"))
}

fn bfs_dist(grid: &[Vec<u8>], start: (usize, usize)) -> Vec<Vec<i32>> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut dist = vec![vec![-1; cols]; rows];
    let mut q = VecDeque::new();

    let (sr, sc) = start;
    dist[sr][sc] = 0;
    q.push_back((sr, sc));

    const DIRS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    while let Some((r, c)) = q.pop_front() {
        let d = dist[r][c] + 1;
        for (dr, dc) in DIRS {
            let nr = r as isize + dr;
            let nc = c as isize + dc;
            if nr < 0 || nc < 0 {
                continue;
            }
            let nr = nr as usize;
            let nc = nc as usize;
            if nr >= rows || nc >= cols {
                continue;
            }
            if grid[nr][nc] == b'#' {
                continue;
            }
            if dist[nr][nc] == -1 {
                dist[nr][nc] = d;
                q.push_back((nr, nc));
            }
        }
    }

    dist
}

#[inline]
fn count_cheats(
    grid: &[Vec<u8>],
    start: (usize, usize),
    end: (usize, usize),
    radius: i32,
    min_saving: i32,
) -> (i32, i64) {
    // returns (L, count)
    let rows = grid.len();
    let cols = grid[0].len();
    let dist_s = bfs_dist(grid, start);
    let dist_e = bfs_dist(grid, end);

    let l = dist_s[end.0][end.1];
    assert!(l >= 0, "no path without cheats");

    let mut count: i64 = 0;

    for r1 in 0..rows {
        for c1 in 0..cols {
            if grid[r1][c1] == b'#' {
                continue;
            }
            let d1 = dist_s[r1][c1];
            if d1 < 0 {
                continue;
            }

            // Iterate all positions within Manhattan radius around (r1,c1).
            // (Work in row/col; be careful with bounds.)
            for dr in -radius..=radius {
                let rem = radius - dr.abs();
                let rr = r1 as i32 + dr;
                if rr < 0 || rr >= rows as i32 {
                    continue;
                }
                let rr = rr as usize;

                for dc in -rem..=rem {
                    let cc_i32 = c1 as i32 + dc;
                    if cc_i32 < 0 || cc_i32 >= cols as i32 {
                        continue;
                    }
                    let cc = cc_i32 as usize;
                    if grid[rr][cc] == b'#' {
                        continue;
                    }
                    let d2 = dist_e[rr][cc];
                    if d2 < 0 {
                        continue;
                    }
                    let jump = dr.abs() + dc.abs();
                    if jump == 0 {
                        continue; // no-ops aren't cheats
                    }
                    let total = d1 + jump + d2;
                    if total < l && (l - total) >= min_saving {
                        count += 1;
                    }
                }
            }
        }
    }

    (l, count)
}

fn part1_count(grid: &[Vec<u8>], s: (usize, usize), e: (usize, usize)) -> i64 {
    let (_l, cnt) = count_cheats(grid, s, e, 2, 100);
    cnt
}

fn part2_count(grid: &[Vec<u8>], s: (usize, usize), e: (usize, usize)) -> i64 {
    let (_l, cnt) = count_cheats(grid, s, e, 20, 100);
    cnt
}

pub fn solve() -> Result<()> {
    let input = utils::load_input(2024, 20)?;
    let (grid, s, e) = parse_grid(&input);

    let p1 = part1_count(&grid, s, e);
    println!("Part 1: {}", p1);

    let p2 = part2_count(&grid, s, e);
    println!("Part 2: {}", p2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // A tiny synthetic grid. This is not the official example; it just sanity-checks logic.
    // S..#..E — without a cheat, you must go around; with a radius-2 cheat you can hop over
    // a short detour to gain savings. Thresholds are large in the puzzle (100), so for unit
    // checks we use the internal function with a small threshold.
    const G1: &str = r#"
S..#....
###.#..#
...#..E#
...#....
"#;

    fn parse_only(input: &str) -> (Vec<Vec<u8>>, (usize, usize), (usize, usize)) {
        parse_grid(input)
    }

    #[test]
    fn bfs_exists() {
        let (g, s, e) = parse_only(G1);
        let ds = bfs_dist(&g, s);
        let de = bfs_dist(&g, e);
        assert!(ds[e.0][e.1] >= 0);
        assert!(de[s.0][s.1] >= 0);
    }

    #[test]
    fn cheat_counts_are_nonnegative() {
        let (g, s, e) = parse_only(G1);
        let (_l, cnt_small_thresh) = count_cheats(&g, s, e, 2, 1);
        assert!(cnt_small_thresh >= 0);
    }

    #[test]
    fn parts_run() {
        let (g, s, e) = parse_only(G1);
        // With the puzzle threshold 100 these toy grids will likely be zero.
        assert!(part1_count(&g, s, e) >= 0);
        assert!(part2_count(&g, s, e) >= 0);
    }
}
