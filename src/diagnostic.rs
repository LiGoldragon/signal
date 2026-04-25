//! Diagnostic IR — structured failure reports.
//!
//! Diagnostics are both transient (returned in a `Reply::Rejected`
//! at the signal layer) and durable (asserted as `Diagnostic`
//! records in sema). The `durable_record` slot flags which.
//!
//! Diagnostic codes are E0000 – E9999 by failure class:
//! parse / schema / ref / invariant / unauthorised /
//! expired-proposal / incomplete-quorum / invalid-signature /
//! cascade.
//!
//! Per [reports/070 §4.1, §6.5](mentci/reports/070-nexus-language-and-contract.md).

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde::{Deserialize, Serialize};

use crate::slot::Slot;

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
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

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiagnosticLevel {
    Error,
    Warning,
    Info,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DiagnosticSite {
    /// A specific record location.
    Slot(Slot),
    /// A span in nexus source text.
    SourceSpan { offset: u32, length: u32, source: String },
    /// An op within a `TxnBatch`.
    OpInBatch(u32),
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DiagnosticSuggestion {
    pub applicability: Applicability,
    pub replacement_text: String,
    pub site: Option<DiagnosticSite>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Applicability {
    MachineApplicable,
    MaybeIncorrect,
    HasPlaceholders,
}
