//! Authentication proof carried on every Frame after handshake.
//!
//! - **MVP**: `SingleOperator` — accepted only when the client
//!   connects via a Unix-socket peer-cred check that already
//!   established trust at the OS layer. No cryptographic proof
//!   on the wire.
//! - **Post-MVP**: `BlsSig` (single-principal cryptographic
//!   signature) and `QuorumProof` (federated quorum proof
//!   referring to a `CommittedMutation` record). Skeletons here.
//!
//! Per `mentci-next/reports/070 §6.1`.

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

use crate::Slot;

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum AuthProof {
    /// Trust established at the OS layer via SO_PEERCRED. No
    /// cryptographic proof. The MVP single-operator default.
    SingleOperator,

    /// Single-principal BLS signature. `signer` slot points at
    /// a `Principal` record in sema.
    BlsSig { sig: BlsG1, signer: Slot },

    /// Federated quorum proof. `committed` slot points at a
    /// `CommittedMutation` record that carries the aggregated
    /// quorum signature.
    QuorumProof { committed: Slot },
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlsG1(pub [u8; 48]);
