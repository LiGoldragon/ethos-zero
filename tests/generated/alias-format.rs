#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
pub type Short = String;
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
pub type SpiritGuardianProviderName = String;
#[rustfmt::skip]
pub type SpiritGuardianMaximumOutputTokens = i64;
