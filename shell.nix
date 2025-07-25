{ pkgs ? import <nixpkgs> {}
}: pkgs.mkShell {
  nativeBuildInputs = with pkgs.buildPackages; [
    cargo
    rustc
    rust-analyzer
    rustfmt
    openssl
    pkgconf
    vscode-langservers-extracted
  ];
}
