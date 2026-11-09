# signal

The shared Signal layer. Every component depends on this repository.

Signal is the messaging layer: a message is an rkyv binary archive —
typed, portable, validated on receive — and frames are length-prefixed on
the socket. Nothing else rides the wire.

This crate owns four things and nothing else:

- **The portable frame.** `Signal<T>` carries a contract value's rkyv
  bytes with the target contract in its type, and the three kinds every
  contract speaks: `Signalizable`, `ByteViewable`, `Restorable`. The six
  generated contract crates each carried a byte-identical copy of this
  before it moved here.
- **The wire framing.** One implementation of the four-byte big-endian
  length prefix and the 8 MiB body capacity, over blocking byte streams.
  Asynchronous framing over `tokio` streams is the `signal-transport`
  crate, so a contract crate builds signal without `tokio`.
- **The shared taxonomy.** The closed component roster, authorized-object
  classification and interest, and typed reachability, generated from
  `ethos/signal.ethos`.
- **The exchange layer.** A contract greeting and concurrent positive-numbered
  exchanges attribute every query, answer and ending on one connection.

It owns no operations, no Nexus, no storage, and no runtime policy.
Contracts import these identities into their own Interfaces.

`ethos/signal.ethos` is the schema authority; `build.rs` checks the
checked-in Rust projection in `src/generated/signal.rs` against it.

The v7 exchange layer is the implemented protocol above the archive.
`DESIGN.md` carries the reasoning inherited from the archived legacy `signal`
repository; it is inheritance, not specification.

## One signal per build graph

`Cargo.toml` declares `links = "signal"`, so Cargo admits one release of
this crate per dependency graph: every contract in a process shares one
`Signal<T>`, one framing, and one exchange and greeting layer, and two
contracts' frames pass through the same transport (two copies of the
crate would make two incompatible `Signal` types for one wire). The cost
is that a crate cannot build two releases of a signal consumer in one
graph, so a cross-release test lives in a `<repo>-test` repository.

## Features

- `datom` — the Datom text projection of the taxonomy.
