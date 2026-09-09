#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub struct A_Data_X_Data {
    pub string: String,
}
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub enum A_Data {
    X(A_Data_X_Data),
}
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub struct B_Data_X_Data {
    pub integer: i64,
}
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub enum B_Data {
    X(B_Data_X_Data),
}
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub enum Outer {
    A(A_Data),
    B(B_Data),
}
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub struct PathOverlap_Data {
    pub first_string: String,
    pub second_string: String,
}
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub enum Rejection {
    PathOverlap(PathOverlap_Data),
}
