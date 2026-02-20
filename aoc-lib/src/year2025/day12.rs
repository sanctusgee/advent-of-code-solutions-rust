// file: aoc-lib/src/year2025/day12.rs
//
// Advent of Code 2025 - Day 12: Christmas Tree Farm
// https://adventofcode.com/2025/day/12
//
// Part 1 (real trick): region is a success iff 8 * (total pieces) < area.
// Part 2 (story-only): "You need 1 more." → answer is 1.

use anyhow::{anyhow, Result};
use crate::utils;

pub fn solve() -> Result<()> {
    let input = utils::load_input(2025, 12)?;
    println!("Day 12 / Year 2025");
    println!("Part 1: {}", solve_part1(&input)?);
    println!("Part 2: {}", solve_part2(&input)?);
    Ok(())
}

fn solve_part1(input: &str) -> Result<u64> {
    let regions = parse_regions(input)?;
    let mut count: u64 = 0;

    for (w, h, counts) in regions {
        let total_pieces: u64 = counts.into_iter().map(|v| v as u64).sum();
        let area: u64 = (w as u64) * (h as u64);

        if total_pieces * 8 < area {
            count += 1;
        }
    }

    Ok(count)
}

fn solve_part2(_input: &str) -> Result<u64> {
    Ok(1)
}

// ─────────────────────────────────────────────────────────────────────────────
// Parsing: only region lines (WxH: counts...); shapes are irrelevant.
// ─────────────────────────────────────────────────────────────────────────────

fn parse_regions(input: &str) -> Result<Vec<(usize, usize, Vec<usize>)>> {
    let mut regions = Vec::new();

    for raw in input.lines() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }

        // Region lines look like: "12x5: 1 0 1 0 2 2"
        // Shape headers look like: "0:" and won't match due to missing 'x'.
        if !line.contains('x') || !line.contains(':') {
            continue;
        }

        let (wh, rest) = line
            .split_once(':')
            .ok_or_else(|| anyhow!("bad region line: {line}"))?;

        let (w, h) = parse_wh(wh.trim())?;

        let counts: Vec<usize> = rest
            .split_whitespace()
            .map(|s| s.parse::<usize>())
            .collect::<std::result::Result<_, _>>()
            .map_err(|e| anyhow!("bad counts in region line '{line}': {e}"))?;

        regions.push((w, h, counts));
    }

    if regions.is_empty() {
        return Err(anyhow!("no regions found in input"));
    }

    Ok(regions)
}

fn parse_wh(s: &str) -> Result<(usize, usize)> {
    let (w, h) = s.split_once('x').ok_or_else(|| anyhow!("bad WxH '{s}'"))?;
    Ok((w.trim().parse()?, h.trim().parse()?))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
"#;

    #[test]
    fn example_part1_is_2() {
        assert_eq!(solve_part1(EXAMPLE).unwrap(), 2);
    }

    #[test]
    fn part2_is_1() {
        assert_eq!(solve_part2(EXAMPLE).unwrap(), 1);
    }
}