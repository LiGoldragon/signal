//! Per-verb Frame round-trip tests — each verb's typed payload
//! survives encode/decode through the rkyv wire layer.
//!
//! These tests live in `tests/` per rust/style.md; the source
//! file `src/frame.rs` no longer carries them inline.

use signal::{
    AssertOperation, AtomicBatch, AuthProof, BatchOperation, Body, Diagnostic, DiagnosticLevel,
    Edge, EdgeQuery, Frame, FrameDecodeError, Graph, GraphQuery, HandshakeRequest,
    MutateOperation, Node, NodeQuery, Ok, OutcomeMessage, PatternField, ProtocolVersion,
    QueryOperation, Records, RelationKind, Reply, Request, RetractOperation, Revision,
    SIGNAL_PROTOCOL_VERSION, Slot,
};

fn round_trip(original: Frame) {
    let bytes = original.encode();
    let decoded = Frame::decode(&bytes).expect("decode");
    assert_eq!(decoded, original);
}

#[test]
fn handshake_request_round_trip() {
    let original = Frame {
        principal_hint: None,
        auth_proof: None,
        body: Body::Request(Request::Handshake(HandshakeRequest {
            client_version: SIGNAL_PROTOCOL_VERSION,
            client_name: "nexus-cli".to_string(),
        })),
    };
    let bytes = original.encode();
    assert!(!bytes.is_empty());
    let decoded = Frame::decode(&bytes).expect("decode");
    assert_eq!(decoded, original);
}

#[test]
fn protocol_version_compatibility() {
    let v0_1_0 = ProtocolVersion { major: 0, minor: 1, patch: 0 };
    let v0_2_0 = ProtocolVersion { major: 0, minor: 2, patch: 0 };
    let v1_0_0 = ProtocolVersion { major: 1, minor: 0, patch: 0 };

    assert!(v0_1_0.is_compatible_with(v0_2_0));
    assert!(!v0_2_0.is_compatible_with(v0_1_0));
    assert!(!v0_1_0.is_compatible_with(v1_0_0));
    assert!(!v1_0_0.is_compatible_with(v0_1_0));
    assert!(v0_1_0.is_compatible_with(v0_1_0));
}

#[test]
fn decode_rejects_garbage() {
    let garbage = vec![0xff; 32];
    assert!(matches!(Frame::decode(&garbage), Err(FrameDecodeError::BadArchive)));
}

#[test]
fn assert_node_round_trip() {
    round_trip(Frame {
        principal_hint: None,
        auth_proof: None,
        body: Body::Request(Request::Assert(AssertOperation::Node(Node {
            name: "User".into(),
        }))),
    });
}

#[test]
fn assert_edge_round_trip() {
    round_trip(Frame {
        principal_hint: None,
        auth_proof: None,
        body: Body::Request(Request::Assert(AssertOperation::Edge(Edge {
            from: Slot::from(100u64),
            to: Slot::from(101u64),
            kind: RelationKind::DependsOn,
        }))),
    });
}

#[test]
fn assert_graph_round_trip() {
    round_trip(Frame {
        principal_hint: Some(Slot::from(7u64)),
        auth_proof: None,
        body: Body::Request(Request::Assert(AssertOperation::Graph(Graph {
            title: "criome request flow".into(),
            nodes: vec![Slot::from(100u64), Slot::from(101u64), Slot::from(102u64)],
            edges: vec![Slot::from(200u64), Slot::from(201u64)],
            subgraphs: vec![],
        }))),
    });
}

#[test]
fn mutate_node_round_trip() {
    round_trip(Frame {
        principal_hint: None,
        auth_proof: None,
        body: Body::Request(Request::Mutate(MutateOperation::Node {
            slot: Slot::from(100u64),
            new: Node { name: "User updated".into() },
            expected_rev: Some(Revision::from(42u64)),
        })),
    });
}

#[test]
fn retract_round_trip() {
    round_trip(Frame {
        principal_hint: None,
        auth_proof: None,
        body: Body::Request(Request::Retract(RetractOperation {
            slot: Slot::from(100u64),
            expected_rev: None,
        })),
    });
}

#[test]
fn atomic_batch_round_trip() {
    round_trip(Frame {
        principal_hint: None,
        auth_proof: None,
        body: Body::Request(Request::AtomicBatch(AtomicBatch {
            operations: vec![
                BatchOperation::Assert(AssertOperation::Node(Node { name: "A".into() })),
                BatchOperation::Mutate(MutateOperation::Node {
                    slot: Slot::from(50u64),
                    new: Node { name: "B".into() },
                    expected_rev: None,
                }),
                BatchOperation::Retract(RetractOperation {
                    slot: Slot::from(60u64),
                    expected_rev: None,
                }),
            ],
        })),
    });
}

#[test]
fn query_node_with_bind_round_trip() {
    round_trip(Frame {
        principal_hint: None,
        auth_proof: None,
        body: Body::Request(Request::Query(QueryOperation::Node(NodeQuery {
            name: PatternField::Bind,
        }))),
    });
}

#[test]
fn query_edge_mixed_pattern_round_trip() {
    round_trip(Frame {
        principal_hint: None,
        auth_proof: None,
        body: Body::Request(Request::Query(QueryOperation::Edge(EdgeQuery {
            from: PatternField::Match(Slot::from(102u64)),
            to: PatternField::Bind,
            kind: PatternField::Wildcard,
        }))),
    });
}

#[test]
fn query_graph_round_trip() {
    round_trip(Frame {
        principal_hint: None,
        auth_proof: None,
        body: Body::Request(Request::Query(QueryOperation::Graph(GraphQuery {
            title: PatternField::Match("criome request flow".into()),
        }))),
    });
}

#[test]
fn reply_outcome_ok_round_trip() {
    round_trip(Frame {
        principal_hint: None,
        auth_proof: None,
        body: Body::Reply(Reply::Outcome(OutcomeMessage::Ok(Ok {}))),
    });
}

#[test]
fn reply_records_node_round_trip() {
    round_trip(Frame {
        principal_hint: None,
        auth_proof: None,
        body: Body::Reply(Reply::Records(Records::Node(vec![
            (Slot::from(1024u64), Node { name: "Alice".into() }),
            (Slot::from(1025u64), Node { name: "Bob".into() }),
        ]))),
    });
}

#[test]
fn reply_records_edge_empty_round_trip() {
    round_trip(Frame {
        principal_hint: None,
        auth_proof: None,
        body: Body::Reply(Reply::Records(Records::Edge(vec![]))),
    });
}

#[test]
fn reply_outcomes_mixed_round_trip() {
    round_trip(Frame {
        principal_hint: None,
        auth_proof: None,
        body: Body::Reply(Reply::Outcomes(vec![
            OutcomeMessage::Ok(Ok {}),
            OutcomeMessage::Diagnostic(Diagnostic {
                level: DiagnosticLevel::Error,
                code: "E1001".into(),
                message: "unknown kind".into(),
                primary_site: None,
                context: vec![],
                suggestions: vec![],
                durable_record: None,
            }),
            OutcomeMessage::Ok(Ok {}),
        ])),
    });
}

#[test]
fn auth_proof_unused_helper() {
    // Compile-time witness that AuthProof reaches the public
    // surface; rkyv-bound type with no test surface yet.
    let _ = AuthProof::SingleOperator;
}
