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
use signal_derive::Schema;

use crate::identity::Principal;
use crate::keybind::KeybindMap;
use crate::layout::Layout;
use crate::slot::Slot;
use crate::style::Theme;

/// Per-Principal index of style/configuration records.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Schema, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tweaks {
    /// Whose Tweaks these are.
    pub principal: Slot<Principal>,
    /// Active Theme record.
    pub theme: Slot<Theme>,
    /// Active Layout record.
    pub layout: Slot<Layout>,
    /// Active KeybindMap record.
    pub keybinds: Slot<KeybindMap>,
}

/// Paired query kind for Tweaks.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NexusPattern, Debug, Clone)]
#[nota(queries = "Tweaks")]
pub struct TweaksQuery {
    pub principal: PatternField<Slot<Principal>>,
    pub theme: PatternField<Slot<Theme>>,
    pub layout: PatternField<Slot<Layout>>,
    pub keybinds: PatternField<Slot<KeybindMap>>,
}
