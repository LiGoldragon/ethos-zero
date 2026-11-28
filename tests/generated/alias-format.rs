#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    unused_extern_crates,
    clippy::type_complexity
)]
#[rustfmt::skip]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
    feature = "datom",
    derive(::datom_codec::Datomizable, ::datom_codec::Composing)
)]
pub struct Short(pub ::std::string::String);
#[rustfmt::skip]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "datom",
    derive(::datom_codec::Datomizable, ::datom_codec::Composing)
)]
pub struct OptionalSpiritGuardianProviderName(
    pub ::core::option::Option<SpiritGuardianProviderName>,
);
#[rustfmt::skip]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "datom",
    derive(::datom_codec::Datomizable, ::datom_codec::Composing)
)]
pub struct OptionalSpiritGuardianMaximumOutputTokens(
    pub ::core::option::Option<SpiritGuardianMaximumOutputTokens>,
);
#[rustfmt::skip]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "datom",
    derive(::datom_codec::Datomizable, ::datom_codec::Composing)
)]
pub struct Nested(
    pub ::std::vec::Vec<
        ::core::option::Option<::core::result::Result<::std::string::String, i64>>,
    >,
);
#[rustfmt::skip]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
    feature = "datom",
    derive(::datom_codec::Datomizable, ::datom_codec::Composing)
)]
pub struct SpiritGuardianProviderName(pub ::std::string::String);
#[rustfmt::skip]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
    feature = "datom",
    derive(::datom_codec::Datomizable, ::datom_codec::Composing)
)]
pub struct SpiritGuardianMaximumOutputTokens(pub i64);
