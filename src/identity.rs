//! Identity records.
//!
//! `Principal` names a participant in the engine — a human, an
//! agent, a script, a service. Capability tokens reference a
//! Principal as their subject; ChangeLogEntry records reference
//! a Principal as the actor; per-user records (Tweaks, Theme,
//! Layout, …) reference a Principal as their owner.
//!
//! For the first mentci-ui a single default Principal exists at
//! genesis (representing the local human). Multi-Principal
//! support comes when the authz model lands; the shape here
//! does not preclude it.

use nota_codec::{NexusPattern, NotaRecord, PatternField};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

/// Identifies a participant. Concrete fields are first-cut;
/// they grow as the authz model fills in.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Principal {
    /// Human-readable label for the principal — display name,
    /// not identity. Identity is the slot.
    pub display_name: String,
    /// Free-form note about this principal — purpose, scope,
    /// where they operate. Optional today; may be a typed
    /// reference in a future revision.
    pub note: String,
}

/// Paired query kind for Principal.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, NexusPattern, Debug, Clone)]
pub struct PrincipalQuery {
    pub display_name: PatternField<String>,
    pub note: PatternField<String>,
}
