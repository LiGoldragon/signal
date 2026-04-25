# ARCHITECTURE — signal

The rkyv messaging schema between **nexus** and **criome**.
Signal is the *rkyv form of nexus*: nexus parses nexus text
into signal frames; criome processes signal frames and replies.

```
nexus text ─┐                        ┌─ signal rkyv ─┐
            │                        │               │
   client ──┴── client-msg ──> nexus ──> signal ──> criome
                                        <── reply ──
```

This crate is rkyv-only — no serde — per Li's decision: signal
is non-text-form, direct rust rkyv. nexus-schema (a dependency)
provides the language IR payload types; signal owns only the
envelope and protocol.

## Boundaries

Owns:

- `Frame` envelope: `correlation_id`, `principal_hint`,
  `auth_proof`, `body`.
- `Body { Request, Reply }`.
- `Request` enum: `Handshake` + edit / query / validate verbs +
  `Goodbye`.
- `Reply` enum: handshake outcomes + unary outcomes +
  subscription stream events + `Goodbye`.
- `HandshakeRequest` / `HandshakeReply` /
  `HandshakeRejectionReason` — the protocol-version exchange
  that opens a connection.
- `ProtocolVersion { major, minor, patch }` and the
  major-exact / minor-forward compatibility rule.
- `AuthProof` (`SingleOperator` MVP, `BlsSig` and `QuorumProof`
  post-MVP skeletons).
- `Effect`, `OkReply`, `RejectedReply`, `QueryHitReply`,
  `ExecutionPlan`, `ExecutionStep` — outcome shapes.

Does not own:

- Language IR (`RawPattern`, `RawOp`, `AssertOp`, `RawRecord`,
  `Diagnostic`, `Slot`, `Revision`, …) — lives in
  [nexus-schema](https://github.com/LiGoldragon/nexus-schema).
  Signal imports payload types from there.
- The nexus text language — [github.com/LiGoldragon/nexus](https://github.com/LiGoldragon/nexus).
- Sema state — owned by criome.

## Wire format

rkyv 0.8 with the canonical pinned feature set per
[mentci/reports/074](https://github.com/LiGoldragon/mentci/blob/main/reports/074-portable-rkyv-discipline.md):
`default-features = false, features = ["std", "bytecheck",
"little_endian", "pointer_width_32", "unaligned"]`.

The frame schema **is** the framing — both parties know the
schema, no length-prefix layer outside rkyv. `Frame::encode` /
`Frame::decode` are `rkyv::to_bytes` / `rkyv::from_bytes` with
`bytecheck` validation on read.

## Handshake

Every connection MUST open with `Request::Handshake`:

1. Client sends `Frame { auth_proof: None, body:
   Request::Handshake(HandshakeRequest{client_version, ...}) }`.
2. Server validates compatibility (major-exact, minor-forward).
3. Server replies `Reply::HandshakeAccepted` or
   `Reply::HandshakeRejected(reason)`.
4. On accepted: subsequent frames carry `auth_proof: Some(...)`
   and normal request/reply traffic.

`SIGNAL_PROTOCOL_VERSION = 0.1.0`. Bump per semver.

## Direct authoring — peer to nexus

Architecturally, signal is a peer-shaped interface to nexus
text. Practically:

- ✓ **Deterministic programmatic clients** (Rust, scripts, CI)
  may compose `AssertOp` / `MutateOp` / `TxnBatch` in rkyv
  directly and send.
- ✗ **LLM agents** speak nexus text today; they cannot author
  rkyv binary until trained on it. Per Li 2026-04-25: *"not
  yet, not until llm models are trained using binary signal
  data."*

Both paths arrive at criome as signal frames.

## Code map

```
src/
├── lib.rs        — module entry + re-exports
├── frame.rs      — Frame envelope, encode/decode, tests
├── handshake.rs  — ProtocolVersion, HandshakeRequest/Reply
├── auth.rs       — AuthProof variants
├── request.rs    — Request enum + SubscribeOp + ValidateOp
├── reply.rs      — Reply enum + ValidateResult + Bindings
└── effect.rs     — Effect, OkReply, RejectedReply,
                    QueryHitReply, ExecutionPlan, ExecutionStep
```

## Status

**Skeleton-as-design**, 4 round-trip tests pass.

## Cross-cutting context

- Three-layer messaging story:
  [mentci/reports/077](https://github.com/LiGoldragon/mentci/blob/main/reports/077-nexus-and-signal.md)
- Project-wide architecture:
  [criome/ARCHITECTURE.md](https://github.com/LiGoldragon/criome/blob/main/ARCHITECTURE.md)
