# CLI Reference

## Usage

```bash
rustfetch [COMMAND] [OPTIONS]
```

## Commands

| Command | Description |
| --- | --- |
| *(default)* / `info` | Show full system information |
| `cpu` | Show CPU model |
| `gpu` | Show GPU information |
| `memory` | Show memory usage |
| `disk` | Show root filesystem disk usage |
| `os` | Show operating system name |
| `uptime` | Show uptime |
| `logo` | Show only the detected distro logo |
| `logo <name>` | Show full system information with a specific logo |
| `logo list` | List all available logo names |

## Options

| Option | Description |
| --- | --- |
| `-h`, `--help` | Show help |
| `-V`, `--version` | Show version |
| `-d`, `--distro <name>` | Use a specific distro logo for full info |
| `--config` | Show configuration path |
| `--no-color` | Disable ANSI colors |
| `--json` | Output full info as JSON (`info` only) |
| `--logo` | Alias for `logo` command |

## Examples

```bash
rustfetch
rustfetch info --json
rustfetch --distro nixos
rustfetch cpu
rustfetch logo fedora
rustfetch logo list
```

## JSON Fields

`--json` output includes:

- `os`
- `distro_id`
- `kernel`
- `hostname`
- `username`
- `uptime`
- `shell`
- `de`
- `wm`
- `terminal`
- `cpu`
- `gpu` (`string` or `null`)
- `memory`
- `disk`
- `architecture`

## Exit Codes

- `0` — success
- `2` — invalid command or option