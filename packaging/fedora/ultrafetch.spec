Name:           ultrafetch
Version:        0.1.1
Release:        1%{?dist}
Summary:        Lightweight, fast, minimal system information tool for Linux

License:        MIT
URL:            https://github.com/q7xk/ultrafetch
Source0:        %{url}/archive/v%{version}/%{name}-%{version}.tar.gz

BuildRequires:  rust-packaging >= 21
BuildRequires:  cargo
BuildRequires:  rust

%description
Ultrafetch is a lightweight, fast, minimal system information tool for Linux,
written in Rust. It displays essential system details alongside a distro-aware
ASCII logo and supports per-field commands and JSON output.

%prep
%autosetup -n %{name}-%{version}

%build
cargo build --release --locked

%check
cargo test --release --locked

%install
install -Dpm0755 target/release/%{name} %{buildroot}%{_bindir}/%{name}
install -Dpm0644 docs/ultrafetch.1 %{buildroot}%{_mandir}/man1/ultrafetch.1
install -Dpm0644 LICENSE %{buildroot}%{_licensedir}/%{name}/LICENSE
install -Dpm0644 README.md %{buildroot}%{_docdir}/%{name}/README.md

%files
%license LICENSE
%doc README.md CHANGELOG.md docs/CLI.md docs/ARCHITECTURE.md docs/PACKAGING.md
%{_bindir}/%{name}
%{_mandir}/man1/ultrafetch.1*

%changelog
* Sat Nov 15 2026 q7xk <q7xk@users.noreply.github.com> - 0.1.1-1
- Initial Fedora package
