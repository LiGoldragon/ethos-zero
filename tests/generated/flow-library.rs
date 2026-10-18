#![allow(dead_code, non_camel_case_types, non_snake_case)]
#![allow(clippy::type_complexity)]
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct FlowId(pub i64);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Voice {
    Psyche(Rank),
    Mind(Rank),
    Field(Rank),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Rank {
    Primary,
    Secondary,
    Tertiary,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Event {
    Started,
    ToolUsed(String),
    Stopped,
}
#[rustfmt::skip]
const _: () = {
    fn assert_flowid_sized<T: Sized>() {}
    let _ = assert_flowid_sized::<FlowId>;
};
#[rustfmt::skip]
const _: () = {
    fn assert_voice_sized<T: Sized>() {}
    let _ = assert_voice_sized::<Voice>;
};
#[rustfmt::skip]
const _: () = {
    fn assert_rank_sized<T: Sized>() {}
    let _ = assert_rank_sized::<Rank>;
};
#[rustfmt::skip]
const _: () = {
    fn assert_event_sized<T: Sized>() {}
    let _ = assert_event_sized::<Event>;
};
