#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum ComponentKind {
    Message,
    Router,
    Criome,
    Mind,
    Spirit,
    Persona,
    Agent,
    Mirror,
    Introspect,
    Harness,
    Terminal,
    System,
    Lojix,
    Orchestrate,
}
#[rustfmt::skip]
pub type ObjectDigest = String;
#[rustfmt::skip]
pub type SocketPath = String;
#[rustfmt::skip]
pub type HostName = String;
#[rustfmt::skip]
pub type NetworkPort = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct NetworkEndpoint {
    pub host_name: HostName,
    pub network_port: NetworkPort,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum StandardSocket {
    NetworkSocket(NetworkEndpoint),
    UnixSocket(SocketPath),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum AuthorizedObjectKind {
    Time,
    Operation,
    Contract,
    Agreement,
    Head,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct AuthorizedObjectReference {
    pub component_kind: ComponentKind,
    pub object_digest: ObjectDigest,
    pub authorized_object_kind: AuthorizedObjectKind,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct ComponentObjectInterest {
    pub component_kind: ComponentKind,
    pub authorized_object_kind: AuthorizedObjectKind,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum AuthorizedObjectInterest {
    Component(ComponentKind),
    ComponentObject(ComponentObjectInterest),
    AnyAuthorizedObject,
    ObjectKind(AuthorizedObjectKind),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct Differentiator {
    pub component_kind: ComponentKind,
    pub authorized_object_kind: AuthorizedObjectKind,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct ComponentClassification {
    pub differentiator: Differentiator,
    pub authorized_object_interest: AuthorizedObjectInterest,
}
