#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq, Eq, Hash)]
pub enum State {
    Running,
    Ended,
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Flow {
    pub flow_id: flow::FlowId,
    pub voice: flow::Voice,
    pub state: State,
    pub event_vector: std::vec::Vec<flow::Event>,
}
