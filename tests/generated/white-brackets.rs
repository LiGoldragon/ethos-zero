#![allow(dead_code, non_camel_case_types, non_snake_case)]
#![allow(clippy::type_complexity)]
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Point {
    pub first_integer: i64,
    pub second_integer: i64,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Label {
    pub string: String,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Path {
    pub point_vector: std::vec::Vec<Point>,
    pub label: Label,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Line_Data {
    pub first_point: Point,
    pub second_point: Point,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Shape {
    Dot(Point),
    Line(Line_Data),
    Polygon(std::vec::Vec<Point>),
}
#[rustfmt::skip]
pub trait Measuring {
    fn length(&self) -> i64;
}
#[rustfmt::skip]
const _: () = {
    fn assert_point_measuring<T: Measuring>() {}
    let _ = assert_point_measuring::<Point>;
};
#[rustfmt::skip]
const _: () = {
    fn assert_path_measuring<T: Measuring>() {}
    let _ = assert_path_measuring::<Path>;
};
#[rustfmt::skip]
const _: () = {
    fn assert_label_measuring<T: Measuring>() {}
    let _ = assert_label_measuring::<Label>;
};
#[rustfmt::skip]
const _: () = {
    fn assert_shape_measuring<T: Measuring>() {}
    let _ = assert_shape_measuring::<Shape>;
};
