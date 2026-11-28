#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    unused_extern_crates,
    clippy::type_complexity
)]
#[rustfmt::skip]
extern crate alloc;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Models(pub alloc::vec::Vec<models::signal::model::Variant>);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Memory {
    pub models: Models,
}
#[rustfmt::skip]
pub trait Memorizable {
    type Memory;
    fn recall(memory: &Self::Memory) -> &Self;
}
#[rustfmt::skip]
impl Memorizable for Models {
    type Memory = Memory;
    fn recall(memory: &Memory) -> &Self {
        &memory.models
    }
}
