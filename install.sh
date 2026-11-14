#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$ROOT"

if ! command -v cargo >/dev/null 2>&1; then
  echo "Error: cargo is not installed. Install Rust (https://rustup.rs/) and try again." >&2
  exit 1
fi

echo "Building rustfetch (release)..."
cargo build --release

BIN_SRC="$ROOT/target/release/rustfetch"
if [[ ! -x "$BIN_SRC" ]]; then
  echo "Error: build output not found at $BIN_SRC" >&2
  exit 1
fi

install_to_usr_local() {
  if command -v sudo >/dev/null 2>&1; then
    echo "Installing to /usr/local/bin (may prompt for sudo password)..."
    sudo install -Dm755 "$BIN_SRC" /usr/local/bin/rustfetch
  else
    install -Dm755 "$BIN_SRC" /usr/local/bin/rustfetch
  fi
  echo "Installed: /usr/local/bin/rustfetch"
}

install_to_local_bin() {
  local dest="$HOME/.local/bin"
  mkdir -p "$dest"
  install -Dm755 "$BIN_SRC" "$dest/rustfetch"
  echo "Installed: $dest/rustfetch"
  case ":$PATH:" in
    *":$dest:"*) ;;
    *) echo "Note: add '$dest' to your PATH to run 'rustfetch' directly." ;;
  esac
}

if [[ -w /usr/local/bin ]] || command -v sudo >/dev/null 2>&1; then
  if install_to_usr_local; then
    echo "Done. You can now run: rustfetch"
    exit 0
  fi
  echo "System install failed, falling back to user install..." >&2
fi

install_to_local_bin
echo "Done. You can now run: rustfetch (if ~/.local/bin is in PATH)"
