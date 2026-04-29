# signal

The base rkyv wire-protocol crate of the sema-ecosystem.
Signal is the **native binary form** of the records criome holds:
sema is by definition computer-cognizable, so its native form is
binary. Nexus text is the human-facing translation; signal is what
criome receives and emits on the wire — and what every front-end
client (nexus, the GUI editor, mentci-lib, agents speaking signal
directly) sends in.

```
nexus (text) → nexus daemon (translates) → signal (rkyv) → criome
criome (response) → signal → nexus daemon (translates) → nexus (text)
```

Effect-bearing wires layer atop signal — re-using its `Frame`,
handshake, and auth — and add their own per-verb payloads:

- [signal-forge](https://github.com/LiGoldragon/signal-forge)
  carries the criome ↔ forge leg (effect-bearing build / deploy
  / store-entry verbs).
- [signal-arca](https://github.com/LiGoldragon/signal-arca)
  carries the writers ↔ arca-daemon leg (`Deposit`-class verbs
  authorised by criome-signed capability tokens).

This crate owns the universal envelope and the front-end verb
surface; the layered crates own only the leg-specific verbs.

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
- **Per-verb typed payloads** — `AssertOp` / `MutateOp` /
  `RetractOp` / `AtomicBatch` / `BatchOp` for edits;
  `QueryOp` for queries; `Records` for typed query results.
  Each is a closed enum of typed kinds — no generic record
  wrapper, no string kind-name lookup.
- **`PatternField<T>`** — `Wildcard | Bind | Match(T)`,
  used per-field in the `*Query` types.
- **Flow-graph kinds** — `Node`, `Edge`, `Graph` (with
  paired `NodeQuery` / `EdgeQuery` / `GraphQuery`), `Ok`,
  `RelationKind` (closed 9-variant enum exposing `::ALL`,
  `::from_variant_name`, `::variant_name`). The first sema
  record category criome handles end-to-end.


- **Auxiliary** — `Diagnostic` / `DiagnosticLevel` /
  `DiagnosticSite` / `DiagnosticSuggestion`; `Slot` /
  `Revision` (transparent u64 newtypes); `Hash` (BLAKE3
  32-byte alias).

## What this crate does *not* define

- **The nexus text language itself** (the syntax humans type) —
  defined by the
  [nexus](https://github.com/LiGoldragon/nexus) grammar and
  parsed by
  [nota-codec](https://github.com/LiGoldragon/nota-codec).
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
[tools-documentation/rust/rkyv.md](https://github.com/LiGoldragon/tools-documentation/blob/main/rust/rkyv.md):
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
