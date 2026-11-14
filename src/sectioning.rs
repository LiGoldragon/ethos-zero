//! Sectioning: a file's sections by role, whatever its root, and every type
//! it declares, in place or at the top of its types section.
//!
//! Signal and Operation share one shape: imports, the variants of a request
//! enum, the variants of a reply enum, the types they carry. Signal implies
//! `Query` and `Response`; Operation implies `Operation` and `Outcome`.
//! Emission declares its emissions as types of their own, then the outcome
//! variants and the types they carry; it implies `Outcome` from its variants
//! and `Emission`, the enum of its emissions. Library and Memory imply
//! nothing.
//!
//! A struct position may declare its type in place, `Brief.String`; that
//! type is a declaration of the file like any other, named by its own name,
//! and found by walking the declarations and the implied enums' variants.

use datom_codec::{Integer, Path};

use crate::checking::Spanning;
use crate::{File, Import, Name, Position, Reference, TypeDeclaration, Variant};

/// An enum a root implies: its name, its variants and the section holding them.
pub(crate) struct Implied<'a> {
    pub(crate) name: Name,
    pub(crate) variants: &'a [Variant],
    pub(crate) section: Integer,
}

/// A section of type declarations and its index in the file's body.
pub(crate) struct DeclarationSection<'a> {
    pub(crate) section: Integer,
    pub(crate) declarations: &'a [TypeDeclaration],
}

/// A type declaration and the path of its node from the file's body.
pub(crate) struct Placed<'a> {
    pub(crate) path: Path,
    pub(crate) declaration: &'a TypeDeclaration,
}

/// The trait whose capabilities read a file's sections by role.
pub(crate) trait Sectioning {
    /// Where imported names come from.
    fn imports(&self) -> &[Import];
    /// Each section of type declarations, in section order: the types
    /// section, and an Emission's emissions before it.
    fn declaration_sections(&self) -> Vec<DeclarationSection<'_>>;
    /// The enums the root implies from written variants, in section order.
    fn implied_enums(&self) -> Vec<Implied<'_>>;
}

impl Sectioning for File {
    fn imports(&self) -> &[Import] {
        match self {
            File::Library(library) => &library.imports,
            File::Signal(signal) => &signal.imports,
            File::Operation(operation) => &operation.imports,
            File::Memory(memory) => &memory.imports,
            File::Emission(emission) => &emission.imports,
        }
    }

    fn declaration_sections(&self) -> Vec<DeclarationSection<'_>> {
        match self {
            File::Library(library) => vec![DeclarationSection {
                section: 1,
                declarations: &library.types,
            }],
            File::Memory(memory) => vec![DeclarationSection {
                section: 1,
                declarations: &memory.types,
            }],
            File::Signal(signal) => vec![DeclarationSection {
                section: 3,
                declarations: &signal.types,
            }],
            File::Operation(operation) => vec![DeclarationSection {
                section: 3,
                declarations: &operation.types,
            }],
            File::Emission(emission) => vec![
                DeclarationSection {
                    section: 1,
                    declarations: &emission.emissions,
                },
                DeclarationSection {
                    section: 3,
                    declarations: &emission.types,
                },
            ],
        }
    }

    fn implied_enums(&self) -> Vec<Implied<'_>> {
        let (request, requests, reply, replies) = match self {
            File::Library(_) | File::Memory(_) => return vec![],
            // The emissions are declarations, not variants: only `Outcome`
            // is written as variants.
            File::Emission(emission) => {
                return vec![Implied {
                    name: Name::try_from("Outcome").expect("static identifier"),
                    variants: &emission.outcomes,
                    section: 2,
                }];
            }
            File::Signal(signal) => ("Query", &signal.queries, "Response", &signal.responses),
            File::Operation(operation) => (
                "Operation",
                &operation.operations,
                "Outcome",
                &operation.outcomes,
            ),
        };
        vec![
            Implied {
                name: Name::try_from(request).expect("static identifier"),
                variants: requests,
                section: 1,
            },
            Implied {
                name: Name::try_from(reply).expect("static identifier"),
                variants: replies,
                section: 2,
            },
        ]
    }
}

/// The trait whose capabilities list every type declaration of a file.
pub(crate) trait Hoisting {
    /// Every declaration: each of the types section in order, each followed
    /// by those declared in place within it, then those declared in place in
    /// the implied enums' variants.
    fn declarations(&self) -> Vec<Placed<'_>>;
    /// Only the declarations made in place, in the same order.
    fn inlined(&self) -> Vec<Placed<'_>>;
}

impl Hoisting for File {
    fn declarations(&self) -> Vec<Placed<'_>> {
        let mut placed = Vec::new();
        for section in self.declaration_sections() {
            let mut at = 0;
            for declaration in section.declarations {
                declaration.hoist(vec![section.section, at], &mut placed);
                at += declaration.span();
            }
        }
        for implied in self.implied_enums() {
            implied
                .variants
                .hoist_variants(vec![implied.section], &mut placed);
        }
        placed
    }

    // A declaration of the types section sits at `[ section at ]`; one
    // declared in place sits deeper.
    fn inlined(&self) -> Vec<Placed<'_>> {
        let mut inlined = Vec::new();
        for placed in self.declarations() {
            if placed.path.len() > 2 {
                inlined.push(placed);
            }
        }
        inlined
    }
}

/// The trait whose capabilities walk the declarations below a node.
trait Hoist {
    fn hoist<'a>(&'a self, path: Path, placed: &mut Vec<Placed<'a>>);
}

/// The trait whose capability walks the declarations in a list of positions.
trait PositionsHoisting {
    fn hoist_positions<'a>(&'a self, path: Path, placed: &mut Vec<Placed<'a>>);
}

/// The trait whose capability walks the declarations in a list of variants.
trait VariantsHoisting {
    fn hoist_variants<'a>(&'a self, path: Path, placed: &mut Vec<Placed<'a>>);
}

impl Hoist for TypeDeclaration {
    fn hoist<'a>(&'a self, path: Path, placed: &mut Vec<Placed<'a>>) {
        let mut body = path.clone();
        body.push(1);
        placed.push(Placed {
            path,
            declaration: self,
        });
        match self {
            TypeDeclaration::Struct(_, positions) => positions.hoist_positions(body, placed),
            TypeDeclaration::Enum(_, variants) => variants.hoist_variants(body, placed),
            TypeDeclaration::Alias(_, _) => {}
        }
    }
}

impl PositionsHoisting for [Position] {
    fn hoist_positions<'a>(&'a self, path: Path, placed: &mut Vec<Placed<'a>>) {
        let mut at = 0;
        for position in self {
            if let Position::Declared(declaration) = position {
                let mut here = path.clone();
                here.push(at);
                declaration.hoist(here, placed);
            }
            at += position.span();
        }
    }
}

impl VariantsHoisting for [Variant] {
    fn hoist_variants<'a>(&'a self, path: Path, placed: &mut Vec<Placed<'a>>) {
        let mut at = 0;
        for variant in self {
            let mut body = path.clone();
            body.extend([at, 1]);
            match variant {
                Variant::Struct(_, positions) => positions.hoist_positions(body, placed),
                Variant::Enum(_, variants) => variants.hoist_variants(body, placed),
                Variant::Bare(_) | Variant::Typed(_, _) => {}
            }
            at += variant.span();
        }
    }
}

/// The trait whose capability yields the type a position holds, by reference:
/// a declaration in place is held by its name.
pub(crate) trait Referencing {
    fn reference(&self) -> Reference;
}

impl Referencing for Position {
    fn reference(&self) -> Reference {
        match self {
            Position::Referenced(reference) => reference.clone(),
            Position::Declared(declaration) => Reference {
                source: None,
                name: crate::checking::Identified::identity(declaration)
                    .name
                    .clone(),
                arguments: vec![],
            },
        }
    }
}

/// The trait whose capability yields every position's type by reference.
pub(crate) trait ReferencingEach {
    fn references(&self) -> Vec<Reference>;
}

impl ReferencingEach for [Position] {
    fn references(&self) -> Vec<Reference> {
        self.iter().map(Referencing::reference).collect()
    }
}
