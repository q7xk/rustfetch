# Rustfetch

Rustfetch is a lightweight, fast, minimal system information tool written entirely in Rust. It displays essential system information in a clean terminal interface inspired by tools like Fastfetch and Neofetch.

## Features

- Operating system and distribution
- Kernel version
- Hostname and username
- CPU model
- Total and used RAM
- GPU information (best effort, when available)
- Desktop environment and window manager/session
- Shell
- Terminal name
- Uptime
- System architecture
- Colorized output with a customizable ASCII logo
- Minimal dependencies (standard library only)

## Requirements

- Stable Rust toolchain (`cargo`, `rustc`)
- Linux (tested on Arch Linux; designed for major Linux distributions)
- No external APIs, internet connection, telemetry, or background processes

## Build

```bash
cargo build --release
```

The compiled binary will be at:

```bash
./target/release/rustfetch
```

## Install

```bash
cargo install --path .
```

Or copy the release binary to a directory in your `PATH`.

## Usage

```bash
rustfetch              # Display system information
rustfetch --help       # Show help
rustfetch --version    # Show version
rustfetch --no-color   # Disable terminal colors
rustfetch --logo       # Display only the logo
rustfetch --config     # Show configuration location
```

## Logo Customization

The default ASCII logo is embedded from `assets/logo.txt` at compile time. To customize it, edit `assets/logo.txt` and rebuild the project.

## Configuration

`rustfetch --config` prints the canonical configuration path:

- `$XDG_CONFIG_HOME/rustfetch/config.toml`, or
- `~/.config/rustfetch/config.toml` if `XDG_CONFIG_HOME` is not set

The MVP does not require or parse a configuration file.

## Notes and Limitations

- GPU detection is best-effort using Linux sysfs (`/sys/class/drm/...`) and may not provide a fully human-readable model name on all systems.
- Desktop environment, window manager/session, and terminal information depend on environment variables available in the current session.
- Missing or unavailable fields are handled gracefully and reported as `Unknown` (or omitted for GPU when unavailable).

## Project Structure

```text
rustfetch/
├── Cargo.toml
├── LICENSE
├── README.md
├── src/
│   ├── main.rs
│   ├── system.rs
│   ├── hardware.rs
│   ├── terminal.rs
│   └── cli.rs
└── assets/
    └── logo.txt
```

## License

MIT. See [LICENSE](LICENSE).
