use std::{fs, path::PathBuf};

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
