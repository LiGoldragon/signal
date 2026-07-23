# Agent instructions — signal

## Repo role

The **sema / criome record vocabulary** with a local legacy wire
envelope. This repo owns its current `Frame`, handshake, auth, slot,
revision, request/reply roots, sema-ecosystem per-verb payloads, and
record kinds (`Node`, `Edge`, `Graph` plus paired `*Query` types).
Reusable pattern markers are imported from `signal-sema`. Read
`NON_IDEAL_AGENTS.md` for legacy envelope debt that must not be copied
into new component contracts.

---

## Carve-outs worth knowing

- **Perfect specificity at the wire** — every verb has its own typed payload (`AssertOperation::Node(Node)`, `MutateOperation::Edge { slot, new, expected_rev }`, `Records::Graph(Vec<Graph>)`). No generic record wrapper, no string-tagged dispatch, no `Unknown` escape variant. New typed kinds and enum variants land here as the schema grows.
- **rkyv feature-set** must match exactly across every rkyv-using crate: `default-features = false, features = ["std", "bytecheck", "little_endian", "pointer_width_32", "unaligned"]`. Pinned to rkyv 0.8.x.

This repository is under fast development and constantly breaking.
