//! [`Frame`] — the wire envelope for every signal message.
//!
//! Each Frame carries an optional principal hint (which Slot-bound
//! principal is making the request), an optional auth proof, and
//! a body (Request or Reply).
//!
//! Replies pair to requests by **position on the connection (FIFO)**;
//! there is no correlation id. The first reply on a connection
//! corresponds to the first request, the second reply to the second
//! request, and so on.
//!
//! The frame schema *is* the framing — both nexus and criome
//! know the rkyv schema, so a single `rkyv::to_bytes` /
//! `rkyv::from_bytes` per Frame covers transport.

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

use crate::Slot;

use crate::auth::AuthProof;
use crate::reply::Reply;
use crate::request::Request;

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
pub struct Frame {
    /// Slot-bound principal making this request. `None` is
    /// allowed during handshake and for unauthenticated probes.
    pub principal_hint: Option<Slot>,

    /// Authentication proof. `None` only during handshake; every
    /// post-handshake frame carries one (SingleOperator MVP).
    pub auth_proof: Option<AuthProof>,

    pub body: Body,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
pub enum Body {
    Request(Request),
    Reply(Reply),
}

impl Frame {
    /// Encode to rkyv-archive bytes for socket write.
    ///
    /// rkyv 0.8 portable feature set per
    /// `mentci/reports/074` guarantees deterministic bytes
    /// across machines (little_endian + pointer_width_32 +
    /// unaligned).
    pub fn encode(&self) -> Vec<u8> {
        rkyv::to_bytes::<rkyv::rancor::Error>(self).expect("rkyv serialisation does not fail for owned values").to_vec()
    }

    /// Decode from rkyv-archive bytes off the socket. Validates
    /// the archive via `bytecheck` before deserialising.
    pub fn decode(bytes: &[u8]) -> Result<Self, FrameDecodeError> {
        rkyv::from_bytes::<Self, rkyv::rancor::Error>(bytes).map_err(|_| FrameDecodeError::BadArchive)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FrameDecodeError {
    #[error("rkyv archive validation failed")]
    BadArchive,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handshake::{HandshakeRequest, ProtocolVersion, SIGNAL_PROTOCOL_VERSION};

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

        // Older client / newer server: compatible.
        assert!(v0_1_0.is_compatible_with(v0_2_0));
        // Newer client / older server: not compatible.
        assert!(!v0_2_0.is_compatible_with(v0_1_0));
        // Major mismatch: not compatible.
        assert!(!v0_1_0.is_compatible_with(v1_0_0));
        assert!(!v1_0_0.is_compatible_with(v0_1_0));
        // Same: compatible.
        assert!(v0_1_0.is_compatible_with(v0_1_0));
    }

    #[test]
    fn decode_rejects_garbage() {
        let garbage = vec![0xff; 32];
        assert!(matches!(Frame::decode(&garbage), Err(FrameDecodeError::BadArchive)));
    }

    // Per-verb round-trip tests — each verb's typed payload survives
    // encode/decode, exercising the perfect-specificity shape end-to-
    // end via Frame.

    use crate::edit::{AssertOp, AtomicBatch, BatchOp, MutateOp, RetractOp};
    use crate::flow::{Edge, EdgeQuery, Graph, GraphQuery, Node, NodeQuery, Ok, RelationKind};
    use crate::pattern::PatternField;
    use crate::query::QueryOp;
    use crate::reply::{OutcomeMessage, Records, Reply};
    use crate::schema::{Cardinality, FieldDecl, KindDecl, KindDeclQuery};
    use crate::slot::{Revision, Slot};

    fn round_trip(original: Frame) {
        let bytes = original.encode();
        let decoded = Frame::decode(&bytes).expect("decode");
        assert_eq!(decoded, original);
    }

    #[test]
    fn assert_node_round_trip() {
        round_trip(Frame {
            principal_hint: None,
            auth_proof: None,
            body: Body::Request(Request::Assert(AssertOp::Node(Node {
                name: "User".into(),
            }))),
        });
    }

    #[test]
    fn assert_edge_round_trip() {
        round_trip(Frame {
            principal_hint: None,
            auth_proof: None,
            body: Body::Request(Request::Assert(AssertOp::Edge(Edge {
                from: Slot(100),
                to: Slot(101),
                kind: RelationKind::DependsOn,
            }))),
        });
    }

    #[test]
    fn assert_graph_round_trip() {
        round_trip(Frame {
            principal_hint: Some(Slot(7)),
            auth_proof: None,
            body: Body::Request(Request::Assert(AssertOp::Graph(Graph {
                title: "criome request flow".into(),
                nodes: vec![Slot(100), Slot(101), Slot(102)],
                edges: vec![Slot(200), Slot(201)],
                subgraphs: vec![],
            }))),
        });
    }

    #[test]
    fn assert_kind_decl_round_trip() {
        round_trip(Frame {
            principal_hint: None,
            auth_proof: None,
            body: Body::Request(Request::Assert(AssertOp::KindDecl(KindDecl {
                name: "Hyperedge".into(),
                fields: vec![
                    FieldDecl {
                        name: "members".into(),
                        type_name: "Slot".into(),
                        cardinality: Cardinality::Many,
                    },
                    FieldDecl {
                        name: "weight".into(),
                        type_name: "f64".into(),
                        cardinality: Cardinality::One,
                    },
                ],
            }))),
        });
    }

    #[test]
    fn mutate_node_round_trip() {
        round_trip(Frame {
            principal_hint: None,
            auth_proof: None,
            body: Body::Request(Request::Mutate(MutateOp::Node {
                slot: Slot(100),
                new: Node { name: "User updated".into() },
                expected_rev: Some(Revision(42)),
            })),
        });
    }

    #[test]
    fn retract_round_trip() {
        round_trip(Frame {
            principal_hint: None,
            auth_proof: None,
            body: Body::Request(Request::Retract(RetractOp {
                slot: Slot(100),
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
                ops: vec![
                    BatchOp::Assert(AssertOp::Node(Node { name: "A".into() })),
                    BatchOp::Mutate(MutateOp::Node {
                        slot: Slot(50),
                        new: Node { name: "B".into() },
                        expected_rev: None,
                    }),
                    BatchOp::Retract(RetractOp { slot: Slot(60), expected_rev: None }),
                ],
            })),
        });
    }

    #[test]
    fn query_node_with_bind_round_trip() {
        round_trip(Frame {
            principal_hint: None,
            auth_proof: None,
            body: Body::Request(Request::Query(QueryOp::Node(NodeQuery {
                name: PatternField::Bind("name".into()),
            }))),
        });
    }

    #[test]
    fn query_edge_mixed_pattern_round_trip() {
        round_trip(Frame {
            principal_hint: None,
            auth_proof: None,
            body: Body::Request(Request::Query(QueryOp::Edge(EdgeQuery {
                from: PatternField::Match(Slot(102)),
                to: PatternField::Bind("to".into()),
                kind: PatternField::Wildcard,
            }))),
        });
    }

    #[test]
    fn query_graph_round_trip() {
        round_trip(Frame {
            principal_hint: None,
            auth_proof: None,
            body: Body::Request(Request::Query(QueryOp::Graph(GraphQuery {
                title: PatternField::Match("criome request flow".into()),
            }))),
        });
    }

    #[test]
    fn query_kind_decl_round_trip() {
        round_trip(Frame {
            principal_hint: None,
            auth_proof: None,
            body: Body::Request(Request::Query(QueryOp::KindDecl(KindDeclQuery {
                name: PatternField::Wildcard,
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
                Node { name: "Alice".into() },
                Node { name: "Bob".into() },
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
        use crate::diagnostic::{Diagnostic, DiagnosticLevel};
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
}
