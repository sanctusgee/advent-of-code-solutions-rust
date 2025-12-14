// file: year2025/day02.rs

// AoC 2025 Day 2 - Gift Shop
// =============================

// - Starting to think algorithmically about the problem. Might be over-engineering it a bit.
// - but I want to avoid brute-force scanning of entire ranges. And kinda like the challenge ;-)

// - The key insight is that valid numbers are formed by repeating a base pattern X.

// No brute-force scanning of entire ranges
// - Generate only valid repeat candidates
// - Single parse, single merge
// - Deduplicated
// - Works for both Part 1 and Part 2

use crate::utils::input::{is_in_sorted_ranges, merge_u64_ranges, parse_ranges_generic};
use crate::utils::load_input;
use crate::utils::numbers::num_digits;
use anyhow::Result;
use std::collections::HashSet;

pub fn solve() -> Result<()> {
    // Load raw input and parse ranges
    let input = load_input(2025, 2)?;
    let ranges = parse_ranges_generic(&input)?;

    // Merge ranges once for fast lookup
    // (also handles any overlapping ranges in input)
    // values become of the format (start, end), eg (100, 200)
    let merged = merge_u64_ranges(&ranges);

    // Solve both parts from the same merged data
    let part1 = solve_day02(&merged, false);
    let part2 = solve_day02(&merged, true);

    println!("Day 2 / Year 2025");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

// the parts are same logically with a flag to allow multi-repeats:
//  - allow_multi_repeat = false -> Part 1
//  - allow_multi_repeat = true  -> Part 2
fn solve_day02(merged: &[(u64, u64)], allow_multi_repeat: bool) -> u64 {
    let max_value = merged.iter().map(|&(_, end)| end).max().unwrap_or(0);
    let max_digits = num_digits(max_value) / 2;

    let mut seen = HashSet::new();
    let mut total = 0u64;

    for digits in 1..=max_digits {
        let base = 10u64.pow(digits);
        let start = base / 10;
        let end = base - 1;

        for x in start..=end {
            // Build X||X directly
            let mut repeated = x * base + x;
            // let mut count = 2;

            if repeated > max_value {
                continue;
            }

            loop {
                if is_in_sorted_ranges(merged, repeated) {
                    if seen.insert(repeated) {
                        total += repeated;
                    }
                }
                // Stop if only single repeat allowed (Part 1 case)
                if !allow_multi_repeat {
                    break;
                }

                match repeated.checked_mul(base).and_then(|v| v.checked_add(x)) {
                    Some(next) => {
                        repeated = next;
                        // count += 1;

                        if repeated > max_value {
                            break;
                        }
                    }
                    None => break,
                }
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::input::parse_ranges_generic;

    #[test]
    fn test_sum_repeated_ids_sample_input() {
        let input = "\
            11-22,\
            95-115,\
            998-1012,\
            1188511880-1188511890,\
            222220-222224,\
            1698522-1698528,\
            446443-446449,\
            38593856-38593862";

        let ranges = parse_ranges_generic(input).expect("Failed to parse test input");
        let merged = merge_u64_ranges(&ranges);

        let result = solve_day02(&merged, false);
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn test_sum_repeated_patterns_sample_input() {
        let input = "\
            11-22,\
            95-115,\
            998-1012,\
            1188511880-1188511890,\
            222220-222224,\
            1698522-1698528,\
            446443-446449,\
            38593856-38593862,\
            565653-565659,\
            824824821-824824827,\
            2121212118-2121212124";

        let ranges = parse_ranges_generic(input).expect("Failed to parse test input");
        let merged = merge_u64_ranges(&ranges);

        let result = solve_day02(&merged, true);
        assert_eq!(result, 4174379265);
    }
}