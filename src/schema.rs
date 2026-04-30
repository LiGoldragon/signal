//! Schema-introspection types — `KindDescriptor`, `FieldDescriptor`,
//! `FieldType`, the `Kind` trait, and the `ALL_KINDS` catalogue.
//!
//! **Role: bootstrap source, NOT runtime catalogue.**
//!
//! Per [mentci/reports/119](https://github.com/LiGoldragon/mentci/blob/main/reports/119-schema-in-sema-corrected-direction-2026-04-30.md):
//! the runtime authority for "what kinds exist" is **sema-resident
//! `KindDecl` records**, not these compile-time consts. These types
//! exist as the input to a build-time projection: the seed step
//! reads `ALL_KINDS` once at engine boot and asserts equivalent
//! `KindDecl` / `FieldDecl` / `VariantDecl` records into sema.
//! From then on, every consumer (mentci-lib's constructor flow,
//! the nexus renderer, agents) queries sema, never reads
//! `ALL_KINDS` directly.
//!
//! Why both: nota-codec needs the type knowledge at compile time
//! for wire encoding/decoding (it's baked into `NotaEnum` /
//! `NotaRecord` derives). Sema needs the same knowledge as data
//! for runtime introspection. Both come from the same source —
//! the Rust type definitions in this crate. Neither is the other's
//! authority. See [reports/119 §2.1](https://github.com/LiGoldragon/mentci/blob/main/reports/119-schema-in-sema-corrected-direction-2026-04-30.md#21--are-we-re-implementing-parts-of-nexus)
//! for the full reasoning.
//!
//! Tracked-as-known-wrong: the consumer-side reading `ALL_KINDS`
//! directly (instead of sema) — see beads `mentci-next-lvg`. The
//! `Kind` trait + `ALL_KINDS` const stay; only the consumer path
//! changes.

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
