# Advent of Code - Rust Solutions

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

This repository contains my [Advent of Code](https://adventofcode.com/) solutions in **Rust**.

Built using the [Advent of Code Rust Template](https://github.com/sanctusgee/advent-of-code-rust-template/tree/main) for multi-year puzzle solving.

## My Solutions

### Year 2024
All 25 days completed! See [Year 2024 README](aoc-lib/src/year2024/Year2024_README.md) for details.

### Year 2025
Work in progress. See solutions in `aoc-lib/src/year2025/`.

## Running Solutions

```bash
# Run a specific day
cargo run --bin aoc run 2024 1
cargo run --bin aoc run 2025 1

# List all solutions
cargo run --bin aoc list
```

## Project Structure

```
├── aoc-lib/src/
│   ├── year2024/          # 2024 solutions (complete)
│   ├── year2025/          # 2025 solutions (in progress)
│   └── utils/             # Shared utilities
├── input/                 # Puzzle inputs (gitignored)
└── benches/               # Performance benchmarks
```

## License

MIT
