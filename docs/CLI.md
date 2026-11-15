# CLI Reference

## Usage

```bash
rustfetch [COMMAND] [OPTIONS]
```

## Commands

- `(default)` / `info` — Show full system information.
- `cpu` — Show CPU model.
- `gpu` — Show GPU information.
- `memory` — Show memory usage.
- `disk` — Show disk usage.
- `os` — Show operating system name.
- `uptime` — Show uptime.
- `logo` — Show only the current distro logo.
- `logo <name>` — Show full system information with a specific distro logo.
- `logo list` — List all available logo names.

## Options

- `-h`, `--help` — Show help.
- `-V`, `--version` — Show version.
- `-d`, `--distro <name>` — Use a specific distro logo for full info.
- `--config` — Show configuration path.
- `--no-color` — Disable ANSI colors.
- `--json` — Output full info as JSON (includes `distro_id`).
- `--logo` — Alias for `logo` command.

## Examples

```bash
rustfetch
rustfetch info --json
rustfetch -d nixos
rustfetch cpu
rustfetch logo fedora
rustfetch logo list
```
