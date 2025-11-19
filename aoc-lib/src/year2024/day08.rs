use std::collections::{HashMap, HashSet};
use crate::utils;
use anyhow::Result;

/// Compute the non-negative gcd of a and b
fn gcd(mut a: isize, mut b: isize) -> isize {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a.abs()
}

/// Check whether (r,c) is inside a grid of size nrows×ncols
fn in_bounds(r: isize, c: isize, nrows: usize, ncols: usize) -> bool {
    r >= 0 && (r as usize) < nrows && c >= 0 && (c as usize) < ncols
}

/// Parse the grid and group antennas by frequency
fn parse_grid(file_data: &Vec<String>) -> (Vec<String>, HashMap<char, Vec<(usize, usize)>>) {
    let grid = file_data.clone(); // file_data is already a Vec<String>
    
    let mut by_freq: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (r, row) in grid.iter().enumerate() {
        for (c, ch) in row.chars().enumerate() {
            if ch != '.' {
                by_freq.entry(ch).or_default().push((r, c));
            }
        }
    }
    
    (grid, by_freq)
}

fn solve_part1(file_data: &Vec<String>) -> Result<()> {
    let (grid, by_freq) = parse_grid(file_data);
    let nrows = grid.len();
    let ncols = match grid.first() {
        Some(row) => row.len(),
        None => return Err(anyhow::anyhow!("Empty grid")),
    };
    
    let mut antinodes = HashSet::new();
    
    // Part 1: Only consider antinodes at specific distances (2:1 ratio)
    for positions in by_freq.values() {
        if positions.len() < 2 { continue; }
        
        // Consider every unordered pair of antennas
        for i in 0..positions.len() - 1 {
            for j in i + 1..positions.len() {
                let (r1, c1) = positions[i];
                let (r2, c2) = positions[j];
                
                // Calculate the vector from antenna1 to antenna2
                let dr = r2 as isize - r1 as isize;
                let dc = c2 as isize - c1 as isize;
                
                // Two antinodes: one extends beyond antenna2, one extends beyond antenna1
                let antinode1_r = r2 as isize + dr;
                let antinode1_c = c2 as isize + dc;
                let antinode2_r = r1 as isize - dr;
                let antinode2_c = c1 as isize - dc;
                
                // Add antinodes that are within bounds
                if in_bounds(antinode1_r, antinode1_c, nrows, ncols) {
                    antinodes.insert((antinode1_r as usize, antinode1_c as usize));
                }
                if in_bounds(antinode2_r, antinode2_c, nrows, ncols) {
                    antinodes.insert((antinode2_r as usize, antinode2_c as usize));
                }
            }
        }
    }
    
    println!("Part 1: {}", antinodes.len());
    Ok(())
}

fn solve_part2(file_data: &Vec<String>) -> Result<()> {
    let (grid, by_freq) = parse_grid(file_data);
    let nrows = grid.len();
    let ncols = match grid.first() {
        Some(row) => row.len(),
        None => return Err(anyhow::anyhow!("Empty grid")),
    };
    
    let mut antinodes = HashSet::new();
    
    // Part 2: Consider all points along the line between antennas
    for positions in by_freq.values() {
        if positions.len() < 2 { continue; }
        
        // Consider every unordered pair of antennas
        for i in 0..positions.len() - 1 {
            for j in i + 1..positions.len() {
                let (r1, c1) = positions[i];
                let (r2, c2) = positions[j];
                
                // Compute step vector reduced to primitive integer direction
                let dr = r2 as isize - r1 as isize;
                let dc = c2 as isize - c1 as isize;
                let g = gcd(dr.abs(), dc.abs());
                let step_r = dr / g;
                let step_c = dc / g;
                
                // Walk backward from (r1,c1) to the edge of the grid
                let mut tr = r1 as isize;
                let mut tc = c1 as isize;
                while in_bounds(tr - step_r, tc - step_c, nrows, ncols) {
                    tr -= step_r;
                    tc -= step_c;
                }
                
                // Walk forward, marking every integer cell on that line
                while in_bounds(tr, tc, nrows, ncols) {
                    antinodes.insert((tr as usize, tc as usize));
                    tr += step_r;
                    tc += step_c;
                }
            }
        }
    }
    
    println!("Part 2: {}", antinodes.len());
    Ok(())
}

pub fn solve() -> Result<()> {
    // Load the file data for day08
    // let file_data = utils::load_file_data("day08")?; // Pass "day08" explicitly
    let file = utils::load_input(2024, 8)?;
    // file is already a Vec<String>, no need to split by delimiter
    let input: Vec<String> = file.lines().map(|s| s.to_string()).collect();

    solve_part1(&input)?;
    solve_part2(&input)?;
    
    Ok(())
}