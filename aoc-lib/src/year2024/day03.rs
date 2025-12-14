// file: src/year2024/day03.rs
// --------------- Advent of Code 2024, Day 3: Mull It Over  --------------- //
use crate::utils;
use anyhow::Result;
use regex::Regex;
use std::num::ParseIntError;

/*
    Problem Description:
        - Part 1: Get the sum of the products of the two numbers in each mul(i32, i32) pattern
        - Part 2:
*/

pub fn solve() -> Result<()> {
    // Define the file path, read the file, and store the content in the input variable
    let input = utils::load_input(2024, 3)?;

    println!();
    println!("---------**----- Day 3 ----**----------");

    let result_part1 = solve_part1(&input)?;
    println!("Day 3 / Part 1 --> Sum of Multiplication results: {:?}", result_part1);

    let result_part2 = solve_part2(&input)?;
    println!("Day 3 / Part 2 --> Removing don't() instructions...{:?}", result_part2);

    Ok(())
}

fn solve_part1(input: &str) -> Result<i32> {
    let products = extract_and_multiply(&input)?;
    let sum = add_products(products);
    Ok(sum)
}
fn solve_part2(input: &str) -> Result<i32> {
    let result = bypass_dont_instructions(input)?;
    // println!("Result: {:?}", result);
    let new_products = extract_and_multiply(&result)?;
    // println!("{:?}", new_products);
    let sum = add_products(new_products);
    Ok(sum)
}


// BEGIN: Part 1
fn extract_and_multiply(input: &str) -> Result<Vec<Option<i32>>, ParseIntError> {
/*
    Define the regex pattern: no space between the numbers
    Pattern:
    The regex pattern r"mul\(\d+,\s*\d+\)" matches the string mul( followed by two
    integers separated by a comma and optional spaces, and then a closing ).
        mul\( matches the literal string mul(.
        \d+ matches one or more digits.
        ,\s* matches a comma followed by zero or more whitespace characters.
        \d+ matches one or more digits.
        \) matches the closing parenthesis ).
    Ensures that only valid mul(i32, i32) patterns are captured, ignoring any malformed occurrences
 */
     // Find all matches
    let re = Regex::new(r"mul\(\d+,\s*\d+\)").unwrap();

    // Find all matches and extract products
    // sample vec_of_mul_strings:
    //  [
    //      "mul(427,266)", "mul(287,390)", "mul(398,319)", "mul(613,600)",
    //      "mul(189,242)", "mul(908,64)", "mul(483,371)",
    //  ]
    // see https://docs.rs/regex/1.5.4/regex/struct.Regex.html#method.find_iter
    re.find_iter(input)
        .map(|mat| {
            let matched_str = mat.as_str();
            let trimmed = &matched_str[4..matched_str.len() - 1]; //sample: "427,266"
            let numbers: Vec<&str> = trimmed.split(',').collect(); //sample: ["427", "266"]
            let num1: i32 = numbers[0].parse()?; //sample: 427
            let num2: i32 = numbers[1].parse()?; // sample: 266
            // this checks for, and handles potential overflow on multiplication
            Ok(num1.checked_mul(num2)) // sample: 113582
        })
        .collect()
}

fn add_products(products: Vec<Option<i32>>) -> i32 {
    products.iter().filter_map(|&opt| opt).sum()
}

// BEGIN: Part 2
// Remove the don't() instructions, replacing them with NULL
fn bypass_dont_instructions(input: &str) -> anyhow::Result<String> {
    // yes I could have used a regex here,
    // but I wanted to show how to do it without regex
    let mut modified_string = String::new();
    let mut within_dont_section = false;
    let input_bytes = input.as_bytes();
    let mut i = 0;

    while i < input_bytes.len() {
        // Check for "don't"
        if i + 5 <= input_bytes.len() && &input_bytes[i..i + 5] == b"don't" {
            within_dont_section = true;
            modified_string.push_str("NULL");
            i += 5; // Length of "don't"
        }
        // Check for "do()"
        else if i + 4 <= input_bytes.len() && &input_bytes[i..i + 4] == b"do()" {
            within_dont_section = false;
            i += 4; // Length of "do()"
        } else {
            if within_dont_section {
                modified_string.push_str("NULL");
                // Skip to the next non-control segment
                while i < input_bytes.len() && (i + 4 > input_bytes.len() || &input_bytes[i..i + 4] != b"do()") && (i + 5 > input_bytes.len() || &input_bytes[i..i + 5] != b"don't") {
                    i += 1;
                }
            } else {
                modified_string.push(input_bytes[i] as char);
                i += 1;
            }
        }
    }

    Ok(modified_string)
}

// END: Part 2

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_extract_and_multiply() {
        let input = "mul(427,266) mul(287,390) mul(398,319) mul(613,600) mul(189,242) mul(908,64) mul(483,371)";
        let products = extract_and_multiply(input).unwrap();
        assert_eq!(products, vec![Some(113582), Some(111930), Some(126962), Some(367800), Some(45738), Some(58112), Some(179193)]);
    }

    #[test]
    fn test_add_products() {
        let products = vec![Some(113582), Some(111630), Some(127162), Some(367800), Some(45738), Some(58112), Some(179193)];
        assert_eq!(add_products(products), 1003217);
    }

    // #[test]
    // fn test_solve_part2() {
    //     let input: &str = "mul(427,266)#mul(287,390)mul(398,319)#!$>don't()mul(613,600)from()@!{-from()[%?mul(189,242)~#$>from(96,165)$do()'{mul(908,64)don'tmul(483,371)h";
    //     let expected_result: &str = "mul(427,266)#mul(287,390)mul(398,319)#!$>NULLNULL'{mul(908,64)NULLNULL";
    //     assert_eq!(bypass_dont_instructions(input), Ok(expected_result.to_string()) );
    // }
}