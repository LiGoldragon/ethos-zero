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
pub trait Processable<A: core::clone::Clone + core::marker::Send, B: serde::Serialize> {
    fn process(&self) -> alloc::string::String;
}
