//! Effect, OkReply, RejectedReply, QueryHitReply,
//! ExecutionPlan, ExecutionStep — the outcome shapes carried in
//! [`crate::Reply`].
//!

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

use crate::diagnostic::Diagnostic;
use crate::hash::Hash;
use crate::query::SortOrder;
use crate::value::RawSegment;
use crate::{RawOp, Revision, Slot};

use crate::reply::Bindings;

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
pub struct OkReply {
    pub revision: Revision,
    pub effects: Vec<Effect>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
pub enum Effect {
    Asserted { slot: Slot, content_hash: Hash },
    Mutated { slot: Slot, old_hash: Hash, new_hash: Hash },
    Retracted { slot: Slot, last_hash: Hash },
    Patched { slot: Slot, path: Vec<RawSegment>, new_hash: Hash },
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
pub struct RejectedReply {
    pub diagnostics: Vec<Diagnostic>,
    /// For TxnBatch failures: the index of the op that failed.
    pub failed_at_op: Option<u32>,
    /// Slots where Diagnostic records were durably asserted in
    /// sema (when criome chose to persist them).
    pub diagnostic_records: Vec<Slot>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
pub struct QueryHitReply {
    pub revision: Revision,
    pub bindings: Vec<Bindings>,
    /// For purely-aggregating queries (e.g., `(Sum @v)`), the
    /// single aggregation result.
    pub aggregation: Option<crate::value::RawValue>,
}

/// Execution plan returned by `Validate { explain: true }`.
/// Granularity is intentionally coarse — enough for editor
/// hints, not for committing to optimiser internals.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
pub struct ExecutionPlan {
    pub steps: Vec<ExecutionStep>,
    pub estimated_cost: u64,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
pub enum ExecutionStep {
    Scan { kind_name: String, estimated_count: u64 },
    Filter { constraints: Vec<String> },
    Join { with_kind: String, via_field: String },
    Aggregate { op: RawOp },
    Sort { by: Vec<(String, SortOrder)> },
    Limit(u64),
}
