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

- `Frame` envelope: `principal_hint`, `auth_proof`, `body`. No
  correlation id — replies pair to requests by **position** on
  the connection (FIFO).
- `Body { Request, Reply }`.
- `Request` enum: `Handshake`, `Assert`, `Mutate`, `Retract`,
  `AtomicBatch`, `Query`, `Subscribe`, `Validate`.
- `Reply` enum: `HandshakeAccepted` / `HandshakeRejected`,
  `Outcome` (single-element edit reply), `Outcomes` (multi-element
  edit reply), `Records` (query result).
- `OutcomeMessage`: `Ok` (success record kind) or `Diagnostic`
  (failure record kind).
- `HandshakeRequest` / `HandshakeReply` /
  `HandshakeRejectionReason` — the protocol-version exchange
  that opens a connection.
- `ProtocolVersion { major, minor, patch }` and the
  major-exact / minor-forward compatibility rule.
- `AuthProof` (`SingleOperator` MVP, `BlsSig` and `QuorumProof`
  post-MVP skeletons).
- The **per-verb typed payloads**: `AssertOperation` /
  `MutateOperation` / `RetractOperation` / `AtomicBatch` /
  `BatchOperation` for edits; `QueryOperation` for queries;
  `Records` for typed query results. Each is a closed enum
  of typed kinds (no generic wrapper).
- The **pattern field** type: `PatternField<T>` with
  `Wildcard | Bind | Match(T)` variants, used per-field in
  `*Query` types.
- The **schema-as-data type**: `KindDecl` (with `FieldDecl`,
  `Cardinality`, and the paired `KindDeclQuery`) — see
  [`src/schema.rs`](src/schema.rs).
- The **flow-graph kinds**: `Node`, `Edge`, `Graph` (with
  paired `NodeQuery` / `EdgeQuery` / `GraphQuery`), `Ok`,
  `RelationKind` (closed enum of 9 relation variants — Flow,
  DependsOn, Contains, References, Produces, Consumes, Calls,
  Implements, IsA). Encoding/decoding handled by the
  [nota-derive](https://github.com/LiGoldragon/nota-derive)
  derives — no hand-written `from_variant_name` /
  `variant_name` helpers needed.
- Auxiliary types: `Diagnostic` + `DiagnosticLevel` +
  `DiagnosticSite` + `DiagnosticSuggestion`; `Slot` and
  `Revision` (private-field `u64` newtypes deriving
  `NotaTransparent` so the wire form is the bare integer
  + `From<u64>` and `From<Slot> for u64` conversions for
  ergonomic construction); `Hash` (32-byte BLAKE3 alias).

Does not own:

- Nexus text grammar or parser — see [github.com/LiGoldragon/nexus](https://github.com/LiGoldragon/nexus).
- Sema state — owned by criome.
- Validator pipeline — owned by criome.

## Schema discipline

Signal is the place where new typed kinds and enum variants land
as the system grows. The "no keywords" rule from the nexus
grammar applies to the **parser** only — there are no reserved
words like `SELECT` or `IF` that the parser dispatches on.
**Schema-level typed enums** (like `RelationKind { DependsOn,
Contains, … }` or `OutcomeMessage { Ok, Diagnostic }`) are
encouraged. Adding new strongly-typed kinds is the central activity
of evolving signal.

### Perfect specificity at the wire

Signal carries the project's [perfect-specificity
invariant](https://github.com/LiGoldragon/criome/blob/main/ARCHITECTURE.md#invariant-d)
in its concrete shape. Every verb's payload is its own closed
enum of typed kinds — `AssertOperation { Node(Node) | Edge(Edge) | … }`,
`MutateOperation { Node { slot, new, expected_rev } | … }`,
`QueryOperation { Node(NodeQuery) | … }`,
`Records { Node(Vec<Node>) | … }`. There is no shared
`KnownRecord` wrapper, no generic record envelope, no string
kind-name lookup at runtime. The wire knows what it carries by
type; consumers `match` exhaustively.

A pattern/query is itself a record kind: `NodeQuery` is paired
with `Node`, generated from the same `KindDecl` by rsc. The
grammar `(| ... |)` dispatches to the `*Query` variant of the
named kind — no parallel "pattern" type-system layer exists.

No `Unknown` escape variant. The closed enum is exhaustively
closed; rebuilds bring the world forward together via the
criome self-host loop. Schema-as-data lives in `KindDecl`
records in sema; the typed Rust code in this crate is its
projection (rsc closes the loop post-M0; M0 hand-edits the
projection to match the bootstrap KindDecls in genesis.nexus).

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

For dependent edits where a later request needs the slot
assigned by an earlier one, the **client orchestrates** —
captures the assigned slot from the earlier reply (in its host
language) and substitutes it into the later request. Nexus has
no variables, no scoping, no cross-request state. For
parallelism, open multiple connections — each is its own serial
lane.

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
├── frame.rs      — Frame envelope, encode/decode, FrameDecodeError
├── handshake.rs  — ProtocolVersion, HandshakeRequest/Reply, HandshakeRejectionReason
├── auth.rs       — AuthProof variants (BlsSignature, ...), BlsG1
├── request.rs    — Request enum (per-verb dispatch) + ValidateOperation
├── reply.rs      — Reply enum, OutcomeMessage, Records (typed per kind)
├── edit.rs       — AssertOperation / MutateOperation / RetractOperation
│                    + AtomicBatch / BatchOperation (rkyv-only for M0)
├── query.rs      — QueryOperation closed enum of typed *Query payloads
├── pattern.rs    — re-exports nota_codec::PatternField
├── schema.rs     — KindDecl, FieldDecl, Cardinality, KindDeclQuery
├── diagnostic.rs — Diagnostic, DiagnosticLevel, DiagnosticSite (incl. OperationInBatch),
│                    DiagnosticSuggestion, Applicability
├── slot.rs       — Slot, Revision (NotaTransparent u64 newtypes)
├── hash.rs       — Hash (BLAKE3 32-byte alias)
└── flow.rs       — Node, Edge, Graph (with paired *Query types via NexusPattern),
                    Ok, RelationKind (NotaEnum)
```

## Status

**Working core.** Wire envelope + per-verb typed payloads +
flow-graph kinds + KindDecl all defined and exercised. Round-
trip tests (18) cover Frame end-to-end across every verb shape
including patterns and typed Records replies.

## Cross-cutting context

- Project-wide architecture:
  [criome/ARCHITECTURE.md](https://github.com/LiGoldragon/criome/blob/main/ARCHITECTURE.md)
- The text-translator daemon at the boundary:
  [nexus/ARCHITECTURE.md](https://github.com/LiGoldragon/nexus/blob/main/ARCHITECTURE.md)
