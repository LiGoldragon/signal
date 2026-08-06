# signal-standard — architecture

## Center

`signal-standard` is the common vocabulary in which components recognize one
another. It complements `signal-frame`: frame owns domain-free transport
mechanics; standard owns genuine cross-component meanings.

The source is meant to be read and thought through, not hidden behind a
compiler. Its Ethos spellings are the human, agent, harness, and GUI surface;
the Rust projection intentionally carries only encoded identities.

## Owned vocabulary

- `ComponentKind`: one closed roster partitioned into Core, Messaging,
  Interaction, Platform, and Aggregate zones with room for local growth.
- `AuthorizedObjectKind` and `AuthorizedObjectInterest`: the shared four-rung
  narrowing lattice.
- `Differentiator`, `AuthorizedObjectReference`, and
  `ComponentClassification`: the common classification shapes.
- `StandardSocket`: typed local and network reachability.

Only genuine cross-component standards belong here. There are no operation
roots, daemon actors, runtime policy, storage, or frame codecs.

## Authority and projection

`ethos/interface.ethos` is the sole canonical schema source. It declares one
role-free strict `Interface.{1 0 0}`: all Input, Output, and Refusal sections
are empty because vocabulary has no operations.

`src/bootstrap_manifest.rs` contains explicitly minted producer-owned seats for
every fixed identity, declaration, and variant. `build.rs` assembles the
authority-approved transaction, verifies the exact version and canonical
ordering, checks the encoded-only Rust projection, and publishes the owned
Ethos directory through Cargo metadata.

`src/schema/lib/generated.rs` is structural projection only.
`src/schema/lib/behavior.rs` supplies the current-stage structural wire, Dotos,
rkyv, and small domain operations. It creates no second naming authority.

## Evolution

Adding a real component inserts it into the appropriate reserved zone. Do not
append blindly or repartition zones; repartitioning is a major-version event.
Schema decisions must remain independent of Rust, LLVM, and the current OS.
