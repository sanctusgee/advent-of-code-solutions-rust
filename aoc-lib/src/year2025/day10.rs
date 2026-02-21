// file: year2025/day10.rs

// Advent of Code
// Day 10: Factory
//
// https://adventofcode.com/2025/day/10

use anyhow::{anyhow, Result};
use crate::utils;
use std::collections::HashMap;

pub fn solve() -> Result<()> {
    let input = utils::load_input(2025, 10)?;

    println!("Day 10 / Year 2025");
    println!("Part 1: {}", solve_part1(&input)?);
    println!("Part 2: {}", solve_part2(&input)?);

    Ok(())
}

// ================= Part 1 =================
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
fn solve_part1(input: &str) -> Result<u64> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(min_presses_for_machine)
        .map(|r| r.map(|x| x as u64))
        .try_fold(0, |a, b| Ok(a + b?))
}

fn min_presses_for_machine(line: &str) -> Result<u32> {
    let diagram = extract_between(line, '[', ']')
        .ok_or_else(|| anyhow!("missing diagram"))?;

    let mut target: u128 = 0;
    for (i, c) in diagram.chars().enumerate() {
        if c == '#' {
            target |= 1u128 << i;
        }
    }

    let buttons = parse_buttons(line)?;

    let rows = (0..diagram.len())
        .map(|light| {
            let mut vars = 0u128;
            for (j, &b) in buttons.iter().enumerate() {
                if (b >> light) & 1 == 1 {
                    vars |= 1u128 << j;
                }
            }
            (vars, (target >> light) & 1 == 1)
        })
        .collect();

    let (x0, basis) = gaussian_elim_affine(rows, buttons.len())?;
    Ok(min_weight_solution(x0, &basis))
}

// ================= Part 2 =================
//
// Joltage mode:
// - Ignore the light diagram.
// - Each button press adds +1 to listed counters.
// - Need minimum total presses to reach exact target vector.
//
// Correct fast strategy:
// Use parity constraints + halving recursion.
// Any solution vector x (press counts) satisfies A x = target over integers.
// Mod 2, that implies A (x mod 2) = (target mod 2) over GF(2).
//
// So we:
// 1) Solve A p = (target mod 2) over GF(2) to get all parity solutions p.
// 2) For each parity p, subtract 1 from affected counters for each pressed button in p.
// 3) Now all counters must be even; divide by 2 and recurse.
// 4) Total presses = popcount(p) + 2 * recurse(half_target).
//
// Memoize by target vector to stay fast.
//
fn solve_part2(input: &str) -> Result<u64> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(min_presses_part2)
        .map(|r| r.map(|x| x as u64))
        .try_fold(0, |a, b| Ok(a + b?))
}

fn min_presses_part2(line: &str) -> Result<u32> {
    let target = parse_jolts(line)?;
    let buttons = parse_buttons(line)?;

    // Build template: which buttons affect each counter
    let rows_template: Vec<u128> = (0..target.len())
        .map(|i| {
            let mut v = 0;
            for (j, &b) in buttons.iter().enumerate() {
                if (b >> i) & 1 == 1 {
                    v |= 1u128 << j;
                }
            }
            v
        })
        .collect();

    let mut pattern_cache: HashMap<u128, Vec<u128>> = HashMap::new();
    let mut memo: HashMap<Vec<i32>, Option<u32>> = HashMap::new();

    solve_rec(
        target,
        &rows_template,
        buttons.len(),
        &mut pattern_cache,
        &mut memo,
    )
    .ok_or_else(|| anyhow!("no solution"))
}

fn solve_rec(
    target: Vec<i32>,
    rows_template: &[u128],
    n_vars: usize,
    pattern_cache: &mut HashMap<u128, Vec<u128>>,
    memo: &mut HashMap<Vec<i32>, Option<u32>>,
) -> Option<u32> {
    if let Some(&r) = memo.get(&target) {
        return r;
    }

    if target.iter().all(|&x| x == 0) {
        memo.insert(target, Some(0));
        return Some(0);
    }

    // pattern bits: which counters are odd
    let pattern = target.iter().enumerate().fold(0u128, |acc, (i, &v)| {
        if v & 1 == 1 { acc | (1u128 << i) } else { acc }
    });

    // Compute parity solutions if needed
    if !pattern_cache.contains_key(&pattern) {
        let rows: Vec<_> = rows_template
            .iter()
            .enumerate()
            .map(|(i, &vars)| (vars, (pattern >> i) & 1 == 1))
            .collect();

        let sols = match gaussian_elim_affine(rows, n_vars) {
            Ok((x0, basis)) => {
                let k = basis.len();
                let mut out = Vec::with_capacity(1 << k);
                for mask in 0..(1u32 << k) {
                    let mut x = x0;
                    for i in 0..k {
                        if (mask >> i) & 1 == 1 {
                            x ^= basis[i];
                        }
                    }
                    out.push(x);
                }
                out.sort_by_key(|x| x.count_ones());
                out
            }
            Err(_) => Vec::new(),
        };

        pattern_cache.insert(pattern, sols);
    }

    let sols = pattern_cache.get(&pattern).unwrap().clone();
    if sols.is_empty() {
        memo.insert(target, None);
        return None;
    }

    let mut best: Option<u32> = None;

    'outer: for x in sols {
        let parity_cost = x.count_ones() as u32;

        let mut after = target.clone();

        // subtract parity presses
        for i in 0..after.len() {
            let dec = (x & rows_template[i]).count_ones() as i32;
            after[i] -= dec;
            if after[i] < 0 || (after[i] & 1) != 0 {
                continue 'outer;
            }
        }

        // divide by 2
        for v in &mut after {
            *v /= 2;
        }

        if let Some(sub) = solve_rec(after, rows_template, n_vars, pattern_cache, memo) {
            let cost = parity_cost + 2 * sub;

            // Keep Rust inference happy.
            best = Some(best.map_or(cost, |b: u32| b.min(cost)));
        }
    }

    memo.insert(target, best);
    best
}

// ================= Linear algebra =================

fn gaussian_elim_affine(
    mut rows: Vec<(u128, bool)>,
    n_vars: usize,
) -> Result<(u128, Vec<u128>)> {
    let mut pivot = vec![None; n_vars];
    let mut r = 0;

    for c in 0..n_vars {
        if let Some(p) = (r..rows.len()).find(|&i| (rows[i].0 >> c) & 1 == 1) {
            rows.swap(r, p);
            pivot[c] = Some(r);

            let (mask, rhs) = rows[r];
            for i in 0..rows.len() {
                if i != r && (rows[i].0 >> c) & 1 == 1 {
                    rows[i].0 ^= mask;
                    rows[i].1 ^= rhs;
                }
            }

            r += 1;
        }
    }

    for (m, rhs) in &rows {
        if *m == 0 && *rhs {
            return Err(anyhow!("no solution"));
        }
    }

    let mut x0 = 0;
    for c in 0..n_vars {
        if let Some(row) = pivot[c] {
            if rows[row].1 {
                x0 |= 1u128 << c;
            }
        }
    }

    let mut basis = Vec::new();
    for f in 0..n_vars {
        if pivot[f].is_none() {
            let mut v = 1u128 << f;
            for c in 0..n_vars {
                if let Some(row) = pivot[c] {
                    if (rows[row].0 >> f) & 1 == 1 {
                        v ^= 1u128 << c;
                    }
                }
            }
            basis.push(v);
        }
    }

    Ok((x0, basis))
}

fn min_weight_solution(x0: u128, basis: &[u128]) -> u32 {
    let mut best: u32 = u32::MAX;

    for mask in 0..(1u32 << basis.len()) {
        let mut x = x0;
        for i in 0..basis.len() {
            if (mask >> i) & 1 == 1 {
                x ^= basis[i];
            }
        }
        best = best.min(x.count_ones());
    }

    best
}

// ================= Parsing helpers =================

fn parse_buttons(line: &str) -> Result<Vec<u128>> {
    let mut out = Vec::new();
    let mut rest = line;

    while let Some((inside, after)) = extract_between_with_rest(rest, '(', ')') {
        let mut mask = 0u128;
        for s in inside.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()) {
            mask |= 1u128 << s.parse::<usize>()?;
        }
        out.push(mask);
        rest = after;
    }

    if out.is_empty() {
        return Err(anyhow!("no buttons"));
    }
    Ok(out)
}

fn parse_jolts(line: &str) -> Result<Vec<i32>> {
    let j = extract_between(line, '{', '}')
        .ok_or_else(|| anyhow!("missing jolts"))?;

    j.split(',')
        .map(|s| s.trim().parse::<i32>().map_err(Into::into))
        .collect()
}

fn extract_between(s: &str, a: char, b: char) -> Option<String> {
    let i = s.find(a)?;
    let j = s[i + 1..].find(b)? + i + 1;
    Some(s[i + 1..j].to_string())
}

fn extract_between_with_rest<'a>(s: &'a str, a: char, b: char) -> Option<(String, &'a str)> {
    let i = s.find(a)?;
    let j = s[i + 1..].find(b)? + i + 1;
    Some((s[i + 1..j].to_string(), &s[j + 1..]))
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
        assert_eq!(solve_part1(input).unwrap(), 7);
    }

    #[test]
    fn example_part2_total_is_33() {
        let input = r#"
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
"#;
        assert_eq!(solve_part2(input).unwrap(), 33);
    }
}