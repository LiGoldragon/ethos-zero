//! Ethos-zero: the ethos schema language, version zero.
//!
//! Ethos specifies the types, datom fills them with data, and ethos
//! generates the Rust. This crate reads an ethos file and generates
//! its Rust module. The layers, top to bottom, and the kind that
//! carries a value from one to the next:
//!
//! | layer | type | kind borne | yields |
//! |---|---|---|---|
//! | Text, as written (the sweet form) | `protos::Text` | [`Canonicalizable`] | [`Canonical`] |
//! | Text, canonical (the braced form) | [`Canonical`] | `protos::Protosizable` | `protos::Delineation` |
//! | Protoform | `protos::Delineation`, `protos::Protoform` | `Conceiving<File>` | [`File`], checked whole |
//! | Concept | [`File`] | [`Generating`] | Rust text, or a whole-file fault |
//!
//! `protos::Potential<File>` bears `protos::Actualizable<File>`: the
//! whole descent in one call, its fault situated by path and extent in
//! the source text. The concept goes back up too: [`File`] bears
//! `protos::Protosizable` and `protos::Textualizable`, which cannot
//! fault.
//!
//! Every fault the reader raises is a [`Error`] carrying the path of
//! the structure at fault, in Protos's path convention: a headed structure's
//! head is child 0 and its body is child 1; a qualified head's arguments are
//! children of that head; an enclosure's children are
//! numbered from 0, and each container prepends its child's index on
//! the way up (`protos::Pathed::within`).
//!
//! Declared structs and enums bear datom-codec's structural kinds through its
//! derives. Signal declarations gate those derives behind their `datom`
//! feature, so a Nexus can use its contract without a text codec.

// A walk over the variants of an enum is written as the loop it is, not
// as an iterator adaptor with an inlined closure: no closure beyond what
// std forces, and no free function, is the crate's own rule.
#![allow(clippy::manual_find, clippy::manual_map)]

use datom_codec::Integer;
use protos::Extent;
use std::marker::PhantomData;

// ---------------------------------------------------------------------------
// The reader errors: declared in error.ethos, generated into error.rs
// ---------------------------------------------------------------------------

#[rustfmt::skip]
mod error;

pub use error::{Arity_Data, Conceptual_Data, Error, Form, Problem, Structural_Error};

// ---------------------------------------------------------------------------
// The concept: the File and its declarations
// ---------------------------------------------------------------------------

/// A validated identifier: the name of a type, kind, variant, capability or constant.
///
/// Construct it with [`TryFrom<&str>`]; `AsRef<str>` reads its validated text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Name(String);

/// The source of an import: a Rust path prefix such as `protos`, `crate` or `std::clone`.
///
/// Construct it with [`TryFrom<&str>`]; `AsRef<str>` reads its validated text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Source {
    text: String,
    segments: Vec<protos::Symbol>,
}

impl TryFrom<&str> for Name {
    type Error = String;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        if !text.starts_with("r#")
            && (text == "Self" || syn::parse_str::<syn::Ident>(text).is_ok())
            && !text.is_empty()
        {
            Ok(Self(text.to_owned()))
        } else {
            Err(text.to_owned())
        }
    }
}

impl TryFrom<String> for Name {
    type Error = String;

    fn try_from(text: String) -> Result<Self, Self::Error> {
        Self::try_from(text.as_str()).map(|_| Self(text))
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl TryFrom<&str> for Source {
    type Error = String;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let Ok(path) = syn::parse_str::<syn::Path>(text) else {
            return Err(text.to_owned());
        };
        if path.leading_colon.is_some() {
            return Err(text.to_owned());
        }
        for segment in &path.segments {
            if segment.ident == "Self" || !segment.arguments.is_none() {
                return Err(text.to_owned());
            }
        }
        let mut segments = Vec::with_capacity(path.segments.len());
        for segment in text.split("::") {
            if segment.is_empty() {
                return Err(text.to_owned());
            }
            segments.push(protos::Symbol(segment.to_owned()));
        }
        Ok(Self {
            text: text.to_owned(),
            segments,
        })
    }
}

impl TryFrom<String> for Source {
    type Error = String;

    fn try_from(text: String) -> Result<Self, Self::Error> {
        Self::try_from(text.as_str())
    }
}

impl AsRef<str> for Source {
    fn as_ref(&self) -> &str {
        &self.text
    }
}

/// The unit of declaration: one file, one Rust module; an enum of its four variants.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum File {
    /// Library: imports, types, kinds and associations in that order.
    Library(Library),
    /// Signal: imports, the query variants, the response variants, the types carried.
    Signal(Signal),
    /// Sema: imports, the record's positions, the types stored.
    Sema(Sema),
}

/// The head of a file: which variant of [`File`] it is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Root {
    /// The library root.
    Library,
    /// The types variant.
    /// The signal variant.
    Signal,
    /// The sema variant.
    Sema,
}

/// A library's complete declaration surface.  Types and kinds share one
/// namespace and associations bind those declared types to declared kinds.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Library {
    /// Where imported names come from.
    pub imports: Vec<Import>,
    /// The declared data types.
    pub types: Vec<TypeDeclaration>,
    /// The declared capability kinds.
    pub kinds: Vec<KindDeclaration>,
    /// The type-to-kind assertions.
    pub associations: Vec<Association>,
}

/// The signal variant of a file: a wire contract whose query type is `Query` and whose response type is `Response`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Signal {
    /// Where imported names come from.
    pub imports: Vec<Import>,
    /// The variants of the query type `Query`.
    pub requests: Vec<Variant>,
    /// The variants of the response type `Response`.
    pub responses: Vec<Variant>,
    /// The types the requests and responses carry.
    pub types: Vec<TypeDeclaration>,
}

/// The sema variant of a file: imports and record-type declarations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sema {
    /// Where imported names come from.
    pub imports: Vec<Import>,
    /// The record type declarations.
    pub types: Vec<TypeDeclaration>,
}

/// An import: a source and the names taken from it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Import {
    /// One name from a source: `protos:Text`.
    One(Source, Imported),
    /// Several names from a source: `protos:[ Text Integer ]`.
    Many(Source, Vec<Imported>),
}

/// An imported name: the ethos name and the source's own name for it, the same unless written `Ethos.Source`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Imported {
    /// The name as ethos writes it.
    pub name: Name,
    /// The name the source gives it, which the generated Rust writes.
    pub emitted: Name,
}

/// A reference to a type or a kind by name: an optional inline source, the name, and its arguments.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Reference {
    /// An inline source qualifying the name: `protos:Error`.
    pub source: Option<Source>,
    /// The name referred to.
    pub name: Name,
    /// The arguments in angle brackets: `Vector<Text>`, `Result<Integer SinkError>`.
    pub arguments: Vec<Reference>,
}

/// The identity of a type or a kind: its name and its constraints, written as one head.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Identity {
    /// The name.
    pub name: Name,
    /// The constraints, one per parameter the Rust needs.
    pub constraints: Vec<Constraint>,
}

/// A constraint: a kind, or a bracket of kinds, bounding one parameter.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Constraint {
    /// One kind: `Serializable`.
    One(Reference),
    /// A bracket of kinds: `[Clonable Sendable]`.
    Many(Vec<Reference>),
}

/// A type declaration: a struct of positions, an enum of variants, or an alias.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypeDeclaration {
    /// A headed brace: the positions in order.
    Struct(Identity, Vec<Reference>),
    /// A headed bracket: the variants.
    Enum(Identity, Vec<Variant>),
    /// A headed bare: the aliased type.
    Alias(Identity, Reference),
}

/// A variant of an enum.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Variant {
    /// Carrying nothing: `Closed`.
    Bare(Name),
    /// Carrying one type: `Lock.LockRequest`.
    Typed(Name, Reference),
    /// Carrying an inline struct, a tuple variant: `Node.{ Tree Tree }`.
    Struct(Name, Vec<Reference>),
    /// Carrying an inline enum, a nested enum type: `Kind.[ A B ]`.
    Enum(Name, Vec<Variant>),
}

/// A kind declaration: the bearer of capabilities, a trait in the Rust.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KindDeclaration {
    /// Its identity: the name and the constraints.
    pub identity: Identity,
    /// Its definition.
    pub body: KindBody,
}

/// The definition of a kind: simple, a bracket of capabilities; or complex, a brace of four brackets.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum KindBody {
    /// `Name.[ capabilities ]`.
    Simple(Vec<Capability>),
    /// `Name.{ [ superkinds ] [ associated types ] [ associated constants ] [ capabilities ] }`.
    Complex {
        /// The kinds it extends.
        superkinds: Vec<Reference>,
        /// Its associated types.
        types: Vec<AssociatedType>,
        /// Its associated constants.
        constants: Vec<AssociatedConstant>,
        /// Its capabilities.
        capabilities: Vec<Capability>,
    },
}

/// An associated type of a kind, with the kinds bounding it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssociatedType {
    /// The name.
    pub name: Name,
    /// The bounds: `Item<Serializable>`.
    pub bounds: Vec<Reference>,
}

/// An associated constant of a kind: its name and its type.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssociatedConstant {
    /// The upper-case name.
    pub name: Name,
    /// The type.
    pub ty: Reference,
}

/// A capability: a function a kind has.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Capability {
    /// The name.
    pub name: Name,
    /// Who is called.
    pub receiver: Receiver,
    /// What it takes and what it yields.
    pub signature: Signature,
}

/// A capability's signature: a yield bracket alone, or a brace of inputs and yield.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Signature {
    /// `name.[ Yield ]`.
    Yielding(Reference),
    /// `name.{ [ inputs ] [ Yield ] }`.
    Taking(Vec<Reference>, Reference),
}

/// Who a capability is called on, said by its separator.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receiver {
    /// `.` takes self.
    Shared,
    /// `!` takes mutable self.
    Mutable,
    /// `:` takes no self.
    Static,
}

/// An association: a type, by its identity, bears kinds.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Association {
    /// The type's identity.
    pub identity: Identity,
    /// The kinds it bears.
    pub kinds: Vec<Reference>,
}

// ---------------------------------------------------------------------------
// The text layer: the canonical form
// ---------------------------------------------------------------------------

/// The canonical text of a file, and the seam where the sweet form was opened into braces.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Canonical {
    /// The braced form the reader sees.
    pub text: String,
    /// The bytes inserted after the head, empty when the text was already canonical.
    pub seam: Extent,
}

/// Text awaiting the one conversion to an Ethos value.
pub struct Potential<T>(pub String, PhantomData<fn() -> T>);

impl<T> From<&str> for Potential<T> {
    fn from(text: &str) -> Self {
        Self(text.to_owned(), PhantomData)
    }
}

impl<T> From<String> for Potential<T> {
    fn from(text: String) -> Self {
        Self(text, PhantomData)
    }
}

// ---------------------------------------------------------------------------
// Resolution: what a name names
// ---------------------------------------------------------------------------

/// The names known without import.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Intrinsic {
    /// Rust's owned string.
    String,
    /// `protos::Integer`.
    Integer,
    /// `protos::Decimal`.
    Decimal,
    /// `protos::Boolean`.
    Boolean,
    /// `datom_codec::Meaning`.
    Meaning,
    /// `Vec`.
    Vector,
    /// `Option`.
    Option,
    /// `Result`.
    Result,
    /// `Self`.
    Itself,
    /// `Sized`, the bound every corporate type bears.
    Sized,
}

/// What a name resolves to in a scope.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Resolution {
    /// An intrinsic, written fully qualified.
    Intrinsic(Intrinsic),
    /// An imported name, written as the source's path and the source's name.
    Imported(Source, Name),
    /// A type declared in this file, written bare.
    Type(Name),
    /// A kind declared in this file, written bare.
    Kind(Name),
    /// The parameter bounded by the enclosing identity's constraint at this index.
    Parameter(Integer),
    /// More than one enclosing parameter has this name among its bounds.
    ///
    /// A body reference cannot say which parameter it means, even when the
    /// constraints differ as whole groups.
    Ambiguous(Name),
    /// An associated type of the enclosing kind, written `Self::Name`.
    Associated(Name),
    /// A name nothing declares.
    Undeclared,
}

/// What a reference is asked to be: a type in a type position, a kind in a bound.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    /// A type: a position, an alias, an input, a yield, an argument.
    Type,
    /// A kind: a constraint, a superkind, a bound, an association.
    Kind,
}

/// The scope a reference resolves in: the file, and the identity and associated types of the enclosing declaration.
#[derive(Clone, Copy, Debug)]
pub struct Scope<'a> {
    /// The file whose imports and declarations are in scope.
    pub file: &'a File,
    /// The enclosing identity, whose single-kind constraints name parameters.
    pub identity: Option<&'a Identity>,
    /// The enclosing kind's associated types.
    pub associated: &'a [AssociatedType],
}

// ---------------------------------------------------------------------------
// Kinds
// ---------------------------------------------------------------------------

/// The kind whose capability yields the canonical form of an ethos text.
pub trait Canonicalizable {
    /// Open the sweet form into the braced form; the text is delineated to find its head.
    fn canonicalize(&self) -> Result<Canonical, protos::Error>;
}

/// The kind whose capability maps an extent of the canonical text back onto the source text.
pub trait Resituating {
    /// Map an extent across the seam.
    fn resituate(&self, extent: Extent) -> Extent;
}

/// The kind whose capability yields the ethos name of a value.
pub trait Named {
    /// The name as ethos writes it.
    fn name(&self) -> &'static str;
}

/// The kind whose static capability identifies a variant from its ethos name, walking the variants.
pub trait Identifiable: Sized {
    /// Identify the variant named.
    fn identify(name: &str) -> Option<Self>;
}

/// The kind whose capability yields which variant of [`File`] a value is.
pub trait Rooted {
    /// The head the file is written under.
    fn root(&self) -> Root;
}

/// The kind whose capability resolves a name to what it names.
pub trait Resolving {
    /// Resolve a name.
    fn resolve(&self, name: &Name) -> Resolution;
}

/// The kind whose capability checks a whole file and generates its Rust module.
pub trait Generating {
    /// The formatted Rust text, or the whole-file fault that prevents generation.
    fn generate(&self) -> Result<String, Error>;
}

/// The kind whose capability actualizes Ethos text into its conceptual value.
pub trait Actualizing<T> {
    type Error;
    fn actualize(&self) -> Result<T, Self::Error>;
}

/// The conversion from Protos structure to an Ethos concept.
pub trait Ethosizable<T> {
    type Error;
    fn ethosize(&self) -> Result<T, Self::Error>;
}

/// The kind whose capability yields a conceptual fault's path and places it below a child.
pub trait Pathed {
    /// The path from the root form to this fault.
    fn path(&self) -> &[Integer];
    /// Prepend a child position to the path.
    fn within(self, index: Integer) -> Self;
}

/// The kind whose capability places a result's fault under a child index.
pub trait Placing {
    /// Prepend the index to the fault's path.
    fn place(self, index: Integer) -> Self;
}

/// The kind whose capability constructs a situated conceptual fault.
pub trait ConceptualFaulting {
    /// Construct the fault from its path and problem.
    fn conceptual(integer_vector: Vec<Integer>, problem: Problem) -> Self;
}

/// The kind whose capability constructs an arity problem.
pub trait ArityProblem {
    /// Construct the problem from expected and actual arity.
    fn arity(first_integer: Integer, second_integer: Integer) -> Self;
}

// ---------------------------------------------------------------------------
// Error interactions
// ---------------------------------------------------------------------------

impl crate::Pathed for Error {
    fn path(&self) -> &[Integer] {
        match self {
            Error::Structural(_) => &[],
            Error::Conceptual(data) => &data.integer_vector,
        }
    }

    fn within(self, index: Integer) -> Self {
        match self {
            Error::Structural(fault) => Error::Structural(fault),
            Error::Conceptual(mut data) => {
                data.integer_vector.insert(0, index);
                Error::Conceptual(data)
            }
        }
    }
}

impl ConceptualFaulting for Error {
    fn conceptual(integer_vector: Vec<Integer>, problem: Problem) -> Self {
        Self::Conceptual(Conceptual_Data {
            integer_vector,
            problem,
        })
    }
}

impl ArityProblem for Problem {
    fn arity(first_integer: Integer, second_integer: Integer) -> Self {
        Self::Arity(Arity_Data {
            first_integer,
            second_integer,
        })
    }
}

impl From<protos::Error> for Error {
    fn from(fault: protos::Error) -> Self {
        Error::Structural(Structural_Error {
            extent: fault.extent,
            problem: fault.problem,
        })
    }
}

impl<T> Placing for Result<T, Error> {
    fn place(self, index: Integer) -> Self {
        match self {
            Ok(value) => Ok(value),
            Err(fault) => Err(fault.within(index)),
        }
    }
}

// ---------------------------------------------------------------------------
// Root and Intrinsic: named, identified by walking the variants
// ---------------------------------------------------------------------------

impl Named for Root {
    fn name(&self) -> &'static str {
        match self {
            Root::Library => "Library",
            Root::Signal => "Signal",
            Root::Sema => "Sema",
        }
    }
}

impl Identifiable for Root {
    fn identify(name: &str) -> Option<Self> {
        for root in [Root::Library, Root::Signal, Root::Sema] {
            if root.name() == name {
                return Some(root);
            }
        }
        None
    }
}

impl Rooted for File {
    fn root(&self) -> Root {
        match self {
            File::Library(_) => Root::Library,
            File::Signal(_) => Root::Signal,
            File::Sema(_) => Root::Sema,
        }
    }
}

impl Named for Intrinsic {
    fn name(&self) -> &'static str {
        match self {
            Intrinsic::String => "String",
            Intrinsic::Integer => "Integer",
            Intrinsic::Decimal => "Decimal",
            Intrinsic::Boolean => "Boolean",
            Intrinsic::Meaning => "Meaning",
            Intrinsic::Vector => "Vector",
            Intrinsic::Option => "Option",
            Intrinsic::Result => "Result",
            Intrinsic::Itself => "Self",
            Intrinsic::Sized => "Sized",
        }
    }
}

impl Identifiable for Intrinsic {
    fn identify(name: &str) -> Option<Self> {
        for intrinsic in [
            Intrinsic::String,
            Intrinsic::Integer,
            Intrinsic::Decimal,
            Intrinsic::Boolean,
            Intrinsic::Meaning,
            Intrinsic::Vector,
            Intrinsic::Option,
            Intrinsic::Result,
            Intrinsic::Itself,
            Intrinsic::Sized,
        ] {
            if intrinsic.name() == name {
                return Some(intrinsic);
            }
        }
        None
    }
}

// ---------------------------------------------------------------------------
// Passes: implementation below, each module named for its pass
// ---------------------------------------------------------------------------

mod actualization;
mod canonicalization;
mod checking;
mod conception;
mod generation;
mod protosization;

#[cfg(test)]
mod behavior {
    use super::{
        Actualizing, Canonicalizable, Error, File, Generating, Identity, Library, Name, Potential,
        TypeDeclaration,
    };
    use protos::{Extent, Protosizable, Textualizable};

    #[test]
    fn library_record_generates_named_datom_fields() {
        let file = match Potential::<File>::from("Library [] [ Record.{ String Integer } ] [] []")
            .actualize()
        {
            Ok(file) => file,
            Err(_) => panic!("approved Library record reads"),
        };
        let rust = match file.generate() {
            Ok(rust) => rust,
            Err(_) => panic!("approved Library record generates"),
        };
        assert!(rust.contains(
            "#[derive(datom_codec::Datomizable, datom_codec::Compositional, Clone, Debug, PartialEq)]"
        ));
        assert!(rust.contains("pub string: String"));
        assert!(rust.contains("pub integer: i64"));
    }

    #[test]
    fn approved_declared_and_inline_payloads_generate_named_fields() {
        let source = "Library [] [ Generation.{ String String } FilePath.String SyntaxError.Vector<FilePath> GenerationFailure.[ SyntaxError Unwritable ] Lock.{ String } LockRejection.[ DuplicateName.Lock PathOverlap.{ Lock Lock } ] ] [] []";
        let file = match Potential::<File>::from(source).actualize() {
            Ok(file) => file,
            Err(_) => {
                let canonical = source.to_owned().canonicalize().unwrap();
                panic!(
                    "approved payload examples read: {:?}",
                    canonical.text.protosize()
                )
            }
        };
        let rust = match file.generate() {
            Ok(rust) => rust,
            Err(_) => panic!("approved payload examples generate"),
        };
        assert!(rust.contains("pub first_string: String"));
        assert!(rust.contains("pub second_string: String"));
        assert!(rust.contains("SyntaxError(SyntaxError)"));
        assert!(rust.contains("PathOverlap(PathOverlap_Data)"));
        assert!(rust.contains("pub first_lock: Lock"));
        assert!(rust.contains("pub second_lock: Lock"));
    }

    #[test]
    fn library_kinds_generate_trait_surfaces() {
        let source = "Library [ std:[ Clonable Sendable Serializable ] ] [ SinkError.[ Closed ] Sink.{ String } ] [ Fillable.[ push!{ [ String ] [ Result<Integer SinkError> ] } drain![ Vector<String> ] create:[ Self ] ] Streamable.{ [ Fillable ] [ Item<Serializable> ] [ CAPACITY.Integer ] [ next![ Option<Item> ] ] } Processable<[Clonable Sendable] Serializable>.[ process.[ String ] ] ] [ Sink.[ Fillable ] ]";
        let file = match Potential::<File>::from(source).actualize() {
            Ok(file) => file,
            Err(_) => {
                let canonical = source.to_owned().canonicalize().unwrap();
                panic!("approved kinds read: {:?}", canonical.text.protosize())
            }
        };
        let rust = match file.generate() {
            Ok(rust) => rust,
            Err(_) => panic!("approved kinds generate"),
        };
        assert!(rust.contains("pub trait Fillable"));
        assert!(rust.contains("fn push("));
        assert!(rust.contains("fn create() -> Self"));
        assert!(rust.contains("pub trait Streamable"));
        assert!(rust.contains("type Item"));
        assert!(rust.contains("const CAPACITY"));
        assert!(rust.contains("pub trait Processable"));
    }

    #[test]
    fn regenerate_cli_contract() {
        let source = include_str!("../ethos-zero.ethos");
        let file = match Potential::<File>::from(source).actualize() {
            Ok(file) => file,
            Err(Error::Conceptual(_)) => panic!("CLI conceptual generation failure"),
            Err(_) => panic!("CLI structural error"),
        };
        let rust = match file.generate() {
            Ok(rust) => rust,
            Err(_) => panic!("CLI contract generates"),
        };
        std::fs::write(
            concat!(env!("CARGO_MANIFEST_DIR"), "/src/ethos-zero.rs"),
            rust,
        )
        .expect("generated contract writes");

        let source = include_str!("../error.ethos");
        let file = match Potential::<File>::from(source).actualize() {
            Ok(file) => file,
            Err(_) => panic!("error schema reads"),
        };
        let rust = match file.generate() {
            Ok(rust) => rust,
            Err(Error::Conceptual(_)) => panic!("error schema conceptual generation failure"),
            Err(Error::Structural(_)) => panic!("error schema structural generation failure"),
        };
        std::fs::write(concat!(env!("CARGO_MANIFEST_DIR"), "/src/error.rs"), rust)
            .expect("generated error module writes");
    }

    #[test]
    fn regenerate_library_fixtures() {
        let root = env!("CARGO_MANIFEST_DIR");
        for entry in std::fs::read_dir(format!("{root}/fixtures")).expect("fixture directory") {
            let entry = entry.expect("fixture entry");
            let path = entry.path();
            if path.extension().and_then(|extension| extension.to_str()) != Some("ethos") {
                continue;
            }
            let source = std::fs::read_to_string(&path).expect("fixture source");
            let file = match Potential::<File>::from(source).actualize() {
                Ok(file) => file,
                Err(Error::Conceptual(_)) => panic!("{} does not read", path.display()),
                Err(Error::Structural(_)) => panic!("{} is structurally invalid", path.display()),
            };
            let rust = file
                .generate()
                .unwrap_or_else(|_| panic!("{} generates", path.display()));
            let stem = path
                .file_stem()
                .and_then(|stem| stem.to_str())
                .expect("fixture stem");
            std::fs::write(format!("{root}/tests/generated/{stem}.rs"), rust)
                .expect("generated fixture writes");
        }
    }

    #[test]
    fn canonical_ascent_round_trips_nonempty_library_and_signal() {
        let library = "Library [ crate:[ Capability ] external:[ Vector ] ] [ FilePath.String SyntaxError.Vector<FilePath> External.external:Vector<FilePath> Record.{ Vector<Option<String>> Result<String Integer> } State.[ Idle Busy ] ] [ Fillable.[ fill!{ [ Vector<Option<String>> ] [ Result<String Integer> ] } ] Processable<[Clonable Sendable] Serializable>.[ process.[ String ] ] ] [ Record.[ Fillable ] ]";
        let signal = "Signal [ datom_codec:[ Error Path ] ] [ Generate.Generation ] [ Generated.String Malformed.Error ] [ Generation.{ String String } ]";
        for source in [library, signal] {
            let file = match Potential::<File>::from(source).actualize() {
                Ok(file) => file,
                Err(_) => panic!("approved source reads"),
            };
            let canonical = file.protosize().textualize();
            let repeated = match Potential::<File>::from(canonical).actualize() {
                Ok(file) => file,
                Err(_) => panic!("ascent reads"),
            };
            assert_eq!(file, repeated);
        }
    }

    #[test]
    fn sema_has_imports_and_record_type_declarations_only() {
        let source = "Sema [ crate:[ Handle ] ] [ Record.{ Handle String } ]";
        let file = match Potential::<File>::from(source).actualize() {
            Ok(file) => file,
            Err(_) => panic!("approved Sema reads"),
        };
        assert!(matches!(file, File::Sema(_)));
        let repeated = match Potential::<File>::from(file.protosize().textualize()).actualize() {
            Ok(file) => file,
            Err(_) => panic!("Sema ascent reads"),
        };
        assert_eq!(file, repeated);
    }

    #[test]
    fn retired_type_and_kind_roots_are_rejected() {
        for source in ["Types [] [] []", "Kinds [] []"] {
            assert!(Potential::<File>::from(source).actualize().is_err());
        }
    }

    #[test]
    fn constrained_data_declarations_are_rejected() {
        let source = "Library [] [ Box<Sized>.{ String } ] [] []";
        assert!(matches!(
            Potential::<File>::from(source).actualize(),
            Err(Error::Conceptual(_))
        ));
    }

    #[test]
    fn manually_constructed_constrained_data_is_rejected_before_generation() {
        use crate::{Constraint, Reference, Scope, checking::Checkable};

        let file = File::Library(Library {
            imports: vec![],
            types: vec![TypeDeclaration::Struct(
                Identity {
                    name: Name("Box".into()),
                    constraints: vec![Constraint::One(Reference {
                        source: None,
                        name: Name("Sized".into()),
                        arguments: vec![],
                    })],
                },
                vec![],
            )],
            kinds: vec![],
            associations: vec![],
        });
        let scope = Scope {
            file: &file,
            identity: None,
            associated: &[],
        };
        assert!(matches!(file.check(&scope), Err(Error::Conceptual(_))));
    }

    #[test]
    fn nested_inline_data_names_include_ancestry_after_the_first_level() {
        let source = "Library [] [ Outer.[ A.[ X.{ String } ] B.[ X.{ Integer } ] ] Rejection.[ PathOverlap.{ String String } ] ] [] []";
        let file = match Potential::<File>::from(source).actualize() {
            Ok(file) => file,
            Err(_) => panic!("nested collision source reads"),
        };
        let rust = match file.generate() {
            Ok(rust) => rust,
            Err(_) => panic!("nested collision source generates"),
        };
        assert!(rust.contains("struct A_Data_X_Data"));
        assert!(rust.contains("struct B_Data_X_Data"));
        assert!(rust.contains("struct PathOverlap_Data"));
        syn::parse_file(&rust).expect("generated nested data is Rust");
    }

    #[test]
    fn conceptual_errors_preserve_the_bad_declaration_path() {
        let source = "Library.{ [] [ Bad ] [] [] }";
        match Potential::<File>::from(source).actualize() {
            Err(Error::Conceptual(data)) => assert_eq!(data.integer_vector, vec![1, 0]),
            _ => panic!("bad declaration must retain its section and child path"),
        }
    }

    #[test]
    fn sweet_structural_errors_resituate_to_the_authored_source() {
        let source = "Library\n[]\n[ Record.{ String ]\n[]\n[]";
        match Potential::<File>::from(source).actualize() {
            Err(Error::Structural(data)) => {
                assert_eq!(data.extent, Extent { start: 29, end: 29 });
                assert_eq!(source.as_bytes()[data.extent.start], b']');
            }
            _ => panic!("expected structural error"),
        }
    }

    #[test]
    fn public_file_protosization_has_the_shared_exact_canonical_extents() {
        let source = "Library [] [ Alias.Vector<Option<String>> Record.{ Vector<Option<String>> Result<String Integer> } ] [ Processable<[Clonable Sendable] Serializable>.[ process.[ String ] ] ] []";
        let file = match Potential::<File>::from(source).actualize() {
            Ok(file) => file,
            Err(_) => panic!("generic source reads"),
        };
        let projected = file.protosize();
        let canonical = projected.textualize();
        let reparsed = canonical.protosize().expect("canonical ascent reads");
        assert_eq!(projected, reparsed);
        let protos::Protos::Headed { extent, .. } = projected else {
            panic!("a File ascends as a headed Protos form")
        };
        assert_eq!(
            extent,
            Extent {
                start: 0,
                end: canonical.len()
            }
        );
    }

    #[test]
    fn wide_manual_file_ascent_never_uses_a_reader_budget() {
        let mut types = Vec::new();
        for index in 0..5_000 {
            types.push(TypeDeclaration::Struct(
                Identity {
                    name: Name(format!("Entry{index}")),
                    constraints: vec![],
                },
                vec![],
            ));
        }
        let file = File::Library(Library {
            imports: vec![],
            types,
            kinds: vec![],
            associations: vec![],
        });
        let protos = file.protosize();
        let protos::Protos::Headed { extent, .. } = &protos else {
            panic!("a File ascends as a headed Protos form")
        };
        assert_eq!(extent.start, 0);
        assert_eq!(extent.end, protos.textualize().len());
    }

    #[test]
    fn signal_sema_import_and_attached_generic_errors_keep_root_paths() {
        let cases = [
            ("Signal [ 1 ] [] [] []", vec![0, 0]),
            ("Sema [] [ 1 ]", vec![1, 0]),
            ("Library [] [ Alias.Vector<1> ] [] []", vec![1, 1, 0]),
        ];
        for (source, expected) in cases {
            match Potential::<File>::from(source).actualize() {
                Err(Error::Conceptual(data)) => assert_eq!(data.integer_vector, expected),
                _ => panic!("{source} must retain its conceptual path"),
            }
        }
    }

    #[test]
    fn kind_capability_and_association_errors_keep_all_structural_parents() {
        let cases = [
            ("Library [] [] [ K.[ 1 ] ] []", vec![2, 0, 1, 0]),
            (
                "Library [] [] [ K.{ [ 1 ] [] [] [] } ] []",
                vec![2, 0, 1, 0, 0],
            ),
            (
                "Library [] [ T.String ] [ K.[] ] [ T.[ 1 ] ]",
                vec![3, 0, 1, 0],
            ),
        ];
        for (source, expected) in cases {
            match Potential::<File>::from(source).actualize() {
                Err(Error::Conceptual(data)) => assert_eq!(data.integer_vector, expected),
                _ => panic!("{source} must retain every structural parent"),
            }
        }
    }
}
