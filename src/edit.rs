//! Edit verbs — per-verb typed payloads.
//!
//! Per the perfect-specificity invariant
//! ([criome/ARCHITECTURE.md §2 Invariant D
//! ](https://github.com/LiGoldragon/criome/blob/main/ARCHITECTURE.md#invariant-d)):
//! each Op enum names exactly the kinds it operates on. No generic
//! record wrapper; no string kind-name dispatch.
//!
//! Three edit verbs:
//! - `Assert` introduces a new record. Criome assigns the slot.
//! - `Mutate` replaces an existing record at a slot. Identity
//!   (slot) is preserved; per-variant carries the typed
//!   replacement and an optional CAS revision.
//! - `Retract` removes a record at a slot. Slot identifies the
//!   target — no per-kind variants needed.
//!
//! Atomic batches wrap a sequence of edit ops as all-or-nothing.

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde::{Deserialize, Serialize};

use crate::flow::{Edge, Graph, Node};
use crate::schema::KindDecl;
use crate::slot::{Revision, Slot};

/// Introduce a new record. Criome assigns the slot internally on
/// commit. Genesis runs the same flow as user-authored asserts —
/// no backdoor for pre-assigned slots.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum AssertOp {
    Node(Node),
    Edge(Edge),
    Graph(Graph),
    KindDecl(KindDecl),
}

/// Whole-record replacement at a slot. Each variant carries the
/// target slot, the typed replacement, and an optional
/// `expected_rev` for compare-and-swap semantics.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MutateOp {
    Node {
        slot: Slot,
        new: Node,
        expected_rev: Option<Revision>,
    },
    Edge {
        slot: Slot,
        new: Edge,
        expected_rev: Option<Revision>,
    },
    Graph {
        slot: Slot,
        new: Graph,
        expected_rev: Option<Revision>,
    },
    KindDecl {
        slot: Slot,
        new: KindDecl,
        expected_rev: Option<Revision>,
    },
}

/// Remove the record at a slot. Validator rejects if any
/// outstanding references would dangle. No per-kind variants —
/// the slot identifies the target uniquely.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RetractOp {
    pub slot: Slot,
    pub expected_rev: Option<Revision>,
}

/// Atomic envelope wrapping a sequence of edit ops. All-or-nothing
/// commit at one Revision in one transaction. The reply is per-
/// element `OutcomeMessage` paired by index.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AtomicBatch {
    pub ops: Vec<BatchOp>,
}

/// One op inside an `AtomicBatch`.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum BatchOp {
    Assert(AssertOp),
    Mutate(MutateOp),
    Retract(RetractOp),
}
