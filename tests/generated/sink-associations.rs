#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    clippy::type_complexity
)]
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Sink {
    pub string: String,
    pub string_vector: std::vec::Vec<String>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum SinkError {
    Closed,
    Full,
}
#[rustfmt::skip]
const _: () = {
    fn assertion_0<T: super::Summarizable>() {}
    let _ = assertion_0::<Sink>;
    fn assertion_1<T: super::Fillable>() {}
    let _ = assertion_1::<Sink>;
};
