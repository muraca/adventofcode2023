# Advent of Code 2023

In this repository you can find my Rust solutions for the [Advent of Code 2023](https://adventofcode.com/2023) challenge.

## Usage
This was designed to run using `cargo test`.

To test all the problems I have solved so far against the examples provided by the challenge, run:
```bash
cargo test test
```

If you want to test a specific problem, run:
```bash
cargo test day01::test::problem1
```

Or if you want to retrieve the solution for a specific problem, run:
```bash
cargo test day01::solution::problem2 -- --nocapture
```
Be aware that some of these solutions may intentionally fail, as they take a lot of time to run, so I intentionally put a panic to avoid running them by mistake.
