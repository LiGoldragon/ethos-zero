#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub struct Node_Data {
    pub first_tree: std::boxed::Box<Tree>,
    pub second_tree: std::boxed::Box<Tree>,
}
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub enum Tree {
    Leaf(i64),
    Node(Node_Data),
    Many(std::vec::Vec<Tree>),
    Maybe(Option<std::boxed::Box<Tree>>),
}
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub struct Chain {
    pub string: String,
    pub chain_option: Option<std::boxed::Box<Chain>>,
}
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub struct Twin {
    pub first_twig: std::boxed::Box<Twig>,
    pub second_twig: std::boxed::Box<Twig>,
}
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub enum Twig {
    Tip,
    Grow(std::boxed::Box<Twin>),
}
pub type Forest = std::vec::Vec<Tree>;
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub struct Wrapped {
    pub integer_option: Option<i64>,
    pub string_integer_result: Result<String, i64>,
    pub string_option_vector: std::vec::Vec<std::option::Option<String>>,
}
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub enum A_Data {
    X,
    Y(i64),
}
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub struct B_Data {
    pub string: String,
}
#[derive(datom_codec::Datomizable, datom_codec::Compositional)]
pub enum Nested {
    A(A_Data),
    B(B_Data),
}
pub type Deep = std::vec::Vec<
    std::vec::Vec<std::vec::Vec<std::option::Option<std::result::Result<String, i64>>>>,
>;
