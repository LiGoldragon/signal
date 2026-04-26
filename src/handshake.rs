//! Handshake — first message of every signal connection.
//!
//! When a client (nexus, agent, tool) connects to criome over
//! UDS, it MUST send a [`Request::Handshake`] before any other
//! request. The server replies with a [`Reply::HandshakeAccepted`]
//! or [`Reply::HandshakeRejected`].
//!
//! Compatibility: major versions must match exactly; minor
//! versions are forward-compatible (a server with newer minor
//! accepts a client with older minor); patch versions are
//! ignored.

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

use crate::Slot;

/// Current signal protocol version. Bump per semver: major for
/// incompatible wire changes, minor for additive changes, patch
/// for fixes that don't touch the wire.
pub const SIGNAL_PROTOCOL_VERSION: ProtocolVersion = ProtocolVersion { major: 0, minor: 1, patch: 0 };

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProtocolVersion {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

impl ProtocolVersion {
    /// True iff `client` is compatible with `server` per the
    /// major-exact / minor-forward rule.
    pub fn is_compatible_with(&self, server: ProtocolVersion) -> bool {
        self.major == server.major && self.minor <= server.minor
    }
}

/// First request on a new connection.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct HandshakeRequest {
    pub client_version: ProtocolVersion,
    /// Free-form client name for logs and diagnostics. Not
    /// authoritative for any decision; criome may log it.
    pub client_name: String,
}

/// Server's reply to a successful handshake.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct HandshakeReply {
    pub server_version: ProtocolVersion,
    /// criome instance identity. For multi-instance setups this
    /// is the slot of a `CriomedInstance` record in sema; for
    /// single-instance MVP it is `Slot(0)`.
    pub server_id: Slot,
}

/// Reasons the server may reject a handshake. Returned inside a
/// `Reply::HandshakeRejected`.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum HandshakeRejectionReason {
    /// Client major version does not match server major version.
    IncompatibleMajor { client: ProtocolVersion, server: ProtocolVersion },
    /// Client minor version is ahead of server minor version.
    /// (Client must downgrade or server must upgrade.)
    ClientMinorAhead { client: ProtocolVersion, server: ProtocolVersion },
    /// Server is shutting down or otherwise refusing connections.
    ServerUnavailable { detail: String },
}
