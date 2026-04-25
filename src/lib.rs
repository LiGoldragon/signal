//! signal — the criome binary language.
//!
//! Per Li 2026-04-25: *"signal is where this goes. nexus depends
//! on signal. anything criome is signal. nexus is just a frontend
//! to it."*
//!
//! Three concentric layers, all rkyv:
//!
//! 1. **Wire envelope** — `Frame`, `Body`, `Request`, `Reply`,
//!    handshake, auth-proof, outcomes.
//! 2. **Language IR** — `RawPattern`, `RawOp`, `AssertOp`,
//!    `RawRecord`, `RawValue`, `Diagnostic`, `Slot`, `Revision`,
//!    etc. The data shapes that nexus text translates *into* and
//!    that programmatic clients author directly.
//! 3. **Sema record kinds** — `KindDecl`/`FieldSpec`/`TypeRef`
//!    (schema-of-schema), `Struct`/`Enum`/`Module`/`Program`/etc.
//!    (Rust-source records). Plus name newtypes, primitives,
//!    type expressions, origin annotations.
//!
//! All three are signal. Layer 3 absorbed from the previously-
//! separate `nexus-schema` crate (now SHELVED 2026-04-25 per Li
//! "we should probably shelf it").
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
pub mod effect;
pub mod frame;
pub mod handshake;
pub mod reply;
pub mod request;

// ─── Language IR ────────────────────────────────────────────
pub mod diagnostic;
pub mod edit;
pub mod pattern;
pub mod query;
pub mod slot;
pub mod value;

// ─── Sema record kinds (Rust-source records absorbed from
//     the former nexus-schema crate) ───────────────────────
pub mod domain;
pub mod module;
pub mod names;
pub mod origin;
pub mod primitive;
pub mod program;
pub mod ty;

// ─── Flow-graph kinds (criomed's first-milestone substrate —
//     storing architectural specs as Mermaid-like records) ───
pub mod flow;

// ─── Wire envelope re-exports ───────────────────────────────
pub use auth::AuthProof;
pub use effect::{Effect, ExecutionPlan, ExecutionStep, OkReply, QueryHitReply, RejectedReply};
pub use frame::{Body, Frame, FrameDecodeError};
pub use handshake::{
    HandshakeRejectionReason, HandshakeReply, HandshakeRequest, ProtocolVersion,
    SIGNAL_PROTOCOL_VERSION,
};
pub use reply::{Bindings, Reply, ValidateResult};
pub use request::{Request, SubscribeOp, ValidateOp};

// ─── Language IR re-exports ─────────────────────────────────
pub use diagnostic::{
    Applicability, Diagnostic, DiagnosticLevel, DiagnosticSite, DiagnosticSuggestion,
};
pub use edit::{AssertOp, MutateOp, PatchOp, RetractOp, TxnBatch, TxnOp};
pub use pattern::{FieldConstraint, RawListPattern, RawPattern};
pub use query::{
    Cursor, RawOp, RawProjField, RawProjection, RevisionRef, Selection, SortOrder,
};
pub use slot::{Revision, Slot};
pub use value::{FieldPath, RawLiteral, RawRecord, RawSegment, RawValue};

// ─── Sema record kinds re-exports ───────────────────────────
pub use domain::{Const, Enum, Field, Newtype, Struct, Variant};
pub use module::{Import, Module, Visibility};
pub use names::{
    AssociatedName, BindingName, ConstId, ConstName, EnumId, EnumName, FieldName, GenericName,
    GenericParamId, Hash, InstanceName, LiteralValue, MethodName, ModuleId, ModuleName, NewtypeId,
    NewtypeName, OriginId, ParamName, PlaceName, ProgramId, RfiName, StructId, StructName,
    TraitDeclId, TraitImplId, TraitName, TypeId, TypeName, VariantName,
};
pub use origin::Origin;
pub use primitive::Primitive;
pub use program::Program;
pub use ty::{GenericParam, TraitBound, Type, TypeApplication};

// ─── Flow-graph re-exports ──────────────────────────────────
pub use flow::{Edge, Graph, Node, KNOWN_KINDS};
