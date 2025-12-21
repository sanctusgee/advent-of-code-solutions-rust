// file: year2025/day06.rs
// Advent of Code 2025 - Day 06
// https://adventofcode.com/2025/day/6
//
// Problem Approach:
//  - Part 1: Read columns top-to-bottom (vertical problems)
//  - Part 2: Read columns right-to-left (horizontal problems)

use anyhow::{Result, Context, bail};
use crate::utils;

pub fn solve() -> Result<()> {
    let input = utils::load_input(2025, 6)?;

    println!("Day 6 / Year 2025");
    println!("Part 1: {}", solve_part1(&input)?);
    println!("Part 2: {}", solve_part2(&input)?);

    Ok(())
}

// Part 1: Read problems vertically (top-to-bottom columns)
//
// Example:
// ```
// 123 328  51 64
//  45 64  387 23
//   6 98  215 314
// *   +   *   +
// ```
// Vertical problems:
//  - 123 * 45 * 6 = 33,210
//  - 328 + 64 + 98 = 490
//  - 51 * 387 * 215 = 4,243,455
//  - 64 + 23 + 314 = 401
//  Grand total: 4,277,556
fn solve_part1(input: &str) -> Result<i64> {
    collect_inputs(input)?
        .iter()
        .enumerate()
        .map(|(idx, (nums, op))| apply_operator(nums, op, idx))
        .sum()
}

// Part 2: Read problems horizontally (right-to-left columns)
//
// Same input, different reading:
// - Rightmost: 4 + 431 + 623 = 1,058
// - Second: 175 * 581 * 32 = 3,253,600
// - Third: 8 + 248 + 369 = 625
// - Leftmost: 356 * 24 * 1 = 8,544
// Grand total: 3,263,827
fn solve_part2(input: &str) -> Result<i64> {
    collect_inputs_horizontal(input)?
        .iter()
        .enumerate()
        .map(|(idx, (nums, op))| apply_operator(nums, op, idx))
        .sum()
}

// Parse input into vertical columns (Part 1)
//
// Steps:
//  1. Split numeric rows from operator row
//  2. Parse numbers: [ [123, 328, 51, 64], [45, 64, 387, 23], [6, 98, 215, 314] ]
//  3. Extract operators: ["*", "+", "*", "+"]
//  4. Transpose to columns: [ (vec![123,45,6], "*"), (vec![328,64,98], "+"), ...]
fn collect_inputs(input: &str) -> Result<Vec<(Vec<i64>, String)>> {
    let lines: Vec<&str> = input.lines().collect();

    if lines.is_empty() {
        bail!("Input is empty");
    }

    // Extract numeric rows (lines starting with digits)
    // LOL - showing off my new found iterator chain skills
    // of course, could write as for..loop but, hey this is not Python or Java ;-)
    let rows: Vec<Vec<i64>> = lines
        .iter()
        .filter(|line| line.trim_start().starts_with(|c: char| c.is_ascii_digit()))
        .enumerate()
        .map(|(idx, line)| {
            line.split_whitespace()
                .map(|token| {
                    // added this just for learning purpose:
                    // Rust best practice - check/capture all errors
                    //of course, with AoC where the input is trusted tthis is not an issue
                    token.parse::<i64>()
                        .with_context(|| format!("Row {}: '{}' is not valid", idx, token))
                })
                .collect::<Result<_>>()
        })
        .collect::<Result<_>>()?;

    if rows.is_empty() {
        bail!("No numeric rows found");
    }

    // Extract operators from last line
    let ops: Vec<&str> = lines
        .last()
        .context("Missing operator line")?
        .split_whitespace()
        .collect();

    // AoC input is never empty but trying to coding best practice
    // not going for speed/benchmark here.
    if ops.is_empty() {
        bail!("Operator row is empty");
    }

    // Validate all rows have same column count
    let expected_cols = ops.len();
    for (idx, row) in rows.iter().enumerate() {
        if row.len() != expected_cols {
            bail!("Row {}: expected {} columns, got {}", idx, expected_cols, row.len());
        }
    }

    // Transpose rows → columns
    Ok((0..expected_cols)
        .map(|col| {
            (rows.iter().map(|row| row[col]).collect(), ops[col].to_owned())
        })
        .collect())
}

// Parse input reading right-to-left (Part 2)
//
// Process columns right-to-left, collecting vertical digits per column
// Blank columns separate problems
/// Parse input reading right-to-left (Part 2)
fn collect_inputs_horizontal(input: &str) -> Result<Vec<(Vec<i64>, String)>> {
    let lines: Vec<&str> = input.lines().collect();

    if lines.is_empty() {
        bail!("Input is empty");
    }

    let height = lines.len();
    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    // problems is the **accumulator** that collects all the parsed column problems
    // as we scan right-to-left.
    //
    // **Type:** Vec<(Vec<i64>, String)>
    //
    // Each "problem" is a tuple of:
    //      - Vec<i64> - the numbers in that problem
    //      - String - the operator for that problem
    let mut problems = Vec::new();
    let mut nums = Vec::new();
    let mut op = None;

    // Scan grid right-to-left, building problems from vertical digit sequences
    for col in (0..width).rev() {
        let mut digits = String::new();
        let mut blank = true;

        for (row, line) in lines.iter().enumerate() {
            match line.chars().nth(col).unwrap_or(' ') {
                '+' | '*' if row == height - 1 => {
                    op = line.chars().nth(col);
                    blank = false;
                }
                c if c.is_ascii_digit() => {
                    digits.push(c);
                    blank = false;
                }
                _ => {}
            }
        }

        if blank && !nums.is_empty() {
            // Hit separator - save current problem
            problems.push((nums, op.unwrap_or('+').to_string()));
            nums = Vec::new();
            op = None;
        } else if !digits.is_empty() {
            nums.push(digits.parse()?);
        }
    }

    // Don't forget leftmost problem
    if !nums.is_empty() {
        problems.push((nums, op.unwrap_or('+').to_string()));
    }

    Ok(problems)
}

// Apply operator to numbers: "+" sums, "*" multiplies
fn apply_operator(nums: &[i64], op: &str, col_idx: usize) -> Result<i64> {
    // Building Rust muscle here: I am removing the initial hard-coding of
    // "4 number rows and 1 operator column" in signature:
    // to allow reuse, eg what if input had more rows.
    // Make it dynamic and independent of input length
    match op {
        "+" => Ok(nums.iter().sum()),
        "*" => Ok(nums.iter().product()),
        _ => bail!("Column {}: unknown operator '{}'", col_idx, op),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
";

    #[test]
    fn test_part1() {
        // 123*45*6 + 328+64+98 + 51*387*215 + 64+23+314 = 4,277,556
        assert_eq!(solve_part1(EXAMPLE).unwrap(), 4_277_556);
    }

    #[test]
    fn test_part2() {
        // Reading right-to-left: 1,058 + 3,253,600 + 625 + 8,544 = 3,263,827
        assert_eq!(solve_part2(EXAMPLE).unwrap(), 3_263_827);
    }

    #[test]
    fn test_columns() {
        let result = collect_inputs(EXAMPLE).unwrap();
        assert_eq!(result[0].0, vec![123, 45, 6]);
        assert_eq!(result[0].1, "*");
    }

    #[test]
    fn test_invalid_number() {
        let input = "1 2\n3 foo\n+ +";
        assert!(collect_inputs(input).is_err());
    }
}