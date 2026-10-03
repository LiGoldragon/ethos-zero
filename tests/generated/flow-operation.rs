#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
pub type Home = String;
#[rustfmt::skip]
pub type Login = std::vec::Vec<String>;
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Capsule {
    pub home: Home,
    pub login: Login,
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Start_Data {
    pub voice: flow::Voice,
    pub capsule: Capsule,
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Record_Data {
    pub flow_id: flow::FlowId,
    pub event: flow::Event,
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Operation {
    Start(Start_Data),
    Record(Record_Data),
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Outcome {
    Started(flow::FlowId),
    Recorded,
    Failed(String),
}
