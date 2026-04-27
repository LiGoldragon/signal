//! `PatternField<T>` — a slot in a `*Query` record kind.
//!
//! Each field of a `*Query` type is a `PatternField<T>` where `T` is
//! the corresponding field type on the data kind. A pattern field is
//! one of:
//!
//! - `Wildcard` — match any value (`_` in nexus text)
//! - `Bind(name)` — match any value and capture as `@name`
//! - `Match(value)` — match the literal value of type `T`
//!
//! The `*Query` record kinds are paired with their data kinds —
//! `NodeQuery` with `Node`, `EdgeQuery` with `Edge`, etc — and
//! generated alongside them by rsc from the same `KindDecl`. M0
//! hand-writes the projection.

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde::{Deserialize, Serialize};

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum PatternField<T> {
    Wildcard,
    Bind(String),
    Match(T),
}
