# Agent instructions — signal

You **MUST** read AGENTS.md at `github:ligoldragon/lore` — the workspace contract.

## Repo role

The **wire format** — `Frame` envelope + per-verb typed payloads (rkyv) + protocol-level types (`HandshakeRequest`, `Diagnostic`, `Slot`, etc.) + the flow-graph data kinds (`Node`, `Edge`, `Graph` plus paired `*Query` types).

---

## Carve-outs worth knowing

- **Perfect specificity at the wire** — every verb has its own typed payload (`AssertOperation::Node(Node)`, `MutateOperation::Edge { slot, new, expected_rev }`, `Records::Graph(Vec<Graph>)`). No generic record wrapper, no string-tagged dispatch, no `Unknown` escape variant. New typed kinds and enum variants land here as the schema grows.
- **rkyv feature-set** must match exactly across every rkyv-using crate per lore/rust/rkyv.md: `default-features = false, features = ["std", "bytecheck", "little_endian", "pointer_width_32", "unaligned"]`. Pinned to rkyv 0.8.x.
