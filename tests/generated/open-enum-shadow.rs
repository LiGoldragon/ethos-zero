#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    clippy::type_complexity
)]
#[rustfmt::skip]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(::datom_codec::Datomizable))]
pub struct Pair {
    pub core: Core,
    pub flow: Flow,
    pub alloc: Alloc,
    pub std: Std,
    pub voice: ::flow::Voice,
}
#[cfg(feature = "datom")]
#[rustfmt::skip]
impl<
    Registry: self::core::Registry + self::flow::Registry + self::alloc::Registry
        + self::std::Registry,
> Opening<Registry> for Pair {
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
                            expected: ::std::string::String::from("Struct"),
                            found: ::std::string::String::from(
                                ::datom_codec::Naming::form_name(found),
                            ),
                        },
                    ),
                );
            }
        };
        if children.len() != 5 {
            return ::core::result::Result::Err(
                <::datom_codec::Error as ::datom_codec::ErrorRaising>::composition(
                    datom.path.clone(),
                    ::datom_codec::ErrorKind::Arity {
                        expected: 5,
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
            alloc: {
                let child = &children[2];
                ::datom_codec::CompositionDepthing::enter_composition(
                    budget,
                    &child.path,
                )?;
                let read = Opening::open(child, budget, registry);
                ::datom_codec::CompositionDepthing::leave_composition(budget);
                read?
            },
            std: {
                let child = &children[3];
                ::datom_codec::CompositionDepthing::enter_composition(
                    budget,
                    &child.path,
                )?;
                let read = Opening::open(child, budget, registry);
                ::datom_codec::CompositionDepthing::leave_composition(budget);
                read?
            },
            voice: ::datom_codec::Composable::compose(&children[4], budget)?,
        })
    }
}
#[rustfmt::skip]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
    feature = "datom",
    derive(::datom_codec::Datomizable, ::datom_codec::Composing)
)]
pub struct Mark(pub ::std::string::String);
#[cfg(feature = "datom")]
#[rustfmt::skip]
impl<Registry> Opening<Registry> for Mark {
    fn open(
        datom: &::datom_codec::Datom,
        budget: &mut ::datom_codec::Budget,
        _registry: &Registry,
    ) -> ::core::result::Result<Self, ::datom_codec::Error> {
        <Self as ::datom_codec::Composing>::compose(datom, budget)
    }
}
#[rustfmt::skip]
pub mod _open {
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
    pub struct Name(pub ::std::string::String);
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
    pub struct Word(pub ::std::string::String);
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
    pub struct Namespace(pub ::std::vec::Vec<Word>);
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
        pub name_vector: ::std::vec::Vec<Name>,
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
        pub type_vector: ::std::vec::Vec<Type>,
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
        Declaration(#[rkyv(omit_bounds)] ::std::boxed::Box<Declaration>),
        Struct(#[rkyv(omit_bounds)] ::std::vec::Vec<Type>),
        Enum(#[rkyv(omit_bounds)] ::std::vec::Vec<Variant>),
        Vector(#[rkyv(omit_bounds)] ::std::boxed::Box<Type>),
        Applied(Applied_Data),
    }
}
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
impl<Registry> Opening<Registry> for Carried {
    fn open(
        datom: &::datom_codec::Datom,
        budget: &mut ::datom_codec::Budget,
        _registry: &Registry,
    ) -> ::core::result::Result<Self, ::datom_codec::Error> {
        <Self as ::datom_codec::Composing>::compose(datom, budget)
    }
}
#[rustfmt::skip]
pub trait Carrying: Sized {
    fn names() -> ::std::vec::Vec<self::_open::Name>;
    fn name(&self) -> self::_open::Name;
    #[cfg(feature = "datom")]
    fn read(
        name: &self::_open::Name,
        datom: &::datom_codec::Datom,
        budget: &mut ::datom_codec::Budget,
    ) -> ::core::result::Result<Self, ::datom_codec::Error>;
}
#[rustfmt::skip]
impl Carrying for Carried {
    fn names() -> ::std::vec::Vec<self::_open::Name> {
        ::std::vec![self::_open::Name(::std::string::String::from("Mark"))]
    }
    fn name(&self) -> self::_open::Name {
        self::_open::Name(
            ::std::string::String::from(
                match self {
                    Carried::Mark(_) => "Mark",
                },
            ),
        )
    }
    #[cfg(feature = "datom")]
    fn read(
        name: &self::_open::Name,
        datom: &::datom_codec::Datom,
        budget: &mut ::datom_codec::Budget,
    ) -> ::core::result::Result<Self, ::datom_codec::Error> {
        match name.0.as_str() {
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
                            expected: ::std::string::String::from(
                                "a type the file declares",
                            ),
                            found: ::std::string::String::from(other),
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
impl<Registry, Value: Opening<Registry>> Opening<Registry> for ::std::vec::Vec<Value> {
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
                            expected: ::std::string::String::from("Vector"),
                            found: ::std::string::String::from(
                                ::datom_codec::Naming::form_name(found),
                            ),
                        },
                    ),
                );
            }
        };
        let mut values = ::std::vec::Vec::with_capacity(children.len());
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
                            expected: ::std::string::String::from("Option"),
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
                        expected: ::std::string::String::from("Option"),
                        found: ::std::string::String::from(head),
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
impl<Registry, Value: Opening<Registry>, Failure: Opening<Registry>> Opening<Registry>
for ::core::result::Result<Value, Failure> {
    fn open(
        datom: &::datom_codec::Datom,
        budget: &mut ::datom_codec::Budget,
        registry: &Registry,
    ) -> ::core::result::Result<Self, ::datom_codec::Error> {
        let (head, body) = ::datom_codec::Variantizing::variant(
            datom,
            budget,
            "Variant",
        )?;
        let ok = match head {
            "Ok" => true,
            "Err" => false,
            found => {
                return ::core::result::Result::Err(
                    <::datom_codec::Error as ::datom_codec::ErrorRaising>::composition(
                        datom.path.clone(),
                        ::datom_codec::ErrorKind::Variant {
                            expected: ::std::string::String::from("Result"),
                            found: ::std::string::String::from(found),
                        },
                    ),
                );
            }
        };
        ::datom_codec::CompositionDepthing::enter_composition(budget, &body.path)?;
        let read = if ok {
            Value::open(body, budget, registry).map(::core::result::Result::Ok)
        } else {
            Failure::open(body, budget, registry).map(::core::result::Result::Err)
        };
        ::datom_codec::CompositionDepthing::leave_composition(budget);
        read
    }
}
#[cfg(feature = "datom")]
#[rustfmt::skip]
impl<Registry> Opening<Registry> for ::std::string::String {
    fn open(
        datom: &::datom_codec::Datom,
        budget: &mut ::datom_codec::Budget,
        _registry: &Registry,
    ) -> ::core::result::Result<Self, ::datom_codec::Error> {
        <Self as ::datom_codec::Composing>::compose(datom, budget)
    }
}
#[cfg(feature = "datom")]
#[rustfmt::skip]
impl<Registry> Opening<Registry> for i64 {
    fn open(
        datom: &::datom_codec::Datom,
        budget: &mut ::datom_codec::Budget,
        _registry: &Registry,
    ) -> ::core::result::Result<Self, ::datom_codec::Error> {
        <Self as ::datom_codec::Composing>::compose(datom, budget)
    }
}
#[cfg(feature = "datom")]
#[rustfmt::skip]
impl<Registry> Opening<Registry> for ::datom_codec::Decimal {
    fn open(
        datom: &::datom_codec::Datom,
        budget: &mut ::datom_codec::Budget,
        _registry: &Registry,
    ) -> ::core::result::Result<Self, ::datom_codec::Error> {
        <Self as ::datom_codec::Composing>::compose(datom, budget)
    }
}
#[cfg(feature = "datom")]
#[rustfmt::skip]
impl<Registry> Opening<Registry> for bool {
    fn open(
        datom: &::datom_codec::Datom,
        budget: &mut ::datom_codec::Budget,
        _registry: &Registry,
    ) -> ::core::result::Result<Self, ::datom_codec::Error> {
        <Self as ::datom_codec::Composing>::compose(datom, budget)
    }
}
#[cfg(feature = "datom")]
#[rustfmt::skip]
impl<Registry> Opening<Registry> for ::datom_codec::Meaning {
    fn open(
        datom: &::datom_codec::Datom,
        budget: &mut ::datom_codec::Budget,
        _registry: &Registry,
    ) -> ::core::result::Result<Self, ::datom_codec::Error> {
        <Self as ::datom_codec::Composing>::compose(datom, budget)
    }
}
#[cfg(feature = "datom")]
#[rustfmt::skip]
impl<Registry, Value: Opening<Registry>> Opening<Registry> for ::std::boxed::Box<Value> {
    fn open(
        datom: &::datom_codec::Datom,
        budget: &mut ::datom_codec::Budget,
        registry: &Registry,
    ) -> ::core::result::Result<Self, ::datom_codec::Error> {
        ::core::result::Result::Ok(
            ::std::boxed::Box::new(Value::open(datom, budget, registry)?),
        )
    }
}
#[rustfmt::skip]
pub mod core {
    pub use super::_open::*;
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
        Listed(::std::vec::Vec<Variant>),
        Refused(::std::string::String),
    }
    pub trait Registry {
        fn variant(&self, name: &Name) -> ::core::option::Option<Variant>;
    }
}
#[rustfmt::skip]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Core {
    pub name: self::_open::Name,
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
                        ::std::boxed::Box::new(body),
                    ),
                }
            }
        }
    }
}
#[cfg(feature = "datom")]
#[rustfmt::skip]
impl<Registry: self::core::Registry> Opening<Registry> for Core {
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
                            expected: ::std::string::String::from("Variant"),
                            found: ::std::string::String::from(
                                ::datom_codec::Naming::form_name(found),
                            ),
                        },
                    ),
                );
            }
        };
        let name = self::_open::Name(head.clone());
        let ::core::option::Option::Some(entry) = registry.variant(&name) else {
            return ::core::result::Result::Err(
                <::datom_codec::Error as ::datom_codec::ErrorRaising>::composition(
                    datom.path.clone(),
                    ::datom_codec::ErrorKind::Variant {
                        expected: ::std::string::String::from(stringify!(Core)),
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
                ::core::option::Option::Some(self::_open::Type::Name(carried)),
                ::core::option::Option::Some(body),
            ) => {
                let carried = <Carried as Carrying>::read(&carried, body, budget)?;
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
                            expected: ::std::string::String::from(
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
    pub use super::_open::*;
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
        Listed(::std::vec::Vec<Variant>),
        Refused(::std::string::String),
    }
    pub trait Registry {
        fn variant(&self, name: &Name) -> ::core::option::Option<Variant>;
    }
}
#[rustfmt::skip]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Flow {
    pub name: self::_open::Name,
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
                        ::std::boxed::Box::new(body),
                    ),
                }
            }
        }
    }
}
#[cfg(feature = "datom")]
#[rustfmt::skip]
impl<Registry: self::flow::Registry> Opening<Registry> for Flow {
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
                            expected: ::std::string::String::from("Variant"),
                            found: ::std::string::String::from(
                                ::datom_codec::Naming::form_name(found),
                            ),
                        },
                    ),
                );
            }
        };
        let name = self::_open::Name(head.clone());
        let ::core::option::Option::Some(entry) = registry.variant(&name) else {
            return ::core::result::Result::Err(
                <::datom_codec::Error as ::datom_codec::ErrorRaising>::composition(
                    datom.path.clone(),
                    ::datom_codec::ErrorKind::Variant {
                        expected: ::std::string::String::from(stringify!(Flow)),
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
                ::core::option::Option::Some(self::_open::Type::Name(carried)),
                ::core::option::Option::Some(body),
            ) => {
                let carried = <Carried as Carrying>::read(&carried, body, budget)?;
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
                            expected: ::std::string::String::from(
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
pub mod alloc {
    pub use super::_open::*;
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
        Listed(::std::vec::Vec<Variant>),
        Refused(::std::string::String),
    }
    pub trait Registry {
        fn variant(&self, name: &Name) -> ::core::option::Option<Variant>;
    }
}
#[rustfmt::skip]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Alloc {
    pub name: self::_open::Name,
    pub carried: ::core::option::Option<Carried>,
}
#[cfg(feature = "datom")]
#[rustfmt::skip]
impl ::datom_codec::Datomizable for Alloc {
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
                        ::std::boxed::Box::new(body),
                    ),
                }
            }
        }
    }
}
#[cfg(feature = "datom")]
#[rustfmt::skip]
impl<Registry: self::alloc::Registry> Opening<Registry> for Alloc {
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
                            expected: ::std::string::String::from("Variant"),
                            found: ::std::string::String::from(
                                ::datom_codec::Naming::form_name(found),
                            ),
                        },
                    ),
                );
            }
        };
        let name = self::_open::Name(head.clone());
        let ::core::option::Option::Some(entry) = registry.variant(&name) else {
            return ::core::result::Result::Err(
                <::datom_codec::Error as ::datom_codec::ErrorRaising>::composition(
                    datom.path.clone(),
                    ::datom_codec::ErrorKind::Variant {
                        expected: ::std::string::String::from(stringify!(Alloc)),
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
                ::core::option::Option::Some(self::_open::Type::Name(carried)),
                ::core::option::Option::Some(body),
            ) => {
                let carried = <Carried as Carrying>::read(&carried, body, budget)?;
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
                            expected: ::std::string::String::from(
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
pub mod std {
    pub use super::_open::*;
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
        Listed(::std::vec::Vec<Variant>),
        Refused(::std::string::String),
    }
    pub trait Registry {
        fn variant(&self, name: &Name) -> ::core::option::Option<Variant>;
    }
}
#[rustfmt::skip]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Std {
    pub name: self::_open::Name,
    pub carried: ::core::option::Option<Carried>,
}
#[cfg(feature = "datom")]
#[rustfmt::skip]
impl ::datom_codec::Datomizable for Std {
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
                        ::std::boxed::Box::new(body),
                    ),
                }
            }
        }
    }
}
#[cfg(feature = "datom")]
#[rustfmt::skip]
impl<Registry: self::std::Registry> Opening<Registry> for Std {
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
                            expected: ::std::string::String::from("Variant"),
                            found: ::std::string::String::from(
                                ::datom_codec::Naming::form_name(found),
                            ),
                        },
                    ),
                );
            }
        };
        let name = self::_open::Name(head.clone());
        let ::core::option::Option::Some(entry) = registry.variant(&name) else {
            return ::core::result::Result::Err(
                <::datom_codec::Error as ::datom_codec::ErrorRaising>::composition(
                    datom.path.clone(),
                    ::datom_codec::ErrorKind::Variant {
                        expected: ::std::string::String::from(stringify!(Std)),
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
                ::core::option::Option::Some(self::_open::Type::Name(carried)),
                ::core::option::Option::Some(body),
            ) => {
                let carried = <Carried as Carrying>::read(&carried, body, budget)?;
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
                            expected: ::std::string::String::from(
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
    Alloc(self::alloc::Input),
    Std(self::std::Input),
}
#[cfg(feature = "datom")]
#[rustfmt::skip]
impl<
    Registry: self::core::Registry + self::flow::Registry + self::alloc::Registry
        + self::std::Registry,
> Opening<Registry> for Input {
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
                            expected: ::std::string::String::from("Input"),
                            found: ::std::string::String::from(other),
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
                    "Alloc" => {
                        ::core::result::Result::Ok(
                            Self::Alloc(
                                ::datom_codec::Composable::compose(body, budget)?,
                            ),
                        )
                    }
                    "Std" => {
                        ::core::result::Result::Ok(
                            Self::Std(::datom_codec::Composable::compose(body, budget)?),
                        )
                    }
                    other => {
                        ::core::result::Result::Err(
                            <::datom_codec::Error as ::datom_codec::ErrorRaising>::composition(
                                datom.path.clone(),
                                ::datom_codec::ErrorKind::Variant {
                                    expected: ::std::string::String::from("Input"),
                                    found: ::std::string::String::from(other),
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
    Refused(::std::string::String),
    Core(self::core::Output),
    Flow(self::flow::Output),
    Alloc(self::alloc::Output),
    Std(self::std::Output),
}
#[cfg(feature = "datom")]
#[rustfmt::skip]
impl<Registry> Opening<Registry> for Output {
    fn open(
        datom: &::datom_codec::Datom,
        budget: &mut ::datom_codec::Budget,
        _registry: &Registry,
    ) -> ::core::result::Result<Self, ::datom_codec::Error> {
        <Self as ::datom_codec::Composing>::compose(datom, budget)
    }
}
#[rustfmt::skip]
#[allow(clippy::result_large_err, clippy::ptr_arg, clippy::borrowed_box)]
pub trait Answering {
    fn launch(&self, pair: &Pair) -> ::core::result::Result<(), Output>;
    fn stop(&self, mark: &Mark) -> ::core::result::Result<::std::string::String, Output>;
    fn core(
        &self,
        core: &self::core::Input,
    ) -> ::core::result::Result<self::core::Output, Output>;
    fn flow(
        &self,
        flow: &self::flow::Input,
    ) -> ::core::result::Result<self::flow::Output, Output>;
    fn alloc(
        &self,
        alloc: &self::alloc::Input,
    ) -> ::core::result::Result<self::alloc::Output, Output>;
    fn std(
        &self,
        std: &self::std::Input,
    ) -> ::core::result::Result<self::std::Output, Output>;
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
            Input::Alloc(carried) => {
                nexus.alloc(carried).map_or_else(|refusal| refusal, Output::Alloc)
            }
            Input::Std(carried) => {
                nexus.std(carried).map_or_else(|refusal| refusal, Output::Std)
            }
        }
    }
}
