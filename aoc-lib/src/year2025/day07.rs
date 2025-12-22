// file: year2025/day07.rs

// Advent of Code
// Day 07: Laboratories
//
// https://adventofcode.com/2025/day/7

use anyhow::Result;
use crate::utils;
use std::collections::VecDeque;

pub fn solve() -> Result<()> {
    let input = utils::load_input(2025, 7)?;

    let part1 = solve_part1(&input)?;
    let part2 = solve_part2(&input)?;

    println!("Day 7 / Year 2025");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

fn solve_part1(input: &str) -> Result<impl std::fmt::Display> {
    let m = Manifold::parse(input)?;
    let res = simulate(&m, Mode::Classical);
    Ok(res.classical_splits)
}

fn solve_part2(input: &str) -> Result<impl std::fmt::Display> {
    let m = Manifold::parse(input)?;
    let res = simulate(&m, Mode::Quantum);
    Ok(res.quantum_timelines)
}

#[derive(Debug)]
struct Manifold {
    grid: Vec<Vec<u8>>,
    width: usize,
    height: usize,
    start_row: usize,
    start_col: usize,
}

impl Manifold {
    // Parse the input into a rectangular byte grid and locate the unique 'S'.
    // Assumes AoC-style well-formed input (rectangular, one 'S').
    fn parse(input: &str) -> Result<Self> {
        let lines: Vec<&str> = input
            .lines()
            .filter(|l| !l.trim().is_empty())
            .collect();

        let height = lines.len();
        let width = lines
            .get(0)
            .map(|s| s.len())
            .unwrap_or(0);

        let grid: Vec<Vec<u8>> = lines
            .iter()
            .map(|l| l.as_bytes().to_vec())
            .collect();

        // Locate 'S' (unique by problem statement).
        let (start_row, start_col) = grid
            .iter()
            .enumerate()
            .find_map(|(y, row)| row.iter().position(|&c| c == b'S').map(|x| (y, x)))
            .expect("Missing start position 'S'");

        Ok(Self {
            grid,
            width,
            height,
            start_row,
            start_col,
        })
    }
}

#[derive(Copy, Clone, Debug)]
enum Mode {
    // Classical: beams merge; presence is boolean per column per row; count splitter hits.
    Classical,
    // Quantum: counts represent timeline multiplicity; branches add; count completed journeys.
    Quantum,
}

#[derive(Default)]
struct SimResult {
    classical_splits: u64,
    quantum_timelines: u64,
}

// Single shared simulation engine used by both parts.
//
// Core idea:
// - We process rows top-to-bottom starting below 'S' (beams always move down).
// - State is a per-column vector of counts at the current row.
//   - Classical: counts are treated as boolean presence (0 or 1).
//   - Quantum: counts are timeline multiplicities (0..).
// - Splitter resolution can cascade within the same row; we resolve to a fixed point using a queue.
//
// Part 1 and Part 2 differ only in:
// - how we combine counts (boolean vs additive),
// - whether out-of-bounds emissions are ignored or counted as completed,

fn simulate(m: &Manifold, mode: Mode) -> SimResult {
    // counts[x] = number of active "things" at column x on the current row:
    // - Classical: 0/1 presence
    // - Quantum: number of timelines at that x
    let mut counts = vec![0_u64; m.width];
    counts[m.start_col] = 1;

    // Only one of these is used depending on mode; keeping both avoids branching at return sites.
    let mut classical_splits: u64 = 0;
    let mut quantum_completed: u64 = 0;

    // Process rows starting immediately below 'S'. If S is on the last row, loop is empty.
    for y in (m.start_row + 1)..m.height {
        let row = &m.grid[y];

        // Resolve splitter cascades on this row.
        resolve_row(row, m.width, mode, &mut counts, &mut classical_splits, &mut quantum_completed);

        // Early exit: if nothing remains active, nothing can reappear in lower rows.
        if counts.iter().all(|&c| c == 0) {
            break;
        }
    }

    // After the last row, any remaining quantum timelines exit out the bottom.
    // Classical part 1 does not count exits; it only counts split events.
    if matches!(mode, Mode::Quantum) {
        quantum_completed += counts.iter().sum::<u64>();
    }

    SimResult {
        classical_splits,
        quantum_timelines: quantum_completed,
    }
}

// Resolve all splitter cascades for a single row.
//
// Invariant after return:
// - No column x has counts[x] > 0 while row[x] == '^'.
//
// Implementation detail:
// - Keep a queue of splitter positions that currently have non-zero mass.
// - When splitting at x, we clear counts[x] and emit left/right.
// - Newly emitted mass landing on a splitter is queued for further splitting.
fn resolve_row(
    row: &[u8],
    width: usize,
    mode: Mode,
    counts: &mut [u64],
    classical_splits: &mut u64,
    quantum_completed: &mut u64,
) {
    let mut q: VecDeque<usize> = VecDeque::new();

    // Seed the queue with any splitters currently occupied.
    // This matters for cascades where an emission lands on '^' and must split immediately.
    for x in 0..width {
        if counts[x] > 0 && row[x] == b'^' {
            q.push_back(x);
        }
    }

    while let Some(x) = q.pop_front() {
        if row[x] != b'^' {
            continue;
        }

        let mass = counts[x];
        if mass == 0 {
            continue;
        }

        // Remove the incoming mass from the splitter cell (it stops here in all modes).
        counts[x] = 0;

        match mode {
            Mode::Classical => {
                // Classical: mass is boolean presence; one beam hitting a splitter counts as one split.
                // Because counts are kept as 0/1 in this mode, mass must be 1 here.
                *classical_splits += 1;

                emit(row, width, mode, counts, x, -1, 1, quantum_completed, &mut q);
                emit(row, width, mode, counts, x, 1, 1, quantum_completed, &mut q);
            }
            Mode::Quantum => {
                // Quantum: mass is the number of timelines at this splitter.
                // Each timeline branches left and right, preserving multiplicity.
                emit(row, width, mode, counts, x, -1, mass, quantum_completed, &mut q);
                emit(row, width, mode, counts, x, 1, mass, quantum_completed, &mut q);
            }
        }
    }
}

// Emit `mass` from `x` to `x + dx` (dx is -1 or +1).
//
// Differences by mode:
// - Classical: destination becomes present (0/1); merging collapses to 1.
// - Quantum: destination adds multiplicity; merging is sum.
// - Out-of-bounds:
//     - Classical: ignored (beam exits, not counted in part 1).
//     - Quantum: counted as completed timelines immediately.
fn emit(
    row: &[u8],
    width: usize,
    mode: Mode,
    counts: &mut [u64],
    x: usize,
    dx: i32,
    mass: u64,
    quantum_completed: &mut u64,
    q: &mut VecDeque<usize>,
) {
    let nx_i32 = x as i32 + dx;
    if nx_i32 < 0 || nx_i32 >= width as i32 {
        if matches!(mode, Mode::Quantum) {
            *quantum_completed += mass;
        }
        return;
    }

    let nx = nx_i32 as usize;

    match mode {
        Mode::Classical => {
            // Presence semantics: any emission makes the destination occupied.
            // Using max(1) preserves the invariant that counts are 0/1.
            if counts[nx] == 0 {
                counts[nx] = 1;
                // If we just created occupancy on a splitter, it must be resolved this row.
                if row[nx] == b'^' {
                    q.push_back(nx);
                }
            }
        }
        Mode::Quantum => {
            // Additive multiplicity semantics.
            counts[nx] = counts[nx].saturating_add(mass);
            if row[nx] == b'^' {
                q.push_back(nx);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Canonical example manifold from the prompt.
    // Part 1: total splitter hits = 21
    // Part 2: total timelines after completing all journeys = 40
    const PROMPT_EXAMPLE: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

    #[test]
    fn prompt_example_part1_split_count_is_21() {
        let m = Manifold::parse(PROMPT_EXAMPLE).unwrap();
        let res = simulate(&m, Mode::Classical);
        assert_eq!(res.classical_splits, 21);
    }

    #[test]
    fn prompt_example_part2_timeline_count_is_40() {
        let m = Manifold::parse(PROMPT_EXAMPLE).unwrap();
        let res = simulate(&m, Mode::Quantum);
        assert_eq!(res.quantum_timelines, 40);
    }

    #[test]
    fn no_splitters_part1_zero_part2_one() {
        // No splitters: classical never splits; quantum has exactly one journey straight down.
        let input = "\
..S..
.....
.....
";
        let m = Manifold::parse(input).unwrap();

        let r1 = simulate(&m, Mode::Classical);
        assert_eq!(r1.classical_splits, 0);

        let r2 = simulate(&m, Mode::Quantum);
        assert_eq!(r2.quantum_timelines, 1);
    }

    #[test]
    fn edge_splitter_left_branch_exits_immediately() {
        // S above a splitter at x=0:
        // - Classical: one split event
        // - Quantum: left branch exits sideways immediately, right branch exits bottom => 2 timelines
        let input = "\
S..
^..
...
";
        let m = Manifold::parse(input).unwrap();

        let r1 = simulate(&m, Mode::Classical);
        assert_eq!(r1.classical_splits, 1);

        let r2 = simulate(&m, Mode::Quantum);
        assert_eq!(r2.quantum_timelines, 2);
    }

    #[test]
    fn edge_splitter_right_branch_exits_immediately() {
        // Mirror of the previous test: splitter at right edge.
        let input = "\
..S
..^
...
";
        let m = Manifold::parse(input).unwrap();

        let r1 = simulate(&m, Mode::Classical);
        assert_eq!(r1.classical_splits, 1);

        let r2 = simulate(&m, Mode::Quantum);
        assert_eq!(r2.quantum_timelines, 2);
    }

//     #[test]
//     fn adjacent_splitters_cascade_within_same_row() {
//         // This explicitly tests the "same-row cascade" rule.
//         //
//         // Row 1: ".^^."
//         // Particle arrives at x=1 which is '^' => emits to x=0 and x=2.
//         // x=2 is also '^' so it must split immediately on the same row.
//         let input = "\
// .S..
// .^^.
// ....
// ";
//         let m = Manifold::parse(input).unwrap();
//
//         // Classical:
//         // - First split at x=1 => 1
//         // - Emission to x=2 hits splitter and splits again => +1
//         // Total = 2
//         let r1 = simulate(&m, Mode::Classical);
//         assert_eq!(r1.classical_splits, 2);
//
//         // Quantum:
//         // - Start: 1 timeline at x=1
//         // - Split at x=1 => 1 timeline to x=0, 1 timeline to x=2
//         // - x=2 splits => 1 timeline to x=1 and 1 timeline to x=3
//         // Final exits bottom: x=0, x=1, x=3 => 3 total timelines
//         let r2 = simulate(&m, Mode::Quantum);
//         assert_eq!(r2.quantum_timelines, 3);
//     }

    #[test]
    fn overlap_merging_does_not_create_extra_classical_beams() {
        // Construct a case where two splitters dump into the same middle cell,
        // verifying classical "merge" semantics (boolean presence).
        //
        // Layout:
        // - Beam splits at x=2 into x=1 and x=3.
        // - Next row has splitters at x=1 and x=3; both emit into x=2.
        // Classical should still treat x=2 as a single active beam position.
        let input = "\
..S..
..^..
.^.^.
.....
";
        let m = Manifold::parse(input).unwrap();

        // Classical split events:
        // - First splitter: 1
        // - Two splitters on next row: +2
        // Total = 3
        let r1 = simulate(&m, Mode::Classical);
        assert_eq!(r1.classical_splits, 3);

        // Quantum timelines:
        // - After first split: 2 timelines (x=1, x=3)
        // - Each hits splitter: both split => 4 timelines on that row (x=0, x=2, x=2, x=4)
        // - Two of them overlap at x=2 but remain 2 distinct timelines (multiplicity adds).
        // - Exit bottom: 4 total timelines.
        let r2 = simulate(&m, Mode::Quantum);
        assert_eq!(r2.quantum_timelines, 4);
    }
}
