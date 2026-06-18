# Rust Learning Workspace

This repository is a focused Rust workspace containing a single crate with multiple example binaries.

Run the binaries in `crates/bin_examples/src/bin/` using `cargo run -p bin_examples --bin <name>`.

Available binaries (in crates/bin_examples/src/bin):
- hello_world
- cli_tool
- http_server
- math_examples

Examples

- Build the workspace:

  cargo build

- Run a specific binary:

  cargo run -p bin_examples --bin hello_world
  cargo run -p bin_examples --bin cli_tool -- Alice
  cargo run -p bin_examples --bin http_server
  cargo run -p bin_examples --bin math_examples
