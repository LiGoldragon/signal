//! `PatternField<T>` — re-exported from `nota-codec`.
//!
//! The shape lives in nota-codec because the codec needs to
//! pattern-match it during pattern-record encoding/decoding;
//! see [reports/099 §4.4](https://github.com/LiGoldragon/mentci/blob/main/reports/099-custom-derive-design-2026-04-27.md)
//! for the layering rationale.
//!
//! Variants: `Wildcard` (`_` in nexus text), `Bind` (`@<schema-
//! field-name>`), `Match(value)`. The bind name is implicit from
//! the field's position in the surrounding `*Query` record — the
//! IR carries no string.

pub use nota_codec::PatternField;
