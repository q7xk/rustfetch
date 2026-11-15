Name: ultrafetch
Version: 0.1.1
Release: alt1

Summary: Lightweight, fast, minimal system information tool for Linux
License: MIT
Group: Monitoring
Url: https://github.com/q7xk/ultrafetch
Vcs: https://github.com/q7xk/ultrafetch.git

Source0: %name-%version.tar

BuildRequires(pre): rpm-build-rust
BuildRequires: rust-cargo
BuildRequires: /proc

%description
Ultrafetch is a lightweight, fast, minimal system information tool for Linux,
written in Rust. It displays essential system details alongside a distro-aware
ASCII logo and supports per-field commands and JSON output.

%prep
%setup

%build
%rust_build

%install
%rust_install
install -Dpm0644 docs/ultrafetch.1 %buildroot%_man1dir/ultrafetch.1
install -Dpm0644 README.md %buildroot%_docdir/%name/README.md

%check
%rust_test

%files
%doc README.md CHANGELOG.md docs/CLI.md docs/ARCHITECTURE.md docs/PACKAGING.md
%_bindir/%name
%_man1dir/ultrafetch.1*

%changelog
* Sat Nov 15 2026 q7xk <q7xk@users.noreply.github.com> 0.1.1-alt1
- Initial build
