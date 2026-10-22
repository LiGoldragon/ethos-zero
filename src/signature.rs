//! Signature: where a reference in a capability's signature stands.
//!
//! A capability speaks in Self, the trait's own parameters and other traits;
//! a concrete type in an input is a trait not yet named. So a reference in
//! an input stands as one of four things: kept as it is written (Self, a
//! parameter of the trait's head, an associated type), a trait the method
//! takes a parameter bounded by, a trait an associated type of the head
//! already binds, or a concrete type, which an input refuses. A yield may
//! name a concrete type; a declared trait there is a parameter too.
//!
//! An imported name says nothing of its role: in an input it is taken as a
//! trait, in a yield as a type.

use crate::{Identifiable, Intrinsic, Name, Reference, Resolution, Resolving, Scope};

/// Where a reference in a capability's signature stands.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Stance {
    /// Written as it is: Self, a head parameter, an associated type, or a
    /// name whose error the ordinary reference check reports.
    Kept,
    /// A trait: the method takes a parameter it bounds.
    Bounding,
    /// A trait an associated type of the enclosing trait already binds.
    Bound(Name),
    /// A concrete type.
    Concrete,
}

/// The position a signature reference stands in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Place {
    Input,
    Yield,
}

/// The trait whose capability says where a signature reference stands.
pub(crate) trait Standing {
    fn stance(&self, scope: &Scope, place: Place) -> Stance;
}

/// The trait whose capability says whether an intrinsic is a type.
trait Concreting {
    fn concrete(&self) -> bool;
}

impl Concreting for Intrinsic {
    fn concrete(&self) -> bool {
        !matches!(self, Intrinsic::Itself | Intrinsic::Sized)
    }
}

/// The trait whose capability finds the associated type a trait already binds.
trait Binding {
    fn binding(&self, scope: &Scope) -> Stance;
}

impl Binding for Reference {
    fn binding(&self, scope: &Scope) -> Stance {
        for associated in scope.associated {
            if associated.bounds.iter().any(|bound| {
                bound.arguments.is_empty() && bound.source == self.source && bound.name == self.name
            }) {
                return Stance::Bound(associated.name.clone());
            }
        }
        Stance::Bounding
    }
}

impl Standing for Reference {
    fn stance(&self, scope: &Scope, place: Place) -> Stance {
        if let Some(source) = &self.source {
            return match Intrinsic::identify(&self.name.0) {
                Some(intrinsic) if source.as_ref() == "protos" && intrinsic.concrete() => {
                    Stance::Concrete
                }
                _ if place == Place::Input => self.binding(scope),
                _ => Stance::Kept,
            };
        }
        match scope.resolve(&self.name) {
            Resolution::Intrinsic(intrinsic) if intrinsic.concrete() => Stance::Concrete,
            Resolution::Intrinsic(Intrinsic::Sized) => self.binding(scope),
            Resolution::Type(_) => Stance::Concrete,
            Resolution::Trait(_) => self.binding(scope),
            Resolution::Imported(source, emitted) => match Intrinsic::identify(&emitted.0) {
                Some(intrinsic) if source.as_ref() == "protos" && intrinsic.concrete() => {
                    Stance::Concrete
                }
                _ if place == Place::Input => self.binding(scope),
                _ => Stance::Kept,
            },
            Resolution::Intrinsic(_)
            | Resolution::Parameter(_)
            | Resolution::Associated(_)
            | Resolution::Ambiguous(_)
            | Resolution::Undeclared => Stance::Kept,
        }
    }
}
