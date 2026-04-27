//! Wire primitives for nexus messages — `Slot` and `Revision`.
//!
//! These are u64 newtypes used at the language and signal layer.
//! Sema binds slots to content-addressed records via
//! `SlotBinding`; revisions are the global write-clock.
//!
//! When `criome-types` lands, these move there and nexus-schema
//! re-exports.

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde::{Deserialize, Serialize};

/// Slot — stable identity for a record across content edits.
///
/// `Mutate` changes the bound content-hash without changing the
/// slot. Cross-record references travel as `Slot`, not as
/// content-hash.
///
/// `#[serde(transparent)]` per the nota newtype-of-primitive
/// rule: in nexus text, a slot is a bare integer
/// (`(Edge 100 101 Flow)`), not a wrapped `(Slot 100)`. The
/// schema position determines the type. rkyv layout is
/// unaffected — the wire stays a single u64.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct Slot(pub u64);

/// Revision — global monotone write-clock. Each successful
/// transaction increments the revision; `expected_rev` carries
/// CAS semantics for `Mutate` / `Retract`.
///
/// `#[serde(transparent)]` per the nota newtype-of-primitive
/// rule: a revision is a bare integer in nexus text.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct Revision(pub u64);
