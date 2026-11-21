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
pub trait Summarizable {
    fn summarize(&self) -> alloc::string::String;
}
#[rustfmt::skip]
pub trait Fillable {
    fn push<N: Summarizable>(
        &mut self,
        input: N,
    ) -> core::result::Result<i64, super::SinkError>;
    fn drain(&mut self) -> alloc::vec::Vec<alloc::string::String>;
    fn create() -> Self
    where
        Self: Sized;
}
