//! Edit IR — Assert/Mutate/Retract/Patch + atomic batches.
//!
//! Five edit verbs: `Assert` introduces a record at a slot,
//! `Mutate` replaces a record whole, `Retract` removes a record,
//! `Patch` does field-level edits, `TxnBatch` wraps a sequence
//! atomically. CAS via `expected_rev`. Forward-refs inside a
//! transaction are not resolved (split into two transactions —
//!//!

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde::{Deserialize, Serialize};

use crate::slot::{Revision, Slot};
use crate::value::{RawRecord, RawSegment, RawValue};

/// Introduce a new record at a slot. `assigned_slot = Some(_)`
/// during genesis; otherwise criome mints a slot.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AssertOp {
    pub record: RawRecord,
    pub assigned_slot: Option<Slot>,
    /// CAS for slot non-existence: `Some(0)` means "fail if any
    /// record currently bound at this slot".
    pub expected_rev: Option<Revision>,
}

/// Whole-record replacement at an existing slot. Slot identity is
/// preserved — subscriptions see a `SubMutate` event.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MutateOp {
    pub slot: Slot,
    pub new_record: RawRecord,
    pub expected_rev: Option<Revision>,
}

/// Remove the record bound at a slot. Validator checks for
/// outstanding slot-refs and rejects with a Diagnostic naming
/// dependents if any.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RetractOp {
    pub slot: Slot,
    pub expected_rev: Option<Revision>,
}

/// Surgical field-level edit at a path inside a record.
/// Type-checked against the field's `TypeRef` at validation.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PatchOp {
    pub slot: Slot,
    pub field_path: Vec<RawSegment>,
    pub new_value: RawValue,
    pub expected_rev: Option<Revision>,
}

/// Atomic envelope wrapping a sequence of edit ops. All-or-nothing;
/// one Revision; one redb transaction. The first error halts the
/// batch; the reply names the failed op index.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TxnBatch {
    pub ops: Vec<TxnOp>,
}

/// One op inside a `TxnBatch`.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum TxnOp {
    Assert(AssertOp),
    Mutate(MutateOp),
    Retract(RetractOp),
    Patch(PatchOp),
}
