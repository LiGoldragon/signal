//! Per-Principal configuration index.
//!
//! `Tweaks` ties together a Principal's preferences — their
//! Theme, Layout, KeybindMap, and any other personal records.
//! When a mentci session opens for a Principal, mentci-lib reads
//! that Principal's Tweaks record and resolves the referenced
//! preference records.
//!
//! Editing a Tweak — picking a different Theme, changing a
//! keybind — is the same Mutate flow as editing any record. The
//! change appears in the wire pane, lands in the change log,
//! and is recursively introspectable.

use nota_codec::{NexusPattern, NotaRecord, PatternField};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

use crate::slot::Slot;

/// Per-Principal index of style/configuration records.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tweaks {
    /// Whose Tweaks these are.
    pub principal: Slot,
    /// Active Theme record.
    pub theme: Slot,
    /// Active Layout record.
    pub layout: Slot,
    /// Active KeybindMap record.
    pub keybinds: Slot,
}

/// Paired query kind for Tweaks.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, NexusPattern, Debug, Clone, Copy)]
pub struct TweaksQuery {
    pub principal: PatternField<Slot>,
    pub theme: PatternField<Slot>,
    pub layout: PatternField<Slot>,
    pub keybinds: PatternField<Slot>,
}
