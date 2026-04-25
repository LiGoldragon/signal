//! Type expressions — references, applications, generic parameters.
//!
//! Recursive references (Type inside Type, GenericParam inside Type)
//! go through content-hash IDs (TypeId, GenericParamId). This keeps
//! each type record sealed and makes the shape a DAG rather than a
//! cycle.

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde::{Deserialize, Serialize};

use crate::names::{
    AssociatedName, FieldName, GenericName, GenericParamId, OriginId, TraitName, TypeId, TypeName,
};

/// A type expression.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Type {
    /// `&T`, optionally with an origin and a view-field restriction.
    Borrowed {
        origin: Option<OriginId>,
        view_fields: Vec<FieldName>,
        inner: TypeApplication,
    },

    /// `~&T` (mutable borrow).
    MutBorrowed {
        origin: Option<OriginId>,
        view_fields: Vec<FieldName>,
        inner: TypeApplication,
    },

    /// Applied type constructor, e.g. `Vec<T>`, `Option<T>`.
    Applied(TypeApplication),

    /// Reference to a generic parameter in scope.
    GenericParamRef(GenericParamId),

    /// A bare type reference by name. The resolver looks up the
    /// declaration record in the current module / scope.
    Named(TypeName),

    /// Associated type on `Self`, e.g. `Self::Item`.
    SelfAssoc { name: AssociatedName },

    /// Raw const pointer `*const T`.
    RawConst(TypeApplication),

    /// Raw mut pointer `*mut T`.
    RawMut(TypeApplication),

    /// Function pointer type, e.g. `fn(u32) -> u32`.
    FnPtr {
        params: Vec<TypeId>,
        return_type: Option<TypeId>,
    },
}

/// `Constructor<Args...>` — a type constructor applied to arguments.
/// Args are content-hash IDs pointing at Type records.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TypeApplication {
    pub constructor: TypeName,
    pub args: Vec<TypeId>,
}

/// A generic parameter: type parameter with bounds, or const parameter.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum GenericParam {
    /// Type parameter: `$Value` or `$Value{Bound}`.
    Type {
        name: GenericName,
        bounds: Vec<TraitBound>,
    },
    /// Const parameter: `const N: Usize` in Rust. Type referenced by
    /// content hash.
    Const {
        name: GenericName,
        type_: TypeId,
    },
}

/// A trait bound: `Trait<Args...>`. Args are Type IDs.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TraitBound {
    pub trait_name: TraitName,
    pub args: Vec<TypeId>,
}
