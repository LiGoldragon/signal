//! `Reply` — what criome sends back.
//!
//! Single-frame replies for unary requests (Ok / Rejected /
//! QueryHit / ValidateResult). Multi-frame streams for
//! subscriptions; every stream frame shares a `subscription_id`.
//!

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

use crate::diagnostic::Diagnostic;
use crate::value::{RawRecord, RawValue};
use crate::Slot;

use crate::effect::{ExecutionPlan, OkReply, QueryHitReply, RejectedReply};
use crate::handshake::{HandshakeRejectionReason, HandshakeReply};

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
pub enum Reply {
    // ─── Handshake ───────────────────────────────────────────
    HandshakeAccepted(HandshakeReply),
    HandshakeRejected(HandshakeRejectionReason),

    // ─── Unary outcomes ──────────────────────────────────────
    Ok(OkReply),
    Rejected(RejectedReply),
    QueryHit(QueryHitReply),
    ValidateResult(ValidateResult),

    // ─── Subscription stream ─────────────────────────────────
    SubReady { subscription_id: u64 },
    SubSnapshot { subscription_id: u64, records: Vec<RawRecord> },
    SubAssert { subscription_id: u64, slot: Slot, record: RawRecord },
    SubMutate { subscription_id: u64, slot: Slot, old: RawRecord, new: RawRecord },
    SubRetract { subscription_id: u64, slot: Slot, last: RawRecord },
    SubError { subscription_id: u64, diagnostic: Diagnostic },
    SubEnd { subscription_id: u64, reason: String },

    // ─── Connection management ───────────────────────────────
    Goodbye,
}

/// Reply to a `Validate` request.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
pub struct ValidateResult {
    pub passes: bool,
    pub diagnostics: Vec<Diagnostic>,
    pub plan: Option<ExecutionPlan>,
}

/// Bindings — for query results, one per match.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
pub struct Bindings(pub Vec<(String, RawValue)>);
