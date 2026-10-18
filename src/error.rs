#![allow(dead_code, non_camel_case_types, non_snake_case)]
#![allow(clippy::type_complexity)]
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Structural_Error {
    pub extent: protos::Extent,
    pub problem: protos::Problem,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Conceptual_Data {
    pub integer_vector: std::vec::Vec<i64>,
    pub problem: Problem,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Error {
    Structural(Structural_Error),
    Conceptual(Conceptual_Data),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Arity_Data {
    pub first_integer: i64,
    pub second_integer: i64,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Problem {
    Root,
    Arity(Arity_Data),
    Expected(Form),
    Separator(protos::Separator),
    Name(String),
    Duplicate(String),
    Undeclared(String),
    Cycle(String),
    Yield,
    Empty,
    Depth,
    Role(String),
    TraitWanted(String),
    Intrinsic(String),
    Case(String),
    Renamed(String),
    Sealed(String),
    Shared(String),
    Foreign(String),
    SinglePosition(String),
    SingleVariant(String),
    Traitless(String),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Form {
    File,
    Section,
    Import,
    Name,
    Declaration,
    Variant,
    Reference,
    Constraint,
    Trait,
    Capability,
    Constant,
    Association,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Location {
    pub line: Line,
    pub column: Column,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Line(pub i64);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Column(pub i64);
#[rustfmt::skip]
const _: () = {
    fn assert_structural_error_sized<T: Sized>() {}
    let _ = assert_structural_error_sized::<Structural_Error>;
};
#[rustfmt::skip]
const _: () = {
    fn assert_error_sized<T: Sized>() {}
    let _ = assert_error_sized::<Error>;
};
#[rustfmt::skip]
const _: () = {
    fn assert_problem_sized<T: Sized>() {}
    let _ = assert_problem_sized::<Problem>;
};
#[rustfmt::skip]
const _: () = {
    fn assert_form_sized<T: Sized>() {}
    let _ = assert_form_sized::<Form>;
};
#[rustfmt::skip]
const _: () = {
    fn assert_location_sized<T: Sized>() {}
    let _ = assert_location_sized::<Location>;
};
#[rustfmt::skip]
const _: () = {
    fn assert_line_sized<T: Sized>() {}
    let _ = assert_line_sized::<Line>;
};
#[rustfmt::skip]
const _: () = {
    fn assert_column_sized<T: Sized>() {}
    let _ = assert_column_sized::<Column>;
};
