# Advent of Code 2025

I might be the first person EVER to choose "Rust" as my language of choice for Advent of Code[^1]! 🤯

Credit to [cargo-aoc](https://github.com/gobanos/cargo-aoc) which inspired a bit of the runner design, alongside the F# runner I wrote [last year](https://github.com/fluxehub/AdventOfCode2024). Runner macros were (mostly) written by me, cargo-aoc harness and benchmarking support with [Criterion.rs](https://github.com/criterion-rs/criterion.rs) was written by Claudius Code.

[^1]: No exotic langs or F# this year, sorry :/

# How to use "my" Super Awesome Runner™

## Installation

Install the cargo-aoc runner:

```bash
cargo install --path cargo-aoc
```

Add a `.session` file in the project root with your AoC session cookie to pull input automatically.

## Usage

### Creating a new day

```bash
cargo aoc new 1
```

This creates a `day01` project with the basic boilerplate.

### Running a solution

```bash
cargo aoc 1              # Shorthand
cargo aoc day01          # With explicit day name
cargo aoc run 1          # With explicit run command
cargo aoc 1 --example    # Run with example input
```

The first time you use `--example`, you'll be asked to paste the example input from the puzzle page.

### Benchmarking

```bash
cargo aoc bench <DAY>
```

Runs [Criterion.rs](https://github.com/criterion-rs/criterion.rs) benchmarks for both parts (parsing included).

## Writing a Solution

A minimal solution looks like this:

```rust
use aoc::*;

#[part_one]
fn solve_part_one(input: &str) -> i32 // Can be any impl Display type {
    // input is the raw puzzle input
    67
}

aoc_day!(1);
```

### Parsing

Use the `#[parse]` attribute to transform input before it reaches your part functions:

```rust
// Parse each line individually, collect into Vec
#[parse(line)]
fn parse_numbers(line: &str) -> i32 {
    line.parse().unwrap()
}

#[part_one]
fn sum_numbers(numbers: &[i32]) -> i32 {
    numbers.iter().sum()
}
```

Parse modes:
- `#[parse]` or `#[parse(text)]` - Receives the entire input as `&str`
- `#[parse(line)]` - Called for each line, results collected into `Vec<T>`
- `#[parse(lines)]` - Receives a `Lines` iterator

Parse functions can return `Result<T>` for fallible parsing:

```rust
#[parse(line)]
fn parse_line(line: &str) -> Result<(i32, i32)> {
    let (a, b) = line.split_once(',').ok_or_eyre("missing comma")?;
    Ok((a.parse()?, b.parse()?))
}
```

### Part Functions

Mark your solution functions with `#[part_one]` and `#[part_two]`:

```rust
#[part_one]
fn solve(data: &ParsedType) -> i32 {
    // data is the parsed input (or raw &str if no #[parse])
    67
}
```

The parsed data can be destructured if it's a tuple:

```rust
#[parse(lines)]
fn parse(lines: Lines) -> (Vec<i32>, Vec<i32>) {
    // ...
}

#[part_one]
fn solve(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    // Tuple elements are passed as separate arguments
}
```

Part functions can also return `Result<T>`:

```rust
#[part_two]
fn solve(data: &Data) -> Result<i32> {
    let value = data.find_thing().ok_or_eyre("not found")?;
    Ok(value)
}
```

Both parts run in parallel for faster execution.
