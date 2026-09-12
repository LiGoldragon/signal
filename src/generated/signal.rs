#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
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
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct NetworkEndpoint {
    pub host_name: HostName,
    pub network_port: NetworkPort,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum StandardSocket {
    NetworkSocket(NetworkEndpoint),
    UnixSocket(SocketPath),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum AuthorizedObjectKind {
    Time,
    Operation,
    Contract,
    Agreement,
    Head,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct AuthorizedObjectReference {
    pub component_kind: ComponentKind,
    pub object_digest: ObjectDigest,
    pub authorized_object_kind: AuthorizedObjectKind,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ComponentObjectInterest {
    pub component_kind: ComponentKind,
    pub authorized_object_kind: AuthorizedObjectKind,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum AuthorizedObjectInterest {
    Component(ComponentKind),
    ComponentObject(ComponentObjectInterest),
    AnyAuthorizedObject,
    ObjectKind(AuthorizedObjectKind),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Differentiator {
    pub component_kind: ComponentKind,
    pub authorized_object_kind: AuthorizedObjectKind,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ComponentClassification {
    pub differentiator: Differentiator,
    pub authorized_object_interest: AuthorizedObjectInterest,
}
#[rustfmt::skip]
pub type ExchangeId = i64;
#[rustfmt::skip]
pub type ContractDigest = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Handshake {
    pub contract_digest: ContractDigest,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum HandshakeRejection {
    ContractMismatch(ContractDigest),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum HandshakeReceipt {
    Greeted(ContractDigest),
    GreetingRefused(HandshakeRejection),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ExchangeFault {
    GreetingExpected,
    GreetingRepeated,
    ExchangeInUse,
    UnknownExchange,
    ExchangeLimit,
    UnreadableQuery,
    Lagged,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Conclusion {
    Completed,
    Faulted(ExchangeFault),
}
