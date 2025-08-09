{
  description = "ESP‑IDF dev shell (nix‑ld + espup)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShells.default = pkgs.mkShell {
          name = "esp‑idf‑shell";
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
            pkgs.stdenv.cc.cc.lib
            pkgs.libxml2
            pkgs.libz
          ];
          NIX_LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
            pkgs.libclang
          ];

          nativeBuildInputs = with pkgs; [
            rustup
            espup
            ldproxy
            rust-analyzer
            espflash
          ];

          shellHook = ''
            espup install -l debug -t esp32 -e -s -d x86_64-unknown-linux-gnu
            espup update

            export PATH="$HOME/.rustup/toolchains/esp/bin:$PATH"
            . ~/export-esp.sh

            exec fish
          '';

        };
      }
    );
}
