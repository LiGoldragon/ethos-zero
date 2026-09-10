#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct Shared {
    pub name: Name,
}
pub type Name = String;
