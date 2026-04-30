//! Schema-derive smoke tests.
//!
//! Each test verifies the descriptor a `#[derive(Schema)]` emits
//! matches the type's actual shape. Catches regressions when the
//! derive's lowering rules change or when a record kind grows
//! a field.

use signal::{
    ALL_KINDS, Edge, FieldType, Graph, Kind, KindShape, Node, RelationKind,
};

#[test]
fn node_descriptor_has_one_text_field_named_name() {
    let desc = Node::DESCRIPTOR;
    assert_eq!(desc.name, "Node");
    let fields = match desc.shape {
        KindShape::Record { fields } => fields,
        _ => panic!("Node should be Record-shaped"),
    };
    assert_eq!(fields.len(), 1);
    assert_eq!(fields[0].name, "name");
    assert!(matches!(fields[0].field_type, FieldType::Text));
    assert!(!fields[0].is_optional);
    assert!(!fields[0].is_list);
}

#[test]
fn edge_descriptor_routes_slot_kinds_to_node_and_relation_to_record() {
    let desc = Edge::DESCRIPTOR;
    assert_eq!(desc.name, "Edge");
    let fields = match desc.shape {
        KindShape::Record { fields } => fields,
        _ => panic!("Edge should be Record-shaped"),
    };
    assert_eq!(fields.len(), 3);

    assert_eq!(fields[0].name, "from");
    assert!(matches!(
        fields[0].field_type,
        FieldType::SlotRef { of_kind: "Node" }
    ));

    assert_eq!(fields[1].name, "to");
    assert!(matches!(
        fields[1].field_type,
        FieldType::SlotRef { of_kind: "Node" }
    ));

    assert_eq!(fields[2].name, "kind");
    assert!(matches!(
        fields[2].field_type,
        FieldType::Record { kind_name: "RelationKind" }
    ));
}

#[test]
fn graph_descriptor_sees_lists_of_kind_typed_slots() {
    let desc = Graph::DESCRIPTOR;
    assert_eq!(desc.name, "Graph");
    let fields = match desc.shape {
        KindShape::Record { fields } => fields,
        _ => panic!("Graph should be Record-shaped"),
    };
    assert_eq!(fields.len(), 4);

    assert_eq!(fields[0].name, "title");
    assert!(matches!(fields[0].field_type, FieldType::Text));

    assert_eq!(fields[1].name, "nodes");
    assert!(fields[1].is_list);
    assert!(matches!(
        fields[1].field_type,
        FieldType::SlotRef { of_kind: "Node" }
    ));

    assert_eq!(fields[2].name, "edges");
    assert!(fields[2].is_list);
    assert!(matches!(
        fields[2].field_type,
        FieldType::SlotRef { of_kind: "Edge" }
    ));

    assert_eq!(fields[3].name, "subgraphs");
    assert!(fields[3].is_list);
    assert!(matches!(
        fields[3].field_type,
        FieldType::SlotRef { of_kind: "Graph" }
    ));
}

#[test]
fn relation_kind_descriptor_lists_all_nine_variants() {
    let desc = RelationKind::DESCRIPTOR;
    assert_eq!(desc.name, "RelationKind");
    let variants = match desc.shape {
        KindShape::Enum { variants } => variants,
        _ => panic!("RelationKind should be Enum-shaped"),
    };
    assert_eq!(
        variants,
        &[
            "Flow",
            "DependsOn",
            "Contains",
            "References",
            "Produces",
            "Consumes",
            "Calls",
            "Implements",
            "IsA",
        ]
    );
}

#[test]
fn all_kinds_catalogue_covers_every_kind_with_a_schema_derive() {
    let names: Vec<&str> = ALL_KINDS.iter().map(|k| k.name).collect();
    // Every kind that derives Schema should appear in ALL_KINDS;
    // every entry in ALL_KINDS should match a deriving kind.
    assert!(names.contains(&"Node"));
    assert!(names.contains(&"Edge"));
    assert!(names.contains(&"Graph"));
    assert!(names.contains(&"RelationKind"));
    assert!(names.contains(&"Principal"));
    assert!(names.contains(&"Tweaks"));
    assert!(names.contains(&"Theme"));
    assert!(names.contains(&"IntentToken"));
    assert!(names.contains(&"Layout"));
    assert!(names.contains(&"NodePlacement"));
    assert!(names.contains(&"SizeIntent"));
    assert!(names.contains(&"KeybindMap"));
    assert!(names.contains(&"ActionToken"));
}
