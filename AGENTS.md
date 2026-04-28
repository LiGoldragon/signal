# Agent instructions

Repo role: the **wire format** — `Frame` envelope + per-verb typed payloads (rkyv) + protocol-level types (`HandshakeRequest`, `Diagnostic`, `Slot`, etc.) + the schema-as-data types (`KindDecl`, `FieldDecl`).

Read [ARCHITECTURE.md](ARCHITECTURE.md) for the boundaries and shape.

Workspace conventions live in [mentci/AGENTS.md](https://github.com/LiGoldragon/mentci/blob/main/AGENTS.md).

**Perfect specificity at the wire** — every verb has its own typed payload (`AssertOperation::Node(Node)`, `MutateOperation::Edge { slot, new, expected_rev }`, `Records::Graph(Vec<Graph>)`). No generic record wrapper, no string-tagged dispatch, no `Unknown` escape variant. New typed kinds and enum variants land here as the schema grows.

**rkyv feature-set** must match exactly across every rkyv-using crate per [tools-documentation/rust/rkyv.md](https://github.com/LiGoldragon/tools-documentation/blob/main/rust/rkyv.md): `default-features = false, features = ["std", "bytecheck", "little_endian", "pointer_width_32", "unaligned"]`. Pinned to rkyv 0.8.x.
