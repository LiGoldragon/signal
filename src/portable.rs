//! The portable rkyv Signal frame and the three kinds every contract speaks.
//!
//! Moved here verbatim in behavior from the six generated contract crates,
//! which each carried a byte-identical copy, and made generic so one
//! implementation serves every contract.

use std::marker::PhantomData;

use rkyv::{
    Archive, Deserialize, Serialize,
    api::high::{HighSerializer, HighValidator},
    bytecheck::CheckBytes,
    de::Pool,
    rancor::{Error, Strategy},
    ser::allocator::ArenaHandle,
    util::AlignedVec,
};

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
pub trait Restorable<T> {
    fn restore(&self) -> Result<T, Error>;
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
    fn restore(&self) -> Result<T, Error> {
        rkyv::from_bytes::<T, Error>(self.bytes())
    }
}
