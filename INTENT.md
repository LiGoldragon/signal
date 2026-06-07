# INTENT — signal

*The sema-ecosystem's record vocabulary with a local legacy wire envelope.
Carries the native binary form of the records `criome` holds in its records
database. Companion to `ARCHITECTURE.md` and `Cargo.toml`. Maintenance:
`primary/skills/repo-intent.md`.*

## Repo-scope only

This file carries only the intent that is FOR this `signal` crate.
Workspace-shape intent stays in the primary workspace `primary/INTENT.md`.
Current shared frame-kernel intent lives in `signal-frame`; Persona channel
intent stays in the `signal-persona-*` contracts. This crate still owns its
local legacy `Frame` / `Request` / `Reply` envelope until a future cutover.

## Today and eventually

Throughout this repo, "criome" is today's `criome` daemon — the
sema-ecosystem records validator. The eventual `Criome` is the universal
computing paradigm in Sema; in that world, `signal-*` as a separate
vocabulary layer disappears because wire and state are one Sema
substrate. Today's `signal` is a realization step on that path, per
`primary/ESSENCE.md` §"Today and eventually — different things, different
names". This is a scope boundary, not a license to cut corners.

## Why this repo exists

`signal` is the **sema-ecosystem's record vocabulary** carried by this crate's
local legacy signal envelope. The records it carries are directly
computer-cognizable: the bytes a record occupies at rest ARE its
meaning — no parsing, no interpretation. `signal` is that form on the
wire. Front-end translators and effect daemons exchange typed sema
record operations with criome through `signal` frames; criome owns
validation, storage authority, and the slot/revision state those
operations affect.

`signal` is the place where new typed kinds and enum variants land as the
system grows. This crate currently owns the legacy frame envelope, handshake,
and request/reply roots for that vocabulary. Reusable pattern markers live in
`signal-sema`; the current shared Signal frame kernel for component contracts is
`signal-frame`.

## What this crate owns

- The per-verb typed payloads for the sema-ecosystem's verbs:
  `AssertOperation` / `MutateOperation` / `RetractOperation` for edits,
  `QueryOperation` for queries, `Records` for typed query results.
- The flow-graph kinds: `Node`, `Edge`, `Graph` (with paired
  `*Query` types), `Ok`, and the closed `RelationKind` enum.
- Auxiliary diagnostic types and the typed `Hash` (BLAKE3 32-byte)
  alias.
- The criome-side local `Request` / `Reply` roots with the sema-ecosystem's
  payload types.

The current local frame envelope, handshake, request/reply roots, `Slot<T>`,
and `Revision` live here. `PatternField<T>`, `(Bind)`, and `(Wildcard)` are
re-exported from `signal-sema`.

## Constraints — perfect specificity at the wire

- Every verb's payload is its own closed enum of typed kinds. No shared
  `KnownRecord` wrapper, no generic record envelope, no string kind-name
  lookup at runtime. The wire knows what it carries by type; consumers
  `match` exhaustively.
- No `Unknown` escape variant. Closed enums are exhaustively closed;
  rebuilds bring the world forward together via the criome self-host
  loop. New kinds land by adding the typed struct plus the closed-enum
  variant.
- Multi-operation atomic commits are still carried by this crate's local
  `AtomicBatch` / `BatchOperation` legacy shape. Replacing that with the
  current structural multi-operation Signal shape is future migration work.
- A pattern/query is itself a record kind (`NodeQuery` paired with
  `Node`), carrying `PatternField<T>` values via typed marker records
  `(Bind)` and `(Wildcard)`. No parallel pattern grammar exists.
- The wire format is rkyv with the canonical pinned feature set; 4-byte
  big-endian length prefix; bytecheck validation on read.
- The contract owns both the wire form (rkyv) and the text form (NOTA)
  of its typed records; consumers do not carry shadow types that
  re-derive text projection. Round-trip witnesses for both forms live in
  `tests/`.

## NOTA and the text boundary

Nexus records in NOTA syntax are the human-facing translation. The
mechanical-translation rule — every Nexus NOTA record has exactly one
signal form, and vice versa — keeps the two surfaces in lockstep. NOTA is
not the inter-component wire: once a request crosses the nexus daemon, it
is signal end-to-end. Programmatic Rust clients may compose typed records
directly; LLM agents author NOTA and let the daemon translate, per the
psyche's statement that binary authoring waits *until llm models are
trained using binary signal data*.

## Non-ownership

This crate does not own:

- the current shared Signal frame kernel (`signal-frame`);
- Nexus's NOTA record vocabulary or parser (`nexus`);
- criome's records database, validator pipeline, or storage authority
  (`criome`);
- Persona channel payloads (`signal-persona` and the per-channel
  `signal-persona-*` repos);
- runtime transport policy (the daemons that use the contract).

## See also

- `ARCHITECTURE.md` — the layered family, the per-verb payloads, the
  flow-graph kinds, schema discipline, and the reply protocol.
- `../signal-frame/ARCHITECTURE.md` — the current shared Signal frame kernel
  that future contract-shaped work targets.
- `../criome/ARCHITECTURE.md` — the validator and storage authority that
  consumes these records.
- `../nexus/ARCHITECTURE.md` — the human-facing NOTA translation surface.
- `primary/skills/contract-repo.md` §"Kernel extraction trigger" — why
  the kernel split out of `signal`.
