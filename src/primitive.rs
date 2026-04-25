//! Built-in primitives — types that exist without module declaration.

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde::{Deserialize, Serialize};

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
pub struct Primitive {
    pub name: String,
    pub arity: u32,
}

impl Primitive {
    pub fn new(name: impl Into<String>, arity: u32) -> Self {
        Self {
            name: name.into(),
            arity,
        }
    }

    /// All built-in primitives recognized at the type level.
    pub fn all() -> Vec<Primitive> {
        [
            ("U8", 0), ("U16", 0), ("U32", 0), ("U64", 0), ("U128", 0),
            ("I8", 0), ("I16", 0), ("I32", 0), ("I64", 0), ("I128", 0),
            ("Usize", 0), ("Isize", 0),
            ("F32", 0), ("F64", 0),
            ("Bool", 0), ("String", 0), ("Char", 0),
            ("Never", 0), ("Unit", 0),
            ("Vec", 1), ("Option", 1), ("Box", 1),
            ("Result", 2),
            ("Array", 2), // Array<T, N>
        ]
        .iter()
        .map(|(n, a)| Primitive::new(*n, *a))
        .collect()
    }

    pub fn is_primitive(name: &str) -> bool {
        Self::all().iter().any(|p| p.name == name)
    }

    pub fn arity_of(name: &str) -> Option<u32> {
        Self::all().iter().find(|p| p.name == name).map(|p| p.arity)
    }
}
