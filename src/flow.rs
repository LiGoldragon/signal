//! Flow-graph kinds — the first sema record category criomed
//! handles end-to-end.
//!
//! Per Li 2026-04-25: *"first milestone is not machina ... I'm
//! leaning towards making the first criomed usage to be for
//! storing specification as flow-graphs (think mermaid language
//! for representing flow charts, but in fully typed binary) —
//! that way we can start designing architecture in sema."*
//!
//! **Logic only, no styling.** Per Li 2026-04-25: *"the flow
//! subset is only about representing logic, not concerning itself
//! with rendering it."* No shape, edge-style, or graph-direction
//! fields — those belong to a separate rendering layer if/when we
//! ever need one.
//!
//! These types are baked into criomed's binary. The validator's
//! schema-check matches incoming `RawRecord.kind_name` against
//! `"Node" | "Edge" | "Graph"`; everything else returns `E0001`.
//! No `KindDecl`/`FieldSpec` indirection — the Rust types ARE
//! the schema for v0.0.1.

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde::{Deserialize, Serialize};

use crate::slot::Slot;

/// A node in a flow-graph. `id` is the human-readable identifier
/// used in nexus text; `label` is the human-readable name.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Node {
    pub id: String,
    pub label: String,
}

/// A directed edge from one node to another. Optional `label`
/// names the relationship.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Edge {
    pub from: Slot,
    pub to: Slot,
    pub label: Option<String>,
}

/// A flow-graph: a titled collection of nodes and edges, with
/// optional nested subgraphs.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Graph {
    pub title: String,
    pub nodes: Vec<Slot>,
    pub edges: Vec<Slot>,
    pub subgraphs: Vec<Slot>,
}

/// The kind names criomed accepts at v0.0.1. The validator's
/// schema-check uses this list directly — no in-sema `KindDecl`
/// records needed for the first milestone.
pub const KNOWN_KINDS: &[&str] = &["Node", "Edge", "Graph"];
