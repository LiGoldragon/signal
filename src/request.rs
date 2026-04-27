//! `Request` — what a signal-speaking client (the nexus daemon, a
//! programmatic Rust client, future criome peers) sends to criome.
//!
//! After the handshake, every Frame body is `Body::Request(...)`
//! or `Body::Reply(...)`.
//!
//! Connection lifecycle is socket-level, not Request-level: there
//! is no Goodbye, Cancel, Resume, Heartbeat, or Unsubscribe verb.
//! Closing the socket ends the connection; subscriptions die with
//! their connection.

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

use crate::edit::{AssertOp, AtomicBatch, BatchOp, MutateOp, RetractOp};
use crate::handshake::HandshakeRequest;
use crate::query::QueryOp;

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
pub enum Request {
    /// MUST be the first request on a new connection. See
    /// [`crate::handshake`].
    Handshake(HandshakeRequest),

    // ─── Edit ────────────────────────────────────────────────
    Assert(AssertOp),
    Mutate(MutateOp),
    Retract(RetractOp),
    AtomicBatch(AtomicBatch),

    // ─── Query ───────────────────────────────────────────────
    Query(QueryOp),
    /// Open a subscription on this connection. Streams matching
    /// events going forward (no initial snapshot — issue a Query
    /// first if you want current state). One subscription per
    /// connection.
    Subscribe(QueryOp),

    // ─── Read-only ───────────────────────────────────────────
    Validate(ValidateOp),
}

/// Dry-run a single op or batch through the validator pipeline.
/// Returns the would-be `OutcomeMessage` (or sequence of them).
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
pub struct ValidateOp {
    pub op: Box<BatchOp>,
}
