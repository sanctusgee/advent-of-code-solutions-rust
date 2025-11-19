//! Advent of Code 2024 – Day 23 (Parts 1 & 2) — LAN Party
//!
//! Input
//! Each line is an undirected connection between two computers:
//!   "aa-bb"
//! Computer names are lowercase strings. Treat edges as bidirectional.
//!
//! Part 1
//! Count the number of distinct triangles (3 computers that are all pairwise
//! connected) where at least one computer's name starts with 't'.
//! A triangle {u, v, w} should be counted once regardless of listing order.
//!
//! Part 2
//! Find the largest fully connected set of computers (a maximum clique) and
//! print the list of names in sorted, comma-separated order.
//!
//! Approach
//! - Parse graph and index names to integers for speed.
//! - Part 1: enumerate triangles using index ordering and neighbor set
//!   intersections to avoid double counting.
//! - Part 2: Bron–Kerbosch with pivoting to find a maximum clique.
//!
//! Extra console output
//! We print progress messages to show where we are in the computation.

use std::collections::{HashMap, HashSet};
use crate::utils;
use anyhow::Result;

/// Parse lines like "aa-bb" into a compact undirected graph.
///
/// Returns:
/// - `names`: index -> original name
/// - `adj`: adjacency sets by index (undirected)
fn parse_graph(input: &str) -> (Vec<String>, Vec<HashSet<usize>>) {
    println!("Parsing input...");
    let mut id: HashMap<String, usize> = HashMap::new();
    let mut edges: Vec<(usize, usize)> = Vec::new();

    for line in input.lines().map(|s| s.trim()).filter(|s| !s.is_empty()) {
        let (a, b) = line
            .split_once('-')
            .expect("each line must contain a single '-' separator");

        let get_idx = |name: &str, id: &mut HashMap<String, usize>| -> usize {
            if let Some(&i) = id.get(name) {
                i
            } else {
                let i = id.len();
                id.insert(name.to_string(), i);
                i
            }
        };

        let ia = get_idx(a, &mut id);
        let ib = get_idx(b, &mut id);
        if ia != ib {
            edges.push((ia, ib));
        }
    }

    let n = id.len();
    let mut names = vec![String::new(); n];
    for (name, idx) in id.into_iter() {
        names[idx] = name;
    }

    let mut adj: Vec<HashSet<usize>> = vec![HashSet::new(); n];
    for (u, v) in edges {
        adj[u].insert(v);
        adj[v].insert(u);
    }

    // Basic summary
    let m: usize = adj.iter().map(|s| s.len()).sum::<usize>() / 2;
    println!("Parsed {} nodes, {} edges.", n, m);

    (names, adj)
}

/// Count triangles where at least one name starts with 't'.
///
/// Strategy:
/// - For each u, iterate neighbors v with v > u to enforce ordering.
/// - Intersect neighbors(u) with neighbors(v), and for each w > v that is in the intersection,
///   we have a triangle (u, v, w).
/// - Check the 't' condition on names[u], names[v], names[w].
fn count_triangles_with_t(names: &[String], adj: &[HashSet<usize>]) -> usize {
    println!("Counting qualifying triangles...");
    let n = names.len();
    let mut count = 0usize;

    for u in 0..n {
        for &v in adj[u].iter().filter(|&&v| v > u) {
            // Iterate intersection of smaller set into larger for speed
            let (small, large) = if adj[u].len() < adj[v].len() {
                (&adj[u], &adj[v])
            } else {
                (&adj[v], &adj[u])
            };

            for &w in small {
                if w > v && large.contains(&w) {
                    // Triangle (u, v, w) found
                    let has_t = names[u].starts_with('t')
                        || names[v].starts_with('t')
                        || names[w].starts_with('t');
                    if has_t {
                        count += 1;
                    }
                }
            }
        }
    }

    println!("Finished counting triangles. Total qualifying triangles: {}", count);
    count
}

/// What was old is now new!! Thank CS300 class - can't believe I am using
/// bron_kerbosch_pivot here. Woow!! I thought it was some abstract you-know-what
/// I'd never need
/// 
/// oh, and here's a quick intro for the curious: 
///     https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm
/// 
/// Bron–Kerbosch with pivoting to find a maximum clique.
/// R = current clique, P = candidates, X = already processed.
/// We track best solution globally.
fn bron_kerbosch_pivot(
    adj: &[HashSet<usize>],
    r: &mut Vec<usize>,
    p: &mut HashSet<usize>,
    x: &mut HashSet<usize>,
    best: &mut Vec<usize>,
) {
    if p.is_empty() && x.is_empty() {
        // Found a maximal clique
        if r.len() > best.len() {
            *best = r.clone();
        }
        return;
    }

    // Choose a pivot u from P ∪ X to reduce branching
    let mut union = HashSet::with_capacity(p.len() + x.len());
    union.extend(p.iter().copied());
    union.extend(x.iter().copied());

    let u = union
        .iter()
        .max_by_key(|&&u| adj[u].len())
        .copied()
        .unwrap_or(0);

    // Candidates are P \ N(u)
    let mut candidates: Vec<usize> = p.difference(&adj[u]).copied().collect();

    // Some light ordering can help
    candidates.sort_unstable_by_key(|&v| std::cmp::Reverse(adj[v].len()));

    for v in candidates {
        // R' = R ∪ {v}
        r.push(v);

        // P' = P ∩ N(v)
        // X' = X ∩ N(v)
        let mut p_next = HashSet::new();
        let mut x_next = HashSet::new();
        for &w in p.iter() {
            if adj[v].contains(&w) {
                p_next.insert(w);
            }
        }
        for &w in x.iter() {
            if adj[v].contains(&w) {
                x_next.insert(w);
            }
        }

        bron_kerbosch_pivot(adj, r, &mut p_next, &mut x_next, best);

        // Backtrack - this is the big kahuna. 3 lines can make or break you (if you get them wrong)
        r.pop();
        p.remove(&v);
        x.insert(v);
    }
}

/// Find names of nodes in a maximum clique, sorted and joined with commas.
fn largest_clique_csv(names: &[String], adj: &[HashSet<usize>]) -> String {
    println!("Finding largest clique using Bron–Kerbosch...");
    let n = names.len();

    // Initialize P with all vertices
    let mut p: HashSet<usize> = (0..n).collect();
    let mut x: HashSet<usize> = HashSet::new();
    let mut r: Vec<usize> = Vec::new();
    let mut best: Vec<usize> = Vec::new();

    bron_kerbosch_pivot(adj, &mut r, &mut p, &mut x, &mut best);

    println!("Finished maximum clique search. Best size: {}", best.len());

    let mut best_names: Vec<String> = best.into_iter().map(|i| names[i].clone()).collect();
    best_names.sort();
    best_names.join(",")
}

// I decided to make it a bit more interactive - not jsut print out answers :-)
pub fn solve() -> Result<()> {
    println!("Starting Day 23 solver...");
    let input = utils::load_input(2024, 23)?;

    // Build graph
    let (names, adj) = parse_graph(&input);

    // Part 1
    println!("Processing Part 1: counting triangles with at least one 't*' node...");
    let p1 = count_triangles_with_t(&names, &adj);
    println!("Part 1: {}", p1);

    // Part 2
    println!("Processing Part 2: searching for the largest clique...");
    let p2 = largest_clique_csv(&names, &adj);
    println!("Part 2: {}", p2);

    println!("All steps finished.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Build a tiny graph:
    // ta-b
    // b-c
    // c-ta   -> triangle {ta, b, c} qualifies due to 'ta'
    // d-e    -> separate edge, no triangle
    const SMALL: &str = r#"
ta-b
b-c
c-ta
d-e
"#;

    #[test]
    fn part1_small_triangle_with_t() {
        let (names, adj) = parse_graph(SMALL);
        let triangles = count_triangles_with_t(&names, &adj);
        assert_eq!(triangles, 1);
    }

    #[test]
    fn part2_small_best_clique() {
        let (names, adj) = parse_graph(SMALL);
        let csv = largest_clique_csv(&names, &adj);
        // Largest clique is size 3: {b, c, ta}
        assert_eq!(csv, "b,c,ta");
    }
}
