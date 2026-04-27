//! `KindDecl` — schema-as-data.
//!
//! The canonical record kind that *describes* other record kinds.
//! Per criome ARCHITECTURE.md Invariant D, schema lives in sema as
//! data; the closed enums in this crate are rsc's projection of
//! these records into Rust source.
//!
//! Bootstrap KindDecls (for `KindDecl` itself + `Node` + `Edge` +
//! `Graph`) ship as `genesis.nexus` and land at first-boot via the
//! same Assert path as user data. New kinds land via Assert + a
//! recompile-the-engine cycle (the criome self-host loop). M0
//! hand-edits the projection because rsc isn't ready yet.

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde::{Deserialize, Serialize};

use crate::pattern::PatternField;

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct KindDecl {
    pub name: String,
    pub fields: Vec<FieldDecl>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct FieldDecl {
    pub name: String,
    /// The Rust type name of the field as it appears in the
    /// projected source — e.g. `"String"`, `"Slot"`,
    /// `"Vec<Slot>"`, `"RelationKind"`.
    pub type_name: String,
    pub cardinality: Cardinality,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cardinality {
    One,
    Many,
    Optional,
}

/// Query for `KindDecl` records. Field-level pattern only on
/// `name`; querying inside the `Vec<FieldDecl>` would require list
/// patterns and is M1+.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct KindDeclQuery {
    pub name: PatternField<String>,
}
