//! Authentication proof carried on every Frame after handshake.
//!
//! - **MVP**: `SingleOperator` — accepted only when the client
//!   connects via a Unix-socket peer-cred check that already
//!   established trust at the OS layer. No cryptographic proof
//!   on the wire.
//! - **Post-MVP**: `BlsSignature` (single-principal cryptographic
//!   signature) and `QuorumProof` (federated quorum proof
//!   referring to a `CommittedMutation` record). Skeletons here.

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

use crate::identity::Principal;
use crate::Slot;

/// Marker for the `CommittedMutation` record kind referenced by
/// `AuthProof::QuorumProof`. The full record shape lands when
/// federated quorum proofs are implemented; until then this
/// marker carries the kind in `Slot<CommittedMutation>` so the
/// type system records what the slot points at.
pub struct CommittedMutation;

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum AuthProof {
    /// Trust established at the OS layer via SO_PEERCRED. No
    /// cryptographic proof. The MVP single-operator default.
    SingleOperator,

    /// Single-principal BLS signature. `signer` slot points at
    /// a `Principal` record in sema.
    BlsSignature { signature: BlsG1, signer: Slot<Principal> },

    /// Federated quorum proof. `committed` slot points at a
    /// `CommittedMutation` record that carries the aggregated
    /// quorum signature.
    QuorumProof { committed: Slot<CommittedMutation> },
}

/// 48-byte BLS signature in G1. Wire/auth-only — never crosses
/// the nexus text boundary, so no `NotaTransparent` derive.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlsG1([u8; 48]);

impl BlsG1 {
    pub fn new(bytes: [u8; 48]) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; 48] {
        &self.0
    }
}
