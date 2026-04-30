//! Schema-introspection types — the descriptor every signal record
//! kind exposes via `#[derive(signal_derive::Schema)]`, plus the
//! `Kind` trait it implements and the `ALL_KINDS` catalogue
//! consumers walk.
//!
//! Designed in [mentci/reports/115](https://github.com/LiGoldragon/mentci/blob/main/reports/115-schema-derive-design-2026-04-30.md).
//! The macro lives in `signal-derive`; the types and the catalogue
//! live here.
//!
//! Consumers — mentci-lib's `CompiledSchema`, nexus-daemon's
//! renderer when wired — call `T::DESCRIPTOR` for any signal kind
//! to learn its shape, or walk `ALL_KINDS` to enumerate every
//! kind in the vocabulary. Resolution of cross-references inside
//! `FieldType::Record { kind_name }` is the consumer's job: look
//! up the named kind in `ALL_KINDS` to learn its shape.

/// Describe a record kind's shape — name + structural shape.
#[derive(Debug, Clone, Copy)]
pub struct KindDescriptor {
    pub name: &'static str,
    pub shape: KindShape,
}

/// Whether a kind is a record (named fields) or a closed-vocabulary
/// enum (unit variants).
#[derive(Debug, Clone, Copy)]
pub enum KindShape {
    Record { fields: &'static [FieldDescriptor] },
    Enum { variants: &'static [&'static str] },
}

/// One field of a record-shaped kind.
#[derive(Debug, Clone, Copy)]
pub struct FieldDescriptor {
    pub name: &'static str,
    pub field_type: FieldType,
    /// `Option<T>` field on the source type.
    pub is_optional: bool,
    /// `Vec<T>` field on the source type.
    pub is_list: bool,
}

/// Mechanically-derivable field-type categories.
///
/// Compound shapes (`Vec<Option<T>>`, `Option<Vec<T>>`) flatten
/// into `is_optional` + `is_list` flags on the `FieldDescriptor`;
/// see [reports/115 §1](https://github.com/LiGoldragon/mentci/blob/main/reports/115-schema-derive-design-2026-04-30.md#1--type--fieldtype-mapping)
/// for the mapping rules the derive applies.
#[derive(Debug, Clone, Copy)]
pub enum FieldType {
    /// `String`.
    Text,
    /// `bool`.
    Bool,
    /// Any of the integer primitives (`u8`..`u64`, `i8`..`i64`).
    Integer,
    /// `f32` or `f64`.
    Float,
    /// `Slot<Kind>` referencing a specific kind.
    SlotRef { of_kind: &'static str },
    /// `Slot<AnyKind>` — type-erased slot reference.
    AnyKind,
    /// Any other named type. The kind name lets the consumer
    /// resolve the referenced descriptor via `ALL_KINDS`.
    Record { kind_name: &'static str },
}

/// Every type that participates in the signal record vocabulary
/// implements this trait — usually via `#[derive(Schema)]` from
/// `signal-derive`. The `DESCRIPTOR` const carries the full shape
/// at compile time.
pub trait Kind {
    const DESCRIPTOR: KindDescriptor;
}

/// The full catalogue of signal record kinds. Consumers walk this
/// to enumerate everything in the vocabulary.
///
/// The list is hand-maintained — adding a new kind to signal
/// means adding a row here. That's the canonical record-of-kinds,
/// not a stop-gap; the durable shape is one source of truth for
/// what kinds exist.
pub const ALL_KINDS: &[KindDescriptor] = &[
    <crate::flow::Node as Kind>::DESCRIPTOR,
    <crate::flow::Edge as Kind>::DESCRIPTOR,
    <crate::flow::Graph as Kind>::DESCRIPTOR,
    <crate::flow::RelationKind as Kind>::DESCRIPTOR,
    <crate::flow::Ok as Kind>::DESCRIPTOR,
    <crate::identity::Principal as Kind>::DESCRIPTOR,
    <crate::tweaks::Tweaks as Kind>::DESCRIPTOR,
    <crate::style::Theme as Kind>::DESCRIPTOR,
    <crate::style::IntentToken as Kind>::DESCRIPTOR,
    <crate::style::KindStyle as Kind>::DESCRIPTOR,
    <crate::style::GlyphToken as Kind>::DESCRIPTOR,
    <crate::style::RelationKindStyle as Kind>::DESCRIPTOR,
    <crate::style::StrokeToken as Kind>::DESCRIPTOR,
    <crate::layout::Layout as Kind>::DESCRIPTOR,
    <crate::layout::NodePlacement as Kind>::DESCRIPTOR,
    <crate::layout::SizeIntent as Kind>::DESCRIPTOR,
    <crate::keybind::KeybindMap as Kind>::DESCRIPTOR,
    <crate::keybind::KeybindEntry as Kind>::DESCRIPTOR,
    <crate::keybind::ActionToken as Kind>::DESCRIPTOR,
];
