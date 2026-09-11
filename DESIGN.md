# Carried design — from the legacy signal repository

Four sections of `ARCHITECTURE.md` in the legacy `signal` repository are
the only part of it worth carrying forward. They are reproduced here
verbatim and unedited. Three keep their original headings; the
paragraph under "One record, one form" carried no heading of its own in
the source, so that heading is this repository's name for it.

Source: `LiGoldragon/signal-legacy` (archived 2026-09-12; the repository
was named `signal` when these were written), `ARCHITECTURE.md` at commit
e64aef5986d2. The code they describe is superseded; the reasoning is not.
Nothing here is ruled for the current Signal layer — the protocol on top
of portable rkyv is still to be decided. Read it as inherited reasoning,
not as specification.

---

## One record, one form

Nexus records in DOTOS syntax are the human-facing translation. The
mechanical-translation rule (every Nexus DOTOS record has exactly one
signal form, and vice versa) keeps the two surfaces in lockstep.
Inside that translation boundary, DOTOS-in becomes signal-out;
signal-replies become DOTOS-out.

---

## Reply protocol

Replies are paired to requests by **position** on the connection:
the N-th reply is for the N-th request. No correlation IDs.
Replies use typed record kinds corresponding to the request position;
the human text projection is explicit Nexus records in DOTOS syntax, not
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
---

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
---

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
