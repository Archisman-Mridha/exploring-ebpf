{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";

		rust-overlay = {
			url = "github:oxalica/rust-overlay";
			inputs = {
				nixpkgs.follows = "nixpkgs";
				flake-utils.follows = "flake-utils";
			};
		};

		pre-commit-hooks.url = "github:cachix/git-hooks.nix";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
			rust-overlay,
			pre-commit-hooks,
			...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
				overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
					inherit system overlays;
				};
      in
      with pkgs;
      {
        devShells.default = mkShell {
					nativeBuildInputs = [
						rust-bin.nightly.latest.default
					];

          buildInputs = [
            llvm
            rustup
						rust-bin.nightly.latest.default

            bpf-linker
          ];
        };

				checks = {
          pre-commit-check = pre-commit-hooks.lib.${system}.run {
            src = ./.;
            hooks = {
							rustfmt.enable = true;
							clippy.enable = true;

              nixfmt-rfc-style.enable = true;
            };
          };
        };
      }
    );
}

