// Contents: Advent of Code - Day 6: Grid Navigation
// file: src/year2024/day06.rs
use std::collections::HashSet;
use crate::utils;
use anyhow::Result;

pub fn solve() -> Result<()> {
    let sim_data = load_simulation_data()?;
    solve_part1(&sim_data)?;
    solve_part2(&sim_data)?;
    Ok(())
}

fn solve_part1(sim_data: &SimulationData) -> Result<()> {
    println!("*************************** PART 1 Solution ***************************");
    println!("      Distinct positions visited: {}", sim_data.visited_positions.len());
    println!("*********************************************************************\n");
    Ok(())
}

fn solve_part2(sim_data: &SimulationData) -> Result<()> {
    let guard_pos = (sim_data.guard_start.0 as isize, sim_data.guard_start.1 as isize);
    let candidates: Vec<(isize, isize)> = sim_data.visited_positions
        .iter()
        .filter(|&&(r, c)| (r, c) != guard_pos && sim_data.grid[r as usize][c as usize] == 0)
        .cloned()
        .collect();
    println!("Candidate positions for obstruction: {}", candidates.len());

    let mut valid_obstruction_count = 0;
    for (i, (r, c)) in candidates.iter().enumerate() {
        let mut mod_grid = sim_data.grid.clone();
        mod_grid[*r as usize][*c as usize] = -1; // Place obstruction.
        if simulate_guard(&mod_grid, sim_data.guard_start, sim_data.max_row, sim_data.max_col) {
            valid_obstruction_count += 1;
        }
        if (i + 1) % 500 == 0 {
            println!("Processed {} / {} candidates", i + 1, candidates.len());
        }
    }
    println!("\n*************************** PART 2 Solution ***************************");
    println!("Valid obstruction count (guard loops): {}", valid_obstruction_count);
    println!("*********************************************************************\n");
    Ok(())
}

// --- This is the shared simulation data helper definitions ---
struct SimulationData {
    grid: Vec<Vec<i32>>,
    guard_start: (usize, usize, i32),
    max_row: isize,
    max_col: isize,
    visited_positions: std::collections::HashSet<(isize, isize)>,
}

fn load_simulation_data() -> Result<SimulationData> {
    let file = utils::load_input(2024, 6)?;
    let file_lines: Vec<String> = file.lines().map(|s| s.to_string()).collect();
    let input = file_lines.join("\n");

    let (grid, guard_start_opt) = parse_input(&input);
    let guard_start = guard_start_opt.ok_or_else(|| anyhow::anyhow!("Guard starting position not found in input."))?;

    let nrows = grid.len();
    let ncols = grid[0].len();
    let max_row = (nrows - 1) as isize;
    let max_col = (ncols - 1) as isize;

    let visited_positions = simulate_unobstructed(&grid, guard_start, max_row, max_col);

    Ok(SimulationData {
        grid,
        guard_start,
        max_row,
        max_col,
        visited_positions,
    })
}



fn parse_input(input: &str) -> (Vec<Vec<i32>>, Option<(usize, usize, i32)>) {
    let mut grid = Vec::new();
    let mut guard_start = None;
    for (r, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (c, ch) in line.chars().enumerate() {
            let val = match ch {
                '#' => -1,
                '^' => {
                    if guard_start.is_none() {
                        guard_start = Some((r, c, 1));
                    }
                    1
                }
                '<' => {
                    if guard_start.is_none() {
                        guard_start = Some((r, c, 2));
                    }
                    2
                }
                'v' => {
                    if guard_start.is_none() {
                        guard_start = Some((r, c, 3));
                    }
                    3
                }
                '>' => {
                    if guard_start.is_none() {
                        guard_start = Some((r, c, 4));
                    }
                    4
                }
                '.' => 0,
                _   => 0,
            };
            row.push(val);
        }
        grid.push(row);
    }
    (grid, guard_start)
}

fn get_delta(direction: i32) -> (isize, isize) {
    // Directions: 1: Up, 2: Left, 3: Down, 4: Right
    match direction {
        1 => (-1, 0),
        2 => (0, -1),
        3 => (1, 0),
        4 => (0, 1),
        _ => (0, 0),
    }
}

fn turn_right(direction: i32) -> i32 {
    // Turn right: Up (1) -> Right (4) -> Down (3) -> Left (2) -> Up (1)
    match direction {
        1 => 4,
        4 => 3,
        3 => 2,
        2 => 1,
        _ => direction,
    }
}

/// Simulate the guard's patrol on the grid.
/// Returns true if the guard eventually loops (repeating a state),
/// false if the guard exits the grid.
fn simulate_guard(
    grid: &Vec<Vec<i32>>,
    start: (usize, usize, i32),
    max_row: isize,
    max_col: isize,
) -> bool {
    let safe_limit = 4 * grid.len() * grid[0].len();
    let mut visited_states = HashSet::new();

    let (mut r, mut c) = (start.0 as isize, start.1 as isize);
    let mut d = start.2;
    let mut steps = 0;

    while steps < safe_limit {
        let state = (r, c, d);
        if !visited_states.insert(state) {
            return true; // Loop detected.
        }

        let (dr, dc) = get_delta(d);
        let nr = r + dr;
        let nc = c + dc;

        // Check if next position is out of bounds.
        if nr < 0 || nr > max_row || nc < 0 || nc > max_col {
            return false;
        }

        if grid[nr as usize][nc as usize] == -1 {
            // Obstacle ahead, turn right.
            d = turn_right(d);
        } else {
            // Move forward.
            r = nr;
            c = nc;
        }

        steps += 1;
    }
    // If we reached the safe limit, assume it's looping.
    true
}

/// simulate the unobstructed guard movement and record visited positions.
fn simulate_unobstructed(
    grid: &Vec<Vec<i32>>,
    start: (usize, usize, i32),
    max_row: isize,
    max_col: isize,
) -> HashSet<(isize, isize)> {
    let safe_limit = 4 * grid.len() * grid[0].len();
    let mut visited_positions = HashSet::new();

    let (mut r, mut c) = (start.0 as isize, start.1 as isize);
    let mut d = start.2;
    let mut steps = 0;

    while steps < safe_limit {
    // Record the current guard position.
    // 'r' is the current row and 'c' is the current column.
    visited_positions.insert((r, c));

    //  direction name based on the current direction 'd'.
    let direction_name = match d {
        1 => "Up",
        2 => "Left",
        3 => "Down",
        4 => "Right",
        _ => "Unknown",
    };

    // println!("Step {}: Position = ({}, {}), direction = {}", steps, r, c, d);
    // Log the current step, position (r, c) and direction (d) but show the direction name instead
    println!("Step {}: Position = ({}, {}), Current direction = {}", steps, r, c, direction_name);

    // Get the movement deltas based on the current direction 'd'.
    // 'dr' is the change in the row (delta row) and 'dc' is the change in the column (delta column).
    let (dr, dc) = get_delta(d);

    // Calculate the new position if the guard moves forward.
    // 'nr' (new row) is the sum of the current row 'r' and the row delta 'dr'.
    // 'nc' (new column) is the sum of the current column 'c' and the column delta 'dc'.
    let nr = r + dr;
    let nc = c + dc;

    // If the new position is outside the grid bounds,
    // then the guard would exit the grid.
    if nr < 0 || nr > max_row || nc < 0 || nc > max_col {
        // Log when the guard is about to exit the grid.
        println!("Guard exits the grid at step {}: attempted position = ({}, {})", steps, nr, nc);
        break; // Guard exits the grid.
    }

    // Check if the cell at the new position (nr, nc) is an obstacle (indicated by -1).
    // If it is, update the direction by turning right.
    if grid[nr as usize][nc as usize] == -1 {
            println!("Encountered obstacle at ({}, {}), turning right", nr, nc);
        d = turn_right(d);
    } else {
        // Otherwise, move the guard to the new position.
        r = nr;
        c = nc;
    }

    // Increment the step counter.
    steps += 1;
}
    visited_positions
}
