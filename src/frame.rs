//! [`Frame`] — the wire envelope for every signal message.
//!
//! Each Frame carries a correlation id (request/reply pairing),
//! an optional principal hint (which Slot-bound principal is
//! making the request), an optional auth proof, and a body
//! (Request or Reply).
//!
//! The frame schema *is* the framing — both nexus and criomed
//! know the rkyv schema, so a single `rkyv::to_bytes` /
//! `rkyv::from_bytes` per Frame covers transport.
//!
//! Per `mentci-next/reports/070 §6.1` and `reports/074 §7`.

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};


use nexus_schema::Slot;

use crate::auth::AuthProof;
use crate::reply::Reply;
use crate::request::Request;

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq,
)]
pub struct Frame {
    /// Request/reply pairing. Server echoes the value from a
    /// request frame onto its reply frame; subscription replies
    /// after `SubReady` use the subscription_id inside the body.
    pub correlation_id: u64,

    /// Slot-bound principal making this request. `None` is
    /// allowed during handshake and for unauthenticated probes.
    pub principal_hint: Option<Slot>,

    /// Authentication proof. `None` only during handshake; every
    /// post-handshake frame carries one (SingleOperator MVP).
    pub auth_proof: Option<AuthProof>,

    pub body: Body,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq,
)]
pub enum Body {
    Request(Request),
    Reply(Reply),
}

impl Frame {
    /// Encode to rkyv-archive bytes for socket write.
    ///
    /// rkyv 0.8 portable feature set per
    /// `mentci-next/reports/074` guarantees deterministic bytes
    /// across machines (little_endian + pointer_width_32 +
    /// unaligned).
    pub fn encode(&self) -> Vec<u8> {
        rkyv::to_bytes::<rkyv::rancor::Error>(self)
            .expect("rkyv serialisation does not fail for owned values")
            .to_vec()
    }

    /// Decode from rkyv-archive bytes off the socket. Validates
    /// the archive via `bytecheck` before deserialising.
    pub fn decode(bytes: &[u8]) -> Result<Self, FrameDecodeError> {
        rkyv::from_bytes::<Self, rkyv::rancor::Error>(bytes)
            .map_err(|_| FrameDecodeError::BadArchive)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FrameDecodeError {
    #[error("rkyv archive validation failed")]
    BadArchive,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handshake::{HandshakeRequest, ProtocolVersion, SIGNAL_PROTOCOL_VERSION};
    use crate::reply::Reply;

    #[test]
    fn handshake_request_round_trip() {
        let original = Frame {
            correlation_id: 1,
            principal_hint: None,
            auth_proof: None,
            body: Body::Request(Request::Handshake(HandshakeRequest {
                client_version: SIGNAL_PROTOCOL_VERSION,
                client_name: "nexus-cli".to_string(),
            })),
        };
        let bytes = original.encode();
        assert!(!bytes.is_empty());
        let decoded = Frame::decode(&bytes).expect("decode");
        assert_eq!(decoded, original);
    }

    #[test]
    fn goodbye_round_trip() {
        let original = Frame {
            correlation_id: 99,
            principal_hint: Some(Slot(100)),
            auth_proof: Some(AuthProof::SingleOperator),
            body: Body::Request(Request::Goodbye),
        };
        let bytes = original.encode();
        let decoded = Frame::decode(&bytes).expect("decode");
        assert_eq!(decoded, original);
    }

    #[test]
    fn protocol_version_compatibility() {
        let v0_1_0 = ProtocolVersion {
            major: 0,
            minor: 1,
            patch: 0,
        };
        let v0_2_0 = ProtocolVersion {
            major: 0,
            minor: 2,
            patch: 0,
        };
        let v1_0_0 = ProtocolVersion {
            major: 1,
            minor: 0,
            patch: 0,
        };

        // Older client / newer server: compatible.
        assert!(v0_1_0.is_compatible_with(v0_2_0));
        // Newer client / older server: not compatible.
        assert!(!v0_2_0.is_compatible_with(v0_1_0));
        // Major mismatch: not compatible.
        assert!(!v0_1_0.is_compatible_with(v1_0_0));
        assert!(!v1_0_0.is_compatible_with(v0_1_0));
        // Same: compatible.
        assert!(v0_1_0.is_compatible_with(v0_1_0));
    }

    #[test]
    fn decode_rejects_garbage() {
        let garbage = vec![0xff; 32];
        assert!(matches!(
            Frame::decode(&garbage),
            Err(FrameDecodeError::BadArchive)
        ));
    }
}
