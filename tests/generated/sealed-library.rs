#![allow(dead_code, non_camel_case_types, non_snake_case)]
#![allow(clippy::type_complexity)]
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable))]
pub struct VerifiedContext {
    pub(super) string: String,
    pub(super) integer: i64,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Rejection {
    Empty,
    Malformed(String),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Locks {
    pub string_vector: std::vec::Vec<String>,
}
#[rustfmt::skip]
pub trait Verifying {
    fn verify() -> std::result::Result<Self, Rejection>
    where
        Self: Sized;
}
#[rustfmt::skip]
pub trait Describing {
    fn describe(&self) -> String;
}
#[rustfmt::skip]
const _: () = {
    fn assert_verifiedcontext_verifying<T: Verifying>() {}
    let _ = assert_verifiedcontext_verifying::<VerifiedContext>;
    fn assert_verifiedcontext_describing<T: Describing>() {}
    let _ = assert_verifiedcontext_describing::<VerifiedContext>;
};
#[rustfmt::skip]
const _: () = {
    fn assert_rejection_describing<T: Describing>() {}
    let _ = assert_rejection_describing::<Rejection>;
};
#[rustfmt::skip]
const _: () = {
    fn assert_locks_describing<T: Describing>() {}
    let _ = assert_locks_describing::<Locks>;
};
