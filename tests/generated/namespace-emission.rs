#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    clippy::type_complexity
)]
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
        Send(::signal::namespace_ordinary::signal::Input),
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
        Sent(::signal::namespace_ordinary::signal::Output),
        Failed(String),
    }
}
