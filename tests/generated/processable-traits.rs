#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    clippy::type_complexity
)]
#[rustfmt::skip]
pub trait Processable<
    A: ::core::clone::Clone + ::core::marker::Send,
    B: ::serde::Serialize,
> {
    fn process(&self) -> ::std::string::String;
}
