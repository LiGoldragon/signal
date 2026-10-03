//! Opaque identities for data-bearing values.
//!
//! An identity is the BLAKE3 digest of a value's complete rkyv archive. It
//! deliberately carries no textual or word rendering: presentation belongs to
//! the caller's context, rather than to this shared archive boundary.

/// An opaque, fixed-width archived-value fingerprint.
#[derive(
    rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash,
)]
pub struct ArchiveFingerprint([u8; 32]);

impl ArchiveFingerprint {
    pub fn of_bytes(bytes: impl AsRef<[u8]>) -> Self {
        Self(*blake3::hash(bytes.as_ref()).as_bytes())
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

/// Gives a data-bearing value an opaque identity derived from its complete
/// archived representation.
pub trait Identifiable {
    fn identity(&self) -> Result<ArchiveFingerprint, rkyv::rancor::Error>;
}

impl<T> Identifiable for T
where
    T: for<'a> rkyv::Serialize<
        rkyv::api::high::HighSerializer<
            rkyv::util::AlignedVec,
            rkyv::ser::allocator::ArenaHandle<'a>,
            rkyv::rancor::Error,
        >,
    >,
{
    fn identity(&self) -> Result<ArchiveFingerprint, rkyv::rancor::Error> {
        rkyv::to_bytes::<rkyv::rancor::Error>(self).map(ArchiveFingerprint::of_bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::Identifiable;

    #[test]
    fn identity_fingerprints_the_complete_archived_value() {
        #[derive(Clone, rkyv::Archive, rkyv::Serialize)]
        struct RequestOccurrence {
            caller: String,
            sequence: u64,
            content: String,
        }

        let first = RequestOccurrence {
            caller: "flow-a".into(),
            sequence: 7,
            content: "inspect the receipt".into(),
        };
        let same = first.clone();
        let conflict = RequestOccurrence {
            content: "change the receipt".into(),
            ..first.clone()
        };

        assert_eq!(first.identity().unwrap(), same.identity().unwrap());
        assert_ne!(first.identity().unwrap(), conflict.identity().unwrap());
    }
}
