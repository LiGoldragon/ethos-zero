//! Sectioning: a file's sections by role, whatever its root, and every type
//! it declares, in place or at the top of its types section.
//!
//! Signal and Operation share one shape: imports, the variants of a request
//! enum, the variants of a reply enum, the types they carry. Signal implies
//! `Query` and `Response`; Operation implies `Operation` and `Outcome`.
//! Library and Memory imply nothing.
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

/// A type declaration and the path of its node from the file's body.
pub(crate) struct Placed<'a> {
    pub(crate) path: Path,
    pub(crate) declaration: &'a TypeDeclaration,
}

/// The trait whose capabilities read a file's sections by role.
pub(crate) trait Sectioning {
    /// Where imported names come from.
    fn imports(&self) -> &[Import];
    /// The types section's declarations.
    fn types(&self) -> &[TypeDeclaration];
    /// The index of the types section.
    fn types_section(&self) -> Integer;
    /// The enums the root implies, in section order.
    fn implied_enums(&self) -> Vec<Implied<'_>>;
}

impl Sectioning for File {
    fn imports(&self) -> &[Import] {
        match self {
            File::Library(library) => &library.imports,
            File::Signal(signal) => &signal.imports,
            File::Operation(operation) => &operation.imports,
            File::Memory(memory) => &memory.imports,
        }
    }

    fn types(&self) -> &[TypeDeclaration] {
        match self {
            File::Library(library) => &library.types,
            File::Signal(signal) => &signal.types,
            File::Operation(operation) => &operation.types,
            File::Memory(memory) => &memory.types,
        }
    }

    fn types_section(&self) -> Integer {
        match self {
            File::Library(_) | File::Memory(_) => 1,
            File::Signal(_) | File::Operation(_) => 3,
        }
    }

    fn implied_enums(&self) -> Vec<Implied<'_>> {
        let (request, requests, reply, replies) = match self {
            File::Library(_) | File::Memory(_) => return vec![],
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
        let section = self.types_section();
        let mut at = 0;
        for declaration in self.types() {
            declaration.hoist(vec![section, at], &mut placed);
            at += declaration.span();
        }
        for implied in self.implied_enums() {
            implied.variants.hoist(vec![implied.section], &mut placed);
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

impl Hoist for TypeDeclaration {
    fn hoist<'a>(&'a self, path: Path, placed: &mut Vec<Placed<'a>>) {
        let mut body = path.clone();
        body.push(1);
        placed.push(Placed {
            path,
            declaration: self,
        });
        match self {
            TypeDeclaration::Struct(_, positions) => positions.hoist(body, placed),
            TypeDeclaration::Enum(_, variants) => variants.hoist(body, placed),
            TypeDeclaration::Alias(_, _) => {}
        }
    }
}

impl Hoist for [Position] {
    fn hoist<'a>(&'a self, path: Path, placed: &mut Vec<Placed<'a>>) {
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

impl Hoist for [Variant] {
    fn hoist<'a>(&'a self, path: Path, placed: &mut Vec<Placed<'a>>) {
        let mut at = 0;
        for variant in self {
            let mut body = path.clone();
            body.extend([at, 1]);
            match variant {
                Variant::Struct(_, positions) => positions.hoist(body, placed),
                Variant::Enum(_, variants) => variants.hoist(body, placed),
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

#[cfg(test)]
mod tests {
    use super::{Hoisting, Integer};
    use crate::checking::Identified;
    use crate::{Actualizing, File, Potential};

    #[test]
    fn nested_position_and_variant_declarations_keep_paths_and_order() {
        let source = "Library [] [ Root.{ Label.String Vector<Label> Child.{ Leaf.Integer } Mode.[ On Off ] } Envelope.[ Wrapped.{ Tag.String } Nested.[ Leaf.{ Bytes.String } Empty ] ] ] [] []";
        let file = Potential::<File>::from(source)
            .actualize()
            .expect("nested declarations read");
        let placed = file.declarations();
        let names: Vec<&str> = placed
            .iter()
            .map(|placed| placed.declaration.identity().name.as_ref())
            .collect();
        assert_eq!(
            names,
            [
                "Root", "Label", "Child", "Leaf", "Mode", "Envelope", "Tag", "Bytes"
            ]
        );
        let paths: Vec<Vec<Integer>> = placed.iter().map(|placed| placed.path.clone()).collect();
        assert_eq!(
            paths,
            vec![
                vec![1, 0],
                vec![1, 0, 1, 0],
                vec![1, 0, 1, 3],
                vec![1, 0, 1, 3, 1, 0],
                vec![1, 0, 1, 4],
                vec![1, 1],
                vec![1, 1, 1, 0, 1, 0],
                vec![1, 1, 1, 1, 1, 0, 1, 0],
            ]
        );
        let inlined = file.inlined();
        let names: Vec<&str> = inlined
            .iter()
            .map(|placed| placed.declaration.identity().name.as_ref())
            .collect();
        assert_eq!(names, ["Label", "Child", "Leaf", "Mode", "Tag", "Bytes"]);
    }

    #[test]
    fn shared_types_precede_implied_query_and_response_declarations() {
        let source = "Signal [] [ Launch.{ Name.String } Cancel ] [ Accepted.{ Code.Integer } Refused ] [ Shared.{ Id.Integer } ]";
        let file = Potential::<File>::from(source)
            .actualize()
            .expect("Signal declarations read");
        let placed = file.declarations();
        let names: Vec<&str> = placed
            .iter()
            .map(|placed| placed.declaration.identity().name.as_ref())
            .collect();
        assert_eq!(names, ["Shared", "Id", "Name", "Code"]);
        let paths: Vec<Vec<Integer>> = placed.iter().map(|placed| placed.path.clone()).collect();
        assert_eq!(
            paths,
            vec![
                vec![3, 0],
                vec![3, 0, 1, 0],
                vec![1, 0, 1, 0],
                vec![2, 0, 1, 0],
            ]
        );
    }
}
