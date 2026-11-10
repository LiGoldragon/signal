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

      # Every test runs under the optional datom feature, the datom tests with
      # it; the default features are tested apart.
      memberChecks =
        { check, ... }:
        {
          signal = check "signal" [ "default" "datom" ];
        };

      extraChecks =
        {
          pkgs,
          toolchain,
          check,
          workspace,
          ...
        }:
        let
          lib = pkgs.lib;
          crates = workspace.internal.crates;
          signal = crates.signal;

          # The packages the built signal crate reaches. The root's optional
          # dependencies join when a feature in the list turns them on; below
          # the root every dependency joins, the superset of what cargo builds.
          enabled =
            features:
            lib.concatMap (
              feature: map (lib.removePrefix "dep:") (lib.filter (lib.hasPrefix "dep:") (signal.features.${feature} or [ ]))
            ) features;
          rootDependencies =
            list: features:
            lib.filter (dependency: !(dependency.optional or false) || lib.elem dependency.name (enabled features)) list;
          reached =
            roots: edges:
            map (item: crates.${item.key}) (
              builtins.genericClosure {
                startSet = map (dependency: { key = dependency.packageId; }) roots;
                operator =
                  item:
                  map (dependency: { key = dependency.packageId; }) (
                    lib.concatMap (edge: crates.${item.key}.${edge} or [ ]) edges
                  );
              }
            );
          runtime = features: reached (rootDependencies signal.dependencies features) [ "dependencies" ];
          building = reached (rootDependencies signal.buildDependencies [ ]) [
            "dependencies"
            "buildDependencies"
          ];
          named = name: crates': lib.filter (crate: crate.crateName == name) crates';
          sourceRev = crate: crate.src.rev or "";
          pinned = name: manifest.dependencies.${name}.rev or manifest.build-dependencies.${name}.rev;
          pinnedAt = name: crate: lib.hasPrefix (pinned name) (sourceRev crate);
          absent =
            where: names: crates':
            lib.concatMap (
              name: lib.optional (named name crates' != [ ]) "${where} contains ${name}"
            ) names;
          unified =
            where: names: crates':
            lib.concatMap (
              name:
              let
                found = named name crates';
              in
              lib.optional (lib.length found != 1) "${where} has ${toString (lib.length found)} ${name}"
              ++ lib.optional (lib.length found == 1 && !pinnedAt name (lib.head found)) "${where} ${name} is not at the pinned revision"
            ) names;
          retired = [
            "core-ethos"
            "name-table"
            "nota"
            (builtins.concatStringsSep "-" [ "nota" "codec" ])
            "rust-logos"
            (builtins.concatStringsSep "-" [ "schema" "language" ])
            "schema-rust"
            "sema-translator"
            "signal-core"
            "signal-sema-translator"
            "structural-codec"
          ];
          failures =
            absent "the default runtime tree" retired (runtime [ ])
            ++ unified "the build tree" [ "ethos-zero" ] building
            ++ absent "the build tree" [
              (builtins.concatStringsSep "-" [ "schema" "language" ])
              "schema-rust"
            ] building
            ++ unified "the datom runtime tree" [ "datom-codec" "protos" ] (runtime [ "datom" ])
            ++ absent "the datom runtime tree" [ "dotos" "nota" ] (runtime [ "datom" ]);
          grep = "${pkgs.gnugrep}/bin/grep";
          manifest = builtins.fromTOML (builtins.readFile ./Cargo.toml);
          rust = pkgs.lib.fileset.toSource {
            root = ./.;
            fileset = pkgs.lib.fileset.unions [ ./src ./tests ];
          };
        in
        {
          test-default-features = check "signal" [ "default" ];
          # The tree Nix builds is the tree these assertions read: one current
          # ethos-zero generator, datom the only text-projection opt-in, and
          # no bootstrap or retired crate in the default runtime tree.
          dependency-boundary = pkgs.runCommand "signal-dependency-boundary" { } (
            if failures == [ ] then
              "touch $out"
            else
              "printf '%s\\n' ${lib.escapeShellArgs failures} >&2; exit 1"
          );
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
        };
    };
}
