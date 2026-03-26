{
  description = "C++ development package";

  inputs = {
    nixpkgs.url     = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            pkgs.llvmPackages_21.clang
            pkgs.cmake
            pkgs.gdb
            pkgs.llvmPackages_21.libllvm
            pkgs.pkgsCross.aarch64-multiplatform.buildPackages.gcc
            pkgs.qemu-user
            (pkgs.python3.withPackages (ps: with ps; [
              matplotlib
              numpy
            ]))
          ];

          shellHook = ''
            export PATH=$PATH:$PWD/build/bin
            export LD_LIBRARY_PATH="$HOME/airlift/target/debug/:$HOME/spectranslator/build/runtime/:$LD_LIBRARY_PATH"
          '';
        };
      }
    );
}
