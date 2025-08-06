{pkgs}:
pkgs.mkShell {
  buildInputs = with pkgs; [
    alejandra
    deadnix
    statix
    nixd

    rustc
    rust-analyzer
    rustfmt
    clippy
    cargo
  ];
}
