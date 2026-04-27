//! signal — the criome binary language.
//!
//! Per Li 2026-04-25: *"signal is where this goes. nexus depends
//! on signal. anything criome is signal. nexus is just a frontend
//! to it."*
//!
//! Two layers, all rkyv:
//!
//! 1. **Wire envelope** — [`Frame`], [`Body`], [`Request`],
//!    [`Reply`], handshake, auth.
//! 2. **Typed records** — [`Node`], [`Edge`], [`Graph`],
//!    [`KindDecl`] and their paired `*Query` kinds.
//!
//! Per the perfect-specificity invariant
//! ([criome/ARCHITECTURE.md §2 Invariant D
//! ](https://github.com/LiGoldragon/criome/blob/main/ARCHITECTURE.md#invariant-d)):
//! every verb's payload is its own closed enum of typed kinds —
//! [`AssertOp`], [`MutateOp`], [`QueryOp`], [`Records`] each name
//! exactly the kinds they operate on. No generic record wrapper;
//! no string kind-name dispatch.
//!
//! ```text
//! nexus (text) → nexus daemon (translates) → signal (rkyv) → criome
//! criome (response) → signal → nexus daemon (translates) → nexus (text)
//! ```
//!
//! Wire format: rkyv 0.8 portable feature set; the frame schema is
//! the framing (both parties know it). Per
//! [mentci/reports/074](https://github.com/LiGoldragon/mentci/blob/main/reports/074-portable-rkyv-discipline.md).

// ─── Wire envelope ──────────────────────────────────────────
pub mod auth;
pub mod frame;
pub mod handshake;
pub mod reply;
pub mod request;

// ─── Typed records & supporting types ───────────────────────
pub mod diagnostic;
pub mod edit;
pub mod hash;
pub mod pattern;
pub mod query;
pub mod schema;
pub mod slot;

// ─── Flow-graph kinds (criome's first-milestone substrate) ──
pub mod flow;

// ─── Wire envelope re-exports ───────────────────────────────
pub use auth::AuthProof;
pub use frame::{Body, Frame, FrameDecodeError};
pub use handshake::{
    HandshakeRejectionReason, HandshakeReply, HandshakeRequest, ProtocolVersion,
    SIGNAL_PROTOCOL_VERSION,
};
pub use reply::{OutcomeMessage, Records, Reply};
pub use request::{Request, ValidateOp};

// ─── Typed records re-exports ───────────────────────────────
pub use diagnostic::{
    Applicability, Diagnostic, DiagnosticLevel, DiagnosticSite, DiagnosticSuggestion,
};
pub use edit::{AssertOp, AtomicBatch, BatchOp, MutateOp, RetractOp};
pub use hash::Hash;
pub use pattern::PatternField;
pub use query::QueryOp;
pub use schema::{Cardinality, FieldDecl, KindDecl, KindDeclQuery};
pub use slot::{Revision, Slot};

// ─── Flow-graph re-exports ──────────────────────────────────
pub use flow::{Edge, EdgeQuery, Graph, GraphQuery, Node, NodeQuery, Ok, RelationKind};
