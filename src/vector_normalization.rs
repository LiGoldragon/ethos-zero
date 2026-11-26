//! Checked references no longer retain the legacy spelling of the built-in
//! Vector. The original tree remains the source of scope and error paths.

use crate::checking::ResolvingReference;
use crate::{
    Association, Capability, Constraint, Entry, File, Intrinsic, Named, Position,
    ProcessDeclaration, Reference, Resolution, Scope, Signature, TraitBody, TraitDeclaration,
    TypeDeclaration, Variant,
};

/// The capability for normalizing vector references after their authored shape,
/// role and arity have been checked.
pub(crate) trait NormalizingVectors {
    fn normalize_vectors(&mut self, scope: &Scope);
}

/// The capability for producing the semantic file from a validated authored file.
pub(crate) trait NormalizedFile {
    fn normalized(&self) -> File;
}

impl NormalizedFile for File {
    fn normalized(&self) -> File {
        let mut checked = self.clone();
        checked.normalize_vectors(&Scope {
            file: self,
            identity: None,
            associated: &[],
        });
        checked
    }
}

impl<T: NormalizingVectors> NormalizingVectors for [T] {
    fn normalize_vectors(&mut self, scope: &Scope) {
        for member in self {
            member.normalize_vectors(scope);
        }
    }
}

impl<T: NormalizingVectors> NormalizingVectors for Option<T> {
    fn normalize_vectors(&mut self, scope: &Scope) {
        if let Some(member) = self {
            member.normalize_vectors(scope);
        }
    }
}

impl NormalizingVectors for Reference {
    fn normalize_vectors(&mut self, scope: &Scope) {
        match self {
            Reference::Vector(element) => element.normalize_vectors(scope),
            Reference::Named(named) => {
                named.arguments.normalize_vectors(scope);
                if named.name.as_ref() == Intrinsic::Vector.name()
                    && named.resolve_reference(scope) == Resolution::Intrinsic(Intrinsic::Vector)
                {
                    *self = Reference::Vector(Box::new(
                        named
                            .arguments
                            .pop()
                            .expect("checked vector has one element"),
                    ));
                }
            }
        }
    }
}

impl NormalizingVectors for File {
    fn normalize_vectors(&mut self, scope: &Scope) {
        match self {
            File::Library(library) => {
                library.types.normalize_vectors(scope);
                library.traits.normalize_vectors(scope);
                library.associations.normalize_vectors(scope);
            }
            File::Signal(signal) => {
                signal.inputs.normalize_vectors(scope);
                signal.outputs.normalize_vectors(scope);
                signal.types.normalize_vectors(scope);
            }
            File::Operation(operation) => {
                operation.operations.normalize_vectors(scope);
                operation.outcomes.normalize_vectors(scope);
                operation.types.normalize_vectors(scope);
            }
            File::Memory(memory) => {
                memory.records.normalize_vectors(scope);
                memory.changes.normalize_vectors(scope);
                memory.answers.normalize_vectors(scope);
                memory.types.normalize_vectors(scope);
            }
            File::Emission(emission) => {
                emission.emissions.normalize_vectors(scope);
                emission.outcomes.normalize_vectors(scope);
                emission.types.normalize_vectors(scope);
            }
            File::Process(process) => {
                process.processes.normalize_vectors(scope);
                process.types.normalize_vectors(scope);
            }
        }
    }
}

impl NormalizingVectors for Entry {
    fn normalize_vectors(&mut self, scope: &Scope) {
        if let Entry::Declared(declaration) = self {
            declaration.normalize_vectors(scope);
        }
    }
}

impl NormalizingVectors for ProcessDeclaration {
    fn normalize_vectors(&mut self, scope: &Scope) {
        self.configured.normalize_vectors(scope);
        self.gifts.normalize_vectors(scope);
        self.outcome.normalize_vectors(scope);
    }
}

impl NormalizingVectors for TypeDeclaration {
    fn normalize_vectors(&mut self, scope: &Scope) {
        match self {
            TypeDeclaration::Struct(identity, positions) => positions.normalize_vectors(&Scope {
                identity: Some(identity),
                ..*scope
            }),
            TypeDeclaration::Enum(identity, variants) => variants.normalize_vectors(&Scope {
                identity: Some(identity),
                ..*scope
            }),
            TypeDeclaration::Alias(identity, reference) => reference.normalize_vectors(&Scope {
                identity: Some(identity),
                ..*scope
            }),
        }
    }
}

impl NormalizingVectors for Position {
    fn normalize_vectors(&mut self, scope: &Scope) {
        match self {
            Position::Referenced(reference) => reference.normalize_vectors(scope),
            Position::Declared(declaration) => declaration.normalize_vectors(&Scope {
                identity: None,
                associated: &[],
                ..*scope
            }),
        }
    }
}

impl NormalizingVectors for Variant {
    fn normalize_vectors(&mut self, scope: &Scope) {
        match self {
            Variant::Bare(_) => (),
            Variant::Typed(_, reference) => reference.normalize_vectors(scope),
            Variant::Struct(_, positions) => positions.normalize_vectors(scope),
            Variant::Enum(_, variants) => variants.normalize_vectors(scope),
        }
    }
}

impl NormalizingVectors for TraitDeclaration {
    fn normalize_vectors(&mut self, scope: &Scope) {
        self.identity.constraints.normalize_vectors(scope);
        let mut inner = Scope {
            identity: Some(&self.identity),
            ..*scope
        };
        match &mut self.body {
            TraitBody::Simple(capabilities) => capabilities.normalize_vectors(&inner),
            TraitBody::Complex {
                supertraits,
                types,
                constants,
                capabilities,
            } => {
                let associated = types.clone();
                inner.associated = &associated;
                supertraits.normalize_vectors(&inner);
                for ty in types {
                    ty.bounds.normalize_vectors(&inner);
                }
                for constant in constants {
                    constant.ty.normalize_vectors(&inner);
                }
                capabilities.normalize_vectors(&inner);
            }
        }
    }
}

impl NormalizingVectors for Constraint {
    fn normalize_vectors(&mut self, scope: &Scope) {
        match self {
            Constraint::One(reference) => reference.normalize_vectors(scope),
            Constraint::Many(references) => references.normalize_vectors(scope),
        }
    }
}

impl NormalizingVectors for Association {
    fn normalize_vectors(&mut self, scope: &Scope) {
        self.identity.constraints.normalize_vectors(scope);
        self.traits.normalize_vectors(scope);
    }
}

impl NormalizingVectors for Capability {
    fn normalize_vectors(&mut self, scope: &Scope) {
        match &mut self.signature {
            Signature::Yielding(yielded) => yielded.normalize_vectors(scope),
            Signature::Taking(taken, yielded) => {
                taken.normalize_vectors(scope);
                yielded.normalize_vectors(scope);
            }
        }
    }
}
