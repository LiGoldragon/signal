//! `Reply` — what criome sends back.
//!
//! Replies are paired to requests by **position** on the connection
//! (FIFO; no correlation IDs). Reply *form* mirrors request form
//! and reuses the request-side sigil discipline at the wire level.
//!
//! Per-position reply shapes:
//! - Single edit (Assert/Mutate/Retract): a single `OutcomeMessage`.
//! - Multi-element edit (Mutate-with-pattern, AtomicBatch): a
//!   `Vec<OutcomeMessage>`, paired by index to the affected items.
//! - Query: a `Vec<RawRecord>` of matching records.
//! - Subscribe: connection enters streaming mode; each event is a
//!   record arriving on the connection (not a Reply variant — see
//!   `Event` below for the M2+ shape).
//!
//! Failure at any reply position is a `Diagnostic` record.

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

use crate::diagnostic::Diagnostic;
use crate::flow::Ok;
use crate::handshake::{HandshakeRejectionReason, HandshakeReply};
use crate::value::{RawRecord, RawValue};

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
pub enum Reply {
    // ─── Handshake ───────────────────────────────────────────
    HandshakeAccepted(HandshakeReply),
    HandshakeRejected(HandshakeRejectionReason),

    // ─── Edit replies ────────────────────────────────────────
    /// Single-element edit reply: `(Ok)` on success or
    /// `(Diagnostic …)` on failure.
    Outcome(OutcomeMessage),
    /// Multi-element edit reply: per-item outcomes paired by
    /// position to the input ops.
    Outcomes(Vec<OutcomeMessage>),

    // ─── Query reply ─────────────────────────────────────────
    /// The matching records (empty `Vec` for zero matches).
    Records(Vec<RawRecord>),
}

/// Per-position outcome — either success acknowledgement or a
/// failure diagnostic. Wire forms: `(Ok)` for success,
/// `(Diagnostic …)` for failure.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
pub enum OutcomeMessage {
    Ok(Ok),
    Diagnostic(Diagnostic),
}

/// Bindings — for query results that carry pattern binds, one per
/// match.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
pub struct Bindings(pub Vec<(String, RawValue)>);
