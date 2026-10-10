#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Vec(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
#[rkyv(
    serialize_bounds(
        __S:rkyv::ser::Writer+rkyv::ser::Allocator,
        __S::Error:rkyv::rancor::Source
    )
)]
#[rkyv(deserialize_bounds(__D::Error:rkyv::rancor::Source))]
#[rkyv(
    bytecheck(
        bounds(__C:rkyv::validation::ArchiveContext, __C::Error:rkyv::rancor::Source)
    )
)]
pub struct Tree {
    #[rkyv(omit_bounds)]
    pub tree_option: std::option::Option<std::boxed::Box<Self>>,
    pub vec_integer_result: std::result::Result<Vec, i64>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Choice_Data_Item_Data {
    pub vec: Vec,
    pub integer: i64,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Choice_Data {
    Item(Choice_Data_Item_Data),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Nested {
    Choice(Choice_Data),
}
