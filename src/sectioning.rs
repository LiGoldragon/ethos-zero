//! Sectioning: a file's sections by role, whatever its root, and every type
//! it declares, in place or at the top of its types section.
//!
//! Signal and Operation share one shape: imports, the variants of a request
//! enum, the variants of a reply enum, the types they carry. Signal implies
//! `Query` and `Response`; Operation implies `Operation` and `Outcome`.
//! Library and Memory imply nothing.
//!
//! A file holds one root or a vector of them,
//! generated into one module, so its roots
//! share one namespace. Each root's body has a
//! path from the file's node: `[ 1 ]` for a root
//! alone, `[ i 1 ]` for the root at index `i`.
//!
//! A struct position may declare its type in place, `Brief.String`; that
//! type is a declaration of the file like any other, named by its own name,
//! and found by walking the declarations and the implied enums' variants.

use datom_codec::{Integer, Path};
use std::slice::from_ref;

use crate::checking::Spanning;
use crate::checking::{Checkable, Declared};
use crate::{Association, TraitDeclaration};
use crate::{Error, Placing, Root};
use crate::{File, Import, Name, Position};
use crate::{Library, Memory, Operation, Signal};
use crate::{Reference, TypeDeclaration, Variant};

/// An enum a root implies: its name, its
/// variants and the path of the section holding
/// them.
pub(crate) struct Implied<'a> {
    pub(crate) name: Name,
    pub(crate) variants: &'a [Variant],
    pub(crate) path: Path,
}

/// A type declaration and the path of its node
/// from the file's node.
pub(crate) struct Placed<'a> {
    pub(crate) path: Path,
    pub(crate) declaration: &'a TypeDeclaration,
}

/// A root of a file and the path of its body.
pub(crate) struct Body<'a> {
    pub(crate) path: Path,
    pub(crate) root: &'a Root,
}

/// The trait whose capabilities read a root's
/// sections by role.
pub(crate) trait Sectioning {
    /// Where imported names come from.
    fn imports(&self) -> &[Import];
    /// The types section's declarations.
    fn types(&self) -> &[TypeDeclaration];
    /// The path, from the body, of the
    /// declaration at Protos index `at` of the
    /// types section; a `Type` root's one
    /// declaration is the body.
    fn type_path(&self, at: Integer) -> Path;
    /// The enums the root implies, in section
    /// order, each at its path from the body.
    fn implied_enums(&self) -> Vec<Implied<'_>>;
    /// The traits the root declares.
    fn traits(&self) -> &[TraitDeclaration];
    /// The associations the root writes.
    fn associations(&self) -> &[Association];
}

/// A section implying an enum: the enum's name
/// and the variants.
type Section<'a> = (&'a str, &'a [Variant]);

/// The trait whose capability makes the enums
/// the request and reply sections imply, the
/// first section's at `[ 1 ]`.
trait Implicit<'a> {
    fn implied(self) -> Vec<Implied<'a>>;
}

impl<'a> Implicit<'a> for [Section<'a>; 2] {
    fn implied(self) -> Vec<Implied<'a>> {
        let mut implied = Vec::new();
        let sections = (1..).zip(self);
        for (index, section) in sections {
            let (name, variants) = section;
            let name = Name::try_from(name);
            let name = name.expect("static");
            let path = vec![index];
            implied.push(Implied {
                name,
                variants,
                path,
            });
        }
        implied
    }
}

impl Sectioning for Library {
    fn imports(&self) -> &[Import] {
        &self.imports
    }

    fn types(&self) -> &[TypeDeclaration] {
        &self.types
    }

    fn type_path(&self, at: Integer) -> Path {
        vec![1, at]
    }

    fn implied_enums(&self) -> Vec<Implied<'_>> {
        vec![]
    }

    fn traits(&self) -> &[TraitDeclaration] {
        &self.traits
    }

    fn associations(&self) -> &[Association] {
        &self.associations
    }
}

impl Sectioning for Signal {
    fn imports(&self) -> &[Import] {
        &self.imports
    }

    fn types(&self) -> &[TypeDeclaration] {
        &self.types
    }

    fn type_path(&self, at: Integer) -> Path {
        vec![3, at]
    }

    fn implied_enums(&self) -> Vec<Implied<'_>> {
        let queries = &self.queries[..];
        let responses = &self.responses[..];
        let request = ("Query", queries);
        let reply = ("Response", responses);
        [request, reply].implied()
    }

    fn traits(&self) -> &[TraitDeclaration] {
        &self.traits
    }

    fn associations(&self) -> &[Association] {
        &self.associations
    }
}

impl Sectioning for Operation {
    fn imports(&self) -> &[Import] {
        &self.imports
    }

    fn types(&self) -> &[TypeDeclaration] {
        &self.types
    }

    fn type_path(&self, at: Integer) -> Path {
        vec![3, at]
    }

    fn implied_enums(&self) -> Vec<Implied<'_>> {
        let operations = &self.operations[..];
        let outcomes = &self.outcomes[..];
        let request = ("Operation", operations);
        let reply = ("Outcome", outcomes);
        [request, reply].implied()
    }

    fn traits(&self) -> &[TraitDeclaration] {
        &self.traits
    }

    fn associations(&self) -> &[Association] {
        &self.associations
    }
}

impl Sectioning for Memory {
    fn imports(&self) -> &[Import] {
        &self.imports
    }

    fn types(&self) -> &[TypeDeclaration] {
        &self.types
    }

    fn type_path(&self, at: Integer) -> Path {
        vec![1, at]
    }

    fn implied_enums(&self) -> Vec<Implied<'_>> {
        vec![]
    }

    fn traits(&self) -> &[TraitDeclaration] {
        &self.traits
    }

    fn associations(&self) -> &[Association] {
        &self.associations
    }
}

/// A `Type` root's one declaration is its body.
impl Sectioning for TypeDeclaration {
    fn imports(&self) -> &[Import] {
        &[]
    }

    fn types(&self) -> &[TypeDeclaration] {
        std::slice::from_ref(self)
    }

    fn type_path(&self, _: Integer) -> Path {
        vec![]
    }

    fn implied_enums(&self) -> Vec<Implied<'_>> {
        vec![]
    }

    fn traits(&self) -> &[TraitDeclaration] {
        &[]
    }

    fn associations(&self) -> &[Association] {
        &[]
    }
}

/// What every head's body bears: its check and
/// its declared names.
pub(crate) trait Checked: Checkable + Declared {}

impl<T: Checkable + Declared> Checked for T {}

/// What every head's body bears: its sections
/// read by role, its check, its declared names.
pub(crate) trait Contents: Sectioning + Checked {}

impl<T: Sectioning + Checked> Contents for T {}

/// The trait whose capability opens a root on
/// the body its head holds.
pub(crate) trait Opening {
    fn contents(&self) -> &dyn Contents;
}

impl Opening for Root {
    fn contents(&self) -> &dyn Contents {
        match self {
            Root::Library(root) => root,
            Root::Signal(root) => root,
            Root::Operation(root) => root,
            Root::Memory(root) => root,
            Root::Type(root) => root,
        }
    }
}

impl Sectioning for Root {
    fn imports(&self) -> &[Import] {
        self.contents().imports()
    }

    fn types(&self) -> &[TypeDeclaration] {
        self.contents().types()
    }

    fn type_path(&self, at: Integer) -> Path {
        self.contents().type_path(at)
    }

    fn implied_enums(&self) -> Vec<Implied<'_>> {
        self.contents().implied_enums()
    }

    fn traits(&self) -> &[TraitDeclaration] {
        self.contents().traits()
    }

    fn associations(&self) -> &[Association] {
        self.contents().associations()
    }
}

/// The trait whose capabilities read a file's
/// roots, each at the path of its body.
pub(crate) trait Rooting {
    /// Every root, with its body's path.
    fn bodies(&self) -> Vec<Body<'_>>;
    /// Every enum the roots imply, each at its
    /// path from the file's node.
    fn implied_enums(&self) -> Vec<Implied<'_>>;
}

impl Rooting for File {
    fn bodies(&self) -> Vec<Body<'_>> {
        let roots = match self {
            File::One(root) => from_ref(root),
            File::Many(roots) => roots.as_slice(),
        };
        let alone = matches!(self, File::One(_));
        let mut bodies = Vec::new();
        let each = roots.iter().enumerate();
        for (index, root) in each {
            let at = index as Integer;
            let path = match alone {
                true => vec![1],
                false => vec![at, 1],
            };
            bodies.push(Body { path, root });
        }
        bodies
    }

    fn implied_enums(&self) -> Vec<Implied<'_>> {
        let mut implied = Vec::new();
        for body in self.bodies() {
            let root = body.root;
            for mut each in root.implied_enums() {
                let mut path = body.path.clone();
                path.append(&mut each.path);
                each.path = path;
                implied.push(each);
            }
        }
        implied
    }
}

/// The trait whose capability places an error
/// below every step of a path.
pub(crate) trait Prefixing {
    fn prefixed(self, path: &Path) -> Self;
}

impl<T> Prefixing for Result<T, Error> {
    fn prefixed(self, path: &Path) -> Self {
        let mut placed = self;
        for step in path.iter().rev() {
            placed = placed.place(*step);
        }
        placed
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

impl Hoisting for Root {
    fn declarations(&self) -> Vec<Placed<'_>> {
        let mut placed = Vec::new();
        let mut at = 0;
        for declaration in self.types() {
            declaration.hoist(self.type_path(at), &mut placed);
            at += declaration.span();
        }
        for implied in self.implied_enums() {
            let path = implied.path;
            let variants = implied.variants;
            let all = &mut placed;
            variants.hoist_variants(path, all);
        }
        placed
    }

    // A declaration of the types section sits at its `type_path`; one
    // declared in place sits deeper.
    fn inlined(&self) -> Vec<Placed<'_>> {
        let mut tops = Vec::new();
        let mut at = 0;
        for declaration in self.types() {
            tops.push(self.type_path(at));
            at += declaration.span();
        }
        let mut inlined = Vec::new();
        for placed in self.declarations() {
            if !tops.contains(&placed.path) {
                inlined.push(placed);
            }
        }
        inlined
    }
}

/// Every root's declarations, in root order,
/// each at its path from the file's node.
impl Hoisting for File {
    fn declarations(&self) -> Vec<Placed<'_>> {
        let mut placed = Vec::new();
        for body in self.bodies() {
            let root = body.root;
            for mut each in root.declarations() {
                let mut path = body.path.clone();
                path.append(&mut each.path);
                each.path = path;
                placed.push(each);
            }
        }
        placed
    }

    fn inlined(&self) -> Vec<Placed<'_>> {
        let mut placed = Vec::new();
        for body in self.bodies() {
            let root = body.root;
            for mut each in root.inlined() {
                let mut path = body.path.clone();
                path.append(&mut each.path);
                each.path = path;
                placed.push(each);
            }
        }
        placed
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
            TypeDeclaration::Sealed(_, positions) | TypeDeclaration::Shared(_, positions) => {
                let mut shape = body;
                shape.push(1);
                positions.hoist_positions(shape, placed)
            }
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
