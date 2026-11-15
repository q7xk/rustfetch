{ lib
, rustPlatform
, fetchFromGitHub
}:

rustPlatform.buildRustPackage rec {
  pname = "rustfetch";
  version = "0.1.1";

  src = fetchFromGitHub {
    owner = "q7xk";
    repo = "rustfetch";
    rev = "v${version}";
    hash = "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";
  };

  cargoHash = "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";

  postInstall = ''
    install -Dm644 docs/rustfetch.1 $out/share/man/man1/rustfetch.1
    install -Dm644 LICENSE $out/share/licenses/${pname}/LICENSE
    install -Dm644 README.md $out/share/doc/${pname}/README.md
  '';

  meta = with lib; {
    description = "A lightweight, fast, minimal system information tool for Linux";
    homepage = "https://github.com/q7xk/rustfetch";
    license = licenses.mit;
    maintainers = [ ];
    platforms = platforms.linux;
    mainProgram = "rustfetch";
  };
}
