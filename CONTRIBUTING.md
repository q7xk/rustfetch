# Contributing to Rustfetch

Thank you for considering a contribution to Rustfetch.

## Prerequisites

- Stable Rust toolchain (`rustc`, `cargo`, `rustfmt`, `clippy`)
- Linux environment for full runtime behavior

## Getting Started

```bash
git clone https://github.com/q7xk/rustfetch.git
cd rustfetch
cargo build
cargo run -- --help
```

## Development Guidelines

- Keep dependencies minimal; prefer standard-library solutions.
- Preserve fast startup and graceful fallbacks for unavailable system fields.
- Do not add network access, telemetry, or unnecessary background processes.
- Keep code clear, idiomatic, and maintainable.
- Update documentation (`README.md`, `docs/`, `CHANGELOG.md`) when behavior or CLI changes.
- Add or update tests for parsing/formatting logic where practical.

## Validation

Before opening a pull request, run:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all
cargo build --release
```

Optional smoke check:

```bash
./tests/smoke_test.sh
```

## Pull Requests

- Keep changes focused and describe the motivation and impact.
- Include validation steps/results in the PR description.
- Link related issues when applicable.

## Reporting Issues

Please include:

- Rustfetch version (`rustfetch --version`)
- OS/distribution and environment details
- Steps to reproduce
- Expected vs. actual behavior
- Relevant command output (use `--no-color` if helpful)