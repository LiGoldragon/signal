use signal::{ComponentKind, NetworkEndpoint, StandardSocket};

#[test]
fn shared_socket_taxonomy_archives_without_text_dependencies() {
    let socket = StandardSocket::NetworkSocket(NetworkEndpoint {
        host_name: "mirror.test".into(),
        network_port: 443,
    });
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&socket).expect("archive shared socket");
    let restored = rkyv::from_bytes::<StandardSocket, rkyv::rancor::Error>(&bytes)
        .expect("restore shared socket");
    assert_eq!(restored, socket);
    assert_eq!(ComponentKind::Mirror, ComponentKind::Mirror);
}

#[cfg(feature = "datom")]
#[test]
fn shared_socket_taxonomy_round_trips_as_datom() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};
    let socket = StandardSocket::UnixSocket("/tmp/mirror.sock".into());
    let text = socket.datomize(vec![]).protosize().textualize();
    let restored = Potential::<StandardSocket>::from(text)
        .actualize(&mut Budget {
            remaining: 1024,
            reader: ReaderBudget { remaining: 1024 },
            depth: 0,
            maximum_depth: 1024,
        })
        .expect("actualize shared socket");
    assert_eq!(restored, socket);
}
