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
- Disk usage (root filesystem)
- System architecture
- Distro-aware ASCII logos with Fastfetch-based coloring (534 embedded logos)
- Colorized output and JSON export
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

Quick install script:

```bash
./install.sh
```

Or install with Cargo:

```bash
cargo install --path .
```

Ensure your Cargo bin directory (usually `~/.cargo/bin`) is in your `PATH`.

## Usage

```bash
rustfetch                     # Display system information
rustfetch info                # Display full system information
rustfetch cpu                 # Show CPU model
rustfetch gpu                 # Show GPU information
rustfetch memory              # Show memory usage
rustfetch disk                # Show disk usage
rustfetch os                  # Show operating system name
rustfetch uptime              # Show uptime
rustfetch logo                # Display only the current distro logo
rustfetch logo <name>         # Show full info with a specific distro logo
rustfetch logo list           # List all available logo names
rustfetch -d <name>           # Display full info with a specific distro logo
rustfetch --json              # Output full info as JSON
rustfetch --no-color          # Disable terminal colors
rustfetch --help              # Show help
rustfetch --version           # Show version
rustfetch --config            # Show configuration location
```

## Logo Customization

Rustfetch embeds a large set of ASCII logos (sourced from Fastfetch) and auto-detects the distro from `/etc/os-release`. You can override the displayed logo with `-d, --distro <name>` or `rustfetch logo <name>`. Use `rustfetch logo list` to see all available logo names.

The default embedded fallback logo asset is `assets/logo.txt`.

## Configuration

`rustfetch --config` prints the canonical configuration path:

- `$XDG_CONFIG_HOME/rustfetch/config.toml`, or
- `~/.config/rustfetch/config.toml` if `XDG_CONFIG_HOME` is not set

The current MVP does not require or parse a configuration file.

## Notes and Limitations

- GPU detection is best-effort using Linux sysfs (`/sys/class/drm/...`) and may not provide a fully human-readable model name on all systems.
- Disk usage is collected for the root filesystem (`/`).
- Desktop environment, window manager/session, terminal, and shell information depend on the current process/session environment.
- Logo coloring follows Fastfetch palette/placeholder data; logos without per-region color placeholders in the source art may have limited or palette-derived coloring.
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
│   ├── logos.rs
│   ├── logo_colors.rs
│   └── cli.rs
└── assets/
    ├── logo.txt
    └── logos/
```

## License

MIT. See [LICENSE](LICENSE).
