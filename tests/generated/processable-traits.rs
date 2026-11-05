#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    clippy::type_complexity
)]
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Marker {
    Clonable,
    Other,
}
#[rustfmt::skip]
pub trait Processable<A: std::clone::Clone + std::marker::Send, B: serde::Serialize> {
    fn process(&self) -> String;
}
#[rustfmt::skip]
pub trait DirectClone {
    fn clone_it<N: std::clone::Clone>(&self, input: N) -> Self
    where
        Self: Sized;
    fn send_it<N: std::marker::Send>(&self, input: N) -> Self
    where
        Self: Sized;
}
