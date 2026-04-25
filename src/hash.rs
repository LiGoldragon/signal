//! `Hash` — 32-byte BLAKE3 digest used for content-addressed
//! references throughout signal.

/// Blake3 hash bytes.
pub type Hash = [u8; 32];
