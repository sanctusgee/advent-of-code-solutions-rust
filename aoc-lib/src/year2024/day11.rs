use crate::utils;
use std::collections::HashMap;
use anyhow::Result;

pub fn solve() -> Result<()> {
    let input = utils::load_input(2024, 11)?;

    let stones: Vec<u64> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("Initial stones: {:?}", stones);

    let part1 = simulate_blinks(&stones, 25);
    let part2 = simulate_blinks(&stones, 75);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

fn simulate_blinks(initial_stones: &[u64], blinks: usize) -> usize {
    // Use a cache to avoid recomputing transformations
    let mut cache: HashMap<u64, Vec<u64>> = HashMap::new();

    // Track counts of each stone value instead of individual stones
    let mut stone_counts: HashMap<u64, usize> = HashMap::new();

    // Initialize counts from initial stones
    for &stone in initial_stones {
        *stone_counts.entry(stone).or_insert(0) += 1;
    }

    for blink in 0..blinks {
        let mut new_counts: HashMap<u64, usize> = HashMap::new();

        for (&stone, &count) in &stone_counts {
            // Get or compute the transformation for this stone
            let next_stones = cache.entry(stone).or_insert_with(|| transform_stone(stone));

            // Add the resulting stones with their counts
            for next_stone in next_stones.iter() {
                *new_counts.entry(*next_stone).or_insert(0) += count;
            }
        }

        stone_counts = new_counts;

        // Print progress for part 2
        if blinks > 25 && (blink + 1) % 10 == 0 {
            println!("After {} blinks: {} stones", blink + 1, stone_counts.values().sum::<usize>());
        }
    }

    stone_counts.values().sum()
}

fn transform_stone(stone: u64) -> Vec<u64> {
    // Rule 1: If the stone is 0, replace with 1
    if stone == 0 {
        return vec![1];
    }

    // Rule 2: If the stone has an even number of digits, split it
    let digits = count_digits(stone);
    if digits % 2 == 0 {
        let divisor = 10u64.pow(digits / 2);
        let left = stone / divisor;
        let right = stone % divisor;
        vec![left, right]
    } else {
        // Rule 3: Otherwise, multiply by 2024
        vec![stone * 2024]
    }
}

fn count_digits(n: u64) -> u32 {
    if n == 0 {
        1
    } else {
        (n as f64).log10().floor() as u32 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_stone() {
        assert_eq!(transform_stone(0), vec![1]);
        assert_eq!(transform_stone(1), vec![2024]);
        assert_eq!(transform_stone(10), vec![1, 0]);
        assert_eq!(transform_stone(99), vec![9, 9]);
        assert_eq!(transform_stone(999), vec![2021976]);
    }

    #[test]
    fn test_simulate_blinks() {
        let stones = vec![125, 17];
        assert_eq!(simulate_blinks(&stones, 6), 22);
        assert_eq!(simulate_blinks(&stones, 25), 55312);
    }
}
