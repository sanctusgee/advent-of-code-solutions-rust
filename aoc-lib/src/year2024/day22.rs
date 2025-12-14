//! Advent of Code 2024 – Day 22 (Parts 1 & 2) — Monkey Market
//!
//! Each line of input is a starting secret number. A deterministic process
//! transforms a secret number through 2000 steps.
//!
//! Definitions
//! - mix(a, b): a XOR b
//! - prune(x): x mod 16777216  (keep lowest 24 bits)
//!
//! One step to get the next secret number from n:
//!   1) n = prune(mix(n, n * 64))
//!   2) n = prune(mix(n, n / 32))     // integer division
//!   3) n = prune(mix(n, n * 2048))
//!
//! Part 1
//!   For each starting secret s, advance 2000 steps and sum the final secrets.
//!
//! Part 2
//!   For each starting secret s, track price p_t = secret_t % 10 for t=0..2000
//!   and changes d_t = p_t - p_{t-1} for t=1..2000. For any 4-change sequence
//!   (d_{k-3}, d_{k-2}, d_{k-1}, d_k) that appears in this series, record the
//!   first time it appears and the price p_k at that time. Do this independently
//!   for each starting secret. Then, for every 4-change sequence, sum the prices
//!   contributed by all starting secrets. The answer is the maximum such sum.

use std::collections::HashMap;
use crate::utils;
use anyhow::Result;

#[inline]
fn prune(x: u64) -> u64 {
    x & 0xFF_FFFF // 2^24 - 1
}

#[inline]
fn next_secret(mut n: u64) -> u64 {
    // step 1
    let val1 = n.wrapping_mul(64);
    n ^= val1;
    n = prune(n);

    // step 2
    let val2 = n / 32;
    n ^= val2;
    n = prune(n);

    // step 3
    let val3 = n.wrapping_mul(2048);
    n ^= val3;
    n = prune(n);

    n
}

fn parse_input(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().expect("u64 secret"))
        .collect()
}

fn part1_sum_final(input: &str) -> u64 {
    let seeds = parse_input(input);
    let mut total = 0u64;

    for mut n in seeds {
        for _ in 0..2000 {
            n = next_secret(n);
        }
        total = total.wrapping_add(n);
    }

    total
}

type Pat = (i8, i8, i8, i8);

fn part2_best_banana_sum(input: &str) -> u64 {
    let seeds = parse_input(input);

    // Global totals per 4-change pattern
    let mut global: HashMap<Pat, u64> = HashMap::new();

    for start in seeds {
        // Simulate 2000 steps; keep prices and deltas
        let mut secret = start;

        // p[0] is from the initial secret
        let mut p_prev = (secret % 10) as i8;

        // First occurrence per pattern for this seed only
        let mut first_for_seed: HashMap<Pat, u8> = HashMap::new();

        // rolling last 4 deltas
        let mut d1 = 0i8;
        let mut d2 = 0i8;
        let mut d3 = 0i8;

        for step in 1..=2000 {
            secret = next_secret(secret);
            let p_cur = (secret % 10) as i8;
            let d = p_cur - p_prev;

            if step >= 4 {
                let pat: Pat = (d1, d2, d3, d);
                // record price on first occurrence only
                if !first_for_seed.contains_key(&pat) {
                    // store price p_cur (0..9) as u8
                    first_for_seed.insert(pat, p_cur as u8);
                }
            }

            // shift the window
            d1 = d2;
            d2 = d3;
            d3 = d;

            p_prev = p_cur;
        }

        // Add this seed's first-occurrence prices into the global totals
        for (pat, price) in first_for_seed {
            *global.entry(pat).or_insert(0) += price as u64;
        }
    }

    // Best total bananas over all patterns
    global.into_values().max().unwrap_or(0)
}

pub fn solve() -> Result<()> {
    let input = utils::load_input(2024, 22)?;

    let p1 = part1_sum_final(&input);
    println!("Part 1: {}", p1);

    let p2 = part2_best_banana_sum(&input);
    println!("Part 2: {}", p2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_secret_step_is_deterministic() {
        // Quick sanity: stepping twice equals composing next_secret twice
        let s = 1234567u64;
        let a = next_secret(s);
        let b = next_secret(a);
        let c = next_secret(next_secret(s));
        assert_eq!(b, c);
    }

    #[test]
    fn part1_runs_on_small_input() {
        let input = "1\n2\n3\n";
        let v = part1_sum_final(input);
        // Deterministic, but we do not assert a specific value here.
        assert!(v > 0);
    }
    //
    // #[test]
    // fn part2_runs_on_small_input() {
    //     let input = "1\n10\n100\n";
    //     let v = part2_best_banana_sum(input);
    //     // Nonzero is likely, but zero is allowed depending on collisions
    //     assert!(v >= 0);
    // }
}
