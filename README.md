# signal

The rkyv messaging schema between **nexus** and **criome**.
Signal is the *rkyv form of nexus*: nexus parses nexus text into
signal frames; criome processes signal frames and replies.

Per [mentci-next/reports/077](https://github.com/LiGoldragon/mentci-next/blob/main/reports/077-nexus-and-signal.md):

> nexus (text) → nexus (translates) → signal (rkyv) → criome
>
> criome (response) → signal → nexus (translates) → nexus (text)

## What this crate defines

- **`Frame`** — the rkyv envelope that crosses the UDS socket.
  Every signal message is one `Frame`.
- **`Request`** / **`Reply`** — the verb / outcome enums. Edit
  verbs (Assert / Mutate / Retract / Patch / TxnBatch); query
  verbs (Query / Subscribe / Unsubscribe); read-only Validate.
- **`HandshakeRequest`** / **`HandshakeReply`** — the protocol
  version exchange that opens a connection.
- **`AuthProof`** — single-operator MVP, BLS / quorum post-MVP.
- **`Effect`**, **`OkReply`**, **`RejectedReply`**,
  **`QueryHitReply`** — outcome shapes.

## What this crate does *not* define

- **Language IR** (RawPattern, RawOp, AssertOp, RawRecord, …)
  lives in [nexus-schema](https://github.com/LiGoldragon/nexus-schema).
  Signal imports those types as request/reply payloads.
- **The nexus text language itself** (the syntax humans type) —
  defined by the `nexus` grammar repo and parsed by
  `nota-serde-core`.
- **Sema state** — owned by criome.

## Direct authoring — architecturally permitted, practically narrow

Signal is **architecturally peer-shaped** to nexus text. The
mechanical-translation rule (per
[mentci-next/reports/070 §7](https://github.com/LiGoldragon/mentci-next/blob/main/reports/070-nexus-language-and-contract.md))
guarantees the two forms agree, and a client that *can* compose
rkyv frames directly is doing a legitimate thing.

**But who can actually do that today?**

- ✓ **Deterministic programmatic clients** — Rust code, scripts,
  CI tools that generate transactional batches from program
  structure. They compose `AssertOp` / `MutateOp` / `TxnBatch` in
  rkyv directly and send.
- ✗ **LLM agents** — current LLMs are trained on text and
  cannot author rkyv binary structures directly. The practical
  client interface for an LLM is **nexus text**, parsed into
  signal by nexus. Direct LLM signal authoring is a future
  capability — it lands when LLM models are trained against
  binary signal formats.

Per Li 2026-04-25: *"not yet, not until llm models are trained
using binary signal data."*

Both paths arrive at criome as signal frames. Choose the path
your client can author.

## Wire format

rkyv 0.8 with the canonical pinned feature set per
[mentci-next/reports/074](https://github.com/LiGoldragon/mentci-next/blob/main/reports/074-portable-rkyv-discipline.md):
`default-features = false, features = ["std", "bytecheck",
"little_endian", "pointer_width_32", "unaligned"]`.

The frame schema *is* the framing — both parties know the rkyv
schema. No length-prefix layer outside rkyv.

## License

[License of Non-Authority](LICENSE.md).
