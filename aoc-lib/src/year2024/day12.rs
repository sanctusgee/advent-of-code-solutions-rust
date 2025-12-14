use std::collections::{HashSet, VecDeque};
use crate::utils;
use anyhow::Result;

#[derive(Debug, Clone)]
struct Region {
    // plant_type: char,
    plots: HashSet<(usize, usize)>,
    area: usize,
    perimeter: usize,
    sides: usize,
}

impl Region {
    // fn new(plant_type: char) -> Self {
     fn new() -> Self {   Self {
            // plant_type,
            plots: HashSet::new(),
            area: 0,
            perimeter: 0,
            sides: 0,
        }
    }
    
    fn price_part1(&self) -> usize {
        self.area * self.perimeter
    }
    
    fn price_part2(&self) -> usize {
        self.area * self.sides
    }
}

fn parse_garden_map(file_data: &Vec<String>) -> Vec<Vec<char>> {
    file_data
        .iter()
        .map(|line| line.chars().collect())
        .collect()
}

fn get_neighbors(row: usize, col: usize, rows: usize, cols: usize) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    
    // Up, Down, Left, Right
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    
    for (dr, dc) in directions {
        let new_row = row as isize + dr;
        let new_col = col as isize + dc;
        
        if new_row >= 0 && new_row < rows as isize && new_col >= 0 && new_col < cols as isize {
            neighbors.push((new_row as usize, new_col as usize));
        }
    }
    
    neighbors
}

fn count_sides(plots: &HashSet<(usize, usize)>, rows: usize, cols: usize) -> usize {
    let mut sides = 0;
    
    // Count horizontal sides (top and bottom edges)
    for row in 0..=rows {
        let mut in_top_edge = false;
        let mut in_bottom_edge = false;
        
        for col in 0..cols {
            // Check for top edge
            let has_plot_below = row < rows && plots.contains(&(row, col));
            let has_plot_above = row > 0 && plots.contains(&(row - 1, col));
            
            let top_edge = has_plot_below && !has_plot_above;
            
            if top_edge && !in_top_edge {
                sides += 1;
                in_top_edge = true;
            } else if !top_edge {
                in_top_edge = false;
            }
            
            // Check for bottom edge
            let bottom_edge = has_plot_above && !has_plot_below;
            
            if bottom_edge && !in_bottom_edge {
                sides += 1;
                in_bottom_edge = true;
            } else if !bottom_edge {
                in_bottom_edge = false;
            }
        }
    }
    
    // Count vertical sides (left and right edges)
    for col in 0..=cols {
        let mut in_left_edge = false;
        let mut in_right_edge = false;
        
        for row in 0..rows {
            // Check for left edge
            let has_plot_right = col < cols && plots.contains(&(row, col));
            let has_plot_left = col > 0 && plots.contains(&(row, col - 1));
            
            let left_edge = has_plot_right && !has_plot_left;
            
            if left_edge && !in_left_edge {
                sides += 1;
                in_left_edge = true;
            } else if !left_edge {
                in_left_edge = false;
            }
            
            // Check for right edge
            let right_edge = has_plot_left && !has_plot_right;
            
            if right_edge && !in_right_edge {
                sides += 1;
                in_right_edge = true;
            } else if !right_edge {
                in_right_edge = false;
            }
        }
    }
    
    sides
}

fn flood_fill_region(
    garden: &Vec<Vec<char>>,
    start_row: usize,
    start_col: usize,
    visited: &mut HashSet<(usize, usize)>
) -> Region {
    let rows = garden.len();
    let cols = garden[0].len();
    let plant_type = garden[start_row][start_col];
    
    // let mut region = Region::new(plant_type);
    let mut region = Region::new();
    
    let mut queue = VecDeque::new();
    
    queue.push_back((start_row, start_col));
    visited.insert((start_row, start_col));
    
    while let Some((row, col)) = queue.pop_front() {
        region.plots.insert((row, col));
        region.area += 1;
        
        // Calculate perimeter contribution for this plot
        let mut plot_perimeter = 4; // Start with 4 sides
        
        for (neighbor_row, neighbor_col) in get_neighbors(row, col, rows, cols) {
            if garden[neighbor_row][neighbor_col] == plant_type {
                plot_perimeter -= 1; // Remove one side if neighbor is same plant type
                
                // Add unvisited neighbors of same type to queue
                if !visited.contains(&(neighbor_row, neighbor_col)) {
                    visited.insert((neighbor_row, neighbor_col));
                    queue.push_back((neighbor_row, neighbor_col));
                }
            }
        }
        
        region.perimeter += plot_perimeter;
    }
    
    // Calculate sides for part 2
    region.sides = count_sides(&region.plots, rows, cols);
    
    region
}

fn find_all_regions(garden: &Vec<Vec<char>>) -> Vec<Region> {
    let rows = garden.len();
    let cols = garden[0].len();
    let mut visited = HashSet::new();
    let mut regions = Vec::new();
    
    for row in 0..rows {
        for col in 0..cols {
            if !visited.contains(&(row, col)) {
                let region = flood_fill_region(garden, row, col, &mut visited);
                regions.push(region);
            }
        }
    }
    
    regions
}

fn solve_part1(file_data: &Vec<String>) -> Result<()> {
    let garden = parse_garden_map(file_data);
    let regions = find_all_regions(&garden);
    
    println!("Found {} regions", regions.len());
    
    let mut total_price = 0;
    for region in &regions {
        let price = region.price_part1();
        total_price += price;
    }
    
    println!("Part 1: {}", total_price);
    Ok(())
}

fn solve_part2(file_data: &Vec<String>) -> Result<()> {
    let garden = parse_garden_map(file_data);
    let regions = find_all_regions(&garden);
    
    println!("Found {} regions for Part 2", regions.len());
    
    let mut total_price = 0;
    for region in &regions {
        let price = region.price_part2();
        total_price += price;
    }
    
    println!("Part 2: {}", total_price);
    Ok(())
}


pub fn solve() -> Result<()> {
    let input = utils::load_input(2024, 12)?;
    let file_data: Vec<String> = input.lines().map(|s| s.to_string()).collect();

    solve_part1(&file_data)?;
    solve_part2(&file_data)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_part2_simple_example() {
        let input = vec![
            "AAAA".to_string(),
            "BBCD".to_string(),
            "BBCC".to_string(),
            "EEEC".to_string(),
        ];
        
        let garden = parse_garden_map(&input);
        let regions = find_all_regions(&garden);
        
        // Should have 5 regions: A, B, C, D, E
        assert_eq!(regions.len(), 5);
        
        let total_price: usize = regions.iter().map(|r| r.price_part2()).sum();
        assert_eq!(total_price, 80);
    }
    
    #[test]
    fn test_part2_large_example() {
        let input = vec![
            "RRRRIICCFF".to_string(),
            "RRRRIICCCF".to_string(),
            "VVRRRCCFFF".to_string(),
            "VVRCCCJFFF".to_string(),
            "VVVVCJJCFE".to_string(),
            "VVIVCCJJEE".to_string(),
            "VVIIICJJEE".to_string(),
            "MIIIIIJJEE".to_string(),
            "MIIISIJEEE".to_string(),
            "MMMISSJEEE".to_string(),
        ];
        
        let garden = parse_garden_map(&input);
        let regions = find_all_regions(&garden);
        
        let total_price: usize = regions.iter().map(|r| r.price_part2()).sum();
        assert_eq!(total_price, 1206);
    }
    
    #[test]
    fn test_simple_example() {
        let input = vec![
            "AAAA".to_string(),
            "BBCD".to_string(),
            "BBCC".to_string(),
            "EEEC".to_string(),
        ];
        
        let garden = parse_garden_map(&input);
        let regions = find_all_regions(&garden);
        
        // Should have 5 regions: A, B, C, D, E
        assert_eq!(regions.len(), 5);
        
        let total_price: usize = regions.iter().map(|r| r.price_part1()).sum();
        assert_eq!(total_price, 140);
    }
    
    #[test]
    fn test_nested_example() {
        let input = vec![
            "OOOOO".to_string(),
            "OXOXO".to_string(),
            "OOOOO".to_string(),
            "OXOXO".to_string(),
            "OOOOO".to_string(),
        ];
        
        let garden = parse_garden_map(&input);
        let regions = find_all_regions(&garden);
        
        // Should have 5 regions: 1 large O region and 4 single X regions
        assert_eq!(regions.len(), 5);
        
        let total_price: usize = regions.iter().map(|r| r.price_part1()).sum();
        assert_eq!(total_price, 772);
    }
}