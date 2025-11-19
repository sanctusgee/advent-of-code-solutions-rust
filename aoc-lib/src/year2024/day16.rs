//! Advent of Code 2024 – Day 16 (Parts 1 & 2) — Reindeer Maze
//!
//! Model each state as (row, col, facing). Moves:
//! - forward 1 cell: +1 cost (cannot enter '#')
//! - rotate left or right 90°: +1000 cost (stay on the same cell)
//!
//! Part 1: minimal score from S (facing East) to E (any facing).
//! Part 2: count tiles that lie on at least one optimal path. A tile counts
//!         if there exists some facing d such that:
//!             dist_start[r][c][d] + dist_goal[r][c][d] == best_total
//!         where dist_goal is computed on the reverse graph starting from E
//!         with all 4 facings at cost 0.

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use crate::utils;
use anyhow::Result;

// #[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]

enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    #[inline]
    fn left(self) -> Self {
        match self {
            Dir::North => Dir::West,
            Dir::West  => Dir::South,
            Dir::South => Dir::East,
            Dir::East  => Dir::North,
        }
    }
    #[inline]
    fn right(self) -> Self {
        match self {
            Dir::North => Dir::East,
            Dir::East  => Dir::South,
            Dir::South => Dir::West,
            Dir::West  => Dir::North,
        }
    }
    #[inline]
    fn delta(self) -> (isize, isize) {
        match self {
            Dir::North => (-1, 0),
            Dir::East  => (0, 1),
            Dir::South => (1, 0),
            Dir::West  => (0, -1),
        }
    }
    #[inline]
    fn idx(self) -> usize {
        match self {
            Dir::North => 0,
            Dir::East  => 1,
            Dir::South => 2,
            Dir::West  => 3,
        }
    }
    #[inline]
    fn all() -> [Dir; 4] {
        [Dir::North, Dir::East, Dir::South, Dir::West]
    }
}

fn parse_grid(input: &str) -> (Vec<Vec<u8>>, (usize, usize), (usize, usize)) {
    let mut grid: Vec<Vec<u8>> = Vec::new();
    let mut s: Option<(usize, usize)> = None;
    let mut e: Option<(usize, usize)> = None;

    for (r, line) in input.lines().filter(|l| !l.trim().is_empty()).enumerate() {
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

fn dijkstra_forward(
    grid: &[Vec<u8>],
    start_r: usize,
    start_c: usize,
    start_dir: Dir,
) -> Vec<Vec<[i64; 4]>> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut dist = vec![vec![[i64::MAX; 4]; cols]; rows];

    let mut pq = BinaryHeap::new();
    dist[start_r][start_c][start_dir.idx()] = 0;
    pq.push((Reverse(0_i64), start_r, start_c, start_dir));

    while let Some((Reverse(cost), r, c, d)) = pq.pop() {
        if cost != dist[r][c][d.idx()] {
            continue;
        }

        // rotate left
        let nd = d.left();
        let ncost = cost + 1000;
        if ncost < dist[r][c][nd.idx()] {
            dist[r][c][nd.idx()] = ncost;
            pq.push((Reverse(ncost), r, c, nd));
        }

        // rotate right
        let nd = d.right();
        let ncost = cost + 1000;
        if ncost < dist[r][c][nd.idx()] {
            dist[r][c][nd.idx()] = ncost;
            pq.push((Reverse(ncost), r, c, nd));
        }

        // forward move
        let (dr, dc) = d.delta();
        let nr = r as isize + dr;
        let nc = c as isize + dc;
        if nr >= 0 && nc >= 0 && (nr as usize) < rows && (nc as usize) < cols {
            let (nr, nc) = (nr as usize, nc as usize);
            if grid[nr][nc] != b'#' {
                let ncost = cost + 1;
                if ncost < dist[nr][nc][d.idx()] {
                    dist[nr][nc][d.idx()] = ncost;
                    pq.push((Reverse(ncost), nr, nc, d));
                }
            }
        }
    }

    dist
}

fn dijkstra_reverse_from_goal(
    grid: &[Vec<u8>],
    end_r: usize,
    end_c: usize,
) -> Vec<Vec<[i64; 4]>> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut dist = vec![vec![[i64::MAX; 4]; cols]; rows];
    let mut pq = BinaryHeap::new();

    for d in Dir::all() {
        dist[end_r][end_c][d.idx()] = 0;
        pq.push((Reverse(0_i64), end_r, end_c, d));
    }

    while let Some((Reverse(cost), r, c, d)) = pq.pop() {
        if cost != dist[r][c][d.idx()] {
            continue;
        }

        // rotation predecessors
        for pd in [d.left(), d.right()] {
            let ncost = cost + 1000;
            if ncost < dist[r][c][pd.idx()] {
                dist[r][c][pd.idx()] = ncost;
                pq.push((Reverse(ncost), r, c, pd));
            }
        }

        // move predecessor
        let (dr, dc) = d.delta();
        let pr = r as isize - dr;
        let pc = c as isize - dc;
        if pr >= 0 && pc >= 0 && (pr as usize) < rows && (pc as usize) < cols {
            let (pr, pc) = (pr as usize, pc as usize);
            if grid[pr][pc] != b'#' {
                let ncost = cost + 1;
                if ncost < dist[pr][pc][d.idx()] {
                    dist[pr][pc][d.idx()] = ncost;
                    pq.push((Reverse(ncost), pr, pc, d));
                }
            }
        }
    }

    dist
}

fn part1_min_score(grid: &[Vec<u8>], s: (usize, usize), e: (usize, usize)) -> i64 {
    let dist_start = dijkstra_forward(grid, s.0, s.1, Dir::East);
    Dir::all()
        .iter()
        .map(|&d| dist_start[e.0][e.1][d.idx()])
        .min()
        .expect("no directions?")
}

fn part2_count_tiles_on_best_paths(grid: &[Vec<u8>], s: (usize, usize), e: (usize, usize)) -> usize {
    let dist_start = dijkstra_forward(grid, s.0, s.1, Dir::East);
    let dist_goal = dijkstra_reverse_from_goal(grid, e.0, e.1);

    let best_total = Dir::all()
        .iter()
        .map(|&d| dist_start[e.0][e.1][d.idx()])
        .min()
        .unwrap_or(i64::MAX);

    let rows = grid.len();
    let cols = grid[0].len();
    let mut on_path = vec![vec![false; cols]; rows];

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == b'#' {
                continue;
            }
            for &d in &Dir::all() {
                let a = dist_start[r][c][d.idx()];
                let b = dist_goal[r][c][d.idx()];
                if a != i64::MAX && b != i64::MAX && a + b == best_total {
                    on_path[r][c] = true;
                    break;
                }
            }
        }
    }

    on_path
        .iter()
        .enumerate()
        .map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter(|(c, &v)| v && grid[r][*c] != b'#')
                .count()
        })
        .sum()
}

pub fn solve() -> Result<()> {
    let input = utils::load_input(2024, 16)?;
    let (grid, start, end) = parse_grid(&input);

    // Part 1: Find lowest score
    let best_score = part1_min_score(&grid, start, end);
    println!("Part 1: {}", best_score);

    // Part 2: Count tiles on any best path
    let tiles_count = part2_count_tiles_on_best_paths(&grid, start, end);
    println!("Part 2: {}", tiles_count);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1: &str = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;

    const EX2: &str = r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"#;

    #[test]
    fn example_part1_a() {
        let (g, s, e) = parse_grid(EX1);
        assert_eq!(part1_min_score(&g, s, e), 7036);
    }

    #[test]
    fn example_part1_b() {
        let (g, s, e) = parse_grid(EX2);
        assert_eq!(part1_min_score(&g, s, e), 11048);
    }

    #[test]
    fn example_part2_a() {
        let (g, s, e) = parse_grid(EX1);
        assert_eq!(part2_count_tiles_on_best_paths(&g, s, e), 45);
    }

    #[test]
    fn example_part2_b() {
        let (g, s, e) = parse_grid(EX2);
        assert_eq!(part2_count_tiles_on_best_paths(&g, s, e), 64);
    }
}
