//! KeybindMap record — gesture/key → action mapping.
//!
//! Per-Principal mapping from input (key combinations,
//! modifier-decorated mouse buttons) to abstract `ActionToken`
//! names. Shells maintain a built-in mapping from
//! ActionToken → native gesture; the user's KeybindMap
//! overrides per-action when present.

use nota_codec::{NexusPattern, NotaEnum, NotaRecord, PatternField};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

/// Map of input → action.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeybindMap {
    pub display_name: String,
    pub bindings: Vec<KeybindEntry>,
}

/// One binding.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeybindEntry {
    /// Input description as a string ("Cmd+S",
    /// "Shift+LeftClick", "Backspace"). Concrete grammar
    /// lands when the binding parser is wired.
    pub input: String,
    pub action: ActionToken,
}

/// Abstract action names the workbench understands. New
/// actions added here are added to every shell's binding
/// dispatch.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActionToken {
    /// Toggle the wire pane.
    ToggleWirePane,
    /// Toggle the tweaks editor.
    ToggleTweaksPane,
    /// Pause / resume the wire stream.
    PauseWire,
    ResumeWire,
    /// Cancel the active constructor flow.
    CancelFlow,
    /// Commit the active constructor flow.
    CommitFlow,
    /// Pin the focused slot.
    PinFocused,
    /// Unpin the focused slot.
    UnpinFocused,
    /// Clear all diagnostics.
    ClearDiagnostics,
    /// Begin renaming the focused slot.
    BeginRename,
    /// Initiate retract on the focused slot.
    RequestRetract,
}

/// Paired query.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, NexusPattern, Debug, Clone)]
pub struct KeybindMapQuery {
    pub display_name: PatternField<String>,
}
