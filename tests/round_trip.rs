use signal::{
    AuthorizedObjectInterest, AuthorizedObjectKind, AuthorizedObjectReference, ComponentKind,
    ComponentObjectInterest, HostName, InterestMatchable, NetworkEndpoint, NetworkPort,
    ObjectDigest, StandardSocket,
};

#[test]
fn authorized_object_interest_preserves_matching_semantics() {
    let reference = AuthorizedObjectReference {
        component_kind: ComponentKind::Criome,
        object_digest: ObjectDigest("contract-digest-fixture".into()),
        authorized_object_kind: AuthorizedObjectKind::Contract,
    };
    assert!(reference.matches_interest(&AuthorizedObjectInterest::AnyAuthorizedObject));
    assert!(
        reference.matches_interest(&AuthorizedObjectInterest::Component(ComponentKind::Criome))
    );
    assert!(
        reference.matches_interest(&AuthorizedObjectInterest::ObjectKind(
            AuthorizedObjectKind::Contract
        ))
    );
    assert!(
        reference.matches_interest(&AuthorizedObjectInterest::ComponentObject(
            ComponentObjectInterest {
                component_kind: ComponentKind::Criome,
                authorized_object_kind: AuthorizedObjectKind::Contract,
            }
        ))
    );
    assert!(!reference.matches_interest(&AuthorizedObjectInterest::Component(ComponentKind::Mind)));
}

#[test]
fn shared_socket_archives_with_current_shape() {
    let socket = StandardSocket::NetworkSocket(NetworkEndpoint {
        host_name: HostName("prometheus.goldragon.criome".into()),
        network_port: NetworkPort(7474),
    });
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&socket).expect("archive");
    assert_eq!(
        rkyv::from_bytes::<StandardSocket, rkyv::rancor::Error>(&bytes).expect("restore"),
        socket
    );
}
