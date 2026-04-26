# signal

The rkyv messaging schema between **nexus** and **criome**.
Signal is the **native binary form** of the records criome holds:
sema is by definition computer-cognizable, so its native form is
binary. Nexus text is the human-facing translation; signal is what
criome receives and emits on the wire.

```
nexus (text) → nexus daemon (translates) → signal (rkyv) → criome
criome (response) → signal → nexus daemon (translates) → nexus (text)
```

## What this crate defines

- **`Frame`** — the rkyv envelope that crosses the UDS socket.
  Every signal message is one `Frame`. No correlation ID; replies
  pair to requests by **position** on the connection (FIFO).
- **`Request`** — verb enum: `Handshake`, `Assert`, `Mutate`,
  `Retract`, `AtomicBatch`, `Query`, `Subscribe`, `Validate`.
- **`Reply`** — outcome enum: `HandshakeAccepted` /
  `HandshakeRejected`, `Outcome` (single), `Outcomes` (multi),
  `Records` (query result).
- **`OutcomeMessage`** — `Ok` (success record kind) or
  `Diagnostic` (failure record kind).
- **`HandshakeRequest`** / **`HandshakeReply`** — the protocol
  version exchange that opens a connection.
- **`AuthProof`** — single-operator MVP, BLS / quorum post-MVP.
- **Language IR** — `RawRecord`, `RawValue`, `Diagnostic`,
  `Slot`, `Revision`, `Hash`, `RawPattern`, `Selection`,
  `RawOp`, `AssertOp`, `MutateOp`, `RetractOp`, `AtomicBatch`,
  `BatchOp`. Absorbed from the former nexus-schema crate
  (shelved 2026-04-25).
- **Flow-graph kinds** — `Node`, `Edge`, `Graph`, `Ok`,
  `RelationKind`, `KNOWN_KINDS`. The first sema record category
  criomed handles end-to-end.

## What this crate does *not* define

- **The nexus text language itself** (the syntax humans type) —
  defined by the
  [nexus](https://github.com/LiGoldragon/nexus) grammar and
  parsed by
  [nota-serde-core](https://github.com/LiGoldragon/nota-serde-core).
- **Sema state** — owned by criome.
- **The validator pipeline** — owned by criome.

## Direct authoring — architecturally permitted, practically narrow

Signal is **architecturally peer-shaped** to nexus text. The
mechanical-translation rule guarantees the two forms agree, and a
client that *can* compose rkyv frames directly is doing a
legitimate thing.

- ✓ **Programmatic Rust clients** — services, CI tools, the
  daemon itself. They compose `AssertOp` / `MutateOp` /
  `AtomicBatch` in rkyv directly and send.
- ✗ **LLM agents** — current LLMs are trained on text and cannot
  author rkyv binary structures directly. The practical client
  interface for an LLM is **nexus text**, parsed into signal by
  the nexus daemon. Direct LLM signal authoring is a future
  capability — it lands when LLM models are trained against
  binary signal formats. Per Li 2026-04-25: *"not yet, not until
  llm models are trained using binary signal data."*

Both paths arrive at criome as signal frames.

## Wire format

rkyv 0.8 with the canonical pinned feature set per
[mentci/reports/074](https://github.com/LiGoldragon/mentci/blob/main/reports/074-portable-rkyv-discipline.md):
`default-features = false, features = ["std", "bytecheck",
"little_endian", "pointer_width_32", "unaligned"]`.

Schema-as-framing: both parties know the record kinds. Frames are
length-prefixed (4-byte big-endian) on the wire so a stream socket
can find frame boundaries; everything after the prefix is a rkyv
archive of `Frame`. Nothing in the bytes describes itself.

`Frame::encode` / `Frame::decode` use `rkyv::to_bytes` /
`rkyv::from_bytes` with `bytecheck` validation on read.

## License

[License of Non-Authority](LICENSE.md).
