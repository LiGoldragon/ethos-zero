#![allow(dead_code, non_camel_case_types, non_snake_case)]
pub trait Streamable: super::Fillable {
    type Item: super::Serializable;
    const CAPACITY: i64;
    fn next(&mut self) -> std::option::Option<Self::Item>;
}
