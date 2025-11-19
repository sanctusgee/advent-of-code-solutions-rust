//! Advent of Code 2024 – Day 18 (Parts 1 & 2) — RAM Run
//!
//! You’re given a list of falling “bytes” (blocked cells) as `x,y` pairs
//! into a square grid. Start is (0,0), goal is (N-1,N-1). Moves are
//! 4-directional (no diagonals).
//!
//! Part 1:
//!   - Use the FIRST K bytes (K=12 for the sample 7×7; K=1024 for the real
//!     71×71 input) as walls, then compute the shortest path length.
//!
//! Part 2:
//!   - Find the earliest byte in the stream whose addition makes the goal
//!     unreachable. Return that byte’s coordinate as `x,y`.
//!
//! Implementation notes
//! --------------------
//! - We parse all coordinates once.
//! - We infer grid size as follows:
//!       if max(x,y) <= 6  => size = 7 (sample), else size = 71 (puzzle)
//! - The "first K" for Part 1 is chosen by the inferred size:
//!       size == 7  => K = 12
//!       size == 71 => K = 1024
//! - BFS is used to compute shortest paths on the 4-connected grid.
//! - Part 2 uses a binary search on K in [0, bytes.len()] to find the first
//!   K where the path disappears; the blocking byte is bytes[K-1].
//!
//! File: input/year2024/day18.txt
//!   Each line: `x,y`

// all these bring back memory of CS2054 - embedded systems programming. So much fuuuun!!
use std::collections::{HashSet, VecDeque};
use crate::utils;
use anyhow::Result;

fn parse_coords(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .filter_map(|l| {
            let l = l.trim();
            if l.is_empty() {
                return None;
            }
            let mut it = l.split(',');
            let x = it.next()?.trim().parse::<usize>().ok()?;
            let y = it.next()?.trim().parse::<usize>().ok()?;
            Some((x, y))
        })
        .collect()
}

fn infer_size(coords: &[(usize, usize)]) -> usize {
    let mut m = 0usize;
    for &(x, y) in coords {
        m = m.max(x.max(y));
    }
    // Sample uses 0..=6 (size 7), real uses 0..=70 (size 71).
    if m <= 6 { 7 } else { 71 }
}

fn k_for_part1(size: usize) -> usize {
    if size == 7 { 12 } else { 1024 }
}

fn in_bounds(n: usize, x: isize, y: isize) -> bool {
    x >= 0 && y >= 0 && (x as usize) < n && (y as usize) < n
}

fn shortest_path_len(size: usize, blocked: &HashSet<(usize, usize)>) -> Option<usize> {
    let start = (0usize, 0usize);
    let goal = (size - 1, size - 1);
    if blocked.contains(&start) || blocked.contains(&goal) {
        return None;
    }

    let mut dist = vec![vec![usize::MAX; size]; size];
    let mut q = VecDeque::new();
    dist[start.1][start.0] = 0;
    q.push_back(start);

    const DIRS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    while let Some((x, y)) = q.pop_front() {
        if (x, y) == goal {
            return Some(dist[y][x]);
        }
        let d = dist[y][x] + 1;
        for (dx, dy) in DIRS {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if !in_bounds(size, nx, ny) {
                continue;
            }
            let (ux, uy) = (nx as usize, ny as usize);
            if blocked.contains(&(ux, uy)) {
                continue;
            }
            if d < dist[uy][ux] {
                dist[uy][ux] = d;
                q.push_back((ux, uy));
            }
        }
    }
    None
}

fn build_blocked(coords: &[(usize, usize)], k: usize) -> HashSet<(usize, usize)> {
    coords
        .iter()
        .take(k.min(coords.len()))
        .copied()
        .collect::<HashSet<_>>()
}

fn part1_min_steps(input: &str) -> Option<usize> {
    let coords = parse_coords(input);
    let size = infer_size(&coords);
    let k = k_for_part1(size);
    let blocked = build_blocked(&coords, k);
    shortest_path_len(size, &blocked)
}

fn part2_first_blocking_byte(input: &str) -> (usize, usize) {
    let coords = parse_coords(input);
    let size = infer_size(&coords);

    // Binary search the first K where path is None.
    let mut lo = 0usize;                 // path exists for lo
    let mut hi = coords.len();           // path does NOT exist for hi (eventually)
    // Ensure invariant: at lo=0, path exists if start != goal blocked (it isn't).
    // If already blocked at k=0, the puzzle is degenerate; but AoC guarantees solvable start.

    // First, grow hi until it breaks, if needed (usually coords.len() is enough).
    // Standard binary search on [lo, hi]:
    while lo < hi {
        let mid = (lo + hi) / 2;
        let blocked = build_blocked(&coords, mid);
        if shortest_path_len(size, &blocked).is_some() {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }

    // lo == hi is the minimal K with no path; answer is the K-th byte (0-based index K-1).
    // Problem states: “the first byte that causes the path to become impossible.”
    let k = lo;
    let idx = k.checked_sub(1)
        .expect("At least one byte must be required to block the path per problem statement");
    coords[idx]
}

pub fn solve() -> Result<()> {
    let input = utils::load_input(2024, 18)?;

    match part1_min_steps(&input) {
        Some(d) => println!("Part 1: {}", d),
        None => println!("Part 1: (no path)"),
    }

    let (x, y) = part2_first_blocking_byte(&input);
    println!("Part 2: {},{}", x, y);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Minimal synthetic checks for BFS and binary search logic.
    // (Not the official sample blob to keep this self-contained.)

    #[test]
    fn bfs_unblocked_small() {
        // On a 7x7 with no blocks, shortest path from (0,0) to (6,6) is 12.
        // Build no blocked list; call directly.
        let size = 7;
        let blocked = HashSet::new();
        assert_eq!(shortest_path_len(size, &blocked), Some(12));
    }

    #[test]
    fn part1_heuristic_on_sample_size_inference() {
        // When all coords within 0..=6, we infer size=7 and K=12.
        // With fewer than 12 bytes, we just use however many exist.
        let input = "1,0\n2,0\n3,0\n4,0\n5,0\n"; // 5 bytes blocking along the top row (not fully walling)
        // Should still have a path (this is a smoke test that it doesn't crash).
        assert!(part1_min_steps(input).is_some());
    }

    #[test]
    fn part2_simple_wall_cut() {
        // Build coordinates that eventually fully wall a row y=1 in a 7x7.
        // First 7 bytes across y=1 will block any path from y=0 to y>=2.
        // The *first* byte that completes the cut is x=6,y=1 (the 7th).
        let mut lines = Vec::new();
        for x in 0..7 {
            lines.push(format!("{},1", x));
        }
        let input = lines.join("\n");
        let (x, y) = part2_first_blocking_byte(&input);
        assert_eq!((x, y), (6, 1));
    }
}
