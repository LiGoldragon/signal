//! Text round-trip tests for every signal type that derives a
//! nota-codec derive.
//!
//! Closes the loop end-to-end: nota-codec's own tests use toy
//! types defined inside the codec crate; this file exercises
//! the *real* signal types (Node, Edge, Graph, KindDecl,
//! AssertOperation, MutateOperation, the four Query types, …)
//! through the Decoder/Encoder protocol.

use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};
use signal::{
    AssertOperation, Cardinality, Edge, EdgeQuery, FieldDecl, Graph, GraphQuery, KindDecl,
    KindDeclQuery, MutateOperation, Node, NodeQuery, Ok, PatternField, QueryOperation,
    RelationKind, RetractOperation, Revision, Slot,
};

fn round_trip<T>(value: T, expected_text: &str)
where
    T: NotaEncode + NotaDecode + PartialEq + std::fmt::Debug,
{
    let mut encoder = Encoder::nexus();
    value.encode(&mut encoder).unwrap();
    let text = encoder.into_string();
    assert_eq!(text, expected_text, "encode produced unexpected text");

    let mut decoder = Decoder::nexus(&text);
    let recovered = T::decode(&mut decoder).unwrap();
    assert_eq!(value, recovered, "decode did not round-trip the value");
}

// ─── NotaTransparent — Slot / Revision ─────────────────────

#[test]
fn slot_round_trips_as_bare_integer() {
    round_trip(Slot::from(42u64), "42");
}

#[test]
fn revision_round_trips_as_bare_integer() {
    round_trip(Revision::from(7u64), "7");
}

// ─── NotaEnum — RelationKind / Cardinality ─────────────────

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
        let mut encoder = Encoder::nexus();
        kind.encode(&mut encoder).unwrap();
        let text = encoder.into_string();
        let mut decoder = Decoder::nexus(&text);
        assert_eq!(RelationKind::decode(&mut decoder).unwrap(), kind);
    }
}

#[test]
fn cardinality_one_emits_its_identifier() {
    round_trip(Cardinality::One, "One");
    round_trip(Cardinality::Many, "Many");
    round_trip(Cardinality::Optional, "Optional");
}

// ─── NotaRecord — flow data kinds ──────────────────────────

#[test]
fn ok_unit_record_round_trips() {
    round_trip(Ok {}, "(Ok)");
}

#[test]
fn node_round_trips() {
    round_trip(Node { name: "User".into() }, "(Node \"User\")");
}

#[test]
fn edge_round_trips() {
    round_trip(
        Edge {
            from: Slot::from(100u64),
            to: Slot::from(200u64),
            kind: RelationKind::DependsOn,
        },
        "(Edge 100 200 DependsOn)",
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
        "(Graph \"criome request flow\" [1 2 3] [10 11] [])",
    );
}

// ─── NotaRecord — schema kinds ─────────────────────────────

#[test]
fn field_decl_round_trips() {
    round_trip(
        FieldDecl {
            name: "from".into(),
            type_name: "Slot".into(),
            cardinality: Cardinality::One,
        },
        "(FieldDecl \"from\" \"Slot\" One)",
    );
}

#[test]
fn kind_decl_with_nested_field_decls_round_trips() {
    round_trip(
        KindDecl {
            name: "Edge".into(),
            fields: vec![
                FieldDecl { name: "from".into(), type_name: "Slot".into(), cardinality: Cardinality::One },
                FieldDecl { name: "to".into(), type_name: "Slot".into(), cardinality: Cardinality::One },
                FieldDecl { name: "kind".into(), type_name: "RelationKind".into(), cardinality: Cardinality::One },
            ],
        },
        "(KindDecl \"Edge\" [(FieldDecl \"from\" \"Slot\" One) (FieldDecl \"to\" \"Slot\" One) (FieldDecl \"kind\" \"RelationKind\" One)])",
    );
}

// ─── NotaRecord — edit operations ──────────────────────────

#[test]
fn retract_operation_with_optional_revision_present_round_trips() {
    round_trip(
        RetractOperation { slot: Slot::from(50u64), expected_rev: Some(Revision::from(7u64)) },
        "(RetractOperation 50 7)",
    );
}

#[test]
fn retract_operation_with_optional_revision_absent_round_trips() {
    round_trip(
        RetractOperation { slot: Slot::from(50u64), expected_rev: None },
        "(RetractOperation 50)",
    );
}

// `AtomicBatch` + `BatchOperation` are wire-only for M0 — the
// canonical text form `[| op1 op2 |]` with sigil-dispatched
// inner operations needs a hand-written codec impl that lands
// in M1+. No text round-trip tests here today.

// ─── NexusVerb — closed-kind dispatch ──────────────────────

#[test]
fn assert_operation_node_round_trips() {
    round_trip(
        AssertOperation::Node(Node { name: "User".into() }),
        "(Node \"User\")",
    );
}

#[test]
fn assert_operation_edge_round_trips() {
    round_trip(
        AssertOperation::Edge(Edge {
            from: Slot::from(1u64),
            to: Slot::from(2u64),
            kind: RelationKind::Flow,
        }),
        "(Edge 1 2 Flow)",
    );
}

#[test]
fn mutate_operation_struct_variant_with_present_optional_round_trips() {
    round_trip(
        MutateOperation::Node {
            slot: Slot::from(100u64),
            new: Node { name: "Alice".into() },
            expected_rev: Some(Revision::from(7u64)),
        },
        "(Node 100 (Node \"Alice\") 7)",
    );
}

#[test]
fn mutate_operation_struct_variant_with_absent_optional_round_trips() {
    round_trip(
        MutateOperation::Node {
            slot: Slot::from(100u64),
            new: Node { name: "Alice".into() },
            expected_rev: None,
        },
        "(Node 100 (Node \"Alice\"))",
    );
}

#[test]
fn query_operation_dispatches_to_node_query() {
    round_trip(
        QueryOperation::Node(NodeQuery { name: PatternField::Wildcard }),
        "(| Node _ |)",
    );
}

// ─── NexusPattern — query records ──────────────────────────

#[test]
fn node_query_with_bind_round_trips() {
    round_trip(
        NodeQuery { name: PatternField::Bind },
        "(| Node @name |)",
    );
}

#[test]
fn node_query_with_match_round_trips() {
    round_trip(
        NodeQuery { name: PatternField::Match("User".into()) },
        "(| Node \"User\" |)",
    );
}

#[test]
fn edge_query_with_three_mixed_pattern_fields_round_trips() {
    round_trip(
        EdgeQuery {
            from: PatternField::Match(Slot::from(102u64)),
            to: PatternField::Bind,
            kind: PatternField::Wildcard,
        },
        "(| Edge 102 @to _ |)",
    );
}

#[test]
fn graph_query_round_trips() {
    round_trip(
        GraphQuery { title: PatternField::Match("criome request flow".into()) },
        "(| Graph \"criome request flow\" |)",
    );
}

#[test]
fn kind_decl_query_with_bind_round_trips() {
    round_trip(
        KindDeclQuery { name: PatternField::Bind },
        "(| KindDecl @name |)",
    );
}

// ─── Cross-cutting: a complex value round-trips ─────────────

#[test]
fn nested_assert_of_kind_decl_with_field_decls_round_trips() {
    round_trip(
        AssertOperation::KindDecl(KindDecl {
            name: "Hyperedge".into(),
            fields: vec![
                FieldDecl { name: "members".into(), type_name: "Slot".into(), cardinality: Cardinality::Many },
                FieldDecl { name: "weight".into(), type_name: "f64".into(), cardinality: Cardinality::One },
            ],
        }),
        "(KindDecl \"Hyperedge\" [(FieldDecl \"members\" \"Slot\" Many) (FieldDecl \"weight\" \"f64\" One)])",
    );
}
