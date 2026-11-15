# Rustfetch

[![CI](https://github.com/q7xk/rustfetch/actions/workflows/ci.yml/badge.svg)](https://github.com/q7xk/rustfetch/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Rustfetch is a lightweight, fast, minimal system information tool for Linux, written in Rust. It displays essential system details alongside a distro-aware ASCII logo in a clean terminal interface.

## Highlights

- **Fast and minimal:** standard-library-only implementation, no runtime services
- **Distro-aware logos:** 534 embedded ASCII logos with Fastfetch-based coloring
- **Flexible output:** full fetch view, per-field commands, and JSON export
- **Robust by default:** missing or unavailable fields degrade gracefully
- **Linux-focused:** reads native system interfaces (`/proc`, `/sys`, env vars)

## Displayed Information

- OS / distribution and architecture
- Kernel version
- Hostname and username
- Uptime
- Shell, desktop environment, window manager/session, terminal
- CPU model
- GPU (best effort)
- Memory usage
- Disk usage (root filesystem)

## Requirements

- Linux (tested on Arch Linux; intended for major distributions)
- Stable Rust toolchain for building (`cargo`, `rustc`)

No external APIs, internet access, telemetry, or background processes are used at runtime.

## Installation

### From source

```bash
git clone https://github.com/q7xk/rustfetch.git
cd rustfetch
cargo build --release
./target/release/rustfetch
```

### Install script

```bash
./install.sh
```

The script builds a release binary and installs it to `/usr/local/bin` when possible (or falls back to `~/.local/bin`).

### Cargo install

```bash
cargo install --path .
```

Ensure your Cargo bin directory (typically `~/.cargo/bin`) is in `PATH`.

## Usage

```bash
rustfetch                     # Show full system information
rustfetch info                # Show full system information
rustfetch cpu                 # Show CPU model
rustfetch gpu                 # Show GPU information
rustfetch memory              # Show memory usage
rustfetch disk                # Show disk usage
rustfetch os                  # Show operating system name
rustfetch uptime              # Show uptime
rustfetch logo                # Show only the detected distro logo
rustfetch logo <name>         # Show full info with a specific logo
rustfetch logo list           # List all available logo names
rustfetch -d, --distro <name> # Show full info with a specific logo
rustfetch --json              # Output full info as JSON
rustfetch --no-color          # Disable ANSI colors
rustfetch --help              # Show help
rustfetch --version           # Show version
rustfetch --config            # Show configuration path
```

### JSON output

`rustfetch --json` includes: `os`, `distro_id`, `kernel`, `hostname`, `username`, `uptime`, `shell`, `de`, `wm`, `terminal`, `cpu`, `gpu`, `memory`, `disk`, `architecture`.

## Logo Behavior

- Logos are embedded at compile time and selected from the detected distro ID.
- You can override the logo with `--distro <name>` or `rustfetch logo <name>`.
- Run `rustfetch logo list` to discover available logo names.
- Coloring follows Fastfetch ASCII color placeholders/palettes where available.

## Configuration

`rustfetch --config` prints the canonical configuration path:

- `$XDG_CONFIG_HOME/rustfetch/config.toml`, or
- `~/.config/rustfetch/config.toml` if `XDG_CONFIG_HOME` is not set

> The current version does not read or require a configuration file.

## Project Structure

```text
.
├── assets/               # Embedded logo assets
├── docs/                 # Architecture and CLI documentation
├── scripts/              # Helper scripts
├── src/                  # Application source
├── tests/                # Smoke/integration test scripts
├── CHANGELOG.md
├── CONTRIBUTING.md
└── README.md
```

## Development

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all
cargo build --release
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## Limitations

- GPU names are best-effort (derived from sysfs; may fall back to vendor/device IDs or driver names).
- Disk usage is reported for `/`.
- Desktop/session/terminal/shell fields depend on environment variables and runtime context.
- Logos without per-region color placeholders in upstream art may use palette-derived or base coloring.

## Acknowledgements

- Inspired by [Fastfetch](https://github.com/fastfetch-cli/fastfetch) and [Neofetch](https://github.com/dylanaraps/neofetch).
- ASCII logo art and color metadata are derived from Fastfetch sources.

## License

Licensed under the [MIT License](LICENSE).