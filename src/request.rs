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

use crate::edit::{AssertOperation, AtomicBatch, BatchOperation, MutateOperation, RetractOperation};
use crate::handshake::HandshakeRequest;
use crate::query::QueryOperation;

/// Wire-only envelope. Text-bound dispatch happens at the codec
/// layer's `Decoder::next_request` (sigil + delimiter routing);
/// `Request` itself is not a `NexusVerb` because its variants
/// dispatch on different surface forms (sigils, delimiters, the
/// Handshake special-case) rather than on a uniform record-head.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
pub enum Request {
    /// MUST be the first request on a new connection. See
    /// [`crate::handshake`].
    Handshake(HandshakeRequest),

    // ─── Edit ────────────────────────────────────────────────
    Assert(AssertOperation),
    Mutate(MutateOperation),
    Retract(RetractOperation),
    AtomicBatch(AtomicBatch),

    // ─── Query ───────────────────────────────────────────────
    Query(QueryOperation),
    /// Open a subscription on this connection. Streams matching
    /// events going forward (no initial snapshot — issue a Query
    /// first if you want current state). One subscription per
    /// connection.
    Subscribe(QueryOperation),

    // ─── Read-only ───────────────────────────────────────────
    Validate(ValidateOperation),
}

/// Dry-run a single operation or batch through the validator
/// pipeline. Returns the would-be `OutcomeMessage` (or sequence
/// of them).
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
pub struct ValidateOperation {
    pub operation: Box<BatchOperation>,
}
