# Upgrades

## 8.0.0 — protos and datom-codec 0.32.2, ethos-zero 16.0.0

The `datom` feature now binds datom-codec 0.32.2 (4dff16b4) and protos
0.32.2 (15b41da8), and the build reads `ethos/signal.ethos` with ethos-zero
16.0.0 (c2653dd8). The generated module is byte-identical, so the archived
layout of every type is unchanged.
What breaks is the trait identity under `datom`: every signal type now
implements datom-codec 0.32.2's `Datomizable` and `Composing`, and no
longer 0.31's. A crate that holds a signal type in a datomized position
and is itself on datom-codec 0.31 no longer compiles against 8.0.0, and a
crate on 0.32.2 no longer compiles against 7.0.0, since Cargo then holds
two datom-codecs whose traits do not meet.

Deploy in one step per consumer, with no compatibility path:

1. Repin `signal` to the 8.0.0 head, and in the same change `protos` and
   `datom-codec` to 0.32.2 (datom-codec with its `rkyv` feature where a
   position holds a `Decimal`, a `Meaning` or a datom-codec `Error`) and
   `ethos-zero` to 16.0.0.
2. Regenerate the consumer's ethos Rust and rebuild; its freshness test
   asserts the result. `cargo tree -d` shows one datom-codec.

Consumers, by the signal pin their main checkouts held when 8.0.0 landed:

- on 7.0.0 (66e7b153): signal-orchestrate, meta-signal-orchestrate,
  signal-ethos-zero, meta-signal-ethos-zero, signal-system,
  meta-signal-system, signal-persona, signal-mirror, and orchestrate
  through the two orchestrate contracts;
- on 5.0.0 (7bcb0949): signal-flow, meta-signal-flow, signal-message,
  meta-signal-message, signal-router, meta-signal-router, signal-mind,
  signal-criome, meta-signal-criome, signal-harness, signal-introspect,
  signal-mentci, meta-signal-mentci, meta-signal-mirror, meta-signal-persona,
  signal-repository-ledger, meta-signal-repository-ledger, and flow and
  message through them;
- on 8f9a0deb: signal-lojix, meta-signal-lojix, lojix, and horizon-lib
  through lojix's lock;
- on 48ae17b4: signal-spirit, meta-signal-spirit, signal-aggregator,
  meta-signal-aggregator, aggregator.

A consumer still on an older signal is not affected until it repins; when
it repins to 8.0.0 it takes the whole step above at once.

## 7.0.0 — the exchange layer

`signal` now owns the protocol above the archive. The contract gains
`ExchangeId`, `ContractDigest`, `Handshake`, `HandshakeReceipt`,
`HandshakeRejection`, `ExchangeFault` and `Conclusion`; the crate gains
`Dispatch<Q>`, `Delivery<R>`, `Opening<Q>`, `Answer<R>`, `Ending`,
`ExchangeLedger` and the kinds `Contracted`, `Greeted`,
`ExchangeTracking`, `ExchangeMinting` and `Exchanged`.

What this replaces, and what goes away with it: `signal-frame`'s
`ExchangeIdentifier`, `ExchangeLane`, `LaneSequence`, `SessionEpoch`,
`StreamEventIdentifier`, `SubscriptionTokenInner`, `ExchangeMode`,
`ExchangeHandshake`, `ContractId`, `WireRevision`, `ContractBinding`,
`ShortHeader`, `WireRoute`, `NonEmpty`, `Request`, `Reply`, `SubReply`
and the whole batch taxonomy. One integer and six variants stand where
ten types stood.

Every consumer adopts it at once; there is no compatibility path. A
contract crate implements `Contracted` for its `Query` root in one line,
naming the `ETHOS` constant it already exports. A client greets, opens an
exchange per query, and reads `Delivery` frames until its exchange ends.
A Nexus greets back, admits the identifiers the peer names, and answers
on them.


## 4.0.0 — the Composing derive

datom-codec 0.27.0 gives arity back to `Compositional`, which now states a
positional type's `ARITY` and builds it `from_positions`, and names the kind a
datom composes into `Composing`. Every generated type derives
`datom_codec::Composing` in place of `datom_codec::Compositional`, so
`src/generated/signal.rs` changes and every consumer regenerates.

Deploy in one step, with no compatibility path:

1. Repin `protos`, `datom-codec`, `ethos-zero` and `signal` to the heads this
   release names.
2. Rebuild: the generated module is regenerated and asserted by `build.rs`.

## 2.0.0 — the signal repository

`signal-standard` 1.0.0 became `signal` 2.0.0. The repository was renamed
on GitHub (`signal-standard` → `signal`); the old name redirects, so
existing git pins keep resolving, but a pin that names the crate must
change.

Deploy in one step, with no compatibility path:

1. Repoint the dependency:

   ```toml
   signal = { git = "https://github.com/LiGoldragon/signal", rev = "<rev>" }
   ```

   The crate is now `signal`, the library `signal`, and the Cargo `links`
   key `signal`. Replace `signal-standard = ...` and `use signal_standard::`
   with `signal = ...` and `use signal::`.

2. The six generated contract crates no longer define `Signal<T>`,
   `Signalizable`, `ByteViewable`, or `Restorable`. Import them from
   `signal` instead:

   ```rust
   use signal::{ByteViewable, Restorable, Signal, Signalizable};
   ```

   The behavior is unchanged. `Signalizable` and `Restorable<T>` are now
   blanket implementations, so a contract crate needs no code at all.

3. **The frame prefix byte order changed for Orchestrate.** Orchestrate's
   transport wrote a four-byte *little-endian* length prefix; Lojix, via
   `triad-runtime`, wrote a *big-endian* one. `signal` carries one
   big-endian implementation. Every Orchestrate peer — the Nexus and both
   CLIs — must be rebuilt and restarted together. A pre-2.0.0 CLI talking
   to a 2.0.0 Nexus, or the reverse, reads a nonsense length and either
   refuses the frame as oversized or blocks.

4. `src/bootstrap_manifest.rs` and `src/schema/` are gone. They compiled
   into nothing and nothing imported them. The
   `SIGNAL_STANDARD_UPDATE_INTERFACE_ARTIFACTS` environment variable no
   longer exists.

5. Two new Cargo features: `transport` (tokio asynchronous framing) and,
   unchanged, `datom`. Default features carry no runtime.
