//! Explicit producer-owned bootstrap authority state for the ordinary shared-standard Interface.
//!
//! Every identity and canonical-order value below is an already-minted opaque
//! seat. None is derived from source spelling, position, or content.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AuthoritySeat {
    pub spelling: &'static str,
    pub local: u16,
    pub canonical: u64,
}

impl AuthoritySeat {
    pub const fn new(spelling: &'static str, local: u16, canonical: u64) -> Self {
        Self {
            spelling,
            local,
            canonical,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DeclarationSeat {
    pub owner_local: Option<u16>,
    pub spelling: &'static str,
    pub local: u16,
    pub canonical: u64,
}

impl DeclarationSeat {
    pub const fn new(
        owner_local: Option<u16>,
        spelling: &'static str,
        local: u16,
        canonical: u64,
    ) -> Self {
        Self {
            owner_local,
            spelling,
            local,
            canonical,
        }
    }
}

pub const AUTHORITY_IDENTITY: [u8; 32] = [
    148, 70, 1, 56, 9, 4, 96, 240, 171, 139, 84, 238, 151, 189, 189, 57, 202, 27, 42, 78, 61, 167,
    104, 116, 55, 129, 247, 91, 153, 227, 244, 140,
];
pub const AUTHORITY_REVISION: u64 = 1;
pub const GRAMMAR_DOCUMENT_LOCAL: u16 = 31089;
pub const GRAMMAR_SYNTAX_LOCAL: u16 = 15492;

pub const INTERFACE_SEAT: AuthoritySeat =
    AuthoritySeat::new("Interface", 18266, 0x024fd9ff7188cded);
pub const NEXUS_SEAT: AuthoritySeat = AuthoritySeat::new("Nexus", 440, 0x8433a4d243eaf485);
pub const SEMA_SEAT: AuthoritySeat = AuthoritySeat::new("Sema", 45259, 0x56f2a6f066ba396c);
pub const INPUT_SEAT: AuthoritySeat = AuthoritySeat::new("Input", 52209, 0xae5d2501d8c7abd8);
pub const OUTPUT_SEAT: AuthoritySeat = AuthoritySeat::new("Output", 30670, 0x28e72cba4f64fe4f);
pub const REFUSAL_SEAT: AuthoritySeat = AuthoritySeat::new("Refusal", 11424, 0xd6d22c1e4d99f268);
pub const STRING_SEAT: AuthoritySeat = AuthoritySeat::new("String", 19035, 0x3b50f4a94f123702);
pub const INTEGER_SEAT: AuthoritySeat = AuthoritySeat::new("Integer", 58134, 0x82bfa8ebe5246f1a);
pub const BOOLEAN_SEAT: AuthoritySeat = AuthoritySeat::new("Boolean", 55179, 0x0cbd07870db167fb);
pub const UNIT_SEAT: AuthoritySeat = AuthoritySeat::new("Unit", 27664, 0x8746bdda76278e39);
pub const VECTOR_SEAT: AuthoritySeat = AuthoritySeat::new("Vector", 64280, 0xc1e1c66716f89031);
pub const OPTION_SEAT: AuthoritySeat = AuthoritySeat::new("Option", 35982, 0x3771085baf34728a);
pub const MAP_SEAT: AuthoritySeat = AuthoritySeat::new("Map", 59595, 0x574e6d1b46e13b92);
pub const RESULT_SEAT: AuthoritySeat = AuthoritySeat::new("Result", 50538, 0x952be7936901ced6);
pub const STREAM_SEAT: AuthoritySeat = AuthoritySeat::new("Stream", 48650, 0x8ec388e2dd8f6e9e);
pub const STREAMIDENTITY_SEAT: AuthoritySeat =
    AuthoritySeat::new("StreamIdentity", 42791, 0x74ab7e5bc03c9634);

pub const RUST_VOCABULARY_LOCALS: [u16; 10] = [
    60989, 27749, 35676, 36795, 17493, 27131, 18607, 35218, 24199, 3963,
];

pub const DECLARATION_SEATS: &[DeclarationSeat] = &[
    DeclarationSeat::new(None, "ComponentKind", 36130, 0x13922767b24a1036),
    DeclarationSeat::new(Some(36130), "Spirit", 13923, 0x538752d3639de7af),
    DeclarationSeat::new(Some(36130), "Mind", 12004, 0x50854473e909bf72),
    DeclarationSeat::new(Some(36130), "Criome", 21730, 0x411d65a7992fe74a),
    DeclarationSeat::new(Some(36130), "Message", 30542, 0x1c217198c34dc5ee),
    DeclarationSeat::new(Some(36130), "Router", 44758, 0x352d3120c4253530),
    DeclarationSeat::new(Some(36130), "Mirror", 33399, 0x7cd18b3740b3742e),
    DeclarationSeat::new(Some(36130), "Terminal", 44878, 0xae1029d2673dcdde),
    DeclarationSeat::new(Some(36130), "Harness", 37145, 0xa342cde203312088),
    DeclarationSeat::new(Some(36130), "Agent", 9341, 0x7cbae7cbe5ff6977),
    DeclarationSeat::new(Some(36130), "System", 13389, 0xe072b8a743250a4e),
    DeclarationSeat::new(Some(36130), "Introspect", 52808, 0x846c5a1545b0e45f),
    DeclarationSeat::new(Some(36130), "Orchestrate", 7872, 0xfbc0dc0ee14391e5),
    DeclarationSeat::new(Some(36130), "Lojix", 7944, 0xf2619a21ba64541a),
    DeclarationSeat::new(Some(36130), "Persona", 55135, 0x6d67f7a4e1c95f4a),
    DeclarationSeat::new(None, "AuthorizedObjectKind", 53632, 0x3a836e7fe4ce9b4d),
    DeclarationSeat::new(Some(53632), "Operation", 11637, 0xb018b79966d8d907),
    DeclarationSeat::new(Some(53632), "Contract", 61674, 0xcbb2f17dd37ba537),
    DeclarationSeat::new(Some(53632), "Agreement", 31428, 0xd18c3e361de7ed52),
    DeclarationSeat::new(Some(53632), "Time", 41890, 0x6ef9ecd00616e6e9),
    DeclarationSeat::new(Some(53632), "Head", 58181, 0xdac0f0b82f4bbe1d),
    DeclarationSeat::new(None, "Differentiator", 23496, 0xffc3b45890268f2d),
    DeclarationSeat::new(None, "ComponentObjectInterest", 59679, 0x4178db398a1dc423),
    DeclarationSeat::new(None, "AuthorizedObjectInterest", 14953, 0xe127a234578a984c),
    DeclarationSeat::new(
        Some(14953),
        "AnyAuthorizedObject",
        43817,
        0xbe46f86c467a78c9,
    ),
    DeclarationSeat::new(Some(14953), "Component", 34483, 0x12975bb552c3c8d7),
    DeclarationSeat::new(Some(14953), "ObjectKind", 61783, 0xf022f58fe7abd021),
    DeclarationSeat::new(Some(14953), "ComponentObject", 10591, 0x48f265b35b6a3bf4),
    DeclarationSeat::new(None, "ObjectDigest", 24248, 0xce447c74f94a635f),
    DeclarationSeat::new(None, "SocketPath", 39049, 0xe790b1eba4da229d),
    DeclarationSeat::new(None, "HostName", 4084, 0x59e2eacdd9dba17d),
    DeclarationSeat::new(None, "NetworkPort", 16179, 0xf5ff9207edf4f892),
    DeclarationSeat::new(None, "NetworkEndpoint", 49529, 0xec8769291da41a62),
    DeclarationSeat::new(None, "StandardSocket", 61029, 0xdab6a1dcfa554131),
    DeclarationSeat::new(Some(61029), "UnixSocket", 30215, 0x8727048b6f3fcbe8),
    DeclarationSeat::new(Some(61029), "NetworkSocket", 8178, 0x7a2a31f05bea8756),
    DeclarationSeat::new(None, "AuthorizedObjectReference", 26798, 0x14c4f8eda6e9d0f2),
    DeclarationSeat::new(None, "ComponentClassification", 27879, 0xfc5ef498fe2f4a7f),
];
