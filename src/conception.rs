//! Conception: structural Protos to Ethos concepts.

use datom_codec::Integer;
use protos::{Enclosure, Protos, Separator, Symbol};

use crate::{
    ArityProblem, AssociatedConstant, AssociatedType, Association, Capability, ConceptualErroring,
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
            Some(Root::Library) => Ok(File::Library(body.conceive().place(1)?)),
            Some(Root::Signal) => Ok(File::Signal(body.conceive().place(1)?)),
            Some(Root::Sema) => Ok(File::Sema(body.conceive().place(1)?)),
            _ => Err(Error::conceptual(vec![0], Problem::Root)),
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
/// The kind whose capability pairs each node of a list with the angle-bracketed
/// constraints written against it.
///
/// `Vector<Integer>` is one type in Ethos and two sibling protos structures: a
/// constrained head that no separator follows is rewound to a bare run, leaving
/// the angled enclosure beside it.
///
/// This re-join is owed to protos, not to Ethos. `Protos::Headed` carries a
/// separator and a body unconditionally, so a name bearing constraints and no
/// body has no node to be read into; for Ethos to read and print
/// `Vector<Integer>` as one structure, protos needs such a node -- constraints
/// on a bare run, or an optional separator and body on a headed one -- a reader
/// that keeps the constraints instead of rewinding past them, and a writer that
/// prints the constraints against the name with no space between. Until then
/// the re-join is done here, once, for every list context, and the canonical
/// reprint spells the type `Vector <Integer>`.
///
/// Each pair carries the node, the constraints if any, and the node's own
/// index, so an error still points at the authored position.
trait Constraining {
    fn constrained(&self) -> Vec<(&Protos, Option<&Vec<Protos>>, usize)>;
}
impl Constraining for [Protos] {
    fn constrained(&self) -> Vec<(&Protos, Option<&Vec<Protos>>, usize)> {
        let mut pairs = Vec::new();
        let mut index = 0;
        while index < self.len() {
            let arguments = match self.get(index + 1) {
                Some(Protos::Enclosed {
                    enclosure: Enclosure::Angled,
                    children,
                    ..
                }) => Some(children),
                _ => None,
            };
            pairs.push((&self[index], arguments, index));
            index += if arguments.is_some() { 2 } else { 1 };
        }
        pairs
    }
}
trait References {
    fn references(&self) -> Result<Vec<Reference>, Error>;
}
impl References for [Protos] {
    fn references(&self) -> Result<Vec<Reference>, Error> {
        let mut values = Vec::new();
        for (node, arguments, index) in self.constrained() {
            let mut reference: Reference = node.conceive().place(index as Integer)?;
            if let Some(children) = arguments {
                reference.arguments = children.references().place((index + 1) as Integer)?;
            }
            values.push(reference);
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
        for (node, arguments, index) in self.constrained() {
            let mut declaration: TypeDeclaration = node.conceive().place(index as Integer)?;
            if let Some(children) = arguments {
                let TypeDeclaration::Alias(_, reference) = &mut declaration else {
                    return Err(Error::conceptual(
                        vec![(index + 1) as Integer],
                        Problem::Expected(Form::Declaration),
                    ));
                };
                reference.arguments = children.references().place((index + 1) as Integer)?;
            }
            values.push(declaration);
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
        for (node, arguments, index) in self.constrained() {
            let mut variant: Variant = node.conceive().place(index as Integer)?;
            if let Some(children) = arguments {
                let Variant::Typed(_, reference) = &mut variant else {
                    return Err(Error::conceptual(
                        vec![(index + 1) as Integer],
                        Problem::Expected(Form::Variant),
                    ));
                };
                reference.arguments = children.references().place((index + 1) as Integer)?;
            }
            values.push(variant);
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
        for (node, arguments, index) in self.constrained() {
            let Protos::Bare { text, .. } = node else {
                return Err(Error::conceptual(
                    vec![index as Integer],
                    Problem::Expected(Form::Kind),
                ));
            };
            let bounds = match arguments {
                Some(children) => Protos::list(children).place((index + 1) as Integer)?,
                None => vec![],
            };
            values.push(AssociatedType {
                name: text.as_str().name().place(index as Integer)?,
                bounds,
            });
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
        let Some(queries) = s[1].children(Enclosure::Bracketed) else {
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
            queries: queries.variants().place(1)?,
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
                    name: head.name().place(0)?,
                    emitted: text.as_str().name().place(1)?,
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
            .map_err(|text| Error::conceptual(vec![0], Problem::Name(text)))?;
        if let Some(children) = body.children(Enclosure::Bracketed) {
            Ok(Import::Many(source, Protos::list(children).place(1)?))
        } else {
            Ok(Import::One(source, body.conceive().place(1)?))
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
                let mut r: Reference = body.conceive().place(1)?;
                r.source = Some(
                    Source::try_from(head.0.as_str())
                        .map_err(|text| Error::conceptual(vec![0], Problem::Name(text)))?,
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
            name: head.name().place(0)?,
            constraints: vec![],
        };
        if let Some(p) = body.children(Enclosure::Braced) {
            Ok(TypeDeclaration::Struct(identity, p.references().place(1)?))
        } else if let Some(v) = body.children(Enclosure::Bracketed) {
            Ok(TypeDeclaration::Enum(identity, v.variants().place(1)?))
        } else {
            Ok(TypeDeclaration::Alias(identity, body.conceive().place(1)?))
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
                let n = head.name().place(0)?;
                if let Some(p) = body.children(Enclosure::Braced) {
                    Ok(Variant::Struct(n, p.references().place(1)?))
                } else if let Some(v) = body.children(Enclosure::Bracketed) {
                    Ok(Variant::Enum(n, v.variants().place(1)?))
                } else {
                    Ok(Variant::Typed(n, body.conceive().place(1)?))
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
                name: head.name().place(0)?,
                bounds: match constraints.as_deref() {
                    Some(Protos::Enclosed { children, .. }) => Protos::list(children).place(0)?,
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
        let sections = body.sections(2).place(1)?;
        let Some(input_nodes) = sections[0].children(Enclosure::Bracketed) else {
            return Err(Error::conceptual(
                vec![1, 0],
                Problem::Expected(Form::Capability),
            ));
        };
        let inputs = input_nodes.references().place(0).place(1)?;
        let Some(y) = sections[1].children(Enclosure::Bracketed) else {
            return Err(Error::conceptual(
                vec![1, 1],
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
            name: head.name().place(0)?,
            constraints: match constraints.as_deref() {
                Some(Protos::Enclosed { children, .. }) => Protos::list(children).place(0)?,
                _ => vec![],
            },
        };
        if let Some(c) = body.children(Enclosure::Bracketed) {
            return Ok(KindDeclaration {
                identity,
                body: KindBody::Simple(Protos::list(c).place(1)?),
            });
        };
        let s = body.sections(4).place(1)?;
        let Some(types) = s[1].children(Enclosure::Bracketed) else {
            return Err(Error::conceptual(vec![1, 1], Problem::Expected(Form::Kind)));
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
