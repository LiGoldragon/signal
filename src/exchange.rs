//! The exchange layer: what rides above the rkyv archive and below a contract.
//!
//! A connection is greeted once and then carries any number of concurrent
//! exchanges. An exchange is opened by the querying side, which mints its
//! [`ExchangeId`]; every frame the answering side sends names the exchange it
//! belongs to. One query and one response is an exchange that ends after one
//! answer; a subscription is an exchange that goes on answering. Nothing on
//! the wire says which — both sides know the contract, so the contract says.
//!
//! What this layer deliberately does not carry, and why:
//!
//! - **No end-of-stream flag.** Vision's Signal is "fully typed with both
//!   sides knowing the full schema, nothing on the wire labeling itself". A
//!   peer that sent `Observe` knows its exchange streams; a peer that sent
//!   `Lock` knows its exchange ends. gRPC's `END_STREAM` and Varlink's
//!   `continues` exist because their payloads are not schema-bound; ours are.
//!   [`Ending`] exists only for what the schema cannot say: the answering
//!   side dropping an exchange the peer still holds open.
//! - **No sequence numbers.** A stream socket delivers one connection's
//!   frames in order and drops none. `signal-frame`'s `LaneSequence` numbered
//!   what the transport already ordered.
//! - **No session epoch.** A reconnect is a new connection, and a
//!   subscription's own semantics — the state on open, then each change —
//!   make resumption unnecessary. `signal-frame`'s `SessionEpoch` guarded
//!   against stale replies landing on a new connection, which cannot happen
//!   when an identifier's scope is the connection that minted it.
//! - **No lane.** One contract per edge, and the querying side is the side
//!   that connected, so identifier space is owned by one end. gRPC splits
//!   stream ids odd and even because either end may open one; here only one
//!   end ever does.
//! - **No subscription token.** The exchange is the token. Abandoning the
//!   exchange, or closing the connection, is how a subscriber unsubscribes.
//! - **No cursor, demand credit or buffer bound.** A stream socket already
//!   applies backpressure, and a Nexus that must not block on a slow
//!   subscriber has exactly one thing to say that the socket cannot:
//!   that it fell behind. [`ExchangeFault::Lagged`] says it, and the
//!   recovery is to open the exchange again — which, by the subscription's
//!   own semantics, delivers the state on open. Reactive-streams credit and
//!   `signal-mind`'s `SubscriptionCursor`/`SubscriptionDemandCredit`/
//!   `SubscriptionBufferBound` are what a contract declares when the
//!   protocol will not say this one thing.
//! - **No batch.** `signal-frame` made a request a `NonEmpty` of payloads
//!   and a reply a `NonEmpty` of sub-replies; every receiver in the estate
//!   refused any length but one, and no site ever constructed anything but
//!   `Reply::committed(NonEmpty::single(SubReply::Ok(_)))`. One query opens
//!   one exchange.
//! - **No caller record.** Peer credentials taken at accept are the
//!   authority, and `signal-frame`'s own `Caller` doc called its contents
//!   advisory. A contract that wants to name a caller declares it.
//! - **No contract marker per frame.** A direct edge's contract is settled by
//!   which socket was connected to, verified once by the greeting. Only a
//!   router needs a per-frame discriminator, and that belongs to the router's
//!   own contract, which addresses a [`crate::ComponentKind`].

use crate::generated::{Conclusion, ExchangeFault, ExchangeId, Handshake, HandshakeReceipt};

/// The identifier that names the connection itself rather than an exchange on
/// it. A minted exchange is never this, so a fault the answering side cannot
/// attribute to an exchange — an unreadable frame, a missing greeting — is
/// reported against the connection.
pub const CONNECTION_EXCHANGE: ExchangeId = 0;

/// The first exchange a connection mints.
pub const FIRST_EXCHANGE: ExchangeId = 1;

/// The greatest number of exchanges one connection holds open at once.
///
/// A ceiling is declared for the reason the archive depth ceiling is: an
/// unbounded peer-driven table is an unbounded allocation, and a Nexus must
/// be able to refuse rather than grow.
pub const MAXIMUM_OPEN_EXCHANGES: usize = 1024;

/// What the querying side of a connection sends.
///
/// Generic over the contract's query root, so one archive carries the
/// envelope and the query together — there is no nested encoding.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Dispatch<Q> {
    /// Settle the contract. Sent once, before any exchange is opened.
    Greet(Handshake),
    /// Open an exchange and put a query into it.
    Open(Opening<Q>),
    /// Give up an exchange the querying side no longer wants answered.
    Abandon(ExchangeId),
}

/// A query on the exchange it opens.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Opening<Q> {
    pub exchange: ExchangeId,
    pub query: Q,
}

/// What the answering side of a connection sends.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Delivery<R> {
    /// The answer to a greeting: the contract settled, or refused.
    Greeted(HandshakeReceipt),
    /// A response on an exchange. A one-answer exchange sends one; a
    /// subscription sends the state on open and then each change.
    Answer(Answer<R>),
    /// The answering side has no more to say on this exchange, and why.
    End(Ending),
}

/// A response on the exchange it answers.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Answer<R> {
    pub exchange: ExchangeId,
    pub response: R,
}

/// The end of an exchange, as the answering side declares it.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Ending {
    pub exchange: ExchangeId,
    pub conclusion: Conclusion,
}

/// A frame of the exchange layer names the exchange it belongs to.
pub trait Exchanged {
    fn exchange(&self) -> ExchangeId;

    /// Whether this frame speaks about the connection rather than an exchange
    /// on it.
    fn is_connection_wide(&self) -> bool {
        self.exchange() == CONNECTION_EXCHANGE
    }
}

impl<Q> Exchanged for Opening<Q> {
    fn exchange(&self) -> ExchangeId {
        self.exchange
    }
}

impl<R> Exchanged for Answer<R> {
    fn exchange(&self) -> ExchangeId {
        self.exchange
    }
}

impl Exchanged for Ending {
    fn exchange(&self) -> ExchangeId {
        self.exchange
    }
}

// The exception taken here: these are constructors, not behaviour, and the
// trait they would otherwise live in would have to be implemented on
// `ExchangeId` — which is an alias for `i64`, so every integer in every
// consumer would gain the verbs. A constructor stays inherent.
impl Ending {
    /// The end of an exchange that ran to completion.
    pub fn completed(exchange: ExchangeId) -> Self {
        Self {
            exchange,
            conclusion: Conclusion::Completed,
        }
    }

    /// The end of an exchange that failed at the exchange layer.
    pub fn faulted(exchange: ExchangeId, fault: ExchangeFault) -> Self {
        Self {
            exchange,
            conclusion: Conclusion::Faulted(fault),
        }
    }

    /// A fault the answering side cannot attribute to any exchange.
    pub fn connection_faulted(fault: ExchangeFault) -> Self {
        Self::faulted(CONNECTION_EXCHANGE, fault)
    }
}
