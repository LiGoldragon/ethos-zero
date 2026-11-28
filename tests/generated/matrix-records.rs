#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    clippy::type_complexity
)]
#[rustfmt::skip]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "datom",
    derive(::datom_codec::Datomizable, ::datom_codec::Composing)
)]
pub struct Watch {
    pub flow_id: ::flow::FlowId,
    pub string: ::std::string::String,
}
#[rustfmt::skip]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "datom",
    derive(::datom_codec::Datomizable, ::datom_codec::Composing)
)]
pub struct Configured(pub ::core::option::Option<Watch>);
#[rustfmt::skip]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "datom",
    derive(::datom_codec::Datomizable, ::datom_codec::Composing)
)]
pub struct Memory {
    pub watch: Watch,
    pub configured: Configured,
    pub event: ::flow::Event,
}
#[rustfmt::skip]
pub trait Memorizable {
    type Memory;
    fn recall(memory: &Self::Memory) -> &Self;
}
#[rustfmt::skip]
impl Memorizable for Watch {
    type Memory = Memory;
    fn recall(memory: &Memory) -> &Self {
        &memory.watch
    }
}
#[rustfmt::skip]
impl Memorizable for Configured {
    type Memory = Memory;
    fn recall(memory: &Memory) -> &Self {
        &memory.configured
    }
}
#[rustfmt::skip]
impl Memorizable for ::flow::Event {
    type Memory = Memory;
    fn recall(memory: &Memory) -> &Self {
        &memory.event
    }
}
