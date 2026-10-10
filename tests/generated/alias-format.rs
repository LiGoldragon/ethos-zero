#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Short(pub String);
#[rustfmt::skip]
pub type OptionalSpiritGuardianProviderName = std::option::Option<
    SpiritGuardianProviderName,
>;
#[rustfmt::skip]
pub type OptionalSpiritGuardianMaximumOutputTokens = std::option::Option<
    SpiritGuardianMaximumOutputTokens,
>;
#[rustfmt::skip]
pub type Nested = std::vec::Vec<std::option::Option<std::result::Result<String, i64>>>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SpiritGuardianProviderName(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SpiritGuardianMaximumOutputTokens(pub i64);
