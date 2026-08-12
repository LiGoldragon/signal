# Agent Instructions

Read `ARCHITECTURE.md` and `NON_IDEAL_AGENTS.md` before editing.

This repository owns only genuine cross-component standards. Keep it pure
vocabulary: no operation roots, daemon actors, storage, runtime policy, or frame
codec.

`ethos/interface.ethos` is the sole schema authority. It is a role-free strict
Interface with explicit producer-owned identity seats in
`src/bootstrap_manifest.rs`. Rust names in the generated projection must remain
encoded; readable aliases create a competing authority and are forbidden.

Regenerate `src/schema/lib/generated.rs` with
`SIGNAL_STANDARD_UPDATE_INTERFACE_ARTIFACTS=1 cargo build`. Structural traits,
Dotos, rkyv, and domain operations belong in producer-owned
`src/schema/lib/behavior.rs` until the language expresses them directly.

`ComponentKind` is closed and partitioned. Insert real components within their
existing zones; do not append blindly or repartition without a major version.

This repository is under fast development and constantly breaking.

## Protos estate status

Stack: correct-new destination
Status: active component contract, current checkout legacy-wired
This checkout is not proof of correct-new adoption.
