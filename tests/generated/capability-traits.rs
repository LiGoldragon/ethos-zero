#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    clippy::type_complexity
)]
#[rustfmt::skip]
pub trait Summarizable {
    fn summarize(&self) -> String;
}
#[rustfmt::skip]
pub trait Fillable {
    fn push<N: Summarizable>(
        &mut self,
        input: N,
    ) -> std::result::Result<i64, super::SinkError>;
    fn drain(&mut self) -> std::vec::Vec<String>;
    fn create() -> Self
    where
        Self: Sized;
}
#[rustfmt::skip]
pub trait Indexed<AEthosParameter: super::Serializable> {
    type A;
}
#[rustfmt::skip]
pub trait A {}
#[rustfmt::skip]
pub trait Bounded<AEthosParameter: A> {}
#[rustfmt::skip]
pub trait SizedBounded<AEthosParameter: Sized> {}
#[rustfmt::skip]
pub trait UsesSizedBounded: SizedBounded<String> {
    type Item: SizedBounded<String>;
    fn use_item(&self, input: Self::Item) -> Self
    where
        Self: Sized;
}
#[rustfmt::skip]
pub trait GenericallyBounded<AEthosParameter: SizedBounded<String>> {}
#[rustfmt::skip]
pub trait N {}
#[rustfmt::skip]
pub trait UsesN {
    fn use_n<NEthosParameter: N>(&self, input: NEthosParameter) -> Self
    where
        Self: Sized;
}
#[rustfmt::skip]
const _: () = {
    fn assertion_0<T: SizedBounded<String>>() {}
    let _ = assertion_0::<String>;
};
