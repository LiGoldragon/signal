# signal

The sema / criome record-vocabulary crate over the shared
`signal-core` wire kernel.

Signal is the **native binary form** of the records criome holds:
sema is by definition computer-cognizable, so its native form is
binary. Nexus records in NOTA syntax are the human-facing
translation; signal is what criome receives and emits after a text
request crosses the nexus daemon.

```
nexus (NOTA) → nexus daemon (translates) → signal (rkyv) → criome
criome (response) → signal → nexus daemon (translates) → nexus (NOTA)
```

Effect-bearing wires use the same family shape — `signal-core`
frames plus relation-specific payload vocabularies:

- signal-forge
  carries the criome ↔ forge leg (effect-bearing build / deploy
  / store-entry verbs).
- signal-arca
  carries the writers ↔ arca-daemon leg (`Deposit`-class verbs
  authorised by criome-signed capability tokens).

`signal-core` owns the universal envelope and six-root verb spine
(`Assert`, `Mutate`, `Retract`, `Match`, `Subscribe`, `Validate`).
Atomicity is structural — multi-op `Request<Payload>` commits as one
unit via its `NonEmpty<Operation>` sequence. This crate owns the
sema / criome payload vocabulary beneath that spine.
Read-algebra (`Project`, `Aggregate`, `Constrain`, `Infer`, `Recurse`)
lives in `sema-engine`'s `ReadPlan`, not as root verbs. The source
still contains transitional duplicate kernel modules while the
kernel-extraction code rebalance finishes; treat `signal-core` as the
authority for those kernel primitives.

## What this crate defines

- **Sema / criome request and reply payloads** — the typed operation
  vocabulary criome consumes and emits.
- **`OutcomeMessage`** — `Ok` (success record kind) or
  `Diagnostic` (failure record kind).
- **Per-verb typed payloads** — `AssertOperation` /
  `MutateOperation` / `RetractOperation` for edits; `QueryOperation`
  for queries; `Records` for typed query results. Multi-op atomic
  commits compose as `Request<Payload>` with `NonEmpty<Operation>`
  via `signal-core::RequestBuilder`. Each payload enum is closed —
  no generic record wrapper, no string kind-name lookup.
- **Flow-graph kinds** — `Node`, `Edge`, `Graph` (with
  paired `NodeQuery` / `EdgeQuery` / `GraphQuery`), `Ok`,
  `RelationKind` (closed 9-variant enum exposing `::ALL`,
  `::from_variant_name`, `::variant_name`). The first sema
  record category criome handles end-to-end.

- **Auxiliary sema record support** — `Diagnostic` /
  `DiagnosticLevel` / `DiagnosticSite` / `DiagnosticSuggestion`;
  `Hash` (BLAKE3 32-byte alias).

`signal-core` defines `Frame`, handshake records, `AuthProof`,
`SemaVerb`, `Slot<T>`, `Revision`, and `PatternField<T>`.

## What this crate does *not* define

- **The Nexus NOTA record vocabulary itself** (the records humans type) —
  defined by the Nexus vocabulary/spec and parsed by nota-codec.
- **The Signal kernel** — owned by `signal-core`.
- **Sema state** — owned by criome.
- **The validator pipeline** — owned by criome.

## Direct authoring — architecturally permitted, practically narrow

Signal is **architecturally peer-shaped** to Nexus records in NOTA
syntax. The mechanical-translation rule guarantees the two forms
agree, and a client that *can* compose rkyv frames directly is doing
a legitimate thing.

- ✓ **Programmatic Rust clients** — services, CI tools, the
  daemon itself. They compose `AssertOperation` / `MutateOperation`
  payloads — single-op via `into_request()`, multi-op via
  `signal-core::RequestBuilder` — in rkyv directly and send.
- ✗ **LLM agents** — current LLMs are trained on text and cannot
  author rkyv binary structures directly. The practical client
  interface for an LLM is **Nexus records in NOTA syntax**, parsed
  into signal by the nexus daemon. Direct LLM signal authoring is a future
  capability — it lands when LLM models are trained against
  binary signal formats. Per Li 2026-04-25: *"not yet, not until
  llm models are trained using binary signal data."*

Both paths arrive at criome as signal frames.

## Wire format

rkyv 0.8 with the canonical pinned feature set per
lore/rust/rkyv.md:
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
