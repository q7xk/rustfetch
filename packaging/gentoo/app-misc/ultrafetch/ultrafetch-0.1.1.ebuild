# Copyright 2026 Gentoo Authors
# Distributed under the terms of the GNU General Public License v2

EAPI=8

CRATES=""

inherit cargo

DESCRIPTION="A lightweight, fast, minimal system information tool for Linux"
HOMEPAGE="https://github.com/q7xk/ultrafetch"
SRC_URI="https://github.com/q7xk/ultrafetch/archive/v${PV}.tar.gz -> ${P}.tar.gz"
S="${WORKDIR}/${P}"

LICENSE="MIT"
SLOT="0"
KEYWORDS="~amd64 ~arm64"

BDEPEND=">=virtual/rust-1.74"

QA_FLAGS_IGNORED="usr/bin/ultrafetch"

src_configure() {
	cargo_src_configure
}

src_compile() {
	cargo_src_compile
}

src_test() {
	cargo_src_test
}

src_install() {
	cargo_src_install
	doman docs/ultrafetch.1
	dodoc README.md CHANGELOG.md docs/CLI.md docs/ARCHITECTURE.md docs/PACKAGING.md
}
