#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    clippy::type_complexity
)]
#[rustfmt::skip]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(::datom_codec::Datomizable))]
pub struct Outcome {
    pub model_model_vector_result: ::core::result::Result<Model, ::std::vec::Vec<Model>>,
}
#[cfg(feature = "datom")]
#[rustfmt::skip]
impl<Registry: self::model::Registry> Opening<Registry> for Outcome {
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
        if children.len() != 1 {
            return ::core::result::Result::Err(
                <::datom_codec::Error as ::datom_codec::ErrorRaising>::composition(
                    datom.path.clone(),
                    ::datom_codec::ErrorKind::Arity {
                        expected: 1,
                        found: children.len() as ::datom_codec::Integer,
                    },
                ),
            );
        }
        ::core::result::Result::Ok(Self {
            model_model_vector_result: {
                let child = &children[0];
                ::datom_codec::CompositionDepthing::enter_composition(
                    budget,
                    &child.path,
                )?;
                let read = Opening::open(child, budget, registry);
                ::datom_codec::CompositionDepthing::leave_composition(budget);
                read?
            },
        })
    }
}
#[rustfmt::skip]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "datom",
    derive(::datom_codec::Datomizable, ::datom_codec::Composing)
)]
pub enum Effort {
    Low,
    High,
}
#[cfg(feature = "datom")]
#[rustfmt::skip]
impl<Registry> Opening<Registry> for Effort {
    fn open(
        datom: &::datom_codec::Datom,
        budget: &mut ::datom_codec::Budget,
        _registry: &Registry,
    ) -> ::core::result::Result<Self, ::datom_codec::Error> {
        <Self as ::datom_codec::Composing>::compose(datom, budget)
    }
}
#[rustfmt::skip]
pub mod open {
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
    Effort(Effort),
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
    fn names() -> ::std::vec::Vec<self::open::Name>;
    fn name(&self) -> self::open::Name;
    #[cfg(feature = "datom")]
    fn read(
        name: &self::open::Name,
        datom: &::datom_codec::Datom,
        budget: &mut ::datom_codec::Budget,
    ) -> ::core::result::Result<Self, ::datom_codec::Error>;
}
#[rustfmt::skip]
impl Carrying for Carried {
    fn names() -> ::std::vec::Vec<self::open::Name> {
        ::std::vec![self::open::Name(::std::string::String::from("Effort"))]
    }
    fn name(&self) -> self::open::Name {
        self::open::Name(
            ::std::string::String::from(
                match self {
                    Carried::Effort(_) => "Effort",
                },
            ),
        )
    }
    #[cfg(feature = "datom")]
    fn read(
        name: &self::open::Name,
        datom: &::datom_codec::Datom,
        budget: &mut ::datom_codec::Budget,
    ) -> ::core::result::Result<Self, ::datom_codec::Error> {
        match name.0.as_str() {
            "Effort" => {
                ::core::result::Result::Ok(
                    Carried::Effort(::datom_codec::Composing::compose(datom, budget)?),
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
pub mod model {
    pub use super::open::*;
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
pub struct Model {
    pub name: self::open::Name,
    pub carried: ::core::option::Option<Carried>,
}
#[cfg(feature = "datom")]
#[rustfmt::skip]
impl ::datom_codec::Datomizable for Model {
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
                    Carried::Effort(ref value) => {
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
impl<Registry: self::model::Registry> Opening<Registry> for Model {
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
        let name = self::open::Name(head.clone());
        let ::core::option::Option::Some(entry) = registry.variant(&name) else {
            return ::core::result::Result::Err(
                <::datom_codec::Error as ::datom_codec::ErrorRaising>::composition(
                    datom.path.clone(),
                    ::datom_codec::ErrorKind::Variant {
                        expected: ::std::string::String::from(stringify!(Model)),
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
                ::core::option::Option::Some(self::open::Type::Name(carried)),
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
    Settle(Outcome),
    Model(self::model::Input),
}
#[cfg(feature = "datom")]
#[rustfmt::skip]
impl<Registry: self::model::Registry> Opening<Registry> for Input {
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
                    "Settle" => {
                        ::core::result::Result::Ok(
                            Self::Settle({
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
                    "Model" => {
                        ::core::result::Result::Ok(
                            Self::Model(
                                ::datom_codec::Composable::compose(body, budget)?,
                            ),
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
    Settled,
    Model(self::model::Output),
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
    fn settle(&self, outcome: &Outcome) -> ::core::result::Result<(), Output>;
    fn model(
        &self,
        model: &self::model::Input,
    ) -> ::core::result::Result<self::model::Output, Output>;
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
            Input::Settle(carried) => {
                nexus
                    .settle(carried)
                    .map_or_else(|refusal| refusal, |()| Output::Settled)
            }
            Input::Model(carried) => {
                nexus.model(carried).map_or_else(|refusal| refusal, Output::Model)
            }
        }
    }
}
