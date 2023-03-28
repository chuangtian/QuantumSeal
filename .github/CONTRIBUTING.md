# Contributing to QuantumSeal

Thanks for considering a contribution. QuantumSeal is a static analysis tool: it
reads text and never executes or implements cryptography.

## Development setup

- **Rust (stable)** - the CLI and library live in `src/`:
  `cargo build`, `cargo test`, `cargo clippy --all-targets -- -D warnings`.
