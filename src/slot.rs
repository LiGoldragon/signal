//! Wire primitives for nexus messages — `Slot` and `Revision`.
//!
//! Both are u64 newtypes. `Slot` is stable identity for a record
//! across content edits; `Revision` is the global monotone
//! write-clock used for compare-and-swap on `MutateOperation` /
//! `RetractOperation`.
//!
//! Both derive `NotaTransparent` so the wire / text form is the
//! bare integer (`(Edge 100 101 Flow)`, not `(Edge (Slot 100) (Slot 101) Flow)`).
//! The wrapped `u64` is private; construction goes through
//! `Slot::from(value)` and read-out through `let n: u64 = slot.into()`.

use nota_codec::NotaTransparent;
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

/// Slot — stable identity for a record across content edits.
///
/// `MutateOperation` changes the bound content-hash without
/// changing the slot. Cross-record references travel as `Slot`,
/// not as content-hash.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Slot(u64);

/// Revision — global monotone write-clock. Each successful
/// transaction increments the revision; `expected_rev` carries
/// CAS semantics for `MutateOperation` / `RetractOperation`.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Revision(u64);
