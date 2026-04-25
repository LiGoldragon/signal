//! Data-definition entities: enum, struct, newtype, const.
//!
//! A *domain* is any data definition. Each top-level declaration is a
//! sealed record addressed by content hash in the DB.
//!
//! No inline nested declarations — Rust doesn't have them. A nested
//! type in source is just a separately-declared type that happens to
//! be referenced inside another declaration; sema stores it as its
//! own record.

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde::{Deserialize, Serialize};

use crate::module::Visibility;
use crate::names::{
    ConstName, EnumName, FieldName, GenericParamId, LiteralValue, NewtypeName, StructName,
    TypeId, VariantName,
};

// ── Enum ─────────────────────────────────────────────────────

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Enum {
    pub name: EnumName,
    pub visibility: Visibility,
    pub generics: Vec<GenericParamId>,
    pub variants: Vec<Variant>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Variant {
    /// `Fire` — no payload.
    Bare { name: VariantName },
    /// `Circle(F64)` — single typed payload.
    Data {
        name: VariantName,
        payload: TypeId,
    },
    /// `Rectangle { Width: F64, Height: F64 }` — struct-like variant.
    Struct {
        name: VariantName,
        fields: Vec<Field>,
    },
}

// ── Struct ───────────────────────────────────────────────────

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Struct {
    pub name: StructName,
    pub visibility: Visibility,
    pub generics: Vec<GenericParamId>,
    pub fields: Vec<Field>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Field {
    /// Named field with explicit type: `name: Type`.
    Typed {
        name: FieldName,
        visibility: Visibility,
        type_: TypeId,
    },
    /// Field whose name is also its type — collapses
    /// `name: Name` to a single `Name` token at declaration.
    SelfTyped {
        name: FieldName,
        visibility: Visibility,
    },
}

// ── Newtype ──────────────────────────────────────────────────

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Newtype {
    pub name: NewtypeName,
    pub visibility: Visibility,
    pub generics: Vec<GenericParamId>,
    pub wraps: TypeId,
}

// ── Const ────────────────────────────────────────────────────

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Const {
    pub name: ConstName,
    pub visibility: Visibility,
    pub type_: TypeId,
    pub value: LiteralValue,
}
