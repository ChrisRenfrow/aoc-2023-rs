# Advent of Code 2023 (Rust edition)

This is where I'm documenting my solutions for each day of Advent of Code 2023!

## Usage

### Test
`cargo test -p <DAY>` or simply `cargo test` to run all.

### Run
Just `cargo run` :)

## Project Layout

Each day's solution is packaged in its own workspace under `solutions`. Each solution crate has unit tests to ensure the solution solves the example case(s) provided by the prompt. The top-level crate has each workspace added as a member and is responsible for running each day's solution against the actual input, which is located in the `input` directory.

## TODO:
- [ ] Add some kind of benchmarking so I can compare my solutions
