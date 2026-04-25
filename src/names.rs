//! Identifier newtypes and content-hash ID newtypes.
//!
//! Two categories of newtype over raw types:
//!
//! 1. **Name newtypes** (String-wrapping): one per naming role. A
//!    StructName is not a FieldName even if both wrap the same string.
//!    These are the human-readable references inside source code.
//!
//! 2. **ID newtypes** (Hash-wrapping): one per record kind that can be
//!    referenced by content hash. A TypeId points at a specific Type
//!    record stored in the DB; cross-record references use these for
//!    content-addressed linking.

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde::{Deserialize, Serialize};

/// Blake3 hash — the underlying bytes of any content-addressed ID.
pub type Hash = [u8; 32];

// ─── Name newtypes (String-wrapping) ────────────────────────

macro_rules! name_newtype {
    ($name:ident) => {
        #[derive(
            Archive,
            RkyvSerialize,
            RkyvDeserialize,
            Serialize,
            Deserialize,
            Debug,
            Clone,
            PartialEq,
            Eq,
            Hash,
        )]
        pub struct $name(pub String);

        impl $name {
            pub fn new(s: impl Into<String>) -> Self {
                Self(s.into())
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
    };
}

// Module-level entity names
name_newtype!(ModuleName);
name_newtype!(EnumName);
name_newtype!(StructName);
name_newtype!(NewtypeName);
name_newtype!(ConstName);
name_newtype!(RfiName);
name_newtype!(TraitName);

// Sub-entity names
name_newtype!(VariantName);
name_newtype!(FieldName);
name_newtype!(MethodName);

// Local / body names
name_newtype!(InstanceName);
name_newtype!(ParamName);
name_newtype!(BindingName);

// Origin places (place-based lifetimes)
name_newtype!(PlaceName);

// Type reference — any type in position (enum, struct, newtype,
// trait, primitive). The resolver determines the kind.
name_newtype!(TypeName);

// Generic parameter (introduced by $X)
name_newtype!(GenericName);

// Associated type name (trait members)
name_newtype!(AssociatedName);

// ─── ID newtypes (Hash-wrapping) ────────────────────────────

macro_rules! id_newtype {
    ($name:ident) => {
        #[derive(
            Archive,
            RkyvSerialize,
            RkyvDeserialize,
            Serialize,
            Deserialize,
            Debug,
            Clone,
            Copy,
            PartialEq,
            Eq,
            Hash,
        )]
        pub struct $name(pub Hash);

        impl $name {
            pub fn new(hash: Hash) -> Self {
                Self(hash)
            }

            pub fn as_bytes(&self) -> &Hash {
                &self.0
            }
        }
    };
}

// Type-system IDs — used to break recursive cycles in type expressions.
id_newtype!(TypeId);
id_newtype!(GenericParamId);
id_newtype!(OriginId);

// Top-level declaration IDs — used when one declaration references
// another by content hash (stable across re-declaration).
id_newtype!(EnumId);
id_newtype!(StructId);
id_newtype!(NewtypeId);
id_newtype!(ConstId);
id_newtype!(TraitDeclId);
id_newtype!(TraitImplId);
id_newtype!(ModuleId);
id_newtype!(ProgramId);

// ─── Literal values ─────────────────────────────────────────

/// Literal value for Const declarations and literal patterns.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum LiteralValue {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Char(u32),
    Unit,
}
