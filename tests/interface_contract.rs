use std::{fs, path::PathBuf};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn current_signal_is_the_sole_schema_authority() {
    let root = root();
    let source = fs::read_to_string(root.join("ethos/signal.ethos")).expect("Signal exists");
    let generated = fs::read_to_string(root.join("src/generated/signal.rs"))
        .expect("generated projection exists");
    assert!(source.starts_with("Signal\n[]\n[]\n[]\n["));
    assert!(source.contains("ComponentKind.["));
    assert!(source.contains("ComponentClassification.{"));
    assert!(
        !root.join("schema").exists(),
        "retired schema projection tree is absent"
    );
    assert!(generated.contains("pub enum ComponentKind"));
    assert!(generated.contains("pub enum AuthorizedObjectInterest"));
}

#[test]
fn public_source_constants_match_authored_and_generated_contract() {
    assert_eq!(
        signal_standard::STANDARD_SIGNAL_SOURCE,
        include_str!("../ethos/signal.ethos")
    );
    assert!(signal_standard::STANDARD_SIGNAL_RUST.contains("pub enum ComponentKind"));
}
