{
  description = "signal-standard - shared cross-component standards: the reconciled ComponentKind roster, the differentiator, and the authorized-object interest lattice";

  inputs = {
    nixpkgs.url = "github:LiGoldragon/nixpkgs?ref=main";

    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";

    crane.url = "github:ipetkov/crane";
  };

  outputs =
    {
      self,
      nixpkgs,
      fenix,
      crane,
    }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      forSystems = function: nixpkgs.lib.genAttrs systems (system: function system);

      mkContext =
        system:
        let
          pkgs = import nixpkgs { inherit system; };
          toolchain = fenix.packages.${system}.stable.withComponents [
            "cargo"
            "rustc"
            "rustfmt"
            "clippy"
            "rust-src"
          ];
          craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
          # Include the canonical Ethos authority for the build-time
          # transaction and generated-artifact freshness check.
          examplesFilter = path: _type: builtins.match ".*/examples(/.*)?$" path != null;
          ethosFilter = path: _type: builtins.match ".*/ethos(/.*)?$" path != null;
          sourceFilter =
            path: type:
            (craneLib.filterCargoSources path type) || (examplesFilter path type) || (ethosFilter path type);
          src = pkgs.lib.cleanSourceWith {
            src = ./.;
            filter = sourceFilter;
            name = "source";
          };
          cargoVendorDir = craneLib.vendorCargoDeps { inherit src; };
          commonArgs = {
            inherit src cargoVendorDir;
            strictDeps = true;
          };
          cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        in
        {
          inherit
            pkgs
            toolchain
            craneLib
            src
            commonArgs
            cargoArtifacts
            ;
        };
    in
    {
      packages = forSystems (
        system:
        let
          context = mkContext system;
        in
        {
          default = context.craneLib.buildPackage (
            context.commonArgs
            // {
              inherit (context) cargoArtifacts;
            }
          );
        }
      );

      checks = forSystems (
        system:
        let
          context = mkContext system;
        in
        {
          build = context.craneLib.cargoBuild (context.commonArgs // { inherit (context) cargoArtifacts; });
          test = context.craneLib.cargoTest (context.commonArgs // { inherit (context) cargoArtifacts; });
          test-round-trip = context.craneLib.cargoTest (
            context.commonArgs
            // {
              inherit (context) cargoArtifacts;
              cargoTestExtraArgs = "--features dotos-text --test round_trip";
            }
          );
          test-dotos-text = context.craneLib.cargoTest (
            context.commonArgs
            // {
              inherit (context) cargoArtifacts;
              cargoTestExtraArgs = "--features dotos-text --all-targets";
            }
          );
          test-doc = context.craneLib.cargoTest (
            context.commonArgs
            // {
              inherit (context) cargoArtifacts;
              cargoTestExtraArgs = "--doc";
            }
          );
          doc = context.craneLib.cargoDoc (
            context.commonArgs
            // {
              inherit (context) cargoArtifacts;
              RUSTDOCFLAGS = "-D warnings";
            }
          );
          fmt = context.craneLib.cargoFmt { inherit (context) src; };
          clippy = context.craneLib.cargoClippy (
            context.commonArgs
            // {
              inherit (context) cargoArtifacts;
              cargoClippyExtraArgs = "--all-targets -- -D warnings";
            }
          );
          clippy-dotos-text = context.craneLib.cargoClippy (
            context.commonArgs
            // {
              inherit (context) cargoArtifacts;
              cargoClippyExtraArgs = "--features dotos-text --all-targets -- -D warnings";
            }
          );
          rkyv-feature-discipline = context.pkgs.runCommand "signal-standard-rkyv-feature-discipline" { } ''
            ${context.pkgs.gnugrep}/bin/grep -F \
              'rkyv = { version = "0.8", default-features = false, features = ["std", "bytecheck", "little_endian", "pointer_width_32", "unaligned"] }' \
              ${./Cargo.toml} > /dev/null
            touch $out
          '';
          strict-interface-is-sole-authority = context.pkgs.runCommand "signal-standard-strict-interface" { } ''
            test -f ${./ethos/interface.ethos}
            test ! -e ${./.}/schema
            ${context.pkgs.gnugrep}/bin/grep -F 'Interface.{1 0 0}' ${./ethos/interface.ethos} > /dev/null
            touch $out
          '';
          contract-crate-carries-no-runtime = context.pkgs.runCommand "signal-standard-no-runtime" { } ''
            ! ${context.pkgs.gnugrep}/bin/grep -R -E '(^|[^[:alnum:]_])(kameo|tokio|redb|sema|ractor)([^[:alnum:]_]|$)' ${./Cargo.toml} ${./src}
            touch $out
          '';
        }
      );

      devShells = forSystems (
        system:
        let
          context = mkContext system;
        in
        {
          default = context.pkgs.mkShell {
            name = "signal-standard";
            packages = [
              context.pkgs.jujutsu
              context.pkgs.pkg-config
              context.toolchain
            ];
          };
        }
      );
    };
}
