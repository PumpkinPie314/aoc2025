{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  buildInputs = with pkgs; [
    # ... other dependencies
    rustc
    cargo
    rust-analyzer
  ];
  # Manually set the source path
  RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
}