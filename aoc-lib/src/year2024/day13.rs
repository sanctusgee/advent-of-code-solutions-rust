// file: src/year2024/day13.rs
use crate::utils;
use anyhow::Result;

#[derive(Debug)]
struct ClawMachine {
    button_a: (i64, i64), // (x, y) movement for button A
    button_b: (i64, i64), // (x, y) movement for button B
    prize: (i64, i64),    // (x, y) position of prize
}

impl ClawMachine {
    fn parse_from_lines(lines: &[String]) -> Result<Self> {
        if lines.len() != 3 {
            anyhow::bail!("Expected exactly 3 lines for a claw machine");
        }

        // Parse Button A: X+94, Y+34
        let button_a = Self::parse_button_line(&lines[0])?;
        
        // Parse Button B: X+22, Y+67
        let button_b = Self::parse_button_line(&lines[1])?;
        
        // Parse Prize: X=8400, Y=5400
        let prize = Self::parse_prize_line(&lines[2])?;

        Ok(ClawMachine {
            button_a,
            button_b,
            prize,
        })
    }

    fn parse_button_line(line: &str) -> Result<(i64, i64)> {
        // Example: "Button A: X+94, Y+34"
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 2 {
            anyhow::bail!("Invalid button line format: {}", line);
        }

        let coords = parts[1].trim();
        let xy_parts: Vec<&str> = coords.split(',').collect();
        if xy_parts.len() != 2 {
            anyhow::bail!("Invalid coordinate format: {}", coords);
        }

        // Parse X+94
        let x_part = xy_parts[0].trim();
        let x = if let Some(x_str) = x_part.strip_prefix("X+") {
            x_str.parse::<i64>()?
        } else if let Some(x_str) = x_part.strip_prefix("X-") {
            -x_str.parse::<i64>()?
        } else {
            anyhow::bail!("Invalid X coordinate format: {}", x_part);
        };

        // Parse Y+34
        let y_part = xy_parts[1].trim();
        let y = if let Some(y_str) = y_part.strip_prefix("Y+") {
            y_str.parse::<i64>()?
        } else if let Some(y_str) = y_part.strip_prefix("Y-") {
            -y_str.parse::<i64>()?
        } else {
            anyhow::bail!("Invalid Y coordinate format: {}", y_part);
        };

        Ok((x, y))
    }

    fn parse_prize_line(line: &str) -> Result<(i64, i64)> {
        // Example: "Prize: X=8400, Y=5400"
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 2 {
            anyhow::bail!("Invalid prize line format: {}", line);
        }

        let coords = parts[1].trim();
        let xy_parts: Vec<&str> = coords.split(',').collect();
        if xy_parts.len() != 2 {
            anyhow::bail!("Invalid coordinate format: {}", coords);
        }

        // Parse X=8400
        let x_part = xy_parts[0].trim();
        let x = if let Some(x_str) = x_part.strip_prefix("X=") {
            x_str.parse::<i64>()?
        } else {
            anyhow::bail!("Invalid X coordinate format: {}", x_part);
        };

        // Parse Y=5400
        let y_part = xy_parts[1].trim();
        let y = if let Some(y_str) = y_part.strip_prefix("Y=") {
            y_str.parse::<i64>()?
        } else {
            anyhow::bail!("Invalid Y coordinate format: {}", y_part);
        };

        Ok((x, y))
    }

    // Solve the system of linear equations using Cramer's rule
    // a * ax + b * bx = px
    // a * ay + b * by = py
    fn solve_linear_system(&self, max_presses: Option<i64>) -> Option<(i64, i64)> {
        let (ax, ay) = self.button_a;
        let (bx, by) = self.button_b;
        let (px, py) = self.prize;

        // Calculate determinant of coefficient matrix
        let det = ax * by - ay * bx;
        
        if det == 0 {
            return None; // No unique solution
        }

        // Use Cramer's rule to solve for a and b
        let a_num = px * by - py * bx;
        let b_num = ax * py - ay * px;

        // Check if solutions are integers
        if a_num % det != 0 || b_num % det != 0 {
            return None; // No integer solution
        }

        let a = a_num / det;
        let b = b_num / det;

        // Check if solutions are non-negative
        if a < 0 || b < 0 {
            return None;
        }

        // Check maximum presses constraint if provided
        if let Some(max) = max_presses {
            if a > max || b > max {
                return None;
            }
        }

        // Verify the solution
        if a * ax + b * bx == px && a * ay + b * by == py {
            Some((a, b))
        } else {
            None
        }
    }

    fn calculate_tokens(&self, max_presses: Option<i64>) -> Option<i64> {
        if let Some((a, b)) = self.solve_linear_system(max_presses) {
            Some(a * 3 + b * 1) // A costs 3 tokens, B costs 1 token
        } else {
            None
        }
    }
}

fn parse_input(lines: Vec<String>) -> Result<Vec<ClawMachine>> {
    let mut machines = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        // Skip empty lines
        if lines[i].trim().is_empty() {
            i += 1;
            continue;
        }

        // Make sure we have at least 3 lines for a complete machine
        if i + 2 >= lines.len() {
            break;
        }

        let machine_lines = &lines[i..i + 3];
        let machine = ClawMachine::parse_from_lines(machine_lines)?;
        machines.push(machine);

        i += 3;
        
        // Skip optional empty line between machines
        if i < lines.len() && lines[i].trim().is_empty() {
            i += 1;
        }
    }

    Ok(machines)
}

fn solve_part1(machines: &[ClawMachine]) -> (i64, i64) {
    let mut total_tokens = 0;
    let mut prizes_won = 0;

    for machine in machines {
        if let Some(tokens) = machine.calculate_tokens(Some(100)) {
            total_tokens += tokens;
            prizes_won += 1;
        }
    }

    (prizes_won, total_tokens)
}

fn solve_part2(machines: &[ClawMachine]) -> (i64, i64) {
    let mut total_tokens = 0;
    let mut prizes_won = 0;

    for machine in machines {
        // For part 2, add 10000000000000 to prize coordinates
        let adjusted_machine = ClawMachine {
            button_a: machine.button_a,
            button_b: machine.button_b,
            prize: (
                machine.prize.0 + 10000000000000,
                machine.prize.1 + 10000000000000,
            ),
        };

        if let Some(tokens) = adjusted_machine.calculate_tokens(None) {
            total_tokens += tokens;
            prizes_won += 1;
        }
    }

    (prizes_won, total_tokens)
}

pub fn solve() -> Result<()> {
    let input = utils::load_input(2024, 13)?;
    let file_data: Vec<String> = input.lines().map(|s| s.to_string()).collect();

    let machines = parse_input(file_data)?;

    println!("Parsed {} claw machines", machines.len());

    // Part 1
    let (prizes_won_p1, total_tokens_p1) = solve_part1(&machines);
    println!("Part 1: Won {} prizes with {} tokens", prizes_won_p1, total_tokens_p1);

    // Part 2
    let (prizes_won_p2, total_tokens_p2) = solve_part2(&machines);
    println!("Part 2: Won {} prizes with {} tokens", prizes_won_p2, total_tokens_p2);

    Ok(())
}

