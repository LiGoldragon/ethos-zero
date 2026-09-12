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
