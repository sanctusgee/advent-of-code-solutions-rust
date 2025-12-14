// day01.rs

// Advent of Code 2025 - Day 1
// https://adventofcode.com/2025/day/1
use anyhow::{Result, anyhow};
use crate::utils;


pub fn solve() -> Result<()> {
// Load your input file.
	let input = utils::load_input(2025, 1)?;

	let part1 = solve_part1(&input)?;
	let part2 = solve_part2(&input)?;

	println!("Day 1 / Year 2025");
	println!("Part 1: {}", part1);
	println!("Part 2: {}", part2);

	Ok(())
}

pub fn solve_part1(input: &str) -> Result<impl std::fmt::Display> {
    let mut start_pos: i32 = 50;      // starts at 50
    let mut zero_hits: u32 = 0;     // count number of times the dial lands on 0

    // the input file is multi-line so we process it line by line
    for (idx, raw) in input.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        if line.len() < 2 {
            return Err(anyhow!("Line {} too short: {}", idx + 1, line));
        }

        // get the direction and distance from the line, eg "L30" becomws ("L", 30)
        let (direction, dist_str) = line.split_at(1);
        let dist: i32 = dist_str.parse()
            .map_err(|e| anyhow!("Line {} invalid distance '{}': {}", idx + 1, dist_str, e))?;

        // this part is just moving the dial left or right and wrapping around at 0 and 99
        // so we use rem_euclid to handle negative wrap-around correctly
        // rem_euclid(100) is the Rust  of Python's % operator for positive modulus
        let d = dist.rem_euclid(100); // normalize any large distance

        // Logic: if moving left, subtract distance; if right, add distance
        start_pos = match direction {
            "L" => (start_pos - d).rem_euclid(100),
            "R" => (start_pos + d).rem_euclid(100),
            _ => return Err(anyhow!("Line {} invalid direction '{}'", idx + 1, direction)),
        };

        // increment by 1 if we hit zero
        if start_pos == 0 {
            zero_hits += 1;
        }
    }

    Ok(zero_hits)
}


// Part 2: count *every* click that lands on 0 during rotations (method 0x434C49434B).
fn solve_part2(input: &str) -> Result<impl std::fmt::Display> {
    let mut start_pos: i32 = 50;
    let mut zero_hits: i64 = 0;

    for (idx, raw) in input.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        if line.len() < 2 {
            return Err(anyhow!("Line {} too short: {}", idx + 1, line));
        }

        let (direction, dist_str) = line.split_at(1);
        let dist: i32 = dist_str
            .parse()
            .map_err(|e| anyhow!("Line {} invalid distance `{}`: {}", idx + 1, dist_str, e))?;

        // Count how many times we pass through position 0 during this move
        // We need to simulate each step to count every time we land on 0
        let step_dir = match direction {
            "R" => 1,
            "L" => -1,
            _ => return Err(anyhow!("Line {} invalid direction `{}`", idx + 1, direction)),
        };

        for _ in 0..dist {
            start_pos = (start_pos + step_dir).rem_euclid(100);
            if start_pos == 0 {
                zero_hits += 1;
            }
        }
    }

    Ok(zero_hits)
}



#[test]
fn part1_example_password_is_3() {
    let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82\n";
    let ans = solve_part1(input).unwrap().to_string();
    assert_eq!(ans, "3");
}

#[test]
fn part2_example_password_is_6() {
    let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82\n";
    let ans = solve_part2(input).unwrap().to_string();
    assert_eq!(ans, "6");
}