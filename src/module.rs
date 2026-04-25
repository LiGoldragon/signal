//! Module containers, imports, and visibility.
//!
//! A Module references its declarations by content hash so that two
//! Module records that contain the same set of declarations hash to
//! the same value.

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde::{Deserialize, Serialize};

use crate::names::{
    ConstId, EnumId, ModuleName, NewtypeId, StructId, TraitDeclId, TraitImplId, TypeName,
};

/// Five-level visibility matching Rust's `pub` / `pub(crate)` /
/// `pub(super)` / `pub(in path)` / private.
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
pub enum Visibility {
    Public,
    Crate,
    Super,
    InPath(Vec<ModuleName>),
    Private,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Import {
    pub source: ModuleName,
    pub names: Vec<TypeName>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Module {
    pub name: ModuleName,
    pub imports: Vec<Import>,
    pub enums: Vec<EnumId>,
    pub structs: Vec<StructId>,
    pub newtypes: Vec<NewtypeId>,
    pub consts: Vec<ConstId>,
    pub trait_decls: Vec<TraitDeclId>,
    pub trait_impls: Vec<TraitImplId>,
}
