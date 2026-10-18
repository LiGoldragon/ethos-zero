#![allow(dead_code, non_camel_case_types, non_snake_case)]
#![allow(clippy::type_complexity)]
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Placed {
    pub integer_option: std::option::Option<i64>,
    pub integer: i64,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Score {
    pub decimal: datom_codec::Decimal,
    pub boolean: bool,
    pub meaning: datom_codec::Meaning,
}
#[rustfmt::skip]
const _: () = {
    fn assert_placed_sized<T: Sized>() {}
    let _ = assert_placed_sized::<Placed>;
};
#[rustfmt::skip]
const _: () = {
    fn assert_score_sized<T: Sized>() {}
    let _ = assert_score_sized::<Score>;
};
