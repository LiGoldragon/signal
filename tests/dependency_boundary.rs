use std::{fs, path::PathBuf, process::Command};

/// The `rev = "…"` this crate's own manifest pins for one Git dependency,
/// rendered as `cargo tree` spells it. Reading the pin instead of repeating
/// it means a repin is witnessed in the resolved graph, not restated here.
fn pinned_revision(crate_name: &str) -> String {
    let manifest = fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"))
        .expect("manifest");
    let line = manifest
        .lines()
        .find(|line| line.trim_start().starts_with(crate_name) && line.contains("rev = \""))
        .unwrap_or_else(|| panic!("{crate_name} is pinned by revision"));
    let (_, rest) = line.split_once("rev = \"").expect("revision");
    let (revision, _) = rest.split_once('"').expect("revision end");
    format!("?rev={revision}")
}

fn cargo_tree(edges: &str, extra: &[&str]) -> String {
    let mut command = Command::new("cargo");
    command.args(["tree", "--edges", edges, "--no-default-features"]);
    command.args(extra);
    let output = command.output().expect("run cargo tree");
    assert!(output.status.success(), "status: {:?}", output.status);
    String::from_utf8(output.stdout).expect("dependency tree")
}

/// Every distinct version of one crate the resolved tree carries. A crate may
/// appear on several branches; more than one version is the disunification
/// this asserts against.
fn resolved_versions(tree: &str, crate_name: &str) -> std::collections::BTreeSet<String> {
    tree.lines()
        .filter_map(|line| {
            let (_, rest) = line.split_once(&format!("{crate_name} v"))?;
            Some(rest.split_whitespace().next()?.to_owned())
        })
        .collect()
}

#[test]
fn default_runtime_tree_excludes_bootstrap_and_retired_crates() {
    let tree = cargo_tree("normal", &[]);
    for forbidden in [
        "core-ethos",
        "name-table",
        "nota",
        concat!("nota", "-codec"),
        "rust-logos",
        concat!("schema", "-language"),
        "schema-rust",
        "sema-translator",
        "signal-core",
        "signal-sema-translator",
        "structural-codec",
    ] {
        assert!(
            !tree.contains(forbidden),
            "runtime contains {forbidden}:\n{tree}"
        );
    }
}

#[test]
fn build_tree_has_one_current_ethos_generator() {
    let tree = cargo_tree("build", &[]);
    assert_eq!(tree.matches("ethos-zero v").count(), 1, "{tree}");
    assert!(tree.contains(&pinned_revision("ethos-zero")), "{tree}");
    assert!(!tree.contains(concat!("schema", "-language")), "{tree}");
    assert!(!tree.contains("schema-rust"), "{tree}");
}

#[test]
fn datom_is_the_only_text_projection_opt_in() {
    let tree = cargo_tree("normal", &["--features", "datom"]);
    for crate_name in ["datom-codec", "protos"] {
        assert_eq!(resolved_versions(&tree, crate_name).len(), 1, "{tree}");
        assert!(tree.contains(&pinned_revision(crate_name)), "{tree}");
    }
    assert!(!tree.contains("dotos"), "{tree}");
    assert!(!tree.contains("nota"), "{tree}");
}

#[test]
fn obsolete_generation_language_is_absent_from_active_build_inputs() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let build = fs::read_to_string(root.join("build.rs")).expect("build source");
    let cargo = fs::read_to_string(root.join("Cargo.toml")).expect("cargo manifest");
    for obsolete in [
        concat!("Generation", "Driver"),
        concat!("Generation", "Plan"),
        concat!("Module", "Emission"),
        concat!("ContractCrate", "Build"),
        concat!("CargoSchema", "Metadata"),
        concat!("schema", "-language"),
    ] {
        assert!(!build.contains(obsolete), "build.rs contains {obsolete}");
        assert!(!cargo.contains(obsolete), "Cargo.toml contains {obsolete}");
    }
}
