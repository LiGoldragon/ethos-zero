#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    clippy::type_complexity
)]
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
pub struct Input {
    #[rkyv(omit_bounds)]
    pub input_option: std::option::Option<std::boxed::Box<Input>>,
}
pub mod signal {
    #[rustfmt::skip]
    #[derive(
        rkyv::Archive,
        rkyv::Serialize,
        rkyv::Deserialize,
        Clone,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[cfg_attr(
        feature = "datom",
        derive(datom_codec::Datomizable, datom_codec::Composing)
    )]
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
    pub enum Input {
        Leaf,
        Recur(#[rkyv(omit_bounds)] std::boxed::Box<Self>),
        Parent(super::Input),
    }
    #[rustfmt::skip]
    #[derive(
        rkyv::Archive,
        rkyv::Serialize,
        rkyv::Deserialize,
        Clone,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[cfg_attr(
        feature = "datom",
        derive(datom_codec::Datomizable, datom_codec::Composing)
    )]
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
    pub enum Output {
        Done,
        Again(#[rkyv(omit_bounds)] std::option::Option<std::boxed::Box<Self>>),
    }
}
