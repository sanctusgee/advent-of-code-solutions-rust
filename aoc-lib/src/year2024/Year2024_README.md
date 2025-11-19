# Advent of Code 2024 Solutions - using Rust


This repository contains my [Advent of Code 2024](https://adventofcode.com/2024) solutions in **Rust**. I originally started in Python but switched to Rust after Day 5. I'd been meaning to get more serious about Rust for a while, and AoC seemed like the perfect excuse to dive deeper than just reading the Rust documentation.

## Running a Solution

### Clone the Repository
Clone this repo locally:
```bash
git clone https://github.com/sanctusgee/advent-of-code.git
cd advent-of-code
```


### Run the project:
The `src/main.rs` file is of the form:

```rust
fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { year, day } => run_solution(year, day),
        Commands::List { year } => list_solutions(year),
        Commands::New { year, day } => create_template(year, day),
        Commands::Download { year, day } => download_input(year, day),
    }
}
```

So to run a specific day's solution, use:
```bash
cargo run --bin aoc run <year> <day_number>

# Run a specific day
cargo run --bin aoc run 2024 1

cargo run --bin aoc run 2024 12  # Runs the solution for Year 2024, Day 12
```

## Progress

| Day    | Part 1   | Part 2  |
|--------|----------|---------|
| Day 1  | Done     | Done    |
| Day 2  | Done     | Done    |
| Day 3  | Done     | Done    |
| Day 4  | Done     | Done    |
| Day 5  | Done     | Done    |
| Day 6  | Done     | Done    |
| Day 7  | Done     | Done    |
| Day 8  | Done     | Done    |
| Day 9  | Done     | Done    |
| Day 10 | Done     | Done    |
| Day 11 | Done     | Done    |
| Day 12 | Done     | Done    |
| Day 13 | Done     | Done    |
| Day 14 | Done     | Done    |
| Day 15 | Done     | Done    |
| Day 16 | Done     | Done    |
| Day 17 | Done     | Done    |
| Day 18 | Done     | Done    |
| Day 19 | Done     | Done    |
| Day 20 | Done     | Done    |
| Day 21 | Done     | Done    |
| Day 22 | Done     | Done    |
| Day 23 | Done     | Done    |
| Day 24 | Done     | Done    |
| Day 25 | Done     | Done    |

![aoc2024_complete.png](images/aoc2024_complete.png)

## Utilities and Templates

For more info on project prerequisites, structure, benchmarking, tests, and shared utilities, e.g. downloading Advent of Code inputs from the webiste and creating templates for new years, see the project's [README](README.md).

## License

This project is open-sourced under the MIT License. See [LICENSE](LICENSE) for details.


