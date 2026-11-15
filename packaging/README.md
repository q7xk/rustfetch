# Packaging Templates

This directory contains starter packaging files for submitting `ultrafetch` to downstream Linux repositories. Review and adapt each file to the target ecosystem's current policies, naming conventions, and tooling before submission.

- `aur/` — Arch Linux (AUR) `PKGBUILD`
- `fedora/` — Fedora RPM spec for package review
- `gentoo/app-misc/ultrafetch/` — Gentoo GURU ebuild
- `alpine/` — Alpine aports `APKBUILD`
- `nix/` — Nixpkgs-style derivation
- `opensuse/` — openSUSE OBS spec and `_service`
- `altlinux/` — ALT Linux spec/gear template
- `exherbo/packages/app-misc/ultrafetch/` — Exherbo exheres-0
- `solus/` — Solus `package.yml` template
- `slackware/` — SlackBuilds.org files
- `void/srcpkgs/ultrafetch/` — Void Linux template
- `venom/` — Venom Linux port recipe

See [`../docs/PACKAGING.md`](../docs/PACKAGING.md) for release tarball and submission notes.
