//! Schema-introspection types — `KindDescriptor`, `FieldDescriptor`,
//! `FieldType`, the `Kind` trait, and the `ALL_KINDS` catalogue.
//!
//! **Status: role under review.** Per criome's `ARCHITECTURE.md`
//! §10.2 + §10.3, the runtime authority for "what kinds exist"
//! is sema-resident records (Kind, Field, Variant, TypeExpression,
//! Localization), constructed by criome's init at engine boot.
//! These compile-time descriptor consts no longer have an obvious
//! consumer. The crate's continuation, repurposing, or retirement
//! is tracked under bd issue `mentci-next-4v6`. The mechanism
//! below stays in place pending decision.

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
/// into `is_optional` + `is_list` flags on the `FieldDescriptor`.
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
