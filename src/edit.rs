//! Edit IR — Assert / Mutate / Retract + atomic batches.
//!
//! Three edit verbs:
//! - `Assert` introduces a new record.
//! - `Mutate` replaces an existing record (including the pattern-
//!   based form `~(\| pat \|) (NewRecord …)` becomes one
//!   MutateOp per matched record, dispatched as an AtomicBatch).
//! - `Retract` removes a record.
//!
//! Atomic batches wrap a sequence of edit ops as all-or-nothing.

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde::{Deserialize, Serialize};

use crate::slot::{Revision, Slot};
use crate::value::RawRecord;

/// Introduce a new record. Criome assigns the slot internally on
/// commit. Genesis runs the same flow as user-authored asserts —
/// no backdoor for pre-assigned slots.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AssertOp {
    pub record: RawRecord,
}

/// Whole-record replacement. Identity (slot) is preserved.
/// Pattern-driven mutation translates to one MutateOp per match
/// — `~(\| pat \|) (NewRecord …)` text becomes a sequence of
/// MutateOps inside an AtomicBatch at the daemon.
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
