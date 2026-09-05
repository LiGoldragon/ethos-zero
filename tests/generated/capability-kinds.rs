#![allow(dead_code)]
#![allow(clippy::redundant_closure)]
pub trait Summarizable {
    fn summarize(&self) -> protos::Text;
}
pub trait Fillable {
    fn push(
        &mut self,
        input: protos::Text,
    ) -> std::result::Result<protos::Integer, super::SinkError>;
    fn drain(&mut self) -> std::vec::Vec<protos::Text>;
    fn create() -> Self;
}
