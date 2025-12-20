// file: year2025/day05.rs

// Advent of Code 2025 Day 5
// https://adventofcode.com/2025/day/5

// Problem Structure:
// 	- Two distinct sections in input (separated by blank line)
// 	- First section: ranges of valid values
// 	- Second section: individual values to check
//  - Question: How many individual values fall within ANY range?


use anyhow::{Context, Result};
use crate::utils;


pub fn solve() -> Result<()> {
// Load your input file.
	let input = utils::load_input(2025, 5)?;

	let part1 = solve_part1(&input)?;
	let part2 = solve_part2(&input)?;

	println!("Day 5 / Year 2025");
	println!("Part 1: {}", part1);
	println!("Part 2: {}", part2);

	Ok(())
}

fn solve_part1(input: &str) -> Result<impl std::fmt::Display> {
    let (ranges, values) = parse_input(input)?;

    // Count how many values fall within any range
    let count = values
        .iter()
        .filter(|&val| is_in_any_range(*val, &ranges))
        .count();

    Ok(count)
}
fn solve_part2(input: &str) -> Result<impl std::fmt::Display> {
    let (ranges, _) = parse_input(input)?;

    // Merge overlapping ranges
    let merged = merge_ranges(ranges);

    // Count total IDs in merged ranges
    let total: i64 = merged
        .iter()
        .map(|(start, end)| end - start + 1)
        .sum();

    Ok(total)
}



// parse input into ranges and values - returns (ranges, values)
// they are separated by a blank line
// ranges are in the format "a-b" (one per line)
// values are one per line
fn parse_input(input: &str) -> Result<(Vec<(i64, i64)>, Vec<i64>)> {
	let (ranges_str, values_str) = input
		.split_once("\n\n")
		.context("Input must have blank line between ranges and values")?;

    let ranges = ranges_str
        .lines()
        .filter_map(|l| l.split_once('-'))
        .filter_map(|(a, b)| Some((a.parse().ok()?, b.parse().ok()?)))
        .collect();

    let values = values_str.lines().filter_map(|l| l.parse().ok()).collect();

    Ok((ranges, values))
}

// Check if a value falls within any range (inclusive)
fn is_in_any_range(val: i64, ranges: &[(i64, i64)]) -> bool {
    ranges.iter().any(|(start, end)| val >= *start && val <= *end)
}


// Merge overlapping ranges into non-overlapping ranges
fn merge_ranges(mut ranges: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    if ranges.is_empty() {
        return vec![];
    }

    // Sort by start position
    ranges.sort_by_key(|r| r.0);

    let mut merged = vec![ranges[0]];

    for (start, end) in ranges.into_iter().skip(1) {
        let last_idx = merged.len() - 1;
        let (last_start, last_end) = merged[last_idx];

        // If current range overlaps or touches the last merged range
        if start <= last_end + 1 {
            // Extend the last range
            merged[last_idx] = (last_start, last_end.max(end));
        } else {
            // No overlap, add as new range
            merged.push((start, end));
        }
    }

    merged
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32";

	#[test]
    fn test_part1_example() {
        let result = solve_part1(EXAMPLE).unwrap();
        assert_eq!(result.to_string(), "3");
    }

    #[test]
    fn test_is_in_any_range() {
        let ranges = vec![(3, 5), (10, 14)];
        assert!(!is_in_any_range(1, &ranges));
        assert!(is_in_any_range(5, &ranges));
        assert!(!is_in_any_range(8, &ranges));
        assert!(is_in_any_range(11, &ranges));
    }
}