#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
pub trait Summarizable {
    fn summarize(&self) -> String;
}
#[rustfmt::skip]
pub trait Fillable {
    fn push(&mut self, input: String) -> std::result::Result<i64, super::SinkError>;
    fn drain(&mut self) -> std::vec::Vec<String>;
    fn create() -> Self
    where
        Self: Sized;
}
