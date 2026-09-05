#![allow(dead_code)]
#![allow(clippy::redundant_closure)]
pub trait Processable<A: std::clone::Clone + std::marker::Send, B: serde::Serialize> {
    fn process(&self) -> protos::Text;
}
