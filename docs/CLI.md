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
- `os` — Show operating system.
- `uptime` — Show uptime.
- `logo` — Show only the distro logo.
- `logo list` — List supported logo families.

## Options

- `-h`, `--help` — Show help.
- `-V`, `--version` — Show version.
- `--config` — Show configuration path.
- `--no-color` — Disable ANSI colors in full output.
- `--json` — Output full info as JSON.
- `--logo` — Alias for `logo` command.

## Examples

```bash
rustfetch
rustfetch info --json
rustfetch cpu
rustfetch logo list
```
