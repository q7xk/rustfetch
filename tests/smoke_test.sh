#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

cargo run --quiet -- --version >/dev/null
cargo run --quiet -- --help >/dev/null
cargo run --quiet -- info >/dev/null
cargo run --quiet -- cpu >/dev/null
cargo run --quiet -- os >/dev/null
cargo run --quiet -- logo >/dev/null

echo "smoke tests passed"
