<div align="center">

# ⚡ Ultrafetch

**Lightning-fast, minimal system information for your Linux terminal**

[![CI](https://github.com/q7xk/ultrafetch/actions/workflows/ci.yml/badge.svg)](https://github.com/q7xk/ultrafetch/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/q7xk/ultrafetch?display_name=tag&sort=semver)](https://github.com/q7xk/ultrafetch/releases)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/platform-linux-lightgrey.svg)](#requirements)

[Features](#-features) • [Installation](#-installation) • [Usage](#-usage) • [Packaging](#-distribution-packaging) • [Docs](#-documentation)

</div>

---

## ✨ Overview

**Ultrafetch** is a lightweight, dependency-free system information tool written in Rust. It shows the essentials—OS, kernel, hardware, desktop environment, and more—next to a beautiful distro-aware ASCII logo, right in your terminal.

```text
                  -`                   q7xk@archlinux
                 .o+`                  --------------
                `ooo/                  OS        Arch Linux x86_64
               `+oooo:                 Kernel    6.x.x-arch1-1
              `+oooooo:                Uptime    3 hours, 14 mins
              -+oooooo+:               Shell     fish
            `/:-:++oooo+:              DE        KDE
           `/++++/+++++++:             WM        Wayland
          `/++++++++++++++:            Terminal  ghostty
         `/+++ooooooooooooo/`          CPU       Intel Core i5-8300H
        ./ooosssso++osssssso+`         GPU       NVIDIA GTX 1050
       .oossssso-````/ossssss+`        Memory    3.2 GiB / 7.6 GiB
      -osssssso.      :ssssssso.       Disk      53.5 GiB / 237.5 GiB (23%)
     :osssssss/        osssso+++.      Arch      x86_64
```

## 🚀 Features

| | |
|---|---|
| ⚡ **Blazing fast** | Minimal startup, standard-library-only runtime |
| 🎨 **534 embedded logos** | Distro-aware ASCII logos with Fastfetch-based coloring |
| 🧩 **Flexible output** | Full fetch, per-field commands, logo-only, and JSON |
| 🛡️ **Graceful fallbacks** | Missing fields are handled safely—no crashes |
| 🐧 **Linux-native** | Reads from `/proc`, `/sys`, and environment variables |
| 🔒 **Private by design** | No network access, telemetry, or background processes |

## 📋 What it shows

- OS / distribution and architecture
- Kernel version
- Hostname and username
- Uptime
- Shell, desktop environment, window manager/session, terminal
- CPU model
- GPU (best effort)
- Memory usage
- Disk usage (root filesystem)

## 📦 Installation

### Build from source

```bash
git clone https://github.com/q7xk/ultrafetch.git
cd ultrafetch
cargo build --release --locked
./target/release/ultrafetch
```

> Requires a stable Rust toolchain (`rustc`/`cargo`, Rust >= 1.74).

### Quick install script

```bash
./install.sh
```

Builds a release binary and installs it to `/usr/local/bin` when possible (or falls back to `~/.local/bin`).

### Makefile

```bash
make
sudo make install PREFIX=/usr/local
```

Installs the binary, man page, license, and documentation.

### Cargo

```bash
cargo install --path .
```

## 🛠️ Usage

```bash
ultrafetch
```

| Command / Option | Description |
|---|---|
| `ultrafetch`, `ultrafetch info` | Show full system information |
| `ultrafetch cpu` | Show CPU model |
| `ultrafetch gpu` | Show GPU information |
| `ultrafetch memory` | Show memory usage |
| `ultrafetch disk` | Show disk usage |
| `ultrafetch os` | Show operating system name |
| `ultrafetch uptime` | Show uptime |
| `ultrafetch logo` | Show only the detected distro logo |
| `ultrafetch logo <name>` | Show full info with a specific logo |
| `ultrafetch logo list` | List all available logo names |
| `-d, --distro <name>` | Use a specific distro logo for full info |
| `--json` | Output full info as JSON |
| `--no-color` | Disable ANSI colors |
| `-h, --help` | Show help |
| `-V, --version` | Show version |
| `--config` | Show configuration path |

### JSON output

```bash
ultrafetch --json
```

Includes: `os`, `distro_id`, `kernel`, `hostname`, `username`, `uptime`, `shell`, `de`, `wm`, `terminal`, `cpu`, `gpu`, `memory`, `disk`, `architecture`.

## 🎨 Logos

- Ultrafetch embeds **534** ASCII logos and auto-detects your distro.
- Override anytime: `ultrafetch -d nixos` or `ultrafetch logo fedora`.
- Discover names with `ultrafetch logo list`.
- Coloring uses Fastfetch-derived palettes and placeholders where available.

## 📚 Documentation

- [CLI Reference](docs/CLI.md)
- [Man Page](docs/ultrafetch.1)
- [Architecture](docs/ARCHITECTURE.md)
- [Packaging Guide](docs/PACKAGING.md)
- [Changelog](CHANGELOG.md)
- [Contributing](CONTRIBUTING.md)
- [Security Policy](SECURITY.md)

## 📦 Distribution Packaging

Starter templates and submission notes are included for downstream repositories:

`AUR` · `Fedora` · `Gentoo (GURU)` · `Alpine (aports)` · `Nixpkgs` · `openSUSE (OBS)` · `ALT Linux` · `Exherbo` · `Solus` · `SlackBuilds` · `Void Linux` · `Venom Linux`

See [`docs/PACKAGING.md`](docs/PACKAGING.md) and [`packaging/`](packaging/).

## ⚠️ Notes & Limitations

- GPU names are best-effort (sysfs-based; may show vendor/device IDs or driver names).
- Disk usage is reported for `/`.
- Desktop/session/terminal/shell values depend on runtime environment variables.
- Logos without per-region placeholders in upstream art may use palette/base coloring.
- `--config` prints the canonical config path; the current version does not parse a config file.

## 🤝 Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) and follow the validation checks:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all
cargo build --release --locked
```

## 🙏 Acknowledgements

- Inspired by [Fastfetch](https://github.com/fastfetch-cli/fastfetch) and [Neofetch](https://github.com/dylanaraps/neofetch).
- ASCII logo art and color metadata are derived from Fastfetch sources.

## 📄 License

Licensed under the [MIT License](LICENSE).

---

<div align="center">
  <sub>Built with Rust • Made for Linux terminals</sub>
</div>
