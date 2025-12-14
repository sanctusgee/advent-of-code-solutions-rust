//! Advent of Code 2024 – Day 17 (Parts 1 & 2) — Chronospatial Computer
//!
//! A tiny 3-bit instruction stream drives a machine with full-width
//! integer registers A, B, C. Instructions are pairs (opcode, operand).
//! The instruction pointer (IP) starts at 0 and normally advances by 2,
//! except when `jnz` jumps.
//!
//! Opcodes
//!   0: adv (combo)  A = A / 2^(combo)
//!   1: bxl (lit)    B = B ^ lit
//!   2: bst (combo)  B = (combo % 8)
//!   3: jnz (lit)    if A != 0 { IP = lit } else { IP += 2 }
//!   4: bxc (_)      B = B ^ C           (operand ignored)
//!   5: out (combo)  output (combo % 8)
//!   6: bdv (combo)  B = A / 2^(combo)
//!   7: cdv (combo)  C = A / 2^(combo)
//!
//! Combo operand decoding:
//!   0..=3 => literal 0..=3
//!   4 => A, 5 => B, 6 => C, 7 => (invalid; won't appear in valid programs)
//!
//! Part 1: run the VM with the given initial registers and program;
//!         join all `out` values with commas.
//!
//! Part 2: find the LOWEST positive initial A such that the program’s
//!         `out` stream equals the program bytes exactly (quining). We
//!         build A in base-8 from least-significant “trit” (3 bits) up:
//!         starting with candidates {0}, at each step try appending 0..7,
//!         keep those whose full output ends with the desired suffix. This
//!         leverages the structure of these puzzles where each loop
//!         reduces A (typically by /8), so the search stays tiny.

use crate::utils;
use anyhow::Result;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct Regs {
    a: u64,
    b: u64,
    c: u64,
}

#[derive(Clone, Debug)]
struct Program {
    bytes: Vec<u8>,
}

#[derive(Clone, Debug)]
struct Computer {
    ip: usize,
    regs: Regs,
    prog: Program,
    out: Vec<u8>,
}

impl Computer {
    fn new(regs: Regs, bytes: Vec<u8>) -> Self {
        Self {
            ip: 0,
            regs,
            prog: Program { bytes },
            out: Vec::new(),
        }
    }

    #[inline]
    fn fetch(&self, idx: usize) -> Option<u8> {
        self.prog.bytes.get(idx).copied()
    }

    #[inline]
    fn combo_value(&self, x: u8) -> u64 {
        match x {
            0..=3 => x as u64,
            4 => self.regs.a,
            5 => self.regs.b,
            6 => self.regs.c,
            7 => unreachable!("Combo 7 is reserved and won't appear in valid programs"),
            _ => unreachable!(),
        }
    }

    #[inline]
    fn div_pow2(num: u64, pow: u64) -> u64 {
        if pow >= 64 { 0 } else { num >> (pow as usize) }
    }

    fn step(&mut self) -> bool {
        // returns false on halt
        let op = match self.fetch(self.ip) {
            Some(v) => v,
            None => return false,
        };
        let operand = match self.fetch(self.ip + 1) {
            Some(v) => v,
            None => return false,
        };

        match op {
            0 => { // adv (combo)
                let pow = self.combo_value(operand);
                self.regs.a = Self::div_pow2(self.regs.a, pow);
                self.ip += 2;
            }
            1 => { // bxl (literal)
                self.regs.b ^= operand as u64;
                self.ip += 2;
            }
            2 => { // bst (combo)
                let v = self.combo_value(operand) % 8;
                self.regs.b = v;
                self.ip += 2;
            }
            3 => { // jnz (literal)
                if self.regs.a != 0 {
                    self.ip = operand as usize;
                } else {
                    self.ip += 2;
                }
            }
            4 => { // bxc (ignored operand)
                self.regs.b ^= self.regs.c;
                self.ip += 2;
            }
            5 => { // out (combo)
                let v = (self.combo_value(operand) % 8) as u8;
                self.out.push(v);
                self.ip += 2;
            }
            6 => { // bdv (combo)
                let pow = self.combo_value(operand);
                self.regs.b = Self::div_pow2(self.regs.a, pow);
                self.ip += 2;
            }
            7 => { // cdv (combo)
                let pow = self.combo_value(operand);
                self.regs.c = Self::div_pow2(self.regs.a, pow);
                self.ip += 2;
            }
            _ => return false, // defensive halt
        }
        true
    }

    fn run(&mut self) {
        while self.step() {}
    }
}

fn parse_input(input: &str) -> (Regs, Vec<u8>) {
    // Expected:
    // Register A: <num>
    // Register B: <num>
    // Register C: <num>
    //
    // Program: x,y,z,...
    let mut a = 0u64;
    let mut b = 0u64;
    let mut c = 0u64;
    let mut program: Vec<u8> = Vec::new();

    for line in input.lines().map(|s| s.trim()).filter(|s| !s.is_empty()) {
        if let Some(rest) = line.strip_prefix("Register A:") {
            a = rest.trim().parse::<u64>().expect("parse A");
        } else if let Some(rest) = line.strip_prefix("Register B:") {
            b = rest.trim().parse::<u64>().expect("parse B");
        } else if let Some(rest) = line.strip_prefix("Register C:") {
            c = rest.trim().parse::<u64>().expect("parse C");
        } else if let Some(rest) = line.strip_prefix("Program:") {
            program = rest
                .split(',')
                .map(|t| t.trim().parse::<u8>().expect("byte"))
                .collect();
        }
    }

    (Regs { a, b, c }, program)
}

fn run_program_with(regs: Regs, prog: &[u8]) -> Vec<u8> {
    let mut cpu = Computer::new(regs, prog.to_vec());
    cpu.run();
    cpu.out
}

fn part1_output(input: &str) -> String {
    let (regs, bytes) = parse_input(input);
    let out = run_program_with(regs, &bytes);
    out.into_iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn part2_find_lowest_quine_a(input: &str) -> u64 {
    let (_regs_ignored, program) = parse_input(input);

    // Build A in base-8 (3-bit “digits”) from least significant digit upward.
    // Maintain a small candidate set; at each step, only keep numbers whose
    // FULL output ends with the desired suffix program[i..].
    let mut candidates: Vec<u64> = vec![0];

    for i in (0..program.len()).rev() {
        let want_suffix = &program[i..]; // desired tail
        let mut next: Vec<u64> = Vec::new();

        for &base in &candidates {
            for add in 0u64..8 {
                let a = (base << 3) | add;
                let out = run_program_with(
                    Regs { a, b: 0, c: 0 },
                    &program,
                );

                if out.len() >= want_suffix.len()
                    && &out[out.len() - want_suffix.len()..] == want_suffix
                {
                    next.push(a);
                }
            }
        }

        // De-dup & keep small
        next.sort_unstable();
        next.dedup();
        candidates = next;
        assert!(
            !candidates.is_empty(),
            "No candidates remain at step {i}; check logic."
        );
    }

    // From final candidates, pick the smallest POSITIVE A whose entire output equals program.
    candidates
        .into_iter()
        .filter(|&a| a > 0 && run_program_with(Regs { a, b: 0, c: 0 }, &program) == program)
        .min()
        .expect("No quining A found")
}

pub fn solve() -> Result<()> {
    let input = utils::load_input(2024, 17)?;

    let p1 = part1_output(&input);
    println!("Part 1: {}", p1);

    let p2_a = part2_find_lowest_quine_a(&input);
    println!("Part 2: {}", p2_a);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_small_out_sequence() {
        // If A=10, program 5,0,5,1,5,4 -> outputs 0,1,2
        let input = r#"
Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4
"#;
        assert_eq!(part1_output(input), "0,1,2");
    }

    #[test]
    fn example_long_out_from_desc() {
        // From the problem statement example:
        // A=729, Program: 0,1,5,4,3,0
        // Final output: 4,6,3,5,6,3,5,2,1,0
        let input = r#"
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
"#;
        assert_eq!(part1_output(input), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn puzzle_input_part1() {
        // Provided in chat; expected Part 1 string:
        // 2,0,7,3,0,3,1,3,7
        let input = r#"
Register A: 18427963
Register B: 0
Register C: 0

Program: 2,4,1,1,7,5,0,3,4,3,1,6,5,5,3,0
"#;
        assert_eq!(part1_output(input), "2,0,7,3,0,3,1,3,7");
    }

    #[test]
    fn part2_example_from_prompt() {
        // Example for Part 2 from the description:
        // Program: 0,3,5,4,3,0 quines if A is 117440.
        let input = r#"
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
"#;
        assert_eq!(part2_find_lowest_quine_a(input), 117440);
    }
}
