#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    clippy::type_complexity
)]
#[rustfmt::skip]
pub trait Following {
    type Change;
    fn after(input_0: Self, input_1: Self::Change) -> Self
    where
        Self: Sized;
}
