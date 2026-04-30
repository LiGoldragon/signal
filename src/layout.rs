//! Layout records — Layout, NodePlacement.
//!
//! Layout carries pane-visibility and pane-size intent for the
//! workbench shell. NodePlacement carries per-Graph per-Node 2D
//! position on the canvas — "where things are" is sema state.
//!
//! Both are intent-shaped, not appearance-shaped: shells map
//! semantic size hints (Narrow / Medium / Wide / Pixels(N)) to
//! their native pixel/em systems.

use nota_codec::{NexusPattern, NotaEnum, NotaRecord, PatternField};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

use crate::flow::{Graph, Node};
use crate::slot::Slot;

/// Workbench pane layout intent.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Layout {
    pub display_name: String,
    pub left_nav_width: SizeIntent,
    pub inspector_width: SizeIntent,
    pub diagnostics_height: SizeIntent,
    pub wire_height: SizeIntent,
    /// Whether the wire pane is currently shown.
    pub wire_visible: bool,
}

/// Per-Graph per-Node 2D position on the canvas. One record
/// per (graph, node) pair; identity is the slot.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodePlacement {
    pub graph: Slot<Graph>,
    pub node: Slot<Node>,
    /// Position is integer hundredths-of-grid-units (1/100 of
    /// a logical canvas grid square). Concrete grid resolution
    /// finalises as the canvas wires up.
    pub x_hundredths: i64,
    pub y_hundredths: i64,
}

/// Semantic size hint. Shells map to native pixel/em systems.
///
/// Intent-only — variants are unit. NotaEnum requires every
/// variant to be a unit variant (the wire form is the variant
/// name; data-carrying variants would break the closed-enum
/// text-roundtrip guarantee). Pixel-precision overrides, if
/// ever needed, will land as a separate `pixel_override:
/// Option<u32>` field on Layout — keeping the semantic-intent
/// names clean.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SizeIntent {
    /// Compact.
    Narrow,
    /// Default.
    Medium,
    /// Spacious.
    Wide,
}

/// Paired queries.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NexusPattern, Debug, Clone)]
#[nota(queries = "Layout")]
pub struct LayoutQuery {
    pub display_name: PatternField<String>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NexusPattern, Debug, Clone)]
#[nota(queries = "NodePlacement")]
pub struct NodePlacementQuery {
    pub graph: PatternField<Slot<Graph>>,
    pub node: PatternField<Slot<Node>>,
}
