{
	description = "Focaccia: A Symbolic Tester for QEMU";

	inputs = {
		self.submodules = true;

		nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

		flake-utils.url = "github:numtide/flake-utils";

		pyproject-nix = {
			url = "github:pyproject-nix/pyproject.nix";
			inputs.nixpkgs.follows = "nixpkgs";
		};

		uv2nix = {
			url = "github:pyproject-nix/uv2nix";
			inputs.nixpkgs.follows = "nixpkgs";
			inputs.pyproject-nix.follows = "pyproject-nix";
		};

		pyproject-build-systems = {
			url = "github:pyproject-nix/build-system-pkgs";
			inputs.uv2nix.follows = "uv2nix";
			inputs.nixpkgs.follows = "nixpkgs";
			inputs.pyproject-nix.follows = "pyproject-nix";
		};
	};

	outputs = {
		uv2nix,
		nixpkgs,
		flake-utils,
		pyproject-nix,
		pyproject-build-systems,
		...
	}:
	flake-utils.lib.eachSystem [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" ] (system:
	let
		# Refine nixpkgs used in flake to system arch
		pkgs = import nixpkgs {
			inherit system;
		};

		# Pin Python version
		python = pkgs.python312;

		# Define workspace root and load uv workspace metadata
		workspace = uv2nix.lib.workspace.loadWorkspace { workspaceRoot = ./.; };

		# Create an overlay for Nix that includes extracted Python packages declared as dependencies
		# in uv
		overlay = workspace.mkPyprojectOverlay { sourcePreference = "wheel"; };

		editableOverlay = workspace.mkEditablePyprojectOverlay {
			# Use environment variable
			root = "$REPO_ROOT";

			members = [ "miasm" ];
		};

		# Another overlay layer for flake-specific overloads
		# This might be needed because uv does not have sufficient metadata
		# Here, uv does include metadata about build systems used by each dependency
		# Ergo we need to add a nativeBuildInput to miasm because it depends on setuptools for its
		# installation
		pyprojectOverrides = self: super: {
			miasm = super.miasm.overrideAttrs (old: {
				nativeBuildInputs = (old.nativeBuildInputs or []) ++ [ self.setuptools ];
			});

			z3-solver = super.z3-solver.overrideAttrs (old: {
				nativeBuildInputs = (old.nativeBuildInputs or []) ++ [ self.setuptools self.cmake ];
			});
		};

		pyprojectOverridesEditable = self: super: {
			miasm = super.miasm.overrideAttrs (old: {
				nativeBuildInputs = (old.nativeBuildInputs or []) ++ [ self.setuptools ];

				src = pkgs.lib.fileset.toSource {
					root = old.src;
					fileset = pkgs.lib.fileset.unions [
						(old.src + "/pyproject.toml")
						(old.src + "/README.md")
						(old.src + "/src/miasm/__init__.py")
					];
				};
			});

			z3-solver = super.z3-solver.overrideAttrs (old: {
				nativeBuildInputs = (old.nativeBuildInputs or []) ++ [ self.setuptools self.cmake ];
			});
		};

		# Build a set of Python packages
		# The call to callPackage here uses the base package set from pyproject.nix
		# We inherit the Python version to ensure that the packages have the same version
		#
		# The overrideScope here customizes the Python package set with an overlay defined by the
		# composition of three overlay functions
		pythonSet = (pkgs.callPackage pyproject-nix.build.packages { inherit python; }).
					 overrideScope (pkgs.lib.composeManyExtensions [
						 pyproject-build-systems.overlays.default
						 overlay
						 pyprojectOverrides 
					 ]);

		pythonSetEditable = pythonSet.overrideScope (
			pkgs.lib.composeManyExtensions [
				editableOverlay
				pyprojectOverridesEditable
			]
		);

		 # Create a Python venv with the default dependency group
		 pythonEnv = pythonSet.mkVirtualEnv "miasm-env" workspace.deps.default;

		 # Create a Python venv with the default dependency group
		 pythonDevEnv = pythonSetEditable.mkVirtualEnv "miasm-env" workspace.deps.all;

		 uvEnv = {
			UV_NO_SYNC = "1";
			UV_PYTHON = python.interpreter;
			UV_PYTHON_DOWNLOADS = "never";
		};

		uvShellHook = ''
			unset PYTHONPATH

			export REPO_ROOT=$(git rev-parse --show-toplevel)
		'';
	in rec {
		# Default package just builds Focaccia
		packages = rec {
			miasm = pythonEnv;

			dev = pythonDevEnv.overrideAttrs (old: {
				propagatedBuildInputs = (old.propagatedBuildInputs or []) ++ [ 
					pkgs.uv
				];
			});

			default = miasm;
		};

		devShells = {
			default = pkgs.mkShell {
				packages = [ packages.dev ];

				env = uvEnv;
				shellHook = uvShellHook;
			};
			
			basic = pkgs.mkShell {
				packages = [ pkgs.uv packages.miasm ];

				env = uvEnv;
				shellHook = uvShellHook;
			};
		};

		# TODO
		checks = {
			miasm-tests = pkgs.stdenv.mkDerivation {
				name = "miasm-tests";
				src = ./.;

				doCheck = true;
				dontBuild = true;

				nativeCheckInputs = [ packages.dev pythonDevEnv ];

				checkPhase = ''
					set -euo pipefail
					export REPO_ROOT="$PWD"
					${packages.dev}/bin/python -m 'pytest' -q tests
					touch $out
				'';

				env = uvEnv;
				shellHook = uvShellHook;
			};
		};
	});
}

