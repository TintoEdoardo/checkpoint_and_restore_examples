{
  description = "Checkpoint and Restore Examples (with Wasm)";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-25.11";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    devshell.url = "github:numtide/devshell";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # rust target name of the `system`
        rust-target = pkgs.pkgsStatic.stdenv.targetPlatform.rust.rustcTarget;


        # Rust distribution for our hostSystem
        rust-toolchain-nixpkgs-current = pkgs.rust-bin.stable.${pkgs.rustc.version}.default.override {
          extensions = [ "rust-src" ];
          targets = [
            rust-target
            "wasm32-unknown-unknown"
          ];
        };
      in
      {
        devShells.default = with pkgs; mkShell {

          packages = with pkgs; [
            stdenv.cc
            coreutils
            rust-toolchain-nixpkgs-current # also contains clippy
            rust-analyzer
            cargo-watch
          ];

          shellHook = ''
            alias ls=eza
            alias find=fd
          '';

        };
      }
    );
}
