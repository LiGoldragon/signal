//! [`Frame`] — the wire envelope for every signal message.
//!
//! Each Frame carries an optional principal hint (which Slot-bound
//! principal is making the request), an optional auth proof, and
//! a body (Request or Reply).
//!
//! Replies pair to requests by **position on the connection (FIFO)**;
//! there is no correlation id. The first reply on a connection
//! corresponds to the first request, the second reply to the second
//! request, and so on.
//!
//! The frame schema *is* the framing — both nexus and criome
//! know the rkyv schema, so a single `rkyv::to_bytes` /
//! `rkyv::from_bytes` per Frame covers transport.
//!
//! Wire-only: never crosses the nexus text boundary as a Frame —
//! the daemon parses verb-payloads from text and assembles them
//! into Frames internally.

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

use crate::Slot;
use crate::identity::Principal;

use crate::auth::AuthProof;
use crate::reply::Reply;
use crate::request::Request;

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
pub struct Frame {
    /// Slot-bound principal making this request. `None` is
    /// allowed during handshake and for unauthenticated probes.
    pub principal_hint: Option<Slot<Principal>>,

    /// Authentication proof. `None` only during handshake; every
    /// post-handshake frame carries one (SingleOperator MVP).
    pub auth_proof: Option<AuthProof>,

    pub body: Body,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
pub enum Body {
    Request(Request),
    Reply(Reply),
}

impl Frame {
    /// Encode to rkyv-archive bytes for socket write.
    ///
    /// rkyv 0.8 portable feature set guarantees deterministic
    /// bytes across machines (little_endian + pointer_width_32
    /// + unaligned).
    pub fn encode(&self) -> Vec<u8> {
        rkyv::to_bytes::<rkyv::rancor::Error>(self)
            .expect("rkyv serialisation does not fail for owned values")
            .to_vec()
    }

    /// Decode from rkyv-archive bytes off the socket. Validates
    /// the archive via `bytecheck` before deserialising.
    pub fn decode(bytes: &[u8]) -> Result<Self, FrameDecodeError> {
        rkyv::from_bytes::<Self, rkyv::rancor::Error>(bytes).map_err(|_| FrameDecodeError::BadArchive)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FrameDecodeError {
    #[error("rkyv archive validation failed")]
    BadArchive,
}
