# Packaging Templates

This directory contains starter packaging files for submitting `rustfetch` to downstream Linux repositories. Review and adapt each file to the target ecosystem's current policies, naming conventions, and tooling before submission.

- `aur/` — Arch Linux (AUR) `PKGBUILD`
- `fedora/` — Fedora RPM spec for package review
- `gentoo/app-misc/rustfetch/` — Gentoo GURU ebuild
- `alpine/` — Alpine aports `APKBUILD`
- `nix/` — Nixpkgs-style derivation
- `opensuse/` — openSUSE OBS spec and `_service`
- `altlinux/` — ALT Linux spec/gear template
- `exherbo/packages/app-misc/rustfetch/` — Exherbo exheres-0
- `solus/` — Solus `package.yml` template
- `slackware/` — SlackBuilds.org files
- `void/srcpkgs/rustfetch/` — Void Linux template
- `venom/` — Venom Linux port recipe

See [`../docs/PACKAGING.md`](../docs/PACKAGING.md) for release tarball and submission notes.
