#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Compositional, Clone, Debug, PartialEq)]
pub enum Query {
    Generate(Generation),
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Compositional, Clone, Debug, PartialEq)]
pub struct Unreadable_Data {
    pub first_string: String,
    pub second_string: String,
}
#[derive(datom_codec::Datomizable, datom_codec::Compositional, Clone, Debug, PartialEq)]
pub struct GenerationRejected_Data {
    pub string: String,
    pub path: datom_codec::Path,
    pub generation__error: Generation_Error,
}
#[derive(datom_codec::Datomizable, datom_codec::Compositional, Clone, Debug, PartialEq)]
pub struct Unwritable_Data {
    pub first_string: String,
    pub second_string: String,
}
#[derive(datom_codec::Datomizable, datom_codec::Compositional, Clone, Debug, PartialEq)]
pub enum Response {
    Generated(std::vec::Vec<String>),
    Arguments(i64),
    Malformed(datom_codec::Error),
    Unreadable(Unreadable_Data),
    GenerationRejected(GenerationRejected_Data),
    Unwritable(Unwritable_Data),
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Compositional, Clone, Debug, PartialEq)]
pub struct Generation {
    pub first_string: String,
    pub second_string: String,
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Compositional, Clone, Debug, PartialEq)]
pub struct Conceptual_Data {
    pub integer_vector: std::vec::Vec<i64>,
    pub problem: ethos_zero::Problem,
}
#[derive(datom_codec::Datomizable, datom_codec::Compositional, Clone, Debug, PartialEq)]
pub enum Generation_Error {
    Structural(protos::Error),
    Conceptual(Conceptual_Data),
}
