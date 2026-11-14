# Architecture

## Overview

Rustfetch is a small Rust CLI that collects Linux system information and renders it in the terminal.

## Modules

- `src/main.rs`: entry point, command dispatch, JSON output.
- `src/cli.rs`: argument parsing, help/version/config text.
- `src/system.rs`: OS, kernel, host/user, uptime, shell, desktop/session, terminal, architecture.
- `src/hardware.rs`: CPU, GPU, memory, disk collection.
- `src/terminal.rs`: distro logo selection and side-by-side terminal rendering.

## Data Sources

- `/etc/os-release`, `/usr/lib/os-release`
- `/proc` and `/sys` virtual filesystems
- Environment variables (`USER`, `SHELL`, `XDG_*`, `TERM*`, etc.)
- `df` command for root filesystem disk usage (best-effort)

## Design Principles

- Minimal dependencies (standard library only).
- Fast startup.
- Graceful degradation when fields are unavailable.
- No network access, telemetry, or background services.
