//! A peer's archive is read under a declared nesting ceiling.
//!
//! rkyv's own `ArchiveValidator::new` passes `None` for the maximum subtree
//! depth, and `rkyv::from_bytes` builds its validator through that
//! constructor. A recursive archive read that way descends as far as the
//! bytes ask, and a few thousand levels overflow the native stack — which in
//! Rust is an abort, not a catchable error. These tests pin that a Signal
//! frame is refused instead.

use rkyv::{Archive, Deserialize, Serialize};
use signal::{MAXIMUM_SIGNAL_DEPTH, Restorable, Signal, Signalizable};

/// A self-reaching contract type, carrying the bounds rkyv's derive cannot
/// infer for itself. This is the shape a peer-facing recursive Signal has.
#[derive(Archive, Serialize, Deserialize, Debug, PartialEq)]
#[rkyv(
    bytecheck(bounds(__C: rkyv::validation::ArchiveContext)),
    serialize_bounds(
        __S: rkyv::ser::Writer + rkyv::ser::Allocator,
        __S::Error: rkyv::rancor::Source,
    ),
    deserialize_bounds(__D::Error: rkyv::rancor::Source),
)]
enum Nest {
    Leaf,
    Deeper(#[rkyv(omit_bounds)] Box<Nest>),
}

/// Builds a nest of exactly `levels` `Deeper` hops without recursing, so the
/// test's own construction cannot be what overflows.
trait Nesting {
    fn nested(self) -> Nest;
}

impl Nesting for usize {
    fn nested(self) -> Nest {
        let mut nest = Nest::Leaf;
        for _ in 0..self {
            nest = Nest::Deeper(Box::new(nest));
        }
        nest
    }
}

#[test]
fn a_nest_within_the_ceiling_restores() {
    let nest = 8_usize.nested();
    let frame = nest.signalize().expect("archive a shallow nest");
    assert_eq!(frame.restore().expect("restore a shallow nest"), nest);
}

#[test]
fn a_nest_past_the_ceiling_is_refused() {
    // Well past the ceiling, and small enough that building and dropping the
    // chain cannot itself exhaust the stack: the refusal must come from the
    // validator, not from the test.
    let levels = MAXIMUM_SIGNAL_DEPTH * 4;
    let frame = levels.nested().signalize().expect("archive a deep nest");
    let refusal = Restorable::<Nest>::restore(&frame);
    assert!(
        refusal.is_err(),
        "an archive nested {levels} levels deep past the {MAXIMUM_SIGNAL_DEPTH} level ceiling \
         was read instead of refused"
    );
}

#[test]
fn the_ceiling_refuses_rather_than_the_byte_capacity() {
    // The deep nest is tiny: a few bytes per level. Nothing about the frame's
    // byte capacity refuses it, so the refusal proves the depth ceiling and
    // not the pre-existing `BodyTooLarge` guard.
    let frame: Signal<Nest> = (MAXIMUM_SIGNAL_DEPTH * 4)
        .nested()
        .signalize()
        .expect("archive a deep nest");
    use signal::ByteViewable;
    assert!(
        frame.bytes().len() < signal::MAXIMUM_SIGNAL_BYTES,
        "the deep nest must sit inside the byte capacity for this test to mean anything"
    );
    assert!(Restorable::<Nest>::restore(&frame).is_err());
}
