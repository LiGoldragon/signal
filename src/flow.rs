//! Flow-graph kinds — Node, Edge, Graph + paired Query kinds.
//!
//! Per Li 2026-04-25: *"first criome usage to be for storing
//! specification as flow-graphs (think mermaid for representing
//! flow charts, but in fully typed binary) — that way we can start
//! designing architecture in sema."*
//!
//! **Logic only, no styling.** Per Li 2026-04-25: *"the flow
//! subset is only about representing logic, not concerning itself
//! with rendering it."* No shape, edge-style, or graph-direction
//! fields — those belong to a separate rendering layer if/when we
//! ever need one.
//!
//! Each data kind has a paired `*Query` kind — `Node` ↔ `NodeQuery`,
//! `Edge` ↔ `EdgeQuery`, `Graph` ↔ `GraphQuery` — per the perfect-
//! specificity invariant ([criome/ARCHITECTURE.md §2 Invariant D
//! ](https://github.com/LiGoldragon/criome/blob/main/ARCHITECTURE.md#invariant-d)).
//! A query is itself a record kind, hand-written today; once
//! `prism` lands, the data and query kinds will be projected from
//! the same source records.
//!
//! **Next expected addition: node-kind taxonomy.** Today a `Node`
//! is just a name — slot identity carries the rest. The taxonomy
//! (Source / Transformer / Sink / Junction / Supervisor) lets a
//! flow-graph say what each node *does* in the dataflow, which
//! prism's first emission template needs to project a graph into
//! Rust. Lands alongside the prism skeleton; until then nodes are
//! relation-only.

use nota_codec::{NexusPattern, NotaEnum, NotaRecord, PatternField};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

use crate::slot::Slot;

// ─── Data kinds ──────────────────────────────────────────────

/// A node in a flow-graph. `name` is the display handle —
/// human-readable text. **Identity is the node's slot**, not its
/// name; two nodes with the same name are two different nodes
/// (different slots). Names exist for display, never for
/// reference.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Node {
    pub name: String,
}

/// A directed edge from one node to another, typed by its relation
/// kind. Per Li 2026-04-26: every edge declares what relation it
/// carries — strongly-typed, closed vocabulary.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Edge {
    pub from: Slot<Node>,
    pub to: Slot<Node>,
    pub kind: RelationKind,
}

/// Closed vocabulary of relation kinds an Edge can carry. Covers
/// PROV-O / UML / Mermaid-class precedent. Extend as new relation
/// semantics are needed; deletions are breaking changes.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelationKind {
    /// Generic forward flow — data, control, anything moving from
    /// `from` to `to`.
    Flow,
    /// `from` depends on `to` (compile, runtime, semantic).
    DependsOn,
    /// `from` contains `to` (composition).
    Contains,
    /// `from` references `to` (weak, non-owning).
    References,
    /// `from` produces `to` as output.
    Produces,
    /// `from` consumes `to` as input.
    Consumes,
    /// `from` invokes `to` (function/method call).
    Calls,
    /// `from` implements interface/trait `to`.
    Implements,
    /// `from` is-a `to` (subtype, kind-of).
    IsA,
}

/// A flow-graph: a titled collection of nodes and edges, with
/// optional nested subgraphs.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Graph {
    pub title: String,
    pub nodes: Vec<Slot<Node>>,
    pub edges: Vec<Slot<Edge>>,
    pub subgraphs: Vec<Slot<Graph>>,
}

// ─── Query kinds ─────────────────────────────────────────────

/// Query for `Node` records. Match by `name` field.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NexusPattern, Debug, Clone, PartialEq)]
#[nota(queries = "Node")]
pub struct NodeQuery {
    pub name: PatternField<String>,
}

/// Query for `Edge` records. Match by any combination of `from`,
/// `to`, `kind`.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NexusPattern, Debug, Clone, PartialEq)]
#[nota(queries = "Edge")]
pub struct EdgeQuery {
    pub from: PatternField<Slot<Node>>,
    pub to: PatternField<Slot<Node>>,
    pub kind: PatternField<RelationKind>,
}

/// Query for `Graph` records. Match by `title`. List-shaped fields
/// (`nodes`, `edges`, `subgraphs`) are not patternable in M0 —
/// list patterns are M1+.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NexusPattern, Debug, Clone, PartialEq)]
#[nota(queries = "Graph")]
pub struct GraphQuery {
    pub title: PatternField<String>,
}

// ─── Outcome message kind ────────────────────────────────────

/// Success acknowledgement message. Empty record kind — the
/// presence of `(Ok)` at a reply position means the request
/// succeeded with no further information. Failure replies use
/// the existing `Diagnostic` kind.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Ok {}
