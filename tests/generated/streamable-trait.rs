#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    unused_extern_crates,
    clippy::type_complexity
)]
#[rustfmt::skip]
extern crate alloc;
#[rustfmt::skip]
pub trait Streamable: super::Fillable {
    type Item: super::Serializable;
    const CAPACITY: i64;
    fn next(&mut self) -> core::option::Option<Self::Item>;
}
