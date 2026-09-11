//! The shared Signal layer.
//!
//! Signal is the messaging layer: a message is an rkyv binary archive,
//! typed, portable, validated on receive, length-prefixed on the socket.
//! This crate owns the three pieces every component shares — the portable
//! [`Signal`] frame and its kinds, the wire framing that carries it, and
//! the cross-component taxonomy generated from `ethos/signal.ethos`.
//!
//! The protocol layered on top of the rkyv archive is not decided; nothing
//! here anticipates it.

pub mod frame;
pub mod generated;
pub mod portable;
pub mod taxonomy;

#[cfg(feature = "transport")]
pub mod transport;

pub use frame::*;
pub use generated::*;
pub use portable::*;
pub use taxonomy::*;

#[cfg(feature = "transport")]
pub use transport::*;

/// The authored Ethos source of the shared taxonomy.
pub const STANDARD_SIGNAL_SOURCE: &str = include_str!("../ethos/signal.ethos");
/// The Rust projection generated from [`STANDARD_SIGNAL_SOURCE`].
pub const STANDARD_SIGNAL_RUST: &str = include_str!("generated/signal.rs");
