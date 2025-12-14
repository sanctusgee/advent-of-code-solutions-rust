// This one was tricky to figure so Notes to Future Self below:
//
// The only tricky bit is try_push_boxes: scan to find the end of a contiguous box run, 
// verify the far end is floor, then shift boxes backwards so you don’t overwrite.
// 
// Remember that GPS uses 0-based (row, col) → 100 * row + col.
// 
// If you want to debug a path, uncomment Warehouse::_render() calls mid-sim 
// because  it overlays @ at the robot’s tracked position.

// Prt 2:
// Horizontal pushes (Part 2) treat each [] as a unit and can push a whole chain in one go. 
// We scan to find the chain, ensure the destination cell just beyond the chain is '.', then shift.
// 
// Vertical pushes (Part 2) can branch: the box above/below your box might touch more boxes. 
// We do a small flood fill to collect all boxes that must move this tick; if any destination
// contains #, the push is blocked. Otherwise we clear originals and write targets.
// 
// GPS in Part 2 counts only '[' tiles (the left edge). That automatically measures from the
// box’s closest edge.


//! Advent of Code 2024 – Day 15 — Warehouse Woes (Parts 1 & 2)
//!
//! Part 1: simulate pushing single-tile boxes (`O`) on a toroidal-free, walled grid.
//! Part 2: expand the map horizontally (2× wide) where boxes are now **two tiles** wide (`[]`)
//!         and simulate under the new rules. GPS changes to measure from the *closest edge*
//!         of the box (i.e., the `[` tile).
//!
//! Prints exactly:
//! ```text
//! Part 1: <sum_of_gps_after_part1>
//! Part 2: <sum_of_gps_after_part2>
//! ```
//!
//! Input format (both parts):
//! ```text
//! <grid rows...>
//! <blank line>
//! <move characters possibly across multiple lines>
//! ```
//! Path assumed: `input/year2024/day15.txt`.

use crate::utils;
use anyhow::Result;
/* ───────────────────────────── Shared parsing ───────────────────────────── */

/// Direction for a single robot move.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Dir {
    Up, Down, Left, Right,
}

impl Dir {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '^' => Some(Self::Up),
            'v' => Some(Self::Down),
            '<' => Some(Self::Left),
            '>' => Some(Self::Right),
            _ => None,
        }
    }
    fn delta(self) -> (isize, isize) {
        match self {
            Dir::Up => (-1, 0),
            Dir::Down => (1, 0),
            Dir::Left => (0, -1),
            Dir::Right => (0, 1),
        }
    }
}

/// Parse the full puzzle input into (grid_lines, moves).
fn parse_input_raw(input: &str) -> (Vec<String>, Vec<Dir>) {
    let mut parts = input.split("\n\n");
    let map_part = parts.next().unwrap_or_default();
    let moves_part = parts.next().unwrap_or_default();

    let grid_lines: Vec<String> = map_part
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|s| s.to_string())
        .collect();

    let moves: Vec<Dir> = moves_part.chars().filter_map(Dir::from_char).collect();
    (grid_lines, moves)
}

/// Single-tile warehouse (Part 1): walls `#`, boxes `O`, floor `.`, robot tracked separately.
#[derive(Clone, Debug)]
struct WarehouseP1 {
    grid: Vec<Vec<char>>,
    r: usize,
    c: usize,
    rows: usize,
    cols: usize,
}

impl WarehouseP1 {
    fn from_lines(lines: &[String]) -> Self {
        let mut grid: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
        let rows = grid.len();
        let cols = grid.first().map_or(0, |r| r.len());
        let (mut rr, mut cc) = (0, 0);
        'find: for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == '@' {
                    rr = r; cc = c;
                    grid[r][c] = '.';
                    break 'find;
                }
            }
        }
        Self { grid, r: rr, c: cc, rows, cols }
    }

    #[inline]
    fn in_bounds(&self, r: isize, c: isize) -> bool {
        r >= 0 && c >= 0 && (r as usize) < self.rows && (c as usize) < self.cols
    }

    fn step(&mut self, dir: Dir) {
        let (dr, dc) = dir.delta();
        let nr = self.r as isize + dr;
        let nc = self.c as isize + dc;
        if !self.in_bounds(nr, nc) { return; }
        match self.grid[nr as usize][nc as usize] {
            '#' => return, // wall
            '.' => { self.r = nr as usize; self.c = nc as usize; }
            'O' => {
                if self.try_push_boxes(nr as usize, nc as usize, dir) {
                    self.r = nr as usize; self.c = nc as usize;
                }
            }
            _ => {}
        }
    }

    /// Push a contiguous run of `O` ahead by one (Part 1).
    fn try_push_boxes(&mut self, r0: usize, c0: usize, dir: Dir) -> bool {
        let (dr, dc) = dir.delta();
        // Find tail of contiguous boxes.
        let mut tail_r = r0 as isize;
        let mut tail_c = c0 as isize;
        loop {
            let nr = tail_r + dr;
            let nc = tail_c + dc;
            if !self.in_bounds(nr, nc) { return false; }
            match self.grid[nr as usize][nc as usize] {
                'O' => { tail_r = nr; tail_c = nc; }
                '#' => return false,
                '.' => break,
                _ => return false,
            }
        }
        // Shift from tail to front.
        let mut r = tail_r as usize;
        let mut c = tail_c as usize;
        loop {
            let dst_r = (r as isize + dr) as usize;
            let dst_c = (c as isize + dc) as usize;
            debug_assert_eq!(self.grid[dst_r][dst_c], '.');
            self.grid[dst_r][dst_c] = 'O';
            self.grid[r][c] = '.';
            if r == r0 && c == c0 { break; }
            r = (r as isize - dr) as usize;
            c = (c as isize - dc) as usize;
        }
        true
    }

    fn gps_sum(&self) -> i64 {
        let mut acc = 0i64;
        for r in 0..self.rows {
            for c in 0..self.cols {
                if self.grid[r][c] == 'O' {
                    acc += 100 * r as i64 + c as i64;
                }
            }
        }
        acc
    }
}

/// Expand the Part 1 map horizontally as specified:
/// - `#` -> "##"
/// - `O` -> "[]"
/// - `.` -> ".."
/// - `@` -> "@."
fn expand_map_horizontally(lines: &[String]) -> Vec<String> {
    let mut out = Vec::with_capacity(lines.len());
    for line in lines {
        let mut row = String::with_capacity(line.len() * 2);
        for ch in line.chars() {
            match ch {
                '#' => row.push_str("##"),
                'O' => row.push_str("[]"),
                '.' => row.push_str(".."),
                '@' => row.push_str("@."),
                other => row.push(other), // shouldn't happen, but keep it safe
            }
        }
        out.push(row);
    }
    out
}

/// Wide-box warehouse (Part 2): walls `#`, floor `.`, **boxes are `[` and `]` as a pair**, robot tracked separately.
#[derive(Clone, Debug)]
struct WarehouseP2 {
    grid: Vec<Vec<char>>,
    r: usize,
    c: usize,
    rows: usize,
    cols: usize,
}

impl WarehouseP2 {
    fn from_expanded_lines(lines: &[String]) -> Self {
        let mut grid: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
        let rows = grid.len();
        let cols = grid.first().map_or(0, |r| r.len());
        let (mut rr, mut cc) = (0, 0);
        'find: for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == '@' {
                    rr = r; cc = c;
                    grid[r][c] = '.'; // track robot separately
                    break 'find;
                }
            }
        }
        Self { grid, r: rr, c: cc, rows, cols }
    }

    #[inline]
    fn in_bounds(&self, r: isize, c: isize) -> bool {
        r >= 0 && c >= 0 && (r as usize) < self.rows && (c as usize) < self.cols
    }

    fn step(&mut self, dir: Dir) {
        let (dr, dc) = dir.delta();
        let nr = self.r as isize + dr;
        let nc = self.c as isize + dc;
        if !self.in_bounds(nr, nc) { return; }

        match self.grid[nr as usize][nc as usize] {
            '#' => return, // wall
            '.' => { self.r = nr as usize; self.c = nc as usize; }
            '[' | ']' => {
                // Need to push **wide** boxes as units. Logic differs for horizontal vs vertical.
                let ok = match dir {
                    Dir::Left | Dir::Right => self.try_push_horizontal(nr as usize, nc as usize, dir),
                    Dir::Up | Dir::Down => self.try_push_vertical(nr as usize, nc as usize, dir),
                };
                if ok {
                    self.r = nr as usize; self.c = nc as usize;
                }
            }
            _ => {}
        }
    }

    fn try_push_horizontal(&mut self, r0: usize, c0: usize, dir: Dir) -> bool {
        // Normalize start to the **left bracket** index of the first box touched.
        let mut start_c = c0;
        if self.grid[r0][c0] == ']' {
            // We’re touching the right half; the box starts at c-1
            start_c = c0 - 1;
        }
        debug_assert_eq!(self.grid[r0][start_c], '[');
        debug_assert_eq!(self.grid[r0][start_c + 1], ']');

        // Scan forward across contiguous boxes to find the last box in the chain.
        // let (dr, dc) = dir.delta();
        let mut end_c = start_c; // end_c points to the left bracket of the last box in the chain
        loop {
            // let ahead_l = (r0 as isize, (end_c as isize + (if dir == Dir::Right { 2 } else { -1 })));
            // let ahead_r = (r0 as isize, (end_c as isize + (if dir == Dir::Right { 3 } else { 0 })));

            // Cell immediately beyond the chain we intend to move into (one column further)
            // let check_col = if dir == Dir::Right { end_c + 2 } else { start_c - 1 };

            // Determine what lies beyond current chain.
            let next_left_col = if dir == Dir::Right { end_c + 2 } else { end_c - 2 };
            if !self.in_bounds(r0 as isize, next_left_col as isize) {
                return false;
            }

            // Is there another box immediately adjacent in our direction?
            let next_c = if dir == Dir::Right { end_c + 2 } else { end_c - 2 };
            if next_c + 1 >= self.cols { /* possible out of bounds */ }

            if dir == Dir::Right {
                // Check tile right after the last box's right bracket.
                let c_after = end_c + 2; // column of tile after `]`
                if c_after >= self.cols { return false; }
                match self.grid[r0][c_after] {
                    '[' => { end_c += 2; continue; } // another box adjacent, extend chain
                    ']' => { // malformed (shouldn't see loose ']'), treat as blocked
                        return false;
                    }
                    '#' => return false,
                    '.' => {
                        // Free space to the right — we can push!
                        break;
                    }
                    _ => return false,
                }
            } else { // Left
                // Check tile just left of the first box's `[`
                if start_c == 0 { return false; }
                let c_before = start_c - 1;
                match self.grid[r0][c_before] {
                    ']' => {
                        // There's another box immediately to the left; extend chain backward.
                        start_c -= 2;
                        if start_c + 1 >= self.cols { return false; }
                        debug_assert_eq!(self.grid[r0][start_c], '[');
                        debug_assert_eq!(self.grid[r0][start_c + 1], ']');
                        continue;
                    }
                    '[' => return false, // malformed
                    '#' => return false,
                    '.' => {
                        // Free space to the left — ready to push.
                        break;
                    }
                    _ => return false,
                }
            }
        }

        // Perform the shift - this is the magic here:
        if dir == Dir::Right {
            // Move from rightmost box to leftmost (avoid overwriting).
            let mut c = end_c;
            loop {
                // Current box at [c,c+1] → move to [c+1,c+2]
                debug_assert_eq!(self.grid[r0][c], '[');
                debug_assert_eq!(self.grid[r0][c+1], ']');
                debug_assert_eq!(self.grid[r0][c+2], '.'); // guaranteed by the scan

                self.grid[r0][c+2] = ']';
                self.grid[r0][c+1] = '[';
                self.grid[r0][c]   = '.';

                if c == start_c { break; }
                c -= 2;
            }
        } else {
            // Dir::Left — move from leftmost box to rightmost (avoid overwriting).
            let mut c = start_c;
            loop {
                // Box at [c,c+1] → move to [c-1,c]
                debug_assert_eq!(self.grid[r0][c], '[');
                debug_assert_eq!(self.grid[r0][c+1], ']');
                debug_assert_eq!(self.grid[r0][c-1], '.');

                self.grid[r0][c-1] = '[';
                self.grid[r0][c]   = ']';
                self.grid[r0][c+1] = '.';

                if c == end_c { break; }
                c += 2;
            }
        }
        true
    }

    /// For Up/Down:
    ///  - Touching either `[` or `]` means the **whole box** (two tiles) must move.
    ///  - That move might collide with more boxes in the next row — include them and continue.
    ///  - Build the set of boxes to move (BFS/stack), ensure target cells are free of `#`,
    ///    then move all boxes one row in the direction.
    fn try_push_vertical(&mut self, r0: usize, c0: usize, dir: Dir) -> bool {
        // Normalize to the **left bracket** col for the first box we touch.
        let mut start_c = c0;
        if self.grid[r0][c0] == ']' { start_c = c0 - 1; }
        if self.grid[r0][start_c] != '[' { return false; }
        if self.grid[r0][start_c + 1] != ']' { return false; }

        let dr = match dir { Dir::Up => -1isize, Dir::Down => 1isize, _ => 0 };
        let mut stack = vec![(r0, start_c)];
        // Use a set (bool grid) to deduplicate boxes.
        let mut mark = vec![vec![false; self.cols]; self.rows];
        mark[r0][start_c] = true;

        // Collect all boxes that must move together
        while let Some((br, bc)) = stack.pop() {
            let nr = (br as isize + dr) as isize;
            if !self.in_bounds(nr, bc as isize) || !self.in_bounds(nr, (bc+1) as isize) {
                return false;
            }
            let (nr, lcol, rcol) = (nr as usize, bc, bc+1);

            // What sits directly above/below the two halves?
            for cc in [lcol, rcol] {
                match self.grid[nr][cc] {
                    '#' => return false, // wall blocks entire move set
                    '.' => {}            // free
                    '[' => {
                        if !mark[nr][cc] {
                            mark[nr][cc] = true;
                            stack.push((nr, cc));
                        }
                    }
                    ']' => {
                        // We hit the right half of a box; normalize to its left half.
                        if cc == 0 { return false; }
                        let left = cc - 1;
                        if self.grid[nr][left] != '[' { return false; }
                        if !mark[nr][left] {
                            mark[nr][left] = true;
                            stack.push((nr, left));
                        }
                    }
                    _ => return false,   // unexpected; treat as blocked
                }
            }
        }

        // Build a list of all boxes to move from `mark`
        let mut boxes: Vec<(usize, usize)> = Vec::new();
        for r in 0..self.rows {
            for c in 0..self.cols {
                if mark[r][c] { boxes.push((r, c)); }
            }
        }

        // Move all boxes one row in `dir`.
        // Strategy: clear originals, then place at targets (prevents overwrite).
        for &(r, c) in &boxes {
            debug_assert_eq!(self.grid[r][c], '[');
            debug_assert_eq!(self.grid[r][c+1], ']');
            self.grid[r][c] = '.';
            self.grid[r][c+1] = '.';
        }
        let tr: isize = if dir == Dir::Up { -1 } else { 1 };
        for &(r, c) in &boxes {
            let nr = (r as isize + tr) as usize;
            // These must be free (by construction from the BFS).
            debug_assert_eq!(self.grid[nr][c], '.');
            debug_assert_eq!(self.grid[nr][c+1], '.');
            self.grid[nr][c] = '[';
            self.grid[nr][c+1] = ']';
        }
        true
    }

    /// GPS sum for Part 2: count only the **left edge** `[` of each wide box.
    fn gps_sum(&self) -> i64 {
        let mut acc = 0i64;
        for r in 0..self.rows {
            for c in 0..self.cols {
                if self.grid[r][c] == '[' {
                    acc += 100 * r as i64 + c as i64;
                }
            }
        }
        acc
    }

    #[allow(dead_code)]
    fn _render(&self) -> String {
        let mut out = String::new();
        for r in 0..self.rows {
            for c in 0..self.cols {
                if r == self.r && c == self.c { out.push('@'); }
                else { out.push(self.grid[r][c]); }
            }
            out.push('\n');
        }
        out
    }
}

/* **-------- Entrypoint -------- */

/// Runs both parts and prints:
/// ```text
/// Part 1: <sum>
/// Part 2: <sum>
/// ```
pub fn solve() -> Result<()> {
    let input = utils::load_input(2024, 15)?;
    let (lines, moves) = parse_input_raw(&input);

    // Part 1
    let mut wh1 = WarehouseP1::from_lines(&lines);
    for d in moves.iter() { wh1.step(*d); }
    let sum1 = wh1.gps_sum();
    println!("Part 1: {}", sum1);

    // Part 2
    let expanded = expand_map_horizontally(&lines);
    let mut wh2 = WarehouseP2::from_expanded_lines(&expanded);
    for d in moves.iter() { wh2.step(*d); }
    let sum2 = wh2.gps_sum();
    println!("Part 2: {}", sum2);

    Ok(())
}


/* -------Tests --- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_small_example_produces_2028() {
        let small_map = r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########"#;

        let moves = "<^^>>>vv<v>>v<<";
        let input = format!("{small_map}\n\n{moves}\n");
        let (lines, m) = parse_input_raw(&input);
        let mut wh = WarehouseP1::from_lines(&lines);
        for d in m { wh.step(d); }
        assert_eq!(wh.gps_sum(), 2028);
    }

    #[test]
    fn p1_large_example_produces_10092() {
        let grid = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########"#;

        let moves = r#"<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;

        let input = format!("{grid}\n\n{moves}\n");
        let (lines, m) = parse_input_raw(&input);
        let mut wh = WarehouseP1::from_lines(&lines);
        for d in m { wh.step(d); }
        assert_eq!(wh.gps_sum(), 10092);
    }

    #[test]
    fn p2_large_example_produces_9021() {
        // Use the same large example, but part 2 uses expansion + wide-box simulation.
        let grid = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########"#;

        let moves = r#"<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;

        let input = format!("{grid}\n\n{moves}\n");
        let (lines, m) = parse_input_raw(&input);
        let expanded = expand_map_horizontally(&lines);
        let mut wh2 = WarehouseP2::from_expanded_lines(&expanded);
        for d in m { wh2.step(d); }
        assert_eq!(wh2.gps_sum(), 9021);
    }
}
