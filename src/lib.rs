//! signal — rkyv messaging schema between nexusd and criomed.
//!
//! Per [mentci-next/reports/077](https://github.com/LiGoldragon/mentci-next):
//!
//! ```text
//! nexus (text) → nexusd (translates) → signal (rkyv) → criomed
//! criomed (response) → signal → nexusd (translates) → nexus (text)
//! ```
//!
//! Signal is the rkyv form of nexus. Agents and tools may compose
//! signal frames directly without round-tripping through nexus
//! text — signal is a peer to nexus, not just its compiled form.
//!
//! Wire format: rkyv 0.8 portable feature set; the frame schema
//! is the framing (both parties know it). Per
//! `mentci-next/reports/074`.

pub mod auth;
pub mod effect;
pub mod frame;
pub mod handshake;
pub mod reply;
pub mod request;

pub use auth::AuthProof;
pub use effect::{Effect, ExecutionPlan, ExecutionStep, OkReply, QueryHitReply, RejectedReply};
pub use frame::{Frame, FrameDecodeError};
pub use handshake::{
    HandshakeRejectionReason, HandshakeReply, HandshakeRequest, ProtocolVersion,
    SIGNAL_PROTOCOL_VERSION,
};
pub use reply::{Bindings, Reply, ValidateResult};
pub use request::{Request, ValidateOp};
