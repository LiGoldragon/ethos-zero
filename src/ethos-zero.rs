#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct Generation {
    pub first_string: String,
    pub second_string: String,
}
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct Conceptual_Data {
    pub integer_vector: std::vec::Vec<i64>,
    pub problem: ethos_zero::Problem,
}
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum Generation_Error {
    Structural(protos::Error),
    Conceptual(Conceptual_Data),
}
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum Query {
    Generate(Generation),
}
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct Unreadable_Data {
    pub first_string: String,
    pub second_string: String,
}
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct GenerationRejected_Data {
    pub string: String,
    pub path: datom_codec::Path,
    pub generation__error: Generation_Error,
}
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct Unwritable_Data {
    pub first_string: String,
    pub second_string: String,
}
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum Response {
    Generated(std::vec::Vec<String>),
    Arguments(i64),
    Malformed(datom_codec::Error),
    Unreadable(Unreadable_Data),
    GenerationRejected(GenerationRejected_Data),
    Unwritable(Unwritable_Data),
}
