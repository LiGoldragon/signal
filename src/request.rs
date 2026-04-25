//! `Request` — what nexusd (or any signal-speaking client) sends
//! to criomed.
//!
//! After the handshake, every Frame body is `Body::Request(...)`
//! or `Body::Reply(...)`. Edit verbs and query verbs use payload
//! types from `nexus-schema` (the language IR is shared between
//! signal-on-wire and sema-stored shapes).
//!
//! Per `mentci-next/reports/070 §6.2`.

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};


use nexus_schema::edit::{AssertOp, MutateOp, PatchOp, RetractOp, TxnBatch, TxnOp};
use nexus_schema::query::Selection;

use crate::handshake::HandshakeRequest;

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq,
)]
pub enum Request {
    /// MUST be the first request on a new connection. See
    /// [`crate::handshake`].
    Handshake(HandshakeRequest),

    // ─── Edit ────────────────────────────────────────────────
    Assert(AssertOp),
    Mutate(MutateOp),
    Retract(RetractOp),
    Patch(PatchOp),
    TxnBatch(TxnBatch),

    // ─── Query ───────────────────────────────────────────────
    Query(Selection),
    Subscribe(SubscribeOp),
    Unsubscribe { subscription_id: u64 },

    // ─── Read-only ───────────────────────────────────────────
    Validate(ValidateOp),

    // ─── Connection management ───────────────────────────────
    /// Close the connection cleanly. Server replies with
    /// [`crate::Reply::Goodbye`] then both sides drop the socket.
    Goodbye,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq,
)]
pub struct SubscribeOp {
    pub selection: Selection,
    /// Resume from this revision; `None` → start now.
    pub from_revision: Option<nexus_schema::Revision>,
    /// If true, the server emits the current matches as a
    /// `Reply::SubSnapshot` before live diffs begin.
    pub initial_snapshot: bool,
}

/// Dry-run a single op through the validator pipeline. Returns
/// diagnostics + an optional [`crate::ExecutionPlan`].
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq,
)]
pub struct ValidateOp {
    pub op: Box<TxnOp>,
    /// Include an `ExecutionPlan` in the reply.
    pub explain: bool,
}
