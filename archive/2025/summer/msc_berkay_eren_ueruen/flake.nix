{
  description = "Development environment for this project";

  inputs = {
    nixpkgs.url = "git+https://github.com/TUM-DSE/nixpkgs.git?ref=nixos-24.11-backports&shallow=1";
    #nixpkgs.url = "github:TUM-DSE/nixpkgs/63dacb46bf939521bdc93981b4cbb7ecb58427a0";
    jetpack-nixos.url = "git+https://github.com/TUM-DSE/jetpack-nixos.git?shallow=1";
    #jetpack-nixos.url = "git+https://github.com/TUM-DSE/jetpack-nixos.git?shallow=1&ref=final-stretch";
    jetpack-nixos.inputs.nixpkgs.follows = "nixpkgs";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs =
    inputs@{ flake-parts, nixpkgs, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } (
      { lib, ... }:
      {
        systems = lib.systems.flakeExposed;

        # Usage:
		# $ nixos-shell --flake .#nvidia-vms
		flake.nixosConfigurations.nvidia-vm = nixpkgs.lib.nixosSystem {
		  modules = [{
		   imports = [
		     ./nvidia-gpu-patrick-latests.nix
		   ];
		  }];
		  specialArgs = {
		    inputs =  inputs;
		  };
		};
        perSystem =
          {
            inputs',
            pkgs,
            system,
            ...
          }:
          {
            _module.args.pkgs = import inputs.nixpkgs {
              inherit system;
              config = {
                allowUnfree = true;
                cudaSupport = true;
                # Only for jetson devices: https://en.wikipedia.org/wiki/CUDA#GPUs_supported
                # Faster compilation time?
                cudaCapabilities = [ "8.7" ];

              };
#              overlays = [ (final: prev: { cudaPackages = inputs'.jetpack-nixos.legacyPackages.cudaPackages; }) ];
            overlays = [  ];
            };
            packages.default = pkgs.mkShell {
			   buildInputs = [
			   pkgs.expat.dev
          ];
              packages = [
                pkgs.bashInteractive
#                pkgs.python3Packages.torch
                #pkgs.cudaPackages.cuda_nvcc
				#pkgs.cudaPackages.cuda_gdb
				pkgs.cudaPackages.cudatoolkit
				pkgs.cudaPackages.cuda_cudart
				pkgs.cudaPackages.cuda_cudart.stubs
				pkgs.expat
				pkgs.nixos-shell
              ];
              shellHook = ''
                export CUDA_PATH=${pkgs.cudaPackages.cudatoolkit}
              '';
              LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
                "/run/opengl-driver"
              ];
            };
          };
      }
    );
}
