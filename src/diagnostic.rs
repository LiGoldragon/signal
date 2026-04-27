//! Diagnostic IR — structured failure reports.
//!
//! Diagnostics are both transient (returned in a
//! `Reply::Outcome(OutcomeMessage::Diagnostic(...))` at the
//! signal layer) and durable (asserted as `Diagnostic` records
//! in sema). The `durable_record` slot flags which.
//!
//! Diagnostic codes are E0000 – E9999 by failure class:
//! parse / schema / ref / invariant / unauthorised /
//! expired-proposal / incomplete-quorum / invalid-signature /
//! cascade.

//! `Diagnostic`-family types are wire-only — text rendering for
//! diagnostics happens ad-hoc in the nexus daemon (per-level
//! formatting, suggestion-application UX, source-span highlight)
//! rather than through the uniform `NotaRecord` derive.
use nota_codec::NotaEnum;
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

use crate::slot::Slot;

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
pub struct Diagnostic {
    pub level: DiagnosticLevel,
    /// `E0000` – `E9999` (string for forward-compat with
    /// extension codes).
    pub code: String,
    pub message: String,
    pub primary_site: Option<DiagnosticSite>,
    pub context: Vec<(String, String)>,
    pub suggestions: Vec<DiagnosticSuggestion>,
    /// If asserted as a durable `Diagnostic` record in sema, the
    /// slot it lives at; `None` for transient-only diagnostics.
    pub durable_record: Option<Slot>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiagnosticLevel {
    Error,
    Warning,
    Info,
}

/// Where in the input a diagnostic applies. Variant shapes are
/// mixed (newtype + struct-variant + newtype-of-u32) so this
/// type is **not** derived through the codec; it's a wire-only
/// rkyv type. Diagnostic-rendering happens through the
/// `Diagnostic` record above which carries it as `Option<DiagnosticSite>`.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
pub enum DiagnosticSite {
    /// A specific record location.
    Slot(Slot),
    /// A span in nexus source text.
    SourceSpan { offset: u32, length: u32, source: String },
    /// An operation within an `AtomicBatch`, by position.
    OperationInBatch(u32),
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
pub struct DiagnosticSuggestion {
    pub applicability: Applicability,
    pub replacement_text: String,
    pub site: Option<DiagnosticSite>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Applicability {
    MachineApplicable,
    MaybeIncorrect,
    HasPlaceholders,
}
