//! Text round-trip tests for every signal type that derives a
//! nota-next derive.
//!
//! Closes the loop end-to-end: nota-next's own tests use toy
//! types defined inside the codec crate; this file exercises
//! the *real* signal types (Node, Edge, Graph, AssertOperation,
//! MutateOperation, the three Query types, …) through the
//! value-shaped NotaSource/to_nota API.

use nota::{NotaDecode, NotaDecodeError, NotaEncode, NotaSource};
use signal::{
    AssertOperation, Edge, EdgeQuery, Graph, GraphQuery, MutateOperation, Node, NodePlacement, NodeQuery, Ok,
    PatternField, QueryOperation, RelationKind, RetractOperation, Revision, Slot,
};

fn round_trip<T>(value: T, expected_text: &str)
where
    T: NotaEncode + NotaDecode + PartialEq + std::fmt::Debug,
{
    let text = value.to_nota();
    assert_eq!(text, expected_text, "encode produced unexpected text");

    let recovered = NotaSource::new(&text).parse::<T>().unwrap();
    assert_eq!(value, recovered, "decode did not round-trip the value");
}

// ─── NotaEncode, NotaDecode — Slot / Revision ─────────────────────

#[test]
fn slot_round_trips_as_bare_integer() {
    round_trip(Slot::<signal::Node>::from(42u64), "42");
}

#[test]
fn revision_round_trips_as_bare_integer() {
    round_trip(Revision::from(7u64), "7");
}

// ─── NotaEncode, NotaDecode — RelationKind ───────────────────────────────

#[test]
fn every_relation_kind_round_trips() {
    for kind in [
        RelationKind::Flow,
        RelationKind::DependsOn,
        RelationKind::Contains,
        RelationKind::References,
        RelationKind::Produces,
        RelationKind::Consumes,
        RelationKind::Calls,
        RelationKind::Implements,
        RelationKind::IsA,
    ] {
        let text = kind.to_nota();
        assert_eq!(NotaSource::new(&text).parse::<RelationKind>().unwrap(), kind);
    }
}

// ─── nota-next-derived flow data kinds ─────────────────────
//
// Derived structs encode without a type tag per the
// three-case PascalCase rule (case 2: `(fields…)` with no leading
// PascalCase identifier). The struct type is determined by the
// schema position the record sits at.

#[test]
fn ok_unit_record_round_trips() {
    round_trip(Ok {}, "()");
}

#[test]
fn node_round_trips() {
    round_trip(Node { name: "alice".into() }, "(alice)");
}

#[test]
fn edge_round_trips() {
    round_trip(
        Edge { from: Slot::from(100u64), to: Slot::from(200u64), kind: RelationKind::DependsOn },
        "(100 200 DependsOn)",
    );
}

#[test]
fn graph_with_populated_collections_round_trips() {
    round_trip(
        Graph {
            title: "criome request flow".into(),
            nodes: vec![Slot::from(1u64), Slot::from(2u64), Slot::from(3u64)],
            edges: vec![Slot::from(10u64), Slot::from(11u64)],
            subgraphs: vec![],
        },
        "([criome request flow] [1 2 3] [10 11] [])",
    );
}

#[test]
fn node_placement_signed_coordinates_round_trip() {
    round_trip(
        NodePlacement { graph: Slot::from(1u64), node: Slot::from(2u64), x_hundredths: -125, y_hundredths: 300 },
        "(1 2 -125 300)",
    );
}

// ─── RetractOperation per-kind variants ────────────────────
//
// Struct variants keep the variant tag. Option<T>
// present wraps as `(Some inner)`.

#[test]
fn retract_node_with_optional_revision_present_round_trips() {
    round_trip(
        RetractOperation::Node { slot: Slot::from(50u64), expected_rev: Some(Revision::from(7u64)) },
        "(Node 50 (Some 7))",
    );
}

#[test]
fn retract_node_with_optional_revision_absent_round_trips() {
    round_trip(RetractOperation::Node { slot: Slot::from(50u64), expected_rev: None }, "(Node 50 None)");
}

// `AtomicBatch` + `BatchOperation` are wire-only for M0 — the
// canonical text form `[| op1 op2 |]` with sigil-dispatched
// inner operations needs a hand-written codec impl that lands
// in M1+. No text round-trip tests here today.

// ─── Closed-kind dispatch ──────────────────────────────────
//
// Newtype variants wrap the payload with the variant tag:
// `(VariantName <payload>)`. When the payload is a derived struct,
// the inner appears as a tag-less nested record.

#[test]
fn assert_operation_node_round_trips() {
    round_trip(AssertOperation::Node(Node { name: "alice".into() }), "(Node (alice))");
}

#[test]
fn assert_operation_edge_round_trips() {
    round_trip(
        AssertOperation::Edge(Edge { from: Slot::from(1u64), to: Slot::from(2u64), kind: RelationKind::Flow }),
        "(Edge (1 2 Flow))",
    );
}

#[test]
fn mutate_operation_struct_variant_with_present_optional_round_trips() {
    round_trip(
        MutateOperation::Node {
            slot: Slot::from(100u64),
            new: Node { name: "alice".into() },
            expected_rev: Some(Revision::from(7u64)),
        },
        "(Node 100 (alice) (Some 7))",
    );
}

#[test]
fn mutate_operation_struct_variant_with_absent_optional_round_trips() {
    round_trip(
        MutateOperation::Node { slot: Slot::from(100u64), new: Node { name: "alice".into() }, expected_rev: None },
        "(Node 100 (alice) None)",
    );
}

#[test]
fn query_operation_dispatches_to_node_query() {
    round_trip(QueryOperation::Node(NodeQuery { name: PatternField::Wildcard }), "(Node ((Wildcard)))");
}

// ─── PatternField — typed marker records ───────────────────

#[test]
fn node_query_with_bind_round_trips() {
    round_trip(NodeQuery { name: PatternField::Bind }, "((Bind))");
}

#[test]
fn node_query_with_match_round_trips() {
    round_trip(NodeQuery { name: PatternField::Match("alice".into()) }, "(alice)");
}

#[test]
fn edge_query_with_three_mixed_pattern_fields_round_trips() {
    round_trip(
        EdgeQuery {
            from: PatternField::Match(Slot::from(102u64)),
            to: PatternField::Bind,
            kind: PatternField::Wildcard,
        },
        "(102 (Bind) (Wildcard))",
    );
}

#[test]
fn graph_query_round_trips() {
    round_trip(GraphQuery { title: PatternField::Match("criome request flow".into()) }, "([criome request flow])");
}

#[test]
fn bind_record_does_not_decode_as_string_field() {
    // A `(Bind)` marker can't appear at the Node.name String
    // position. With Node now tag-less, the wire `((Bind))` opens
    // Node then encounters `(` where the name string is expected.
    let error = NotaSource::new("((Bind))").parse::<Node>().unwrap_err();
    assert!(matches!(
        error,
        NotaDecodeError::ExpectedDelimited { type_name: "String", delimiter: "string atom or square bracket" }
    ));
}
