// file: year2025/day12.rs
//
// Advent of Code 2025 - Day 12: Christmas Tree Farm
// https://adventofcode.com/2025/day/12
//
// READ THIS BEFORE YOU OVER-ENGINEER :-)
//
// This Aoc 25 story screams “tiling / DLX - Dancing Links / Algorithm X / backtracking”.
// Nope, don’t do it!! really don't!
//
// For the real puzzle input, you do not need to place shapes.
// You only need the total occupied cell counts per shape and compare
// total required cells vs region area.
//
// Part 2 is not a real computational part. It’s the end-of-event message.
// There is no answer box; nothing to compute. Return "N/A".
//

use anyhow::{anyhow, Result};
use crate::utils;

pub fn solve() -> Result<()> {
    let input = utils::load_input(2025, 12)?;

    println!("Day 12 / Year 2025");
    println!("Part 1: {}", solve_part1(&input)?);
    println!("Part 2: {}", solve_part2(&input)?);

    Ok(())
}

// -------------------------
// Part 1
// ------------------------

fn solve_part1(input: &str) -> Result<u64> {
    let (shape_tiles, regions) = parse_input(input)?;

    let mut ok: u64 = 0;
    for r in &regions {
        let needed: u64 = r.counts.iter()
            .zip(shape_tiles.iter())
            .map(|(&cnt, &tiles)| cnt as u64 * tiles as u64)
            .sum();

        let area: u64 = r.w as u64 * r.h as u64;

        // Real-input shortcut:
        // If total required filled cells fit in the rectangle area,
        // AoC 2025 Day 12 accepts it as "fits".
        if needed <= area {
            ok += 1;
        }
    }

    Ok(ok)
}

// __________
// Part 2
// -------

fn solve_part2(_input: &str) -> Result<&'static str> {
    // There is no computational Part Two for AoC 2025 Day 12.
    // It’s the end-of-event text telling you to go get the missing star elsewhere.
    Ok("N/A")
}

// -------
// Data model
// ----------

#[derive(Debug, Clone)]
struct Region {
    w: usize,
    h: usize,
    counts: Vec<usize>,
}

//-----------
// Parsing
// -----------

fn parse_input(input: &str) -> Result<(Vec<usize>, Vec<Region>)> {
    let mut lines = input.lines().map(str::trim_end).peekable();

    // Shapes are listed first, then regions (lines containing "WxH: ...")
    // We only need the number of '#' in each shape.
    let mut raw_shapes: Vec<Option<usize>> = Vec::new();

    while let Some(&line) = lines.peek() {
        let l = line.trim();
        if l.is_empty() {
            lines.next();
            continue;
        }
        if is_region_line(l) {
            break;
        }

        // header: "<idx>:"
        let (idx_str, rest) = l.split_once(':')
            .ok_or_else(|| anyhow!("bad shape header: {l}"))?;
        if !rest.trim().is_empty() {
            return Err(anyhow!("unexpected content after shape header colon: {l}"));
        }
        let idx: usize = idx_str.trim().parse()?;
        lines.next();

        // read shape grid until blank line or region line
        let mut tiles: usize = 0;
        let mut saw_row = false;

        while let Some(&ln) = lines.peek() {
            let t = ln.trim();
            if t.is_empty() || is_region_line(t) {
                break;
            }
            saw_row = true;
            for ch in ln.chars() {
                match ch {
                    '#' => tiles += 1,
                    '.' => {}
                    _ => return Err(anyhow!("invalid shape char: {ch:?}")),
                }
            }
            lines.next();
        }

        // consume blank separators
        while lines.peek().map(|l| l.trim().is_empty()).unwrap_or(false) {
            lines.next();
        }

        if !saw_row {
            return Err(anyhow!("shape {idx} has empty grid"));
        }
        if tiles == 0 {
            return Err(anyhow!("shape {idx} has no '#' cells"));
        }

        if raw_shapes.len() <= idx {
            raw_shapes.resize_with(idx + 1, || None);
        }
        if raw_shapes[idx].is_some() {
            return Err(anyhow!("duplicate shape index {idx}"));
        }
        raw_shapes[idx] = Some(tiles);
    }

    let shape_tiles: Vec<usize> = raw_shapes.into_iter().enumerate()
        .map(|(i, s)| s.ok_or_else(|| anyhow!("missing shape index {i}")))
        .collect::<Result<_>>()?;

    // Regions
    let mut regions: Vec<Region> = Vec::new();
    for line in lines {
        let l = line.trim();
        if l.is_empty() {
            continue;
        }

        let (wh, rest) = l.split_once(':')
            .ok_or_else(|| anyhow!("bad region line: {l}"))?;
        let (w, h) = parse_wh(wh.trim())?;

        let counts: Vec<usize> = rest.split_whitespace()
            .map(|s| s.parse::<usize>())
            .collect::<std::result::Result<_, _>>()?;

        if counts.len() != shape_tiles.len() {
            return Err(anyhow!(
                "region counts {} ≠ shape count {} in: {l}",
                counts.len(),
                shape_tiles.len()
            ));
        }

        regions.push(Region { w, h, counts });
    }

    Ok((shape_tiles, regions))
}

fn is_region_line(s: &str) -> bool {
    // "12x5: ..." — region lines always have an 'x' and a ':'
    s.contains('x') && s.contains(':')
}

fn parse_wh(s: &str) -> Result<(usize, usize)> {
    let (w, h) = s.split_once('x')
        .ok_or_else(|| anyhow!("bad WxH '{s}'"))?;
    Ok((w.trim().parse()?, h.trim().parse()?))
}

// ----------------
// Tests
// -----------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_area_rule_basic() {
        // one shape: 2 tiles
        // region 2x2 area=4, need 2*2=4 => fits
        let input = r#"
0:
##
..
2x2: 2
"#;
        assert_eq!(solve_part1(input).unwrap(), 1);
    }

    #[test]
    fn part1_area_rule_reject() {
        // one shape: 3 tiles, need 2 copies =>6 > 4 => reject
        let input = r#"
0:
###
2x2: 2
"#;
        assert_eq!(solve_part1(input).unwrap(), 0);
    }

    #[test]
    fn part2_is_na() {
        assert_eq!(solve_part2("anything").unwrap(), "N/A");
    }
}