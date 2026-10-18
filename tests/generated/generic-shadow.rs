#![allow(dead_code, non_camel_case_types, non_snake_case)]
#![allow(clippy::type_complexity)]
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct A(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Holder {
    pub string: String,
    pub a: A,
}
#[rustfmt::skip]
const _: () = {
    fn assert_a_sized<T: Sized>() {}
    let _ = assert_a_sized::<A>;
};
#[rustfmt::skip]
const _: () = {
    fn assert_holder_sized<T: Sized>() {}
    let _ = assert_holder_sized::<Holder>;
};
