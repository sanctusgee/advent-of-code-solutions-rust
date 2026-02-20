// file: year2025/day11.rs
//
// Advent of Code
// Day 11: Reactor
//
// https://adventofcode.com/2025/day/11

use anyhow::{anyhow, Result};
use crate::utils;

pub fn solve() -> Result<()> {
    let input = utils::load_input(2025, 11)?;

    let part1 = solve_part1(&input)?;
    let part2 = solve_part2(&input)?;

    println!("Day 11 / Year 2025");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

// -------------
// Part 1
// -------------------
//
// Count distinct directed paths from "you" to "out".
//
fn solve_part1(input: &str) -> Result<u64> {
    let g = Graph::parse(input)?;
    g.count_paths("you", "out", &[])
}

// -------------
// Part 2
// -------------------
//
// Count distinct directed paths from "svr" to "out" that visit BOTH "dac" and "fft" (any order).
//
fn solve_part2(input: &str) -> Result<u64> {
    let g = Graph::parse(input)?;
    g.count_paths("svr", "out", &["dac", "fft"])
}

// - Intern node names -> usize IDs once during parsing.
// - DP state is (node_id, mask) where mask tracks which required nodes have been visited.
// - DFS + memoization computes number of paths from (node,mask) to end satisfying requirements.
// - Cycle detection per-state. If a reachable cycle exists, the number of paths can be infinite;
use std::collections::HashMap;

#[derive(Debug)]
struct Graph {
    id_of: HashMap<String, usize>,
    name_of: Vec<String>,
    next: Vec<Vec<usize>>,
}

impl Graph {
    fn parse(input: &str) -> Result<Self> {
        let mut id_of: HashMap<String, usize> = HashMap::new();
        let mut name_of: Vec<String> = Vec::new();

        // Intern a name and return its id.
        fn intern(id_of: &mut HashMap<String, usize>, name_of: &mut Vec<String>, s: &str) -> usize {
            if let Some(&id) = id_of.get(s) {
                return id;
            }
            let id = name_of.len();
            id_of.insert(s.to_string(), id);
            name_of.push(s.to_string());
            id
        }

        // Collect edges (from_id, outs_ids) while interning.
        let mut edges: Vec<(usize, Vec<usize>)> = Vec::new();

        for raw in input.lines() {
            let line = raw.trim();
            if line.is_empty() {
                continue;
            }

            let (lhs, rhs) = line
                .split_once(':')
                .ok_or_else(|| anyhow!("bad line (missing ':'): {line}"))?;

            let from_name = lhs.trim();
            if from_name.is_empty() {
                return Err(anyhow!("bad line (empty device name): {line}"));
            }

            let from_id = intern(&mut id_of, &mut name_of, from_name);

            let mut outs: Vec<usize> = Vec::new();
            for tok in rhs.split_whitespace() {
                let to_name = tok.trim();
                if to_name.is_empty() {
                    continue;
                }
                let to_id = intern(&mut id_of, &mut name_of, to_name);
                outs.push(to_id);
            }

            edges.push((from_id, outs));
        }

        // Build adjacency list. Nodes that only appear on RHS will have empty adjacency.
        let mut next: Vec<Vec<usize>> = vec![Vec::new(); name_of.len()];

        // Detect duplicate definitions for stability.
        let mut defined: Vec<bool> = vec![false; name_of.len()];
        for (from_id, outs) in edges {
            if defined[from_id] {
                return Err(anyhow!("duplicate device definition: {}", name_of[from_id]));
            }
            defined[from_id] = true;
            next[from_id] = outs;
        }

        Ok(Self { id_of, name_of, next })
    }

    fn id(&self, name: &str) -> Result<usize> {
        self.id_of
            .get(name)
            .copied()
            .ok_or_else(|| anyhow!("unknown device: {name}"))
    }

    // Count paths from start->end, requiring that all nodes in `required` are visited.
    // `required` can be empty (Part 1).
    fn count_paths(&self, start: &str, end: &str, required: &[&str]) -> Result<u64> {
        let start_id = self.id(start)?;
        let end_id = self.id(end)?;

        // Deduplicate required ids
        let mut req_ids: Vec<usize> = Vec::new();
        for &r in required {
            let rid = self.id(r)?;
            if !req_ids.contains(&rid) {
                req_ids.push(rid);
            }
        }

        // 2^k state space; k is tiny for AoC. Guard anyway.
        if req_ids.len() > 20 {
            return Err(anyhow!("too many required nodes ({}): mask too large", req_ids.len()));
        }

        let k = req_ids.len();
        let states = 1usize << k;
        let full_mask: u32 = if k == 0 { 0 } else { (1u32 << k) - 1 };

        // For fast updates: required_bit[node_id] = bit to OR into mask (or 0).
        let mut required_bit: Vec<u32> = vec![0; self.name_of.len()];
        for (i, &rid) in req_ids.iter().enumerate() {
            required_bit[rid] = 1u32 << i;
        }

        let start_mask = required_bit[start_id];

        // memo[idx] caches the number of valid paths from (node,mask) to end.
        // idx = node * states + mask
        let mut memo: Vec<Option<u64>> = vec![None; self.name_of.len() * states];

        // visiting[idx] is recursion stack marker for cycle detection.
        // 0 = not in stack, 1 = in stack
        let mut visiting: Vec<u8> = vec![0; self.name_of.len() * states];

        fn dfs(
            g: &Graph,
            node: usize,
            mask: u32,
            end_id: usize,
            states: usize,
            full_mask: u32,
            required_bit: &[u32],
            memo: &mut [Option<u64>],
            visiting: &mut [u8],
        ) -> Result<u64> {
            // If we hit end, count only if requirements satisfied.
            if node == end_id {
                return Ok(if mask == full_mask { 1 } else { 0 });
            }

            let idx = node * states + mask as usize;

            if let Some(v) = memo[idx] {
                return Ok(v);
            }

            // Cycle detection at (node,mask) granularity.
            if visiting[idx] == 1 {
                return Err(anyhow!("cycle detected in constrained state at node_id={node} mask={mask:02b}"));
            }
            visiting[idx] = 1;

            let mut total: u64 = 0;

            for &nxt in &g.next[node] {
                let next_mask = mask | required_bit[nxt];
                let add = dfs(
                    g,
                    nxt,
                    next_mask,
                    end_id,
                    states,
                    full_mask,
                    required_bit,
                    memo,
                    visiting,
                )?;
                total = total
                    .checked_add(add)
                    .ok_or_else(|| anyhow!("path count overflow (too many paths)"))?;
            }

            visiting[idx] = 0;
            memo[idx] = Some(total);

            Ok(total)
        }

        dfs(
            self,
            start_id,
            start_mask,
            end_id,
            states,
            full_mask,
            &required_bit,
            &mut memo,
            &mut visiting,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1_paths_is_5() {
        let input = r#"
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
"#;

        assert_eq!(solve_part1(input).unwrap(), 5);
    }

    #[test]
    fn example_part2_paths_is_2() {
        let input = r#"
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
"#;

        assert_eq!(solve_part2(input).unwrap(), 2);
    }

    #[test]
    fn cycle_is_error() {
        let input = r#"
you: a
a: b
b: a
out:
"#;

        let g = Graph::parse(input).unwrap();
        assert!(g.count_paths("you", "out", &[]).is_err());
    }
}