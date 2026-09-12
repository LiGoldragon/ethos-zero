#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
#[derive(
    datom_codec::Datomizable,
    datom_codec::Composing,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct Node_Data {
    pub first_tree: std::boxed::Box<Tree>,
    pub second_tree: std::boxed::Box<Tree>,
}
#[derive(
    datom_codec::Datomizable,
    datom_codec::Composing,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub enum Tree {
    Leaf(i64),
    Node(Node_Data),
    Many(std::vec::Vec<Tree>),
    Maybe(Option<std::boxed::Box<Tree>>),
}
#[rustfmt::skip]
#[derive(
    datom_codec::Datomizable,
    datom_codec::Composing,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct Chain {
    pub string: String,
    pub chain_option: Option<std::boxed::Box<Chain>>,
}
#[rustfmt::skip]
#[derive(
    datom_codec::Datomizable,
    datom_codec::Composing,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct Twin {
    pub first_twig: std::boxed::Box<Twig>,
    pub second_twig: std::boxed::Box<Twig>,
}
#[rustfmt::skip]
#[derive(
    datom_codec::Datomizable,
    datom_codec::Composing,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub enum Twig {
    Tip,
    Grow(std::boxed::Box<Twin>),
}
#[rustfmt::skip]
pub type Forest = std::vec::Vec<Tree>;
#[rustfmt::skip]
#[derive(
    datom_codec::Datomizable,
    datom_codec::Composing,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct Wrapped {
    pub integer_option: Option<i64>,
    pub string_integer_result: Result<String, i64>,
    pub string_option_vector: std::vec::Vec<std::option::Option<String>>,
}
#[rustfmt::skip]
#[derive(
    datom_codec::Datomizable,
    datom_codec::Composing,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub enum A_Data {
    X,
    Y(i64),
}
#[derive(
    datom_codec::Datomizable,
    datom_codec::Composing,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct B_Data {
    pub string: String,
}
#[derive(
    datom_codec::Datomizable,
    datom_codec::Composing,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub enum Nested {
    A(A_Data),
    B(B_Data),
}
#[rustfmt::skip]
pub type Deep = std::vec::Vec<
    std::vec::Vec<std::vec::Vec<std::option::Option<std::result::Result<String, i64>>>>,
>;
