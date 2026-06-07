//! Layout records — Layout, NodePlacement.
//!
//! Layout carries pane-visibility and pane-size intent for the
//! workbench shell. NodePlacement carries per-Graph per-Node 2D
//! position on the canvas — "where things are" is sema state.
//!
//! Both are intent-shaped, not appearance-shaped: shells map
//! semantic size hints (Narrow / Medium / Wide / Pixels(N)) to
//! their native pixel/em systems.

use nota_next::{Block, Delimiter, NotaBlock, NotaDecode, NotaDecodeError, NotaEncode};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_derive::Schema;

use crate::PatternField;
use crate::flow::{Graph, Node};
use crate::slot::Slot;

/// Workbench pane layout intent.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Schema, Debug, Clone, PartialEq, Eq, Hash,
)]
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
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Schema, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodePlacement {
    pub graph: Slot<Graph>,
    pub node: Slot<Node>,
    /// Position is integer hundredths-of-grid-units (1/100 of
    /// a logical canvas grid square). Concrete grid resolution
    /// finalises as the canvas wires up.
    pub x_hundredths: i64,
    pub y_hundredths: i64,
}

impl NodePlacement {
    fn signed_integer_from_block(block: &Block, type_name: &'static str) -> Result<i64, NotaDecodeError> {
        let value = block.demote_to_string().ok_or(NotaDecodeError::ExpectedAtom { type_name })?;
        value.parse::<i64>().map_err(|_| NotaDecodeError::InvalidInteger { value: value.to_owned() })
    }
}

impl NotaEncode for NodePlacement {
    fn to_nota(&self) -> String {
        Delimiter::Parenthesis.wrap([
            self.graph.to_nota(),
            self.node.to_nota(),
            self.x_hundredths.to_string(),
            self.y_hundredths.to_string(),
        ])
    }
}

impl NotaDecode for NodePlacement {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        let children = NotaBlock::new(block).expect_children(Delimiter::Parenthesis, "NodePlacement", 4)?;
        Ok(Self {
            graph: Slot::<Graph>::from_nota_block(&children[0])?,
            node: Slot::<Node>::from_nota_block(&children[1])?,
            x_hundredths: Self::signed_integer_from_block(&children[2], "NodePlacement.x")?,
            y_hundredths: Self::signed_integer_from_block(&children[3], "NodePlacement.y")?,
        })
    }
}

/// Semantic size hint. Shells map to native pixel/em systems.
///
/// Intent-only — variants are unit. The NOTA codec requires every
/// variant to be a unit variant (the wire form is the variant
/// name; data-carrying variants would break the closed-enum
/// text-roundtrip guarantee). Pixel-precision overrides, if
/// ever needed, will land as a separate `pixel_override:
/// Option<u32>` field on Layout — keeping the semantic-intent
/// names clean.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Schema, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum SizeIntent {
    /// Compact.
    Narrow,
    /// Default.
    Medium,
    /// Spacious.
    Wide,
}

/// Paired queries.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone)]
pub struct LayoutQuery {
    pub display_name: PatternField<String>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone)]
pub struct NodePlacementQuery {
    pub graph: PatternField<Slot<Graph>>,
    pub node: PatternField<Slot<Node>>,
}
