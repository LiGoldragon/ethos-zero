#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub struct Result {
    pub string: String,
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub struct Box {
    pub string: String,
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub struct Tree {
    pub self_option: Option<std::boxed::Box<Self>>,
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub struct Choice_Data_Item_Data {
    pub result: Result,
    pub r#box: Box,
}
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub enum Choice_Data {
    Item(Choice_Data_Item_Data),
}
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub enum Nested {
    Choice(Choice_Data),
}
