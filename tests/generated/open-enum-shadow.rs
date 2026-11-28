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
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "datom",
    derive(::datom_codec::Datomizable, ::datom_codec::Composing)
)]
pub struct Pair {
    pub core: Core,
    pub flow: Flow,
    pub voice: ::flow::Voice,
}
#[rustfmt::skip]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
    feature = "datom",
    derive(::datom_codec::Datomizable, ::datom_codec::Composing)
)]
pub struct Mark(pub ::alloc::string::String);
#[rustfmt::skip]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "datom",
    derive(::datom_codec::Datomizable, ::datom_codec::Composing)
)]
pub enum Carried {
    Mark(Mark),
}
#[cfg(feature = "datom")]
#[rustfmt::skip]
pub trait Opening<Registry>: Sized {
    fn open(
        datom: &::datom_codec::Datom,
        budget: &mut ::datom_codec::Budget,
        registry: &Registry,
    ) -> ::core::result::Result<Self, ::datom_codec::Error>;
}
#[rustfmt::skip]
pub mod core {
    #[rustfmt::skip]
    #[derive(
        ::rkyv::Archive,
        ::rkyv::Serialize,
        ::rkyv::Deserialize,
        Clone,
        Debug,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Hash
    )]
    #[cfg_attr(
        feature = "datom",
        derive(::datom_codec::Datomizable, ::datom_codec::Composing)
    )]
    pub struct Name(pub ::alloc::string::String);
    #[rustfmt::skip]
    #[derive(
        ::rkyv::Archive,
        ::rkyv::Serialize,
        ::rkyv::Deserialize,
        Clone,
        Debug,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Hash
    )]
    #[cfg_attr(
        feature = "datom",
        derive(::datom_codec::Datomizable, ::datom_codec::Composing)
    )]
    pub struct Word(pub ::alloc::string::String);
    #[rustfmt::skip]
    #[derive(
        ::rkyv::Archive,
        ::rkyv::Serialize,
        ::rkyv::Deserialize,
        Clone,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[cfg_attr(
        feature = "datom",
        derive(::datom_codec::Datomizable, ::datom_codec::Composing)
    )]
    pub struct Namespace(pub ::alloc::vec::Vec<Word>);
    #[rustfmt::skip]
    #[derive(
        ::rkyv::Archive,
        ::rkyv::Serialize,
        ::rkyv::Deserialize,
        Clone,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[cfg_attr(
        feature = "datom",
        derive(::datom_codec::Datomizable, ::datom_codec::Composing)
    )]
    pub struct Import {
        pub namespace: Namespace,
        pub name_vector: ::alloc::vec::Vec<Name>,
    }
    #[rustfmt::skip]
    #[derive(
        ::rkyv::Archive,
        ::rkyv::Serialize,
        ::rkyv::Deserialize,
        Clone,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[cfg_attr(
        feature = "datom",
        derive(::datom_codec::Datomizable, ::datom_codec::Composing)
    )]
    #[rkyv(
        serialize_bounds(
            __S: ::rkyv::ser::Writer+::rkyv::ser::Allocator,
            __S::Error: ::rkyv::rancor::Source
        )
    )]
    #[rkyv(deserialize_bounds(__D::Error: ::rkyv::rancor::Source))]
    #[rkyv(
        bytecheck(
            bounds(
                __C: ::rkyv::validation::ArchiveContext,
                __C::Error: ::rkyv::rancor::Source
            )
        )
    )]
    pub struct Declaration {
        pub name: Name,
        #[rkyv(omit_bounds)]
        pub r#type: Type,
    }
    #[rustfmt::skip]
    #[derive(
        ::rkyv::Archive,
        ::rkyv::Serialize,
        ::rkyv::Deserialize,
        Clone,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[cfg_attr(
        feature = "datom",
        derive(::datom_codec::Datomizable, ::datom_codec::Composing)
    )]
    #[rkyv(
        serialize_bounds(
            __S: ::rkyv::ser::Writer+::rkyv::ser::Allocator,
            __S::Error: ::rkyv::rancor::Source
        )
    )]
    #[rkyv(deserialize_bounds(__D::Error: ::rkyv::rancor::Source))]
    #[rkyv(
        bytecheck(
            bounds(
                __C: ::rkyv::validation::ArchiveContext,
                __C::Error: ::rkyv::rancor::Source
            )
        )
    )]
    pub struct Variant {
        pub name: Name,
        #[rkyv(omit_bounds)]
        pub type_option: ::core::option::Option<Type>,
    }
    #[rustfmt::skip]
    #[derive(
        ::rkyv::Archive,
        ::rkyv::Serialize,
        ::rkyv::Deserialize,
        Clone,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[cfg_attr(
        feature = "datom",
        derive(::datom_codec::Datomizable, ::datom_codec::Composing)
    )]
    #[rkyv(
        serialize_bounds(
            __S: ::rkyv::ser::Writer+::rkyv::ser::Allocator,
            __S::Error: ::rkyv::rancor::Source
        )
    )]
    #[rkyv(deserialize_bounds(__D::Error: ::rkyv::rancor::Source))]
    #[rkyv(
        bytecheck(
            bounds(
                __C: ::rkyv::validation::ArchiveContext,
                __C::Error: ::rkyv::rancor::Source
            )
        )
    )]
    pub struct Applied_Data {
        pub name: Name,
        #[rkyv(omit_bounds)]
        pub type_vector: ::alloc::vec::Vec<Type>,
    }
    #[rustfmt::skip]
    #[derive(
        ::rkyv::Archive,
        ::rkyv::Serialize,
        ::rkyv::Deserialize,
        Clone,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[cfg_attr(
        feature = "datom",
        derive(::datom_codec::Datomizable, ::datom_codec::Composing)
    )]
    #[rkyv(
        serialize_bounds(
            __S: ::rkyv::ser::Writer+::rkyv::ser::Allocator,
            __S::Error: ::rkyv::rancor::Source
        )
    )]
    #[rkyv(deserialize_bounds(__D::Error: ::rkyv::rancor::Source))]
    #[rkyv(
        bytecheck(
            bounds(
                __C: ::rkyv::validation::ArchiveContext,
                __C::Error: ::rkyv::rancor::Source
            )
        )
    )]
    pub enum Type {
        Name(Name),
        Import(Import),
        Declaration(#[rkyv(omit_bounds)] ::alloc::boxed::Box<Declaration>),
        Struct(#[rkyv(omit_bounds)] ::alloc::vec::Vec<Type>),
        Enum(#[rkyv(omit_bounds)] ::alloc::vec::Vec<Variant>),
        Vector(#[rkyv(omit_bounds)] ::alloc::boxed::Box<Type>),
        Applied(Applied_Data),
    }
    #[rustfmt::skip]
    #[derive(
        ::rkyv::Archive,
        ::rkyv::Serialize,
        ::rkyv::Deserialize,
        Clone,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[cfg_attr(
        feature = "datom",
        derive(::datom_codec::Datomizable, ::datom_codec::Composing)
    )]
    pub enum Input {
        Add(Variant),
        Remove(Name),
        List,
    }
    #[rustfmt::skip]
    #[derive(
        ::rkyv::Archive,
        ::rkyv::Serialize,
        ::rkyv::Deserialize,
        Clone,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[cfg_attr(
        feature = "datom",
        derive(::datom_codec::Datomizable, ::datom_codec::Composing)
    )]
    pub enum Output {
        Added(Variant),
        Removed(Name),
        Listed(::alloc::vec::Vec<Variant>),
        Refused(::alloc::string::String),
    }
    pub trait Registry {
        fn variant(&self, name: &Name) -> ::core::option::Option<Variant>;
    }
}
#[rustfmt::skip]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Core {
    pub name: core::Name,
    pub carried: ::core::option::Option<Carried>,
}
#[cfg(feature = "datom")]
#[rustfmt::skip]
impl ::datom_codec::Datomizable for Core {
    fn datomize(&self, at: ::datom_codec::Path) -> ::datom_codec::Datom {
        match &self.carried {
            ::core::option::Option::None => {
                ::datom_codec::Datom {
                    path: at,
                    form: ::datom_codec::Form::Bare(self.name.0.clone()),
                }
            }
            ::core::option::Option::Some(carried) => {
                let body = match *carried {
                    Carried::Mark(ref value) => {
                        ::datom_codec::Datomizable::datomize(
                            value,
                            ::datom_codec::Branchable::child(&at, 1),
                        )
                    }
                };
                ::datom_codec::Datom {
                    path: at,
                    form: ::datom_codec::Form::Variant(
                        ::datom_codec::Symbol(self.name.0.clone()),
                        ::alloc::boxed::Box::new(body),
                    ),
                }
            }
        }
    }
}
#[cfg(feature = "datom")]
#[rustfmt::skip]
impl ::datom_codec::Composing for Core {
    fn compose(
        datom: &::datom_codec::Datom,
        budget: &mut ::datom_codec::Budget,
    ) -> ::core::result::Result<Self, ::datom_codec::Error> {
        ::datom_codec::Budgeted::spend(budget, &datom.path)?;
        match &datom.form {
            ::datom_codec::Form::Bare(head) => {
                ::core::result::Result::Ok(Self {
                    name: core::Name(head.clone()),
                    carried: ::core::option::Option::None,
                })
            }
            found => {
                ::core::result::Result::Err(
                    <::datom_codec::Error as ::datom_codec::ErrorRaising>::composition(
                        datom.path.clone(),
                        ::datom_codec::ErrorKind::Form {
                            expected: ::alloc::string::String::from(
                                "Bare; a name that carries a type is read with its registry, by Opening",
                            ),
                            found: ::alloc::string::String::from(
                                ::datom_codec::Naming::form_name(found),
                            ),
                        },
                    ),
                )
            }
        }
    }
}
#[cfg(feature = "datom")]
#[rustfmt::skip]
impl<Registry: core::Registry> Opening<Registry> for Core {
    fn open(
        datom: &::datom_codec::Datom,
        budget: &mut ::datom_codec::Budget,
        registry: &Registry,
    ) -> ::core::result::Result<Self, ::datom_codec::Error> {
        ::datom_codec::Budgeted::spend(budget, &datom.path)?;
        let (head, body) = match &datom.form {
            ::datom_codec::Form::Bare(head) => (head, ::core::option::Option::None),
            ::datom_codec::Form::Variant(head, body) => {
                (&head.0, ::core::option::Option::Some(&**body))
            }
            found => {
                return ::core::result::Result::Err(
                    <::datom_codec::Error as ::datom_codec::ErrorRaising>::composition(
                        datom.path.clone(),
                        ::datom_codec::ErrorKind::Form {
                            expected: ::alloc::string::String::from("Variant"),
                            found: ::alloc::string::String::from(
                                ::datom_codec::Naming::form_name(found),
                            ),
                        },
                    ),
                );
            }
        };
        let name = core::Name(head.clone());
        let ::core::option::Option::Some(entry) = registry.variant(&name) else {
            return ::core::result::Result::Err(
                <::datom_codec::Error as ::datom_codec::ErrorRaising>::composition(
                    datom.path.clone(),
                    ::datom_codec::ErrorKind::Variant {
                        expected: ::alloc::string::String::from(stringify!(Core)),
                        found: head.clone(),
                    },
                ),
            );
        };
        match (entry.type_option, body) {
            (::core::option::Option::None, ::core::option::Option::None) => {
                ::core::result::Result::Ok(Self {
                    name,
                    carried: ::core::option::Option::None,
                })
            }
            (
                ::core::option::Option::Some(core::Type::Name(carried)),
                ::core::option::Option::Some(body),
            ) => {
                let carried = match carried.0.as_str() {
                    "Mark" => {
                        Carried::Mark(::datom_codec::Composing::compose(body, budget)?)
                    }
                    other => {
                        return ::core::result::Result::Err(
                            <::datom_codec::Error as ::datom_codec::ErrorRaising>::composition(
                                datom.path.clone(),
                                ::datom_codec::ErrorKind::Variant {
                                    expected: ::alloc::string::String::from(
                                        "a type the file declares",
                                    ),
                                    found: ::alloc::string::String::from(other),
                                },
                            ),
                        );
                    }
                };
                ::core::result::Result::Ok(Self {
                    name,
                    carried: ::core::option::Option::Some(carried),
                })
            }
            _ => {
                ::core::result::Result::Err(
                    <::datom_codec::Error as ::datom_codec::ErrorRaising>::composition(
                        datom.path.clone(),
                        ::datom_codec::ErrorKind::Value {
                            expected: ::alloc::string::String::from(
                                "the form of the type its entry names",
                            ),
                            value: head.clone(),
                        },
                    ),
                )
            }
        }
    }
}
#[rustfmt::skip]
pub mod flow {
    #[rustfmt::skip]
    #[derive(
        ::rkyv::Archive,
        ::rkyv::Serialize,
        ::rkyv::Deserialize,
        Clone,
        Debug,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Hash
    )]
    #[cfg_attr(
        feature = "datom",
        derive(::datom_codec::Datomizable, ::datom_codec::Composing)
    )]
    pub struct Name(pub ::alloc::string::String);
    #[rustfmt::skip]
    #[derive(
        ::rkyv::Archive,
        ::rkyv::Serialize,
        ::rkyv::Deserialize,
        Clone,
        Debug,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Hash
    )]
    #[cfg_attr(
        feature = "datom",
        derive(::datom_codec::Datomizable, ::datom_codec::Composing)
    )]
    pub struct Word(pub ::alloc::string::String);
    #[rustfmt::skip]
    #[derive(
        ::rkyv::Archive,
        ::rkyv::Serialize,
        ::rkyv::Deserialize,
        Clone,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[cfg_attr(
        feature = "datom",
        derive(::datom_codec::Datomizable, ::datom_codec::Composing)
    )]
    pub struct Namespace(pub ::alloc::vec::Vec<Word>);
    #[rustfmt::skip]
    #[derive(
        ::rkyv::Archive,
        ::rkyv::Serialize,
        ::rkyv::Deserialize,
        Clone,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[cfg_attr(
        feature = "datom",
        derive(::datom_codec::Datomizable, ::datom_codec::Composing)
    )]
    pub struct Import {
        pub namespace: Namespace,
        pub name_vector: ::alloc::vec::Vec<Name>,
    }
    #[rustfmt::skip]
    #[derive(
        ::rkyv::Archive,
        ::rkyv::Serialize,
        ::rkyv::Deserialize,
        Clone,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[cfg_attr(
        feature = "datom",
        derive(::datom_codec::Datomizable, ::datom_codec::Composing)
    )]
    #[rkyv(
        serialize_bounds(
            __S: ::rkyv::ser::Writer+::rkyv::ser::Allocator,
            __S::Error: ::rkyv::rancor::Source
        )
    )]
    #[rkyv(deserialize_bounds(__D::Error: ::rkyv::rancor::Source))]
    #[rkyv(
        bytecheck(
            bounds(
                __C: ::rkyv::validation::ArchiveContext,
                __C::Error: ::rkyv::rancor::Source
            )
        )
    )]
    pub struct Declaration {
        pub name: Name,
        #[rkyv(omit_bounds)]
        pub r#type: Type,
    }
    #[rustfmt::skip]
    #[derive(
        ::rkyv::Archive,
        ::rkyv::Serialize,
        ::rkyv::Deserialize,
        Clone,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[cfg_attr(
        feature = "datom",
        derive(::datom_codec::Datomizable, ::datom_codec::Composing)
    )]
    #[rkyv(
        serialize_bounds(
            __S: ::rkyv::ser::Writer+::rkyv::ser::Allocator,
            __S::Error: ::rkyv::rancor::Source
        )
    )]
    #[rkyv(deserialize_bounds(__D::Error: ::rkyv::rancor::Source))]
    #[rkyv(
        bytecheck(
            bounds(
                __C: ::rkyv::validation::ArchiveContext,
                __C::Error: ::rkyv::rancor::Source
            )
        )
    )]
    pub struct Variant {
        pub name: Name,
        #[rkyv(omit_bounds)]
        pub type_option: ::core::option::Option<Type>,
    }
    #[rustfmt::skip]
    #[derive(
        ::rkyv::Archive,
        ::rkyv::Serialize,
        ::rkyv::Deserialize,
        Clone,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[cfg_attr(
        feature = "datom",
        derive(::datom_codec::Datomizable, ::datom_codec::Composing)
    )]
    #[rkyv(
        serialize_bounds(
            __S: ::rkyv::ser::Writer+::rkyv::ser::Allocator,
            __S::Error: ::rkyv::rancor::Source
        )
    )]
    #[rkyv(deserialize_bounds(__D::Error: ::rkyv::rancor::Source))]
    #[rkyv(
        bytecheck(
            bounds(
                __C: ::rkyv::validation::ArchiveContext,
                __C::Error: ::rkyv::rancor::Source
            )
        )
    )]
    pub struct Applied_Data {
        pub name: Name,
        #[rkyv(omit_bounds)]
        pub type_vector: ::alloc::vec::Vec<Type>,
    }
    #[rustfmt::skip]
    #[derive(
        ::rkyv::Archive,
        ::rkyv::Serialize,
        ::rkyv::Deserialize,
        Clone,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[cfg_attr(
        feature = "datom",
        derive(::datom_codec::Datomizable, ::datom_codec::Composing)
    )]
    #[rkyv(
        serialize_bounds(
            __S: ::rkyv::ser::Writer+::rkyv::ser::Allocator,
            __S::Error: ::rkyv::rancor::Source
        )
    )]
    #[rkyv(deserialize_bounds(__D::Error: ::rkyv::rancor::Source))]
    #[rkyv(
        bytecheck(
            bounds(
                __C: ::rkyv::validation::ArchiveContext,
                __C::Error: ::rkyv::rancor::Source
            )
        )
    )]
    pub enum Type {
        Name(Name),
        Import(Import),
        Declaration(#[rkyv(omit_bounds)] ::alloc::boxed::Box<Declaration>),
        Struct(#[rkyv(omit_bounds)] ::alloc::vec::Vec<Type>),
        Enum(#[rkyv(omit_bounds)] ::alloc::vec::Vec<Variant>),
        Vector(#[rkyv(omit_bounds)] ::alloc::boxed::Box<Type>),
        Applied(Applied_Data),
    }
    #[rustfmt::skip]
    #[derive(
        ::rkyv::Archive,
        ::rkyv::Serialize,
        ::rkyv::Deserialize,
        Clone,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[cfg_attr(
        feature = "datom",
        derive(::datom_codec::Datomizable, ::datom_codec::Composing)
    )]
    pub enum Input {
        Add(Variant),
        Remove(Name),
        List,
    }
    #[rustfmt::skip]
    #[derive(
        ::rkyv::Archive,
        ::rkyv::Serialize,
        ::rkyv::Deserialize,
        Clone,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[cfg_attr(
        feature = "datom",
        derive(::datom_codec::Datomizable, ::datom_codec::Composing)
    )]
    pub enum Output {
        Added(Variant),
        Removed(Name),
        Listed(::alloc::vec::Vec<Variant>),
        Refused(::alloc::string::String),
    }
    pub trait Registry {
        fn variant(&self, name: &Name) -> ::core::option::Option<Variant>;
    }
}
#[rustfmt::skip]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Flow {
    pub name: flow::Name,
    pub carried: ::core::option::Option<Carried>,
}
#[cfg(feature = "datom")]
#[rustfmt::skip]
impl ::datom_codec::Datomizable for Flow {
    fn datomize(&self, at: ::datom_codec::Path) -> ::datom_codec::Datom {
        match &self.carried {
            ::core::option::Option::None => {
                ::datom_codec::Datom {
                    path: at,
                    form: ::datom_codec::Form::Bare(self.name.0.clone()),
                }
            }
            ::core::option::Option::Some(carried) => {
                let body = match *carried {
                    Carried::Mark(ref value) => {
                        ::datom_codec::Datomizable::datomize(
                            value,
                            ::datom_codec::Branchable::child(&at, 1),
                        )
                    }
                };
                ::datom_codec::Datom {
                    path: at,
                    form: ::datom_codec::Form::Variant(
                        ::datom_codec::Symbol(self.name.0.clone()),
                        ::alloc::boxed::Box::new(body),
                    ),
                }
            }
        }
    }
}
#[cfg(feature = "datom")]
#[rustfmt::skip]
impl ::datom_codec::Composing for Flow {
    fn compose(
        datom: &::datom_codec::Datom,
        budget: &mut ::datom_codec::Budget,
    ) -> ::core::result::Result<Self, ::datom_codec::Error> {
        ::datom_codec::Budgeted::spend(budget, &datom.path)?;
        match &datom.form {
            ::datom_codec::Form::Bare(head) => {
                ::core::result::Result::Ok(Self {
                    name: flow::Name(head.clone()),
                    carried: ::core::option::Option::None,
                })
            }
            found => {
                ::core::result::Result::Err(
                    <::datom_codec::Error as ::datom_codec::ErrorRaising>::composition(
                        datom.path.clone(),
                        ::datom_codec::ErrorKind::Form {
                            expected: ::alloc::string::String::from(
                                "Bare; a name that carries a type is read with its registry, by Opening",
                            ),
                            found: ::alloc::string::String::from(
                                ::datom_codec::Naming::form_name(found),
                            ),
                        },
                    ),
                )
            }
        }
    }
}
#[cfg(feature = "datom")]
#[rustfmt::skip]
impl<Registry: flow::Registry> Opening<Registry> for Flow {
    fn open(
        datom: &::datom_codec::Datom,
        budget: &mut ::datom_codec::Budget,
        registry: &Registry,
    ) -> ::core::result::Result<Self, ::datom_codec::Error> {
        ::datom_codec::Budgeted::spend(budget, &datom.path)?;
        let (head, body) = match &datom.form {
            ::datom_codec::Form::Bare(head) => (head, ::core::option::Option::None),
            ::datom_codec::Form::Variant(head, body) => {
                (&head.0, ::core::option::Option::Some(&**body))
            }
            found => {
                return ::core::result::Result::Err(
                    <::datom_codec::Error as ::datom_codec::ErrorRaising>::composition(
                        datom.path.clone(),
                        ::datom_codec::ErrorKind::Form {
                            expected: ::alloc::string::String::from("Variant"),
                            found: ::alloc::string::String::from(
                                ::datom_codec::Naming::form_name(found),
                            ),
                        },
                    ),
                );
            }
        };
        let name = flow::Name(head.clone());
        let ::core::option::Option::Some(entry) = registry.variant(&name) else {
            return ::core::result::Result::Err(
                <::datom_codec::Error as ::datom_codec::ErrorRaising>::composition(
                    datom.path.clone(),
                    ::datom_codec::ErrorKind::Variant {
                        expected: ::alloc::string::String::from(stringify!(Flow)),
                        found: head.clone(),
                    },
                ),
            );
        };
        match (entry.type_option, body) {
            (::core::option::Option::None, ::core::option::Option::None) => {
                ::core::result::Result::Ok(Self {
                    name,
                    carried: ::core::option::Option::None,
                })
            }
            (
                ::core::option::Option::Some(flow::Type::Name(carried)),
                ::core::option::Option::Some(body),
            ) => {
                let carried = match carried.0.as_str() {
                    "Mark" => {
                        Carried::Mark(::datom_codec::Composing::compose(body, budget)?)
                    }
                    other => {
                        return ::core::result::Result::Err(
                            <::datom_codec::Error as ::datom_codec::ErrorRaising>::composition(
                                datom.path.clone(),
                                ::datom_codec::ErrorKind::Variant {
                                    expected: ::alloc::string::String::from(
                                        "a type the file declares",
                                    ),
                                    found: ::alloc::string::String::from(other),
                                },
                            ),
                        );
                    }
                };
                ::core::result::Result::Ok(Self {
                    name,
                    carried: ::core::option::Option::Some(carried),
                })
            }
            _ => {
                ::core::result::Result::Err(
                    <::datom_codec::Error as ::datom_codec::ErrorRaising>::composition(
                        datom.path.clone(),
                        ::datom_codec::ErrorKind::Value {
                            expected: ::alloc::string::String::from(
                                "the form of the type its entry names",
                            ),
                            value: head.clone(),
                        },
                    ),
                )
            }
        }
    }
}
#[rustfmt::skip]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "datom",
    derive(::datom_codec::Datomizable, ::datom_codec::Composing)
)]
pub enum Input {
    Launch(Pair),
    Stop(Mark),
    Core(self::core::Input),
    Flow(self::flow::Input),
}
#[rustfmt::skip]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "datom",
    derive(::datom_codec::Datomizable, ::datom_codec::Composing)
)]
pub enum Output {
    Launched,
    Refused(::alloc::string::String),
    Core(self::core::Output),
    Flow(self::flow::Output),
}
#[rustfmt::skip]
#[allow(clippy::result_large_err, clippy::ptr_arg, clippy::borrowed_box)]
pub trait Answering {
    fn launch(&self, pair: &Pair) -> ::core::result::Result<(), Output>;
    fn stop(
        &self,
        mark: &Mark,
    ) -> ::core::result::Result<::alloc::string::String, Output>;
    fn core(&self, core: &core::Input) -> ::core::result::Result<core::Output, Output>;
    fn flow(&self, flow: &flow::Input) -> ::core::result::Result<flow::Output, Output>;
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
            Input::Launch(carried) => {
                nexus
                    .launch(carried)
                    .map_or_else(|refusal| refusal, |()| Output::Launched)
            }
            Input::Stop(carried) => {
                nexus.stop(carried).map_or_else(|refusal| refusal, Output::Refused)
            }
            Input::Core(carried) => {
                nexus.core(carried).map_or_else(|refusal| refusal, Output::Core)
            }
            Input::Flow(carried) => {
                nexus.flow(carried).map_or_else(|refusal| refusal, Output::Flow)
            }
        }
    }
}
