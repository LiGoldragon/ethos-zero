//! Conception: structural Protos to Ethos concepts.

use datom_codec::Integer;
use protos::{Enclosure, Protos, Separator, Symbol};

use crate::{
    ArityProblem, AssociatedConstant, AssociatedType, Association, Capability, ConceptualFaulting,
    Constraint, Error, Ethosizable, File, Form, Identifiable, Identity, Import, Imported, KindBody,
    KindDeclaration, Library, Name, Placing, Problem, Receiver, Reference, Root, Sema, Signal,
    Signature, Source, TypeDeclaration, Variant,
};

pub(crate) trait Conceiving<C> {
    fn conceive(&self) -> Result<C, Error>;
}
impl Ethosizable<File> for Protos {
    type Error = Error;
    fn ethosize(&self) -> Result<File, Self::Error> {
        self.conceive()
    }
}
trait Children {
    fn children(&self, enclosure: Enclosure) -> Option<&[Protos]>;
}
impl Children for Protos {
    fn children(&self, enclosure: Enclosure) -> Option<&[Protos]> {
        match self {
            Protos::Enclosed {
                enclosure: found,
                children,
                ..
            } if *found == enclosure => Some(children),
            _ => None,
        }
    }
}
trait Naming {
    fn name(&self) -> Result<Name, Error>;
}
impl Naming for Symbol {
    fn name(&self) -> Result<Name, Error> {
        Name::try_from(self.0.as_str())
            .map_err(|text| Error::conceptual(vec![], Problem::Name(text)))
    }
}
impl Naming for str {
    fn name(&self) -> Result<Name, Error> {
        Name::try_from(self).map_err(|text| Error::conceptual(vec![], Problem::Name(text)))
    }
}
trait Reading {
    fn list<C>(children: &[Protos]) -> Result<Vec<C>, Error>
    where
        Protos: Conceiving<C>;
}
impl Reading for Protos {
    fn list<C>(children: &[Protos]) -> Result<Vec<C>, Error>
    where
        Protos: Conceiving<C>,
    {
        let mut values = Vec::new();
        for (index, child) in children.iter().enumerate() {
            values.push(<Protos as Conceiving<C>>::conceive(child).place(index as Integer)?);
        }
        Ok(values)
    }
}

impl Conceiving<File> for Protos {
    fn conceive(&self) -> Result<File, Error> {
        let Protos::Headed {
            head,
            constraints,
            separator,
            body,
            ..
        } = self
        else {
            return Err(Error::conceptual(vec![], Problem::Root));
        };
        if constraints.is_some() || *separator != Separator::Period {
            return Err(Error::conceptual(vec![], Problem::Root));
        };
        match Root::identify(&head.0) {
            Some(Root::Library) => Ok(File::Library(body.conceive()?)),
            Some(Root::Signal) => Ok(File::Signal(body.conceive()?)),
            Some(Root::Sema) => Ok(File::Sema(body.conceive()?)),
            _ => Err(Error::conceptual(vec![], Problem::Root)),
        }
    }
}
trait Sections {
    fn sections(&self, count: usize) -> Result<&[Protos], Error>;
}
impl Sections for Protos {
    fn sections(&self, count: usize) -> Result<&[Protos], Error> {
        let Some(parts) = self.children(Enclosure::Braced) else {
            return Err(Error::conceptual(vec![], Problem::Expected(Form::File)));
        };
        if parts.len() != count {
            return Err(Error::conceptual(
                vec![],
                Problem::arity(count as Integer, parts.len() as Integer),
            ));
        };
        Ok(parts)
    }
}
trait Brackets {
    fn bracket<C>(&self) -> Result<Vec<C>, Error>
    where
        Protos: Conceiving<C>;
}
impl Brackets for Protos {
    fn bracket<C>(&self) -> Result<Vec<C>, Error>
    where
        Protos: Conceiving<C>,
    {
        let Some(children) = self.children(Enclosure::Bracketed) else {
            return Err(Error::conceptual(vec![], Problem::Expected(Form::Section)));
        };
        Protos::list(children)
    }
}
trait References {
    fn references(&self) -> Result<Vec<Reference>, Error>;
}
impl References for [Protos] {
    fn references(&self) -> Result<Vec<Reference>, Error> {
        let mut values = Vec::new();
        let mut index = 0;
        while index < self.len() {
            if let Protos::Bare { text, .. } = &self[index]
                && let Some(Protos::Enclosed {
                    enclosure: Enclosure::Angled,
                    children,
                    ..
                }) = self.get(index + 1)
            {
                values.push(Reference {
                    source: None,
                    name: text.as_str().name().place(index as Integer)?,
                    arguments: children.references().place((index + 1) as Integer)?,
                });
                index += 2;
                continue;
            }
            values.push(self[index].conceive().place(index as Integer)?);
            index += 1;
        }
        Ok(values)
    }
}
trait Declarations {
    fn declarations(&self) -> Result<Vec<TypeDeclaration>, Error>;
}
impl Declarations for [Protos] {
    fn declarations(&self) -> Result<Vec<TypeDeclaration>, Error> {
        let mut values = Vec::new();
        let mut index = 0;
        while index < self.len() {
            let mut declaration: TypeDeclaration =
                self[index].conceive().place(index as Integer)?;
            if let (
                TypeDeclaration::Alias(_, reference),
                Some(Protos::Enclosed {
                    enclosure: Enclosure::Angled,
                    children,
                    ..
                }),
            ) = (&mut declaration, self.get(index + 1))
            {
                reference.arguments = children.references().place((index + 1) as Integer)?;
                index += 1;
            }
            values.push(declaration);
            index += 1;
        }
        Ok(values)
    }
}
trait Variants {
    fn variants(&self) -> Result<Vec<Variant>, Error>;
}
impl Variants for [Protos] {
    fn variants(&self) -> Result<Vec<Variant>, Error> {
        let mut values = Vec::new();
        let mut index = 0;
        while index < self.len() {
            let mut variant: Variant = self[index].conceive().place(index as Integer)?;
            if let (
                Variant::Typed(_, reference),
                Some(Protos::Enclosed {
                    enclosure: Enclosure::Angled,
                    children,
                    ..
                }),
            ) = (&mut variant, self.get(index + 1))
            {
                reference.arguments = children.references().place((index + 1) as Integer)?;
                index += 1;
            }
            values.push(variant);
            index += 1;
        }
        Ok(values)
    }
}
trait AssociatedTypes {
    fn associated_types(&self) -> Result<Vec<AssociatedType>, Error>;
}
impl AssociatedTypes for [Protos] {
    fn associated_types(&self) -> Result<Vec<AssociatedType>, Error> {
        let mut values = Vec::new();
        let mut index = 0;
        while index < self.len() {
            let Protos::Bare { text, .. } = &self[index] else {
                return Err(Error::conceptual(
                    vec![index as Integer],
                    Problem::Expected(Form::Kind),
                ));
            };
            let bounds = match self.get(index + 1) {
                Some(Protos::Enclosed {
                    enclosure: Enclosure::Angled,
                    children,
                    ..
                }) => {
                    index += 1;
                    Protos::list(children).place((index + 1) as Integer)?
                }
                _ => vec![],
            };
            values.push(AssociatedType {
                name: text.as_str().name().place(index as Integer)?,
                bounds,
            });
            index += 1;
        }
        Ok(values)
    }
}
impl Conceiving<Library> for Protos {
    fn conceive(&self) -> Result<Library, Error> {
        let s = self.sections(4)?;
        let Some(declarations) = s[1].children(Enclosure::Bracketed) else {
            return Err(Error::conceptual(vec![1], Problem::Expected(Form::Section)));
        };
        Ok(Library {
            imports: s[0].bracket().place(0)?,
            types: declarations.declarations().place(1)?,
            kinds: s[2].bracket().place(2)?,
            associations: s[3].bracket().place(3)?,
        })
    }
}
impl Conceiving<Signal> for Protos {
    fn conceive(&self) -> Result<Signal, Error> {
        let s = self.sections(4)?;
        let Some(requests) = s[1].children(Enclosure::Bracketed) else {
            return Err(Error::conceptual(vec![1], Problem::Expected(Form::Section)));
        };
        let Some(responses) = s[2].children(Enclosure::Bracketed) else {
            return Err(Error::conceptual(vec![2], Problem::Expected(Form::Section)));
        };
        let Some(declarations) = s[3].children(Enclosure::Bracketed) else {
            return Err(Error::conceptual(vec![3], Problem::Expected(Form::Section)));
        };
        Ok(Signal {
            imports: s[0].bracket().place(0)?,
            requests: requests.variants().place(1)?,
            responses: responses.variants().place(2)?,
            types: declarations.declarations().place(3)?,
        })
    }
}
impl Conceiving<Sema> for Protos {
    fn conceive(&self) -> Result<Sema, Error> {
        let s = self.sections(2)?;
        let Some(types) = s[1].children(Enclosure::Bracketed) else {
            return Err(Error::conceptual(vec![1], Problem::Expected(Form::Section)));
        };
        Ok(Sema {
            imports: s[0].bracket().place(0)?,
            types: types.declarations().place(1)?,
        })
    }
}
impl Conceiving<Imported> for Protos {
    fn conceive(&self) -> Result<Imported, Error> {
        let name = match self {
            Protos::Bare { text, .. } => text.as_str().name()?,
            Protos::Headed {
                head,
                separator: Separator::Period,
                body,
                ..
            } => {
                let Protos::Bare { text, .. } = body.as_ref() else {
                    return Err(Error::conceptual(vec![1], Problem::Expected(Form::Import)));
                };
                return Ok(Imported {
                    name: head.name()?,
                    emitted: text.as_str().name()?,
                });
            }
            _ => return Err(Error::conceptual(vec![], Problem::Expected(Form::Import))),
        };
        Ok(Imported {
            name: name.clone(),
            emitted: name,
        })
    }
}
impl Conceiving<Import> for Protos {
    fn conceive(&self) -> Result<Import, Error> {
        let Protos::Headed {
            head,
            separator: Separator::Colon,
            body,
            ..
        } = self
        else {
            return Err(Error::conceptual(vec![], Problem::Expected(Form::Import)));
        };
        let source = Source::try_from(head.0.as_str())
            .map_err(|text| Error::conceptual(vec![], Problem::Name(text)))?;
        if let Some(children) = body.children(Enclosure::Bracketed) {
            Ok(Import::Many(source, Protos::list(children)?))
        } else {
            Ok(Import::One(source, body.conceive()?))
        }
    }
}
impl Conceiving<Reference> for Protos {
    fn conceive(&self) -> Result<Reference, Error> {
        match self {
            Protos::Bare { text, .. } => Ok(Reference {
                source: None,
                name: text.as_str().name()?,
                arguments: vec![],
            }),
            Protos::Headed {
                head,
                separator: Separator::Colon,
                body,
                ..
            } => {
                let mut r: Reference = body.conceive()?;
                r.source = Some(
                    Source::try_from(head.0.as_str())
                        .map_err(|text| Error::conceptual(vec![], Problem::Name(text)))?,
                );
                Ok(r)
            }
            _ => Err(Error::conceptual(
                vec![],
                Problem::Expected(Form::Reference),
            )),
        }
    }
}
impl Conceiving<TypeDeclaration> for Protos {
    fn conceive(&self) -> Result<TypeDeclaration, Error> {
        let Protos::Headed {
            head,
            constraints,
            separator: Separator::Period,
            body,
            ..
        } = self
        else {
            return Err(Error::conceptual(
                vec![],
                Problem::Expected(Form::Declaration),
            ));
        };
        if constraints.is_some() {
            return Err(Error::conceptual(
                vec![],
                Problem::Expected(Form::Declaration),
            ));
        }
        let identity = Identity {
            name: head.name()?,
            constraints: vec![],
        };
        if let Some(p) = body.children(Enclosure::Braced) {
            Ok(TypeDeclaration::Struct(identity, p.references()?))
        } else if let Some(v) = body.children(Enclosure::Bracketed) {
            Ok(TypeDeclaration::Enum(identity, v.variants()?))
        } else {
            Ok(TypeDeclaration::Alias(identity, body.conceive()?))
        }
    }
}
impl Conceiving<Variant> for Protos {
    fn conceive(&self) -> Result<Variant, Error> {
        match self {
            Protos::Bare { text, .. } => Ok(Variant::Bare(text.as_str().name()?)),
            Protos::Headed {
                head,
                separator: Separator::Period,
                body,
                ..
            } => {
                let n = head.name()?;
                if let Some(p) = body.children(Enclosure::Braced) {
                    Ok(Variant::Struct(n, p.references()?))
                } else if let Some(v) = body.children(Enclosure::Bracketed) {
                    Ok(Variant::Enum(n, v.variants()?))
                } else {
                    Ok(Variant::Typed(n, body.conceive()?))
                }
            }
            _ => Err(Error::conceptual(vec![], Problem::Expected(Form::Variant))),
        }
    }
}

impl Conceiving<Constraint> for Protos {
    fn conceive(&self) -> Result<Constraint, Error> {
        if let Some(c) = self.children(Enclosure::Bracketed) {
            Ok(Constraint::Many(Protos::list(c)?))
        } else {
            Ok(Constraint::One(self.conceive()?))
        }
    }
}
impl Conceiving<Receiver> for Separator {
    fn conceive(&self) -> Result<Receiver, Error> {
        Ok(match self {
            Separator::Period => Receiver::Shared,
            Separator::Exclamation => Receiver::Mutable,
            Separator::Colon => Receiver::Static,
        })
    }
}
impl Conceiving<AssociatedType> for Protos {
    fn conceive(&self) -> Result<AssociatedType, Error> {
        match self {
            Protos::Bare { text, .. } => Ok(AssociatedType {
                name: text.as_str().name()?,
                bounds: vec![],
            }),
            Protos::Headed {
                head, constraints, ..
            } => Ok(AssociatedType {
                name: head.name()?,
                bounds: match constraints.as_deref() {
                    Some(Protos::Enclosed { children, .. }) => Protos::list(children)?,
                    _ => vec![],
                },
            }),
            _ => Err(Error::conceptual(vec![], Problem::Expected(Form::Kind))),
        }
    }
}
impl Conceiving<AssociatedConstant> for Protos {
    fn conceive(&self) -> Result<AssociatedConstant, Error> {
        let Protos::Headed {
            head,
            separator: Separator::Period,
            body,
            ..
        } = self
        else {
            return Err(Error::conceptual(vec![], Problem::Expected(Form::Constant)));
        };
        Ok(AssociatedConstant {
            name: head.name().place(0)?,
            ty: body.conceive().place(1)?,
        })
    }
}
impl Conceiving<Capability> for Protos {
    fn conceive(&self) -> Result<Capability, Error> {
        let Protos::Headed {
            head,
            separator,
            body,
            ..
        } = self
        else {
            return Err(Error::conceptual(
                vec![],
                Problem::Expected(Form::Capability),
            ));
        };
        let name = head.name().place(0)?;
        let receiver = separator.conceive()?;
        if let Some(y) = body.children(Enclosure::Bracketed) {
            let yields = y.references().place(1)?;
            if yields.len() != 1 {
                return Err(Error::conceptual(vec![1], Problem::Yield));
            };
            return Ok(Capability {
                name,
                receiver,
                signature: Signature::Yielding(yields.into_iter().next().expect("one yield")),
            });
        };
        let sections = body.sections(2)?;
        let Some(input_nodes) = sections[0].children(Enclosure::Bracketed) else {
            return Err(Error::conceptual(
                vec![0],
                Problem::Expected(Form::Capability),
            ));
        };
        let inputs = input_nodes.references().place(0).place(1)?;
        let Some(y) = sections[1].children(Enclosure::Bracketed) else {
            return Err(Error::conceptual(
                vec![1],
                Problem::Expected(Form::Capability),
            ));
        };
        let yields = y.references().place(1).place(1)?;
        if yields.len() != 1 {
            return Err(Error::conceptual(vec![1, 1], Problem::Yield));
        };
        Ok(Capability {
            name,
            receiver,
            signature: Signature::Taking(inputs, yields.into_iter().next().expect("one yield")),
        })
    }
}
impl Conceiving<KindDeclaration> for Protos {
    fn conceive(&self) -> Result<KindDeclaration, Error> {
        let Protos::Headed {
            head,
            constraints,
            separator: Separator::Period,
            body,
            ..
        } = self
        else {
            return Err(Error::conceptual(vec![], Problem::Expected(Form::Kind)));
        };
        let identity = Identity {
            name: head.name()?,
            constraints: match constraints.as_deref() {
                Some(Protos::Enclosed { children, .. }) => Protos::list(children)?,
                _ => vec![],
            },
        };
        if let Some(c) = body.children(Enclosure::Bracketed) {
            return Ok(KindDeclaration {
                identity,
                body: KindBody::Simple(Protos::list(c).place(1)?),
            });
        };
        let s = body.sections(4)?;
        let Some(types) = s[1].children(Enclosure::Bracketed) else {
            return Err(Error::conceptual(vec![1], Problem::Expected(Form::Kind)));
        };
        Ok(KindDeclaration {
            identity,
            body: KindBody::Complex {
                superkinds: s[0].bracket().place(0).place(1)?,
                types: types.associated_types().place(1).place(1)?,
                constants: s[2].bracket().place(2).place(1)?,
                capabilities: s[3].bracket().place(3).place(1)?,
            },
        })
    }
}
impl Conceiving<Association> for Protos {
    fn conceive(&self) -> Result<Association, Error> {
        let Protos::Headed {
            head,
            separator: Separator::Period,
            body,
            ..
        } = self
        else {
            return Err(Error::conceptual(
                vec![],
                Problem::Expected(Form::Association),
            ));
        };
        Ok(Association {
            identity: Identity {
                name: head.name().place(0)?,
                constraints: vec![],
            },
            kinds: body.bracket().place(1)?,
        })
    }
}
