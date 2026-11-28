#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    unused_extern_crates,
    clippy::type_complexity
)]
#[rustfmt::skip]
extern crate alloc;
#[cfg(feature = "datom")]
#[rustfmt::skip]
pub trait Opening<Registry>: Sized {
    fn open(
        datom: &datom_codec::Datom,
        budget: &mut datom_codec::Budget,
        registry: &Registry,
    ) -> core::result::Result<Self, datom_codec::Error>;
}
#[cfg(feature = "datom")]
#[rustfmt::skip]
impl<Registry, Value: Opening<Registry>> Opening<Registry> for alloc::vec::Vec<Value> {
    fn open(
        datom: &datom_codec::Datom,
        budget: &mut datom_codec::Budget,
        registry: &Registry,
    ) -> core::result::Result<Self, datom_codec::Error> {
        datom_codec::Budgeted::spend(budget, &datom.path)?;
        let children = match &datom.form {
            datom_codec::Form::Vector(children) => children,
            found => {
                return core::result::Result::Err(
                    <datom_codec::Error as datom_codec::ErrorRaising>::composition(
                        datom.path.clone(),
                        datom_codec::ErrorKind::Form {
                            expected: alloc::string::String::from("Vector"),
                            found: alloc::string::String::from(
                                datom_codec::Naming::form_name(found),
                            ),
                        },
                    ),
                );
            }
        };
        let mut values = alloc::vec::Vec::with_capacity(children.len());
        for child in children {
            datom_codec::CompositionDepthing::enter_composition(budget, &child.path)?;
            let read = Value::open(child, budget, registry);
            datom_codec::CompositionDepthing::leave_composition(budget);
            values.push(read?);
        }
        core::result::Result::Ok(values)
    }
}
#[cfg(feature = "datom")]
#[rustfmt::skip]
impl<Registry, Value: Opening<Registry>> Opening<Registry>
for core::option::Option<Value> {
    fn open(
        datom: &datom_codec::Datom,
        budget: &mut datom_codec::Budget,
        registry: &Registry,
    ) -> core::result::Result<Self, datom_codec::Error> {
        if let datom_codec::Form::Bare(head) = &datom.form {
            datom_codec::Budgeted::spend(budget, &datom.path)?;
            return if head == "None" {
                core::result::Result::Ok(core::option::Option::None)
            } else {
                core::result::Result::Err(
                    <datom_codec::Error as datom_codec::ErrorRaising>::composition(
                        datom.path.clone(),
                        datom_codec::ErrorKind::Variant {
                            expected: alloc::string::String::from("Option"),
                            found: head.clone(),
                        },
                    ),
                )
            };
        }
        let (head, body) = datom_codec::Variantizing::variant(datom, budget, "Variant")?;
        if head != "Some" {
            return core::result::Result::Err(
                <datom_codec::Error as datom_codec::ErrorRaising>::composition(
                    datom.path.clone(),
                    datom_codec::ErrorKind::Variant {
                        expected: alloc::string::String::from("Option"),
                        found: alloc::string::String::from(head),
                    },
                ),
            );
        }
        datom_codec::CompositionDepthing::enter_composition(budget, &body.path)?;
        let read = Value::open(body, budget, registry);
        datom_codec::CompositionDepthing::leave_composition(budget);
        core::result::Result::Ok(core::option::Option::Some(read?))
    }
}
#[cfg(feature = "datom")]
#[rustfmt::skip]
impl<Registry, Value: Opening<Registry>> Opening<Registry> for alloc::boxed::Box<Value> {
    fn open(
        datom: &datom_codec::Datom,
        budget: &mut datom_codec::Budget,
        registry: &Registry,
    ) -> core::result::Result<Self, datom_codec::Error> {
        core::result::Result::Ok(
            alloc::boxed::Box::new(Value::open(datom, budget, registry)?),
        )
    }
}
#[rustfmt::skip]
pub mod model {
    extern crate alloc;
    #[rustfmt::skip]
    #[derive(
        rkyv::Archive,
        rkyv::Serialize,
        rkyv::Deserialize,
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
        derive(datom_codec::Datomizable, datom_codec::Composing)
    )]
    pub struct Name(pub alloc::string::String);
    #[rustfmt::skip]
    #[derive(
        rkyv::Archive,
        rkyv::Serialize,
        rkyv::Deserialize,
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
        derive(datom_codec::Datomizable, datom_codec::Composing)
    )]
    pub struct Word(pub alloc::string::String);
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
    pub struct Namespace(pub alloc::vec::Vec<Word>);
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
    pub struct Import {
        pub namespace: Namespace,
        pub name_vector: alloc::vec::Vec<Name>,
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
    pub struct Declaration {
        pub name: Name,
        #[rkyv(omit_bounds)]
        pub r#type: Type,
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
    pub struct Variant {
        pub name: Name,
        #[rkyv(omit_bounds)]
        pub type_option: core::option::Option<Type>,
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
    pub struct Applied_Data {
        pub name: Name,
        #[rkyv(omit_bounds)]
        pub type_vector: alloc::vec::Vec<Type>,
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
    pub enum Type {
        Name(Name),
        Import(Import),
        Declaration(#[rkyv(omit_bounds)] alloc::boxed::Box<Declaration>),
        Struct(#[rkyv(omit_bounds)] alloc::vec::Vec<Type>),
        Enum(#[rkyv(omit_bounds)] alloc::vec::Vec<Variant>),
        Vector(#[rkyv(omit_bounds)] alloc::boxed::Box<Type>),
        Applied(Applied_Data),
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
    pub enum Input {
        Add(Variant),
        Remove(Name),
        List,
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
    pub enum Output {
        Added(Variant),
        Removed(Name),
        Listed(alloc::vec::Vec<Variant>),
        Refused(alloc::string::String),
    }
    pub trait Registry {
        fn variant(&self, name: &Name) -> core::option::Option<Variant>;
    }
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Model {
    pub name: model::Name,
}
#[cfg(feature = "datom")]
#[rustfmt::skip]
impl datom_codec::Datomizable for Model {
    fn datomize(&self, at: datom_codec::Path) -> datom_codec::Datom {
        datom_codec::Datom {
            path: at,
            form: datom_codec::Form::Bare(self.name.0.clone()),
        }
    }
}
#[cfg(feature = "datom")]
#[rustfmt::skip]
impl<Registry: model::Registry> Opening<Registry> for Model {
    fn open(
        datom: &datom_codec::Datom,
        budget: &mut datom_codec::Budget,
        registry: &Registry,
    ) -> core::result::Result<Self, datom_codec::Error> {
        datom_codec::Budgeted::spend(budget, &datom.path)?;
        let (head, body) = match &datom.form {
            datom_codec::Form::Bare(head) => (head, core::option::Option::None),
            datom_codec::Form::Variant(head, body) => {
                (&head.0, core::option::Option::Some(&**body))
            }
            found => {
                return core::result::Result::Err(
                    <datom_codec::Error as datom_codec::ErrorRaising>::composition(
                        datom.path.clone(),
                        datom_codec::ErrorKind::Form {
                            expected: alloc::string::String::from("Variant"),
                            found: alloc::string::String::from(
                                datom_codec::Naming::form_name(found),
                            ),
                        },
                    ),
                );
            }
        };
        let name = model::Name(head.clone());
        let core::option::Option::Some(entry) = registry.variant(&name) else {
            return core::result::Result::Err(
                <datom_codec::Error as datom_codec::ErrorRaising>::composition(
                    datom.path.clone(),
                    datom_codec::ErrorKind::Variant {
                        expected: alloc::string::String::from(stringify!(Model)),
                        found: head.clone(),
                    },
                ),
            );
        };
        match (entry.type_option, body) {
            (core::option::Option::None, core::option::Option::None) => {
                core::result::Result::Ok(Self { name })
            }
            _ => {
                core::result::Result::Err(
                    <datom_codec::Error as datom_codec::ErrorRaising>::composition(
                        datom.path.clone(),
                        datom_codec::ErrorKind::Value {
                            expected: alloc::string::String::from(
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
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Input {
    Model(model::Input),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Output {
    Model(model::Output),
}
#[rustfmt::skip]
#[allow(clippy::result_large_err, clippy::ptr_arg, clippy::borrowed_box)]
pub trait Answering {
    fn model(&self, model: &model::Input) -> core::result::Result<model::Output, Output>;
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
            Input::Model(carried) => {
                nexus.model(carried).map_or_else(|refusal| refusal, Output::Model)
            }
        }
    }
}
