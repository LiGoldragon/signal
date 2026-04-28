//! Query verb — per-kind typed Query payload.
//!
//! Per the perfect-specificity invariant: each variant carries
//! the precise `*Query` kind it operates on. The `*Query` types
//! live alongside their data kinds (`NodeQuery` / `EdgeQuery` /
//! `GraphQuery` in [`crate::flow`]).
//!
//! Aggregation operators (`Count`, `Sum`, `GroupBy`, `OrderBy`,
//! …) are M2+ work and live outside this enum when they land. M0
//! supports kind-and-field matching only.

use nota_codec::NexusVerb;
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

use crate::flow::{EdgeQuery, GraphQuery, NodeQuery};

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NexusVerb, Debug, Clone, PartialEq)]
pub enum QueryOperation {
    Node(NodeQuery),
    Edge(EdgeQuery),
    Graph(GraphQuery),
}
