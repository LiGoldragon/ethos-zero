#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    clippy::type_complexity
)]
#[rustfmt::skip]
pub trait Publishing {
    type State;
    type Change;
    fn open(&self, input: Self::State) -> signal::Stream<Self::State, Self::Change>;
}
