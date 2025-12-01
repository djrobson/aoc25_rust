# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is an Advent of Code 2025 solutions repository using Rust. It's based on the [advent-of-code-rust template](https://github.com/fspoettel/advent-of-code-rust) which provides a CLI-based workflow for scaffolding, solving, testing, and benchmarking daily puzzles.

## Project Structure

- `src/bin/` - Individual solution binaries for each day (e.g., `01.rs`, `02.rs`)
- `src/template/` - Template infrastructure code (commands, runner, etc.)
- `src/lib.rs` - Entry point for shared helper functions and modules
- `data/inputs/` - Real puzzle inputs (one file per day)
- `data/examples/` - Example inputs for testing (one file per day)
- `data/puzzles/` - Downloaded puzzle descriptions in markdown format

## Key Commands

### Development Workflow
- `cargo scaffold <day>` - Create a new solution file from template
- `cargo solve <day>` - Run solution against real input
- `cargo test` - Run all tests
- `cargo test --bin <day>` - Run tests for a specific day
- `cargo test --bin <day> part_one` - Run specific test for a day

### Building and Quality
- `cargo fmt` - Format code
- `cargo clippy` - Lint code

### Benchmarking
- `cargo time <day>` - Benchmark a specific day
- `cargo time --all` - Benchmark all solutions
- `cargo time <day> --store` - Benchmark and update README

### Running Multiple Solutions
- `cargo all` - Run all solutions sequentially
- `cargo all --release` - Run all solutions with optimizations

### Optional aoc-cli Integration
If aoc-cli is installed:
- `cargo download <day>` - Download puzzle input and description
- `cargo scaffold <day> --download` - Scaffold and download in one step
- `cargo read <day>` - Read puzzle description in terminal
- `cargo solve <day> --submit <part>` - Submit solution (part 1 or 2)
- `cargo today` - During December, scaffold/download/read current day

### Advanced Options
- `cargo solve <day> --release` - Run optimized build
- `cargo solve <day> --dhat` - Profile heap allocations

## Solution Architecture

### Solution Template Structure
Each day's solution in `src/bin/<day>.rs` follows this pattern:

```rust
advent_of_code::solution!(day_number);

pub fn part_one(input: &str) -> Option<u64> {
    // Solution implementation
}

pub fn part_two(input: &str) -> Option<u64> {
    // Solution implementation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None); // Update with expected value
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None); // Update with expected value
    }
}
```

### Important Macros and Helpers

- `advent_of_code::solution!(day)` - Macro that sets up the solution binary, creates the `DAY` constant, and wires up the runner
- `advent_of_code::template::read_file(folder, DAY)` - Read input files from `data/{folder}/{day}.txt`
- `advent_of_code::template::read_file_part(folder, DAY, part)` - Read input files with part suffix (e.g., `01-2.txt`) for days with multiple example inputs

### Command Line Infrastructure

The main binary (`src/main.rs`) uses `pico-args` for parsing and dispatches to command handlers in `src/template/commands/`:
- `scaffold.rs` - Creates new solution files from template
- `solve.rs` - Runs solutions
- `download.rs` - Downloads inputs via aoc-cli
- `read.rs` - Displays puzzle descriptions
- `time.rs` - Benchmarking logic
- `all.rs` - Runs all solutions

## Development Guidelines

### When implementing solutions:
1. Start with `cargo scaffold <day>` to create the solution file
2. Add example input to `data/examples/<day>.txt`
3. Implement and test against example input using the unit tests
4. VS Code with rust-analyzer will show "Run Test" / "Debug Test" buttons above test functions
5. Once tests pass, add real input to `data/inputs/<day>.txt` and run `cargo solve <day>`
6. Return `Option<u64>` from part functions (or adjust return type as needed)

### Testing with multiple example files:
If a day requires multiple example inputs, create files like `01-2.txt`, `01-3.txt` and use:
```rust
let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
```

### Shared code:
Add helper functions and reusable modules to `src/lib.rs`. Import them in solutions with `use advent_of_code::module_name;`.

## Cargo Features

- `dhat-heap` - Enable heap profiling with DHAT
- `today` - Enable the `cargo today` command (only works during December)
- `test_lib` - For internal testing

## Edition and Dependencies

- Rust edition: 2024
- Key dependencies:
  - `pico-args` - Command-line argument parsing
  - `tinyjson` - JSON parsing for benchmark data
  - `chrono` - Date handling for `today` command (optional)
  - `dhat` - Heap profiling (optional)
