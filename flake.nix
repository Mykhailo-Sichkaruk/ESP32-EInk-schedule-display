{
  description = "ESP‑IDF dev shell (nix‑ld + espup)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          config.allowUnfree = true;
          overlays = [ rust-overlay.overlays.default ];
        };

      in
      {
        devShells.default = pkgs.mkShell {
          name = "esp‑idf‑shell";
          NIX_LD = pkgs.lib.fileContents "${pkgs.stdenv.cc}/nix-support/dynamic-linker";
          NIX_LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
            pkgs.glibc
            pkgs.gcc.libc
            pkgs.zlib
          ];
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
            pkgs.stdenv.cc.cc.lib
            pkgs.libxml2
          ];

          nativeBuildInputs = with pkgs; [
            rustup
            espup
            cargo
            cargo-binutils 
            python3
            cmake
            ninja 
            ldproxy
          ];

          shellHook = ''
            espup install -t esp32 -e -d x86_64-unknown-linux-gnu

            . ~/export-esp.sh
          '';

        };
      }
    );
}
