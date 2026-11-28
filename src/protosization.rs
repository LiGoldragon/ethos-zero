//! Protosization: the infallible structural ascent of a checked Ethos file.

use protos::{
    Canonicalizable, Enclosure, Extent, ExtentAccess, Protos, Protosizable, Separator, Symbol,
};

use crate::{
    AssociatedConstant, AssociatedType, Association, Capability, Constraint, Emission, Entry, File,
    Identity, Import, Imported, Library, Matrix, Memory, Operation, Organism, Pointer, Position,
    Process, ProcessDeclaration, Receiver, Reference, ReferenceAccess, Signal, Signature, Source,
    Spelled, TraitBody, TraitDeclaration, TypeDeclaration, Variant,
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
            extent: Extent::from_offsets(0, 0),
            text: self.to_owned(),
        }
    }
    fn enclosed(&self, enclosure: Enclosure, children: Vec<Protos>) -> Protos {
        Protos::Enclosed {
            extent: Extent::from_offsets(0, 0),
            enclosure,
            children,
        }
    }
    fn headed(&self, constraints: Option<Protos>, separator: Separator, body: Protos) -> Protos {
        Protos::Headed {
            extent: Extent::from_offsets(0, 0),
            head: Symbol(self.to_owned()),
            constraints: constraints.map(Box::new),
            separator,
            body: Box::new(body),
        }
    }
}

/// The trait whose capabilities write a source before what it qualifies.
///
/// A rooted source is one bare run with the name, `:flow_ethos:message:Type`,
/// or the chain an inline alias's colon heads, `custom:sub:Name` in
/// `Topic:custom:sub:Name`; a chained source heads the name, each segment
/// heading the rest, `protos:Error` or `custom:sub:Error`.
trait Qualifying {
    fn qualified(&self, name: &str) -> Protos;
    fn rooted_text(&self, tail: &str) -> String;
    fn chain(&self, body: Protos) -> Protos;
}
impl Qualifying for Source {
    fn qualified(&self, name: &str) -> Protos {
        if self.rooted() {
            self.rooted_text(name).as_str().bare()
        } else {
            self.chain(name.bare())
        }
    }
    fn rooted_text(&self, tail: &str) -> String {
        let mut text = String::new();
        for segment in self.segments() {
            text.push(':');
            text.push_str(&segment.0);
        }
        text.push(':');
        text.push_str(tail);
        text
    }
    fn chain(&self, body: Protos) -> Protos {
        let mut node = body;
        for segment in self.segments().iter().rev() {
            node = segment.0.as_str().headed(None, Separator::Colon, node);
        }
        node
    }
}

/// An alias whose source is rooted is written with the import inline, the
/// colon in place of the period: `Topic:custom:sub:Name`.
trait AliasHeading {
    fn alias(&self, reference: &Reference) -> Protos;
}
impl AliasHeading for Identity {
    fn alias(&self, reference: &Reference) -> Protos {
        let Some(reference) = reference.named() else {
            return self.heading(Separator::Period, reference.protos());
        };
        match &reference.source {
            Some(source) if source.rooted() => self.heading(
                Separator::Colon,
                source.chain(reference.name.as_ref().bare()),
            ),
            Some(source) => {
                self.heading(Separator::Period, source.qualified(reference.name.as_ref()))
            }
            None => self.heading(Separator::Period, reference.name.as_ref().bare()),
        }
    }
}

/// An imports section: a rooted import of several names is its bare source
/// and the bracket beside it, `:protos:[ String Integer ]`.
trait ImportsProtosizing {
    fn import_nodes(&self) -> Vec<Protos>;
}
impl ImportsProtosizing for [Import] {
    fn import_nodes(&self) -> Vec<Protos> {
        let mut nodes = Vec::new();
        for import in self {
            match import {
                Import::Many(source, imported) if source.rooted() => {
                    nodes.push(source.rooted_text("").as_str().bare());
                    nodes.push("".enclosed(Enclosure::Bracketed, imported.protos_list()));
                }
                _ => nodes.push(import.protos()),
            }
        }
        nodes
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
            if let Reference::Named(reference) = reference
                && !reference.arguments.is_empty()
            {
                let base = match &reference.source {
                    Some(source) => source.qualified(reference.name.as_ref()),
                    None => reference.name.as_ref().bare(),
                };
                nodes.push(base);
                nodes.push("".enclosed(Enclosure::Angled, reference.arguments.reference_nodes()));
            } else {
                nodes.push(reference.protos());
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
/// a headed `Item<...>.` invents a separator and cannot be read back.  What
/// protos would need for the unsplit shape is stated on `Constraining` in
/// `conception`.
trait AssociatedTypesProtosizing {
    fn associated_type_nodes(&self) -> Vec<Protos>;
}
impl AssociatedTypesProtosizing for [AssociatedType] {
    fn associated_type_nodes(&self) -> Vec<Protos> {
        let mut nodes = Vec::new();
        for associated in self {
            nodes.push(associated.name.as_ref().bare());
            if !associated.bounds.is_empty() {
                nodes.push("".enclosed(Enclosure::Angled, associated.bounds.reference_nodes()));
            }
        }
        nodes
    }
}
/// A variant carrying a type with arguments is a headed name and the
/// arguments' angled enclosure beside it in the list: `Maybe.Option<Lock>`.
trait VariantsProtosizing {
    fn variant_nodes(&self) -> Vec<Protos>;
}
impl VariantsProtosizing for [Variant] {
    fn variant_nodes(&self) -> Vec<Protos> {
        let mut nodes = Vec::new();
        for variant in self {
            match variant {
                Variant::Typed(name, Reference::Named(reference))
                    if !reference.arguments.is_empty() =>
                {
                    let body = match &reference.source {
                        Some(source) => source.qualified(reference.name.as_ref()),
                        None => reference.name.as_ref().bare(),
                    };
                    nodes.push(name.as_ref().headed(None, Separator::Period, body));
                    nodes.push(
                        "".enclosed(Enclosure::Angled, reference.arguments.reference_nodes()),
                    );
                }
                _ => nodes.push(variant.protos()),
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
                TypeDeclaration::Alias(identity, reference @ Reference::Named(named))
                    if !named.arguments.is_empty() =>
                {
                    nodes.push(identity.alias(reference));
                    nodes.push("".enclosed(Enclosure::Angled, named.arguments.reference_nodes()));
                }
                _ => nodes.push(declaration.protos()),
            }
        }
        nodes
    }
}

/// A struct's positions: each reference, or each declaration in place, with
/// the angled enclosure of its arguments beside it.
trait PositionsProtosizing {
    fn position_nodes(&self) -> Vec<Protos>;
}
impl PositionsProtosizing for [Position] {
    fn position_nodes(&self) -> Vec<Protos> {
        let mut nodes = Vec::new();
        for position in self {
            match position {
                Position::Referenced(reference) => {
                    nodes.extend(std::slice::from_ref(reference).reference_nodes())
                }
                Position::Declared(declaration) => {
                    nodes.extend(std::slice::from_ref(declaration).declaration_nodes())
                }
            }
        }
        nodes
    }
}

impl Protosizing for Reference {
    fn protos(&self) -> Protos {
        let reference = match self {
            Self::Vector(element) => {
                let children = std::slice::from_ref(element.as_ref()).reference_nodes();
                return "".enclosed(Enclosure::WhiteBracketed, children);
            }
            Self::Named(reference) => reference,
        };
        let base = reference.name.as_ref().bare();
        let applied = if reference.arguments.is_empty() {
            base
        } else {
            let arguments = "".enclosed(Enclosure::Angled, reference.arguments.protos_list());
            "".enclosed(Enclosure::Braced, vec![base, arguments])
        };
        match &reference.source {
            Some(source) if source.rooted() => {
                let base = source.qualified(reference.name.as_ref());
                if reference.arguments.is_empty() {
                    base
                } else {
                    let arguments =
                        "".enclosed(Enclosure::Angled, reference.arguments.protos_list());
                    "".enclosed(Enclosure::Braced, vec![base, arguments])
                }
            }
            Some(source) => source.chain(applied),
            None => applied,
        }
    }
}

impl Protosizing for Constraint {
    fn protos(&self) -> Protos {
        match self {
            Self::One(reference) => reference.protos(),
            Self::Many(references) => {
                "".enclosed(Enclosure::Bracketed, references.reference_nodes())
            }
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
            Self::One(source, imported) if source.rooted() => {
                let tail = if imported.name == imported.emitted {
                    imported.name.as_ref().to_owned()
                } else {
                    format!("{}.{}", imported.name.as_ref(), imported.emitted.as_ref())
                };
                source.rooted_text(&tail).as_str().bare()
            }
            Self::One(source, imported) => source.chain(imported.protos()),
            Self::Many(source, imported) => {
                source.chain("".enclosed(Enclosure::Bracketed, imported.protos_list()))
            }
        }
    }
}
impl Protosizing for TypeDeclaration {
    fn protos(&self) -> Protos {
        match self {
            Self::Struct(identity, positions) => identity.heading(
                Separator::Period,
                "".enclosed(Enclosure::Braced, positions.position_nodes()),
            ),
            Self::Enum(identity, variants) => identity.heading(
                Separator::Period,
                "".enclosed(Enclosure::Bracketed, variants.variant_nodes()),
            ),
            Self::Alias(identity, reference) => match reference.named() {
                Some(named) if named.source.as_ref().is_some_and(Spelled::rooted) => {
                    identity.alias(reference)
                }
                _ => identity.heading(Separator::Period, reference.protos()),
            },
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
                "".enclosed(Enclosure::Braced, positions.position_nodes()),
            ),
            Self::Enum(name, variants) => name.as_ref().headed(
                None,
                Separator::Period,
                "".enclosed(Enclosure::Bracketed, variants.variant_nodes()),
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
            Signature::Yielding(yielding) => {
                "".enclosed(Enclosure::Bracketed, yielding.as_slice().reference_nodes())
            }
            Signature::Taking(inputs, yielding) => "".enclosed(
                Enclosure::Braced,
                vec![
                    "".enclosed(Enclosure::Bracketed, inputs.reference_nodes()),
                    "".enclosed(Enclosure::Bracketed, yielding.as_slice().reference_nodes()),
                ],
            ),
        };
        self.name.as_ref().headed(None, separator, body)
    }
}
impl Protosizing for TraitDeclaration {
    fn protos(&self) -> Protos {
        let body = match &self.body {
            TraitBody::Simple(capabilities) => {
                "".enclosed(Enclosure::Bracketed, capabilities.protos_list())
            }
            TraitBody::Complex {
                supertraits,
                types,
                constants,
                capabilities,
            } => "".enclosed(
                Enclosure::Braced,
                vec![
                    "".enclosed(Enclosure::Bracketed, supertraits.reference_nodes()),
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
            "".enclosed(Enclosure::Bracketed, self.traits.reference_nodes()),
        )
    }
}
impl Protosizing for Library {
    fn protos(&self) -> Protos {
        "".enclosed(
            Enclosure::Braced,
            vec![
                "".enclosed(Enclosure::Bracketed, self.imports.import_nodes()),
                "".enclosed(Enclosure::Bracketed, self.types.declaration_nodes()),
                "".enclosed(Enclosure::Bracketed, self.traits.protos_list()),
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
                "".enclosed(Enclosure::Bracketed, self.imports.import_nodes()),
                "".enclosed(Enclosure::Bracketed, self.inputs.variant_nodes()),
                "".enclosed(Enclosure::Bracketed, self.outputs.variant_nodes()),
                "".enclosed(Enclosure::Bracketed, self.types.declaration_nodes()),
            ],
        )
    }
}
impl Protosizing for Operation {
    fn protos(&self) -> Protos {
        "".enclosed(
            Enclosure::Braced,
            vec![
                "".enclosed(Enclosure::Bracketed, self.imports.import_nodes()),
                "".enclosed(Enclosure::Bracketed, self.operations.variant_nodes()),
                "".enclosed(Enclosure::Bracketed, self.outcomes.variant_nodes()),
                "".enclosed(Enclosure::Bracketed, self.types.declaration_nodes()),
            ],
        )
    }
}
impl Protosizing for Emission {
    fn protos(&self) -> Protos {
        "".enclosed(
            Enclosure::Braced,
            vec![
                "".enclosed(Enclosure::Bracketed, self.imports.import_nodes()),
                "".enclosed(Enclosure::Bracketed, self.emissions.variant_nodes()),
                "".enclosed(Enclosure::Bracketed, self.outcomes.variant_nodes()),
                "".enclosed(Enclosure::Bracketed, self.types.declaration_nodes()),
            ],
        )
    }
}
impl Protosizing for ProcessDeclaration {
    fn protos(&self) -> Protos {
        let mut parts = vec![
            "".enclosed(Enclosure::Bracketed, self.configured.reference_nodes()),
            "".enclosed(Enclosure::Bracketed, self.gifts.reference_nodes()),
        ];
        parts.extend(self.outcome.as_slice().reference_nodes());
        self.name.as_ref().headed(
            None,
            Separator::Period,
            "".enclosed(Enclosure::Braced, parts),
        )
    }
}
impl Protosizing for Process {
    fn protos(&self) -> Protos {
        "".enclosed(
            Enclosure::Braced,
            vec![
                "".enclosed(Enclosure::Bracketed, self.imports.import_nodes()),
                "".enclosed(Enclosure::Bracketed, self.processes.protos_list()),
                "".enclosed(Enclosure::Bracketed, self.types.declaration_nodes()),
            ],
        )
    }
}
/// A pointer is written as its source stands, the head before it: a rooted
/// source is one bare run, `:flow:signal`; a chained one heads itself,
/// `flow:signal`, `Codex:harness:signal`.
impl Protosizing for Pointer {
    fn protos(&self) -> Protos {
        let mut segments = Vec::with_capacity(self.source.segments().len());
        for segment in self.source.segments() {
            segments.push(segment.0.as_str());
        }
        if self.source.rooted() {
            let mut text = String::new();
            for segment in &segments {
                text.push(':');
                text.push_str(segment);
            }
            return text.as_str().bare();
        }
        let last = segments.pop().unwrap_or_default();
        let mut node = last.bare();
        for segment in segments.iter().rev() {
            node = segment.headed(None, Separator::Colon, node);
        }
        match &self.head {
            Some(head) => head.as_ref().headed(None, Separator::Colon, node),
            None => node,
        }
    }
}
impl Protosizing for Organism {
    fn protos(&self) -> Protos {
        "".enclosed(
            Enclosure::Braced,
            vec![
                "".enclosed(Enclosure::Bracketed, self.receptor.protos_list()),
                "".enclosed(Enclosure::Bracketed, self.emitter.protos_list()),
                "".enclosed(Enclosure::Bracketed, self.memory.protos_list()),
                "".enclosed(Enclosure::Bracketed, self.organs.protos_list()),
            ],
        )
    }
}
impl Protosizing for Memory {
    fn protos(&self) -> Protos {
        "".enclosed(
            Enclosure::Braced,
            vec![
                "".enclosed(Enclosure::Bracketed, self.imports.import_nodes()),
                "".enclosed(Enclosure::Bracketed, self.records.entry_nodes()),
                "".enclosed(Enclosure::Bracketed, self.changes.entry_nodes()),
                "".enclosed(Enclosure::Bracketed, self.answers.entry_nodes()),
                "".enclosed(Enclosure::Bracketed, self.types.declaration_nodes()),
            ],
        )
    }
}
impl Protosizing for Matrix {
    fn protos(&self) -> Protos {
        "".enclosed(
            Enclosure::Braced,
            vec![
                "".enclosed(Enclosure::Bracketed, self.imports.import_nodes()),
                "".enclosed(Enclosure::Bracketed, self.records.entry_nodes()),
            ],
        )
    }
}
/// The trait whose capability writes the entries of a Memory or Matrix section: a name
/// bare, a declaration as a declaration is written.
trait EntriesProtosizing {
    fn entry_nodes(&self) -> Vec<Protos>;
}
impl EntriesProtosizing for [Entry] {
    fn entry_nodes(&self) -> Vec<Protos> {
        let mut nodes = Vec::new();
        for entry in self {
            match entry {
                Entry::Named(name) => nodes.push(name.as_ref().bare()),
                Entry::Declared(declaration) => {
                    nodes.extend(std::slice::from_ref(declaration).declaration_nodes())
                }
            }
        }
        nodes
    }
}
impl Protosizing for File {
    fn protos(&self) -> Protos {
        let raw = match self {
            Self::Library(library) => "Library".headed(None, Separator::Period, library.protos()),
            Self::Signal(signal) => "Signal".headed(None, Separator::Period, signal.protos()),
            Self::Operation(operation) => {
                "Operation".headed(None, Separator::Period, operation.protos())
            }
            Self::Memory(memory) => "Memory".headed(None, Separator::Period, memory.protos()),
            Self::Matrix(matrix) => "Matrix".headed(None, Separator::Period, matrix.protos()),
            Self::Emission(emission) => {
                "Emission".headed(None, Separator::Period, emission.protos())
            }
            Self::Process(process) => "Process".headed(None, Separator::Period, process.protos()),
            Self::Organism(organism) => {
                "Organism".headed(None, Separator::Period, organism.protos())
            }
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
