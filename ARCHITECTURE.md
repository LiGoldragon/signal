# signal — architecture

## Center

`signal` is the one repository every component depends on. It holds what
every signal needs in common: the portable rkyv frame, the framing that
carries it on a socket, and the vocabulary in which components recognize
one another.

The source is meant to be read and thought through, not hidden behind a
compiler. The Ethos spelling of the taxonomy is the human, agent,
harness, and GUI surface; the Rust projection carries only encoded
identities.

## The portable frame — `src/portable.rs`

`Signal<T>` is a contract value's rkyv bytes with the target contract
carried in its type. `Signalizable` forms one, `ByteViewable` exposes its
peer-wire bytes, and `Restorable<T>` validates and restores the value.
Each kind is implemented once, generically: any rkyv-serializable type is
`Signalizable`, and any `Signal<T>` whose `T` is archived and checkable is
`Restorable<T>`. A contract crate declares its types and inherits all
three.

## The framing — `src/frame.rs`, `src/transport.rs`

A Signal frame is a four-byte big-endian length prefix followed by that
many body bytes, with a `FrameCapacity` — 8 MiB by default — refusing
anything larger before a body is allocated. `Framable` prefixes peer-wire
bytes; `FrameReading` and `FrameWriting` carry whole frames over blocking
byte streams; `AsyncFrameReading` and `AsyncFrameWriting`, behind the
`transport` feature, carry them over `tokio` streams.

Byte order is big-endian. Before this crate held the framing, Orchestrate
hand-rolled a little-endian prefix in its transport while Lojix took a
big-endian one from `triad-runtime`; the two were silently incompatible.
Big-endian is kept, following `triad-runtime`, the legacy signal
architecture document, and network byte order. `triad-runtime` keeps its
own streaming path.

## The taxonomy — `ethos/signal.ethos`, `src/generated/`, `src/taxonomy.rs`

- `ComponentKind`: one closed roster partitioned into Core, Messaging,
  Interaction, Platform, and Aggregate zones with room for local growth.
- `AuthorizedObjectKind` and `AuthorizedObjectInterest`: the shared
  four-rung narrowing lattice.
- `Differentiator`, `AuthorizedObjectReference`, and
  `ComponentClassification`: the common classification shapes.
- `StandardSocket`: typed local and network reachability.

Only genuine cross-component standards belong here. There are no
operation roots, Nexus actors, runtime policy, or storage.

## The exchange layer — `ethos/signal.ethos`, `src/exchange.rs`, `src/accord.rs`

Above the archive, the layer that says which question an answer answers.

A connection is greeted once. The greeting carries the digest of the
contract's authored Ethos source, computed in a `const fn` so a
contract's identity is fixed when it is compiled. Two peers agree
exactly when their sources agree; a mismatch is a typed refusal and
never a negotiation.

After the greeting a connection carries any number of concurrent
exchanges. The querying side mints each exchange's identifier and every
frame the answering side sends names the exchange it belongs to.
`Dispatch<Q>` is what the querying side sends — `Greet`, `Open`,
`Abandon`; `Delivery<R>` is what the answering side sends — `Greeted`,
`Answer`, `End`. Both are generic over the contract's root, so one
archive carries the envelope and the contract value together.

Whether an exchange takes one answer or goes on answering is not on the
wire: both sides know the contract. A subscription is an exchange that
goes on answering, its first answer the state on open. A contract whose
exchange streams must send that first answer even when the state is
empty, or a peer cannot tell an empty state from a state not yet sent.

`ExchangeLedger` is one connection's exchange state — greeted or not,
and which identifiers are open — bounded so a peer-driven table cannot
grow without limit. `ExchangeFault` is the closed vocabulary of what
goes wrong at this layer, distinct from a contract's own refusals, which
are ordinary response variants. A fault the answering side cannot
attribute to an exchange is reported against `CONNECTION_EXCHANGE`.

## Authority and projection

`ethos/signal.ethos` is the canonical schema source. `build.rs` generates
from it with `ethos-zero` and asserts the result equals the checked-in
`src/generated/signal.rs`, so the projection cannot drift from its source.
`ethos/interface.ethos` records the same identities as a role-free strict
`Interface.{1 0 0}`; it is kept as authored Ethos and has no Rust
projection.

## Evolution

Adding a real component inserts it into the appropriate reserved zone. Do
not append blindly or repartition zones; repartitioning is a major-version
event. Schema decisions must remain independent of Rust, LLVM, and the
current OS.

The protocol on top of the rkyv archive is to be decided. `DESIGN.md`
carries, verbatim, the four sections of the archived legacy `signal`
repository's architecture document that are worth inheriting. They are
inherited reasoning, not specification.
