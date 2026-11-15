# Architecture

## Overview

Rustfetch is a small, dependency-free Rust CLI that collects Linux system information and renders it in the terminal as a logo + info layout, per-field output, or JSON.

## Modules

- `src/main.rs` — program entry point, command dispatch, JSON rendering.
- `src/cli.rs` — argument parsing, command/option validation, help/version/config text.
- `src/system.rs` — OS/distro identity, kernel, host/user, uptime, shell, desktop/session, terminal, architecture.
- `src/hardware.rs` — CPU, GPU, memory, and root disk usage collection.
- `src/terminal.rs` — logo selection/aliasing and side-by-side terminal rendering.
- `src/logos.rs` — embedded ASCII logo data access.
- `src/logo_colors.rs` — Fastfetch-derived color palette data for logos.

## Data Flow

1. CLI arguments are parsed in `cli`.
2. `main` dispatches to command handlers.
3. `system` and `hardware` collectors gather best-effort data from local sources.
4. `terminal` resolves a logo and composes formatted output (or `main` emits JSON).

## Data Sources

- `/etc/os-release`, `/usr/lib/os-release`
- `/proc` and `/sys` virtual filesystems
- Environment variables (`USER`, `LOGNAME`, `SHELL`, `XDG_*`, `TERM*`, etc.)
- `df -B1 -P /` for root filesystem usage (best-effort)

## Design Principles

- Minimal dependencies (standard library only)
- Fast startup and low overhead
- Graceful degradation when fields are unavailable
- No network access, telemetry, or background services
- Keep behavior predictable and CLI-focused