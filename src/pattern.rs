//! Pattern IR — `(| KindName :field-constraint @bind |)` and
//! `{| ... |}` conjunctions.
//!
//! A pattern matches records of a given kind under field
//! constraints, optionally binding values to `@name`. List shapes
//! are matched by `RawListPattern`. Conjunctions join multiple
//! patterns by shared bind names (datalog-style).
//!
//! Per [reports/070 §3, §6.3](mentci/reports/070-nexus-language-and-contract.md).

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde::{Deserialize, Serialize};

use crate::value::{FieldPath, RawLiteral};

/// Pattern: a kind name plus constraints, binds, optional list
/// shape, and conjuncts.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[rkyv(serialize_bounds(
    __S: rkyv::ser::Writer + rkyv::ser::Allocator,
    __S::Error: rkyv::rancor::Source,
))]
#[rkyv(deserialize_bounds(__D::Error: rkyv::rancor::Source))]
#[rkyv(bytecheck(bounds(__C: rkyv::validation::ArchiveContext, __C::Error: rkyv::rancor::Source)))]
pub struct RawPattern {
    /// Kind name to match (resolved at validation time).
    pub kind_name: String,

    /// Field-level constraints. A constraint may be equality, a
    /// range, a string predicate, a bind, or a negation.
    pub field_constraints: Vec<(String, FieldConstraint)>,

    /// Bound names (`@x`) and the field paths they capture.
    pub binds: Vec<(String, FieldPath)>,

    /// Negated fields or sub-patterns (the `!` form).
    pub negations: Vec<String>,

    /// Optional list-shape constraint when matching a record's
    /// list-typed field.
    #[rkyv(omit_bounds)]
    pub list_pattern: Option<RawListPattern>,

    /// Sibling patterns joined by shared bind names. Implements
    /// the `{| pat1 pat2 |}` datalog-style conjunction.
    #[rkyv(omit_bounds)]
    pub conjunctions: Vec<RawPattern>,
}

/// A single field's constraint inside a pattern.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[rkyv(serialize_bounds(
    __S: rkyv::ser::Writer + rkyv::ser::Allocator,
    __S::Error: rkyv::rancor::Source,
))]
#[rkyv(deserialize_bounds(__D::Error: rkyv::rancor::Source))]
#[rkyv(bytecheck(bounds(__C: rkyv::validation::ArchiveContext, __C::Error: rkyv::rancor::Source)))]
pub enum FieldConstraint {
    Eq(RawLiteral),
    StartsWith(String),
    EndsWith(String),
    Contains(RawLiteral),
    Range {
        min: Option<RawLiteral>,
        max: Option<RawLiteral>,
    },
    /// `@name` — bind without constraining.
    Bind(String),
    /// `!constraint` — negation.
    Negate(#[rkyv(omit_bounds)] Box<FieldConstraint>),
}

/// Shape of a list field inside a pattern.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[rkyv(serialize_bounds(
    __S: rkyv::ser::Writer + rkyv::ser::Allocator,
    __S::Error: rkyv::rancor::Source,
))]
#[rkyv(deserialize_bounds(__D::Error: rkyv::rancor::Source))]
#[rkyv(bytecheck(bounds(__C: rkyv::validation::ArchiveContext, __C::Error: rkyv::rancor::Source)))]
pub enum RawListPattern {
    /// `[head | @tail]` — match head pattern, bind tail.
    HeadTail {
        #[rkyv(omit_bounds)]
        head: Box<RawPattern>,
        tail: String,
    },
    /// `[a b c]` — match positionally.
    Positional(#[rkyv(omit_bounds)] Vec<RawPattern>),
    /// `[.. p ..]` — `p` matches anywhere in the list.
    Anywhere(#[rkyv(omit_bounds)] Box<RawPattern>),
}
