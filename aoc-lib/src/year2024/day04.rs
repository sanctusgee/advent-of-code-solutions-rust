use anyhow::Result;
use crate::utils;

const EXPECTED_XMAS: &[char] = &['M', 'A', 'S'];

pub fn solve() -> Result<()> {
    let input = utils::load_input(2024, 4)?;

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let result_part1 = solve_part1(&grid);
    println!("Day 4 / Part 1 --> Count: {}", result_part1);

    let result_part2 = solve_part2(&grid);
    println!("Day 4 / Part 2 --> X-MAS Count: {}", result_part2);

    Ok(())
}

fn solve_part1(grid: &[Vec<char>]) -> usize {
    let all_directions = generate_all_directions();
    let mut count = 0;

    for y in 0..grid.len() as isize {
        for x in 0..grid[0].len() as isize {
            if get_char(grid, x, y) != Some('X') {
                continue;
            }

            for dir in &all_directions {
                let mut pos_x = x;
                let mut pos_y = y;
                let mut valid = true;
                for &expected in EXPECTED_XMAS {
                    pos_x += dir.dx;
                    pos_y += dir.dy;
                    if get_char(grid, pos_x, pos_y) != Some(expected) {
                        valid = false;
                        break;
                    }
                }
                if valid {
                    count += 1;
                }
            }
        }
    }

    count
}

fn solve_part2(grid: &[Vec<char>]) -> usize {
    let mas_directions = generate_diagonal_pairs();
    let mut count = 0;

    for y in 0..grid.len() as isize {
        for x in 0..grid[0].len() as isize {
            if get_char(grid, x, y) != Some('A') {
                continue;
            }

            let mut valid = true;
            for mas in &mas_directions {
                let mut m = false;
                let mut s = false;
                for dir in mas {
                    let chr = dir.next_position(x, y).and_then(|(nx, ny)| get_char(grid, nx, ny));
                    match chr {
                        Some('M') => m = true,
                        Some('S') => s = true,
                        _ => {
                            valid = false;
                            break;
                        }
                    }
                }
                if !(m && s) {
                    valid = false;
                    break;
                }
            }

            if valid {
                count += 1;
            }
        }
    }

    count
}

#[derive(Copy, Clone)]
struct Direction {
    dx: isize,
    dy: isize,
}

impl Direction {
    fn next_position(&self, x: isize, y: isize) -> Option<(isize, isize)> {
        Some((x + self.dx, y + self.dy))
    }
}

// Function to generate all directions for "XMAS"
fn generate_all_directions() -> Vec<Direction> {
    vec![
        Direction { dx: 0, dy: 1 },  // Right
        Direction { dx: 0, dy: -1 }, // Left
        Direction { dx: 1, dy: 0 },  // Down
        Direction { dx: -1, dy: 0 }, // Up
        Direction { dx: 1, dy: 1 },  // Down-Right
        Direction { dx: 1, dy: -1 }, // Down-Left
        Direction { dx: -1, dy: 1 }, // Up-Right
        Direction { dx: -1, dy: -1 } // Up-Left
    ]
}

// Function to generate all diagonal directions
fn generate_diagonal_pairs() -> Vec<Vec<Direction>> {
    vec![
        vec![Direction { dx: 1, dy: 1 }, Direction { dx: -1, dy: -1 }],
        vec![Direction { dx: 1, dy: -1 }, Direction { dx: -1, dy: 1 }]
    ]
}

fn get_char(grid: &[Vec<char>], x: isize, y: isize) -> Option<char> {
    if x >= 0 && y >= 0 {
        grid.get(y as usize)?.get(x as usize).copied()
    } else {
        None
    }
}


#[cfg(test)]
mod test {
    use super::*;

    const CASE: &str = "
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_part1() {
        let grid: Vec<Vec<char>> = CASE.trim().lines().map(|line| line.chars().collect()).collect();
        let result = solve_part1(&grid);
        println!("Test Part 1 Result: {}", result);
        assert_eq!(result, 18);
    }

    #[test]
    fn test_part2() {
        let grid: Vec<Vec<char>> = CASE.trim().lines().map(|line| line.chars().collect()).collect();
        let result = solve_part2(&grid);
        println!("Test Part 2 Result: {}", result);
        assert_eq!(result, 9);
    }
}
