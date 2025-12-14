// day01.rs

// Advent of Code 2025 - Day 1
// https://adventofcode.com/2025/day/1
use anyhow::{Result, Context};
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
    let mut pos = 50;  // Dial starts at position 50
    let mut hits = 0;  // Count final positions that land on 0

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() { continue; }

        let (dir, dist) = parse_move(line)?;
        // Jump directly to final position after full rotation
        pos = (pos + if dir == 'L' { -dist } else { dist }).rem_euclid(100);

        if pos == 0 { hits += 1; }
    }

    Ok(hits)
}

fn solve_part2(input: &str) -> Result<impl std::fmt::Display> {
    let mut pos: i8 = 50;
    let mut hits = 0i64;  // u64 would overflow on large inputs

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() { continue; }

        let (dir, dist) = parse_move(line)?;
        let step = if dir == 'L' { -1 } else { 1 };

        // Count every individual click that lands on 0 during rotation
        for _ in 0..dist {
            pos = (pos + step).rem_euclid(100);
            if pos == 0 { hits += 1; }
        }
    }

    Ok(hits)
}

// Parse "L30" -> ('L', 30)
fn parse_move(line: &str) -> Result<(char, i32)> {
    let dir = line.chars().next().context("Empty line")?;
     // as per comments online --> always trust AoC input is always valid; be less defensive
    let dist = line[1..].parse()?;
    Ok((dir, dist))
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