//! Flow-graph kinds — the first sema record category criomed
//! handles end-to-end.
//!
//! Per Li 2026-04-25: *"first milestone is not machina ... I'm
//! leaning towards making the first criomed usage to be for
//! storing specification as flow-graphs (think mermaid language
//! for representing flow charts, but in fully typed binary) —
//! that way we can start designing architecture in sema."*
//!
//! These types are baked into criomed's binary. The validator's
//! schema-check matches incoming `RawRecord.kind_name` against
//! `"Node" | "Edge" | "Graph"`; everything else returns `E0001`.
//! No `KindDecl`/`FieldSpec` indirection — the Rust types ARE
//! the schema for v0.0.1.
//!
//! Extension policy: add fields to existing structs only when
//! Mermaid demands them; add new variants to `NodeShape` /
//! `EdgeStyle` / `GraphDirection` as needed; new kinds (e.g.
//! `Cluster`, `Swimlane`) land here as new sibling structs.

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde::{Deserialize, Serialize};

use crate::slot::Slot;

/// A node in a flow-graph. Identified by a human-readable
/// `id` (used in nexus text and in display); rendered with
/// `label` text inside `shape`.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct Node {
    pub id: String,
    pub label: String,
    pub shape: NodeShape,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum NodeShape {
    Rect,
    Round,
    Diamond,
    Cylinder,
    Subroutine,
    Hexagon,
    Parallelogram,
}

/// A directed edge from one node to another. Optional `label`
/// renders next to the arrow.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct Edge {
    pub from: Slot,
    pub to: Slot,
    pub label: Option<String>,
    pub style: EdgeStyle,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum EdgeStyle {
    Solid,
    Dashed,
    Thick,
    Bidirectional,
}

/// A flow-graph: a titled collection of nodes and edges, with a
/// flow direction and optional nested subgraphs.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct Graph {
    pub title: String,
    pub direction: GraphDirection,
    pub nodes: Vec<Slot>,
    pub edges: Vec<Slot>,
    pub subgraphs: Vec<Slot>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum GraphDirection {
    TopDown,
    LeftRight,
    BottomTop,
    RightLeft,
}

/// The kind names criomed accepts at v0.0.1. The validator's
/// schema-check uses this list directly — no in-sema `KindDecl`
/// records needed for the first milestone.
pub const KNOWN_KINDS: &[&str] = &["Node", "Edge", "Graph"];
