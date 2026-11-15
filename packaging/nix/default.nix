{ lib
, rustPlatform
, fetchFromGitHub
}:

rustPlatform.buildRustPackage rec {
  pname = "ultrafetch";
  version = "0.1.1";

  src = fetchFromGitHub {
    owner = "q7xk";
    repo = "ultrafetch";
    rev = "v${version}";
    hash = "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";
  };

  cargoHash = "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";

  postInstall = ''
    install -Dm644 docs/ultrafetch.1 $out/share/man/man1/ultrafetch.1
    install -Dm644 LICENSE $out/share/licenses/${pname}/LICENSE
    install -Dm644 README.md $out/share/doc/${pname}/README.md
  '';

  meta = with lib; {
    description = "A lightweight, fast, minimal system information tool for Linux";
    homepage = "https://github.com/q7xk/ultrafetch";
    license = licenses.mit;
    maintainers = [ ];
    platforms = platforms.linux;
    mainProgram = "ultrafetch";
  };
}
