#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub struct Record {
    pub string: String,
    pub integer: i64,
}
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub struct Report {
    pub string: String,
    pub integer_vector: std::vec::Vec<i64>,
}
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub enum SinkError {
    Closed,
    Full,
}
#[rustfmt::skip]
pub type LockId = i64;
