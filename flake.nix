{
  description = "signal - the shared Signal layer: the portable rkyv Signal frame, its wire framing, and the cross-component taxonomy every Nexus depends on";

  inputs.base.url = "github:LiGoldragon/build-base";

  # Cargo.nix is generated from Cargo.lock by the build base's crate2nix and
  # committed, so evaluation imports no derivation:
  #   nix shell github:LiGoldragon/build-base#toolchain github:LiGoldragon/build-base#crate2nix -c crate2nix generate
  outputs =
    { self, base }:
    base.lib.workspaceOutputs {
      cargoNix = ./Cargo.nix;

      # Every test runs under both optional features, the datom and transport
      # tests with them; the default features are tested apart.
      memberChecks =
        { check, ... }:
        {
          signal = check "signal" [ "default" "datom" "transport" ];
        };

      extraChecks =
        {
          pkgs,
          toolchain,
          check,
          ...
        }:
        let
          grep = "${pkgs.gnugrep}/bin/grep";
          manifest = builtins.fromTOML (builtins.readFile ./Cargo.toml);
          rust = pkgs.lib.fileset.toSource {
            root = ./.;
            fileset = pkgs.lib.fileset.unions [ ./src ./tests ];
          };
          featured = check "signal" [ "default" "datom" "transport" ];
        in
        {
          test-default-features = check "signal" [ "default" ];
          rkyv-feature-discipline = pkgs.runCommand "signal-rkyv-feature-discipline" { } ''
            ${grep} -F \
              'rkyv = { version = "0.8", default-features = false, features = ["std", "bytecheck", "little_endian", "pointer_width_32", "unaligned"] }' \
              ${./Cargo.toml} > /dev/null
            touch $out
          '';
          strict-signal-is-sole-authority = pkgs.runCommand "signal-strict-signal" { } ''
            test -f ${./ethos/signal.ethos}
            ${grep} -F 'Signal' ${./ethos/signal.ethos} > /dev/null
            ! ${grep} -R -E 'schema-rust|dotos' ${./Cargo.toml} ${./build.rs}
            touch $out
          '';
          carries-no-engine = pkgs.runCommand "signal-no-engine" { } ''
            ! ${grep} -R -E '(^|[^[:alnum:]_])(kameo|redb|sema|ractor)([^[:alnum:]_]|$)' ${./Cargo.toml} ${./src}
            touch $out
          '';
          fmt = pkgs.runCommand "signal-fmt" { nativeBuildInputs = [ toolchain ]; } ''
            rustfmt --check --edition ${manifest.package.edition} ${rust}/src/lib.rs ${rust}/tests/*.rs
            touch $out
          '';
          # The test build, every feature, compiled by clippy-driver under the
          # crate's lints, every target, warnings refused.
          clippy = featured.tests.override {
            useClippy = true;
            clippy = toolchain;
            inherit (manifest) lints;
            extraRustcOpts = [ "-D" "warnings" ];
          };
        };
    };
}
