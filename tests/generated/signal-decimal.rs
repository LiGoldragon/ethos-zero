#![allow(dead_code, non_camel_case_types, non_snake_case)]
#![allow(clippy::type_complexity)]
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Measurement(pub datom_codec::Decimal);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Query {
    Measure(Measurement),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Response {
    Measured(Measurement),
}
#[rustfmt::skip]
const _: () = {
    fn assert_measurement_sized<T: Sized>() {}
    let _ = assert_measurement_sized::<Measurement>;
};
#[rustfmt::skip]
const _: () = {
    fn assert_query_sized<T: Sized>() {}
    let _ = assert_query_sized::<Query>;
};
#[rustfmt::skip]
const _: () = {
    fn assert_response_sized<T: Sized>() {}
    let _ = assert_response_sized::<Response>;
};
