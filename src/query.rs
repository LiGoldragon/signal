//! Query IR — `Selection`, operators, projection.
//!
//! A query is a `RawPattern` followed by a left-to-right operator
//! chain and an optional projection. Operators are Pascal-named
//! records (Sum, GroupBy, Limit, …) — first-class values, no
//! pipeline syntax. Operators compose by nesting: `(Top 10
//! (GroupBy @m (Sum @v)))`.
//!

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde::{Deserialize, Serialize};

use crate::hash::Hash;
use crate::pattern::RawPattern;
use crate::slot::Revision;

/// Full query selection — pattern + ops + projection.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Selection {
    pub pattern: RawPattern,
    pub operators: Vec<RawOp>,
    pub projection: Option<RawProjection>,
}

/// Query operator — applied left-to-right after the pattern.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[rkyv(serialize_bounds(
    __S: rkyv::ser::Writer + rkyv::ser::Allocator,
    __S::Error: rkyv::rancor::Source,
))]
#[rkyv(deserialize_bounds(__D::Error: rkyv::rancor::Source))]
#[rkyv(bytecheck(bounds(__C: rkyv::validation::ArchiveContext, __C::Error: rkyv::rancor::Source)))]
pub enum RawOp {
    // Aggregation
    Count,
    CountDistinct(String),
    Sum(String),
    Min(String),
    Max(String),
    Avg(String),

    // Grouping
    GroupBy {
        binds: Vec<String>,
        #[rkyv(omit_bounds)]
        inner: Vec<RawOp>,
    },
    Having(RawPattern),

    // Ordering / pagination
    OrderBy(Vec<(String, SortOrder)>),
    Limit(u64),
    Offset(u64),
    Top(u64, #[rkyv(omit_bounds)] Box<RawOp>),
    Bottom(u64, #[rkyv(omit_bounds)] Box<RawOp>),
    Before(Cursor),
    After(Cursor),

    // Set operations
    Distinct,
    DistinctBy(String),
    Project(Vec<String>),

    // Temporal (Phase 2)
    TimeAt(RevisionRef),
    TimeBetween(RevisionRef, RevisionRef),
    TimeAll,

    // Cross-instance (Phase 3 sketch)
    RemoteInstance(String),
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SortOrder {
    Asc,
    Desc,
}

/// Opaque cursor bytes for `Before`/`After` pagination.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cursor(pub Vec<u8>);

/// Refer to a database revision either by ordinal or by content
/// hash of the resulting state.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RevisionRef {
    Rev(Revision),
    Hash(Hash),
}

/// Result-shape projection — a list of fields, binds, aggregations,
/// or nested sub-projections.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[rkyv(serialize_bounds(
    __S: rkyv::ser::Writer + rkyv::ser::Allocator,
    __S::Error: rkyv::rancor::Source,
))]
#[rkyv(deserialize_bounds(__D::Error: rkyv::rancor::Source))]
#[rkyv(bytecheck(bounds(__C: rkyv::validation::ArchiveContext, __C::Error: rkyv::rancor::Source)))]
pub struct RawProjection {
    #[rkyv(omit_bounds)]
    pub fields: Vec<RawProjField>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[rkyv(serialize_bounds(
    __S: rkyv::ser::Writer + rkyv::ser::Allocator,
    __S::Error: rkyv::rancor::Source,
))]
#[rkyv(deserialize_bounds(__D::Error: rkyv::rancor::Source))]
#[rkyv(bytecheck(bounds(__C: rkyv::validation::ArchiveContext, __C::Error: rkyv::rancor::Source)))]
pub enum RawProjField {
    /// `@bind` — return a bound value.
    Bind(String),
    /// `field-name` — return a field by name.
    Field(String),
    /// `(Sum @v)` — return an aggregation result.
    Aggregation(Box<RawOp>),
    /// `{ key { sub-projection } }` — nested grouped result.
    Nested {
        key: String,
        #[rkyv(omit_bounds)]
        inner: Box<RawProjection>,
    },
}
