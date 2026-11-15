# Contributing to Ultrafetch

Thank you for considering a contribution to Ultrafetch.

## Prerequisites

- Stable Rust toolchain (`rustc`, `cargo`, `rustfmt`, `clippy`, Rust >= 1.74)
- Linux environment for full runtime behavior

## Getting Started

```bash
git clone https://github.com/q7xk/ultrafetch.git
cd ultrafetch
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
- Keep packaging templates and metadata (`packaging/`, `Cargo.toml`, man page) in sync for releases.

## Validation

Before opening a pull request, run:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all
cargo build --release --locked
```

Optional smoke check:

```bash
./tests/smoke_test.sh
```

## Releases and Packaging

- Bump version in `Cargo.toml` and update `CHANGELOG.md` / `docs/ultrafetch.1`.
- Create a tag `v<version>` and push it to trigger the release workflow.
- Generate a source tarball with `./scripts/create-release-tarball.sh` when needed for downstream submissions.
- Refresh checksums/versions in `packaging/` templates as appropriate (see `docs/PACKAGING.md`).

## Pull Requests

- Keep changes focused and describe the motivation and impact.
- Include validation steps/results in the PR description.
- Link related issues when applicable.

## Reporting Issues

Please include:

- Ultrafetch version (`ultrafetch --version`)
- OS/distribution and environment details
- Steps to reproduce
- Expected vs. actual behavior
- Relevant command output (use `--no-color` if helpful)
