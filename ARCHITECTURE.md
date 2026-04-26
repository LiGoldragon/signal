# ARCHITECTURE — signal

Signal is the **native binary form** of the records criome holds.
Sema — the records — is by definition directly computer-cognizable:
the bytes a record occupies at rest *are* its meaning, no parsing,
no interpretation. Criome IS sema's engine, so criome receives and
serves sema in its native form. Signal is that form on the wire.

Nexus text exists as the human-facing translation. The mechanical-
translation rule (every nexus text construct has exactly one signal
form, and vice versa) keeps the two surfaces in lockstep. Inside
the nexus daemon, text-in becomes signal-out; signal-replies become
text-out.

```
text-speaking peers                  signal-speaking peers
(humans, LLM agents,                  (the nexus daemon talking
 nexus-cli, editor LSPs)              to criome — and any peer
                                       holding typed records)
        │                                       │
        │ pure nexus text                       │ length-prefixed
        │ in / out                              │ rkyv frames
        ▼                                       ▼
┌──────────────────┐                    ┌─────────────────┐
│ /tmp/nexus.sock  │                    │ /tmp/criome.sock│
│  nexus daemon    │  ──── signal ────► │     criome      │
│ (text translator)│  ◄─── signal ───── │ (validator+sema)│
└──────────────────┘                    └─────────────────┘
```

Nexus text is the only non-signal surface in the sema-ecosystem.
Once a request crosses the daemon, it is signal end-to-end.

## Boundaries

Owns:

- `Frame` envelope: `correlation_id`, `principal_hint`,
  `auth_proof`, `body`.
- `Body { Request, Reply }`.
- `Request` and `Reply` enums for every verb (assert, mutate,
  retract, validate, query, subscribe, atomic-batch, handshake).
- `HandshakeRequest` / `HandshakeReply` /
  `HandshakeRejectionReason` — the protocol-version exchange
  that opens a connection.
- `ProtocolVersion { major, minor, patch }` and the
  major-exact / minor-forward compatibility rule.
- `AuthProof` (`SingleOperator` MVP, `BlsSig` and `QuorumProof`
  post-MVP skeletons).
- The full **language IR** absorbed from the former nexus-schema
  crate: `RawRecord`, `RawValue`, `RawLiteral`, `RawPattern`,
  `Selection`, `RawOp`, `AssertOp` / `MutateOp` / `RetractOp`
  / `PatchOp` / `TxnBatch`, `Diagnostic`, `Slot`, `Revision`,
  `Hash`, etc.
- The **flow-graph kinds** (`Node`, `Edge`, `Graph`,
  `KNOWN_KINDS`) — criome's first-milestone substrate.

Does not own:

- Nexus text grammar or parser — see [github.com/LiGoldragon/nexus](https://github.com/LiGoldragon/nexus).
- Sema state — owned by criome.
- Validator pipeline — owned by criome.

## Wire format

rkyv 0.8 with the canonical pinned feature set per
[mentci/reports/074](https://github.com/LiGoldragon/mentci/blob/main/reports/074-portable-rkyv-discipline.md):
`default-features = false, features = ["std", "bytecheck",
"little_endian", "pointer_width_32", "unaligned"]`.

Schema-as-framing: reader and writer both know the record kinds.
Frames are length-prefixed (4-byte big-endian) so a stream socket
can find frame boundaries; everything after the prefix is a rkyv
archive of `Frame`. Nothing in the bytes describes itself.

`Frame::encode` / `Frame::decode` use `rkyv::to_bytes` /
`rkyv::from_bytes` with `bytecheck` validation on read.

## Handshake

Every connection opens with `Request::Handshake`:

1. Initiator sends `Frame { body: Request::Handshake(...) }`.
2. Server validates compatibility (major-exact, minor-forward).
3. Server replies `HandshakeAccepted` or `HandshakeRejected`.
4. On accepted: subsequent frames carry the agreed protocol
   version implicitly.

`SIGNAL_PROTOCOL_VERSION = 0.1.0`. Bump per semver.

## Reply protocol

Replies are paired to requests by **position** on the connection:
the N-th reply is for the N-th request. No correlation IDs.
Replies use the same record kinds as requests; the verb sigil
discipline carries through (`(R)` ↔ `(R)`, `~(R)` ↔ `~(R)`,
`!(R)` ↔ `!(R)`, etc.). Sequence-shaped replies (Query results)
are atomic at the position — never half-emitted; partial failure
becomes a `Diagnostic` *instead of* the sequence at that position.

See [reports/083](https://github.com/LiGoldragon/mentci/blob/main/reports/083-the-return-protocol.md)
for the full reply-protocol design (slot dependencies via tempid
binds, multi-connection parallelism, cancellation by socket
close, subscription event semantics).

## Direct authoring — peer to nexus

Architecturally, signal is peer-shaped to nexus text:

- ✓ **Programmatic Rust clients** (services, CI, the daemon itself)
  may compose typed records directly and send them as signal
  frames — no text round-trip.
- ✗ **LLM agents** author nexus text and let the daemon translate.
  The text is the form they're trained on. Per Li 2026-04-25:
  *"not yet, not until llm models are trained using binary
  signal data."*

Both paths arrive at criome as signal frames.

## Code map

```
src/
├── lib.rs        — module entry + re-exports
├── frame.rs      — Frame envelope, encode/decode, tests
├── handshake.rs  — ProtocolVersion, HandshakeRequest/Reply
├── auth.rs       — AuthProof variants
├── request.rs    — Request enum
├── reply.rs      — Reply enum
├── effect.rs     — Effect, OkReply, RejectedReply, QueryHitReply
├── value.rs      — RawRecord, RawValue, RawLiteral, FieldPath
├── pattern.rs    — RawPattern, FieldConstraint
├── query.rs      — Selection, RawOp, RawProjection
├── edit.rs       — AssertOp, MutateOp, RetractOp, PatchOp,
│                    TxnBatch, TxnOp
├── diagnostic.rs — Diagnostic, DiagnosticLevel, DiagnosticSite
├── slot.rs       — Slot, Revision
├── hash.rs       — Hash (Blake3 32-byte content hash)
└── flow.rs       — Node, Edge, Graph, KNOWN_KINDS
```

## Status

**Skeleton-as-design.** Wire envelope + IR types + flow-graph
kinds defined; round-trip tests cover the envelope.

## Cross-cutting context

- Three-layer messaging story:
  [mentci/reports/077](https://github.com/LiGoldragon/mentci/blob/main/reports/077-nexus-and-signal.md)
- Project-wide architecture:
  [criome/ARCHITECTURE.md](https://github.com/LiGoldragon/criome/blob/main/ARCHITECTURE.md)
