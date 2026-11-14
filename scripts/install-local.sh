#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
cargo install --path .
echo "Installed rustfetch to cargo bin (ensure ~/.cargo/bin is in PATH)."
