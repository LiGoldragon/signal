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
fn build_tree_has_one_exact_corrected_schema_rust() {
    let tree = cargo_tree("build", &[]);
    assert_eq!(tree.matches("schema-rust v0.15.1").count(), 1, "{tree}");
    assert!(tree.contains("schema-rust.git?rev=664335240a40728826cfaa09e3100cd867031912#66433524"));
    assert!(!tree.contains(concat!("schema", "-language")), "{tree}");
}

#[test]
fn dotos_is_the_only_text_projection_opt_in() {
    let tree = cargo_tree("normal", &["--features", "dotos-text"]);
    assert!(tree.contains("dotos"), "{tree}");
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
