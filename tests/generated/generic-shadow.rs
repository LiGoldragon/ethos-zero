#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub struct A {
    pub string: String,
}
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub struct Holder {
    pub string: String,
    pub a: A,
}
