# signal

The rkyv messaging schema between **nexusd** and **criomed**.
Signal is the *rkyv form of nexus*: nexusd parses nexus text into
signal frames; criomed processes signal frames and replies.

Per [mentci-next/reports/077](https://github.com/LiGoldragon/mentci-next/blob/main/reports/077-nexus-and-signal.md):

> nexus (text) → nexusd (translates) → signal (rkyv) → criomed
>
> criomed (response) → signal → nexusd (translates) → nexus (text)

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
- **Sema state** — owned by criomed.

## Direct authoring

Agents and tools that prefer to skip nexus parsing may compose
signal `Frame`s directly in rkyv and send them to nexusd. Signal
is a peer to nexus text, not just its compiled form. The
mechanical-translation rule (per
[mentci-next/reports/070 §7](https://github.com/LiGoldragon/mentci-next/blob/main/reports/070-nexus-language-and-contract.md))
guarantees the two forms agree.

## Wire format

rkyv 0.8 with the canonical pinned feature set per
[mentci-next/reports/074](https://github.com/LiGoldragon/mentci-next/blob/main/reports/074-portable-rkyv-discipline.md):
`default-features = false, features = ["std", "bytecheck",
"little_endian", "pointer_width_32", "unaligned"]`.

The frame schema *is* the framing — both parties know the rkyv
schema. No length-prefix layer outside rkyv.

## License

[License of Non-Authority](LICENSE.md).
