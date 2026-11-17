//! The subscription stream: the frames one subscription yields, and the live
//! stream that yields them.
//!
//! A subscription is one exchange. A Signal whose output answers with a
//! stream (`Stream<State Change>` in its Ethos) generates an `Output` variant
//! holding one [`Frame`] and a capability yielding a [`Stream`]. The answering
//! side reads the stream and sends each frame as one `Delivery::Answer` of the
//! subscription's exchange; when the stream finishes, it sends
//! `Delivery::End` with the exchange's `Ending` and its `Conclusion`. Several
//! streams share a connection as exchanges do.
//!
//! The frames of one stream come in this order:
//!
//! - [`Frame::Opened`] is first, and comes once: the whole state when the
//!   subscription opens, sent even when the state is empty, so a peer can
//!   tell an empty state from a state not yet sent.
//! - [`Frame::Changed`] carries one change to that state, in the order the
//!   changes happened.
//! - [`Frame::Lagged`] carries the whole state anew, sent when the subscriber
//!   fell behind and changes were dropped; the peer replaces what it held and
//!   goes on from it. It is a recovery, not an end.
//!
//! A frame says only what an exchange does not already say: the end of a
//! stream is the end of its exchange, and is no frame.
//!
//! The state of a subscription bears [`Following`](crate::Following): each
//! change is folded into it with `Following::after`, by the answering side as
//! it publishes and by the subscriber as it receives a `Changed`.
//!
//! [`Frame`] and [`Stream`] are generic over the subscription's state and
//! change, so they are written here once, as `Vector` is in Rust; every
//! other type of the layer is declared in `ethos/signal.ethos`, and
//! `Following` in `ethos/library.ethos`. A `Frame` is
//! a wire value and bears what every generated Output bears. A `Stream` is
//! never a wire value: it lives in the answering process.

use std::pin::Pin;
use std::task::{Context, Poll};

/// One frame of a subscription stream, carried on the wire inside an Output.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Composing)
)]
pub enum Frame<State, Change> {
    /// The whole state when the subscription opens; first, and once.
    Opened(State),
    /// One change to the state.
    Changed(Change),
    /// The whole state anew, after the subscriber fell behind.
    Lagged(State),
}

/// The live stream of one subscription's frames, held by the answering side.
pub struct Stream<State, Change> {
    frames: Pin<Box<dyn futures_core::Stream<Item = Frame<State, Change>> + Send>>,
}

impl<State, Change> futures_core::Stream for Stream<State, Change> {
    type Item = Frame<State, Change>;

    fn poll_next(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.frames.as_mut().poll_next(context)
    }
}

impl<State, Change> From<Pin<Box<dyn futures_core::Stream<Item = Frame<State, Change>> + Send>>>
    for Stream<State, Change>
{
    fn from(
        frames: Pin<Box<dyn futures_core::Stream<Item = Frame<State, Change>> + Send>>,
    ) -> Self {
        Self { frames }
    }
}
