//! signal — the criome binary language.
//!
//! Per Li 2026-04-25: *"signal is where this goes. nexus depends
//! on signal. anything criome is signal. nexus is just a frontend
//! to it."*
//!
//! Two layers:
//!
//! 1. **Wire envelope** — [`Frame`], [`Body`], [`Request`],
//!    [`Reply`], handshake, auth. rkyv-only — never crosses the
//!    nexus text boundary in raw form.
//! 2. **Typed records and per-verb payloads** — [`Node`], [`Edge`],
//!    [`Graph`], [`KindDecl`], the data kinds and their paired
//!    `*Query` kinds, plus `AssertOperation` / `MutateOperation` /
//!    `RetractOperation` / `QueryOperation` / `BatchOperation`
//!    closed enums. All derive both rkyv (for the wire) and the
//!    appropriate `nota-codec` derive (for nexus text).
//!
//! Per the perfect-specificity invariant
//! ([criome/ARCHITECTURE.md §2 Invariant D
//! ](https://github.com/LiGoldragon/criome/blob/main/ARCHITECTURE.md#invariant-d)):
//! every verb's payload is its own closed enum of typed kinds —
//! [`AssertOperation`], [`MutateOperation`], [`QueryOperation`],
//! [`Records`] each name exactly the kinds they operate on. No
//! generic record wrapper; no string kind-name dispatch.
//!
//! ```text
//! nexus (text) → nexus daemon (translates) → signal (rkyv) → criome
//! criome (response) → signal → nexus daemon (translates) → nexus (text)
//! ```
//!
//! Wire format: rkyv 0.8 portable feature set; the frame schema is
//! the framing (both parties know it). Discipline:
//! [tools-documentation/rust/rkyv.md](https://github.com/LiGoldragon/tools-documentation/blob/main/rust/rkyv.md).
//! Text format: nota-codec + nota-derive at the nexus dialect.

// ─── Wire envelope ──────────────────────────────────────────
pub mod auth;
pub mod frame;
pub mod handshake;
pub mod reply;
pub mod request;

// ─── Typed records and supporting types ─────────────────────
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
pub use auth::{AuthProof, BlsG1};
pub use frame::{Body, Frame, FrameDecodeError};
pub use handshake::{
    HandshakeRejectionReason, HandshakeReply, HandshakeRequest, ProtocolVersion,
    SIGNAL_PROTOCOL_VERSION,
};
pub use reply::{OutcomeMessage, Records, Reply};
pub use request::{Request, ValidateOperation};

// ─── Typed records re-exports ───────────────────────────────
pub use diagnostic::{
    Applicability, Diagnostic, DiagnosticLevel, DiagnosticSite, DiagnosticSuggestion,
};
pub use edit::{
    AssertOperation, AtomicBatch, BatchOperation, MutateOperation, RetractOperation,
};
pub use hash::Hash;
pub use pattern::PatternField;
pub use query::QueryOperation;
pub use schema::{Cardinality, FieldDecl, KindDecl, KindDeclQuery};
pub use slot::{Revision, Slot};

// ─── Flow-graph re-exports ──────────────────────────────────
pub use flow::{Edge, EdgeQuery, Graph, GraphQuery, Node, NodeQuery, Ok, RelationKind};
