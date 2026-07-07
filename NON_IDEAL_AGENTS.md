# Non-ideal agent guidance — signal

This file names required temporary debt. Treat each item as a future fix target, not as a pattern to copy.

- This crate still uses its local legacy `Frame` / `Request` / `Reply` envelope for the sema/criome vocabulary. It is current compatibility debt until the future cutover to the shared `signal-frame` and schema-derived contract/runtime stack. Do not copy or deepen the local envelope for new component contracts.
- Multi-operation atomic commits still use this crate's local legacy `AtomicBatch` / `BatchOperation` shape. Keep necessary compatibility changes local, but do not treat this shape as the pattern for new multi-operation Signal work.
