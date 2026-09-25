//! Generation: File to Rust text (cannot err, the file having been checked).
//!
//! Each declaration emits itself: a struct declaration its struct and
//! its datomic machinery, an enum declaration its enum and its
//! machinery, a kind declaration its trait, an association its
//! assertion; the file emits by walking its variant's sections. Names
//! are resolved through the scope, never a table; the generated Rust
//! carries no `use` and writes every foreign name fully qualified.
//!
//! One rule decides boxing: a position is boxed where it closes a by-value
//! cycle, that is where it names, through aliases, `Option` and `Result` but
//! not through `Vector`, a type declared no later than its owner that reaches
//! the owner by value. The box sits immediately around the recursive
//! argument (`std::option::Option<std::boxed::Box<Tree>>`).
//!
//! One rule decides archive bounds: in a Signal, a position that reaches its
//! owner by any path, `Vector` included, omits its rkyv bounds, and its type
//! states the serializer, deserializer and validator bounds once instead, so
//! a recursive type archives and restores.

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use crate::checking::{Checkable, Declaring, Inlining};
use crate::{
    AssociatedConstant, AssociatedType, Association, Capability, Constraint, File, Generating,
    Identity, Intrinsic, KindBody, KindDeclaration, Name, Receiver, Reference, Resolution,
    Resolving, Scope, Signature, Source, TypeDeclaration, Variant,
};

// ---------------------------------------------------------------------------
// Tokens of names and sources
// ---------------------------------------------------------------------------

/// The kind whose capability yields a value's Rust tokens without any scope.
pub(crate) trait Tokening {
    fn tokens(&self) -> TokenStream;
}

impl Tokening for Name {
    fn tokens(&self) -> TokenStream {
        let ident = Ident::new(&self.0, Span::call_site());
        quote! { #ident }
    }
}

impl Tokening for Source {
    fn tokens(&self) -> TokenStream {
        let path: syn::Path =
            syn::parse_str(self.as_ref()).expect("a source was validated as a path");
        quote! { #path }
    }
}

impl Tokening for Intrinsic {
    fn tokens(&self) -> TokenStream {
        match self {
            Intrinsic::String => quote! { String },
            // Signal contracts must compile without their optional `datom`
            // feature.  The wire-level integer is therefore the ordinary
            // Rust scalar; Datom derives know how to compose it when enabled.
            Intrinsic::Integer => quote! { i64 },
            // A datom decimal is finite and point-mandatory. `f64` is neither,
            // so it has no datom text; `datom_codec::Decimal` admits only a
            // finite value, which is also what makes a type reaching one Eq
            // and Hash.
            Intrinsic::Decimal => quote! { datom_codec::Decimal },
            Intrinsic::Boolean => quote! { bool },
            Intrinsic::Meaning => quote! { datom_codec::Meaning },
            Intrinsic::Vector => quote! { std::vec::Vec },
            Intrinsic::Option => quote! { std::option::Option },
            Intrinsic::Result => quote! { std::result::Result },
            Intrinsic::Itself => quote! { Self },
            Intrinsic::Sized => quote! { Sized },
        }
    }
}

/// The kind whose capability yields the name of the parameter at an index: A, B, C.
trait Lettering {
    fn letter(&self, scope: &Scope) -> Ident;
}

impl Lettering for usize {
    fn letter(&self, scope: &Scope) -> Ident {
        let letter = char::from(b'A' + *self as u8);
        let simple = Name::try_from(letter.to_string()).expect("static identifier");
        let allocated = if scope.file.declaration(&simple).is_none() {
            simple
        } else {
            let mut allocated = Name::try_from(format!("{}EthosParameter", simple.0))
                .expect("parameter identifiers are identifiers");
            while scope.file.declaration(&allocated).is_some() {
                allocated = Name::try_from(format!("{}X", allocated.0))
                    .expect("parameter identifiers are identifiers");
            }
            allocated
        };
        Ident::new(&allocated.0, Span::call_site())
    }
}

/// The kind whose capability lowercases a name for an assertion function.
trait Lowering {
    fn lowered(&self) -> String;
}

impl Lowering for Reference {
    fn lowered(&self) -> String {
        let mut lowered = self.name.0.to_lowercase();
        for argument in &self.arguments {
            lowered.push('_');
            lowered.push_str(&argument.lowered());
        }
        lowered
    }
}

// ---------------------------------------------------------------------------
// Emitting: Rust tokens in a scope
// ---------------------------------------------------------------------------

/// The kind whose capability yields a value's Rust tokens in a scope.
pub(crate) trait Emitting {
    fn emit(&self, scope: &Scope) -> TokenStream;
}

impl Emitting for Reference {
    fn emit(&self, scope: &Scope) -> TokenStream {
        let mut arguments = Vec::with_capacity(self.arguments.len());
        for argument in &self.arguments {
            arguments.push(argument.emit(scope));
        }
        let applied = if arguments.is_empty() {
            TokenStream::new()
        } else {
            quote! { < #( #arguments ),* > }
        };
        let name = self.name.tokens();
        if let Some(source) = &self.source {
            let source = source.tokens();
            return quote! { #source :: #name #applied };
        }
        match scope.resolve(&self.name) {
            Resolution::Intrinsic(intrinsic) => {
                let intrinsic = intrinsic.tokens();
                quote! { #intrinsic #applied }
            }
            Resolution::Imported(source, emitted) => {
                if source.as_ref() == "std" {
                    if emitted.0 == "Clone" {
                        return quote! { std::clone::Clone #applied };
                    }
                    if emitted.0 == "Send" {
                        return quote! { std::marker::Send #applied };
                    }
                }
                let source = source.tokens();
                let emitted = emitted.tokens();
                quote! { #source :: #emitted #applied }
            }
            Resolution::Type(_)
            | Resolution::Kind(_)
            | Resolution::Ambiguous(_)
            | Resolution::Undeclared => {
                quote! { #name #applied }
            }
            Resolution::Parameter(index) => {
                let letter = (index as usize).letter(scope);
                quote! { #letter }
            }
            Resolution::Associated(name) => {
                let name = name.tokens();
                quote! { Self::#name }
            }
        }
    }
}

/// The kind whose capability yields the bounds of a constraint: `A + B`.
trait Bounding {
    fn bounds(&self, scope: &Scope) -> TokenStream;
}

impl Bounding for Constraint {
    fn bounds(&self, scope: &Scope) -> TokenStream {
        let references = match self {
            Constraint::One(reference) => std::slice::from_ref(reference),
            Constraint::Many(references) => references,
        };
        references.bounds(scope)
    }
}

impl Bounding for [Reference] {
    fn bounds(&self, scope: &Scope) -> TokenStream {
        let mut bounds = Vec::with_capacity(self.len());
        for reference in self {
            bounds.push(reference.emit(scope));
        }
        quote! { #( #bounds )+* }
    }
}

/// The kind whose capabilities yield an identity's generics: the parameters with their bounds, and the arguments.
pub(crate) trait Parametrizing {
    fn parameters(&self, scope: &Scope) -> TokenStream;
    fn arguments(&self, scope: &Scope) -> TokenStream;
}

impl Parametrizing for Identity {
    fn parameters(&self, scope: &Scope) -> TokenStream {
        if self.constraints.is_empty() {
            return TokenStream::new();
        }
        // The bounds name kinds outside the identity they bound.
        let outer = Scope {
            file: scope.file,
            identity: None,
            associated: scope.associated,
        };
        let mut parameters = Vec::with_capacity(self.constraints.len());
        for (index, constraint) in self.constraints.iter().enumerate() {
            let letter = index.letter(scope);
            let bounds = constraint.bounds(&outer);
            parameters.push(quote! { #letter: #bounds });
        }
        quote! { < #( #parameters ),* > }
    }

    fn arguments(&self, scope: &Scope) -> TokenStream {
        if self.constraints.is_empty() {
            return TokenStream::new();
        }
        let mut letters = Vec::with_capacity(self.constraints.len());
        for index in 0..self.constraints.len() {
            letters.push(index.letter(scope));
        }
        quote! { < #( #letters ),* > }
    }
}

// ---------------------------------------------------------------------------
// Reaching: the boxing rule and the archive-bound rule
// ---------------------------------------------------------------------------

/// Which containment a reach walks through. A value holds what it reaches
/// `ByValue` inside its own size; a `Vector` puts its elements behind a heap
/// pointer, so it stops a by-value reach but not an `Any` reach, which is
/// what trait bounds follow.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Passage {
    ByValue,
    Any,
}

/// The kind whose capability tells whether a value holds the target type, through declared types, aliases, Option and Result, and Vector when the passage allows it.
trait Reaching {
    fn reaches(
        &self,
        target: &Name,
        passage: Passage,
        file: &File,
        visited: &mut Vec<Name>,
    ) -> bool;
}

impl Reaching for Reference {
    fn reaches(
        &self,
        target: &Name,
        passage: Passage,
        file: &File,
        visited: &mut Vec<Name>,
    ) -> bool {
        if self.source.is_none() {
            if &self.name == target || self.name.0 == "Self" {
                return true;
            }
            match file.resolve(&self.name) {
                Resolution::Intrinsic(Intrinsic::Vector) if passage == Passage::ByValue => {
                    return false;
                }
                Resolution::Type(name) if !visited.contains(&name) => {
                    visited.push(name.clone());
                    if let Some(declaration) = file.declaration(&name)
                        && declaration.reaches(target, passage, file, visited)
                    {
                        return true;
                    }
                }
                _ => {}
            }
        }
        for argument in &self.arguments {
            if argument.reaches(target, passage, file, visited) {
                return true;
            }
        }
        false
    }
}

impl Reaching for [Reference] {
    fn reaches(
        &self,
        target: &Name,
        passage: Passage,
        file: &File,
        visited: &mut Vec<Name>,
    ) -> bool {
        for reference in self {
            if reference.reaches(target, passage, file, visited) {
                return true;
            }
        }
        false
    }
}

impl Reaching for TypeDeclaration {
    fn reaches(
        &self,
        target: &Name,
        passage: Passage,
        file: &File,
        visited: &mut Vec<Name>,
    ) -> bool {
        match self {
            TypeDeclaration::Struct(_, positions) => {
                positions.reaches(target, passage, file, visited)
            }
            TypeDeclaration::Enum(_, variants) => variants.reaches(target, passage, file, visited),
            TypeDeclaration::Alias(_, aliased) => aliased.reaches(target, passage, file, visited),
        }
    }
}

impl Reaching for Variant {
    fn reaches(
        &self,
        target: &Name,
        passage: Passage,
        file: &File,
        visited: &mut Vec<Name>,
    ) -> bool {
        match self {
            Variant::Bare(name) => match file.declaration(name) {
                Some(_) => name.carried().reaches(target, passage, file, visited),
                None => false,
            },
            Variant::Typed(_, reference) => reference.reaches(target, passage, file, visited),
            Variant::Struct(_, positions) => positions.reaches(target, passage, file, visited),
            Variant::Enum(_, variants) => variants.reaches(target, passage, file, visited),
        }
    }
}

impl Reaching for [Variant] {
    fn reaches(
        &self,
        target: &Name,
        passage: Passage,
        file: &File,
        visited: &mut Vec<Name>,
    ) -> bool {
        for variant in self {
            if variant.reaches(target, passage, file, visited) {
                return true;
            }
        }
        false
    }
}

/// The kind whose capability yields the reference a bare variant naming a declared type carries.
trait Carried {
    fn carried(&self) -> Reference;
}

impl Carried for Name {
    fn carried(&self) -> Reference {
        Reference {
            source: None,
            name: self.clone(),
            arguments: vec![],
        }
    }
}

/// The kind whose capability gives a declared type's place in its file's declaration order.
trait Placing {
    fn place(&self, name: &Name) -> Option<usize>;
}

impl Placing for File {
    fn place(&self, name: &Name) -> Option<usize> {
        let declarations = match self {
            File::Library(library) => &library.types,
            File::Signal(signal) => &signal.types,
            File::Sema(sema) => &sema.types,
        };
        declarations
            .iter()
            .position(|declaration| declaration.resolve(name) != Resolution::Undeclared)
    }
}

/// The kind whose capability tells whether a reference closes a by-value
/// cycle back to its owner: it names, by value, a struct or enum declared no
/// later than the owner that reaches the owner by value. Every by-value cycle
/// has at least one edge that does not move forward in declaration order, so
/// boxing exactly these edges breaks every cycle, and an edge that moves
/// forward is left unboxed: in `Twin.{ Twig Twig }  Twig.[ Tip  Grow.Twin ]`
/// only `Grow` is boxed.
trait Closing {
    fn closes(&self, owner: &Name, file: &File) -> bool;
}

impl Closing for Reference {
    fn closes(&self, owner: &Name, file: &File) -> bool {
        if self.source.is_some() {
            return false;
        }
        if &self.name == owner || self.name.0 == "Self" {
            return true;
        }
        match file.resolve(&self.name) {
            Resolution::Intrinsic(Intrinsic::Vector) => return false,
            Resolution::Type(name) => match file.declaration(&name) {
                Some(TypeDeclaration::Alias(_, aliased)) => {
                    if aliased.closes(owner, file) {
                        return true;
                    }
                }
                Some(declaration) => {
                    let earlier = match (file.place(&name), file.place(owner)) {
                        (Some(named), Some(owning)) => named <= owning,
                        _ => false,
                    };
                    if earlier
                        && declaration.reaches(owner, Passage::ByValue, file, &mut vec![name])
                    {
                        return true;
                    }
                }
                None => {}
            },
            _ => {}
        }
        for argument in &self.arguments {
            if argument.closes(owner, file) {
                return true;
            }
        }
        false
    }
}

/// The kind whose capabilities yield a position's Rust type, boxed where it
/// closes a by-value cycle, and the archive attribute it bears.
pub(crate) trait Positioning {
    fn boxed(&self, scope: &Scope, owner: &Name) -> bool;
    fn recursive(&self, scope: &Scope, owner: &Name) -> bool;
    fn position(&self, scope: &Scope, owner: &Name) -> TokenStream;
    fn archival(&self, scope: &Scope, owner: &Name, carriage: Carriage) -> TokenStream;
}

impl Positioning for Reference {
    fn boxed(&self, scope: &Scope, owner: &Name) -> bool {
        self.closes(owner, scope.file)
    }

    fn recursive(&self, scope: &Scope, owner: &Name) -> bool {
        self.reaches(owner, Passage::Any, scope.file, &mut vec![])
    }

    fn position(&self, scope: &Scope, owner: &Name) -> TokenStream {
        // Put the indirection immediately around the recursive argument.
        // `Option<Box<Tree>>` lets the Datom derive see the recursive edge,
        // while `Box<Option<Tree>>` hides it behind the container and causes
        // an infinitely recursive derive bound.
        if self.source.is_none()
            && let Resolution::Intrinsic(intrinsic @ (Intrinsic::Option | Intrinsic::Result)) =
                scope.file.resolve(&self.name)
        {
            let name = intrinsic.tokens();
            let mut arguments = Vec::with_capacity(self.arguments.len());
            for argument in &self.arguments {
                let ty = if argument.boxed(scope, owner) {
                    let inner = argument.emit(scope);
                    quote! { std::boxed::Box<#inner> }
                } else {
                    argument.emit(scope)
                };
                arguments.push(ty);
            }
            return quote! { #name < #( #arguments ),* > };
        }

        let ty = self.emit(scope);
        if self.boxed(scope, owner) {
            quote! { std::boxed::Box<#ty> }
        } else {
            ty
        }
    }

    // rkyv's derive bounds every field's type by the trait it derives, so a
    // position that reaches its owner, through a Vector as much as a Box,
    // makes the bound depend on itself and the trait solver overflows. Such
    // a position omits its bound; its type then states the bounds its
    // containers need instead (`Recursing`).
    fn archival(&self, scope: &Scope, owner: &Name, carriage: Carriage) -> TokenStream {
        if carriage == Carriage::Archived && self.recursive(scope, owner) {
            quote! { #[rkyv(omit_bounds)] }
        } else {
            TokenStream::new()
        }
    }
}

/// The kind whose capability yields the archive bounds a type states once
/// some position of it omits its own: what `Box` and `Vec` ask of the
/// serializer, the deserializer and the validator, named once for the type.
trait Recursing {
    fn recursion_bounds(&self) -> TokenStream;
}

impl Recursing for Carriage {
    fn recursion_bounds(&self) -> TokenStream {
        match self {
            Carriage::Archived => quote! {
                #[rkyv(serialize_bounds(__S: rkyv::ser::Writer + rkyv::ser::Allocator, __S::Error: rkyv::rancor::Source))]
                #[rkyv(deserialize_bounds(__D::Error: rkyv::rancor::Source))]
                #[rkyv(bytecheck(bounds(__C: rkyv::validation::ArchiveContext, __C::Error: rkyv::rancor::Source)))]
            },
            Carriage::Plain => TokenStream::new(),
        }
    }
}

/// The kind whose capability yields the stem a position's field name is built from.
trait Fielding {
    fn field_base(&self, owner: &Name) -> String;
}

trait SnakeCasing {
    fn snake_case(&self) -> String;
}

impl SnakeCasing for str {
    fn snake_case(&self) -> String {
        let characters: Vec<char> = self.chars().collect();
        let mut result = String::new();
        for (index, character) in characters.iter().enumerate() {
            let previous = index.checked_sub(1).and_then(|i| characters.get(i));
            let next = characters.get(index + 1);
            if character.is_uppercase()
                && index != 0
                && (previous.is_some_and(|previous| previous.is_lowercase())
                    || next.is_some_and(|next| next.is_lowercase()))
            {
                result.push('_');
            }
            result.extend(character.to_lowercase());
        }
        result
    }
}

impl Fielding for Reference {
    fn field_base(&self, owner: &Name) -> String {
        let mut parts: Vec<String> = self
            .arguments
            .iter()
            .map(|argument| argument.field_base(owner))
            .collect();
        // `Self` in a position is the enclosing type, and `self` is not a name a
        // field may bear; the field takes the enclosing type's name instead.
        let named = if self.source.is_none() && self.name.0 == "Self" {
            owner
        } else {
            &self.name
        };
        parts.push(named.0.as_str().snake_case());
        parts.join("_")
    }
}

trait Ordinaling {
    fn ordinal(&self) -> &'static str;
}

impl Ordinaling for usize {
    fn ordinal(&self) -> &'static str {
        match self {
            0 => "first",
            1 => "second",
            2 => "third",
            3 => "fourth",
            4 => "fifth",
            5 => "sixth",
            6 => "seventh",
            7 => "eighth",
            8 => "ninth",
            9 => "tenth",
            _ => "position",
        }
    }
}

/// The kind whose capability names every field of a struct of these positions.
trait FieldNaming {
    fn field_names(&self, owner: &Name) -> Vec<Ident>;
}

impl FieldNaming for [Reference] {
    fn field_names(&self, owner: &Name) -> Vec<Ident> {
        let bases: Vec<String> = self
            .iter()
            .map(|position| position.field_base(owner))
            .collect();
        let mut fields = Vec::with_capacity(bases.len());
        for (index, base) in bases.iter().enumerate() {
            let repetitions = bases.iter().filter(|other| *other == base).count();
            let prior = bases[..index].iter().filter(|other| *other == base).count();
            let name = if repetitions == 1 {
                base.clone()
            } else if prior < 10 {
                format!("{}_{}", prior.ordinal(), base)
            } else {
                format!("position_{}_{}", prior + 1, base)
            };
            let field = if syn::parse_str::<Ident>(&name).is_ok() {
                Ident::new(&name, Span::call_site())
            } else if matches!(name.as_str(), "crate" | "self" | "super" | "Self") {
                // The four keywords no raw identifier may spell.
                Ident::new(&format!("{name}_"), Span::call_site())
            } else {
                Ident::new_raw(&name, Span::call_site())
            };
            fields.push(field);
        }
        fields
    }
}

/// The kind whose capabilities yield a variant's definition and the items its inline enum needs.
trait Varianted {
    fn definition(
        &self,
        scope: &Scope,
        owner: &Name,
        enclosing: &Identity,
        carriage: Carriage,
    ) -> TokenStream;
    fn recursive(&self, scope: &Scope, owner: &Name) -> bool;
    fn nested(
        &self,
        scope: &Scope,
        owner: &Name,
        enclosing: &Identity,
        carriage: Carriage,
    ) -> TokenStream;
}

/// The kind whose capability yields the identity of the payload a variant
/// declares in place, named file-wide unique by the file ([`Inlining`]).
trait Nesting {
    fn nested_identity(&self, scope: &Scope, owner: &Name, name: &Name) -> Identity;
}

impl Nesting for Identity {
    fn nested_identity(&self, scope: &Scope, owner: &Name, name: &Name) -> Identity {
        Identity {
            name: scope.file.inline_name(owner, &self.name, name),
            constraints: self.constraints.clone(),
        }
    }
}

impl Varianted for Variant {
    fn definition(
        &self,
        scope: &Scope,
        owner: &Name,
        enclosing: &Identity,
        carriage: Carriage,
    ) -> TokenStream {
        match self {
            Variant::Bare(name) => {
                let variant = name.tokens();
                if scope.file.declaration(name).is_some() {
                    let carried = name.carried();
                    let archival = carried.archival(scope, owner, carriage);
                    let ty = carried.position(scope, owner);
                    quote! { #variant(#archival #ty) }
                } else {
                    quote! { #variant }
                }
            }
            Variant::Typed(name, reference) => {
                let name = name.tokens();
                let archival = reference.archival(scope, owner, carriage);
                let ty = reference.position(scope, owner);
                quote! { #name(#archival #ty) }
            }
            Variant::Struct(name, _) | Variant::Enum(name, _) => {
                let nested = enclosing.nested_identity(scope, owner, name);
                let ty = nested.name.tokens();
                let arguments = nested.arguments(scope);
                let name = name.tokens();
                quote! { #name(#ty #arguments) }
            }
        }
    }

    fn nested(
        &self,
        scope: &Scope,
        owner: &Name,
        enclosing: &Identity,
        carriage: Carriage,
    ) -> TokenStream {
        match self {
            Variant::Struct(name, positions) => {
                let nested = enclosing.nested_identity(scope, owner, name);
                positions.structure(scope, owner, &nested, carriage)
            }
            Variant::Enum(name, variants) => {
                let nested = enclosing.nested_identity(scope, owner, name);
                variants.enumeration(scope, owner, &nested, carriage)
            }
            Variant::Bare(_) | Variant::Typed(_, _) => TokenStream::new(),
        }
    }

    // Only the variant's own field counts: an inline payload is its own
    // type, and its positions omit their bounds there.
    fn recursive(&self, scope: &Scope, owner: &Name) -> bool {
        match self {
            Variant::Bare(name) => {
                scope.file.declaration(name).is_some() && name.carried().recursive(scope, owner)
            }
            Variant::Typed(_, reference) => reference.recursive(scope, owner),
            Variant::Struct(_, _) | Variant::Enum(_, _) => false,
        }
    }
}

// ---------------------------------------------------------------------------
// Declaring: the items a type declaration emits
// ---------------------------------------------------------------------------

/// The kind whose capability emits a struct of these positions with its datomic machinery.
trait Structuring {
    fn structure(
        &self,
        scope: &Scope,
        owner: &Name,
        identity: &Identity,
        carriage: Carriage,
    ) -> TokenStream;
}

/// The kind whose capability emits an enum of these variants with its datomic machinery.
trait Enumerating {
    fn enumeration(
        &self,
        scope: &Scope,
        owner: &Name,
        identity: &Identity,
        carriage: Carriage,
    ) -> TokenStream;
}

/// How a declaration's projection is carried. A Signal's types cross a wire,
/// so they archive and their datom kinds are gated behind the `datom` feature
/// the Nexus does not enable; a Library's or a Sema's do not.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Carriage {
    Archived,
    Plain,
}

trait DatomDeriving {
    fn datom_derives(&self) -> TokenStream;
}

/// What every declared type derives, and what none does.
///
/// `Clone`, `Debug` and `PartialEq` always. `Eq` and `Hash` always too: every
/// intrinsic a position can hold is now `Eq` and `Hash`, `Decimal` included,
/// because a datom decimal is finite. Without them no contract value could be
/// a map key, and the orphan rule leaves a consumer no way to add them.
///
/// Never `Copy`: a wire type's size is not part of its contract, and a
/// contract that grew a `String` position would silently break every consumer
/// relying on it. Never `Default`: a default is a policy the consumer holds,
/// not a value the wire carries, and a manufactured zero satisfies the type
/// while violating the schema's invariants.
impl DatomDeriving for Carriage {
    fn datom_derives(&self) -> TokenStream {
        match self {
            Carriage::Archived => quote! {
                #[rustfmt::skip]
                #[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
                #[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
            },
            Carriage::Plain => quote! {
                #[rustfmt::skip]
                #[derive(datom_codec::Datomizable, datom_codec::Composing, Clone, Debug, PartialEq, Eq, Hash)]
            },
        }
    }
}

/// A file says how the types it declares are carried.
pub trait Carrying {
    fn carriage(&self) -> Carriage;
}

impl Carrying for File {
    fn carriage(&self) -> Carriage {
        match self {
            File::Signal(_) => Carriage::Archived,
            File::Library(_) | File::Sema(_) => Carriage::Plain,
        }
    }
}

impl Structuring for [Reference] {
    fn structure(
        &self,
        scope: &Scope,
        owner: &Name,
        identity: &Identity,
        carriage: Carriage,
    ) -> TokenStream {
        let name = identity.name.tokens();
        let parameters = identity.parameters(scope);
        let mut types = Vec::with_capacity(self.len());
        let mut archivals = Vec::with_capacity(self.len());
        let mut recursive = false;
        for position in self {
            types.push(position.position(scope, owner));
            archivals.push(position.archival(scope, owner, carriage));
            recursive |= position.recursive(scope, owner);
        }
        let fields = self.field_names(owner);
        let derive = carriage.datom_derives();
        let bounds = if recursive {
            carriage.recursion_bounds()
        } else {
            TokenStream::new()
        };
        quote! {
            #derive
            #bounds
            pub struct #name #parameters { #( #archivals pub #fields: #types ),* }
        }
    }
}

impl Enumerating for [Variant] {
    fn enumeration(
        &self,
        scope: &Scope,
        owner: &Name,
        identity: &Identity,
        carriage: Carriage,
    ) -> TokenStream {
        let name = identity.name.tokens();
        let parameters = identity.parameters(scope);
        let derive = carriage.datom_derives();
        let mut definitions = Vec::with_capacity(self.len());
        let mut nested = Vec::new();
        let mut recursive = false;
        for variant in self {
            definitions.push(variant.definition(scope, owner, identity, carriage));
            nested.push(variant.nested(scope, owner, identity, carriage));
            recursive |= variant.recursive(scope, owner);
        }
        let bounds = if recursive {
            carriage.recursion_bounds()
        } else {
            TokenStream::new()
        };
        quote! {
            #( #nested )*
            #derive
            #bounds
            pub enum #name #parameters { #( #definitions ),* }
        }
    }
}

impl Emitting for TypeDeclaration {
    fn emit(&self, scope: &Scope) -> TokenStream {
        match self {
            TypeDeclaration::Struct(identity, positions) => {
                let inner = Scope {
                    file: scope.file,
                    identity: Some(identity),
                    associated: scope.associated,
                };
                positions.structure(&inner, &identity.name, identity, scope.file.carriage())
            }
            TypeDeclaration::Enum(identity, variants) => {
                let inner = Scope {
                    file: scope.file,
                    identity: Some(identity),
                    associated: scope.associated,
                };
                variants.enumeration(&inner, &identity.name, identity, scope.file.carriage())
            }
            TypeDeclaration::Alias(identity, aliased) => {
                let inner = Scope {
                    file: scope.file,
                    identity: Some(identity),
                    associated: scope.associated,
                };
                let name = identity.name.tokens();
                let parameters = identity.parameters(&inner);
                let aliased = aliased.emit(&inner);
                quote! { #[rustfmt::skip] pub type #name #parameters = #aliased; }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Kinds: traits
// ---------------------------------------------------------------------------

/// The named sections that compose a kind declaration.
struct KindContents<'a> {
    superkinds: &'a [Reference],
    types: &'a [AssociatedType],
    constants: &'a [AssociatedConstant],
    capabilities: &'a [Capability],
}

/// The kind whose capability exposes a declaration's named kind sections.
trait Containing {
    fn contents(&self) -> KindContents<'_>;
}

impl Containing for KindDeclaration {
    fn contents(&self) -> KindContents<'_> {
        match &self.body {
            KindBody::Simple(capabilities) => KindContents {
                superkinds: &[],
                types: &[],
                constants: &[],
                capabilities,
            },
            KindBody::Complex {
                superkinds,
                types,
                constants,
                capabilities,
            } => KindContents {
                superkinds,
                types,
                constants,
                capabilities,
            },
        }
    }
}

impl Emitting for AssociatedType {
    fn emit(&self, scope: &Scope) -> TokenStream {
        let name = self.name.tokens();
        if self.bounds.is_empty() {
            quote! { type #name; }
        } else {
            let bounds = self.bounds.bounds(scope);
            quote! { type #name: #bounds; }
        }
    }
}

impl Emitting for AssociatedConstant {
    fn emit(&self, scope: &Scope) -> TokenStream {
        let name = self.name.tokens();
        let ty = self.ty.emit(scope);
        quote! { const #name: #ty; }
    }
}

impl Emitting for Capability {
    fn emit(&self, scope: &Scope) -> TokenStream {
        let name = self.name.tokens();
        let mut parameters = Vec::new();
        match self.receiver {
            Receiver::Shared => parameters.push(quote! { &self }),
            Receiver::Mutable => parameters.push(quote! { &mut self }),
            Receiver::Static => {}
        }
        let yields = match &self.signature {
            Signature::Yielding(yields) => yields,
            Signature::Taking(inputs, yields) => {
                if let [input] = inputs.as_slice() {
                    let ty = input.emit(scope);
                    parameters.push(quote! { input: #ty });
                } else {
                    for (index, input) in inputs.iter().enumerate() {
                        let input_name = Ident::new(&format!("input_{index}"), Span::call_site());
                        let ty = input.emit(scope);
                        parameters.push(quote! { #input_name: #ty });
                    }
                }
                yields
            }
        };
        let yields = yields.emit(scope);
        let sized = if self.contains_self() {
            quote! { where Self: Sized }
        } else {
            TokenStream::new()
        };
        quote! { fn #name( #( #parameters ),* ) -> #yields #sized; }
    }
}

/// Whether a capability signature mentions `Self`, which requires a sized
/// trait receiver when lowered into Rust's sized generic constructors.
trait SelfContaining {
    fn contains_self(&self) -> bool;
}

impl SelfContaining for Reference {
    fn contains_self(&self) -> bool {
        (self.source.is_none() && self.name.0 == "Self")
            || self.arguments.iter().any(SelfContaining::contains_self)
    }
}

impl SelfContaining for Capability {
    fn contains_self(&self) -> bool {
        match &self.signature {
            Signature::Yielding(yielded) => yielded.contains_self(),
            Signature::Taking(inputs, yielded) => {
                inputs.iter().any(SelfContaining::contains_self) || yielded.contains_self()
            }
        }
    }
}

impl Emitting for KindDeclaration {
    fn emit(&self, scope: &Scope) -> TokenStream {
        let KindContents {
            superkinds,
            types,
            constants,
            capabilities,
        } = self.contents();
        let inner = Scope {
            file: scope.file,
            identity: Some(&self.identity),
            associated: types,
        };
        let name = self.identity.name.tokens();
        let parameters = self.identity.parameters(&inner);
        let extends = if superkinds.is_empty() {
            TokenStream::new()
        } else {
            let bounds = superkinds.bounds(&inner);
            quote! { : #bounds }
        };
        let mut items = Vec::new();
        for associated in types {
            items.push(associated.emit(&inner));
        }
        for constant in constants {
            items.push(constant.emit(&inner));
        }
        for capability in capabilities {
            items.push(capability.emit(&inner));
        }
        quote! {
            pub trait #name #parameters #extends { #( #items )* }
        }
    }
}

// ---------------------------------------------------------------------------
// Associations: compile-time assertions
// ---------------------------------------------------------------------------

impl Emitting for Association {
    fn emit(&self, scope: &Scope) -> TokenStream {
        let inner = Scope {
            file: scope.file,
            identity: Some(&self.identity),
            associated: scope.associated,
        };
        let subject = Reference {
            source: None,
            name: self.identity.name.clone(),
            arguments: vec![],
        };
        let ty = subject.emit(scope);
        let arguments = self.identity.arguments(&inner);
        let parameters = self.identity.parameters(&inner);
        let mut assertions = Vec::with_capacity(self.kinds.len());
        for kind in &self.kinds {
            let assertion = Ident::new(
                &format!(
                    "assert_{}_{}",
                    self.identity.name.0.to_lowercase(),
                    kind.lowered()
                ),
                Span::call_site(),
            );
            let bound = kind.emit(scope);
            if self.identity.constraints.is_empty() {
                assertions.push(quote! {
                    fn #assertion<T: #bound>() {}
                    let _ = #assertion::<#ty>;
                });
            } else {
                assertions.push(quote! {
                    fn #assertion #parameters () {
                        fn assertion<T: #bound>() {}
                        let _ = assertion::<#ty #arguments>;
                    }
                });
            }
        }
        quote! { const _: () = { #( #assertions )* }; }
    }
}

// ---------------------------------------------------------------------------
// The file: its sections walked
// ---------------------------------------------------------------------------

impl Emitting for File {
    fn emit(&self, scope: &Scope) -> TokenStream {
        // prettyplease owns each generated item's canonical text. rustfmt makes
        // different width decisions for aliases and ordinary items, so each
        // generated item is excluded individually while every authored Rust
        // item remains checked by the repository formatter. A type declaration
        // may emit several items (its inline payloads before it), so each
        // struct, enum and alias bears its own skip; kinds and associations
        // are one item each and receive it here.
        let mut items = Vec::new();
        match self {
            File::Library(library) => {
                for declaration in &library.types {
                    items.push(declaration.emit(scope));
                }
                for declaration in &library.kinds {
                    let item = declaration.emit(scope);
                    items.push(quote! { #[rustfmt::skip] #item });
                }
                for association in &library.associations {
                    let item = association.emit(scope);
                    items.push(quote! { #[rustfmt::skip] #item });
                }
            }
            File::Signal(signal) => {
                for declaration in &signal.types {
                    items.push(declaration.emit(scope));
                }
                let query = TypeDeclaration::Enum(
                    Identity {
                        name: Name::try_from("Query").expect("static identifier"),
                        constraints: vec![],
                    },
                    signal.queries.clone(),
                );
                let response = TypeDeclaration::Enum(
                    Identity {
                        name: Name::try_from("Response").expect("static identifier"),
                        constraints: vec![],
                    },
                    signal.responses.clone(),
                );
                if !signal.queries.is_empty() {
                    items.push(query.emit(scope));
                }
                if !signal.responses.is_empty() {
                    items.push(response.emit(scope));
                }
            }
            File::Sema(sema) => {
                for declaration in &sema.types {
                    items.push(declaration.emit(scope));
                }
            }
        }
        quote! {
            #![allow(dead_code, non_camel_case_types, non_snake_case)]
            #( #items )*
        }
    }
}

/// `#[rustfmt::skip]` on an item does not stop rustfmt reformatting the
/// item's other attributes, so a derive list that prettyplease breaks across
/// lines and rustfmt would rejoin makes the two formatters disagree and the
/// repository's `fmt` gate fail on generated text. The derive list is written
/// on one line here, which is the form rustfmt wants.
trait DeriveCollapsing {
    fn collapse_derives(&self) -> String;
}

impl DeriveCollapsing for String {
    fn collapse_derives(&self) -> String {
        let mut out = String::with_capacity(self.len());
        let mut lines = self.lines().peekable();
        while let Some(line) = lines.next() {
            if line.trim_end() == "#[derive(" {
                let indent = &line[..line.len() - line.trim_start().len()];
                let mut names: Vec<&str> = Vec::new();
                for inner in lines.by_ref() {
                    let trimmed = inner.trim();
                    if trimmed == ")]" {
                        break;
                    }
                    names.push(trimmed.trim_end_matches(','));
                }
                out.push_str(indent);
                out.push_str("#[derive(");
                out.push_str(&names.join(", "));
                out.push_str(")]\n");
            } else {
                out.push_str(line);
                out.push('\n');
            }
        }
        out
    }
}

impl Generating for File {
    fn generate(&self) -> Result<String, crate::Error> {
        let scope = Scope {
            file: self,
            identity: None,
            associated: &[],
        };
        self.check(&scope)?;
        let tokens = self.emit(&scope);
        let file: syn::File = syn::parse2(tokens).expect("generated tokens are a Rust file");
        Ok(prettyplease::unparse(&file).collapse_derives())
    }
}
