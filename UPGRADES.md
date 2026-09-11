# Upgrades

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
