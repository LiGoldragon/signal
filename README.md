# signal-standard

The shared cross-component vocabulary: the closed, partitioned component
roster; authorized-object classification and interest; component nameplates;
and ordinary daemon connection points.

The crate is deliberately only vocabulary. It owns no operations, daemon,
storage, frame codec, or runtime policy. Contracts import these identities into
their own Interfaces.

`ethos/interface.ethos` is the sole schema authority. It is a role-free strict
`Interface.{1 0 0}` whose identities have explicit producer-owned seats.
`build.rs` verifies the authorized transaction and checks the encoded-only Rust
projection in `src/schema/lib/generated.rs`.

The default feature set carries binary rkyv behavior without a text parser.
`dotos-text` adds the Dotos surface used by humans, agents, harnesses, and GUIs.
Regenerate the checked projection with:

```sh
SIGNAL_STANDARD_UPDATE_INTERFACE_ARTIFACTS=1 cargo build
```
