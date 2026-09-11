# Agent Instructions

Read `ARCHITECTURE.md`, `DESIGN.md`, and `NON_IDEAL_AGENTS.md` before
editing.

This repository owns the portable rkyv Signal frame, the wire framing that
carries it, and genuine cross-component vocabulary. It owns no operation
roots, no Nexus actors, no storage, and no runtime policy.

`ethos/signal.ethos` is the schema authority for the taxonomy. `build.rs`
generates from it and asserts the result equals the checked-in
`src/generated/signal.rs`; regenerate by writing the generator's output to
that file. Rust names in the generated projection must remain encoded;
readable aliases create a competing authority and are forbidden.

`ComponentKind` is closed and partitioned. Insert real components within
their existing zones; do not append blindly or repartition without a major
version.

The protocol layered on top of the rkyv archive is not decided. Do not
design it here. `DESIGN.md` is inherited reasoning from the archived legacy
`signal` repository, not specification.

The framing's byte order is big-endian and is a wire commitment: changing
it is a major version and breaks every peer at once.

This repository is under fast development and constantly breaking.

## Protos estate status

Stack: correct-new destination
Status: active component contract, current checkout legacy-wired
This checkout is not proof of correct-new adoption.
