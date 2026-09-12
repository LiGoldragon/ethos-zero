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
pub struct Placed {
    pub integer_option: Option<i64>,
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
pub struct Score {
    pub decimal: datom_codec::Decimal,
    pub boolean: bool,
    pub meaning: datom_codec::Meaning,
}
