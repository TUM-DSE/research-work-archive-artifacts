#{ pkgs ? import /ssd_extension/teofil/nixpkgs {} }:
#{ pkgs ? import <nixpkgs> {cudaSupport = true;} }:
#{pkgs ? import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/8b27c1239e5c421a2bbc2c65d52e4a6fbf2ff296.tar.gz") {} }:   
{pkgs ? import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/63dacb46bf939521bdc93981b4cbb7ecb58427a0.tar.gz") {} }:   

(pkgs.buildFHSUserEnv {
 name = "cudazone";
 targetPkgs = pkgs: (with pkgs; [
		 gcc12
		 gdb
		 ccache
#		 cudaPackages.nsight_systems
#		 cudaPackages.cuda_nvcc
		 cudatoolkit
		 cudaPackages.cuda_cudart.stubs
		 git-lfs
		 glibc.dev
		 gnumake
		 cmake
		 man
		 go
		 unzip
		 curl.dev
		 phoronix-test-suite
		 python3
		 php
		 nixos-shell
		 fzf
 ]);
runScript = "bash ./bench_native_base.sh";
 }).env

