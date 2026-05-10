# Agent instructions — signal

You **MUST** read AGENTS.md at `github:ligoldragon/lore` — the workspace contract.

## Repo role

The **sema / criome record vocabulary** layered over
`signal-core`'s shared wire kernel. `signal-core` owns the generic
frame, handshake, auth, slot, revision, pattern-field, and verb spine
types. This repo owns the sema-ecosystem's per-verb payloads and
record kinds (`Node`, `Edge`, `Graph` plus paired `*Query` types).

---

## Carve-outs worth knowing

- **Perfect specificity at the wire** — every verb has its own typed payload (`AssertOperation::Node(Node)`, `MutateOperation::Edge { slot, new, expected_rev }`, `Records::Graph(Vec<Graph>)`). No generic record wrapper, no string-tagged dispatch, no `Unknown` escape variant. New typed kinds and enum variants land here as the schema grows.
- **rkyv feature-set** must match exactly across every rkyv-using crate per lore/rust/rkyv.md: `default-features = false, features = ["std", "bytecheck", "little_endian", "pointer_width_32", "unaligned"]`. Pinned to rkyv 0.8.x.
