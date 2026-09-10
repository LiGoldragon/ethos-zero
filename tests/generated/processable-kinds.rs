#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
pub trait Processable<A: std::clone::Clone + std::marker::Send, B: serde::Serialize> {
    fn process(&self) -> String;
}
