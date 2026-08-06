//! Authority-verified shared cross-component vocabulary.
//!
//! `ethos/interface.ethos` is the canonical textual projection of one
//! role-free bootstrap Interface. Its Rust projection exposes only encoded
//! identities; human-readable spellings belong to Ethos and Dotos.

pub mod bootstrap_manifest;
pub mod schema;

pub use schema::lib::*;

pub const STANDARD_INTERFACE_SOURCE: &str = include_str!("../ethos/interface.ethos");
pub const STANDARD_INTERFACE_RUST: &str = include_str!("schema/lib/generated.rs");
