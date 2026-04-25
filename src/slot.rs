//! Wire primitives for nexus messages — `Slot` and `Revision`.
//!
//! These are u64 newtypes used at the language and signal layer.
//! Sema binds slots to content-addressed records via
//! `SlotBinding`; revisions are the global write-clock.
//!
//! Per [reports/070 §6.6](mentci-next/reports/070-nexus-language-and-contract.md).
//! When `criome-types` lands, these move there and nexus-schema
//! re-exports.

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde::{Deserialize, Serialize};

/// Slot — stable identity for a record across content edits.
///
/// `Mutate` and `Patch` change the bound content-hash without
/// changing the slot. Cross-record references travel as `Slot`,
/// not as content-hash.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Slot(pub u64);

/// Revision — global monotone write-clock. Each successful
/// transaction increments the revision; `expected_rev` carries
/// CAS semantics for `Mutate`/`Retract`/`Patch`.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Revision(pub u64);
