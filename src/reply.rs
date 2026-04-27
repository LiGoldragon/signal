//! `Reply` — what criome sends back. Per-position pairing (FIFO,
//! no correlation IDs).
//!
//! Per the perfect-specificity invariant, query results are typed
//! per kind via [`Records`] — a Node-targeted query returns
//! `Records::Node(Vec<Node>)`, never a heterogeneous list.
//! Consumers `match` on the `Records` variant and know the
//! element shape without further dispatch.
//!
//! Per-position reply shapes:
//! - Single edit (Assert/Mutate/Retract): one `OutcomeMessage`.
//! - Multi-element edit (AtomicBatch, mutate-with-pattern at the
//!   daemon): `Vec<OutcomeMessage>` paired by index.
//! - Query: `Records` carrying the typed result sequence.
//! - Subscribe: connection enters streaming mode; each event is
//!   a typed record arriving on the connection.
//!
//! Failure at any reply position is a `Diagnostic` record.
//!
//! Wire-only: text rendering of replies is done by the nexus
//! daemon in a custom shape (per-position dispatch on the verb
//! the reply pairs to), not via a uniform NexusVerb derive on
//! `Reply` itself.

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

use crate::diagnostic::Diagnostic;
use crate::flow::{Edge, Graph, Node, Ok};
use crate::handshake::{HandshakeRejectionReason, HandshakeReply};
use crate::schema::KindDecl;

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
    /// position to the input operations.
    Outcomes(Vec<OutcomeMessage>),

    // ─── Query reply ─────────────────────────────────────────
    /// Typed per-kind result sequence (empty `Vec` for zero
    /// matches).
    Records(Records),
}

/// Per-position outcome — either success acknowledgement or a
/// failure diagnostic. Wire forms: `(Ok)` for success,
/// `(Diagnostic …)` for failure.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
pub enum OutcomeMessage {
    Ok(Ok),
    Diagnostic(Diagnostic),
}

/// Typed per-kind query result. Each variant matches the kind
/// the query targeted; the consumer `match`es and gets a typed
/// `Vec<Kind>` directly.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
pub enum Records {
    Node(Vec<Node>),
    Edge(Vec<Edge>),
    Graph(Vec<Graph>),
    KindDecl(Vec<KindDecl>),
}
