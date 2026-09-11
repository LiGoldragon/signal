# signal

The shared Signal layer. Every component depends on this repository.

Signal is the messaging layer: a message is an rkyv binary archive —
typed, portable, validated on receive — and frames are length-prefixed on
the socket. Nothing else rides the wire.

This crate owns three things and nothing else:

- **The portable frame.** `Signal<T>` carries a contract value's rkyv
  bytes with the target contract in its type, and the three kinds every
  contract speaks: `Signalizable`, `ByteViewable`, `Restorable`. The six
  generated contract crates each carried a byte-identical copy of this
  before it moved here.
- **The wire framing.** One implementation of the four-byte big-endian
  length prefix and the 8 MiB body capacity, blocking and — behind the
  `transport` feature — asynchronous.
- **The shared taxonomy.** The closed component roster, authorized-object
  classification and interest, and typed reachability, generated from
  `ethos/signal.ethos`.

It owns no operations, no Nexus, no storage, and no runtime policy.
Contracts import these identities into their own Interfaces.

`ethos/signal.ethos` is the schema authority; `build.rs` checks the
checked-in Rust projection in `src/generated/signal.rs` against it.

The protocol layered on top of the rkyv archive is not decided. Nothing
here anticipates it. `DESIGN.md` carries the reasoning inherited from the
archived legacy `signal` repository; it is inheritance, not specification.

## Features

- `datom` — the Datom text projection of the taxonomy.
- `transport` — `tokio` asynchronous frame reading and writing.
