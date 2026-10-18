#![allow(dead_code, non_camel_case_types, non_snake_case)]
#![allow(clippy::type_complexity)]
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Home(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Login(pub std::vec::Vec<String>);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Capsule {
    pub home: Home,
    pub login: Login,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Start_Data {
    pub voice: flow::Voice,
    pub capsule: Capsule,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Record_Data {
    pub flow_id: flow::FlowId,
    pub event: flow::Event,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Operation {
    Start(Start_Data),
    Record(Record_Data),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Outcome {
    Started(flow::FlowId),
    Recorded,
    Failed(String),
}
#[rustfmt::skip]
const _: () = {
    fn assert_capsule_sized<T: Sized>() {}
    let _ = assert_capsule_sized::<Capsule>;
};
#[rustfmt::skip]
const _: () = {
    fn assert_home_sized<T: Sized>() {}
    let _ = assert_home_sized::<Home>;
};
#[rustfmt::skip]
const _: () = {
    fn assert_login_sized<T: Sized>() {}
    let _ = assert_login_sized::<Login>;
};
#[rustfmt::skip]
const _: () = {
    fn assert_operation_sized<T: Sized>() {}
    let _ = assert_operation_sized::<Operation>;
};
#[rustfmt::skip]
const _: () = {
    fn assert_outcome_sized<T: Sized>() {}
    let _ = assert_outcome_sized::<Outcome>;
};
