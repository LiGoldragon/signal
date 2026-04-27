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

use nota_codec::{NexusVerb, NotaRecord};

// `AtomicBatch` and `BatchOperation` derive only rkyv (no
// `NotaRecord` / `NexusVerb`) for M0 — see their per-type docs
// for the M1+ hand-impl plan.
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

use crate::flow::{Edge, Graph, Node};
use crate::schema::KindDecl;
use crate::slot::{Revision, Slot};

/// Introduce a new record. Criome assigns the slot internally on
/// commit. Genesis runs the same flow as user-authored asserts —
/// no backdoor for pre-assigned slots.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NexusVerb, Debug, Clone, PartialEq)]
pub enum AssertOperation {
    Node(Node),
    Edge(Edge),
    Graph(Graph),
    KindDecl(KindDecl),
}

/// Whole-record replacement at a slot. Each variant carries the
/// target slot, the typed replacement, and an optional
/// `expected_rev` for compare-and-swap semantics.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NexusVerb, Debug, Clone, PartialEq)]
pub enum MutateOperation {
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
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RetractOperation {
    pub slot: Slot,
    pub expected_rev: Option<Revision>,
}

/// Atomic envelope wrapping a sequence of edit operations.
/// All-or-nothing commit at one Revision in one transaction.
/// The reply is per-element `OutcomeMessage` paired by index.
///
/// **Wire form (rkyv only for M0).** The canonical nexus text
/// form is `[| op1 op2 op3 |]` with sigil-dispatched inner
/// operations (`(Node …)` for assert, `~(Node …)` for mutate,
/// `!slot` for retract). That dispatch can't be derived
/// uniformly because the inner shape switches by sigil; the
/// hand-written `NotaEncode`/`NotaDecode` impls land in M1+
/// alongside the hand-written `Decoder::next_request` extension
/// for `[|` openers.
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
