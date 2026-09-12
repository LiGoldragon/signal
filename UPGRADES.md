# Upgrades

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
