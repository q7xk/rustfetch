# Contributing to Rustfetch

## Development

Requirements:
- Stable Rust toolchain (`rustc`, `cargo`)
- Linux environment for full runtime behavior

Build and run:

```bash
cargo build
cargo run --quiet --
```

## Guidelines

- Keep dependencies minimal and prefer standard library solutions.
- Preserve fast startup and graceful fallbacks for unavailable system fields.
- Do not add network access, telemetry, or unnecessary background processes.
- Keep code clean, idiomatic, and maintainable.
- Update `README.md` and `CHANGELOG.md` when behavior or CLI changes.

## Validation

Before submitting changes, run:

```bash
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test
cargo build --release
```
