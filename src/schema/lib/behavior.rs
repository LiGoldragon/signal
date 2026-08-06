// Producer-owned structural behavior for the authority-verified shared-standard Interface.
//
// The strict bootstrap projection owns every structural type below. This file
// owns only behavior the current bootstrap language cannot yet express:
// structural runtime traits, Dotos projection, rkyv projection, and the small
// domain operations that make this vocabulary useful to its consumers.

use rkyv::{
    Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize,
    rancor::Source as _,
};

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
#[rkyv(serialize_bounds(
    __S: rkyv::ser::Writer + rkyv::ser::Allocator,
    __S::Error: rkyv::rancor::Source,
))]
#[rkyv(deserialize_bounds(__D::Error: rkyv::rancor::Source))]
#[rkyv(bytecheck(bounds(__C: rkyv::validation::ArchiveContext)))]
#[doc(hidden)]
pub enum WireValue {
    Text(std::string::String), Integer(u64), Boolean(bool),
    Sequence(#[rkyv(omit_bounds)] Vec<WireValue>),
    Absent, Present(#[rkyv(omit_bounds)] Box<WireValue>),
    Product(#[rkyv(omit_bounds)] Vec<WireValue>),
    Variant { ordinal: u16, #[rkyv(omit_bounds)] fields: Vec<WireValue> },
}
#[derive(Debug, thiserror::Error)]
#[error("structural wire value does not match the authority-verified Interface")]
#[doc(hidden)]
pub struct WireShapeError;

/// Current-stage structural behavior shared by Interfaces that import these
/// producer-owned types.
#[doc(hidden)]
pub trait WireShape: Sized {
    fn to_wire(&self) -> WireValue;
    fn from_wire(value: WireValue) -> Result<Self, WireShapeError>;
}

impl WireShape for std::string::String {
    fn to_wire(&self) -> WireValue { WireValue::Text(self.clone()) }
    fn from_wire(value: WireValue) -> Result<Self, WireShapeError> { match value { WireValue::Text(value) => Ok(value), _ => Err(WireShapeError) } }
}
impl WireShape for u64 {
    fn to_wire(&self) -> WireValue { WireValue::Integer(*self) }
    fn from_wire(value: WireValue) -> Result<Self, WireShapeError> { match value { WireValue::Integer(value) => Ok(value), _ => Err(WireShapeError) } }
}
impl WireShape for bool {
    fn to_wire(&self) -> WireValue { WireValue::Boolean(*self) }
    fn from_wire(value: WireValue) -> Result<Self, WireShapeError> { match value { WireValue::Boolean(value) => Ok(value), _ => Err(WireShapeError) } }
}
impl<Value: WireShape> WireShape for Vec<Value> {
    fn to_wire(&self) -> WireValue { WireValue::Sequence(self.iter().map(WireShape::to_wire).collect()) }
    fn from_wire(value: WireValue) -> Result<Self, WireShapeError> {
        let WireValue::Sequence(values) = value else { return Err(WireShapeError) };
        values.into_iter().map(Value::from_wire).collect()
    }
}
impl<Value: WireShape> WireShape for Option<Value> {
    fn to_wire(&self) -> WireValue { match self { Some(value) => WireValue::Present(Box::new(value.to_wire())), None => WireValue::Absent } }
    fn from_wire(value: WireValue) -> Result<Self, WireShapeError> {
        match value { WireValue::Present(value) => Ok(Some(Value::from_wire(*value)?)), WireValue::Absent => Ok(None), _ => Err(WireShapeError) }
    }
}
fn one_field(mut fields: Vec<WireValue>) -> Result<WireValue, WireShapeError> {
    if fields.len() != 1 { return Err(WireShapeError); }
    Ok(fields.pop().expect("one field checked"))
}

macro_rules! wire_traits {
    ($name:ident) => {
        impl Clone for $name { fn clone(&self) -> Self { Self::from_wire(self.to_wire()).expect("a projected value revalidates") } }
        impl std::fmt::Debug for $name { fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.to_wire().fmt(formatter) } }
        impl PartialEq for $name { fn eq(&self, other: &Self) -> bool { self.to_wire() == other.to_wire() } }
        impl Eq for $name {}
    };
}
macro_rules! wire_external_newtype {
    ($name:ident, $inner:ty) => {
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue { self.payload().to_wire() }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> { Ok(Self::new(<$inner as WireShape>::from_wire(value)?)) }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                dotos::DotosEncode::to_dotos(self.payload())
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                <$inner as dotos::DotosDecode>::from_dotos_block(block).map(Self::new)
            }
        }
    };
}
macro_rules! wire_struct {
    ($name:ident { $($field:ident: $field_type:ty),* $(,)? }) => {
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue { WireValue::Product(vec![$(self.$field.to_wire()),*]) }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> {
                let WireValue::Product(fields) = value else { return Err(WireShapeError) };
                let mut fields = fields.into_iter();
                let result = Self { $($field: <$field_type as WireShape>::from_wire(fields.next().ok_or(WireShapeError)?)?),* };
                if fields.next().is_some() { return Err(WireShapeError); }
                Ok(result)
            }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                dotos::Delimiter::Parenthesis.wrap([
                    $(dotos::DotosEncode::to_dotos(&self.$field)),*
                ])
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                let body = dotos::DotosBody::from_delimited(
                    block,
                    dotos::Delimiter::Parenthesis,
                    stringify!($name),
                )?;
                let expected = 0usize $(+ {
                    let _ = stringify!($field);
                    1usize
                })*;
                #[allow(unused_mut, unused_variables)]
                let mut fields = body.expect_fields(stringify!($name), expected)?.iter();
                Ok(Self {
                    $($field: <$field_type as dotos::DotosDecode>::from_dotos_block(
                        fields.next().expect("field count checked"),
                    )?),*
                })
            }
        }
    };
}
macro_rules! wire_enum {
    ($name:ident {
        unit { $($unit_ordinal:literal => $unit:ident : $unit_visible:literal),* $(,)? }
        unary { $($unary_ordinal:literal => $unary:ident($payload:ty) : $unary_visible:literal),* $(,)? }
    }) => {
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue {
                match self {
                    $(Self::$unit => WireValue::Variant { ordinal: $unit_ordinal, fields: Vec::new() },)*
                    $(Self::$unary(payload) => WireValue::Variant { ordinal: $unary_ordinal, fields: vec![payload.to_wire()] },)*
                }
            }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> {
                let WireValue::Variant { ordinal, fields } = value else { return Err(WireShapeError) };
                match ordinal {
                    $($unit_ordinal if fields.is_empty() => Ok(Self::$unit),)*
                    $($unary_ordinal => Ok(Self::$unary(<$payload as WireShape>::from_wire(one_field(fields)?)?)),)*
                    _ => Err(WireShapeError),
                }
            }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                match self {
                    $(Self::$unit => $unit_visible.to_owned(),)*
                    $(Self::$unary(payload) => format!(
                        "{}.{}",
                        $unary_visible,
                        dotos::DotosEncode::to_dotos(payload),
                    ),)*
                }
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                if let Some(variant) = block.demote_to_string() {
                    return match variant {
                        $($unit_visible => Ok(Self::$unit),)*
                        _ => Err(dotos::DotosDecodeError::UnknownVariant {
                            enum_name: stringify!($name),
                            variant: variant.to_owned(),
                        }),
                    };
                }
                let (head, payload) = block.as_application().ok_or(
                    dotos::DotosDecodeError::ExpectedAtom { type_name: stringify!($name) },
                )?;
                let _ = &payload;
                let variant = head.demote_to_string().ok_or(
                    dotos::DotosDecodeError::ExpectedAtom { type_name: stringify!($name) },
                )?;
                match variant {
                    $($unary_visible => Ok(Self::$unary(
                        <$payload as dotos::DotosDecode>::from_dotos_block(payload)?,
                    )),)*
                    _ => Err(dotos::DotosDecodeError::UnknownVariant {
                        enum_name: stringify!($name),
                        variant: variant.to_owned(),
                    }),
                }
            }
        }
    };
}

wire_enum!(z2VWWD { unit { 0 => z2VUqs : "Message", 1 => z2VZ4y : "Router", 2 => z2VSDw : "Criome", 3 => z2VPLF : "Mind", 4 => z2VPuL : "Spirit", 5 => z2Vc9t : "Persona", 6 => z2VNYL : "Agent", 7 => z2VVh8 : "Mirror", 8 => z2VbTm : "Introspect", 9 => z2VWoi : "Harness", 10 => z2VZ73 : "Terminal", 11 => z2VPk8 : "System", 12 => z2VN8F : "Lojix", 13 => z2VN71 : "Orchestrate" } unary {  } });
wire_struct!(z2VTjK { field_0: z2VWWD, field_1: z2VSyM, field_2: z2Vbhy });
wire_enum!(z2Vbhy { unit { 0 => z2VYDX : "Time", 1 => z2VPDv : "Operation", 2 => z2Ve6d : "Contract", 3 => z2VV79 : "Agreement", 4 => z2Vd4Q : "Head" } unary {  } });
wire_struct!(z2VdWE { field_0: z2VWWD, field_1: z2Vbhy });
wire_external_newtype!(z2VLyh, std::string::String);
wire_external_newtype!(z2VSyM, std::string::String);
wire_enum!(z2VduW { unit {  } unary { 0 => z2VNCH(z2VaVE) : "NetworkSocket", 1 => z2VUkE(z2VXNY) : "UnixSocket" } });
wire_enum!(z2VQD6 { unit { 2 => z2VYnk : "AnyAuthorizedObject" } unary { 0 => z2VW1p(z2VWWD) : "Component", 1 => z2VNut(z2VdWE) : "ComponentObject", 3 => z2Ve8W(z2Vbhy) : "ObjectKind" } });
wire_external_newtype!(z2VXNY, std::string::String);
wire_struct!(z2VaVE { field_0: z2VLyh, field_1: z2VQaE });
wire_external_newtype!(z2VQaE, u64);
wire_struct!(z2VU3x { field_0: z2VSkP, field_1: z2VQD6 });
wire_struct!(z2VSkP { field_0: z2VWWD, field_1: z2Vbhy });

macro_rules! archive_root {
    ($root:ident) => {
        impl Archive for $root {
            type Archived = <WireValue as Archive>::Archived;
            type Resolver = <WireValue as Archive>::Resolver;
            fn resolve(&self, resolver: Self::Resolver, out: rkyv::Place<Self::Archived>) {
                self.to_wire().resolve(resolver, out);
            }
        }
        impl<Serializer> RkyvSerialize<Serializer> for $root
        where
            Serializer: rkyv::rancor::Fallible + ?Sized,
            WireValue: RkyvSerialize<Serializer>,
        {
            fn serialize(
                &self,
                serializer: &mut Serializer,
            ) -> Result<Self::Resolver, Serializer::Error> {
                self.to_wire().serialize(serializer)
            }
        }
        impl<Deserializer> RkyvDeserialize<$root, Deserializer> for ArchivedWireValue
        where
            Deserializer: rkyv::rancor::Fallible + ?Sized,
            Deserializer::Error: rkyv::rancor::Source,
            ArchivedWireValue: RkyvDeserialize<WireValue, Deserializer>,
        {
            fn deserialize(
                &self,
                deserializer: &mut Deserializer,
            ) -> Result<$root, Deserializer::Error> {
                let wire = <ArchivedWireValue as RkyvDeserialize<
                    WireValue,
                    Deserializer,
                >>::deserialize(self, deserializer)?;
                <$root as WireShape>::from_wire(wire).map_err(Deserializer::Error::new)
            }
        }
    };
}
archive_root!(z2VWWD);
archive_root!(z2VTjK);
archive_root!(z2Vbhy);
archive_root!(z2VdWE);
archive_root!(z2VLyh);
archive_root!(z2VSyM);
archive_root!(z2VduW);
archive_root!(z2VQD6);
archive_root!(z2VXNY);
archive_root!(z2VaVE);
archive_root!(z2VQaE);
archive_root!(z2VU3x);
archive_root!(z2VSkP);
impl z2VSyM {
    pub fn as_str(&self) -> &str { self.0.as_str() }
}

impl z2VXNY {
    pub fn as_str(&self) -> &str { self.0.as_str() }
}

impl z2VLyh {
    pub fn as_str(&self) -> &str { self.0.as_str() }
}

impl z2VQaE {
    pub fn into_u16(self) -> u16 { self.0 as u16 }
}

impl z2VaVE {
    pub fn new(host: z2VLyh, port: z2VQaE) -> Self {
        Self { field_0: host, field_1: port }
    }
}

impl z2VSkP {
    pub fn new(component: z2VWWD, kind: z2Vbhy) -> Self {
        Self { field_0: component, field_1: kind }
    }
}

impl z2VdWE {
    pub fn new(component: z2VWWD, kind: z2Vbhy) -> Self {
        Self { field_0: component, field_1: kind }
    }
}

impl z2VTjK {
    pub fn new(component: z2VWWD, digest: z2VSyM, kind: z2Vbhy) -> Self {
        Self { field_0: component, field_1: digest, field_2: kind }
    }

    pub fn matches_interest(&self, interest: &z2VQD6) -> bool {
        match interest {
            z2VQD6::z2VYnk => true,
            z2VQD6::z2VW1p(component) => self.field_0 == *component,
            z2VQD6::z2Ve8W(kind) => self.field_2 == *kind,
            z2VQD6::z2VNut(value) => {
                self.field_0 == value.field_0 && self.field_2 == value.field_1
            }
        }
    }
}

impl z2VU3x {
    pub fn new(differentiator: z2VSkP, advertises: z2VQD6) -> Self {
        Self { field_0: differentiator, field_1: advertises }
    }

    pub fn over_any(differentiator: z2VSkP) -> Self {
        Self::new(differentiator, z2VQD6::z2VYnk)
    }
}
