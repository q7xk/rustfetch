Name:           rustfetch
Version:        0.1.1
Release:        0
Summary:        Lightweight, fast, minimal system information tool for Linux
License:        MIT
URL:            https://github.com/q7xk/rustfetch
Source0:        %{name}-%{version}.tar.gz
BuildRequires:  cargo
BuildRequires:  rust >= 1.74

%description
Rustfetch is a lightweight, fast, minimal system information tool for Linux,
written in Rust. It displays essential system details alongside a distro-aware
ASCII logo and supports per-field commands and JSON output.

%prep
%autosetup

%build
cargo build --release --locked

%check
cargo test --release --locked

%install
install -Dpm0755 target/release/%{name} %{buildroot}%{_bindir}/%{name}
install -Dpm0644 docs/rustfetch.1 %{buildroot}%{_mandir}/man1/rustfetch.1
install -Dpm0644 LICENSE %{buildroot}%{_licensedir}/%{name}/LICENSE
install -Dpm0644 README.md %{buildroot}%{_docdir}/%{name}/README.md

%files
%license LICENSE
%doc README.md CHANGELOG.md docs/CLI.md docs/ARCHITECTURE.md docs/PACKAGING.md
%{_bindir}/%{name}
%{_mandir}/man1/rustfetch.1%{?ext_man}

%changelog
