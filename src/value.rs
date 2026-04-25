//! Wire-record values — what travels inside Assert/Mutate/Patch
//! ops and Query replies before kind-resolution at criome.
//!
//! `RawRecord` is the signal-side form: a kind name plus field
//! name/value pairs. Criomed resolves `kind_name` against a
//! `KindDecl` in sema during validation step 1 (schema-check).
//! `RawValue` is the recursive value tree; `RawLiteral` is the
//! leaf literal type.
//!
//! Per [reports/070 §6.6](mentci-next/reports/070-nexus-language-and-contract.md).

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde::{Deserialize, Serialize};

use crate::hash::Hash;
use crate::slot::{Revision, Slot};

/// Signal-side record. `kind_name` is unresolved on the signal —
/// criome looks it up against `KindDecl` at validation time. This
/// keeps the contract schema-evolution-friendly (adding a new kind
/// does not change the signal format).
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[rkyv(serialize_bounds(
    __S: rkyv::ser::Writer + rkyv::ser::Allocator,
    __S::Error: rkyv::rancor::Source,
))]
#[rkyv(deserialize_bounds(__D::Error: rkyv::rancor::Source))]
#[rkyv(bytecheck(bounds(__C: rkyv::validation::ArchiveContext, __C::Error: rkyv::rancor::Source)))]
pub struct RawRecord {
    pub kind_name: String,
    #[rkyv(omit_bounds)]
    pub fields: Vec<(String, RawValue)>,
}

/// Recursive value — leaf literal, slot-ref, list, nested record,
/// or raw bytes. Bare integers in nexus text become `SlotRef` when
/// the target field is `Slot`-typed; this is resolved by criome
/// at parse-time mapping (nexus) given schema knowledge.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[rkyv(serialize_bounds(
    __S: rkyv::ser::Writer + rkyv::ser::Allocator,
    __S::Error: rkyv::rancor::Source,
))]
#[rkyv(deserialize_bounds(__D::Error: rkyv::rancor::Source))]
#[rkyv(bytecheck(bounds(__C: rkyv::validation::ArchiveContext, __C::Error: rkyv::rancor::Source)))]
pub enum RawValue {
    Lit(RawLiteral),
    SlotRef(Slot),
    List(#[rkyv(omit_bounds)] Vec<RawValue>),
    Record(#[rkyv(omit_bounds)] Box<RawRecord>),
    Bytes(Vec<u8>),
}

/// Leaf literal types carried on signal. `Slot` and `Revision`
/// appear here for explicit literal forms; structural references
/// use `RawValue::SlotRef`.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum RawLiteral {
    U64(u64),
    I64(i64),
    F64(f64),
    Bool(bool),
    String(String),
    Bytes(Vec<u8>),
    Blake3(Hash),
    Slot(Slot),
    Revision(Revision),
}

/// One step in a path inside a record. `Field` for struct fields,
/// `Index` for list positions, `Variant` for sum-type selection.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum RawSegment {
    Field(String),
    Index(u32),
    Variant(String),
}

/// Bind path inside a `RawPattern` — either a top-level field or
/// a nested traversal.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum FieldPath {
    Direct(String),
    Nested(Vec<RawSegment>),
}
