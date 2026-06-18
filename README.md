# Rust Learning Workspace

This repository is a multi-crate Rust workspace with several small example crates for learning.

Run individual crates with `cargo run -p <package>` (examples below).

Available crates:
- rt_common: shared library with small helper functions
- hello_world: simple executable that prints a greeting
- cli_tool: small CLI that greets a named user
- http_server: minimal synchronous HTTP responder (for learning purposes)
- math_examples: small algorithms (Fibonacci)

Examples

- Build the whole workspace:

  cargo build

- Run a specific crate:

  cargo run -p hello_world
  cargo run -p cli_tool -- Alice
  cargo run -p http_server
  cargo run -p math_examples
