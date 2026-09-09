//! Protosization: the infallible structural ascent of a checked Ethos file.

use protos::{Canonicalizable, Enclosure, Extent, Protos, Protosizable, Separator, Symbol};

use crate::{
    AssociatedConstant, AssociatedType, Association, Capability, Constraint, File, Identity,
    Import, Imported, KindBody, KindDeclaration, Library, Receiver, Reference, Sema, Signal,
    Signature, TypeDeclaration, Variant,
};

pub(crate) trait Protosizing {
    fn protos(&self) -> Protos;
}

trait Structuring {
    fn bare(&self) -> Protos;
    fn enclosed(&self, enclosure: Enclosure, children: Vec<Protos>) -> Protos;
    fn headed(&self, constraints: Option<Protos>, separator: Separator, body: Protos) -> Protos;
}

impl Structuring for str {
    fn bare(&self) -> Protos {
        Protos::Bare {
            extent: Extent { start: 0, end: 0 },
            text: self.to_owned(),
        }
    }
    fn enclosed(&self, enclosure: Enclosure, children: Vec<Protos>) -> Protos {
        Protos::Enclosed {
            extent: Extent { start: 0, end: 0 },
            enclosure,
            children,
        }
    }
    fn headed(&self, constraints: Option<Protos>, separator: Separator, body: Protos) -> Protos {
        Protos::Headed {
            extent: Extent { start: 0, end: 0 },
            head: Symbol(self.to_owned()),
            constraints: constraints.map(Box::new),
            separator,
            body: Box::new(body),
        }
    }
}

trait ListProtosizing {
    fn protos_list(&self) -> Vec<Protos>;
}
impl<T: Protosizing> ListProtosizing for [T] {
    fn protos_list(&self) -> Vec<Protos> {
        self.iter().map(Protosizing::protos).collect()
    }
}

trait ReferencesProtosizing {
    fn reference_nodes(&self) -> Vec<Protos>;
}
impl ReferencesProtosizing for [Reference] {
    fn reference_nodes(&self) -> Vec<Protos> {
        let mut nodes = Vec::new();
        for reference in self {
            if reference.arguments.is_empty() {
                nodes.push(reference.protos());
            } else {
                let base = match &reference.source {
                    Some(source) => source.as_ref().headed(
                        None,
                        Separator::Colon,
                        reference.name.as_ref().bare(),
                    ),
                    None => reference.name.as_ref().bare(),
                };
                nodes.push(base);
                nodes.push("".enclosed(Enclosure::Angled, reference.arguments.reference_nodes()));
            }
        }
        nodes
    }
}

trait DeclarationsProtosizing {
    fn declaration_nodes(&self) -> Vec<Protos>;
}

/// Associated types use the substrate's adjacent bare-plus-angle sequence:
/// `Item<Serializable>`.  They are declarations only conceptually; emitting
/// a headed `Item<...>.` invents a separator and cannot be read back.
trait AssociatedTypesProtosizing {
    fn associated_type_nodes(&self) -> Vec<Protos>;
}
impl AssociatedTypesProtosizing for [AssociatedType] {
    fn associated_type_nodes(&self) -> Vec<Protos> {
        let mut nodes = Vec::new();
        for associated in self {
            nodes.push(associated.name.as_ref().bare());
            if !associated.bounds.is_empty() {
                nodes.push("".enclosed(Enclosure::Angled, associated.bounds.protos_list()));
            }
        }
        nodes
    }
}
impl DeclarationsProtosizing for [TypeDeclaration] {
    fn declaration_nodes(&self) -> Vec<Protos> {
        let mut nodes = Vec::new();
        for declaration in self {
            match declaration {
                TypeDeclaration::Alias(identity, reference) if !reference.arguments.is_empty() => {
                    let body = match &reference.source {
                        Some(source) => source.as_ref().headed(
                            None,
                            Separator::Colon,
                            reference.name.as_ref().bare(),
                        ),
                        None => reference.name.as_ref().bare(),
                    };
                    nodes.push(identity.heading(Separator::Period, body));
                    nodes.push(
                        "".enclosed(Enclosure::Angled, reference.arguments.reference_nodes()),
                    );
                }
                _ => nodes.push(declaration.protos()),
            }
        }
        nodes
    }
}

impl Protosizing for Reference {
    fn protos(&self) -> Protos {
        let base = self.name.as_ref().bare();
        let applied = if self.arguments.is_empty() {
            base
        } else {
            let arguments = "".enclosed(Enclosure::Angled, self.arguments.protos_list());
            "".enclosed(Enclosure::Braced, vec![base, arguments])
        };
        match &self.source {
            Some(source) => source.as_ref().headed(None, Separator::Colon, applied),
            None => applied,
        }
    }
}

impl Protosizing for Constraint {
    fn protos(&self) -> Protos {
        match self {
            Self::One(reference) => reference.protos(),
            Self::Many(references) => "".enclosed(Enclosure::Bracketed, references.protos_list()),
        }
    }
}

impl Protosizing for Identity {
    fn protos(&self) -> Protos {
        let constraints = (!self.constraints.is_empty())
            .then(|| "".enclosed(Enclosure::Angled, self.constraints.protos_list()));
        self.name
            .as_ref()
            .headed(constraints, Separator::Period, "".bare())
    }
}

trait Heading {
    fn heading(&self, separator: Separator, body: Protos) -> Protos;
}
impl Heading for Identity {
    fn heading(&self, separator: Separator, body: Protos) -> Protos {
        let constraints = (!self.constraints.is_empty())
            .then(|| "".enclosed(Enclosure::Angled, self.constraints.protos_list()));
        self.name.as_ref().headed(constraints, separator, body)
    }
}

impl Protosizing for Imported {
    fn protos(&self) -> Protos {
        if self.name == self.emitted {
            self.name.as_ref().bare()
        } else {
            self.name
                .as_ref()
                .headed(None, Separator::Period, self.emitted.as_ref().bare())
        }
    }
}
impl Protosizing for Import {
    fn protos(&self) -> Protos {
        match self {
            Self::One(source, imported) => {
                source
                    .as_ref()
                    .headed(None, Separator::Colon, imported.protos())
            }
            Self::Many(source, imported) => source.as_ref().headed(
                None,
                Separator::Colon,
                "".enclosed(Enclosure::Bracketed, imported.protos_list()),
            ),
        }
    }
}
impl Protosizing for TypeDeclaration {
    fn protos(&self) -> Protos {
        match self {
            Self::Struct(identity, positions) => identity.heading(
                Separator::Period,
                "".enclosed(Enclosure::Braced, positions.reference_nodes()),
            ),
            Self::Enum(identity, variants) => identity.heading(
                Separator::Period,
                "".enclosed(Enclosure::Bracketed, variants.protos_list()),
            ),
            Self::Alias(identity, reference) => {
                identity.heading(Separator::Period, reference.protos())
            }
        }
    }
}
impl Protosizing for Variant {
    fn protos(&self) -> Protos {
        match self {
            Self::Bare(name) => name.as_ref().bare(),
            Self::Typed(name, reference) => {
                name.as_ref()
                    .headed(None, Separator::Period, reference.protos())
            }
            Self::Struct(name, positions) => name.as_ref().headed(
                None,
                Separator::Period,
                "".enclosed(Enclosure::Braced, positions.reference_nodes()),
            ),
            Self::Enum(name, variants) => name.as_ref().headed(
                None,
                Separator::Period,
                "".enclosed(Enclosure::Bracketed, variants.protos_list()),
            ),
        }
    }
}
impl Protosizing for AssociatedConstant {
    fn protos(&self) -> Protos {
        self.name
            .as_ref()
            .headed(None, Separator::Period, self.ty.protos())
    }
}
impl Protosizing for Capability {
    fn protos(&self) -> Protos {
        let separator = match self.receiver {
            Receiver::Shared => Separator::Period,
            Receiver::Mutable => Separator::Exclamation,
            Receiver::Static => Separator::Colon,
        };
        let body = match &self.signature {
            Signature::Yielding(yielding) => "".enclosed(
                Enclosure::Bracketed,
                std::slice::from_ref(yielding).reference_nodes(),
            ),
            Signature::Taking(inputs, yielding) => "".enclosed(
                Enclosure::Braced,
                vec![
                    "".enclosed(Enclosure::Bracketed, inputs.reference_nodes()),
                    "".enclosed(
                        Enclosure::Bracketed,
                        std::slice::from_ref(yielding).reference_nodes(),
                    ),
                ],
            ),
        };
        self.name.as_ref().headed(None, separator, body)
    }
}
impl Protosizing for KindDeclaration {
    fn protos(&self) -> Protos {
        let body = match &self.body {
            KindBody::Simple(capabilities) => {
                "".enclosed(Enclosure::Bracketed, capabilities.protos_list())
            }
            KindBody::Complex {
                superkinds,
                types,
                constants,
                capabilities,
            } => "".enclosed(
                Enclosure::Braced,
                vec![
                    "".enclosed(Enclosure::Bracketed, superkinds.protos_list()),
                    "".enclosed(Enclosure::Bracketed, types.associated_type_nodes()),
                    "".enclosed(Enclosure::Bracketed, constants.protos_list()),
                    "".enclosed(Enclosure::Bracketed, capabilities.protos_list()),
                ],
            ),
        };
        self.identity.heading(Separator::Period, body)
    }
}
impl Protosizing for Association {
    fn protos(&self) -> Protos {
        self.identity.heading(
            Separator::Period,
            "".enclosed(Enclosure::Bracketed, self.kinds.protos_list()),
        )
    }
}
impl Protosizing for Library {
    fn protos(&self) -> Protos {
        "".enclosed(
            Enclosure::Braced,
            vec![
                "".enclosed(Enclosure::Bracketed, self.imports.protos_list()),
                "".enclosed(Enclosure::Bracketed, self.types.declaration_nodes()),
                "".enclosed(Enclosure::Bracketed, self.kinds.protos_list()),
                "".enclosed(Enclosure::Bracketed, self.associations.protos_list()),
            ],
        )
    }
}
impl Protosizing for Signal {
    fn protos(&self) -> Protos {
        "".enclosed(
            Enclosure::Braced,
            vec![
                "".enclosed(Enclosure::Bracketed, self.imports.protos_list()),
                "".enclosed(Enclosure::Bracketed, self.requests.protos_list()),
                "".enclosed(Enclosure::Bracketed, self.responses.protos_list()),
                "".enclosed(Enclosure::Bracketed, self.types.declaration_nodes()),
            ],
        )
    }
}
impl Protosizing for Sema {
    fn protos(&self) -> Protos {
        "".enclosed(
            Enclosure::Braced,
            vec![
                "".enclosed(Enclosure::Bracketed, self.imports.protos_list()),
                "".enclosed(Enclosure::Bracketed, self.types.declaration_nodes()),
            ],
        )
    }
}
impl Protosizing for File {
    fn protos(&self) -> Protos {
        let raw = match self {
            Self::Library(library) => "Library".headed(None, Separator::Period, library.protos()),
            Self::Signal(signal) => "Signal".headed(None, Separator::Period, signal.protos()),
            Self::Sema(sema) => "Sema".headed(None, Separator::Period, sema.protos()),
        };
        // Shared Protos canonicalization assigns exact UTF-8 byte spans
        // without routing a valid conceptual ascent through a finite reader.
        let mut canonical = raw;
        canonical.canonicalize();
        canonical
    }
}

impl Protosizable for File {
    type Output = Protos;

    fn protosize(&self) -> Self::Output {
        self.protos()
    }
}
