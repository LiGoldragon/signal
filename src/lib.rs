//! The shared Signal layer.
//!
//! Signal is the messaging layer: a message is an rkyv binary archive,
//! typed, portable, validated on receive, length-prefixed on the socket.
//! This crate owns the three pieces every component shares — the portable
//! [`Signal`] frame and its kinds, the wire framing that carries it, and
//! the cross-component taxonomy generated from `ethos/signal.ethos`.
//!
//! Above the archive sits the exchange layer: a connection is greeted once,
//! settling the contract by the digest of its Ethos source, and then carries
//! any number of concurrent exchanges, each named by an identifier the
//! querying side mints. One query and one response is an exchange that ends
//! after one answer; a subscription is an exchange that goes on answering.
//! See [`exchange`] for what the layer deliberately does not carry.

pub mod accord;
pub mod exchange;
pub mod frame;
pub mod generated;
pub mod portable;
pub mod taxonomy;

#[cfg(feature = "transport")]
pub mod transport;

pub use accord::*;
pub use exchange::*;
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
