//! Wire primitives for nexus messages — `Slot<T>` and `Revision`.
//!
//! `Slot<T>` is stable identity for a record across content edits,
//! phantom-typed by the kind it points at: `Slot<Node>`,
//! `Slot<Edge>`, `Slot<Principal>`, etc. The `T` is compile-time
//! only; the wire form is a bare `u64`. This carries the kind of a
//! reference inside the type system — `Edge.from: Slot<Node>`
//! tells the type checker, the schema-derive, and the human reader
//! the same thing in one place.
//!
//! `Slot<AnyKind>` is the type-erased form for the genuinely
//! heterogeneous case (a diagnostic site that may point at a record
//! of any kind, for instance). Prefer the typed form whenever the
//! kind is known statically.
//!
//! `Revision` is the global monotone write-clock used for
//! compare-and-swap on `MutateOperation` / `RetractOperation`. Not
//! kind-typed because it indexes the global write sequence, not a
//! per-kind reference.
//!
//! Wire form is the bare integer in nota / nexus text — `(Edge 100
//! 101 Flow)`, not `(Edge (Slot 100) (Slot 101) Flow)`. The wrapped
//! `u64` is private; construction goes through `Slot::from(value)`
//! and read-out through `let n: u64 = slot.into()` (or
//! `slot.value()`).

use core::marker::PhantomData;

use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode, NotaTransparent, Result};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

/// Marker for slots whose kind isn't statically known at the point
/// the slot appears — diagnostic sites, wire-tap entries pointing
/// at arbitrary records, type-erased dynamic dispatch.
///
/// Using `Slot<AnyKind>` is a deliberate signal that the kind is
/// genuinely unknowable in this position, not a workaround. Prefer
/// `Slot<SpecificKind>` when the kind IS known.
pub struct AnyKind;

/// Slot — stable identity for a record across content edits.
///
/// `MutateOperation` changes the bound content-hash without
/// changing the slot. Cross-record references travel as
/// `Slot<Kind>`, not as content-hash.
pub struct Slot<T>(u64, PhantomData<T>);

// ─── Manual impls (NotaTransparent doesn't handle generics) ───

impl<T> Slot<T> {
    /// Read out the underlying integer. Equivalent to `u64::from(slot)`.
    pub fn value(&self) -> u64 {
        self.0
    }
}

impl<T> From<u64> for Slot<T> {
    fn from(value: u64) -> Self {
        Slot(value, PhantomData)
    }
}

impl<T> From<Slot<T>> for u64 {
    fn from(slot: Slot<T>) -> Self {
        slot.0
    }
}

impl<T> Clone for Slot<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Slot<T> {}

impl<T> PartialEq for Slot<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for Slot<T> {}

impl<T> core::hash::Hash for Slot<T> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T> core::fmt::Debug for Slot<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Render as bare integer; kind is in the type, not the
        // formatted text.
        write!(f, "Slot({})", self.0)
    }
}

impl<T> NotaEncode for Slot<T> {
    fn encode(&self, encoder: &mut Encoder) -> Result<()> {
        <u64 as NotaEncode>::encode(&self.0, encoder)
    }
}

impl<T> NotaDecode for Slot<T> {
    fn decode(decoder: &mut Decoder<'_>) -> Result<Self> {
        let inner = <u64 as NotaDecode>::decode(decoder)?;
        Ok(Slot(inner, PhantomData))
    }
}

// ─── rkyv Archive — manual impl since derive doesn't see through
//      the phantom cleanly across kind types ───

#[derive(Debug)]
#[repr(transparent)]
pub struct ArchivedSlot<T>(rkyv::rend::u64_le, PhantomData<T>);

impl<T> Clone for ArchivedSlot<T> {
    fn clone(&self) -> Self {
        ArchivedSlot(self.0, PhantomData)
    }
}

impl<T> Copy for ArchivedSlot<T> {}

// SAFETY: ArchivedSlot is #[repr(transparent)] over u64_le; the
// PhantomData is zero-size and contributes no bytes. Therefore
// ArchivedSlot has the same layout as u64_le, which is itself
// Portable.
unsafe impl<T> rkyv::Portable for ArchivedSlot<T> {}

// SAFETY: ArchivedSlot is #[repr(transparent)] over u64_le, which
// is itself NoUndef (no padding, no enum discriminants).
unsafe impl<T> rkyv::traits::NoUndef for ArchivedSlot<T> {}

impl<T> Archive for Slot<T> {
    type Archived = ArchivedSlot<T>;
    type Resolver = ();

    fn resolve(&self, _resolver: Self::Resolver, out: rkyv::Place<Self::Archived>) {
        out.write(ArchivedSlot(rkyv::rend::u64_le::from_native(self.0), PhantomData));
    }
}

impl<S, T> RkyvSerialize<S> for Slot<T>
where
    S: rkyv::rancor::Fallible + ?Sized,
{
    fn serialize(&self, _serializer: &mut S) -> core::result::Result<Self::Resolver, S::Error> {
        Ok(())
    }
}

impl<D, T> RkyvDeserialize<Slot<T>, D> for ArchivedSlot<T>
where
    D: rkyv::rancor::Fallible + ?Sized,
{
    fn deserialize(&self, _deserializer: &mut D) -> core::result::Result<Slot<T>, D::Error> {
        Ok(Slot(self.0.to_native(), PhantomData))
    }
}

// rkyv bytecheck integration — ArchivedSlot is valid iff its
// inner u64_le is valid. PhantomData adds nothing to verify.
unsafe impl<T, C> rkyv::bytecheck::CheckBytes<C> for ArchivedSlot<T>
where
    C: rkyv::rancor::Fallible + ?Sized,
    rkyv::rend::u64_le: rkyv::bytecheck::CheckBytes<C>,
{
    unsafe fn check_bytes(value: *const Self, context: &mut C) -> core::result::Result<(), C::Error> {
        unsafe {
            <rkyv::rend::u64_le as rkyv::bytecheck::CheckBytes<C>>::check_bytes(
                value as *const rkyv::rend::u64_le,
                context,
            )?;
        }
        Ok(())
    }
}

/// Revision — global monotone write-clock. Each successful
/// transaction increments the revision; `expected_rev` carries
/// CAS semantics for `MutateOperation` / `RetractOperation`.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Revision(u64);
