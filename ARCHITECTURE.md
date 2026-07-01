# ARCHITECTURE — signal

Signal is the **sema-ecosystem's record vocabulary** carried by this
crate's local legacy wire envelope. It carries the native binary
form of the records **criome** holds in its records database: those
records are directly computer-cognizable; the bytes a record
occupies at rest *are* its meaning, no parsing, no interpretation.
Signal is that form on the wire.

> **Scope.** "criome" throughout this doc is today's `criome` daemon
> (sema-ecosystem records validator). The eventual `Criome` is the
> universal computing paradigm in Sema; in that world, signal-* as a
> separate vocabulary layer disappears because wire and state are one
> Sema substrate. Today's signal is a realization step. See
> `~/primary/ARCHITECTURE.md` §"Workspace vision and intent".

Relation sentence: `signal` is the sema / criome vocabulary
relation; front-end translators and effect daemons exchange typed
sema record operations with criome through `signal` frames, while
criome owns validation, storage authority, and the slot/revision
state those operations affect.

The wider workspace uses **signal** as the family name for typed
inter-component communication. Current component contracts use
`signal-frame` as their shared frame kernel. This older `signal` crate
still owns a local `Frame` / `Request` / `Reply` envelope for the
sema/criome vocabulary until it is cut over. Reusable pattern markers
(`Bind`, `Wildcard`, `PatternField<T>`) are imported from `signal-sema`.

Signal owns the sema-ecosystem's per-verb typed payloads —
`AssertOperation`, `MutateOperation`, `RetractOperation`,
`QueryOperation`, `Records` — plus the flow-graph kinds (`Node`,
`Edge`, `Graph`, paired `*Query` types, `RelationKind`), the
auxiliary diagnostic types, and the typed `Hash` alias.
Multi-operation atomic commits still use this crate's local
`AtomicBatch` / `BatchOperation` legacy shape. Replacing that with
the current structural multi-operation Signal shape belongs to a future
cutover.

Effect-bearing wires layered around this vocabulary — currently
signal-forge for the criome-to-forge leg and signal-arca for the
writers-to-arca-daemon leg — add relation-specific payload
vocabularies. Builder-internal churn in those layered crates does not
recompile front-end clients that depend only on `signal`.

Nexus records in NOTA syntax are the human-facing translation. The
mechanical-translation rule (every Nexus NOTA record has exactly one
signal form, and vice versa) keeps the two surfaces in lockstep.
Inside the nexus daemon, NOTA-in becomes signal-out; signal-replies
become NOTA-out.

```
text-speaking peers                  signal-speaking peers
(humans, LLM agents,                  (the nexus daemon talking
 nexus-cli, editor LSPs)              to criome — and any peer
                                       holding typed records)
        │                                       │
        │ Nexus records                         │ length-prefixed
        │ in NOTA syntax                        │ rkyv frames
        ▼                                       ▼
┌──────────────────┐                    ┌─────────────────┐
│ /tmp/nexus.sock  │                    │ /tmp/criome.sock│
│  nexus daemon    │  ──── signal ────► │     criome      │
│ (text translator)│  ◄─── signal ───── │ (validator+sema)│
└──────────────────┘                    └─────────────────┘
```

Nexus's NOTA surface is the only non-signal request surface in the
sema-ecosystem. Once a request crosses the daemon, it is signal
end-to-end.

```mermaid
flowchart LR
    core["signal local frame<br/>legacy envelope"]
    signal["signal<br/>sema/criome contract"]
    forge["signal-forge<br/>criome ↔ forge"]
    arca["signal-arca<br/>writers ↔ arca-daemon"]
    persona["signal-persona-*<br/>Persona channels"]

    signal --> core
    forge --> signal
    arca --> signal
    persona --> core
```

## Boundaries

Owns the **sema-ecosystem record vocabulary** carried by the local
legacy wire envelope:

- The **per-verb typed payloads** for the sema-ecosystem's verbs:
  `AssertOperation` / `MutateOperation` / `RetractOperation` for
  edits; `QueryOperation` for queries; `Records` for typed query
  results. Multi-op atomic commits still compose through this crate's
  local `AtomicBatch` payload. Each payload enum is closed (no generic
  wrapper).
- The **flow-graph kinds**: `Node`, `Edge`, `Graph` (with
  paired `NodeQuery` / `EdgeQuery` / `GraphQuery`), `Ok`,
  `RelationKind` (closed enum of 9 relation variants — Flow,
  DependsOn, Contains, References, Produces, Consumes, Calls,
  Implements, IsA). Encoding/decoding handled by `nota-next`
  derives — no hand-written `from_variant_name` /
  `variant_name` helpers needed. The node-kind taxonomy
  (Source / Transformer / Sink / Junction / Supervisor) belongs
  here when prism needs flow-graph records to express what each
  node *does* in the dataflow rather than only how nodes connect.
- Auxiliary types: `Diagnostic` + `DiagnosticLevel` +
  `DiagnosticSite` + `DiagnosticSuggestion`; `Hash` (32-byte
  BLAKE3 alias).
- The criome-side `Request` / `Reply` aliases over
  `crate::Request` / `crate::Reply`
  with the sema-ecosystem's payload types — `BuildRequest` is the
  next expected verb (asks criome to forward a build to forge over
  signal-forge; lands alongside forge-daemon).
- `OutcomeMessage`: `Ok` (success record kind) or `Diagnostic`
  (failure record kind).

External/shared pieces:

- `PatternField<T>` with the typed marker records `(Bind)` and
  `(Wildcard)` comes from `signal-sema`.
- The current shared component frame kernel is `signal-frame`; this
  crate does not use it yet.

Does not own:

- Nexus's NOTA record vocabulary or parser — see github.com/LiGoldragon/nexus.
- Criome's records database — owned by criome (criome.sema,
  managed through the sema library).
- Validator pipeline — owned by criome.
- Persona channel payloads — owned by `signal-persona` and the
  per-channel `signal-persona-*` contract repos.
- Runtime transport policy — owned by the daemons that use the
  contract, not by this wire crate.

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

Signal carries the project's perfect-specificity
invariant
in its concrete shape. Every verb's payload is its own closed
enum of typed kinds — `AssertOperation { Node(Node) | Edge(Edge) | … }`,
`MutateOperation { Node { slot, new, expected_rev } | … }`,
`QueryOperation { Node(NodeQuery) | … }`,
`Records { Node(Vec<Node>) | … }`. There is no shared
`KnownRecord` wrapper, no generic record envelope, no string
kind-name lookup at runtime. The wire knows what it carries by
type; consumers `match` exhaustively.

A pattern/query is itself a record kind: `NodeQuery` is paired
with `Node`, hand-written today; once `prism` lands, data and
query kinds will be projected from the same source records. The
query record carries `PatternField<T>` values using typed marker
records such as `(Bind)` and `(Wildcard)` — no parallel
"pattern" grammar exists.

No `Unknown` escape variant. The closed enum is exhaustively
closed; rebuilds bring the world forward together via the
criome self-host loop. New kinds land by adding the typed
struct + the closed-enum variant in this crate, propagating
through criome's hand-coded dispatch — schema-as-data records
are not authoritative until `prism` and a real reader exist.

This crate owns both the wire form (rkyv) and the text form (NOTA)
of its typed records. Consumers do not carry shadow types that
re-derive the text projection; round-trip witnesses for both forms
live in `tests/`.

## Wire format

This crate's local wire format is rkyv 0.8 with the canonical pinned feature set
(`default-features = false, features = ["std", "bytecheck",
"little_endian", "pointer_width_32", "unaligned"]`); 4-byte
big-endian length prefix; bytecheck validation on read.

This crate defines the typed payloads that travel inside that
wire envelope. The reader and writer both know the record kinds
because they compile against the same closed enums in this crate.

## Channel boilerplate

This crate predates the current `signal-frame` + schema-derived contract
shape. It hand-defines its local `Frame`, `Request`, and `Reply` roots and
uses `signal-derive` for record metadata. Future work should move the
criome vocabulary onto the same contract/runtime stack as the rest of the
components instead of deepening this local envelope.

## Handshake

Handshake records (`HandshakeRequest`, `HandshakeReply`,
`HandshakeRejectionReason`, `ProtocolVersion`) and the major-exact
/ minor-forward compatibility rule live in this crate's local handshake
module. Every sema-ecosystem connection opens with the local handshake:

1. Initiator sends a length-prefixed handshake frame.
2. Server validates compatibility (major-exact, minor-forward).
3. Server replies `HandshakeAccepted` or `HandshakeRejected`.
4. On accepted: subsequent frames carry the agreed protocol
   version implicitly.

The sema-ecosystem's protocol version is bumped per semver as
this crate's record vocabulary evolves.

## Reply protocol

Replies are paired to requests by **position** on the connection:
the N-th reply is for the N-th request. No correlation IDs.
Replies use typed record kinds corresponding to the request position;
the human text projection is explicit Nexus records in NOTA syntax, not
shorthand delimiter forms. Sequence-shaped replies (Query results) are atomic
at the position — never half-emitted; partial failure becomes a `Diagnostic`
*instead of* the sequence at that position.

For dependent edits where a later request needs the slot
assigned by an earlier one, the **client orchestrates** —
captures the assigned slot from the earlier reply (in its host
language) and substitutes it into the later request. Nexus has
no variables, no scoping, no cross-request state. For
parallelism, open multiple connections — each is its own serial
lane.

## Direct authoring — peer to Nexus NOTA records

Architecturally, signal is peer-shaped to Nexus records written in
NOTA syntax:

- ✓ **Programmatic Rust clients** (services, CI, the daemon itself)
  may compose typed records directly and send them as signal
  frames — no text round-trip.
- ✗ **LLM agents** author Nexus records in NOTA syntax and let
  the daemon translate. The text is the form they're trained on.
  Per Li 2026-04-25:
  *"not yet, not until llm models are trained using binary
  signal data."*

Both paths arrive at criome as signal frames.

## Code map

This crate's owned source — the sema-ecosystem record vocabulary and local
legacy envelope:

```
src/
├── lib.rs        — module entry + re-exports
├── request.rs    — Request alias + ValidateOperation
├── reply.rs      — Reply alias, OutcomeMessage, Records (typed per kind)
├── edit.rs       — AssertOperation / MutateOperation / RetractOperation
│                    (multi-op atomic commits compose via
│                    `Request` constructors)
├── query.rs      — QueryOperation closed enum of typed *Query payloads
├── diagnostic.rs — Diagnostic, DiagnosticLevel, DiagnosticSite (incl. OperationInBatch),
│                    DiagnosticSuggestion, Applicability
├── hash.rs       — Hash (BLAKE3 32-byte alias)
└── flow.rs       — Node, Edge, Graph (with paired *Query types),
                    Ok, RelationKind (nota-next codecs)
```

Local legacy envelope source — `frame.rs`, `handshake.rs`, `auth.rs`,
`slot.rs`, and `identity.rs` — still exists in this repo. `pattern.rs`
is now only a re-export of `signal-sema::PatternField`.

## Target Signal direction

This crate is a realization step. The workspace target is the shared
**Signal Protocol** — one universal mail mechanism every component
speaks — with the framing, dispatch, and route-derivation declared in
schema rather than hand-rolled per crate. When this local envelope is
cut over, the following discipline governs it. (Today's crate is the
legacy precursor; the target shape is owned by `signal-frame` and the
per-component `signal-*` contract crates.)

### Signal is binary only

Signal carries binary/rkyv only. `signal-frame` and the `signal-*` /
`meta-signal-*` contract crates carry no NOTA encoding or projection on
their request/reply types. NOTA is a separate text-edge concern that
lives outside Signal entirely, applied only by the text-translator
daemon at the boundary. A binary-only client structurally rejects NOTA
text at the wire; only DOUBLE clients (text + binary, e.g. a CLI) carry
the NOTA derive at all, and that derive lives in the consumer, not in
the contract.

### Signal is message triage

A Signal engine does admission, dispatch, identity-stamping,
validation, and wire-frame handling — nothing more. It owns no heavy
algorithmic logic or decision-making: it routes input to Nexus and
routes Nexus replies back to wire output. CLIs are thin Signal clients.
A component's CLI is the *complete* typed text edge for that component:
every operation reachable through a GUI or agent-facing client is also
reachable through the thin CLI as a signal-backed call, with multi-step
interaction carried by returned identifiers a later invocation
references rather than by a persistent REPL.

### Origin route — implicit return address

Every message carries an **origin route** as automatic metadata that is
*not* declared in the schema: a short, statistically-unique identifier
acting as a return address that travels the whole way through Signal,
then Nexus, then SEMA, and back, so a reply is associated with its
originating query when it returns. It is internal to each component and
need not be a long hash — just an echoed return address. This is the
concrete form of request-reply correlation; the wire protocol's message
data types carry correlation identity and lifecycle state
(sent / queued / processing / replied) at the data-type level rather
than through an external dispatcher table.

`Communicate` is the wire trait between any two components, over binary
rkyv with `signal-frame` (connection setup, async correlation
identifiers, handshake). A universal mail-queue manager commits intent
on accept, and the reply carries a database marker (hash plus counter)
so a client verifies the transaction and keeps local state consistent
against the daemon's authoritative database.

### Short header — the 64-bit per-message prefix

Every message carries a **short header**: a 64-bit prefix made of eight
enums (one root verb plus seven sub-enums) that discriminates any
namespace object in constant time, emitted by every signal contract.
Everything past the header is body; the header carries discriminator
information only, never payload.

- Byte 0 is the **root verb** (e.g. Help / Query / Message); bytes 1–7
  are verb-namespace sub-variants. In schema the header is a positional
  `[Variant ...]` vector: data-carrying variants take the low slots and
  bare unit variants follow, so adding a unit variant is a no-op wire
  upgrade.
- The root-verb namespace is **per-component**, not workspace-wide:
  byte 0 is each channel's own root-verb enum (up to 256 verbs), keyed
  for decoding by which channel produced the message. The namespace is
  partitioned into a `SignalCore` system-types zone (universal
  primitives) and a component-specific zone with pre-allocated sizes;
  repartitioning is a major-version event. Byte 0 may split by golden
  ratio between an owner-contract zone and a public-contract zone, with
  both contracts required to agree at compile time. `SignalCore`
  survives as a namespace concept even though the prior rename moved
  content out of it.
- Sub-enums default to one byte but pack tighter by inner type
  (`Bool` = 1 bit, `Option` = 2, a 16-variant enum = 4 bits), letting
  SEMA beat raw rkyv; freed bits and multi-byte sub-enums cover
  more than 256 variants within 64 bits. Integers are typed values,
  not sub-enum kinds.
- **Universal data variants** (`U8`, `U16`, and other small primitives)
  are pre-allocated across every signal namespace so each namespace
  inherits them — used for things like a Criome 16-bit short public-key
  identifier, generic counters, and small shared fixed-size values.
- **Version is not in the header.** The engine enforces schema version
  at the database level (one schema version per database; migration
  changes the database, not per-message tags). Per-repo `signal-X`
  versioned schema libraries are what enable cross-version decoding and
  recovery.

The wire-header pattern grows by **extension, not replacement**. The
8-byte Tier-1 micro header is the base and lives as a parseable prefix
inside longer headers; the prefix doubles as a default-on tap-anywhere
observation channel after the network-fingerprint check that precedes
zero-copy rkyv access. Three sizing tiers exist: (1) the 64-bit micro
header, (2) a fixed extended Tier-2 header for payloads needing public
keys, identities, signatures, or short descriptions (e.g. a Criome
quorum authorization payload), and (3) the full unrestricted rkyv
message.

### Schema as protocol substrate

The framing, short-header dispatch, route-enum derivation, and codec
object belong declaratively in a root-level `signal-frame.schema`
imported by component schemas, which inherit frame methods on their
`Input` / `Output` surfaces — schema-as-protocol-substrate replacing
hand-rolled transport route tables and frame codecs. The root
schema-generated signal object carries the `signal-frame` protocol
behavior for rkyv serialization and process-to-process dispatch.

Schema files live with their contract's source-of-truth, named
`<crate>/<contract-name>.schema`. Wire/signal schemas live in
`signal-<component>` as the cross-component compilation boundary Rust
sees as one canonical source, separate from daemon logic so consumers
are not rebuilt on internal changes. A daemon's configuration type
lives in the signal contract (imported for binary startup decode; a
meta-signal `Configure` wraps it), not hand-written in the daemon.

A daemon may expose more than one signal surface; configuration is just
another typed signal surface, differentiated inside the **root
enumerator** that contains the daemon's accepted signal surfaces.
Real-time streaming is a separate Signal capability (working name
`signal-real-time`) — it is Signal, not SEMA, and its storage format is
open.

### Authorization at the wire

Owner / permission distinction can live as a typed `Permission` variant
(`Owner` / `Permission` / `Unpermission`) inside the signal, with a
general socket routing by variant prefix. The current implementation
keeps the two-socket filesystem-managed approach (ordinary socket +
owner socket); the contract is generated *as if* two sockets so a
message is authenticated by the socket it arrived on, and
permission-in-variant is deferred until needed.

A **universal `Magnitude`** type in the shared typed-record crate
(`signal-core` / `signal-sema`) replaces small ordinal enums. Its eight
variants — `Zero`, `Minimum`, `VeryLow`, `Low`, `Medium`, `High`,
`VeryHigh`, `Maximum` — declare `Zero` first so derived `Ord` places
the neutral absent rung lowest (chosen over `Option`/`None`). The
fixed-byte rkyv discriminant makes the width free; each consumer picks
its own subset.

## Status

**Working core.** Wire envelope + per-verb typed payloads +
flow-graph kinds all defined and exercised. 35 tests
total — 17 wire-envelope round-trip + 18 text-format round-trip
across every verb shape, pattern, and typed `Records` reply.

## Cross-cutting context

- Project-wide architecture:
  criome/ARCHITECTURE.md
- The text-translator daemon at the boundary:
  nexus/ARCHITECTURE.md
- The current shared Signal frame kernel that future contract-shaped
  work targets when this local envelope is cut over:
  signal-frame/ARCHITECTURE.md
- Why the kernel split out of `signal`:
  `~/primary/skills/contract-repo.md` §"Kernel extraction trigger".
