#![allow(dead_code, non_camel_case_types, non_snake_case)]
#![allow(clippy::type_complexity)]
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Short(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct OptionalSpiritGuardianProviderName(
    pub std::option::Option<SpiritGuardianProviderName>,
);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct OptionalSpiritGuardianMaximumOutputTokens(
    pub std::option::Option<SpiritGuardianMaximumOutputTokens>,
);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Nested(
    pub std::vec::Vec<std::option::Option<std::result::Result<String, i64>>>,
);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SpiritGuardianProviderName(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SpiritGuardianMaximumOutputTokens(pub i64);
#[rustfmt::skip]
const _: () = {
    fn assert_short_sized<T: Sized>() {}
    let _ = assert_short_sized::<Short>;
};
#[rustfmt::skip]
const _: () = {
    fn assert_optionalspiritguardianprovidername_sized<T: Sized>() {}
    let _ = assert_optionalspiritguardianprovidername_sized::<
        OptionalSpiritGuardianProviderName,
    >;
};
#[rustfmt::skip]
const _: () = {
    fn assert_optionalspiritguardianmaximumoutputtokens_sized<T: Sized>() {}
    let _ = assert_optionalspiritguardianmaximumoutputtokens_sized::<
        OptionalSpiritGuardianMaximumOutputTokens,
    >;
};
#[rustfmt::skip]
const _: () = {
    fn assert_nested_sized<T: Sized>() {}
    let _ = assert_nested_sized::<Nested>;
};
#[rustfmt::skip]
const _: () = {
    fn assert_spiritguardianprovidername_sized<T: Sized>() {}
    let _ = assert_spiritguardianprovidername_sized::<SpiritGuardianProviderName>;
};
#[rustfmt::skip]
const _: () = {
    fn assert_spiritguardianmaximumoutputtokens_sized<T: Sized>() {}
    let _ = assert_spiritguardianmaximumoutputtokens_sized::<
        SpiritGuardianMaximumOutputTokens,
    >;
};
