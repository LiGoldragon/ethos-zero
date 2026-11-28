//! What a type reaches: the open enums of its file a value holds, decided
//! once for a file, where every file given is seen.
//!
//! A value reaches an open enum it names, bare or by a source that names the
//! file itself (`:flow:signal:Model`, `signal` given as the namespace `flow`),
//! or that a type the file declares reaches, through struct positions, variant
//! payloads, vectors, options and every type argument. Reach is the closure
//! over what a type holds: `Self` adds nothing to it, so a recursive type that
//! holds no open enum reaches none. Check's `Unused` finding, the `Carried` a
//! Signal exports, and the `Opening` the generator writes all read one
//! [`Reach`].

use crate::checking::Identified;
use crate::sectioning::{Hoisting, Sectioning};
use crate::{File, Name, Position, Reference, Source, TypeDeclaration, Variant};

/// What the types of one file reach: the open enums it names, each source it
/// writes that names the file itself, and each type it declares, inline ones
/// among them, with the open enums that type reaches.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Reach {
    opened: Vec<Name>,
    homes: Vec<Source>,
    reached: Vec<(Name, Vec<Name>)>,
}

/// The trait whose capability decides what a file's types reach, given the
/// sources it writes that name the file itself.
pub(crate) trait Reckoning {
    fn reckon(&self, homes: Vec<Source>) -> Reach;
}

impl Reckoning for File {
    fn reckon(&self, homes: Vec<Source>) -> Reach {
        let opened = self.opened().to_vec();
        let declarations = self.declarations();
        let mut reach = Reach {
            reached: declarations
                .iter()
                .map(|placed| (placed.declaration.identity().name.clone(), Vec::new()))
                .collect(),
            opened,
            homes,
        };
        if reach.opened.is_empty() {
            return reach;
        }
        // The closure: each pass adds what a type holds through the types it
        // names, until a pass adds nothing.
        loop {
            let mut grown = false;
            for (at, placed) in declarations.iter().enumerate() {
                let held = reach.opens_held(placed.declaration);
                if held.len() > reach.reached[at].1.len() {
                    reach.reached[at].1 = held;
                    grown = true;
                }
            }
            if !grown {
                return reach;
            }
        }
    }
}

/// The trait whose capabilities read a [`Reach`]: the open enums a value
/// reaches, and the types a Signal's open enums can carry.
pub(crate) trait Reaching {
    /// The open enums of the file the value reaches, in the order the file names them.
    fn opens_held<Held: Bearing + ?Sized>(&self, value: &Held) -> Vec<Name>;
    /// The types a Signal's open enums can carry, each a variant of its
    /// `Carried`: every type the file declares at the top of its types section
    /// that is not generic and reaches no open enum, which would make the
    /// value hold itself. None leaves the file without `Carried`, which it
    /// then neither generates nor exports.
    fn carriable<'a>(&self, file: &'a File) -> Vec<&'a TypeDeclaration>;
}

impl Reaching for Reach {
    fn opens_held<Held: Bearing + ?Sized>(&self, value: &Held) -> Vec<Name> {
        self.opened
            .iter()
            .filter(|open| value.bears(open, self))
            .cloned()
            .collect()
    }

    fn carriable<'a>(&self, file: &'a File) -> Vec<&'a TypeDeclaration> {
        let mut carriable = Vec::new();
        if self.opened.is_empty() {
            return carriable;
        }
        for declaration in file.types() {
            if declaration.identity().constraints.is_empty()
                && self.opens_held(declaration).is_empty()
            {
                carriable.push(declaration);
            }
        }
        carriable
    }
}

/// The trait whose capability says whether a value holds an open enum, by
/// what a [`Reach`] decided of the types it names.
pub(crate) trait Bearing {
    fn bears(&self, open: &Name, reach: &Reach) -> bool;
}

impl Bearing for Reference {
    fn bears(&self, open: &Name, reach: &Reach) -> bool {
        let reference = match self {
            Reference::Vector(element) => return element.bears(open, reach),
            Reference::Named(reference) => reference,
        };
        let home = match &reference.source {
            None => true,
            Some(source) => reach.homes.contains(source),
        };
        if home && reference.name == *open {
            return true;
        }
        if home && reach.declared(&reference.name, open) {
            return true;
        }
        reference.arguments.bears(open, reach)
    }
}

impl Bearing for Position {
    fn bears(&self, open: &Name, reach: &Reach) -> bool {
        match self {
            Position::Referenced(reference) => reference.bears(open, reach),
            Position::Declared(declaration) => declaration.bears(open, reach),
        }
    }
}

impl Bearing for TypeDeclaration {
    fn bears(&self, open: &Name, reach: &Reach) -> bool {
        match self {
            TypeDeclaration::Struct(_, positions) => positions.bears(open, reach),
            TypeDeclaration::Enum(_, variants) => variants.bears(open, reach),
            TypeDeclaration::Alias(_, aliased) => aliased.bears(open, reach),
        }
    }
}

/// A bare variant holds what the type the file declares of its name reaches.
impl Bearing for Variant {
    fn bears(&self, open: &Name, reach: &Reach) -> bool {
        match self {
            Variant::Bare(name) => reach.declared(name, open),
            Variant::Typed(_, reference) => reference.bears(open, reach),
            Variant::Struct(_, positions) => positions.bears(open, reach),
            Variant::Enum(_, variants) => variants.bears(open, reach),
        }
    }
}

impl<Held: Bearing> Bearing for [Held] {
    fn bears(&self, open: &Name, reach: &Reach) -> bool {
        self.iter().any(|element| element.bears(open, reach))
    }
}

/// The trait whose capability says whether a type the file declares reaches
/// an open enum, as the closure has decided so far.
trait Declared {
    fn declared(&self, name: &Name, open: &Name) -> bool;
}

impl Declared for Reach {
    fn declared(&self, name: &Name, open: &Name) -> bool {
        self.reached
            .iter()
            .any(|(declared, opens)| declared == name && opens.contains(open))
    }
}
