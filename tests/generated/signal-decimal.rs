#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    clippy::type_complexity
)]
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Measurement {
    pub decimal: datom_codec::Decimal,
}
pub mod signal {
    #[rustfmt::skip]
    #[derive(
        rkyv::Archive,
        rkyv::Serialize,
        rkyv::Deserialize,
        Clone,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[cfg_attr(
        feature = "datom",
        derive(datom_codec::Datomizable, datom_codec::Composing)
    )]
    pub enum Input {
        Measure(super::Measurement),
    }
    #[rustfmt::skip]
    #[derive(
        rkyv::Archive,
        rkyv::Serialize,
        rkyv::Deserialize,
        Clone,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[cfg_attr(
        feature = "datom",
        derive(datom_codec::Datomizable, datom_codec::Composing)
    )]
    pub enum Output {
        Measured(super::Measurement),
    }
}
