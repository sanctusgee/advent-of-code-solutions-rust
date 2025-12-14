use crate::utils;
use anyhow::Result;

type Schematic = Vec<usize>;

fn parse(input: &str) -> (Vec<Schematic>, Vec<Schematic>) {
    println!("Day 25: parsing input...");
    let blocks: Vec<&str> = input.split("\n\n").collect();
    
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    
    for block in blocks {
        let lines: Vec<&str> = block.lines().collect();
        if lines.is_empty() {
            continue;
        }
        
        let is_lock = lines[0].chars().all(|c| c == '#');
        let mut heights = vec![0; 5];
        
        // Count # in each column (excluding first and last rows)
        for row in 1..lines.len()-1 {
            for (col, ch) in lines[row].chars().enumerate() {
                if ch == '#' {
                    heights[col] += 1;
                }
            }
        }
        
        if is_lock {
            locks.push(heights);
        } else {
            keys.push(heights);
        }
    }
    
    println!("Parsed {} locks and {} keys.", locks.len(), keys.len());
    (locks, keys)
}

fn fits(lock: &Schematic, key: &Schematic) -> bool {
    for i in 0..5 {
        if lock[i] + key[i] > 5 {
            return false;
        }
    }
    true
}

fn part1(input: &str) -> usize {
    println!("Part 1: counting compatible lock/key pairs...");
    let (locks, keys) = parse(input);
    
    let mut count = 0;
    for lock in &locks {
        for key in &keys {
            if fits(lock, key) {
                count += 1;
            }
        }
    }
    
    println!("Part 1: found {} compatible pairs.", count);
    count
}

fn part2() -> String {
    "Part 2: Day 25 has no Part 2!".to_string()
}

pub fn solve() -> Result<()> {
    println!("Starting Day 25 solver...");
    let input = utils::load_input(2024, 25)?;

    println!("Processing Part 1...");
    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    println!("Processing Part 2...");
    let p2 = part2();
    println!("Part 2: {}", p2);

    println!("Day 25 complete - Merry Christmas!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_case() {
        let input = "\
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";
        assert_eq!(part1(input), 3);
    }
}