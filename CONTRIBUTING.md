# Contributing to DepegGuard Signal Logger

## Getting started

1. Fork the repository and clone locally
2. Install the Rust toolchain: `rustup toolchain install nightly-2026-01-01`
3. Add the wasm target: `rustup target add wasm32-unknown-unknown`
4. Install Odra CLI: `cargo install cargo-odra`

## Development workflow

```bash
cd signal-logger
cargo odra build          # compile contract
cargo odra test           # run tests against mock env
cargo odra build -b casper  # build wasm for Casper
```

## Submitting changes

- Open an issue first for non-trivial changes
- Branch from `main`, PR back to `main`
- Include test coverage for any new entry points
- Run `cargo clippy` and `cargo fmt` before pushing

## Reporting bugs

Use the [bug report template](.github/ISSUE_TEMPLATE/bug_report.md). Include the Casper transaction hash if the bug is on-chain.

## Code of conduct

This project follows the [Contributor Covenant](CODE_OF_CONDUCT.md).
