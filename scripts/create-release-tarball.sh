#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

VERSION="$(grep '^version' Cargo.toml | head -n1 | cut -d'"' -f2)"
TAG="v${VERSION}"
OUT="${1:-ultrafetch-${VERSION}.tar.gz}"

if ! git rev-parse --verify "$TAG" >/dev/null 2>&1; then
  echo "Error: git tag '$TAG' not found. Create and push the tag before packaging a release tarball." >&2
  exit 1
fi

git archive --format=tar.gz --prefix="ultrafetch-${VERSION}/" -o "$OUT" "$TAG"
echo "Created $OUT from $TAG"
