{
  description = "cheri-microanalysis dev shell";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    doctor-cluster.url = "github:TUM-DSE/doctor-cluster-config";
    doctor-cluster.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    { nixpkgs, doctor-cluster, ... }:
    let
      system = "aarch64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      doctorPackages = doctor-cluster.packages.${system};
      python = pkgs.python3;
      pythonPackages = python.pkgs;
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        nativeBuildInputs = [
          doctorPackages.clang-morello
          doctorPackages.llvm-morello-purecap
          doctorPackages.musl-morello-purecap
        ];
        buildInputs = [
          pkgs.valgrind
          pkgs.just
          pkgs.bc
          pkgs.hwloc
          python
          pythonPackages.pandas
          pythonPackages.matplotlib
          pythonPackages.numpy
          pythonPackages.jinja2
        ];

        CLANG_HYBRID_PATH = "${doctorPackages.clang-morello}/bin/clang";
        CLANG_PURECAP_PATH = "${doctorPackages.llvm-morello-purecap}/bin/clang";
        MUSL_PATH = "${doctorPackages.musl-morello-purecap}";
        LLVM_PATH = "${doctorPackages.llvm-morello-purecap}";

        shellHook = ''
          alias clang-hybrid="$CLANG_HYBRID_PATH"
          alias clang-purecap="$CLANG_PURECAP_PATH"
        '';
      };
    };
}
