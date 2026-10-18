#![allow(dead_code, non_camel_case_types, non_snake_case)]
#![allow(clippy::type_complexity)]
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Record {
    pub string: String,
    pub integer: i64,
}
#[rustfmt::skip]
const _: () = {
    fn assert_record_sized<T: Sized>() {}
    let _ = assert_record_sized::<Record>;
};
