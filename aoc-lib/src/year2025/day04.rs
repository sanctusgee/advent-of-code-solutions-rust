// file: year2025/day04.rs

// ----Advent of Code 2025 - Day 4: Warehouse Paper Roll Management ----
// -- https://adventofcode.com/2025/day/4 --
// ---------------------------------------------------------------------

use anyhow::Result;
use crate::utils;

pub fn solve() -> Result<()> {
	// Load your input file.
	let input = utils::load_input(2025, 4)?;

	let part1 = solve_part1(&input)?;
	let part2 = solve_part2(&input)?;

	println!("Day 4 / Year 2025");
	println!("Part 1: {}", part1);
	println!("Part 2: {}", part2);

	Ok(())
}

pub fn solve_part1(input: &str) -> Result<impl std::fmt::Display> {
    let grid = parse_grid(input);
    Ok(count_accessible(&grid))
}

fn solve_part2(input: &str) -> Result<impl std::fmt::Display> {
    let mut grid = parse_grid(input);
    let mut total_removed = 0;

    // Iteratively remove accessible rolls until none remain
    loop {
        let accessible = find_accessible(&grid);
        if accessible.is_empty() {
            break;
        }

        // Remove all accessible rolls this round
        for (r, c) in accessible {
            grid[r][c] = '.';
            total_removed += 1;
        }
    }

    Ok(total_removed)
}

// Parse input into 2D grid of characters
fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

// Count how many rolls are currently accessible (< 4 neighbors)
fn count_accessible(grid: &[Vec<char>]) -> usize {
    find_accessible(grid).len()
}

// Find all paper rolls accessible by forklift (< 4 adjacent rolls)
fn find_accessible(grid: &[Vec<char>]) -> Vec<(usize, usize)> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut accessible = Vec::new();

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == '@' && count_adjacent(grid, r, c) < 4 {
                accessible.push((r, c));
            }
        }
    }

    accessible
}

// Count adjacent paper rolls in all 8 directions
// this is a simple algorithmic function which uses nested loops to check neighbors
fn count_adjacent(grid: &[Vec<char>], r: usize, c: usize) -> usize {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    let mut count = 0;

    // Check all 8 neighbors (cardinal + diagonal)
    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 {
                continue;  // Skip self
            }

            let nr = r as i32 + dr;
            let nc = c as i32 + dc;

            // Check bounds and if neighbor is a paper roll
            if nr >= 0 && nr < rows && nc >= 0 && nc < cols {
                if grid[nr as usize][nc as usize] == '@' {
                    count += 1;
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part1_example() {
        let result = solve_part1(EXAMPLE).unwrap();
        assert_eq!(result.to_string(), "13");
    }

    #[test]
    fn test_part2_example() {
        let result = solve_part2(EXAMPLE).unwrap();
        assert_eq!(result.to_string(), "43");
    }

    #[test]
    fn test_count_adjacent_corner() {
        let grid = parse_grid("@@.\n@..\n...");
        // Top-left @ has 2 adjacent rolls
        assert_eq!(count_adjacent(&grid, 0, 0), 2);
    }

    #[test]
    fn test_count_adjacent_surrounded() {
        let grid = parse_grid("@@@\n@@@\n@@@");
        // Center @ has 8 adjacent rolls
        assert_eq!(count_adjacent(&grid, 1, 1), 8);
    }

    #[test]
    fn test_find_accessible_isolated() {
        let grid = parse_grid("@..\n...\n..@");
        // Both isolated rolls are accessible
        let accessible = find_accessible(&grid);
        assert_eq!(accessible.len(), 2);
    }

    #[test]
    fn test_find_accessible_none() {
        let grid = parse_grid("@@@@\n@@@@\n@@@@\n@@@@");
        // No rolls accessible (all have 4+ neighbors)
        let accessible = find_accessible(&grid);
        assert_eq!(accessible.len(), 0);
    }
}