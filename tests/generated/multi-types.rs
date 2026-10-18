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
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Report {
    pub string: String,
    pub integer_vector: std::vec::Vec<i64>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum SinkError {
    Closed,
    Full,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct LockId(pub i64);
#[rustfmt::skip]
const _: () = {
    fn assert_record_sized<T: Sized>() {}
    let _ = assert_record_sized::<Record>;
};
#[rustfmt::skip]
const _: () = {
    fn assert_report_sized<T: Sized>() {}
    let _ = assert_report_sized::<Report>;
};
#[rustfmt::skip]
const _: () = {
    fn assert_sinkerror_sized<T: Sized>() {}
    let _ = assert_sinkerror_sized::<SinkError>;
};
#[rustfmt::skip]
const _: () = {
    fn assert_lockid_sized<T: Sized>() {}
    let _ = assert_lockid_sized::<LockId>;
};
