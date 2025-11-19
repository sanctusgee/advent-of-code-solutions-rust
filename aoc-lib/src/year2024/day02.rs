// file: src/year2024/day02.rs
// --------------- Advent of Code 2024, Day 2: Red Nosed Reports  --------------- //

use crate::utils;
use anyhow::Result;

/*
    Problem Description:
    The elves have been working hard to prepare the reports for the upcoming holiday season.
    However, the reports are not safe yet. The reports are stored as a list of levels, where each level is an integer.
    A report ONLY counts as safe if BOTH of the following are true:
        - The levels are either all increasing or all decreasing.
        - Any two adjacent levels differ by at least one and at most three.

    Part 1:
    Count the number of safe reports.

    Part 2:
    The elves have developed a problem dampener to make the reports safe.
    The problem dampener can remove ONLY ONE level from a report to make it safe.
    If a report is already safe, the dampener will not remove any level.
    If a report is unsafe, the dampener will remove one level to (try to) make it safe.
    If a report is still unsafe after removing any level, the dampener will not remove any level.
    Count the number of safe reports after using the problem dampener.
*/

pub fn solve() -> Result<()> {
    // Define the file path, read the file, and store the content in the input variable
    let input = utils::load_input(2024, 02)?;

    // Solve Part 1
    let result_part1 = solve_part1(&input);

    // D-R-Y: use the reports that were extracted in part 1 to solve part 2.
    // This is to avoid re-extracting the reports.
    let new_safe_count = solve_part2(&result_part1.0);

    println!("----------------------------------");
    println!("Part 1: Red Nosed Reports Completed!");
    println!("Part 2: Problem Dampener Activated!");
    println!("----------------------------------");
    println!("Stabilizing the reports...");
    println!("...the reports are now safe!");
    println!("----------------------------------");

    println!("Day 2 / Part 1 --> Safe report count BEFORE Problem Dampener: {:?}", result_part1.1);
    println!("Day 2 / Part 2 --> Safe reports count AFTER using Problem Dampener: {:?}", new_safe_count);

    Ok(())
}

fn solve_part1(input: &str) -> (Vec<Vec<i32>>, usize) {
    // Steps:
    // 1. Extract the reports from the input
    // 2. Count the number of safe reports
    // 3. Return the reports and the count
    //      - A reference to the reports will be used in part 2

    let reports = extract_reports(input);
    let count = count_valid_reports(&reports);
    (reports, count)
}

fn solve_part2(reports: &Vec<Vec<i32>>) -> usize {
    // STEPS:
    // 1. Use the problem dampener to get the new safe count

    // get ALL reports that are now safe after using the problem dampener
    let new_safe_count = use_problem_dampener(reports);
    new_safe_count
}

// BEGIN: Part 1
// Get the individual reports from the input
fn extract_reports(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|num_str| num_str.parse::<i32>().ok())
                .collect()
        })
        .collect()
}

// Check if the levels are increasing
fn is_increasing(levels: &[i32]) -> bool {
    levels.windows(2).all(|w| w[0] < w[1])
}

// Check if the levels are decreasing
fn is_decreasing(levels: &[i32]) -> bool {
    levels.windows(2).all(|w| w[0] > w[1])
}

// Check if any two adjacent levels differ by at least one and at most three
fn is_valid_difference(levels: &[i32]) -> bool {
    levels.windows(2).all(|w| {
        let diff = (w[1] - w[0]).abs();
        diff >= 1 && diff <= 3
        // (1..=3).contains(&diff)
    })
}

// Check if the report is safe
//
// "... a report ONLY counts as safe if BOTH of the following are true:
//      - The levels are either all increasing or all decreasing.
//      - Any two adjacent levels differ by at least one and at most three."
fn is_safe(levels: &[i32]) -> bool {
    // Check if the levels are either all increasing or all decreasing, AND the difference is valid
    (is_increasing(levels) || is_decreasing(levels)) && is_valid_difference(levels)
}

// Count the number of safe reports
fn count_valid_reports(reports: &[Vec<i32>]) -> usize {
    // count only the ones where final output is 'true' --> boolean table
    reports.iter().filter(|&report| is_safe(report)).count()
}
// END: Part 1

//---****************** BEGIN: Part 2 (Problem Dampener)
fn use_problem_dampener(reports: &Vec<Vec<i32>>) -> usize {
    let mut safe_count = 0;

    for (_i, levels) in reports.iter().enumerate() {
        print!("Report {}: {:?}:  --> ", _i + 1, levels);

        // First check if the report is already safe
        if is_safe(levels) {
            println!("Safe without removing any level.");
            safe_count += 1;
            continue;
        }

        // If not safe, try removing each level one by one
        let mut made_safe = false;
        for i in 0..levels.len() {
            let mut temp_levels = levels.clone();
            temp_levels.remove(i);

            // check if the report is safe after removing the level at index i
            if is_safe(&temp_levels) {
                println!("Safe by removing the level at index {}: {:?}", i, temp_levels);
                made_safe = true;
                safe_count += 1;
                break;
            }
        }

        if !made_safe {
            println!("Unsafe regardless of which level is removed.");
        }
    }

    safe_count
}
//---****************** END: Part 2

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_reports() {
        // Example test cases
        let test_cases = vec![
            vec![7, 6, 4, 2, 1], // Safe without removing any level
            vec![1, 2, 7, 8, 9], // Unsafe regardless of which level is removed
            vec![9, 7, 6, 2, 1], // Unsafe regardless of which level is removed
            vec![1, 3, 2, 4, 5], // Safe by removing the second level, 3
            vec![8, 6, 4, 4, 1], // Safe by removing the third level, 4
            vec![1, 3, 6, 7, 9], // Safe without removing any level
        ];

        use_problem_dampener(&test_cases);
    }
}
