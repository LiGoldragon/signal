{
  description = "signal - the shared Signal layer: the portable rkyv Signal frame, its wire framing, and the cross-component taxonomy every Nexus depends on";

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
          ethosFilter = path: _type: builtins.match ".*/ethos(/.*)?$" path != null;
          sourceFilter =
            path: type:
            (craneLib.filterCargoSources path type) || (ethosFilter path type);
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
          test-datom-round-trip = context.craneLib.cargoTest (
            context.commonArgs
            // {
              inherit (context) cargoArtifacts;
              cargoTestExtraArgs = "--features datom --test round_trip";
            }
          );
          test-datom = context.craneLib.cargoTest (
            context.commonArgs
            // {
              inherit (context) cargoArtifacts;
              cargoTestExtraArgs = "--features datom --all-targets";
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
          test-transport = context.craneLib.cargoTest (
            context.commonArgs
            // {
              inherit (context) cargoArtifacts;
              cargoTestExtraArgs = "--features transport --all-targets";
            }
          );
          clippy-transport = context.craneLib.cargoClippy (
            context.commonArgs
            // {
              inherit (context) cargoArtifacts;
              cargoClippyExtraArgs = "--features transport --all-targets -- -D warnings";
            }
          );
          clippy-datom = context.craneLib.cargoClippy (
            context.commonArgs
            // {
              inherit (context) cargoArtifacts;
              cargoClippyExtraArgs = "--features datom --all-targets -- -D warnings";
            }
          );
          rkyv-feature-discipline = context.pkgs.runCommand "signal-rkyv-feature-discipline" { } ''
            ${context.pkgs.gnugrep}/bin/grep -F \
              'rkyv = { version = "0.8", default-features = false, features = ["std", "bytecheck", "little_endian", "pointer_width_32", "unaligned"] }' \
              ${./Cargo.toml} > /dev/null
            touch $out
          '';
          strict-signal-is-sole-authority = context.pkgs.runCommand "signal-strict-signal" { } ''
            test -f ${./ethos/signal.ethos}
            ${context.pkgs.gnugrep}/bin/grep -F 'Signal' ${./ethos/signal.ethos} > /dev/null
            ! ${context.pkgs.gnugrep}/bin/grep -R -E 'schema-rust|dotos' ${./Cargo.toml} ${./build.rs}
            touch $out
          '';
          carries-no-engine = context.pkgs.runCommand "signal-no-engine" { } ''
            ! ${context.pkgs.gnugrep}/bin/grep -R -E '(^|[^[:alnum:]_])(kameo|redb|sema|ractor)([^[:alnum:]_]|$)' ${./Cargo.toml} ${./src}
            touch $out
          '';
          default-features-carry-no-runtime = context.craneLib.cargoBuild (
            context.commonArgs
            // {
              inherit (context) cargoArtifacts;
              cargoExtraArgs = "--no-default-features";
            }
          );
        }
      );

      devShells = forSystems (
        system:
        let
          context = mkContext system;
        in
        {
          default = context.pkgs.mkShell {
            name = "signal";
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
