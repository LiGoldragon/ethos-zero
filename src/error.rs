#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub struct Structural_Error {
    pub extent: protos::Extent,
    pub problem: protos::Problem,
}
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub struct Conceptual_Data {
    pub integer_vector: std::vec::Vec<i64>,
    pub problem: Problem,
}
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub enum Error {
    Structural(Structural_Error),
    Conceptual(Conceptual_Data),
}
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub struct Arity_Data {
    pub first_integer: i64,
    pub second_integer: i64,
}
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
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
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
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
