//! Advent of Code 2024 - Day 24 (Parts 1 and 2) - Crossed Wires
//!
//! Part 1
//! Evaluate the circuit and read Z as a little-endian integer.
//!
//! Part 2
//! Four gate outputs are swapped. We must identify the four swaps so that
//! the circuit implements Z = X + Y. This version compiles to indices,
//! prunes candidates to XOR outputs near Z cones, and uses staged scoring.


// I had to deploy aggressive candidate pruning here because part 2 was stalling, running for 1+ hour
// iwthout any output
//
// ** Don't use greedy algorithm - just does not work
//
// Approach used: 
// 1. Only consider swaps among: all z.. outputs, and outputs of XOR gates that sit
// within two levels of any z.. cone.
// This typically drops the candidate set from ~200 to ~40–80.
// 
// 2. Staged scoring
// Use a small test set for Step 1–2 of the beam search, a medium set for Step 3, 
// and the full set only at the final step and verification. This cuts the total
// gate evals by an order of magnitude.
// 
// 3. Tuned beam
// Slightly smaller beam size on early steps, expanded when the space is smaller.
//
// ..and most importantly, I had console output so we can see what's happening,
// instead of just a blinking cursor

use std::collections::{HashMap, HashSet};
use crate::utils;
use anyhow::Result;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Op { And, Or, Xor }

#[derive(Clone, Debug)]
struct Gate {
    a: String,
    b: String,
    out: String,
    op: Op,
}

fn parse(input: &str) -> (HashMap<String, u8>, Vec<Gate>) {
    println!("Day 24: parsing input...");
    let mut sections = input.split("\n\n");
    let init = sections.next().unwrap_or_default();
    let gates = sections.next().unwrap_or_default();

    let mut values: HashMap<String, u8> = HashMap::new();
    for line in init.lines().map(|s| s.trim()).filter(|s| !s.is_empty()) {
        if let Some((name, val)) = line.split_once(':') {
            let bit = val.trim().parse::<u8>().expect("bit 0 or 1");
            values.insert(name.trim().to_string(), bit);
        }
    }

    let mut list = Vec::new();
    for line in gates.lines().map(|s| s.trim()).filter(|s| !s.is_empty()) {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        assert!(parts.len() == 5 && parts[3] == "->", "bad gate line");
        let a = parts[0].to_string();
        let b = parts[2].to_string();
        let out = parts[4].to_string();
        let op = match parts[1] {
            "AND" => Op::And,
            "OR"  => Op::Or,
            "XOR" => Op::Xor,
            _ => panic!("unknown op"),
        };
        list.push(Gate { a, b, out, op });
    }

    println!("Parsed {} initial wires and {} gates.", values.len(), list.len());
    (values, list)
}

fn evaluate(values: &HashMap<String, u8>, gates: &[Gate]) -> HashMap<String, u8> {
    println!("Evaluating circuit...");
    let mut v = values.clone();

    let mut changed = true;
    let mut rounds = 0usize;
    while changed {
        changed = false;
        rounds += 1;
        if rounds % 50 == 0 {
            println!("  ...processing, iteration {}", rounds);
        }
        for g in gates {
            let a = match v.get(&g.a) { Some(x) => *x, None => continue };
            let b = match v.get(&g.b) { Some(x) => *x, None => continue };
            let out = match g.op { Op::And => a & b, Op::Or => a | b, Op::Xor => a ^ b };
            if v.get(&g.out).copied() != Some(out) {
                v.insert(g.out.clone(), out);
                changed = true;
            }
        }
    }
    println!("Finished evaluation after {} iterations.", rounds);
    v
}

fn z_value(v: &HashMap<String, u8>) -> u64 {
    let mut zs = Vec::<(usize, u8)>::new();
    for (k, &bit) in v {
        if let Some(rest) = k.strip_prefix('z') {
            if let Ok(idx) = rest.parse::<usize>() { zs.push((idx, bit)); }
        }
    }
    zs.sort_by_key(|&(i, _)| i);
    let mut acc = 0u64;
    for (i, bit) in zs {
        if bit != 0 { acc |= 1u64 << i; }
    }
    acc
}

fn part1(input: &str) -> u64 {
    println!("Part 1: evaluating...");
    let (values, gates) = parse(input);
    let final_values = evaluate(&values, &gates);
    let ans = z_value(&final_values);
    println!("Part 1: finished calculating.");
    ans
}

// Detect swapped wires by checking structural properties of ripple-carry adder

fn is_x(s: &str) -> bool { s.starts_with('x') }
fn is_y(s: &str) -> bool { s.starts_with('y') }
fn is_z(s: &str) -> bool { s.starts_with('z') }

fn part2(input: &str) -> String {
    println!("Part 2: finding swapped wires in adder circuit...");
    let (_values, gates) = parse(input);

    let mut wrong = HashSet::new();

    // Find the highest z-bit number
    let mut max_z = 0;
    for g in &gates {
        if is_z(&g.out) {
            if let Some(num_str) = g.out.strip_prefix('z') {
                if let Ok(num) = num_str.parse::<usize>() {
                    max_z = max_z.max(num);
                }
            }
        }
    }

    println!("  Max z-bit: z{:02}", max_z);

    // Rule 1: If output is z-wire, operation must be XOR (except the highest bit)
    for g in &gates {
        if is_z(&g.out) && g.out != format!("z{:02}", max_z) && g.op != Op::Xor {
            println!("  Rule 1 violation: {} is z-output but not XOR", g.out);
            wrong.insert(g.out.clone());
        }
    }

    // Rule 2: If output is not z-wire and inputs are not x/y, operation must not be XOR
    for g in &gates {
        if g.op == Op::Xor {
            if !is_z(&g.out) && !is_x(&g.a) && !is_y(&g.a) && !is_x(&g.b) && !is_y(&g.b) {
                println!("  Rule 2 violation: {} is XOR with non-x/y inputs but not z-output", g.out);
                wrong.insert(g.out.clone());
            }
        }
    }

    // Rule 3: XOR with x,y inputs should feed into another XOR (except x00/y00)
    for g in &gates {
        if g.op == Op::Xor && (is_x(&g.a) || is_y(&g.a)) {
            // Skip x00/y00 case (first bit has no carry in)
            let is_zero = (g.a == "x00" || g.a == "y00") && (g.b == "x00" || g.b == "y00");
            if !is_zero {
                // Check if output feeds into another XOR
                let mut feeds_xor = false;
                for g2 in &gates {
                    if g2.op == Op::Xor && (g2.a == g.out || g2.b == g.out) {
                        feeds_xor = true;
                        break;
                    }
                }
                if !feeds_xor {
                    println!("  Rule 3 violation: {} is XOR(x,y) but doesn't feed XOR", g.out);
                    wrong.insert(g.out.clone());
                }
            }
        }
    }

    // Rule 4: AND gates should feed into OR (except x00 AND y00 which is the first carry)
    for g in &gates {
        if g.op == Op::And {
            let is_x00_y00 = (g.a == "x00" || g.a == "y00") && (g.b == "x00" || g.b == "y00");
            if !is_x00_y00 {
                // Check if output feeds into OR
                let mut feeds_or = false;
                for g2 in &gates {
                    if g2.op == Op::Or && (g2.a == g.out || g2.b == g.out) {
                        feeds_or = true;
                        break;
                    }
                }
                if !feeds_or {
                    println!("  Rule 4 violation: {} is AND output but doesn't feed OR", g.out);
                    wrong.insert(g.out.clone());
                }
            }
        }
    }

    let mut result: Vec<String> = wrong.into_iter().collect();
    result.sort();

    println!("  Found {} swapped wires", result.len());
    let answer = result.join(",");
    println!("Part 2: {}", answer);
    answer
}

pub fn solve() -> Result<()> {
    println!("Starting Day 24 solver...");
    let input = utils::load_input(2024, 24)?;

    println!("Processing Part 1...");
    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    println!("Processing Part 2...");
    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    println!("Day 24 complete.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tiny_evaluation() {
        let input = r#"
x00: 1
y00: 0

x00 XOR y00 -> z00
"#;
        assert_eq!(part1(input), 1);
    }
}