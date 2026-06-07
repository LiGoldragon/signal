//! Edit verbs — per-verb typed payloads.
//!
//! Per the perfect-specificity invariant
//! ([criome/ARCHITECTURE.md §2 Invariant D
//! ](https://github.com/LiGoldragon/criome/blob/main/ARCHITECTURE.md#invariant-d)):
//! each Operation enum names exactly the kinds it operates on.
//! No generic record wrapper; no string kind-name dispatch.
//!
//! Three edit verbs:
//! - `Assert` introduces a new record. Criome assigns the slot.
//! - `Mutate` replaces an existing record at a slot. Identity
//!   (slot) is preserved; per-variant carries the typed
//!   replacement and an optional CAS revision.
//! - `Retract` removes a record at a slot. Slot identifies the
//!   target — no per-kind variants needed.
//!
//! Atomic batches wrap a sequence of edit operations as
//! all-or-nothing.

use nota_next::{Block, Delimiter, NotaBlock, NotaDecode, NotaDecodeError, NotaEncode};

// `AtomicBatch` and `BatchOperation` derive only rkyv (no
// `NotaEncode` / `NotaDecode`) for M0 — see their per-type docs
// for the M1+ hand-impl plan.
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

use crate::flow::{Edge, Graph, Node};
use crate::slot::{Revision, Slot};

/// Introduce a new record. Criome assigns the slot internally on
/// commit. Genesis runs the same flow as user-authored asserts —
/// no backdoor for pre-assigned slots.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq)]
pub enum AssertOperation {
    Node(Node),
    Edge(Edge),
    Graph(Graph),
}

/// Whole-record replacement at a slot. Each variant carries the
/// target slot, the typed replacement, and an optional
/// `expected_rev` for compare-and-swap semantics.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
pub enum MutateOperation {
    Node { slot: Slot<Node>, new: Node, expected_rev: Option<Revision> },
    Edge { slot: Slot<Edge>, new: Edge, expected_rev: Option<Revision> },
    Graph { slot: Slot<Graph>, new: Graph, expected_rev: Option<Revision> },
}

impl NotaEncode for MutateOperation {
    fn to_nota(&self) -> String {
        match self {
            Self::Node { slot, new, expected_rev } => {
                Delimiter::Parenthesis.wrap(["Node".to_owned(), slot.to_nota(), new.to_nota(), expected_rev.to_nota()])
            }
            Self::Edge { slot, new, expected_rev } => {
                Delimiter::Parenthesis.wrap(["Edge".to_owned(), slot.to_nota(), new.to_nota(), expected_rev.to_nota()])
            }
            Self::Graph { slot, new, expected_rev } => {
                Delimiter::Parenthesis.wrap(["Graph".to_owned(), slot.to_nota(), new.to_nota(), expected_rev.to_nota()])
            }
        }
    }
}

impl NotaDecode for MutateOperation {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        let children = NotaBlock::new(block).expect_children(Delimiter::Parenthesis, "MutateOperation", 4)?;
        let variant = children[0]
            .demote_to_string()
            .ok_or(NotaDecodeError::ExpectedAtom { type_name: "MutateOperation variant" })?;
        match variant {
            "Node" => Ok(Self::Node {
                slot: Slot::<Node>::from_nota_block(&children[1])?,
                new: Node::from_nota_block(&children[2])?,
                expected_rev: Option::<Revision>::from_nota_block(&children[3])?,
            }),
            "Edge" => Ok(Self::Edge {
                slot: Slot::<Edge>::from_nota_block(&children[1])?,
                new: Edge::from_nota_block(&children[2])?,
                expected_rev: Option::<Revision>::from_nota_block(&children[3])?,
            }),
            "Graph" => Ok(Self::Graph {
                slot: Slot::<Graph>::from_nota_block(&children[1])?,
                new: Graph::from_nota_block(&children[2])?,
                expected_rev: Option::<Revision>::from_nota_block(&children[3])?,
            }),
            other => Err(NotaDecodeError::UnknownVariant { enum_name: "MutateOperation", variant: other.to_owned() }),
        }
    }
}

/// Remove an existing record. Per-kind variants for the same
/// reason as `MutateOperation` and `AssertOperation`: the type
/// system carries which kind is being retracted, so the validator
/// can dispatch per-kind reachability checks without
/// stringly-typed lookups.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
pub enum RetractOperation {
    Node { slot: Slot<Node>, expected_rev: Option<Revision> },
    Edge { slot: Slot<Edge>, expected_rev: Option<Revision> },
    Graph { slot: Slot<Graph>, expected_rev: Option<Revision> },
}

impl NotaEncode for RetractOperation {
    fn to_nota(&self) -> String {
        match self {
            Self::Node { slot, expected_rev } => {
                Delimiter::Parenthesis.wrap(["Node".to_owned(), slot.to_nota(), expected_rev.to_nota()])
            }
            Self::Edge { slot, expected_rev } => {
                Delimiter::Parenthesis.wrap(["Edge".to_owned(), slot.to_nota(), expected_rev.to_nota()])
            }
            Self::Graph { slot, expected_rev } => {
                Delimiter::Parenthesis.wrap(["Graph".to_owned(), slot.to_nota(), expected_rev.to_nota()])
            }
        }
    }
}

impl NotaDecode for RetractOperation {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        let children = NotaBlock::new(block).expect_children(Delimiter::Parenthesis, "RetractOperation", 3)?;
        let variant = children[0]
            .demote_to_string()
            .ok_or(NotaDecodeError::ExpectedAtom { type_name: "RetractOperation variant" })?;
        match variant {
            "Node" => Ok(Self::Node {
                slot: Slot::<Node>::from_nota_block(&children[1])?,
                expected_rev: Option::<Revision>::from_nota_block(&children[2])?,
            }),
            "Edge" => Ok(Self::Edge {
                slot: Slot::<Edge>::from_nota_block(&children[1])?,
                expected_rev: Option::<Revision>::from_nota_block(&children[2])?,
            }),
            "Graph" => Ok(Self::Graph {
                slot: Slot::<Graph>::from_nota_block(&children[1])?,
                expected_rev: Option::<Revision>::from_nota_block(&children[2])?,
            }),
            other => Err(NotaDecodeError::UnknownVariant { enum_name: "RetractOperation", variant: other.to_owned() }),
        }
    }
}

/// Atomic envelope wrapping a sequence of edit operations.
/// All-or-nothing commit at one Revision in one transaction.
/// The reply is per-element `OutcomeMessage` paired by index.
///
/// **Wire form (rkyv only for M0).** The canonical Nexus record
/// form in NOTA syntax is `[| op1 op2 op3 |]` with sigil-dispatched inner
/// operations (`(Node …)` for assert, `~(Node …)` for mutate,
/// `!slot` for retract). That dispatch can't be derived
/// uniformly because the inner shape switches by sigil; the
/// hand-written `NotaEncode`/`NotaDecode` impls land in M1+
/// alongside the Nexus translator extension for `[|` openers.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
pub struct AtomicBatch {
    pub operations: Vec<BatchOperation>,
}

/// One operation inside an `AtomicBatch`. Wire-only for M0;
/// see [`AtomicBatch`] for the M1+ text-form plan.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
pub enum BatchOperation {
    Assert(AssertOperation),
    Mutate(MutateOperation),
    Retract(RetractOperation),
}
