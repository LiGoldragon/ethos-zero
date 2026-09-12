#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
#[derive(
    datom_codec::Datomizable,
    datom_codec::Composing,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct Record {
    pub string: String,
    pub integer: i64,
}
#[rustfmt::skip]
#[derive(
    datom_codec::Datomizable,
    datom_codec::Composing,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct Report {
    pub string: String,
    pub integer_vector: std::vec::Vec<i64>,
}
#[rustfmt::skip]
#[derive(
    datom_codec::Datomizable,
    datom_codec::Composing,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub enum SinkError {
    Closed,
    Full,
}
#[rustfmt::skip]
pub type LockId = i64;
