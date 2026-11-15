# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1] - 2026-11-15

### Added
- `-d, --distro <name>` option to render full output with a specific logo.
- `distro_id` field in JSON output.
- Expanded embedded ASCII logo set (534 logos) with Fastfetch-derived assets and palettes.
- Unit tests for CLI parsing, OS parsing/uptime formatting, and hardware formatting helpers.
- Man page (`docs/ultrafetch.1`) and `Makefile` install targets for packagers.
- Distribution packaging templates and packaging documentation for common Linux repositories.
- Release workflow for tagged source/binary artifacts.

### Changed
- `os` subcommand now reports the distro pretty name (without duplicating architecture).
- Logo rendering now preserves Fastfetch color placeholders and applies palette/base colors consistently.
- Improved distro/logo name normalization and alias handling (e.g. `tux`).
- Refreshed documentation, metadata, and repository hygiene files.

### Fixed
- Restored missing Fastfetch color placeholders in logo assets to correct regional coloring.
- Resolved clippy warnings and minor formatting/consistency issues.

## [0.1.0] - 2026-11-14

### Added
- Initial Ultrafetch MVP.
- System information collection (OS, kernel, hostname, user, uptime, shell, DE/WM, terminal, architecture).
- Hardware information collection (CPU, GPU, memory, disk).
- Distro-aware ASCII logo rendering.
- CLI commands and options: `info`, `cpu`, `gpu`, `memory`, `disk`, `os`, `uptime`, `logo`, `logo list`, `--json`, `--no-color`, `--help`, `--version`, `--config`.
- Project documentation and MIT license.

[Unreleased]: https://github.com/q7xk/ultrafetch/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/q7xk/ultrafetch/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/q7xk/ultrafetch/releases/tag/v0.1.0
