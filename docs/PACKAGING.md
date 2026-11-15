# Packaging Rustfetch

This document provides packaging templates and submission guidance for common Linux distribution repositories. Templates are kept in `packaging/` and should be adapted to each ecosystem's latest policies before submission.

## Common Upstream Requirements

- **Versioned release:** create a git tag (e.g. `v0.1.1`) and publish a source tarball/checksums.
- **License:** MIT (`LICENSE`).
- **Build:** standard Rust build (`cargo build --release --locked`) with no runtime network access.
- **Install artifacts:** `rustfetch` binary, `docs/rustfetch.1` man page, `LICENSE`, and `README.md` (paths vary by distro).
- **Metadata:** description, homepage/repository, and license must match upstream.

## Release Tarball

After tagging a release:

```bash
./scripts/create-release-tarball.sh
sha256sum rustfetch-<version>.tar.gz
```

Use the checksum in downstream manifests/recipes where required.

## Distro Targets

| Distribution | Repository / Workflow | Template |
| --- | --- | --- |
| Arch Linux | AUR (`PKGBUILD` + `.SRCINFO`) | `packaging/aur/` |
| Fedora | Bugzilla Package Review (`.spec`) | `packaging/fedora/` |
| Gentoo | GURU ebuild | `packaging/gentoo/app-misc/rustfetch/` |
| Alpine | aports `APKBUILD` | `packaging/alpine/` |
| NixOS | Nixpkgs derivation | `packaging/nix/` |
| openSUSE | OBS `_service` + `.spec` | `packaging/opensuse/` |
| ALT Linux | ALT Git `.spec`/gear | `packaging/altlinux/` |
| Exherbo | exheres-0 in package repo | `packaging/exherbo/packages/app-misc/rustfetch/` |
| Solus | `package.yml` + `pspec_*.xml` | `packaging/solus/` |
| Slackware | SlackBuilds.org submission | `packaging/slackware/` |
| Void Linux | void-packages `template` | `packaging/void/srcpkgs/rustfetch/` |
| Venom Linux | ports/recipe | `packaging/venom/` |

## Notes by Ecosystem

- **Rust ecosystems (AUR/Alpine/Void/openSUSE/etc.):** prefer building from the release source tarball with `--locked`. Some repos may require vendoring dependencies for offline builds even if upstream has none.
- **Nixpkgs:** update `cargoHash`/`hash` as required by nixpkgs tooling and follow nixpkgs Rust conventions.
- **Gentoo/GURU:** set accurate `KEYWORDS`/arches after testing; include `Cargo.lock` in manifests as required.
- **Fedora/ALT:** follow each distro's Rust packaging macros/policies and review process.
- **Exherbo/Solus/Slackware/Venom:** adapt install paths and metadata fields to repository-specific helpers and policies.

Always verify with the target repository's current contribution guidelines before submitting.
