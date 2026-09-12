//! The portable rkyv Signal frame and the three kinds every contract speaks.
//!
//! Moved here verbatim in behavior from the six generated contract crates,
//! which each carried a byte-identical copy, and made generic so one
//! implementation serves every contract.

use std::{marker::PhantomData, num::NonZeroUsize};

use rkyv::{
    Archive, Deserialize, Serialize,
    api::high::{HighSerializer, HighValidator},
    bytecheck::CheckBytes,
    de::Pool,
    rancor::{Error, Strategy},
    ser::allocator::ArenaHandle,
    util::AlignedVec,
    validation::{Validator, archive::ArchiveValidator, shared::SharedValidator},
};

/// The deepest subtree a Signal archive may nest before it is refused.
///
/// Cap'n Proto's figure, and for Cap'n Proto's reason: the limit exists
/// "to prevent stack overflow when handling a deeply-nested recursive type".
/// rkyv ships the same facility and leaves it off — `ArchiveValidator::new`
/// passes `None`, and `rkyv::from_bytes` builds its validator through that
/// constructor — so an archive from a peer is otherwise walked as deep as its
/// bytes ask, and a stack overflow in Rust is an abort, not an error a Nexus
/// can refuse. The ceiling is declared here, beside the byte capacity, rather
/// than in each consumer.
pub const MAXIMUM_SIGNAL_DEPTH: usize = 64;

/// The deepest nesting one archive's validation may descend.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SignalDepth {
    levels: NonZeroUsize,
}

impl Default for SignalDepth {
    fn default() -> Self {
        Self {
            levels: NonZeroUsize::new(MAXIMUM_SIGNAL_DEPTH)
                .expect("the Signal depth ceiling is not zero"),
        }
    }
}

impl From<NonZeroUsize> for SignalDepth {
    fn from(levels: NonZeroUsize) -> Self {
        Self { levels }
    }
}

/// A depth bounds the validator that reads an archive under it.
pub trait DepthBounding {
    fn levels(&self) -> NonZeroUsize;

    /// The validator `rkyv::from_bytes` would have built, with the subtree
    /// depth this bound declares instead of none at all.
    fn validator<'a>(&self, bytes: &'a [u8]) -> Validator<ArchiveValidator<'a>, SharedValidator> {
        Validator::new(
            ArchiveValidator::with_max_depth(bytes, Some(self.levels())),
            SharedValidator::new(),
        )
    }
}

impl DepthBounding for SignalDepth {
    fn levels(&self) -> NonZeroUsize {
        self.levels
    }
}

/// A portable rkyv Signal frame whose target contract is carried in its type.
pub struct Signal<T> {
    bytes: Vec<u8>,
    target: PhantomData<fn() -> T>,
}

/// Data that can form a portable Signal frame.
pub trait Signalizable: Sized {
    fn signalize(&self) -> Result<Signal<Self>, Error>;
}

/// A frame exposes its peer-wire bytes for transport framing.
pub trait ByteViewable {
    fn bytes(&self) -> &[u8];
}

/// A typed portable Signal can restore the contract value it carries.
///
/// Restoring validates the archive first, under a declared nesting ceiling.
/// An archive that nests past it is refused, not read.
pub trait Restorable<T> {
    /// Restore under the default [`SignalDepth`].
    fn restore(&self) -> Result<T, Error> {
        self.restore_within(SignalDepth::default())
    }

    /// Restore under a stated ceiling, for a peer whose contract nests
    /// deeper or shallower than the shared default.
    fn restore_within(&self, depth: SignalDepth) -> Result<T, Error>;
}

impl<T> Signalizable for T
where
    T: for<'a> Serialize<HighSerializer<AlignedVec, ArenaHandle<'a>, Error>>,
{
    fn signalize(&self) -> Result<Signal<Self>, Error> {
        Ok(Signal::from(rkyv::to_bytes::<Error>(self)?.to_vec()))
    }
}

impl<T> From<Vec<u8>> for Signal<T> {
    fn from(bytes: Vec<u8>) -> Self {
        Self {
            bytes,
            target: PhantomData,
        }
    }
}

impl<T> ByteViewable for Signal<T> {
    fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl<T> Restorable<T> for Signal<T>
where
    T: Archive,
    T::Archived:
        for<'a> CheckBytes<HighValidator<'a, Error>> + Deserialize<T, Strategy<Pool, Error>>,
{
    fn restore_within(&self, depth: SignalDepth) -> Result<T, Error> {
        // `rkyv::from_bytes` is `access` followed by `deserialize_using`, with
        // a validator built by `ArchiveValidator::new` — which declares no
        // depth at all. This is that same pair, with the ceiling declared.
        let bytes = self.bytes();
        let mut context = depth.validator(bytes);
        let archived =
            rkyv::api::access_with_context::<T::Archived, _, Error>(bytes, &mut context)?;
        let mut deserializer = Pool::default();
        rkyv::api::deserialize_using(archived, &mut deserializer)
    }
}
