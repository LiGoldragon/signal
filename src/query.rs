//! Query verb — per-kind typed Query payload.
//!
//! Per the perfect-specificity invariant: each variant carries the
//! precise `*Query` kind it operates on. The `*Query` types live
//! alongside their data kinds (`NodeQuery`/`EdgeQuery`/`GraphQuery`
//! in [`crate::flow`], `KindDeclQuery` in [`crate::schema`]).
//!
//! Aggregation operators (`Count`, `Sum`, `GroupBy`, `OrderBy`,
//! …) are M2+ work and live outside this enum when they land. M0
//! supports kind-and-field matching only.

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde::{Deserialize, Serialize};

use crate::flow::{EdgeQuery, GraphQuery, NodeQuery};
use crate::schema::KindDeclQuery;

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum QueryOp {
    Node(NodeQuery),
    Edge(EdgeQuery),
    Graph(GraphQuery),
    KindDecl(KindDeclQuery),
}
