#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub struct Record {
    pub string: String,
    pub entry_vector: std::vec::Vec<Entry>,
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub struct Entry {
    pub string: String,
    pub integer: i64,
}
