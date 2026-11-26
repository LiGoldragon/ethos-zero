//! The Nexus: the root files of one namespace, read together.
//!
//! A Nexus is its roots, each a module named for its file: a Signal, a meta
//! Signal, a Memory, an Emission, a Library and a Process. They name each
//! other by the Nexus's own namespace, the crate that holds them: a module
//! as `tally:signal:[ Total ]`, the Library at the crate's root as
//! `tally:[ Add ]`. The namespace is written nowhere: it is the one first
//! segment whose imports, those that name a module of the set, land on one that declares
//! what is imported.
//!
//! What a Process root names, it names from the other roots: the input or
//! record a process is born of, and each thing it gives, an output, a
//! record, a change, an emission or a process. Read together they say what
//! each process holds, what returns for each thing it gives, and so the
//! `Gift`, `Taken`, `Outcome` and `Process` enums over them, which the
//! Process root's module is generated with.
//!
//! What returns for a thing given: nothing for an output; the standard
//! `process::Answer` for a record kept; the `Result` of its answer or
//! `process::Unremembered` for a change, the answer being the one of the
//! change's place; the `Result` of its outcome or `process::Unobserved` for
//! an emission; and for a process, its outcome at its end. Where one kind of
//! thing returns to a process, its `Taken` is that type, bare.

use datom_codec::Integer;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use crate::vector_normalization::NormalizedFile;

use crate::checking::{
    Carrying, Declaring, Identified, Inlining, Payloading, Spanning, Titled, Uniting,
};
use crate::generation::{
    DatomDeriving, Emitting, FieldNaming, Identifying, Itemizing, Rendering, SnakeCasing,
    StructFielding, Tokening,
};
use crate::sectioning::{ReferencingEach, Sectioning};
use crate::{
    ConceptualErroring, Entry, Error, File, Generating, Identity, Import, Imported, Name,
    NamedReference, Problem, Reference, ReferenceAccess, Resolution, Resolving, Scope, Source,
    Spelled, TypeDeclaration, Validating, Variant,
};

/// One root file of a Nexus and the name its module bears: the file's stem.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Module {
    /// The module's name, the stem of its file.
    pub stem: String,
    /// The root read from it.
    pub file: File,
}

/// The roots of one namespace, read together.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Nexus {
    /// The roots, in the order they were given.
    pub modules: Vec<Module>,
}

/// An error and the module of the Nexus it lies in, by its place in the set.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Refusal {
    /// The place of the module in the Nexus.
    pub module: usize,
    /// The error, at a path of that module's file.
    pub error: Error,
}

/// The Rust a module of a Nexus is generated as.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Assembled {
    /// The module's name, the stem its file had.
    pub stem: String,
    /// The module's Rust text.
    pub rust: String,
}

/// The trait whose capability checks a Nexus whole: each root, then the roots among one another.
pub trait Verifying {
    /// The refusal that would prevent generation, if any.
    fn verify(&self) -> Result<(), Refusal>;
}

/// The trait whose capability generates every module of a Nexus, a Process
/// root's with the enums and impls its processes need from the other roots.
pub trait Assembling {
    /// Each module's Rust, or the refusal that prevents generation.
    fn assemble(&self) -> Result<Vec<Assembled>, Refusal>;
}

// ---------------------------------------------------------------------------
// Where a name is declared
// ---------------------------------------------------------------------------

/// The trait whose capability says whether a root declares a name, as a
/// type, a trait, a variant of the enum it implies, a unit or an entry that
/// names one: what another root of the Nexus may import from it.
trait Providing {
    fn provides(&self, name: &Name) -> bool;
}

impl Providing for File {
    fn provides(&self, name: &Name) -> bool {
        if self.declaration(name).is_some() {
            return true;
        }
        if let File::Library(library) = self {
            for declared in &library.traits {
                if &declared.identity.name == name {
                    return true;
                }
            }
        }
        if let File::Memory(memory) = self {
            for entries in [&memory.records, &memory.changes, &memory.answers] {
                for entry in entries {
                    if let Entry::Named(named) = entry
                        && named == name
                    {
                        return true;
                    }
                }
            }
        }
        for implied in self.implied_enums() {
            for variant in implied.variants {
                if variant.title() == name {
                    return true;
                }
            }
        }
        for unit in self.units() {
            if &unit.name == name {
                return true;
            }
        }
        false
    }
}

/// The trait whose capability reads what an import takes from where.
trait Importing {
    fn source(&self) -> &Source;
    fn imported(&self) -> Vec<&Imported>;
}

impl Importing for Import {
    fn source(&self) -> &Source {
        match self {
            Import::One(source, _) | Import::Many(source, _) => source,
        }
    }

    fn imported(&self) -> Vec<&Imported> {
        match self {
            Import::One(_, imported) => vec![imported],
            Import::Many(_, imported) => imported.iter().collect(),
        }
    }
}

/// The trait whose capability names an entry's type.
trait Entered {
    fn entered(&self) -> &Name;
}

impl Entered for Entry {
    fn entered(&self) -> &Name {
        match self {
            Entry::Declared(declaration) => &declaration.identity().name,
            Entry::Named(name) => name,
        }
    }
}

/// The trait whose capability names the type a root writes for a name it provides:
/// the name of the payload a variant declares in place, the name itself for every
/// other name, a variant that carries a defined type or a declared type among them.
trait Writing {
    fn written(&self, name: &Name) -> Name;
}

impl Writing for File {
    fn written(&self, name: &Name) -> Name {
        if self.declaration(name).is_some() {
            return name.clone();
        }
        for implied in self.implied_enums() {
            for variant in implied.variants {
                if variant.title() == name && variant.payload().is_some() {
                    return self.inline_name(&implied.name, &implied.name, name);
                }
            }
        }
        name.clone()
    }
}

/// What a name a root declares is, to a process that is born of it or gives it.
enum Kind<'a> {
    Input(&'a Variant),
    Output(&'a Variant),
    Record(&'a Entry),
    Change(usize, &'a Entry),
    Emission(usize, &'a Variant),
}

/// The trait whose capability says what a name a root declares is.
trait Kinding {
    fn kind(&self, name: &Name) -> Option<Kind<'_>>;
}

impl Kinding for File {
    fn kind(&self, name: &Name) -> Option<Kind<'_>> {
        match self {
            File::Signal(signal) => {
                for input in &signal.inputs {
                    if input.title() == name {
                        return Some(Kind::Input(input));
                    }
                }
                for output in &signal.outputs {
                    if output.title() == name {
                        return Some(Kind::Output(output));
                    }
                }
                None
            }
            File::Memory(memory) => {
                for record in &memory.records {
                    if record.entered() == name {
                        return Some(Kind::Record(record));
                    }
                }
                for (index, change) in memory.changes.iter().enumerate() {
                    if change.entered() == name {
                        return Some(Kind::Change(index, change));
                    }
                }
                None
            }
            File::Emission(emission) => {
                for (index, variant) in emission.emissions.iter().enumerate() {
                    if variant.title() == name {
                        return Some(Kind::Emission(index, variant));
                    }
                }
                None
            }
            File::Library(_) | File::Operation(_) | File::Process(_) => None,
        }
    }
}

// ---------------------------------------------------------------------------
// The namespace and its modules
// ---------------------------------------------------------------------------

/// The trait whose capabilities find the roots of a Nexus by where they are imported from.
trait Finding {
    fn library(&self) -> Option<usize>;
    fn module(&self, source: &Source, namespace: &str) -> Option<usize>;
    fn home(&self, module: usize, namespace: &str) -> Source;
    fn namespace(&self) -> Result<Option<String>, Refusal>;
    fn lands(&self, namespace: &str) -> bool;
    fn landed(&self, process: usize, namespace: Option<&str>, name: &Name)
    -> Option<(usize, Name)>;
    fn resolved(&self, module: usize, namespace: Option<&str>) -> File;
}

impl Finding for Nexus {
    /// The Library, whose names stand at the crate's root.
    fn library(&self) -> Option<usize> {
        for (index, module) in self.modules.iter().enumerate() {
            if matches!(module.file, File::Library(_)) {
                return Some(index);
            }
        }
        None
    }

    /// The module a source of the namespace names: the Library for the
    /// namespace alone, a module for the namespace and its stem.
    fn module(&self, source: &Source, namespace: &str) -> Option<usize> {
        let segments = source.segments();
        if segments.first().map(|segment| segment.0.as_str()) != Some(namespace) {
            return None;
        }
        match segments {
            [_] => self.library(),
            [_, stem] => {
                for (index, module) in self.modules.iter().enumerate() {
                    if module.stem == stem.0 && !matches!(module.file, File::Library(_)) {
                        return Some(index);
                    }
                }
                None
            }
            _ => None,
        }
    }

    /// Where a module's types are written from: the namespace and the stem,
    /// the namespace alone for the Library.
    fn home(&self, module: usize, namespace: &str) -> Source {
        let written = match self.modules[module].file {
            File::Library(_) => namespace.to_owned(),
            _ => format!("{namespace}:{}", self.modules[module].stem),
        };
        Source::try_from(written).expect("a namespace and a stem are segments")
    }

    /// Whether every import that begins at the namespace and names a module of
    /// the set lands on one that declares what it takes, and at least one does.
    fn lands(&self, namespace: &str) -> bool {
        let mut landed = false;
        for module in &self.modules {
            for import in module.file.imports() {
                if import.source().segments()[0].0 != namespace {
                    continue;
                }
                // A module the set lacks is a file not given, not another namespace.
                let Some(target) = self.module(import.source(), namespace) else {
                    continue;
                };
                for imported in import.imported() {
                    if !self.modules[target].file.provides(&imported.emitted) {
                        return false;
                    }
                }
                landed = true;
            }
        }
        landed
    }

    /// The namespace the roots of the set name each other by, if any.
    fn namespace(&self) -> Result<Option<String>, Refusal> {
        let mut candidates: Vec<String> = Vec::new();
        for module in &self.modules {
            for import in module.file.imports() {
                let first = import.source().segments()[0].0.clone();
                if !candidates.contains(&first) {
                    candidates.push(first);
                }
            }
        }
        let mut landing: Vec<String> = Vec::new();
        for candidate in candidates {
            if self.lands(&candidate) {
                landing.push(candidate);
            }
        }
        match landing.len() {
            0 => Ok(None),
            1 => Ok(landing.pop()),
            _ => Err(Refusal {
                module: 0,
                error: Error::conceptual(vec![1, 0], Problem::Namespace(landing[1].clone())),
            }),
        }
    }

    /// The module of the Nexus an imported name of the Process root comes
    /// from, and the name that module gives it.
    fn landed(
        &self,
        process: usize,
        namespace: Option<&str>,
        name: &Name,
    ) -> Option<(usize, Name)> {
        let Resolution::Imported(source, emitted) =
            self.modules[process].file.imports().resolve(name)
        else {
            return None;
        };
        let module = self.module(&source, namespace?)?;
        Some((module, emitted))
    }

    /// The root of a module as it is written: each name it imports from
    /// another root of the Nexus names the type that root resolves it to, so
    /// that every position of the module writes the type the import resolved.
    fn resolved(&self, module: usize, namespace: Option<&str>) -> File {
        let mut file = self.modules[module].file.clone();
        let Some(namespace) = namespace else {
            return file;
        };
        let imports = match &mut file {
            File::Library(library) => &mut library.imports,
            File::Signal(signal) => &mut signal.imports,
            File::Operation(operation) => &mut operation.imports,
            File::Memory(memory) => &mut memory.imports,
            File::Emission(emission) => &mut emission.imports,
            File::Process(process) => &mut process.imports,
        };
        for import in imports.iter_mut() {
            let (source, imported) = match import {
                Import::One(source, imported) => (&*source, std::slice::from_mut(imported)),
                Import::Many(source, imported) => (&*source, imported.as_mut_slice()),
            };
            let Some(target) = self.module(source, namespace) else {
                continue;
            };
            for imported in imported {
                imported.emitted = self.modules[target].file.written(&imported.emitted);
            }
        }
        file
    }
}

// ---------------------------------------------------------------------------
// Types of another root, written from the Process root
// ---------------------------------------------------------------------------

/// The trait whose capability writes a reference of another root as it is
/// written from the Process root: a name the root declares is written from
/// the root's module, an imported one from where it was imported.
trait Relocating {
    fn relocated(&self, file: &File, home: &Source) -> Reference;
}

impl Relocating for Reference {
    fn relocated(&self, file: &File, home: &Source) -> Reference {
        let reference = match self {
            Self::Vector(element) => return Self::Vector(Box::new(element.relocated(file, home))),
            Self::Named(reference) => reference,
        };
        let mut arguments = Vec::with_capacity(reference.arguments.len());
        for argument in &reference.arguments {
            arguments.push(argument.relocated(file, home));
        }
        if reference.source.is_some() {
            return Reference::Named(NamedReference {
                source: reference.source.clone(),
                name: reference.name.clone(),
                arguments,
            });
        }
        match file.resolve(&reference.name) {
            Resolution::Imported(source, emitted) => Reference::Named(NamedReference {
                source: Some(source),
                name: emitted,
                arguments,
            }),
            Resolution::Type(_) => Reference::Named(NamedReference {
                source: Some(home.clone()),
                name: reference.name.clone(),
                arguments,
            }),
            _ => Reference::Named(NamedReference {
                source: None,
                name: reference.name.clone(),
                arguments,
            }),
        }
    }
}

/// The trait whose capability names a type by its name alone.
trait Referring {
    fn referring(&self) -> Reference;
}

impl Referring for Name {
    fn referring(&self) -> Reference {
        Reference::Named(NamedReference {
            source: None,
            name: self.clone(),
            arguments: vec![],
        })
    }
}

/// The trait whose capability yields the type a variant of a root carries,
/// as the Process root writes it: the type it is typed with, the type a bare
/// variant is named for, the payload it declares in place; and for an
/// emission that is a unit of its own, the unit.
trait Carriage {
    fn carried(&self, file: &File, home: &Source, owner: &str, unit: bool) -> Option<Reference>;
}

impl Carriage for Variant {
    fn carried(&self, file: &File, home: &Source, owner: &str, unit: bool) -> Option<Reference> {
        let scope = Scope {
            file,
            identity: None,
            associated: &[],
        };
        let owner = Name::try_from(owner).expect("a root's implied enum is named");
        match self {
            Variant::Bare(name) if scope.carries(name) => {
                Some(name.referring().relocated(file, home))
            }
            Variant::Bare(name) if unit => Some(Reference::Named(NamedReference {
                source: Some(home.clone()),
                name: name.clone(),
                arguments: vec![],
            })),
            Variant::Bare(_) => None,
            Variant::Typed(_, reference) => Some(reference.relocated(file, home)),
            Variant::Struct(name, _) | Variant::Enum(name, _) => {
                Some(Reference::Named(NamedReference {
                    source: Some(home.clone()),
                    name: file.inline_name(&owner, &owner, name),
                    arguments: vec![],
                }))
            }
        }
    }
}

// ---------------------------------------------------------------------------
// What a process is
// ---------------------------------------------------------------------------

/// What a process is born of, and so what it holds.
enum Birth<'a> {
    /// An input of a Signal, in the module at that place.
    Call(usize, &'a Variant),
    /// A record of a Memory, in the module at that place.
    Record(usize, &'a Entry),
    /// A process that gives it: a type the Process root declares.
    Given(&'a TypeDeclaration),
}

/// Who a root process waits on: the signal process, or the memory process.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Waiting {
    Signal,
    Memory,
}

/// What returns to a process for a thing it gave: the name its variant of
/// `Taken` bears, and its type.
#[derive(Clone)]
struct Taking {
    name: Name,
    ty: TokenStream,
}

/// A thing a process gives: its name, the type that is Givable and whether
/// its variant of `Gift` carries it, the type this module generates for it
/// if any, what returns for it, and where the Process root names it.
struct Given {
    name: Name,
    given: TokenStream,
    carries: bool,
    item: Option<TokenStream>,
    taking: Option<Taking>,
    path: Vec<Integer>,
}

/// A process position's name and its resolved type, both viewed from the Process root.
struct HeldPosition {
    named: Reference,
    written: Reference,
}

impl From<Reference> for HeldPosition {
    fn from(reference: Reference) -> Self {
        Self {
            written: reference.clone(),
            named: reference,
        }
    }
}

/// The trait whose capability keeps the authored position while relocating its type.
trait HoldingFrom {
    fn held_from(self, origin: &File, resolved: &File, home: &Source) -> HeldPosition;
}

impl HoldingFrom for Reference {
    fn held_from(self, origin: &File, resolved: &File, home: &Source) -> HeldPosition {
        HeldPosition {
            named: self.relocated(origin, home),
            written: self.relocated(resolved, home),
        }
    }
}

/// A process, read from the Process root and the roots it names.
struct Described {
    name: Name,
    waiting: Waiting,
    positions: Vec<HeldPosition>,
    outcome: Option<TokenStream>,
    gifts: Vec<Given>,
}

/// The trait whose capabilities read the Process root against the other roots.
trait Describing {
    fn birth(
        &self,
        process: usize,
        namespace: Option<&str>,
        name: &Name,
    ) -> Result<Birth<'_>, Problem>;
    fn gift(&self, process: usize, namespace: Option<&str>, name: &Name) -> Result<Given, Problem>;
    fn output(&self, file: &File, home: &Source, variant: &Variant, scope: &Scope) -> Given;
    fn describe(&self, process: usize, namespace: Option<&str>) -> Result<Vec<Described>, Refusal>;
}

impl Describing for Nexus {
    fn birth(
        &self,
        process: usize,
        namespace: Option<&str>,
        name: &Name,
    ) -> Result<Birth<'_>, Problem> {
        if let File::Process(file) = &self.modules[process].file {
            for declaration in &file.types {
                if declaration.identity().name == *name {
                    return Ok(Birth::Given(declaration));
                }
            }
        }
        let Some((module, emitted)) = self.landed(process, namespace, name) else {
            return Err(Problem::Foreign(name.0.clone()));
        };
        match self.modules[module].file.kind(&emitted) {
            Some(Kind::Input(variant)) => Ok(Birth::Call(module, variant)),
            Some(Kind::Record(entry)) => Ok(Birth::Record(module, entry)),
            _ => Err(Problem::Unborn(name.0.clone())),
        }
    }

    fn gift(&self, process: usize, namespace: Option<&str>, name: &Name) -> Result<Given, Problem> {
        let File::Process(file) = &self.modules[process].file else {
            return Err(Problem::Ungivable(name.0.clone()));
        };
        let written = self.resolved(process, namespace);
        let scope = Scope {
            file: &written,
            identity: None,
            associated: &[],
        };
        // A process this root declares, born of the process that gives it.
        if scope.file.declaration(name).is_some() {
            for declared in &file.processes {
                if declared.name == *name {
                    let taking = declared.outcome.as_ref().map(|outcome| Taking {
                        name: Name(outcome.title().to_owned()),
                        ty: outcome.emit(&scope),
                    });
                    return Ok(Given {
                        name: name.clone(),
                        given: name.referring().emit(&scope),
                        carries: true,
                        item: None,
                        taking,
                        path: vec![],
                    });
                }
            }
            return Err(Problem::Ungivable(name.0.clone()));
        }
        let Some((module, emitted)) = self.landed(process, namespace, name) else {
            return Err(Problem::Foreign(name.0.clone()));
        };
        let namespace = namespace.expect("a name landed in a namespace");
        let origin = &self.modules[module].file;
        let spoken = self.resolved(module, Some(namespace));
        let home = self.home(module, namespace);
        match origin.kind(&emitted) {
            Some(Kind::Output(variant)) => Ok(self.output(&spoken, &home, variant, &scope)),
            Some(Kind::Record(entry)) => {
                let given = entry
                    .entered()
                    .referring()
                    .relocated(&spoken, &home)
                    .emit(&scope);
                Ok(Given {
                    name: name.clone(),
                    given,
                    carries: true,
                    item: None,
                    taking: Some(Taking {
                        name: Name::try_from("Answer").expect("a standard name"),
                        ty: quote! { process::Answer },
                    }),
                    path: vec![],
                })
            }
            Some(Kind::Change(index, entry)) => {
                let File::Memory(memory) = origin else {
                    return Err(Problem::Ungivable(name.0.clone()));
                };
                let Some(answer) = memory.answers.get(index) else {
                    return Err(Problem::Unpaired(name.0.clone()));
                };
                let given = entry
                    .entered()
                    .referring()
                    .relocated(&spoken, &home)
                    .emit(&scope);
                let answered = answer
                    .entered()
                    .referring()
                    .relocated(&spoken, &home)
                    .emit(&scope);
                Ok(Given {
                    name: name.clone(),
                    given,
                    carries: true,
                    item: None,
                    taking: Some(Taking {
                        name: answer.entered().clone(),
                        ty: quote! { core::result::Result<#answered, process::Unremembered> },
                    }),
                    path: vec![],
                })
            }
            Some(Kind::Emission(index, variant)) => {
                let File::Emission(emission) = origin else {
                    return Err(Problem::Ungivable(name.0.clone()));
                };
                let Some(outcome) = emission.outcomes.get(index) else {
                    return Err(Problem::Unpaired(name.0.clone()));
                };
                let Some(carried) = variant.carried(&spoken, &home, "Emission", true) else {
                    return Err(Problem::Ungivable(name.0.clone()));
                };
                let observed = match outcome.carried(&spoken, &home, "Outcome", false) {
                    Some(observed) => observed.emit(&scope),
                    None => quote! { () },
                };
                Ok(Given {
                    name: name.clone(),
                    given: carried.emit(&scope),
                    carries: true,
                    item: None,
                    taking: Some(Taking {
                        name: outcome.title().clone(),
                        ty: quote! { core::result::Result<#observed, process::Unobserved> },
                    }),
                    path: vec![],
                })
            }
            Some(Kind::Input(_)) | None => Err(Problem::Ungivable(name.0.clone())),
        }
    }

    /// An output given: a type of this module's own, a newtype over what the
    /// output carries, or a unit where it carries nothing; the payload an
    /// output declares in place is its own type already, and is given as it is.
    fn output(&self, file: &File, home: &Source, variant: &Variant, scope: &Scope) -> Given {
        let name = variant.title().clone();
        let own = name.tokens();
        let identity = Identity {
            name: name.clone(),
            constraints: vec![],
        };
        match variant {
            Variant::Struct(_, _) | Variant::Enum(_, _) => {
                let carried = variant
                    .carried(file, home, "Output", false)
                    .expect("a payload declared in place is a type");
                Given {
                    name,
                    given: carried.emit(scope),
                    carries: true,
                    item: None,
                    taking: None,
                    path: vec![],
                }
            }
            Variant::Bare(_) | Variant::Typed(_, _) => {
                match variant.carried(file, home, "Output", false) {
                    Some(carried) => Given {
                        name,
                        given: own,
                        carries: true,
                        item: Some(TypeDeclaration::Alias(identity, carried).emit(scope)),
                        taking: None,
                        path: vec![],
                    },
                    None => Given {
                        name,
                        given: own,
                        carries: false,
                        item: Some(TypeDeclaration::Struct(identity, vec![]).emit(scope)),
                        taking: None,
                        path: vec![],
                    },
                }
            }
        }
    }

    fn describe(&self, process: usize, namespace: Option<&str>) -> Result<Vec<Described>, Refusal> {
        let File::Process(file) = &self.modules[process].file else {
            return Ok(vec![]);
        };
        let written = self.resolved(process, namespace);
        let scope = Scope {
            file: &written,
            identity: None,
            associated: &[],
        };
        let mut described: Vec<Described> = Vec::new();
        for (at, declaration) in file.processes.iter().enumerate() {
            let place: Vec<Integer> = vec![1, 1, at as Integer];
            let birth = match self.birth(process, namespace, &declaration.name) {
                Ok(birth) => birth,
                Err(problem) => {
                    return Err(Refusal {
                        module: process,
                        error: Error::conceptual(place.joined(&[0]), problem),
                    });
                }
            };
            let mut positions = birth.holds(self, namespace);
            for configured in &declaration.configured {
                positions.push(configured.clone().into());
            }
            let waiting = match birth {
                Birth::Record(_, _) => Waiting::Memory,
                Birth::Call(_, _) | Birth::Given(_) => Waiting::Signal,
            };
            let mut gifts: Vec<Given> = Vec::new();
            let mut index: Integer = 0;
            for gift in &declaration.gifts {
                let path = place.joined(&[1, 1, index]);
                index += gift.span();
                let Some(gift) = gift.named() else {
                    return Err(Refusal {
                        module: process,
                        error: Error::conceptual(path, Problem::Expected(crate::Form::Name)),
                    });
                };
                match self.gift(process, namespace, &gift.name) {
                    Ok(mut given) => {
                        given.path = path;
                        gifts.push(given);
                    }
                    Err(problem) => {
                        return Err(Refusal {
                            module: process,
                            error: Error::conceptual(path, problem),
                        });
                    }
                }
            }
            described.push(Described {
                name: declaration.name.clone(),
                waiting,
                positions,
                outcome: declaration
                    .outcome
                    .as_ref()
                    .map(|outcome| outcome.emit(&scope)),
                gifts,
            });
        }
        // Two kinds of thing that return under one name cannot be told apart.
        let mut named: Vec<(Name, String)> = Vec::new();
        for process_described in &described {
            for gift in &process_described.gifts {
                let Some(taking) = &gift.taking else {
                    continue;
                };
                let ty = taking.ty.to_string();
                for (seen, seen_ty) in &named {
                    if *seen == taking.name && *seen_ty != ty {
                        return Err(Refusal {
                            module: process,
                            error: Error::conceptual(
                                gift.path.clone(),
                                Problem::Duplicate(taking.name.0.clone()),
                            ),
                        });
                    }
                }
                named.push((taking.name.clone(), ty));
            }
        }
        Ok(described)
    }
}

/// The trait whose capability joins a path with the steps below it.
trait Joining {
    fn joined(&self, tail: &[Integer]) -> Vec<Integer>;
}

impl Joining for Vec<Integer> {
    fn joined(&self, tail: &[Integer]) -> Vec<Integer> {
        let mut path = self.clone();
        path.extend_from_slice(tail);
        path
    }
}

/// The trait whose capability lists the positions a process born of this holds.
trait Holding {
    fn holds(&self, nexus: &Nexus, namespace: Option<&str>) -> Vec<HeldPosition>;
}

impl Holding for Birth<'_> {
    fn holds(&self, nexus: &Nexus, namespace: Option<&str>) -> Vec<HeldPosition> {
        match self {
            Birth::Call(module, variant) => {
                let origin = &nexus.modules[*module].file;
                let spoken = nexus.resolved(*module, namespace);
                let home = nexus.home(*module, namespace.expect("a call landed in a namespace"));
                let mut positions = vec![
                    Reference::Named(NamedReference {
                        source: Some(Source::try_from("process").expect("a source")),
                        name: Name::try_from("Caller").expect("a standard name"),
                        arguments: vec![],
                    })
                    .into(),
                ];
                match variant {
                    Variant::Struct(_, held) => {
                        for position in held.references() {
                            positions.push(position.held_from(origin, &spoken, &home));
                        }
                    }
                    other => {
                        let named = other.carried(origin, &home, "Input", false);
                        let written = other.carried(&spoken, &home, "Input", false);
                        if let (Some(named), Some(written)) = (named, written) {
                            positions.push(HeldPosition { named, written });
                        }
                    }
                }
                positions
            }
            Birth::Record(module, entry) => {
                let origin = &nexus.modules[*module].file;
                let spoken = nexus.resolved(*module, namespace);
                let home = nexus.home(*module, namespace.expect("a record landed in a namespace"));
                match entry {
                    Entry::Declared(TypeDeclaration::Struct(_, held)) => {
                        let mut positions = Vec::new();
                        for position in held.references() {
                            positions.push(position.held_from(origin, &spoken, &home));
                        }
                        positions
                    }
                    other => vec![
                        other
                            .entered()
                            .referring()
                            .held_from(origin, &spoken, &home),
                    ],
                }
            }
            Birth::Given(declaration) => match declaration {
                TypeDeclaration::Struct(_, held) => {
                    held.references().into_iter().map(Into::into).collect()
                }
                other => vec![other.identity().name.referring().into()],
            },
        }
    }
}

// ---------------------------------------------------------------------------
// What the Process root's module is generated with
// ---------------------------------------------------------------------------

/// The trait whose capabilities collect what returns to a process, and what a Nexus's processes give.
trait Collecting {
    fn takings(&self) -> Vec<Taking>;
}

impl Collecting for Described {
    /// Each kind of thing that returns to the process, once, in the order of its gifts.
    fn takings(&self) -> Vec<Taking> {
        let mut takings: Vec<Taking> = Vec::new();
        for gift in &self.gifts {
            if let Some(taking) = &gift.taking {
                let mut seen = false;
                for other in &takings {
                    if other.ty.to_string() == taking.ty.to_string() {
                        seen = true;
                    }
                }
                if !seen {
                    takings.push(taking.clone());
                }
            }
        }
        takings
    }
}

/// The trait whose capabilities collect what a Nexus's processes share.
trait Gathering {
    fn gifts(&self) -> Vec<&Given>;
    fn kinds(&self) -> Vec<Taking>;
}

impl Gathering for [Described] {
    /// Every thing given by any process, once, in the order it first is.
    fn gifts(&self) -> Vec<&Given> {
        let mut gifts: Vec<&Given> = Vec::new();
        for process in self {
            for gift in &process.gifts {
                let mut seen = false;
                for other in &gifts {
                    if other.name == gift.name {
                        seen = true;
                    }
                }
                if !seen {
                    gifts.push(gift);
                }
            }
        }
        gifts
    }

    /// Every kind of thing that returns to any process, once.
    fn kinds(&self) -> Vec<Taking> {
        let mut kinds: Vec<Taking> = Vec::new();
        for process in self {
            for taking in process.takings() {
                let mut seen = false;
                for other in &kinds {
                    if other.ty.to_string() == taking.ty.to_string() {
                        seen = true;
                    }
                }
                if !seen {
                    kinds.push(taking);
                }
            }
        }
        kinds
    }
}

/// The trait whose capability yields a name as an identifier.
trait Identing {
    fn ident(&self) -> Ident;
}

impl Identing for Name {
    fn ident(&self) -> Ident {
        Ident::new(&self.0, Span::call_site())
    }
}

/// The trait whose capabilities name what a process generates: its struct of
/// positions, what it takes, and the binding its body is called by.
trait Calling {
    fn structure(&self) -> Name;
    fn taken(&self) -> Name;
    fn bound(&self) -> Ident;
}

impl Calling for Described {
    fn structure(&self) -> Name {
        Name::try_from(format!("{}Process", self.name.0)).expect("a process name is an identifier")
    }

    fn taken(&self) -> Name {
        Name::try_from(format!("{}Taken", self.name.0)).expect("a process name is an identifier")
    }

    fn bound(&self) -> Ident {
        self.name.0.snake_case().identifier()
    }
}

/// The trait whose capability yields an enum of the variants given, with the
/// derives every generated type bears: each variant by its name and the type
/// it carries, or none.
trait Enumerated {
    fn enumeration(&self, name: &Name) -> TokenStream;
}

impl Enumerated for [(Ident, Option<TokenStream>)] {
    fn enumeration(&self, name: &Name) -> TokenStream {
        let identity = Identity {
            name: name.clone(),
            constraints: vec![],
        };
        let derive = identity.datom_derives();
        let name = name.ident();
        let mut variants = Vec::with_capacity(self.len());
        for (variant, carried) in self {
            variants.push(match carried {
                Some(carried) => quote! { #variant(#carried) },
                None => quote! { #variant },
            });
        }
        quote! { #derive pub enum #name { #( #variants ),* } }
    }
}

/// The trait whose capability writes what a Nexus's processes take: the type
/// that stands for it, which is the one kind that returns, bare, or an enum
/// of the kinds that do.
trait Returning {
    fn taking(&self, name: &Name) -> TokenStream;
}

impl Returning for [Taking] {
    fn taking(&self, name: &Name) -> TokenStream {
        let alias = name.ident();
        match self {
            [] => quote! { #[rustfmt::skip] pub type #alias = core::convert::Infallible; },
            [only] => {
                let ty = &only.ty;
                quote! { #[rustfmt::skip] pub type #alias = #ty; }
            }
            many => {
                let mut variants = Vec::with_capacity(many.len());
                for kind in many {
                    variants.push((kind.name.ident(), Some(kind.ty.clone())));
                }
                variants.enumeration(name)
            }
        }
    }
}

/// The trait whose capability writes the `take` of the Process enum: a thing
/// taken goes to the process it is for, as that process takes it.
trait Handing {
    fn takes(&self, kinds: &[Taking]) -> TokenStream;
}

impl Handing for [Described] {
    fn takes(&self, kinds: &[Taking]) -> TokenStream {
        let head = quote! {
            fn take<N: process::Giving<Gift>>(&mut self, taken: Taken, giving: N) -> core::option::Option<Outcome>
        };
        if kinds.is_empty() {
            return quote! { #head { let _ = (self, giving); match taken {} } };
        }
        let mut arms = Vec::new();
        let mut covered = true;
        for process in self {
            let variant = process.name.ident();
            let bound = process.bound();
            let own = process.takings();
            let taken = process.taken().ident();
            let mapping = match &process.outcome {
                Some(_) => quote! { Outcome::#variant },
                None => quote! { |()| Outcome::#variant },
            };
            if own.len() < kinds.len() || own.is_empty() {
                covered = false;
            }
            for kind in &own {
                let name = kind.name.ident();
                let (pattern, handed) = if kinds.len() == 1 {
                    (quote! { carried }, quote! { carried })
                } else if own.len() == 1 {
                    (quote! { Taken::#name(carried) }, quote! { carried })
                } else {
                    (
                        quote! { Taken::#name(carried) },
                        quote! { #taken::#name(carried) },
                    )
                };
                arms.push(quote! {
                    (Process::#variant(#bound), #pattern) => process::Processable::take(#bound, #handed, giving).map(#mapping),
                });
            }
        }
        if !covered {
            arms.push(quote! { _ => core::option::Option::None, });
        }
        quote! { #head { match (self, taken) { #( #arms )* } } }
    }
}

/// The trait whose capability generates everything the Process root's module
/// holds beyond the root's own types: the things given that it generates,
/// each process's positions and what it takes, the enums over them and
/// their traits.
trait Producing {
    fn produce(&self, scope: &Scope) -> Vec<TokenStream>;
}

impl Producing for [Described] {
    fn produce(&self, scope: &Scope) -> Vec<TokenStream> {
        let mut items: Vec<TokenStream> = Vec::new();
        let gifts = self.gifts();
        for gift in &gifts {
            if let Some(item) = &gift.item {
                items.push(item.clone());
            }
        }
        // Each process: its positions, and what it takes.
        for process in self {
            let mut named = Vec::with_capacity(process.positions.len());
            let mut written = Vec::with_capacity(process.positions.len());
            for position in &process.positions {
                named.push(position.named.clone());
                written.push(position.written.clone());
            }
            let structure = Identity {
                name: process.structure(),
                constraints: vec![],
            };
            let fields = named.field_names(&structure.name);
            items.push(written.structure_with_fields(scope, &structure.name, &structure, &fields));
            items.push(process.takings().taking(&process.taken()));
        }
        // The enums over the Nexus's processes.
        let name = |text: &str| Name::try_from(text).expect("a static identifier");
        let mut processes = Vec::with_capacity(self.len());
        let mut outcomes = Vec::with_capacity(self.len() + 1);
        for process in self {
            let structure = process.structure().ident();
            processes.push((process.name.ident(), Some(quote! { #structure })));
            outcomes.push((process.name.ident(), process.outcome.clone()));
        }
        outcomes.push((name("Unwound").ident(), None));
        items.push(processes.enumeration(&name("Process")));
        items.push(outcomes.enumeration(&name("Outcome")));
        let mut given = Vec::with_capacity(gifts.len());
        for gift in &gifts {
            let carried = if gift.carries {
                Some(gift.given.clone())
            } else {
                None
            };
            given.push((gift.name.ident(), carried));
        }
        items.push(given.enumeration(&name("Gift")));
        let kinds = self.kinds();
        items.push(kinds.taking(&name("Taken")));
        // What each thing given is, and what comes back for it.
        for gift in &gifts {
            let type_given = &gift.given;
            let variant = gift.name.ident();
            let returned = match &gift.taking {
                Some(taking) => taking.ty.clone(),
                None => quote! { () },
            };
            let from = if gift.carries {
                quote! { fn from(gift: #type_given) -> Gift { Gift::#variant(gift) } }
            } else {
                quote! { fn from(_: #type_given) -> Gift { Gift::#variant } }
            };
            items.push(quote! {
                #[rustfmt::skip]
                impl process::Givable for #type_given { type Returned = #returned; }
            });
            items.push(quote! {
                #[rustfmt::skip]
                impl core::convert::From<#type_given> for Gift { #from }
            });
        }
        items.push(quote! {
            #[rustfmt::skip]
            impl process::Unwindable for Outcome {
                fn unwound() -> Self { Outcome::Unwound }
            }
        });
        // Who owns a process that is born of no process.
        let mut signal = Vec::new();
        let mut memory = Vec::new();
        for process in self {
            let variant = process.name.ident();
            match process.waiting {
                Waiting::Signal => signal.push(quote! { Process::#variant(_) }),
                Waiting::Memory => memory.push(quote! { Process::#variant(_) }),
            }
        }
        let mut owners = Vec::new();
        if !memory.is_empty() {
            owners.push(quote! { #( #memory )|* => process::Owner::Memory, });
        }
        if !signal.is_empty() {
            owners.push(quote! { #( #signal )|* => process::Owner::Signal, });
        }
        items.push(quote! {
            #[rustfmt::skip]
            impl process::Ownable for Process {
                fn owner(&self) -> process::Owner { match self { #( #owners )* } }
            }
        });
        // The Process the manager holds: each begin and take goes to the
        // process's own body, written by hand.
        let mut begins = Vec::with_capacity(self.len());
        for process in self {
            let variant = process.name.ident();
            let bound = process.bound();
            let mapping = match &process.outcome {
                Some(_) => quote! { Outcome::#variant },
                None => quote! { |()| Outcome::#variant },
            };
            begins.push(quote! {
                Process::#variant(#bound) => process::Processable::begin(#bound, giving).map(#mapping),
            });
        }
        let take = self.takes(&kinds);
        items.push(quote! {
            #[rustfmt::skip]
            impl process::Processable for Process {
                type Taken = Taken;
                type Gift = Gift;
                type Outcome = Outcome;
                fn begin<N: process::Giving<Gift>>(&mut self, giving: N) -> core::option::Option<Outcome> {
                    match self { #( #begins )* }
                }
                #take
            }
        });
        items
    }
}

/// The trait whose capability writes the Process root as its own types are
/// generated from it: without the imports that name an input or an output.
/// An outcome's bare variant carries the type its name is imported for, and an
/// input or an output is no type, so `Configuring.[ Configured Unremembered ]`
/// beside the output `Configured` a process gives declares two variants that
/// carry nothing.
trait Stripping {
    fn stripped(&self, process: usize, namespace: Option<&str>) -> File;
}

impl Stripping for Nexus {
    fn stripped(&self, process: usize, namespace: Option<&str>) -> File {
        let File::Process(file) = self.resolved(process, namespace) else {
            return self.modules[process].file.clone();
        };
        let mut imports = Vec::with_capacity(file.imports.len());
        for import in &file.imports {
            let mut kept = Vec::new();
            for imported in import.imported() {
                let names = match self.landed(process, namespace, &imported.name) {
                    Some((module, emitted)) => matches!(
                        self.modules[module].file.kind(&emitted),
                        Some(Kind::Input(_) | Kind::Output(_))
                    ),
                    None => false,
                };
                if !names {
                    kept.push(imported.clone());
                }
            }
            match (import, kept.len()) {
                (_, 0) => {}
                (Import::One(source, _), _) => {
                    imports.push(Import::One(source.clone(), kept.remove(0)));
                }
                (Import::Many(source, _), _) => imports.push(Import::Many(source.clone(), kept)),
            }
        }
        File::Process(crate::Process {
            imports,
            processes: file.processes,
            types: file.types,
        })
    }
}

// ---------------------------------------------------------------------------
// Verifying and assembling
// ---------------------------------------------------------------------------

impl Verifying for Nexus {
    fn verify(&self) -> Result<(), Refusal> {
        let mut processes: Option<usize> = None;
        let mut libraries: Option<usize> = None;
        for (index, module) in self.modules.iter().enumerate() {
            for earlier in &self.modules[..index] {
                if earlier.stem == module.stem {
                    return Err(Refusal {
                        module: index,
                        error: Error::conceptual(vec![0], Problem::Duplicate(module.stem.clone())),
                    });
                }
            }
            if let Err(error) = module.file.validate() {
                return Err(Refusal {
                    module: index,
                    error,
                });
            }
            let slot = match &module.file {
                File::Process(_) => &mut processes,
                File::Library(_) => &mut libraries,
                File::Signal(_) | File::Operation(_) | File::Memory(_) | File::Emission(_) => {
                    continue;
                }
            };
            if slot.is_some() {
                return Err(Refusal {
                    module: index,
                    error: Error::conceptual(vec![0], Problem::Duplicate(module.stem.clone())),
                });
            }
            *slot = Some(index);
        }
        let namespace = self.namespace()?;
        if let Some(process) = processes {
            self.describe(process, namespace.as_deref())?;
        }
        Ok(())
    }
}

impl Assembling for Nexus {
    fn assemble(&self) -> Result<Vec<Assembled>, Refusal> {
        self.verify()?;
        let namespace = self.namespace()?;
        let mut process_modules = Vec::with_capacity(self.modules.len());
        for module in &self.modules {
            process_modules.push(Module {
                stem: module.stem.clone(),
                file: module.file.normalized(),
            });
        }
        let processes = Nexus {
            modules: process_modules,
        };
        let mut assembled = Vec::with_capacity(self.modules.len());
        for (index, module) in self.modules.iter().enumerate() {
            let rust = match &module.file {
                File::Process(_) => {
                    let described = processes.describe(index, namespace.as_deref())?;
                    let stripped = processes.stripped(index, namespace.as_deref());
                    let scope = Scope {
                        file: &stripped,
                        identity: None,
                        associated: &[],
                    };
                    let mut items = stripped.items(&scope);
                    if !described.is_empty() {
                        let written = processes.resolved(index, namespace.as_deref());
                        items.extend(described.produce(&Scope {
                            file: &written,
                            ..scope
                        }));
                    }
                    items.render()
                }
                File::Library(_)
                | File::Signal(_)
                | File::Operation(_)
                | File::Memory(_)
                | File::Emission(_) => {
                    match self.resolved(index, namespace.as_deref()).generate() {
                        Ok(rust) => rust,
                        Err(error) => {
                            return Err(Refusal {
                                module: index,
                                error,
                            });
                        }
                    }
                }
            };
            assembled.push(Assembled {
                stem: module.stem.clone(),
                rust,
            });
        }
        Ok(assembled)
    }
}
