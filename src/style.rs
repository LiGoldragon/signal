//! Style records — Theme, KindStyle, RelationKindStyle.
//!
//! Themes describe **intent**, not appearance. Semantic role
//! names (`selected`, `stale`, `rejected`, `pending`, `bg`,
//! `fg`, `accent`) — each shell maps these names to its native
//! palette. The same Theme record is portable across
//! mentci-egui, mentci-iced, and mentci-flutter without
//! changes.
//!
//! KindStyle and RelationKindStyle are the per-record-kind
//! visual encoding — a Source node uses such-and-such glyph
//! and intent slot; a Flow edge uses such-and-such stroke and
//! intent slot. The shells map the glyph/stroke names to their
//! native idioms.

use nota_codec::{NexusPattern, NotaEnum, NotaRecord, PatternField};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_derive::Schema;

use crate::flow::RelationKind;

/// A semantic-intent palette.
///
/// Field shape is first-cut; the semantic-role list grows as
/// the workbench surfaces new state distinctions. Each field
/// names an *intent slot* (the meaning); shells map intent
/// slots to concrete colours.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Schema, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Theme {
    /// Display label for this theme.
    pub display_name: String,
    /// The intent slots — semantic role → token name. Tokens
    /// are abstract identifiers shells resolve to colours.
    /// Concrete shape grows as roles surface.
    pub bg_intent: IntentToken,
    pub fg_intent: IntentToken,
    pub accent_intent: IntentToken,
    pub selected_intent: IntentToken,
    pub pending_intent: IntentToken,
    pub stale_intent: IntentToken,
    pub rejected_intent: IntentToken,
}

/// A token naming an intent. Shells maintain a built-in
/// mapping from token to native colour. New tokens added here
/// are added to every shell's mapping table.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Schema, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntentToken {
    /// Use the shell's neutral background.
    NeutralBg,
    /// Use the shell's neutral foreground.
    NeutralFg,
    /// Use the shell's primary accent (selection, focus).
    PrimaryAccent,
    /// Use the shell's secondary accent.
    SecondaryAccent,
    /// "Pending" intent — work in flight, optimistic edit.
    Pending,
    /// "Stale" intent — subscription push expected; current
    /// view may be out of date.
    Stale,
    /// "Rejected" intent — last write was rejected.
    Rejected,
}

/// Per-node-kind visual encoding.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Schema, Debug, Clone, PartialEq, Eq, Hash)]
pub struct KindStyle {
    /// Which node-kind this style applies to (matches the
    /// node-kind name in the schema).
    pub kind_name: String,
    /// Glyph token (e.g. "circle-with-dot" for Source). Each
    /// shell renders the named glyph in its native graphics.
    pub glyph: GlyphToken,
    /// Intent slot for this kind's colour.
    pub intent: IntentToken,
}

/// Named glyph tokens. Shells maintain a built-in mapping. New
/// tokens are added when new visual idioms are needed.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Schema, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GlyphToken {
    /// ⊙ Source-style glyph.
    SourceCircle,
    /// ⊡ Transformer-style glyph.
    TransformerSquare,
    /// ⊠ Sink-style glyph.
    SinkSquareX,
    /// ⊕ Junction-style glyph.
    JunctionPlus,
    /// ▶ Supervisor-style glyph.
    SupervisorTriangle,
    /// Default for unknown kinds.
    Generic,
}

/// Per-RelationKind visual encoding.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Schema, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RelationKindStyle {
    pub relation: RelationKind,
    pub stroke: StrokeToken,
}

/// Named stroke tokens. Shells map to native line styles.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Schema, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StrokeToken {
    /// Solid line with open arrowhead.
    SolidOpenArrow,
    /// Dashed line with filled arrowhead.
    DashedFilledArrow,
    /// Thick line with bracket arrowhead.
    ThickBracketArrow,
    /// Thin line with dot terminator.
    ThinDot,
    /// Default fallback.
    Generic,
}

/// Paired query kinds — first cut.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NexusPattern, Debug, Clone)]
#[nota(queries = "Theme")]
pub struct ThemeQuery {
    pub display_name: PatternField<String>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NexusPattern, Debug, Clone)]
#[nota(queries = "KindStyle")]
pub struct KindStyleQuery {
    pub kind_name: PatternField<String>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NexusPattern, Debug, Clone)]
#[nota(queries = "RelationKindStyle")]
pub struct RelationKindStyleQuery {
    pub relation: PatternField<RelationKind>,
}
