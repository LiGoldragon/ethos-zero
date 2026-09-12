#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq)]
pub struct Structural_Error {
    pub extent: protos::Extent,
    pub problem: protos::Problem,
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq)]
pub struct Conceptual_Data {
    pub integer_vector: std::vec::Vec<i64>,
    pub problem: Problem,
}
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq)]
pub enum Error {
    Structural(Structural_Error),
    Conceptual(Conceptual_Data),
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq)]
pub struct Arity_Data {
    pub first_integer: i64,
    pub second_integer: i64,
}
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq)]
pub enum Problem {
    Root,
    Arity(Arity_Data),
    Expected(Form),
    Separator(protos::Separator),
    Name(String),
    Duplicate(String),
    Undeclared(String),
    Cycle(String),
    Yield,
    Empty,
    Depth,
    Role(String),
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq)]
pub enum Form {
    File,
    Section,
    Import,
    Name,
    Declaration,
    Variant,
    Reference,
    Constraint,
    Kind,
    Capability,
    Constant,
    Association,
}
