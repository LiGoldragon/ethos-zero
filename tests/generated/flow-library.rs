#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
pub type FlowId = i64;
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Voice {
    Psyche(Rank),
    Mind(Rank),
    Field(Rank),
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Rank {
    Primary,
    Secondary,
    Tertiary,
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Event {
    Started,
    ToolUsed(String),
    Stopped,
}
