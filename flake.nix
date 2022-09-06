{
  description = "Build a cargo project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs = inputs@{ self, nixpkgs, crane, flake-utils, advisory-db, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

        inherit (pkgs) lib;

        craneLib = crane.lib.${system};
        src = ./.;

        # Build dependencies
        cargoArtifacts = craneLib.buildDepsOnly {
          inherit src;
        };

        # Build the crate
        unii = craneLib.buildPackage {
          inherit cargoArtifacts src;
        };
      in
      {
        checks = {
          inherit unii;

          # Run clippy
          unii-clippy = craneLib.cargoClippy {
            inherit cargoArtifacts src;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          };

          # Check formatting
          unii-fmt = craneLib.cargoFmt {
            inherit src;
          };


          # Audit dependencies
          unii-audit = craneLib.cargoAudit {
            inherit src advisory-db;
          };

          # Run tests with cargo-nextest
          unii-nextest = craneLib.cargoNextest {
            inherit cargoArtifacts src;
            partitions = 1;
            partitionType = "count";
          };
        } // lib.optionalAttrs (system == "x86_64-linux") {
          # Check code coverage
          unii-coverage = craneLib.cargoTarpaulin {
            inherit cargoArtifacts src;
          };
        };

        packages.default = unii;

        apps.default = flake-utils.lib.mkApp {
          drv = unii;
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = builtins.attrValues self.checks;

          nativeBuildInputs = with pkgs; [
            cargo
            rustc
          ];
        };
      });
}
