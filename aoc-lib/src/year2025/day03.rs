// file: year2025/day03.rs

// AoC 2025 Day 3 - Joltage Banks
// =============================

use anyhow::Result;
use crate::utils;


// Example template.

pub fn solve() -> Result<()> {
// Load your input file.
	let input = utils::load_input(2025, 3)?;

	let part1 = solve_part1(&input)?;
	let part2 = solve_part2(&input)?;

	println!("Day 3 / Year 2025");
	println!("Part 1: {}", part1);
	println!("Part 2: {}", part2);

	Ok(())
}

// Given a line of digit characters (0–9),
// returns the maximum 2-digit joltage that can be formed
// by choosing two digits in order.
// Returns None if the line is invalid or too short.
fn solve_part1(input: &str) -> Result<impl std::fmt::Display> {
    let mut total = 0u64;

    for line in input.lines() {
        let bytes = line.trim().as_bytes();

        // Need at least 2 digits to form a 2-digit number
        if bytes.len() < 2 {
            continue;
        }

        // Right-to-left scan: best ones-digit is always max seen so far to the right
        let last_idx = bytes.len() - 1;
        let mut max_right = bytes[last_idx] - b'0';  // Start with rightmost digit
        // starting to use idiomatic Rust style here ;-)
        // could write as let mut best: u16 = 0;
        let mut best = 0u16;

        // Each position is a potential tens-digit, paired with best ones-digit to its right
        for &b in bytes[..last_idx].iter().rev() {
            let digit = b - b'0';
            let jolts = digit as u16 * 10 + max_right as u16;  // Form 2-digit number

            best = best.max(jolts);        // Track best joltage for this bank
            max_right = max_right.max(digit);  // Update max available for next tens-digit
        }

        total += best as u64;
    }

    Ok(total)
}


fn solve_part2(input: &str) -> Result<impl std::fmt::Display> {
    let mut total = 0u128;  // 12-digit numbers need bigger storage

    for line in input.lines() {
        let bytes = line.trim().as_bytes();
        if bytes.len() < 12 {
            continue;
        }

        let to_remove = bytes.len() - 12;

        // Monotonic decreasing stack: keep largest digits, remove smaller ones
        let mut stack = Vec::with_capacity(12);
        let mut removals_left = to_remove;

        for &b in bytes {
            // Pop smaller digits while we have removals left
            while !stack.is_empty()
                && removals_left > 0
                && stack.last().unwrap() < &b
            {
                stack.pop();
                removals_left -= 1;
            }
            stack.push(b);
        }

        // If we still have removals left, trim from the end
        stack.truncate(12);

        // Convert stack to number
        let num_str = std::str::from_utf8(&stack).unwrap();
        let jolts: u128 = num_str.parse().unwrap();
        total += jolts;
    }

    Ok(total)
}
