//! Edit IR — Assert / Mutate / Retract + atomic batches.
//!
//! Three edit verbs:
//! - `Assert` introduces a new record.
//! - `Mutate` replaces an existing record (including the pattern-
//!   based form that subsumes per-field Patch).
//! - `Retract` removes a record.
//!
//! Atomic batches wrap a sequence of edit ops as all-or-nothing.

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde::{Deserialize, Serialize};

use crate::slot::{Revision, Slot};
use crate::value::RawRecord;

/// Introduce a new record. `assigned_slot = Some(_)` during
/// genesis seeding; otherwise criome assigns the slot internally.
/// CAS via `expected_rev` (`Some(0)` = "fail if any record currently
/// bound at this slot").
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AssertOp {
    pub record: RawRecord,
    pub assigned_slot: Option<Slot>,
    pub expected_rev: Option<Revision>,
}

/// Whole-record replacement. Identity (slot) is preserved.
/// Pattern-driven mutation (replace every match of a pattern with
/// a new record) is the same shape — the daemon translates
/// `~(\| pat \|) (NewRecord …)` text into one MutateOp per match.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MutateOp {
    pub slot: Slot,
    pub new_record: RawRecord,
    pub expected_rev: Option<Revision>,
}

/// Remove the record bound at a slot. Validator rejects if any
/// outstanding references would dangle.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RetractOp {
    pub slot: Slot,
    pub expected_rev: Option<Revision>,
}

/// Atomic envelope wrapping a sequence of edit ops. All-or-nothing
/// commit at one Revision in one transaction. The reply is a
/// `Vec<OutcomeMessage>` paired by index to the input ops.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AtomicBatch {
    pub ops: Vec<BatchOp>,
}

/// One op inside an `AtomicBatch`.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum BatchOp {
    Assert(AssertOp),
    Mutate(MutateOp),
    Retract(RetractOp),
}
