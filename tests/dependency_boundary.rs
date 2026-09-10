use std::{fs, path::PathBuf, process::Command};

fn cargo_tree(edges: &str, extra: &[&str]) -> String {
    let mut command = Command::new("cargo");
    command.args(["tree", "--edges", edges, "--no-default-features"]);
    command.args(extra);
    let output = command.output().expect("run cargo tree");
    assert!(output.status.success(), "status: {:?}", output.status);
    String::from_utf8(output.stdout).expect("dependency tree")
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
    assert_eq!(tree.matches("ethos-zero v6.1.6").count(), 1, "{tree}");
    assert!(tree.contains("ethos-zero?rev=4695ee0c1f5d00dcf5cceba08f5fa00412b92184"));
    assert!(!tree.contains(concat!("schema", "-language")), "{tree}");
    assert!(!tree.contains("schema-rust"), "{tree}");
}

#[test]
fn datom_is_the_only_text_projection_opt_in() {
    let tree = cargo_tree("normal", &["--features", "datom"]);
    assert!(tree.contains("datom-codec v0.25.6"), "{tree}");
    assert!(tree.contains("protos v0.29.1"), "{tree}");
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
