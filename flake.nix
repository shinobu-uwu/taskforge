{
  description = "Taskforge task manager";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs = {
    self,
    nixpkgs,
    crane,
    flake-utils,
    advisory-db,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};
      craneLib = crane.mkLib pkgs;
      lib = pkgs.lib;

      src = craneLib.cleanCargoSource ./.;

      runtimeDeps = with pkgs; [
        pkg-config
        wayland
        libxkbcommon
        vulkan-loader
        libGL
        freetype
        fontconfig
        alsa-lib
        udev
        libX11
        libxcb
      ];

      commonArgs = {
        inherit src;
        strictDeps = true;

        buildInputs =
          runtimeDeps
          ++ [pkgs.pkg-config pkgs.mold]
          ++ lib.optionals pkgs.stdenv.isDarwin [pkgs.libiconv];
      };

      cargoArtifacts = craneLib.buildDepsOnly commonArgs;

      workspace = craneLib.buildPackage (
        commonArgs
        // {
          inherit cargoArtifacts;
        }
      );
    in {
      packages.default = workspace;

      checks = {
        build = workspace;

        audit = craneLib.cargoAudit {
          inherit src advisory-db;
        };

        fmt = craneLib.cargoFmt {
          inherit src;
        };

        clippy = craneLib.cargoClippy (
          commonArgs
          // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--workspace --all-targets -- --deny warnings";
          }
        );

        doc = craneLib.cargoDoc (
          commonArgs
          // {
            inherit cargoArtifacts;
          }
        );

        nextest = craneLib.cargoNextest (
          commonArgs
          // {
            inherit cargoArtifacts;
          }
        );
      };

      devShells.default = craneLib.devShell {
        checks = self.checks.${system};

        packages = [
          pkgs.pkg-config
          pkgs.wayland
          pkgs.libxkbcommon
        ];

        LD_LIBRARY_PATH = lib.makeLibraryPath runtimeDeps;
      };
    });
}
