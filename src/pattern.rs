//! `PatternField<T>` — re-exported from `signal-sema`.
//!
//! Variants: `Wildcard` (`(Wildcard)` in nota text), `Bind`
//! (`(Bind)`), `Match(value)`. Bind captures by field position;
//! the IR carries no string.

pub use signal_sema::PatternField;
