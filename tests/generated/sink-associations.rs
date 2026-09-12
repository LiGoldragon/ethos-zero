#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Sink {
    pub string: String,
    pub string_vector: std::vec::Vec<String>,
}
#[rustfmt::skip]
#[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq, Eq, Hash)]
pub enum SinkError {
    Closed,
    Full,
}
#[rustfmt::skip]
const _: () = {
    fn assert_sink_summarizable<T: super::Summarizable>() {}
    let _ = assert_sink_summarizable::<Sink>;
    fn assert_sink_fillable<T: super::Fillable>() {}
    let _ = assert_sink_fillable::<Sink>;
};
