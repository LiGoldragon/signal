#![cfg(feature = "dotos-text")]

use dotos::{DotosDecode, DotosEncode, DotosSource};
use signal_standard::schema::lib::{
    z2VLyh, z2VQD6, z2VQaE, z2VSkP, z2VSyM, z2VTjK, z2VU3x, z2VWWD, z2VXNY, z2VaVE, z2Vbhy, z2VdWE,
    z2VduW,
};

fn round_trip<T>(value: T)
where
    T: DotosEncode + DotosDecode + Clone + PartialEq + std::fmt::Debug,
{
    let text = value.to_dotos();
    let recovered = DotosSource::new(&text).parse::<T>().expect("Dotos decodes");
    assert_eq!(recovered, value);
    assert_eq!(recovered.to_dotos(), text);
}

#[test]
fn every_component_kind_round_trips_without_readable_rust_aliases() {
    let roster = [
        z2VWWD::z2VPuL,
        z2VWWD::z2VPLF,
        z2VWWD::z2VSDw,
        z2VWWD::z2VUqs,
        z2VWWD::z2VZ4y,
        z2VWWD::z2VVh8,
        z2VWWD::z2VZ73,
        z2VWWD::z2VWoi,
        z2VWWD::z2VNYL,
        z2VWWD::z2VPk8,
        z2VWWD::z2VbTm,
        z2VWWD::z2VN71,
        z2VWWD::z2VN8F,
        z2VWWD::z2Vc9t,
    ];
    assert_eq!(roster.len(), 14);
    for component in roster {
        round_trip(component);
    }
}

#[test]
fn shared_vocabulary_round_trips_and_preserves_its_domain_logic() {
    for kind in [
        z2Vbhy::z2VPDv,
        z2Vbhy::z2Ve6d,
        z2Vbhy::z2VV79,
        z2Vbhy::z2VYDX,
        z2Vbhy::z2Vd4Q,
    ] {
        round_trip(kind);
    }

    let reference = z2VTjK::new(
        z2VWWD::z2VSDw,
        z2VSyM::new("contract-digest-fixture".to_owned()),
        z2Vbhy::z2Ve6d,
    );
    assert!(reference.matches_interest(&z2VQD6::z2VYnk));
    assert!(reference.matches_interest(&z2VQD6::z2VW1p(z2VWWD::z2VSDw)));
    assert!(reference.matches_interest(&z2VQD6::z2Ve8W(z2Vbhy::z2Ve6d)));
    assert!(
        reference.matches_interest(&z2VQD6::z2VNut(
            z2VdWE::new(z2VWWD::z2VSDw, z2Vbhy::z2Ve6d,)
        ))
    );
    assert!(!reference.matches_interest(&z2VQD6::z2VW1p(z2VWWD::z2VZ4y)));
    round_trip(reference);

    let local = z2VduW::z2VUkE(z2VXNY::new("/run/user/1000/criome.socket".to_owned()));
    assert_eq!(local.to_dotos(), "UnixSocket./run/user/1000/criome.socket");
    round_trip(local);

    let endpoint = z2VaVE::new(
        z2VLyh::new("prometheus.goldragon.criome".to_owned()),
        z2VQaE::new(7474),
    );
    assert_eq!(endpoint.field_0.as_str(), "prometheus.goldragon.criome");
    assert_eq!(endpoint.field_1.clone().into_u16(), 7474);
    round_trip(z2VduW::z2VNCH(endpoint));

    round_trip(z2VU3x::over_any(z2VSkP::new(
        z2VWWD::z2VNYL,
        z2Vbhy::z2Ve6d,
    )));
}

#[test]
fn rkyv_round_trip_uses_the_same_structural_shape() {
    let value = z2VU3x::over_any(z2VSkP::new(z2VWWD::z2VPuL, z2Vbhy::z2VYDX));
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&value).expect("archive shared vocabulary");
    let recovered =
        rkyv::from_bytes::<z2VU3x, rkyv::rancor::Error>(&bytes).expect("recover shared vocabulary");
    assert_eq!(recovered, value);
}
