# Contributing to QuantumSeal

Thanks for considering a contribution. QuantumSeal is a static analysis tool: it
reads text and never executes or implements cryptography.

## Development setup

- **Rust (stable)** - the CLI and library live in `src/`:
  `cargo build`, `cargo test`, `cargo clippy --all-targets -- -D warnings`.
- **Node 20+** - the viewer lives in `viewer/`:
  `npm install`, `npm run typecheck`, `npm run build`, `npm test`.

## Before you open a pull request

1. `cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test`
2. `cd viewer && npm run typecheck && npm test`
