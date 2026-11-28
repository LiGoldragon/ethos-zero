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
#[cfg_attr(feature = "datom", derive(::datom_codec::Datomizable))]
pub struct Pair {
    pub core: Core,
    pub flow: Flow,
    pub voice: ::flow::Voice,
}
#[cfg(feature = "datom")]
#[rustfmt::skip]
impl<Registry: core::Registry + flow::Registry> Opening<Registry> for Pair {
    fn open(
        datom: &::datom_codec::Datom,
        budget: &mut ::datom_codec::Budget,
        registry: &Registry,
    ) -> ::core::result::Result<Self, ::datom_codec::Error> {
        ::datom_codec::Budgeted::spend(budget, &datom.path)?;
        let children = match &datom.form {
            ::datom_codec::Form::Struct(children) => children,
            found => {
                return ::core::result::Result::Err(
                    <::datom_codec::Error as ::datom_codec::ErrorRaising>::composition(
                        datom.path.clone(),
                        ::datom_codec::ErrorKind::Form {
                            expected: ::alloc::string::String::from("Struct"),
                            found: ::alloc::string::String::from(
                                ::datom_codec::Naming::form_name(found),
                            ),
                        },
                    ),
                );
            }
        };
        if children.len() != 3 {
            return ::core::result::Result::Err(
                <::datom_codec::Error as ::datom_codec::ErrorRaising>::composition(
                    datom.path.clone(),
                    ::datom_codec::ErrorKind::Arity {
                        expected: 3,
                        found: children.len() as ::datom_codec::Integer,
                    },
                ),
            );
        }
        ::core::result::Result::Ok(Self {
            core: {
                let child = &children[0];
                ::datom_codec::CompositionDepthing::enter_composition(
                    budget,
                    &child.path,
                )?;
                let read = Opening::open(child, budget, registry);
                ::datom_codec::CompositionDepthing::leave_composition(budget);
                read?
            },
            flow: {
                let child = &children[1];
                ::datom_codec::CompositionDepthing::enter_composition(
                    budget,
                    &child.path,
                )?;
                let read = Opening::open(child, budget, registry);
                ::datom_codec::CompositionDepthing::leave_composition(budget);
                read?
            },
            voice: ::datom_codec::Composable::compose(&children[2], budget)?,
        })
    }
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
#[rustfmt::skip]
pub trait Carrying: Sized {
    const NAMES: &'static [&'static str];
    fn name(&self) -> &'static str;
    #[cfg(feature = "datom")]
    fn read(
        name: &str,
        datom: &::datom_codec::Datom,
        budget: &mut ::datom_codec::Budget,
    ) -> ::core::result::Result<Self, ::datom_codec::Error>;
}
#[rustfmt::skip]
impl Carrying for Carried {
    const NAMES: &'static [&'static str] = &["Mark"];
    fn name(&self) -> &'static str {
        match self {
            Carried::Mark(_) => "Mark",
        }
    }
    #[cfg(feature = "datom")]
    fn read(
        name: &str,
        datom: &::datom_codec::Datom,
        budget: &mut ::datom_codec::Budget,
    ) -> ::core::result::Result<Self, ::datom_codec::Error> {
        match name {
            "Mark" => {
                ::core::result::Result::Ok(
                    Carried::Mark(::datom_codec::Composing::compose(datom, budget)?),
                )
            }
            other => {
                ::core::result::Result::Err(
                    <::datom_codec::Error as ::datom_codec::ErrorRaising>::composition(
                        datom.path.clone(),
                        ::datom_codec::ErrorKind::Variant {
                            expected: ::alloc::string::String::from(
                                "a type the file declares",
                            ),
                            found: ::alloc::string::String::from(other),
                        },
                    ),
                )
            }
        }
    }
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
#[cfg(feature = "datom")]
#[rustfmt::skip]
impl<Registry, Value: Opening<Registry>> Opening<Registry> for ::alloc::vec::Vec<Value> {
    fn open(
        datom: &::datom_codec::Datom,
        budget: &mut ::datom_codec::Budget,
        registry: &Registry,
    ) -> ::core::result::Result<Self, ::datom_codec::Error> {
        ::datom_codec::Budgeted::spend(budget, &datom.path)?;
        let children = match &datom.form {
            ::datom_codec::Form::Vector(children) => children,
            found => {
                return ::core::result::Result::Err(
                    <::datom_codec::Error as ::datom_codec::ErrorRaising>::composition(
                        datom.path.clone(),
                        ::datom_codec::ErrorKind::Form {
                            expected: ::alloc::string::String::from("Vector"),
                            found: ::alloc::string::String::from(
                                ::datom_codec::Naming::form_name(found),
                            ),
                        },
                    ),
                );
            }
        };
        let mut values = ::alloc::vec::Vec::with_capacity(children.len());
        for child in children {
            ::datom_codec::CompositionDepthing::enter_composition(budget, &child.path)?;
            let read = Value::open(child, budget, registry);
            ::datom_codec::CompositionDepthing::leave_composition(budget);
            values.push(read?);
        }
        ::core::result::Result::Ok(values)
    }
}
#[cfg(feature = "datom")]
#[rustfmt::skip]
impl<Registry, Value: Opening<Registry>> Opening<Registry>
for ::core::option::Option<Value> {
    fn open(
        datom: &::datom_codec::Datom,
        budget: &mut ::datom_codec::Budget,
        registry: &Registry,
    ) -> ::core::result::Result<Self, ::datom_codec::Error> {
        if let ::datom_codec::Form::Bare(head) = &datom.form {
            ::datom_codec::Budgeted::spend(budget, &datom.path)?;
            return if head == "None" {
                ::core::result::Result::Ok(::core::option::Option::None)
            } else {
                ::core::result::Result::Err(
                    <::datom_codec::Error as ::datom_codec::ErrorRaising>::composition(
                        datom.path.clone(),
                        ::datom_codec::ErrorKind::Variant {
                            expected: ::alloc::string::String::from("Option"),
                            found: head.clone(),
                        },
                    ),
                )
            };
        }
        let (head, body) = ::datom_codec::Variantizing::variant(
            datom,
            budget,
            "Variant",
        )?;
        if head != "Some" {
            return ::core::result::Result::Err(
                <::datom_codec::Error as ::datom_codec::ErrorRaising>::composition(
                    datom.path.clone(),
                    ::datom_codec::ErrorKind::Variant {
                        expected: ::alloc::string::String::from("Option"),
                        found: ::alloc::string::String::from(head),
                    },
                ),
            );
        }
        ::datom_codec::CompositionDepthing::enter_composition(budget, &body.path)?;
        let read = Value::open(body, budget, registry);
        ::datom_codec::CompositionDepthing::leave_composition(budget);
        ::core::result::Result::Ok(::core::option::Option::Some(read?))
    }
}
#[cfg(feature = "datom")]
#[rustfmt::skip]
impl<Registry, Value: Opening<Registry>> Opening<Registry>
for ::alloc::boxed::Box<Value> {
    fn open(
        datom: &::datom_codec::Datom,
        budget: &mut ::datom_codec::Budget,
        registry: &Registry,
    ) -> ::core::result::Result<Self, ::datom_codec::Error> {
        ::core::result::Result::Ok(
            ::alloc::boxed::Box::new(Value::open(datom, budget, registry)?),
        )
    }
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
                let carried = <Carried as Carrying>::read(
                    carried.0.as_str(),
                    body,
                    budget,
                )?;
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
                let carried = <Carried as Carrying>::read(
                    carried.0.as_str(),
                    body,
                    budget,
                )?;
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
#[cfg_attr(feature = "datom", derive(::datom_codec::Datomizable))]
pub enum Input {
    Launch(Pair),
    Stop(Mark),
    Core(self::core::Input),
    Flow(self::flow::Input),
}
#[cfg(feature = "datom")]
#[rustfmt::skip]
impl<Registry: core::Registry + flow::Registry> Opening<Registry> for Input {
    fn open(
        datom: &::datom_codec::Datom,
        budget: &mut ::datom_codec::Budget,
        registry: &Registry,
    ) -> ::core::result::Result<Self, ::datom_codec::Error> {
        match &datom.form {
            ::datom_codec::Form::Bare(head) => {
                ::datom_codec::Budgeted::spend(budget, &datom.path)?;
                let other = head.as_str();
                ::core::result::Result::Err(
                    <::datom_codec::Error as ::datom_codec::ErrorRaising>::composition(
                        datom.path.clone(),
                        ::datom_codec::ErrorKind::Variant {
                            expected: ::alloc::string::String::from("Input"),
                            found: ::alloc::string::String::from(other),
                        },
                    ),
                )
            }
            _ => {
                let (head, body) = ::datom_codec::Variantizing::variant(
                    datom,
                    budget,
                    "Variant",
                )?;
                match head {
                    "Launch" => {
                        ::core::result::Result::Ok(
                            Self::Launch({
                                let child = body;
                                ::datom_codec::CompositionDepthing::enter_composition(
                                    budget,
                                    &child.path,
                                )?;
                                let read = Opening::open(child, budget, registry);
                                ::datom_codec::CompositionDepthing::leave_composition(
                                    budget,
                                );
                                read?
                            }),
                        )
                    }
                    "Stop" => {
                        ::core::result::Result::Ok(
                            Self::Stop(::datom_codec::Composable::compose(body, budget)?),
                        )
                    }
                    "Core" => {
                        ::core::result::Result::Ok(
                            Self::Core(::datom_codec::Composable::compose(body, budget)?),
                        )
                    }
                    "Flow" => {
                        ::core::result::Result::Ok(
                            Self::Flow(::datom_codec::Composable::compose(body, budget)?),
                        )
                    }
                    other => {
                        ::core::result::Result::Err(
                            <::datom_codec::Error as ::datom_codec::ErrorRaising>::composition(
                                datom.path.clone(),
                                ::datom_codec::ErrorKind::Variant {
                                    expected: ::alloc::string::String::from("Input"),
                                    found: ::alloc::string::String::from(other),
                                },
                            ),
                        )
                    }
                }
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
