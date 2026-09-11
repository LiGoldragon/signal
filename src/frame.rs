//! The Signal wire frame: a four-byte big-endian length prefix and a body.
//!
//! One implementation, shared by every Nexus. Before this crate held it,
//! Orchestrate hand-rolled the prefix in its transport and Lojix took it
//! from `triad-runtime`'s `LengthPrefixedCodec`; the two disagreed on byte
//! order. Big-endian is kept, following `triad-runtime` and the legacy
//! signal architecture document.

use std::io::{Read, Write};

use thiserror::Error;

use crate::portable::ByteViewable;

/// The width of the length prefix that opens every Signal frame.
pub const FRAME_PREFIX_BYTES: usize = 4;

/// The greatest body a Signal frame carries by default: 8 MiB.
pub const MAXIMUM_SIGNAL_BYTES: usize = 8 * 1024 * 1024;

/// The greatest number of body bytes one frame may carry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FrameCapacity {
    bytes: usize,
}

/// The bytes of one Signal frame body.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FrameBody {
    bytes: Vec<u8>,
}

/// The four-byte big-endian prefix that opens a Signal frame.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FramePrefix {
    bytes: [u8; FRAME_PREFIX_BYTES],
}

#[derive(Debug, Error)]
pub enum FrameError {
    #[error("Signal frame I/O failed: {0}")]
    Io(#[from] std::io::Error),
    #[error("Signal frame body of {found} bytes exceeds the {capacity} byte capacity")]
    BodyTooLarge { found: usize, capacity: usize },
}

impl Default for FrameCapacity {
    fn default() -> Self {
        Self {
            bytes: MAXIMUM_SIGNAL_BYTES,
        }
    }
}

impl From<usize> for FrameCapacity {
    fn from(bytes: usize) -> Self {
        Self { bytes }
    }
}

impl From<Vec<u8>> for FrameBody {
    fn from(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }
}

impl From<FrameBody> for Vec<u8> {
    fn from(body: FrameBody) -> Self {
        body.bytes
    }
}

impl From<[u8; FRAME_PREFIX_BYTES]> for FramePrefix {
    fn from(bytes: [u8; FRAME_PREFIX_BYTES]) -> Self {
        Self { bytes }
    }
}

impl ByteViewable for FrameBody {
    fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

/// A capacity says how much it holds and refuses what it does not admit.
pub trait Capacious {
    fn capacity_bytes(&self) -> usize;

    fn admits(&self, length: usize) -> bool {
        length <= self.capacity_bytes()
    }

    fn admit(&self, length: usize) -> Result<usize, FrameError> {
        if self.admits(length) {
            Ok(length)
        } else {
            Err(FrameError::BodyTooLarge {
                found: length,
                capacity: self.capacity_bytes(),
            })
        }
    }
}

impl Capacious for FrameCapacity {
    fn capacity_bytes(&self) -> usize {
        self.bytes
    }
}

/// A prefix declares how many body bytes follow it.
pub trait LengthDeclaring {
    fn declared_length(&self) -> usize;

    fn admitted_length(&self, capacity: FrameCapacity) -> Result<usize, FrameError> {
        capacity.admit(self.declared_length())
    }
}

impl LengthDeclaring for FramePrefix {
    fn declared_length(&self) -> usize {
        u32::from_be_bytes(self.bytes) as usize
    }
}

/// Peer-wire bytes can be prefixed into a complete Signal frame.
pub trait Framable {
    fn framed(&self, capacity: FrameCapacity) -> Result<Vec<u8>, FrameError>;
}

impl<T: ByteViewable> Framable for T {
    fn framed(&self, capacity: FrameCapacity) -> Result<Vec<u8>, FrameError> {
        let body = self.bytes();
        capacity.admit(body.len())?;
        let length = u32::try_from(body.len()).map_err(|_| FrameError::BodyTooLarge {
            found: body.len(),
            capacity: capacity.capacity_bytes(),
        })?;
        let mut frame = Vec::with_capacity(FRAME_PREFIX_BYTES + body.len());
        frame.extend_from_slice(&length.to_be_bytes());
        frame.extend_from_slice(body);
        Ok(frame)
    }
}

/// A blocking byte source yields one whole Signal frame at a time.
pub trait FrameReading {
    fn read_frame(&mut self, capacity: FrameCapacity) -> Result<FrameBody, FrameError>;
}

impl<R: Read> FrameReading for R {
    fn read_frame(&mut self, capacity: FrameCapacity) -> Result<FrameBody, FrameError> {
        let mut prefix = [0_u8; FRAME_PREFIX_BYTES];
        self.read_exact(&mut prefix)?;
        let length = FramePrefix::from(prefix).admitted_length(capacity)?;
        let mut bytes = vec![0_u8; length];
        self.read_exact(&mut bytes)?;
        Ok(FrameBody::from(bytes))
    }
}

/// A blocking byte sink accepts one whole Signal frame at a time.
pub trait FrameWriting {
    fn write_frame(
        &mut self,
        body: &impl Framable,
        capacity: FrameCapacity,
    ) -> Result<(), FrameError>;
}

impl<W: Write> FrameWriting for W {
    fn write_frame(
        &mut self,
        body: &impl Framable,
        capacity: FrameCapacity,
    ) -> Result<(), FrameError> {
        self.write_all(&body.framed(capacity)?)?;
        self.flush()?;
        Ok(())
    }
}
