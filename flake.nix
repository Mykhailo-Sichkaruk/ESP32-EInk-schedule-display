{
  description = "Shell for ESP32 EInk development";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
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
          overlays = [ (import rust-overlay) ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          name = "esp32-shell-basic";
          buildInputs = [
            pkgs.cargo-espflash
          ];

          packages = with pkgs; [
            espup
            esp-generate
            rustup
            cargo
            nix-ld
            cargo-binutils
            vscode
          ];

          shellHook = ''
            export NIX_LD=${pkgs.glibc}/lib/ld-linux-x86-64.so.2
            if [ ! -d "$HOME/.rustup/toolchains/esp" ]; then
              echo "🔧 installing esp toolchain"
              rustup toolchain install stable --component rust-src --target riscv32imc-unknown-none-elf
              rustup toolchain install nightly
              rustup component add rust-src --toolchain nightly
              espup install -e -d x86_64-unknown-linux-gnu 
              espup update
            else
              echo "🔧 esp toolchain already installed"
            fi
            echo "🔧 build with   : cargo +esp build --target esp32 --release"
            echo "🚀 flash device : cargo espflash /dev/ttyUSB0 --release"
            source $HOME/export-esp.sh
            exec fish
          '';
        };
      }
    );
}
