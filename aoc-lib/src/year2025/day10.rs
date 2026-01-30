// file: year2025/day10.rs

// Advent of Code
// Day 10: Factory
//
// https://adventofcode.com/2025/day/10

use anyhow::{anyhow, Result};
use crate::utils;

pub fn solve() -> Result<()> {
    let input = utils::load_input(2025, 10)?;

    let part1 = solve_part1(&input)?;
    let part2 = solve_part2(&input)?;

    println!("Day 10 / Year 2025");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

// -------------
// Part 1
// -------------------
//
// For each machine (line), compute the minimum number of button presses
// needed to reach the target light configuration.
// Total answer is the sum across machines.
//
fn solve_part1(input: &str) -> Result<impl std::fmt::Display> {
    let mut total: u64 = 0;

    for line in input.lines().filter(|l| !l.trim().is_empty()) {
        total += min_presses_for_machine(line)? as u64;
    }

    Ok(total)
}

fn solve_part2(_input: &str) -> Result<impl std::fmt::Display> {
    Ok(0)
}

// -------------------
// Core logic
// -------------------
//
// Each machine is a linear system over GF(2):
//
//   sum(x_j * button_j) = target   (mod 2)
//
// Press counts only matter mod 2, so each button is either pressed (1)
// or not pressed (0). We want the solution with minimum popcount(x).
//
// Strategy:
// - Build the system Ax = b over GF(2)
// - Gaussian eliminate to find:
//     * one particular solution x0
//     * a basis for the nullspace
// - Enumerate all combinations of the nullspace basis and choose
//   the solution with minimum popcount
//
fn min_presses_for_machine(line: &str) -> Result<u32> {
    // Parse diagram: [.#.#]
    let diagram = extract_between(line, '[', ']')
        .ok_or_else(|| anyhow!("missing diagram"))?;
    let n_lights = diagram.len();
    if n_lights > 128 {
        return Err(anyhow!("too many lights"));
    }

    // Target vector b (bit i = desired state of light i)
    let mut target: u128 = 0;
    for (i, c) in diagram.chars().enumerate() {
        if c == '#' {
            target |= 1u128 << i;
        }
    }

    // Parse buttons: each (...) becomes a bitmask over lights
    let mut buttons: Vec<u128> = Vec::new();
    let mut rest = line;

    while let Some((inside, after)) = extract_between_with_rest(rest, '(', ')') {
        let mut mask: u128 = 0;
        for idx in inside.split(',').map(|s| s.trim()) {
            let i: usize = idx.parse()?;
            mask |= 1u128 << i;
        }
        buttons.push(mask);
        rest = after;
    }

    let m = buttons.len();
    if m == 0 {
        return Err(anyhow!("no buttons"));
    }
    if m > 128 {
        return Err(anyhow!("too many buttons"));
    }

    // Build system: one equation per light
    // Each row is (variable_mask, rhs_bit)
    let mut rows: Vec<(u128, bool)> = Vec::new();

    for light in 0..n_lights {
        let mut vars: u128 = 0;
        for (j, &b) in buttons.iter().enumerate() {
            if ((b >> light) & 1) == 1 {
                vars |= 1u128 << j;
            }
        }
        let rhs = ((target >> light) & 1) == 1;
        rows.push((vars, rhs));
    }

    // Solve Ax = b over GF(2), minimizing popcount(x)
    let (x0, basis) = gaussian_elim_affine(rows, m)?;
    Ok(min_weight_solution(x0, &basis))
}

// -------------------
// Linear algebra over GF(2)
// -------------------
//
// Returns:
// - x0: one particular solution
// - basis: nullspace basis vectors
//
fn gaussian_elim_affine(
    mut rows: Vec<(u128, bool)>,
    n_vars: usize,
) -> Result<(u128, Vec<u128>)> {
    let mut pivot_row: Vec<Option<usize>> = vec![None; n_vars];
    let mut r = 0;

    // Forward elimination
    for col in 0..n_vars {
        // Find pivot
        let pivot = (r..rows.len()).find(|&i| ((rows[i].0 >> col) & 1) == 1);
        let Some(p) = pivot else { continue };

        rows.swap(r, p);
        pivot_row[col] = Some(r);

        let (mask, rhs) = rows[r];

        // Eliminate from all other rows
        for i in 0..rows.len() {
            if i != r && ((rows[i].0 >> col) & 1) == 1 {
                rows[i].0 ^= mask;
                rows[i].1 ^= rhs;
            }
        }

        r += 1;
        if r == rows.len() {
            break;
        }
    }

    // Check for inconsistency: 0 = 1
    for (mask, rhs) in &rows {
        if *mask == 0 && *rhs {
            return Err(anyhow!("no solution"));
        }
    }

    // Identify free variables
    let mut is_pivot = vec![false; n_vars];
    for (c, pr) in pivot_row.iter().enumerate() {
        if pr.is_some() {
            is_pivot[c] = true;
        }
    }

    let free_vars: Vec<usize> =
        (0..n_vars).filter(|&c| !is_pivot[c]).collect();

    // Particular solution: free vars = 0
    let mut x0: u128 = 0;
    for col in 0..n_vars {
        if let Some(row) = pivot_row[col] {
            if rows[row].1 {
                x0 |= 1u128 << col;
            }
        }
    }

    // Nullspace basis
    let mut basis: Vec<u128> = Vec::new();
    for &f in &free_vars {
        let mut v = 1u128 << f;
        for col in 0..n_vars {
            if let Some(row) = pivot_row[col] {
                if ((rows[row].0 >> f) & 1) == 1 {
                    v ^= 1u128 << col;
                }
            }
        }
        basis.push(v);
    }

    Ok((x0, basis))
}

// Enumerate all combinations of nullspace basis vectors
// and return the minimum popcount solution.
//
fn min_weight_solution(x0: u128, basis: &[u128]) -> u32 {
    let k = basis.len();
    let mut best = u32::MAX;

    // AoC inputs keep k small; brute force is fine.
    for mask in 0u32..(1u32 << k) {
        let mut x = x0;
        for i in 0..k {
            if ((mask >> i) & 1) == 1 {
                x ^= basis[i];
            }
        }
        best = best.min(x.count_ones());
    }

    best
}

// -------------------
// Small helpers
// -------------------

fn extract_between(s: &str, open: char, close: char) -> Option<String> {
    let start = s.find(open)?;
    let end = s[start + 1..].find(close)? + start + 1;
    Some(s[start + 1..end].to_string())
}

fn extract_between_with_rest<'a>(
    s: &'a str,
    open: char,
    close: char,
) -> Option<(String, &'a str)> {
    let start = s.find(open)?;
    let end = s[start + 1..].find(close)? + start + 1;
    Some((s[start + 1..end].to_string(), &s[end + 1..]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_total_is_7() {
        let input = r#"
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
"#;
        assert_eq!(solve_part1(input).unwrap().to_string(), "7");
    }
}
