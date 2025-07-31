{pkgs}:
pkgs.mkShell {
  buildInputs = with pkgs; [
    alejandra
    statix
    nixd

    rustc
    rust-analyzer
    clippy
    cargo
  ];
}
