//! signal — the criome binary language.
//!
//! Per Li 2026-04-25: *"signal is where this goes. nexus depends
//! on signal. anything criome is signal. nexus is just a frontend
//! to it."*
//!
//! Two layers, all rkyv:
//!
//! 1. **Wire envelope** — `Frame`, `Body`, `Request`, `Reply`,
//!    handshake, auth-proof, outcomes.
//! 2. **Language IR** — `RawPattern`, `RawOp`, `AssertOp`,
//!    `RawRecord`, `RawValue`, `Diagnostic`, `Slot`, `Revision`,
//!    `Hash`. The shapes nexus text translates *into* and that
//!    programmatic clients author directly.
//!
//! Plus the **flow-graph kinds** (`Node`, `Edge`, `Graph`) — the
//! v0.0.1 sema record category criomed handles end-to-end. Per
//! Li 2026-04-25: *"first criomed usage to be for storing
//! specification as flow-graphs ... so we can start designing
//! architecture in sema."*
//!
//! ```text
//! nexus (text) → nexus daemon (translates) → signal (rkyv) → criome
//! criome (response) → signal → nexus daemon (translates) → nexus (text)
//! ```
//!
//! Wire format: rkyv 0.8 portable feature set; the frame schema
//! is the framing (both parties know it). Per
//! `mentci/reports/074`.

// ─── Wire envelope ──────────────────────────────────────────
pub mod auth;
pub mod frame;
pub mod handshake;
pub mod reply;
pub mod request;

// ─── Language IR ────────────────────────────────────────────
pub mod diagnostic;
pub mod edit;
pub mod hash;
pub mod pattern;
pub mod query;
pub mod slot;
pub mod value;

// ─── Flow-graph kinds (criomed's first-milestone substrate) ──
pub mod flow;

// ─── Wire envelope re-exports ───────────────────────────────
pub use auth::AuthProof;
pub use frame::{Body, Frame, FrameDecodeError};
pub use handshake::{
    HandshakeRejectionReason, HandshakeReply, HandshakeRequest, ProtocolVersion, SIGNAL_PROTOCOL_VERSION,
};
pub use reply::{Bindings, OutcomeMessage, Reply};
pub use request::{Request, ValidateOp};

// ─── Language IR re-exports ─────────────────────────────────
pub use diagnostic::{Applicability, Diagnostic, DiagnosticLevel, DiagnosticSite, DiagnosticSuggestion};
pub use edit::{AssertOp, AtomicBatch, BatchOp, MutateOp, RetractOp};
pub use hash::Hash;
pub use pattern::{FieldConstraint, RawListPattern, RawPattern};
pub use query::{Cursor, RawOp, RawProjField, RawProjection, RevisionRef, Selection, SortOrder};
pub use slot::{Revision, Slot};
pub use value::{FieldPath, RawLiteral, RawRecord, RawSegment, RawValue};

// ─── Flow-graph re-exports ──────────────────────────────────
pub use flow::{Edge, Graph, Node, Ok, KNOWN_KINDS};
