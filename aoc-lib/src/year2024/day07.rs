use itertools::Itertools;
use once_cell::sync::Lazy;
use std::fmt::Write;
use crate::utils;
use anyhow::Result;

// Cache operators with lazy operators
// static OPERATORS: Lazy<Vec<&str>> = Lazy::new(|| vec!["+", "*"]);    // part 1 operators:
static OPERATORS: Lazy<Vec<&str>> = Lazy::new(|| vec!["+", "*", "||"]); // part 2 Operators:

pub fn solve() -> Result<()> {
    let file = utils::load_input(2024, 7)?;
    let lines: Vec<String> = file.lines().map(|s| s.to_string()).collect();
    let input = utils::parse_lines_with_delimiter(&lines, ":")?;

    solve_part1(input)?;
    Ok(())
}


fn solve_part1(input_data: Vec<(u64, Vec<u64>)>) -> Result<()> {
    let mut valid_entries: Vec<u64> = Vec::new();

    println!("Generating valid inputs...");

    for (expected_value, test_numbers) in input_data {
        let operations = generate_operator_permutations(&test_numbers);

        // find the ones that match the expected value
        let calibrated: Vec<u64> = evaluate_and_filter(&operations, expected_value);
        valid_entries.extend(&calibrated);

        // for every equation, if the value part of the equation matches the expected value
        //  then print the equation --> numbers and operators
        print_matching_expressions(&operations, &calibrated, expected_value);
    }
    print_calibration_summary(&valid_entries);
    Ok(())
}

// Generates all possible combinations for operators (+, *, ||) for a given number of integers
fn generate_operator_permutations(list_of_numbers: &[u64]) -> Vec<String> {

    let n = list_of_numbers.len(); // Get the count of numbers
    let mut ops_list = Vec::with_capacity(OPERATORS.len().pow(n as u32 - 1)); // Pre-size results

    for combination in (0..n - 1)
        .map(|_| OPERATORS.iter())
        .multi_cartesian_product()
    {
        // 1. By pre-sizing with Vec::with_capacity to eliminate resizing overhead
        // 2.   instead of format!() - use write! with a String buffer for improved efficiency:
        let mut expression = String::with_capacity(n * 3);
        for (i, op) in combination.iter().enumerate() {
            write!(expression, "{} {} ", list_of_numbers[i], op).unwrap();
        }
        write!(expression, "{}", list_of_numbers[n - 1]).unwrap();
        ops_list.push(expression);
    }
    ops_list
}

fn compute_expression_result(expression: &str) -> Result<u64, String> {
    let mut value = 0;
    let mut current_op = "+";

    for token in expression.split_whitespace() {
        if let Ok(num) = token.parse::<u64>() {
            match current_op {
                "+" => value += num,
                "*" => value *= num,
                "||" => {
                    let concat = format!("{}{}", value, num);
                    value = concat.parse::<u64>()
                        .map_err(|e| format!("Failed to parse '{}': {}", concat, e))?;
                }
                _ => return Err(format!("Unknown operator '{}'", current_op)),
            }
        } else {
            match token {
                "+" | "*" | "||" => current_op = token,
                _ => return Err(format!("Invalid token '{}'", token)),
            }
        }
    }

    Ok(value)
}

// ****************************************************************************************
// Rust best practice: clear separation between business logic and output.
// Instead of mixing logic, eg summing, filtering, and printing, separate these concerns:
//
//          Modularization is key to writing clean, maintainable code. Oh, yeah!!
// ****************************************************************************************

fn evaluate_and_filter(
    operations: &[String],
    expected_value: u64,
) -> Vec<u64> {
    operations
        .iter()
        .filter_map(|op| match compute_expression_result(op) {
            Ok(result) if result == expected_value => Some(result),
            _ => None,
        })
        .unique()
        .collect()
}

// ------ ** BEGIN Printing Functions **------------------
#[allow(unused)]
fn print_matching_expressions(
    operations: &[String],
    calibrated: &Vec<u64>,
    expected_value: u64,
) {
    for op in operations {
        // Honor **result**, daughter of evaluate_expression,
        // and grant her the rightful crown of 'u64',
        // not that grubby, two-faced 'Result<u64, String>' imposter.
        //
        // Let us cleanse this sauce-stained loop,
        // elevate result to her noble form, and cast out the shadow of unhandled errors.
        if let Ok(result) = compute_expression_result(op) {
            if calibrated.contains(&result) {
                // println!("Expected value: {}", expected_value);
                // print!("Found valid combo!" );
                println!("Valid combo! {:?} --> {} ", op, expected_value);
            }
        }
    }
}

fn print_calibration_summary(valid_items: &[u64]) {
    println!();
    println!("List of valid numbers: {:?}", valid_items);
    // get the sum of all the valid numbers
    println!();
    println!("Total Calibration Result is: {}", valid_items.iter().sum::<u64>());
}

// -----------------------------------------------//
// -------------** BEGIN Tests **------------------
// -----------------------------------------------//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_190() {
        // Test case: 10 19
        let input = "10 * 19"; // Should be 190
        let expected = 190;
        assert_eq!(compute_expression_result(input).unwrap(), expected);

        let incorrect = "10 + 19"; // This gives 29
        assert_ne!(
            compute_expression_result(incorrect).unwrap(),
            expected
        );
    }

    #[test]
    fn test_3267() {
        // Test case: 81 40 27
        let expected = 3267;

        let case1 = "81 + 40 * 27"; // equals 3267
        let case2 = "81 * 40 + 27"; // also 3267

        assert_eq!(
            compute_expression_result(case1).unwrap(),
            expected
        );
        assert_eq!(
            compute_expression_result(case2).unwrap(),
            expected
        );

        let incorrect1 = "81 + 40 + 27"; // 148
        let incorrect2 = "81 * 40 * 27"; // big
        assert_ne!(
            compute_expression_result(incorrect1).unwrap(),
            expected
        );
        assert_ne!(
            compute_expression_result(incorrect2).unwrap(),
            expected
        );
    }

    #[test]
    fn test_292() {
        let input = "11 + 6 * 16 + 20"; // 292
        let expected = 292;
        assert_eq!(
            compute_expression_result(input).unwrap(),
            expected
        );

        let incorrect1 = "11 + 6 + 16 + 20"; // 53
        let incorrect2 = "11 * 6 * 16 * 20"; // huge
        let incorrect3 = "11 * 6 + 16 + 20"; // 102

        assert_ne!(
            compute_expression_result(incorrect1).unwrap(),
            expected
        );
        assert_ne!(
            compute_expression_result(incorrect2).unwrap(),
            expected
        );
        assert_ne!(
            compute_expression_result(incorrect3).unwrap(),
            expected
        );
    }

    #[test]
    fn test_and_symbol() {
        // Test case: 17 || 8 + 14 = 192 (i.e., 17 concatenated with 8 = 178, 178 + 14 = 192)
        let input = "17 || 8 + 14";
        let expected = 192;

        assert_eq!(
            compute_expression_result(input).unwrap(),
            expected
        );

        let incorrect1 = "17 + 8 || 14"; // 25 || 14 = 2514
        let incorrect2 = "17 + 8 * 14";  // 17 + 112 = 129
        let incorrect3 = "11 * 6 || 16 + 20"; // probably doesn't hit 192

        assert_ne!(
            compute_expression_result(incorrect1).unwrap(),
            expected
        );
        assert_ne!(
            compute_expression_result(incorrect2).unwrap(),
            expected
        );
        assert_ne!(
            compute_expression_result(incorrect3).unwrap(),
            expected
        );
    }
}

