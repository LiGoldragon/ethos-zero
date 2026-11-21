#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    unused_extern_crates,
    clippy::type_complexity
)]
#[rustfmt::skip]
extern crate alloc;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
#[rkyv(
    serialize_bounds(
        __S:rkyv::ser::Writer+rkyv::ser::Allocator,
        __S::Error:rkyv::rancor::Source
    )
)]
#[rkyv(deserialize_bounds(__D::Error:rkyv::rancor::Source))]
#[rkyv(
    bytecheck(
        bounds(__C:rkyv::validation::ArchiveContext, __C::Error:rkyv::rancor::Source)
    )
)]
pub struct Node_Data {
    #[rkyv(omit_bounds)]
    pub first_tree: alloc::boxed::Box<Tree>,
    #[rkyv(omit_bounds)]
    pub second_tree: alloc::boxed::Box<Tree>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
#[rkyv(
    serialize_bounds(
        __S:rkyv::ser::Writer+rkyv::ser::Allocator,
        __S::Error:rkyv::rancor::Source
    )
)]
#[rkyv(deserialize_bounds(__D::Error:rkyv::rancor::Source))]
#[rkyv(
    bytecheck(
        bounds(__C:rkyv::validation::ArchiveContext, __C::Error:rkyv::rancor::Source)
    )
)]
pub enum Tree {
    Leaf(i64),
    Node(Node_Data),
    Many(#[rkyv(omit_bounds)] alloc::vec::Vec<Tree>),
    Maybe(#[rkyv(omit_bounds)] core::option::Option<alloc::boxed::Box<Tree>>),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
#[rkyv(
    serialize_bounds(
        __S:rkyv::ser::Writer+rkyv::ser::Allocator,
        __S::Error:rkyv::rancor::Source
    )
)]
#[rkyv(deserialize_bounds(__D::Error:rkyv::rancor::Source))]
#[rkyv(
    bytecheck(
        bounds(__C:rkyv::validation::ArchiveContext, __C::Error:rkyv::rancor::Source)
    )
)]
pub struct Chain {
    pub string: alloc::string::String,
    #[rkyv(omit_bounds)]
    pub chain_option: core::option::Option<alloc::boxed::Box<Chain>>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
#[rkyv(
    serialize_bounds(
        __S:rkyv::ser::Writer+rkyv::ser::Allocator,
        __S::Error:rkyv::rancor::Source
    )
)]
#[rkyv(deserialize_bounds(__D::Error:rkyv::rancor::Source))]
#[rkyv(
    bytecheck(
        bounds(__C:rkyv::validation::ArchiveContext, __C::Error:rkyv::rancor::Source)
    )
)]
pub struct Twin {
    #[rkyv(omit_bounds)]
    pub first_twig: Twig,
    #[rkyv(omit_bounds)]
    pub second_twig: Twig,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
#[rkyv(
    serialize_bounds(
        __S:rkyv::ser::Writer+rkyv::ser::Allocator,
        __S::Error:rkyv::rancor::Source
    )
)]
#[rkyv(deserialize_bounds(__D::Error:rkyv::rancor::Source))]
#[rkyv(
    bytecheck(
        bounds(__C:rkyv::validation::ArchiveContext, __C::Error:rkyv::rancor::Source)
    )
)]
pub enum Twig {
    Tip,
    Grow(#[rkyv(omit_bounds)] alloc::boxed::Box<Twin>),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
#[rkyv(
    serialize_bounds(
        __S:rkyv::ser::Writer+rkyv::ser::Allocator,
        __S::Error:rkyv::rancor::Source
    )
)]
#[rkyv(deserialize_bounds(__D::Error:rkyv::rancor::Source))]
#[rkyv(
    bytecheck(
        bounds(__C:rkyv::validation::ArchiveContext, __C::Error:rkyv::rancor::Source)
    )
)]
pub enum Knot {
    End,
    Loop(#[rkyv(omit_bounds)] Loop),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
#[rkyv(
    serialize_bounds(
        __S:rkyv::ser::Writer+rkyv::ser::Allocator,
        __S::Error:rkyv::rancor::Source
    )
)]
#[rkyv(deserialize_bounds(__D::Error:rkyv::rancor::Source))]
#[rkyv(
    bytecheck(
        bounds(__C:rkyv::validation::ArchiveContext, __C::Error:rkyv::rancor::Source)
    )
)]
pub struct Loop {
    #[rkyv(omit_bounds)]
    pub knot: alloc::boxed::Box<Knot>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Forest(pub alloc::vec::Vec<Tree>);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Wrapped {
    pub integer_option: core::option::Option<i64>,
    pub string_integer_result: core::result::Result<alloc::string::String, i64>,
    pub string_option_vector: alloc::vec::Vec<
        core::option::Option<alloc::string::String>,
    >,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum A_Data {
    X,
    Y(i64),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct B_Data {
    pub string: alloc::string::String,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Nested {
    A(A_Data),
    B(B_Data),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Deep(
    pub alloc::vec::Vec<
        alloc::vec::Vec<
            alloc::vec::Vec<
                core::option::Option<core::result::Result<alloc::string::String, i64>>,
            >,
        >,
    >,
);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Input {
    Plant(Tree),
    Twine(Twin),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Output {
    Planted(Forest),
    Twined(Twig),
}
#[rustfmt::skip]
#[allow(clippy::result_large_err, clippy::ptr_arg, clippy::borrowed_box)]
pub trait Answering {
    fn plant(&self, tree: &Tree) -> core::result::Result<Forest, Output>;
    fn twine(&self, twin: &Twin) -> core::result::Result<Twig, Output>;
}
#[rustfmt::skip]
pub trait Answerable {
    type Answer;
    fn answer<Nexus: Answering>(&self, nexus: &Nexus) -> Self::Answer;
}
#[rustfmt::skip]
impl Answerable for Input {
    type Answer = Output;
    fn answer<Nexus: Answering>(&self, nexus: &Nexus) -> Output {
        match self {
            Input::Plant(carried) => {
                nexus.plant(carried).map_or_else(|refusal| refusal, Output::Planted)
            }
            Input::Twine(carried) => {
                nexus.twine(carried).map_or_else(|refusal| refusal, Output::Twined)
            }
        }
    }
}
