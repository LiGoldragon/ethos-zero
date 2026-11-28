//! Generation: File to Rust text (cannot err, the file having been checked).
//!
//! Each declaration emits itself: a struct declaration its struct and
//! its datomic machinery, an enum declaration its enum and its
//! machinery, a trait declaration its trait, an association its
//! assertion; the file emits by walking its variant's sections. Names
//! are resolved through the scope, never a table; the generated Rust
//! carries no `use` and writes every foreign name fully qualified, each path
//! into another crate from the extern prelude (`::core::…`, `::flow::…`), so
//! no name the file declares, an open enum's module among them, shadows it.
//!
//! One rule decides boxing: a position is boxed where it closes a by-value
//! cycle, that is where it names, through aliases, `Option` and `Result` but
//! not through `Vector`, a type declared no later than its owner that reaches
//! the owner by value. The box sits immediately around the recursive
//! argument (`::core::option::Option<::std::boxed::Box<Tree>>`).
//!
//! One rule decides archive bounds: a position that reaches its owner by any
//! path, `Vector` included, omits its rkyv bounds, and its type
//! states the serializer, deserializer and validator bounds once instead, so
//! a recursive type archives and restores.

use datom_codec::Integer;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use crate::checking::{Carrying, Declaring, Inlining, Spanning, Streaming, Titled, Uniting};
use crate::reach::{Bearing, Reach, Reaching as _, Reckoning};
use crate::sectioning::{Hoisting, Referencing, ReferencingEach, Sectioning};
use crate::signature::{Place, Stance, Standing};
use crate::{
    Actualizing, AssociatedConstant, AssociatedType, Association, Capability, Constraint, File,
    Generating, Identity, Intrinsic, Name, NamedReference, Position, Potential, Receiver,
    Reference, ReferenceAccess, Resolution, Resolving, Scope, Signature, Source, TraitBody,
    TraitDeclaration, TypeDeclaration, Variant,
};

// ---------------------------------------------------------------------------
// Tokens of names and sources
// ---------------------------------------------------------------------------

/// The trait whose capability yields a value's Rust tokens without any scope.
pub(crate) trait Tokening {
    fn tokens(&self) -> TokenStream;
}

impl Tokening for Name {
    fn tokens(&self) -> TokenStream {
        let ident = Ident::new(&self.0, Span::call_site());
        quote! { #ident }
    }
}

/// A source is written from the extern prelude, `::flow::library`, so a
/// module the file declares under the same name, an open enum's, never
/// shadows it; a source headed by `crate`, `super` or `self` is a path of the
/// including crate, which no declared name can shadow, and is written as it is.
impl Tokening for Source {
    fn tokens(&self) -> TokenStream {
        let mut path: syn::Path =
            syn::parse_str(self.as_ref()).expect("a source was validated as a path");
        // `crate`, `super` and `self` are the keywords a path segment may be;
        // a keyword is no identifier to syn.
        let relative = path
            .segments
            .first()
            .is_some_and(|head| syn::parse_str::<Ident>(&head.ident.to_string()).is_err());
        if !relative {
            path.leading_colon = Some(Default::default());
        }
        quote! { #path }
    }
}

impl Tokening for Intrinsic {
    fn tokens(&self) -> TokenStream {
        match self {
            Intrinsic::String => quote! { ::std::string::String },
            // Generated contracts must compile without their optional `datom`
            // feature.  The wire-level integer is therefore the ordinary
            // Rust scalar; Datom derives know how to compose it when enabled.
            Intrinsic::Integer => quote! { i64 },
            // A datom decimal is finite and point-mandatory. `f64` is neither,
            // so it has no datom text; `::datom_codec::Decimal` admits only a
            // finite value, which is also what makes a type reaching one Eq
            // and Hash.
            Intrinsic::Decimal => quote! { ::datom_codec::Decimal },
            Intrinsic::Boolean => quote! { bool },
            Intrinsic::Meaning => quote! { ::datom_codec::Meaning },
            Intrinsic::Vector => quote! { ::std::vec::Vec },
            Intrinsic::Option => quote! { ::core::option::Option },
            Intrinsic::Result => quote! { ::core::result::Result },
            Intrinsic::Stream => quote! { ::signal::Stream },
            Intrinsic::Itself => quote! { Self },
            Intrinsic::Sized => quote! { Sized },
        }
    }
}

/// The trait whose capability yields the Rust type a position holds of an
/// intrinsic: a stream's position, the whole payload of a Signal's output,
/// holds one frame of it, `::signal::Frame`, which archives as every position
/// does, and each frame on the wire is one output; the live
/// `::signal::Stream` is never archived. Every other intrinsic is held as it
/// is written.
trait Holding {
    fn held(&self) -> TokenStream;
}

impl Holding for Intrinsic {
    fn held(&self) -> TokenStream {
        match self {
            Intrinsic::Stream => quote! { ::signal::Frame },
            Intrinsic::String
            | Intrinsic::Integer
            | Intrinsic::Decimal
            | Intrinsic::Boolean
            | Intrinsic::Meaning
            | Intrinsic::Vector
            | Intrinsic::Option
            | Intrinsic::Result
            | Intrinsic::Itself
            | Intrinsic::Sized => self.tokens(),
        }
    }
}

/// The trait whose capability yields the name of the parameter at an index: A, B, C.
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

/// The trait whose capability lowercases a name for an assertion function.
trait Lowering {
    fn lowered(&self) -> String;
}

impl Lowering for Reference {
    fn lowered(&self) -> String {
        let mut lowered = self.title().to_lowercase();
        for argument in self.arguments() {
            lowered.push('_');
            lowered.push_str(&argument.lowered());
        }
        lowered
    }
}

// ---------------------------------------------------------------------------
// Emitting: Rust tokens in a scope
// ---------------------------------------------------------------------------

/// The trait whose capability yields a value's Rust tokens in a scope.
pub(crate) trait Emitting {
    fn emit(&self, scope: &Scope) -> TokenStream;
}

impl Emitting for Reference {
    fn emit(&self, scope: &Scope) -> TokenStream {
        let reference = match self {
            Self::Vector(element) => {
                let vector = Intrinsic::Vector.tokens();
                let element = element.emit(scope);
                return quote! { #vector <#element> };
            }
            Self::Named(reference) => reference,
        };
        let mut arguments = Vec::with_capacity(reference.arguments.len());
        for argument in &reference.arguments {
            arguments.push(argument.emit(scope));
        }
        let applied = if arguments.is_empty() {
            TokenStream::new()
        } else {
            quote! { < #( #arguments ),* > }
        };
        let name = reference.name.tokens();
        if let Some(source) = &reference.source {
            let source = source.tokens();
            return quote! { #source :: #name #applied };
        }
        match scope.resolve(&reference.name) {
            Resolution::Intrinsic(intrinsic) => {
                let intrinsic = intrinsic.tokens();
                quote! { #intrinsic #applied }
            }
            Resolution::Imported(source, emitted) => {
                if source.as_ref() == "std" {
                    if emitted.0 == "Clone" {
                        return quote! { ::core::clone::Clone #applied };
                    }
                    if emitted.0 == "Send" {
                        return quote! { ::core::marker::Send #applied };
                    }
                }
                let source = source.tokens();
                let emitted = emitted.tokens();
                quote! { #source :: #emitted #applied }
            }
            Resolution::Type(_)
            | Resolution::Trait(_)
            | Resolution::Ambiguous(_)
            | Resolution::AmbiguousImport(_)
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

/// The trait whose capability yields the bounds of a constraint: `A + B`.
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

/// The trait whose capabilities yield an identity's generics: the parameters with their bounds, and the arguments.
pub(crate) trait Parametrizing {
    fn parameters(&self, scope: &Scope) -> TokenStream;
    fn arguments(&self, scope: &Scope) -> TokenStream;
}

impl Parametrizing for Identity {
    fn parameters(&self, scope: &Scope) -> TokenStream {
        if self.constraints.is_empty() {
            return TokenStream::new();
        }
        // The bounds name traits outside the identity they bound.
        let outer = Scope {
            file: scope.file,
            identity: None,
            associated: scope.associated,
            namespaces: scope.namespaces,
            reach: scope.reach,
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

/// The trait whose capability tells whether a value holds the target type, through declared types, aliases, Option and Result, and Vector when the passage allows it.
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
        let reference = match self {
            Self::Vector(element) => {
                return passage == Passage::Any && element.reaches(target, passage, file, visited);
            }
            Self::Named(reference) => reference,
        };
        if reference.source.is_none() {
            if &reference.name == target || reference.name.0 == "Self" {
                return true;
            }
            match file.resolve(&reference.name) {
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
        for argument in &reference.arguments {
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

impl Reaching for [Position] {
    fn reaches(
        &self,
        target: &Name,
        passage: Passage,
        file: &File,
        visited: &mut Vec<Name>,
    ) -> bool {
        for position in self {
            if position.reference().reaches(target, passage, file, visited) {
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

/// The trait whose capability yields the reference a bare variant naming a declared type carries.
trait Carried {
    fn carried(&self) -> Reference;
}

impl Carried for Name {
    fn carried(&self) -> Reference {
        Reference::Named(NamedReference {
            source: None,
            name: self.clone(),
            arguments: vec![],
        })
    }
}

/// The trait whose capability gives a declared type's place in its file's declaration order.
trait Placing {
    fn place(&self, name: &Name) -> Option<usize>;
}

impl Placing for File {
    fn place(&self, name: &Name) -> Option<usize> {
        self.declarations()
            .iter()
            .position(|placed| placed.declaration.resolve(name) != Resolution::Undeclared)
    }
}

/// The trait whose capability tells whether a reference closes a by-value
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
        let Some(reference) = self.named() else {
            return false;
        };
        if reference.source.is_some() {
            return false;
        }
        if &reference.name == owner || reference.name.0 == "Self" {
            return true;
        }
        match file.resolve(&reference.name) {
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
        for argument in &reference.arguments {
            if argument.closes(owner, file) {
                return true;
            }
        }
        false
    }
}

/// The trait whose capabilities yield a position's Rust type, boxed where it
/// closes a by-value cycle, and the archive attribute it bears.
pub(crate) trait Positioning {
    fn boxed(&self, scope: &Scope, owner: &Name) -> bool;
    fn recursive(&self, scope: &Scope, owner: &Name) -> bool;
    fn position(&self, scope: &Scope, owner: &Name) -> TokenStream;
    fn archival(&self, scope: &Scope, owner: &Name) -> TokenStream;
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
        if let Some(reference) = self.named()
            && reference.source.is_none()
            && let Resolution::Intrinsic(
                intrinsic @ (Intrinsic::Option | Intrinsic::Result | Intrinsic::Stream),
            ) = scope.file.resolve(&reference.name)
        {
            let name = intrinsic.held();
            let mut arguments = Vec::with_capacity(reference.arguments.len());
            for argument in &reference.arguments {
                let ty = if argument.boxed(scope, owner) {
                    let inner = argument.emit(scope);
                    quote! { ::std::boxed::Box<#inner> }
                } else {
                    argument.emit(scope)
                };
                arguments.push(ty);
            }
            return quote! { #name < #( #arguments ),* > };
        }

        let ty = self.emit(scope);
        if self.boxed(scope, owner) {
            quote! { ::std::boxed::Box<#ty> }
        } else {
            ty
        }
    }

    // rkyv's derive bounds every field's type by the trait it derives, so a
    // position that reaches its owner, through a Vector as much as a Box,
    // makes the bound depend on itself and the trait solver overflows. Such
    // a position omits its bound; its type then states the bounds its
    // containers need instead (`Recursing`).
    fn archival(&self, scope: &Scope, owner: &Name) -> TokenStream {
        if self.recursive(scope, owner) {
            quote! { #[rkyv(omit_bounds)] }
        } else {
            TokenStream::new()
        }
    }
}

/// The trait whose capability yields the archive bounds a declared type
/// states once some position of it omits its own: what `Box` and `Vec` ask
/// of the serializer, the deserializer and the validator, named once for
/// the type.
trait Recursing {
    fn recursion_bounds(&self) -> TokenStream;
}

impl Recursing for Identity {
    fn recursion_bounds(&self) -> TokenStream {
        quote! {
            #[rkyv(serialize_bounds(__S: ::rkyv::ser::Writer + ::rkyv::ser::Allocator, __S::Error: ::rkyv::rancor::Source))]
            #[rkyv(deserialize_bounds(__D::Error: ::rkyv::rancor::Source))]
            #[rkyv(bytecheck(bounds(__C: ::rkyv::validation::ArchiveContext, __C::Error: ::rkyv::rancor::Source)))]
        }
    }
}

/// The trait whose capability yields the stem a position's field name is built from.
trait Fielding {
    fn field_base(&self, owner: &Name) -> String;
}

pub(crate) trait SnakeCasing {
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
            .arguments()
            .iter()
            .map(|argument| argument.field_base(owner))
            .collect();
        let name = match self.named() {
            // The field takes the name of the type that encloses Self.
            Some(named) if named.source.is_none() && named.name.0 == "Self" => owner.as_ref(),
            _ => self.title(),
        };
        parts.push(name.snake_case());
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

/// The trait whose capability spells a snake-case name as a Rust identifier:
/// as it is where it is one, raw where it is a keyword, and with a trailing
/// underscore where it is one of the four keywords no raw identifier may spell.
pub(crate) trait Identifying {
    fn identifier(&self) -> Ident;
}

impl Identifying for str {
    fn identifier(&self) -> Ident {
        if syn::parse_str::<Ident>(self).is_ok() {
            Ident::new(self, Span::call_site())
        } else if matches!(self, "crate" | "self" | "super" | "Self") {
            Ident::new(&format!("{self}_"), Span::call_site())
        } else {
            Ident::new_raw(self, Span::call_site())
        }
    }
}

/// The trait whose capability names every field of a struct of these positions.
pub(crate) trait FieldNaming {
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
            fields.push(name.identifier());
        }
        fields
    }
}

/// The trait whose capabilities yield a variant's definition and the items its inline enum needs.
trait Varianted {
    fn definition(&self, scope: &Scope, owner: &Name, enclosing: &Identity) -> TokenStream;
    fn recursive(&self, scope: &Scope, owner: &Name) -> bool;
    fn nested(&self, scope: &Scope, owner: &Name, enclosing: &Identity) -> TokenStream;
}

/// The trait whose capability yields the identity of the payload a variant
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
    fn definition(&self, scope: &Scope, owner: &Name, enclosing: &Identity) -> TokenStream {
        match self {
            Variant::Bare(name) => {
                let variant = name.tokens();
                if scope.carries(name) {
                    let carried = name.carried();
                    let archival = carried.archival(scope, owner);
                    let ty = carried.position(scope, owner);
                    quote! { #variant(#archival #ty) }
                } else {
                    quote! { #variant }
                }
            }
            Variant::Typed(name, reference) => {
                let name = name.tokens();
                let archival = reference.archival(scope, owner);
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

    fn nested(&self, scope: &Scope, owner: &Name, enclosing: &Identity) -> TokenStream {
        match self {
            Variant::Struct(name, positions) => {
                let nested = enclosing.nested_identity(scope, owner, name);
                positions.structure(scope, owner, &nested)
            }
            Variant::Enum(name, variants) => {
                let nested = enclosing.nested_identity(scope, owner, name);
                variants.enumeration(scope, owner, &nested)
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

/// The trait whose capability emits a struct of these positions with its datomic machinery.
trait Structuring {
    fn structure(&self, scope: &Scope, owner: &Name, identity: &Identity) -> TokenStream;
}

/// The trait whose capability emits a struct with field names taken from its authored positions.
pub(crate) trait StructFielding {
    fn structure_with_fields(
        &self,
        scope: &Scope,
        owner: &Name,
        identity: &Identity,
        fields: &[Ident],
    ) -> TokenStream;
}

/// The trait whose capability emits an enum of these variants with its datomic machinery.
trait Enumerating {
    fn enumeration(&self, scope: &Scope, owner: &Name, identity: &Identity) -> TokenStream;
}

/// The trait whose capability yields what a declared type derives.
pub(crate) trait DatomDeriving {
    fn datom_derives(&self) -> TokenStream;
    fn scalar_datom_derives(&self, scalar: Scalar) -> TokenStream;
    fn writing_derives(&self) -> TokenStream;
}

/// What every declared type derives, and what none does.
///
/// Every root carries its types alike, a Library's, an Operation's and a
/// Memory's as a Signal's: any of them can cross a wire, so each archives
/// with rkyv, and its datom derives sit behind the `datom` feature that a CLI
/// enables where it textualizes and a Nexus does not, so a Nexus compiles
/// its whole contract without datom-codec.
///
/// `Clone`, `Debug` and `PartialEq` always. `Eq` and `Hash` always too: every
/// intrinsic a position can hold is now `Eq` and `Hash`, `Decimal` included,
/// because a datom decimal is finite. Without them no contract value could be
/// a map key, and the orphan rule leaves a consumer no way to add them.
///
/// `Copy` and the orderings only on a newtype whose underlying scalar
/// supports them (see `Scalaring`): a struct or enum is never `Copy`, since a
/// wire type's size is not part of its contract, and a contract that grew a
/// `String` position would silently break every consumer relying on it.
/// Never `Default`: a default is a policy the consumer holds,
/// not a value the wire carries, and a manufactured zero satisfies the type
/// while violating the schema's invariants.
impl DatomDeriving for Identity {
    fn datom_derives(&self) -> TokenStream {
        quote! {
            #[rustfmt::skip]
            #[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
            #[cfg_attr(feature = "datom", derive(::datom_codec::Datomizable, ::datom_codec::Composing))]
        }
    }

    /// The derives of a type that reaches an open enum: it is written as
    /// any type is, and read only through `Opening`, with the registry of
    /// added variants, so it bears no `Composing` that reads without one.
    fn writing_derives(&self) -> TokenStream {
        quote! {
            #[rustfmt::skip]
            #[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
            #[cfg_attr(feature = "datom", derive(::datom_codec::Datomizable))]
        }
    }

    /// The same derives with the scalar's added: `Copy` after `Clone` where
    /// the scalar is copyable, the orderings after `PartialEq` and `Eq`.
    fn scalar_datom_derives(&self, scalar: Scalar) -> TokenStream {
        match scalar {
            Scalar::Copyable => quote! {
                #[rustfmt::skip]
                #[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
                #[cfg_attr(feature = "datom", derive(::datom_codec::Datomizable, ::datom_codec::Composing))]
            },
            Scalar::Ordered => quote! {
                #[rustfmt::skip]
                #[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
                #[cfg_attr(feature = "datom", derive(::datom_codec::Datomizable, ::datom_codec::Composing))]
            },
        }
    }
}

/// What the scalar under a newtype supports beyond the derives every
/// declared type bears.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Scalar {
    /// A scalar of fixed size and total order: `Integer`, `Boolean`, `Decimal`.
    Copyable,
    /// A scalar of total order and no fixed size: `String`.
    Ordered,
}

/// The trait whose capability yields the scalar under a reference, if any.
///
/// A newtype whose underlying type is a scalar, directly or through other
/// newtypes, derives what that scalar supports: `Copyable` gives `Copy`,
/// `PartialOrd` and `Ord`; `Ordered` gives `PartialOrd` and `Ord`. A type
/// whose underlying type is no scalar, or whose capability is not
/// established (`Meaning`, a container, a generic parameter, an imported
/// name), derives neither. No association is written; the scalar implies the
/// traits. Arithmetic is never derived: a capability that mints or combines
/// values is declared on the type through a trait with a handwritten body.
trait Scalaring {
    fn scalar(&self, scope: &Scope) -> Option<Scalar>;
}

impl Scalaring for Reference {
    fn scalar(&self, scope: &Scope) -> Option<Scalar> {
        let mut reference = self.named()?;
        let mut resolution = scope.resolve(&reference.name);
        // The chain of newtypes ends at the file's declarations, so it is
        // bounded by their count.
        for _ in 0..=scope.file.declarations().len() {
            if reference.source.is_some() || !reference.arguments.is_empty() {
                return None;
            }
            match resolution {
                Resolution::Intrinsic(
                    Intrinsic::Integer | Intrinsic::Boolean | Intrinsic::Decimal,
                ) => return Some(Scalar::Copyable),
                Resolution::Intrinsic(Intrinsic::String) => return Some(Scalar::Ordered),
                Resolution::Type(name) => match scope.file.declaration(&name) {
                    Some(TypeDeclaration::Alias(identity, aliased))
                        if identity.constraints.is_empty() =>
                    {
                        reference = aliased.named()?;
                        resolution = scope.file.resolve(&reference.name);
                    }
                    _ => return None,
                },
                _ => return None,
            }
        }
        None
    }
}

/// A type declared in a position is emitted before the struct that holds
/// it, as a declaration of the file outside the enclosing identity; the
/// position then holds it by name.
impl Structuring for [Position] {
    fn structure(&self, scope: &Scope, owner: &Name, identity: &Identity) -> TokenStream {
        let outer = Scope {
            file: scope.file,
            identity: None,
            associated: &[],
            namespaces: scope.namespaces,
            reach: scope.reach,
        };
        let mut declared = Vec::new();
        for position in self {
            if let Position::Declared(declaration) = position {
                declared.push(declaration.emit(&outer));
            }
        }
        let references = self.references();
        let fields = references.field_names(owner);
        let structure = references.structure_with_fields(scope, owner, identity, &fields);
        quote! { #( #declared )* #structure }
    }
}

impl StructFielding for [Reference] {
    fn structure_with_fields(
        &self,
        scope: &Scope,
        owner: &Name,
        identity: &Identity,
        fields: &[Ident],
    ) -> TokenStream {
        let name = identity.name.tokens();
        let parameters = identity.parameters(scope);
        let mut types = Vec::with_capacity(self.len());
        let mut archivals = Vec::with_capacity(self.len());
        let mut recursive = false;
        for position in self {
            types.push(position.position(scope, owner));
            archivals.push(position.archival(scope, owner));
            recursive |= position.recursive(scope, owner);
        }
        let opens = scope.reach.opens_held(self);
        let (derive, opening) = if opens.is_empty() || !identity.constraints.is_empty() {
            (identity.datom_derives(), identity.plain_open(scope))
        } else {
            (
                identity.writing_derives(),
                identity.open_impl(&opens, self.struct_reading(scope.reach, fields)),
            )
        };
        let bounds = if recursive {
            identity.recursion_bounds()
        } else {
            TokenStream::new()
        };
        quote! {
            #derive
            #bounds
            pub struct #name #parameters { #( #archivals pub #fields: #types ),* }
            #opening
        }
    }
}

impl Enumerating for [Variant] {
    fn enumeration(&self, scope: &Scope, owner: &Name, identity: &Identity) -> TokenStream {
        let name = identity.name.tokens();
        let parameters = identity.parameters(scope);
        let opens = scope.reach.opens_held(self);
        let (derive, opening) = if opens.is_empty() || !identity.constraints.is_empty() {
            (identity.datom_derives(), identity.plain_open(scope))
        } else {
            (
                identity.writing_derives(),
                identity.open_impl(&opens, self.enum_reading(scope, identity)),
            )
        };
        let mut definitions = Vec::with_capacity(self.len());
        let mut nested = Vec::new();
        let mut recursive = false;
        for variant in self {
            definitions.push(variant.definition(scope, owner, identity));
            nested.push(variant.nested(scope, owner, identity));
            recursive |= variant.recursive(scope, owner);
        }
        let bounds = if recursive {
            identity.recursion_bounds()
        } else {
            TokenStream::new()
        };
        quote! {
            #( #nested )*
            #derive
            #bounds
            pub enum #name #parameters { #( #definitions ),* }
            #opening
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
                    namespaces: scope.namespaces,
                    reach: scope.reach,
                };
                positions.structure(&inner, &identity.name, identity)
            }
            TypeDeclaration::Enum(identity, variants) => {
                let inner = Scope {
                    file: scope.file,
                    identity: Some(identity),
                    associated: scope.associated,
                    namespaces: scope.namespaces,
                    reach: scope.reach,
                };
                variants.enumeration(&inner, &identity.name, identity)
            }
            TypeDeclaration::Alias(identity, aliased) => {
                let inner = Scope {
                    file: scope.file,
                    identity: Some(identity),
                    associated: scope.associated,
                    namespaces: scope.namespaces,
                    reach: scope.reach,
                };
                let name = identity.name.tokens();
                let parameters = identity.parameters(&inner);
                let aliased_reference = aliased;
                let aliased = aliased.emit(&inner);
                let opens = scope.reach.opens_held(aliased_reference);
                let (derive, opening) = match aliased_reference.scalar(&inner) {
                    Some(scalar) => (
                        identity.scalar_datom_derives(scalar),
                        identity.plain_open(scope),
                    ),
                    None if opens.is_empty() || !identity.constraints.is_empty() => {
                        (identity.datom_derives(), identity.plain_open(scope))
                    }
                    None => (
                        identity.writing_derives(),
                        identity.open_impl(
                            &opens,
                            quote! { ::core::result::Result::Ok(Self(Opening::open(datom, budget, registry)?)) },
                        ),
                    ),
                };
                quote! { #derive pub struct #name #parameters (pub #aliased); #opening }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Traits: traits
// ---------------------------------------------------------------------------

/// The named sections that compose a trait declaration.
struct TraitContents<'a> {
    supertraits: &'a [Reference],
    types: &'a [AssociatedType],
    constants: &'a [AssociatedConstant],
    capabilities: &'a [Capability],
}

/// The trait whose capability exposes a declaration's named trait sections.
trait Containing {
    fn contents(&self) -> TraitContents<'_>;
}

impl Containing for TraitDeclaration {
    fn contents(&self) -> TraitContents<'_> {
        match &self.body {
            TraitBody::Simple(capabilities) => TraitContents {
                supertraits: &[],
                types: &[],
                constants: &[],
                capabilities,
            },
            TraitBody::Complex {
                supertraits,
                types,
                constants,
                capabilities,
            } => TraitContents {
                supertraits,
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

/// The method's own parameters: one per trait its signature names, lettered
/// from N past the trait's head parameters.
struct MethodParameters {
    first: usize,
    declared: Vec<TokenStream>,
}

/// The trait whose capability yields a signature reference's Rust type,
/// taking a method parameter where the reference names a trait.
trait Signing {
    fn sign(&self, scope: &Scope, place: Place, method: &mut MethodParameters) -> TokenStream;
}

impl Signing for Reference {
    fn sign(&self, scope: &Scope, place: Place, method: &mut MethodParameters) -> TokenStream {
        match self.stance(scope, place) {
            Stance::Bounding => {
                let letter = (method.first + method.declared.len()).letter(scope);
                let bound = self.emit(scope);
                method.declared.push(quote! { #letter: #bound });
                quote! { #letter }
            }
            Stance::Bound(associated) => {
                let associated = associated.tokens();
                quote! { Self::#associated }
            }
            Stance::Kept | Stance::Concrete => self.emit(scope),
        }
    }
}

impl Emitting for Capability {
    fn emit(&self, scope: &Scope) -> TokenStream {
        let name = self.name.tokens();
        let heads = scope
            .identity
            .map_or(0, |identity| identity.constraints.len());
        let mut method = MethodParameters {
            first: heads.max(13),
            declared: Vec::new(),
        };
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
                    let ty = input.sign(scope, Place::Input, &mut method);
                    parameters.push(quote! { input: #ty });
                } else {
                    for (index, input) in inputs.iter().enumerate() {
                        let input_name = Ident::new(&format!("input_{index}"), Span::call_site());
                        let ty = input.sign(scope, Place::Input, &mut method);
                        parameters.push(quote! { #input_name: #ty });
                    }
                }
                yields
            }
        };
        // An empty yield returns nothing: the method is written without `->`.
        let yields = yields.as_ref().map(|yields| {
            let yields = yields.sign(scope, Place::Yield, &mut method);
            quote! { -> #yields }
        });
        let generics = if method.declared.is_empty() {
            TokenStream::new()
        } else {
            let declared = &method.declared;
            quote! { < #( #declared ),* > }
        };
        let sized = if self.contains_self() {
            quote! { where Self: Sized }
        } else {
            TokenStream::new()
        };
        quote! { fn #name #generics ( #( #parameters ),* ) #yields #sized; }
    }
}

/// Whether a capability signature mentions `Self`, which requires a sized
/// trait receiver when lowered into Rust's sized generic constructors.
trait SelfContaining {
    fn contains_self(&self) -> bool;
}

impl SelfContaining for Reference {
    fn contains_self(&self) -> bool {
        self.named()
            .is_some_and(|reference| reference.source.is_none() && reference.name.0 == "Self")
            || self.arguments().iter().any(SelfContaining::contains_self)
    }
}

impl SelfContaining for Capability {
    fn contains_self(&self) -> bool {
        match &self.signature {
            Signature::Yielding(yielded) => yielded.iter().any(SelfContaining::contains_self),
            Signature::Taking(inputs, yielded) => inputs
                .iter()
                .chain(yielded)
                .any(SelfContaining::contains_self),
        }
    }
}

impl Emitting for TraitDeclaration {
    fn emit(&self, scope: &Scope) -> TokenStream {
        let TraitContents {
            supertraits,
            types,
            constants,
            capabilities,
        } = self.contents();
        let inner = Scope {
            file: scope.file,
            identity: Some(&self.identity),
            associated: types,
            namespaces: scope.namespaces,
            reach: scope.reach,
        };
        let name = self.identity.name.tokens();
        let parameters = self.identity.parameters(&inner);
        let extends = if supertraits.is_empty() {
            TokenStream::new()
        } else {
            let bounds = supertraits.bounds(&inner);
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
            namespaces: scope.namespaces,
            reach: scope.reach,
        };
        let subject = Reference::Named(NamedReference {
            source: None,
            name: self.identity.name.clone(),
            arguments: vec![],
        });
        let ty = subject.emit(scope);
        let arguments = self.identity.arguments(&inner);
        let parameters = self.identity.parameters(&inner);
        let mut assertions = Vec::with_capacity(self.traits.len());
        for trait_definition in &self.traits {
            let assertion = Ident::new(
                &format!(
                    "assert_{}_{}",
                    self.identity.name.0.to_lowercase(),
                    trait_definition.lowered()
                ),
                Span::call_site(),
            );
            let bound = trait_definition.emit(scope);
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

/// The trait whose capability yields the items a file emits, before the
/// header they are framed in: a Nexus adds the items it reads from its other
/// roots to a Process file's own.
pub(crate) trait Itemizing {
    fn items(&self, scope: &Scope) -> Vec<TokenStream>;
}

/// The trait whose capability writes a file's items under its header, and
/// formats them.
pub(crate) trait Rendering {
    fn render(&self) -> String;
}

impl Itemizing for File {
    fn items(&self, scope: &Scope) -> Vec<TokenStream> {
        // prettyplease owns each generated item's canonical text. rustfmt makes
        // different width decisions for aliases and ordinary items, so each
        // generated item is excluded individually while every authored Rust
        // item remains checked by the repository formatter. A type declaration
        // may emit several items (its inline payloads before it), so each
        // struct, enum and alias bears its own skip; traits and associations
        // are one item each and receive it here.
        let mut items = Vec::new();
        match self {
            File::Library(library) => {
                for declaration in &library.types {
                    items.push(declaration.emit(scope));
                }
                for declaration in &library.traits {
                    let item = declaration.emit(scope);
                    items.push(quote! { #[rustfmt::skip] #item });
                }
                for association in &library.associations {
                    let item = association.emit(scope);
                    items.push(quote! { #[rustfmt::skip] #item });
                }
            }
            File::Signal(_)
            | File::Operation(_)
            | File::Memory(_)
            | File::Matrix(_)
            | File::Emission(_)
            | File::Process(_)
            | File::Organism(_) => {
                for declaration in self.listed_declarations() {
                    items.push(declaration.emit(scope));
                }
                // A Signal's open enums: what each is, its contract, and how it is read.
                items.extend(self.open_items(scope));
                // A Matrix's `Memory`: one field per record.
                if let Some(memory) = self.matrix_memory() {
                    items.push(memory.emit(scope));
                }
                // The unit each bare emission is, a struct of no fields: it
                // bears Emitted, and its variant in the enum carries nothing,
                // so its datom is written bare.
                for unit in self.units() {
                    let unit = TypeDeclaration::Struct(
                        Identity {
                            name: unit.name,
                            constraints: vec![],
                        },
                        vec![],
                    );
                    items.push(unit.emit(scope));
                }
                // A Signal's `Input` and `Output`, an Operation's
                // `Operation` and `Outcome`: each an enum of its section's
                // variants, emitted when it has any.
                for implied in self.implied_enums() {
                    // A Signal's open enums each add one variant, carrying the
                    // contract module's own enum of the same name.
                    let mut variants = implied.variants.to_vec();
                    variants.extend(self.opened().contract_variants(&implied.name));
                    if variants.is_empty() {
                        continue;
                    }
                    let enumeration = TypeDeclaration::Enum(
                        Identity {
                            name: implied.name,
                            constraints: vec![],
                        },
                        variants,
                    );
                    items.push(enumeration.emit(scope));
                }
                let implied = match self {
                    File::Signal(signal) => signal.implied_traits(scope),
                    File::Emission(emission) => emission.implied_traits(scope),
                    File::Matrix(matrix) => matrix.implied_traits(scope),
                    File::Library(_)
                    | File::Operation(_)
                    | File::Memory(_)
                    | File::Process(_)
                    | File::Organism(_) => vec![],
                };
                for item in implied {
                    items.push(quote! { #[rustfmt::skip] #item });
                }
            }
        }
        items
    }
}

impl Rendering for [TokenStream] {
    fn render(&self) -> String {
        let items = self;
        let tokens = quote! {
            #![allow(dead_code, non_camel_case_types, non_snake_case, clippy::type_complexity)]
            // Every crate path is written from the extern prelude, `::core::…` and
            // `::std::…`, so no declared name shadows it and the module needs no
            // header: an open enum `Alloc` or `Std` is a module like any other.
            #( #items )*
        };
        let file: syn::File = syn::parse2(tokens).expect("generated tokens are a Rust file");
        prettyplease::unparse(&file)
            .collapse_derives()
            .format_allow()
            .space_colons()
    }
}

// ---------------------------------------------------------------------------
// Root-implied traits: the pairing of requests and replies by position
// ---------------------------------------------------------------------------

/// The trait whose capability yields a section's variants as the types they
/// bind: a bare emission that is a unit of its own ([`Uniting`]) binds the
/// unit by name, as a variant naming a type binds it, and every other variant
/// is as written. The implied enum itself holds the variants as written, so a
/// unit's variant carries nothing.
trait United {
    fn united(&self, scope: &Scope, section: Integer) -> Vec<Variant>;
}

impl United for [Variant] {
    fn united(&self, scope: &Scope, section: Integer) -> Vec<Variant> {
        let units = scope.file.units();
        let mut variants = Vec::with_capacity(self.len());
        let mut at = 0;
        for variant in self {
            let unit = units.iter().any(|unit| unit.path == [section, at]);
            variants.push(match variant {
                Variant::Bare(name) if unit => Variant::Typed(
                    name.clone(),
                    Reference::Named(NamedReference {
                        source: None,
                        name: name.clone(),
                        arguments: vec![],
                    }),
                ),
                Variant::Bare(_)
                | Variant::Typed(_, _)
                | Variant::Struct(_, _)
                | Variant::Enum(_, _) => variant.clone(),
            });
            at += variant.span();
        }
        variants
    }
}

/// The trait whose capability yields the Rust type a variant of an implied
/// enum carries, as the variant's field holds it, or none where it carries
/// nothing.
trait Borne {
    fn borne(&self, scope: &Scope, owner: &Name) -> Option<TokenStream>;
}

impl Borne for Variant {
    fn borne(&self, scope: &Scope, owner: &Name) -> Option<TokenStream> {
        match self {
            Variant::Bare(name) if scope.carries(name) => {
                Some(name.carried().position(scope, owner))
            }
            Variant::Bare(_) => None,
            Variant::Typed(_, reference) => Some(reference.owned(owner).position(scope, owner)),
            Variant::Struct(name, _) | Variant::Enum(name, _) => {
                Some(scope.file.inline_name(owner, owner, name).tokens())
            }
        }
    }
}

/// The trait whose capability names the input a capability takes for a
/// variant: for the type the variant carries, as a field of it is named, and
/// for a payload declared in place, for the variant. A variant that carries
/// nothing takes no input.
trait InputNaming {
    fn input_name(&self, scope: &Scope, owner: &Name) -> Option<String>;
}

impl InputNaming for Variant {
    fn input_name(&self, scope: &Scope, owner: &Name) -> Option<String> {
        match self {
            Variant::Bare(name) if scope.carries(name) => Some(name.0.snake_case()),
            Variant::Bare(_) => None,
            Variant::Typed(_, reference) => Some(reference.owned(owner).field_base(owner)),
            Variant::Struct(name, _) | Variant::Enum(name, _) => Some(name.0.snake_case()),
        }
    }
}

/// The trait whose capability writes `Self` in a variant's reference as the
/// enum it stands for, so the type reads the same outside that enum: in a
/// trait or an impl, `Self` would name something else.
trait Owning {
    fn owned(&self, owner: &Name) -> Reference;
}

impl Owning for Reference {
    fn owned(&self, owner: &Name) -> Reference {
        let reference = match self {
            Self::Vector(element) => return Self::Vector(Box::new(element.owned(owner))),
            Self::Named(reference) => reference,
        };
        let mut arguments = Vec::with_capacity(reference.arguments.len());
        for argument in &reference.arguments {
            arguments.push(argument.owned(owner));
        }
        let name = if reference.source.is_none() && reference.name.0 == "Self" {
            owner.clone()
        } else {
            reference.name.clone()
        };
        Reference::Named(NamedReference {
            source: reference.source.clone(),
            name,
            arguments,
        })
    }
}

/// The trait whose capability emits the traits a root implies on what it
/// declares, beside its implied enums: the items, each to be skipped by
/// rustfmt.
trait TraitImplying {
    fn implied_traits(&self, scope: &Scope) -> Vec<TokenStream>;
}

/// An Emission's `Emittable`: the translator turns an emission into its call
/// and yields its outcome. The n-th outcome is the n-th emission's, bound by
/// `Emitted` on the type the emission carries; `Emittable` extends
/// `Emitted`, so a hand-written `emit` yields the bound outcome and no other.
/// A payload-less outcome binds `()`; a bare emission carries its unit
/// ([`Uniting`]) where the file names no type of its name, so every emission
/// binds its outcome. No standard library holds the root-implied traits yet,
/// so both traits are generated into the file.
impl TraitImplying for crate::Emission {
    fn implied_traits(&self, scope: &Scope) -> Vec<TokenStream> {
        if self.emissions.is_empty() {
            return vec![];
        }
        let emitted = Name::try_from("Emission").expect("static identifier");
        let observed = Name::try_from("Outcome").expect("static identifier");
        let mut items = vec![
            quote! { pub trait Emitted { type Outcome; } },
            quote! {
                pub trait Emittable<Translator>: Emitted {
                    fn emit(&self, translator: &Translator) -> Self::Outcome;
                }
            },
        ];
        let emissions = self.emissions.united(scope, 1);
        for (emission, outcome) in emissions.iter().zip(&self.outcomes) {
            let Some(carried) = emission.borne(scope, &emitted) else {
                continue;
            };
            let outcome = match outcome.borne(scope, &observed) {
                Some(outcome) => outcome,
                None => quote! { () },
            };
            items.push(quote! { impl Emitted for #carried { type Outcome = #outcome; } });
        }
        items
    }
}

/// A Matrix's `Memorizable`: every record bears it, naming the `Memory` it is
/// held in and recalling itself from that Memory's field for it. The trait
/// says nothing of change. No standard library holds the root-implied traits
/// yet, so the trait is generated into the module of each Matrix file.
impl TraitImplying for crate::Matrix {
    fn implied_traits(&self, scope: &Scope) -> Vec<TokenStream> {
        let Some(TypeDeclaration::Struct(identity, positions)) = scope.file.matrix_memory() else {
            return vec![];
        };
        let references = positions.references();
        // A Matrix of no records generates no trait, as an empty Emission
        // generates none; its empty Memory stays.
        if references.is_empty() {
            return vec![];
        }
        let mut items = vec![quote! {
            pub trait Memorizable { type Memory; fn recall(memory: &Self::Memory) -> &Self; }
        }];
        let memory = identity.name.tokens();
        let fields = references.field_names(&identity.name);
        for (reference, field) in references.iter().zip(fields) {
            let record = reference.position(scope, &identity.name);
            items.push(quote! {
                impl Memorizable for #record {
                    type Memory = #memory;
                    fn recall(memory: &#memory) -> &Self { &memory.#field }
                }
            });
        }
        items
    }
}

/// A Signal's `Answering`, one capability per input, and `Answerable`, the
/// input answered by the bearer of `Answering`. The n-th output answers the
/// n-th input; an output past the last input answers none, a refusal or a
/// reaction. A capability is the input's variant in snake case; it borrows
/// what the input carries, or takes nothing, and yields what its output
/// carries, or `()`, or the refusal: any `Output`, which the dispatch
/// answers whole. The dispatch is the input's: `Input`'s `answer` borrows
/// the bearer and lends each capability the payload it matched, so an input
/// named `Answer` meets no name of the dispatch. Every arm binds the payload
/// as `carried`, a name no capability's parameter shadows the bearer with.
/// No standard library holds the root-implied traits yet, so `Answerable` is
/// generated into the file.
///
/// `Answerable` names what an answer is, its `Answer`. A Signal none of whose
/// inputs is answered by a stream answers each input once, with an `Output`.
/// Where an output answering an input is a `Stream`, its capability yields
/// the live `::signal::Stream` or the refusal, and the file generates `Answer`:
/// `Once(Output)` for every answer given once, a refusal among them, and one
/// variant per stream output, named as that output, holding its stream. The
/// stream is live and never archived, so `Answer` derives nothing.
impl TraitImplying for crate::Signal {
    fn implied_traits(&self, scope: &Scope) -> Vec<TokenStream> {
        let opened = scope.file.opened();
        if self.inputs.is_empty() && opened.is_empty() {
            return vec![];
        }
        let asked = Name::try_from("Input").expect("static identifier");
        let answered = Name::try_from("Output").expect("static identifier");
        let streaming = self
            .outputs
            .iter()
            .take(self.inputs.len())
            .any(|output| output.stream(scope).is_some());
        let mut capabilities = Vec::with_capacity(self.inputs.len());
        let mut arms = Vec::with_capacity(self.inputs.len());
        let mut streams = Vec::new();
        for (input, output) in self.inputs.iter().zip(&self.outputs) {
            let capability = input.title().0.snake_case().identifier();
            let variant = input.title().tokens();
            let answer = output.title().tokens();
            let (yields, answering) = match output.stream(scope) {
                Some(stream) => {
                    let live = stream.owned(&answered).emit(scope);
                    streams.push(quote! { #answer(#live) });
                    (live, quote! { map_or_else(Answer::Once, Answer::#answer) })
                }
                None => {
                    let (yields, wrapping) = match output.borne(scope, &answered) {
                        Some(yields) => (yields, quote! { Output::#answer }),
                        None => (quote! { () }, quote! { |()| Output::#answer }),
                    };
                    (yields, quote! { map_or_else(|refusal| refusal, #wrapping) })
                }
            };
            let carried = input.borne(scope, &asked);
            let parameter = input.input_name(scope, &asked);
            let (signature, pattern, call) = match (carried, parameter) {
                (Some(ty), Some(parameter)) => {
                    let parameter = parameter.identifier();
                    (
                        quote! { (&self, #parameter: &#ty) },
                        quote! { Input::#variant(carried) },
                        quote! { nexus.#capability(carried).#answering },
                    )
                }
                _ => (
                    quote! { (&self) },
                    quote! { Input::#variant },
                    quote! { nexus.#capability().#answering },
                ),
            };
            capabilities.push(quote! {
                fn #capability #signature -> ::core::result::Result<#yields, Output>;
            });
            let answered_once = streaming && output.stream(scope).is_none();
            arms.push(if answered_once {
                quote! { #pattern => Answer::Once(#call), }
            } else {
                quote! { #pattern => #call, }
            });
        }
        // An open enum's input is answered by one capability of its own, taking
        // the contract's `Input` and yielding its `Output`: an answer of the
        // contract itself, a refusal of it among them, so the capability's
        // refusal is any other `Output`.
        for name in opened {
            let capability = name.0.snake_case().identifier();
            let segment = name.module();
            let module = quote! { self::#segment };
            let variant = name.tokens();
            capabilities.push(quote! {
                fn #capability(&self, #capability: &#module::Input) -> ::core::result::Result<#module::Output, Output>;
            });
            let call = quote! {
                nexus.#capability(carried).map_or_else(|refusal| refusal, Output::#variant)
            };
            arms.push(if streaming {
                quote! { Input::#variant(carried) => Answer::Once(#call), }
            } else {
                quote! { Input::#variant(carried) => #call, }
            });
        }
        let answer = if streaming {
            quote! { Answer }
        } else {
            quote! { Output }
        };
        // A refusal is any `Output`, however large its largest variant:
        // the wire value is answered whole, so it is not boxed for the lint.
        // A capability borrows exactly the type its input carries, so the
        // dispatch lends the matched payload as it is: `&String`, `&Vec<_>`
        // and `&Box<_>` stand where the lints would ask for a narrower borrow.
        let mut items = Vec::new();
        if streaming {
            // An answer given once is the wire's `Output`, held whole beside
            // the streams, however much larger it is than they are.
            items.push(quote! {
                #[allow(clippy::large_enum_variant)]
                pub enum Answer {
                    Once(Output),
                    #( #streams ),*
                }
            });
        }
        items.extend([
            quote! {
                #[allow(clippy::result_large_err, clippy::ptr_arg, clippy::borrowed_box)]
                pub trait Answering {
                    #( #capabilities )*
                }
            },
            quote! {
                pub trait Answerable {
                    type Answer;
                    fn answer<Nexus: Answering>(&self, nexus: &Nexus) -> Self::Answer;
                }
            },
            quote! {
                impl Answerable for Input {
                    type Answer = #answer;
                    fn answer<Nexus: Answering>(&self, nexus: &Nexus) -> #answer {
                        match self { #( #arms )* }
                    }
                }
            },
        ]);
        items
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

/// The trait whose capability writes the generated file allowance in the
/// form rustfmt accepts.
trait AllowFormatting {
    fn format_allow(&self) -> String;
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

/// The trait whose capability parts a bound's colon from the path after it.
///
/// prettyplease prints the tokens of an attribute it does not parse, an rkyv
/// bound `__S: ::rkyv::ser::Writer`, with no space between the colon and the
/// leading `::`, and `:::` lexes as `::` and `:`. No Rust is written `:::`, so
/// every one is the bound's colon followed by a leading `::`.
trait ColonSpacing {
    fn space_colons(&self) -> String;
}

impl ColonSpacing for String {
    fn space_colons(&self) -> String {
        self.replace(":::", ": ::")
    }
}

impl AllowFormatting for String {
    fn format_allow(&self) -> String {
        self.replacen(
            "#![allow(dead_code, non_camel_case_types, non_snake_case, clippy::type_complexity)]\n",
            "#![allow(\n    dead_code,\n    non_camel_case_types,\n    non_snake_case,\n    clippy::type_complexity\n)]\n",
            1,
        )
    }
}

// ---------------------------------------------------------------------------
// Open enums: the types a Signal's fifth section generates
// ---------------------------------------------------------------------------

/// The contract of a file's open enums, written in Ethos and generated by the
/// same pass as every other file: the syntax module's `Name`, `Word`,
/// `Variant` and `Type` (an open enum's variants are data, a name and the type
/// it carries), with the `Import` and `Declaration` a `Type` holds, and the
/// `Input` and `Output` that add, remove and list them. What makes a variant
/// is one for the file, generated once into a module every open enum of it
/// shares; the `Input` and `Output`, with the `Registry`, are each open enum's
/// own, generated into its module. It is generated here, as the module the
/// syntax would be imported from is not yet declared.
const OPEN_CONTRACT: &str = include_str!("open-enum.ethos");

/// The contract each open enum's module is: what generates it, and what it
/// therefore holds, read by the generator and by the resolution of an import
/// alike.
pub(crate) struct Contract;

/// The trait whose capabilities read the contract: the root its Ethos is, the
/// name of the `Registry` trait generated beside what that root generates, the
/// types of that root each open enum has its own of, and the module of the
/// file that holds the rest, shared by every open enum of the file.
pub(crate) trait Contractual {
    fn file(&self) -> File;
    fn registry(&self) -> Name;
    fn own(&self) -> Vec<Name>;
    fn shared(&self) -> Ident;
}

impl Contractual for Contract {
    fn file(&self) -> File {
        Potential::<File>::from(OPEN_CONTRACT.to_owned())
            .actualize()
            .expect("the contract of an open enum is Ethos")
    }

    fn registry(&self) -> Name {
        Name::try_from("Registry").expect("a generated name is an identifier")
    }

    fn own(&self) -> Vec<Name> {
        ["Input", "Output"]
            .into_iter()
            .map(|name| Name::try_from(name).expect("a generated name is an identifier"))
            .collect()
    }

    fn shared(&self) -> Ident {
        Ident::new("open", Span::call_site())
    }
}

/// The items the contract's Ethos generates, parted: those every open enum of
/// a file shares, and those each has its own of.
struct ContractItems {
    shared: Vec<TokenStream>,
    own: Vec<TokenStream>,
}

/// The trait whose capability yields the items the contract's Ethos
/// generates, parted into what the file shares and what each enum owns.
trait ContractParting {
    fn contract_items(&self) -> ContractItems;
}

impl ContractParting for Contract {
    fn contract_items(&self) -> ContractItems {
        let file = self.file();
        let reach = file.reckon(vec![]);
        let scope = Scope {
            file: &file,
            identity: None,
            associated: &[],
            namespaces: &[],
            reach: &reach,
        };
        let owned = self.own();
        let mut items = ContractItems {
            shared: Vec::new(),
            own: Vec::new(),
        };
        for declaration in file.types() {
            let item = declaration.emit(&scope);
            if owned.contains(&crate::checking::Identified::identity(declaration).name) {
                items.own.push(item);
            } else {
                items.shared.push(item);
            }
        }
        items
    }
}

/// The trait whose capability names the module an open enum's contract is
/// generated into: its name in snake case.
pub(crate) trait Modulating {
    fn module(&self) -> Ident;
}

impl Modulating for Name {
    fn module(&self) -> Ident {
        self.0.snake_case().identifier()
    }
}

/// The trait whose capability yields the variant each open enum adds to a
/// Signal's `Input` or `Output`: named for the enum, carrying the enum of
/// that name in the contract's module.
trait Contracting {
    fn contract_variants(&self, implied: &Name) -> Vec<Variant>;
}

impl Contracting for [Name] {
    fn contract_variants(&self, implied: &Name) -> Vec<Variant> {
        let mut variants = Vec::with_capacity(self.len());
        for name in self {
            // The contract module is the file's own, reached by `self::`, not
            // from the extern prelude every other source is written from.
            let module = format!("self:{}", name.module());
            variants.push(Variant::Typed(
                name.clone(),
                Reference::Named(NamedReference {
                    source: Some(Source::try_from(module.as_str()).expect("a module is a source")),
                    name: implied.clone(),
                    arguments: vec![],
                }),
            ));
        }
        variants
    }
}

/// The trait whose capability yields the module of the contract a file's open
/// enums share: the syntax of a variant, generated once.
trait SharedContracting {
    fn shared_contract(&self) -> TokenStream;
}

impl SharedContracting for Contract {
    fn shared_contract(&self) -> TokenStream {
        let items = self.contract_items().shared;
        let module = self.shared();
        quote! {
            #[rustfmt::skip]
            pub mod #module {
                #( #items )*
            }
        }
    }
}

/// The trait whose capability yields the module an open enum's contract is:
/// what the file shares, re-exported, so that every name the contract
/// generates is imported from the module of the enum, the `Input` and
/// `Output` that are the enum's own, and the `Registry` a runtime implements
/// to say which variants have been added.
trait Contracted {
    fn contract(&self) -> TokenStream;
}

impl Contracted for Name {
    fn contract(&self) -> TokenStream {
        let items = Contract.contract_items().own;
        let shared = Contract.shared();
        let module = self.module();
        let registry = Contract.registry().tokens();
        quote! {
            #[rustfmt::skip]
            pub mod #module {
                pub use super::#shared::*;
                #( #items )*
                pub trait #registry {
                    fn variant(&self, name: &Name) -> ::core::option::Option<Variant>;
                }
            }
        }
    }
}

/// The trait whose capability yields the reading of one child datom: through
/// `Opening` with the registry where the child reaches an open enum, through
/// `Composing` where it does not, each within the composition depth.
trait ChildReading {
    fn child_reading(&self, reach: &Reach, child: TokenStream) -> TokenStream;
}

impl<Held: Bearing + ?Sized> ChildReading for Held {
    fn child_reading(&self, reach: &Reach, child: TokenStream) -> TokenStream {
        if reach.opens_held(self).is_empty() {
            quote! { ::datom_codec::Composable::compose(#child, budget)? }
        } else {
            quote! {
                {
                    let child = #child;
                    ::datom_codec::CompositionDepthing::enter_composition(budget, &child.path)?;
                    let read = Opening::open(child, budget, registry);
                    ::datom_codec::CompositionDepthing::leave_composition(budget);
                    read?
                }
            }
        }
    }
}

/// The trait whose capability wraps a reading into the `Opening` impl of a
/// type, bound by the registry of each open enum it reaches.
trait OpenImplementing {
    fn open_impl(&self, opens: &[Name], body: TokenStream) -> TokenStream;
    /// The `Opening` of a type that reaches no open enum, in a file that has
    /// one: its plain compose, the registry unread, beside its `Composing`, so
    /// that every position of the file is read through `Opening` and a
    /// container never asks which side reaches. A generic type, or a file of
    /// no open enum, has none.
    fn plain_open(&self, scope: &Scope) -> TokenStream;
}

impl OpenImplementing for Identity {
    fn open_impl(&self, opens: &[Name], body: TokenStream) -> TokenStream {
        let name = self.name.tokens();
        let modules: Vec<Ident> = opens.iter().map(Modulating::module).collect();
        quote! {
            #[cfg(feature = "datom")]
            #[rustfmt::skip]
            impl<Registry: #( self::#modules::Registry )+*> Opening<Registry> for #name {
                fn open(
                    datom: &::datom_codec::Datom,
                    budget: &mut ::datom_codec::Budget,
                    registry: &Registry,
                ) -> ::core::result::Result<Self, ::datom_codec::Error> {
                    #body
                }
            }
        }
    }

    fn plain_open(&self, scope: &Scope) -> TokenStream {
        if scope.file.opened().is_empty() || !self.constraints.is_empty() {
            return TokenStream::new();
        }
        self.name.tokens().plain_open()
    }
}

/// The trait whose capability writes the `Opening` of a type read by its
/// `Composing` alone, the registry unread: a scalar, or a declared type that
/// reaches no open enum. Its caller has entered the composition depth, as
/// `Composing` expects of one reading a child.
trait PlainOpening {
    fn plain_open(&self) -> TokenStream;
}

impl PlainOpening for TokenStream {
    fn plain_open(&self) -> TokenStream {
        let value = self;
        quote! {
            #[cfg(feature = "datom")]
            #[rustfmt::skip]
            impl<Registry> Opening<Registry> for #value {
                fn open(
                    datom: &::datom_codec::Datom,
                    budget: &mut ::datom_codec::Budget,
                    _registry: &Registry,
                ) -> ::core::result::Result<Self, ::datom_codec::Error> {
                    <Self as ::datom_codec::Composing>::compose(datom, budget)
                }
            }
        }
    }
}

/// The trait whose capability yields the reading of a struct of these
/// positions, as the `Composing` derive reads one: a struct form of its
/// arity, each position in order.
trait StructReading {
    fn struct_reading(&self, reach: &Reach, fields: &[Ident]) -> TokenStream;
}

impl StructReading for [Reference] {
    fn struct_reading(&self, reach: &Reach, fields: &[Ident]) -> TokenStream {
        let arity = proc_macro2::Literal::usize_unsuffixed(self.len());
        let mut reads = Vec::with_capacity(self.len());
        for (index, reference) in self.iter().enumerate() {
            let index = proc_macro2::Literal::usize_unsuffixed(index);
            reads.push(reference.child_reading(reach, quote! { &children[#index] }));
        }
        quote! {
            ::datom_codec::Budgeted::spend(budget, &datom.path)?;
            let children = match &datom.form {
                ::datom_codec::Form::Struct(children) => children,
                found => {
                    return ::core::result::Result::Err(
                        <::datom_codec::Error as ::datom_codec::ErrorRaising>::composition(
                            datom.path.clone(),
                            ::datom_codec::ErrorKind::Form {
                                expected: ::std::string::String::from("Struct"),
                                found: ::std::string::String::from(::datom_codec::Naming::form_name(found)),
                            },
                        ),
                    );
                }
            };
            if children.len() != #arity {
                return ::core::result::Result::Err(
                    <::datom_codec::Error as ::datom_codec::ErrorRaising>::composition(
                        datom.path.clone(),
                        ::datom_codec::ErrorKind::Arity {
                            expected: #arity,
                            found: children.len() as ::datom_codec::Integer,
                        },
                    ),
                );
            }
            ::core::result::Result::Ok(Self { #( #fields: #reads ),* })
        }
    }
}

/// The trait whose capability yields the reading of an enum of these
/// variants, as the `Composing` derive reads one: a bare head for a variant
/// that carries nothing, a head and its body for one that carries.
trait EnumReading {
    fn enum_reading(&self, scope: &Scope, identity: &Identity) -> TokenStream;
}

impl EnumReading for [Variant] {
    fn enum_reading(&self, scope: &Scope, identity: &Identity) -> TokenStream {
        let spelled = identity.name.0.clone();
        let mut units = Vec::new();
        let mut unit_spellings = Vec::new();
        let mut carriers = Vec::new();
        let mut carrier_spellings = Vec::new();
        let mut reads = Vec::new();
        for variant in self {
            let (name, carries) = match variant {
                Variant::Bare(name) => (name, scope.carries(name)),
                Variant::Typed(name, _) | Variant::Struct(name, _) | Variant::Enum(name, _) => {
                    (name, true)
                }
            };
            if carries {
                carriers.push(name.tokens());
                carrier_spellings.push(name.0.clone());
                reads.push(variant.child_reading(scope.reach, quote! { body }));
            } else {
                units.push(name.tokens());
                unit_spellings.push(name.0.clone());
            }
        }
        let unknown = quote! {
            ::core::result::Result::Err(
                <::datom_codec::Error as ::datom_codec::ErrorRaising>::composition(
                    datom.path.clone(),
                    ::datom_codec::ErrorKind::Variant {
                        expected: ::std::string::String::from(#spelled),
                        found: ::std::string::String::from(other),
                    },
                ),
            )
        };
        // A head with no variant that carries nothing is unknown outright.
        let bare = if units.is_empty() {
            quote! {
                let other = head.as_str();
                #unknown
            }
        } else {
            quote! {
                match head.as_str() {
                    #( #unit_spellings => ::core::result::Result::Ok(Self::#units), )*
                    other => #unknown,
                }
            }
        };
        quote! {
            match &datom.form {
                ::datom_codec::Form::Bare(head) => {
                    ::datom_codec::Budgeted::spend(budget, &datom.path)?;
                    #bare
                }
                _ => {
                    let (head, body) = ::datom_codec::Variantizing::variant(datom, budget, "Variant")?;
                    match head {
                        #( #carrier_spellings => ::core::result::Result::Ok(Self::#carriers(#reads)), )*
                        other => ::core::result::Result::Err(
                            <::datom_codec::Error as ::datom_codec::ErrorRaising>::composition(
                                datom.path.clone(),
                                ::datom_codec::ErrorKind::Variant {
                                    expected: ::std::string::String::from(#spelled),
                                    found: ::std::string::String::from(other),
                                },
                            ),
                        ),
                    }
                }
            }
        }
    }
}

/// The trait whose capability yields the items a Signal's open enums
/// generate. Once for the file, `Carried`, an enum of every type the file
/// declares, one variant each, and `Opening`, by which a datom is read with
/// the registry of added variants. For each open enum, its contract module
/// and its value: the variant's name with an `Option` of what it carries.
///
/// A type the file declares is carried unless it is generic, or reaches an
/// open enum, which would make the value hold itself. The value's datom is
/// the name, and after it what the entry of that name in the registry says it
/// carries, read as that type: `Fable.High`. Written, the value needs no
/// registry; read, it does, so the value, and every type the file declares
/// that reaches it, bears no `Composing` and is read only by `Opening`, with
/// a registry of each open enum it reaches. `Carrying` gives a body what it
/// needs of `Carried`: the names of its types, and the reading of one by its
/// name. `Opening` names datom-codec in its signature, so it stands behind
/// the `datom` feature, as `Carrying::read` does.
trait Opening {
    fn open_items(&self, scope: &Scope) -> Vec<TokenStream>;
}

impl Opening for File {
    fn open_items(&self, scope: &Scope) -> Vec<TokenStream> {
        let opened = self.opened();
        if opened.is_empty() {
            return vec![];
        }
        let shared = Contract.shared();
        let mut variants = Vec::new();
        let mut names = Vec::new();
        let mut spellings = Vec::new();
        for declaration in scope.reach.carriable(self) {
            let identity = crate::checking::Identified::identity(declaration);
            variants.push(Variant::Typed(
                identity.name.clone(),
                identity.name.carried(),
            ));
            names.push(identity.name.tokens());
            spellings.push(identity.name.0.clone());
        }
        // A Signal that declares nothing to carry has no `Carried`, and its
        // open enums' values are names alone: an enum of no variants cannot
        // be archived.
        let empty = names.is_empty();
        let mut items = vec![Contract.shared_contract()];
        if !empty {
            items.push(
                TypeDeclaration::Enum(
                    Identity {
                        name: Name::try_from("Carried").expect("static identifier"),
                        constraints: vec![],
                    },
                    variants,
                )
                .emit(scope),
            );
            // What a body needs of `Carried`: the names of its types, the
            // name of the one a value is, and the reading of one by its name.
            items.push(quote! {
                #[rustfmt::skip]
                pub trait Carrying: Sized {
                    fn names() -> ::std::vec::Vec<self::#shared::Name>;
                    fn name(&self) -> self::#shared::Name;
                    #[cfg(feature = "datom")]
                    fn read(
                        name: &self::#shared::Name,
                        datom: &::datom_codec::Datom,
                        budget: &mut ::datom_codec::Budget,
                    ) -> ::core::result::Result<Self, ::datom_codec::Error>;
                }
            });
            items.push(quote! {
                #[rustfmt::skip]
                impl Carrying for Carried {
                    fn names() -> ::std::vec::Vec<self::#shared::Name> {
                        ::std::vec![ #( self::#shared::Name(::std::string::String::from(#spellings)) ),* ]
                    }
                    fn name(&self) -> self::#shared::Name {
                        self::#shared::Name(::std::string::String::from(match self {
                            #( Carried::#names(_) => #spellings, )*
                        }))
                    }
                    #[cfg(feature = "datom")]
                    fn read(
                        name: &self::#shared::Name,
                        datom: &::datom_codec::Datom,
                        budget: &mut ::datom_codec::Budget,
                    ) -> ::core::result::Result<Self, ::datom_codec::Error> {
                        match name.0.as_str() {
                            #( #spellings => ::core::result::Result::Ok(Carried::#names(::datom_codec::Composing::compose(datom, budget)?)), )*
                            other => ::core::result::Result::Err(
                                <::datom_codec::Error as ::datom_codec::ErrorRaising>::composition(
                                    datom.path.clone(),
                                    ::datom_codec::ErrorKind::Variant {
                                        expected: ::std::string::String::from("a type the file declares"),
                                        found: ::std::string::String::from(other),
                                    },
                                ),
                            ),
                        }
                    }
                }
            });
        }
        items.push(quote! {
            #[cfg(feature = "datom")]
            #[rustfmt::skip]
            pub trait Opening<Registry>: Sized {
                fn open(
                    datom: &::datom_codec::Datom,
                    budget: &mut ::datom_codec::Budget,
                    registry: &Registry,
                ) -> ::core::result::Result<Self, ::datom_codec::Error>;
            }
        });
        // The containers a type reaches an open enum through, read as the
        // codec reads them, each element with the registry.
        items.push(quote! {
            #[cfg(feature = "datom")]
            #[rustfmt::skip]
            impl<Registry, Value: Opening<Registry>> Opening<Registry> for ::std::vec::Vec<Value> {
                fn open(
                    datom: &::datom_codec::Datom,
                    budget: &mut ::datom_codec::Budget,
                    registry: &Registry,
                ) -> ::core::result::Result<Self, ::datom_codec::Error> {
                    ::datom_codec::Budgeted::spend(budget, &datom.path)?;
                    let children = match &datom.form {
                        ::datom_codec::Form::Vector(children) => children,
                        found => {
                            return ::core::result::Result::Err(
                                <::datom_codec::Error as ::datom_codec::ErrorRaising>::composition(
                                    datom.path.clone(),
                                    ::datom_codec::ErrorKind::Form {
                                        expected: ::std::string::String::from("Vector"),
                                        found: ::std::string::String::from(::datom_codec::Naming::form_name(found)),
                                    },
                                ),
                            );
                        }
                    };
                    let mut values = ::std::vec::Vec::with_capacity(children.len());
                    for child in children {
                        ::datom_codec::CompositionDepthing::enter_composition(budget, &child.path)?;
                        let read = Value::open(child, budget, registry);
                        ::datom_codec::CompositionDepthing::leave_composition(budget);
                        values.push(read?);
                    }
                    ::core::result::Result::Ok(values)
                }
            }
        });
        items.push(quote! {
            #[cfg(feature = "datom")]
            #[rustfmt::skip]
            impl<Registry, Value: Opening<Registry>> Opening<Registry> for ::core::option::Option<Value> {
                fn open(
                    datom: &::datom_codec::Datom,
                    budget: &mut ::datom_codec::Budget,
                    registry: &Registry,
                ) -> ::core::result::Result<Self, ::datom_codec::Error> {
                    if let ::datom_codec::Form::Bare(head) = &datom.form {
                        ::datom_codec::Budgeted::spend(budget, &datom.path)?;
                        return if head == "None" {
                            ::core::result::Result::Ok(::core::option::Option::None)
                        } else {
                            ::core::result::Result::Err(
                                <::datom_codec::Error as ::datom_codec::ErrorRaising>::composition(
                                    datom.path.clone(),
                                    ::datom_codec::ErrorKind::Variant {
                                        expected: ::std::string::String::from("Option"),
                                        found: head.clone(),
                                    },
                                ),
                            )
                        };
                    }
                    let (head, body) = ::datom_codec::Variantizing::variant(datom, budget, "Variant")?;
                    if head != "Some" {
                        return ::core::result::Result::Err(
                            <::datom_codec::Error as ::datom_codec::ErrorRaising>::composition(
                                datom.path.clone(),
                                ::datom_codec::ErrorKind::Variant {
                                    expected: ::std::string::String::from("Option"),
                                    found: ::std::string::String::from(head),
                                },
                            ),
                        );
                    }
                    ::datom_codec::CompositionDepthing::enter_composition(budget, &body.path)?;
                    let read = Value::open(body, budget, registry);
                    ::datom_codec::CompositionDepthing::leave_composition(budget);
                    ::core::result::Result::Ok(::core::option::Option::Some(read?))
                }
            }
        });
        // A Result is read as an Option is, each side through `Opening`:
        // `Ok.<value>` or `Err.<failure>`, as datom-codec composes one.
        items.push(quote! {
            #[cfg(feature = "datom")]
            #[rustfmt::skip]
            impl<Registry, Value: Opening<Registry>, Failure: Opening<Registry>> Opening<Registry>
                for ::core::result::Result<Value, Failure>
            {
                fn open(
                    datom: &::datom_codec::Datom,
                    budget: &mut ::datom_codec::Budget,
                    registry: &Registry,
                ) -> ::core::result::Result<Self, ::datom_codec::Error> {
                    let (head, body) = ::datom_codec::Variantizing::variant(datom, budget, "Variant")?;
                    let ok = match head {
                        "Ok" => true,
                        "Err" => false,
                        found => {
                            return ::core::result::Result::Err(
                                <::datom_codec::Error as ::datom_codec::ErrorRaising>::composition(
                                    datom.path.clone(),
                                    ::datom_codec::ErrorKind::Variant {
                                        expected: ::std::string::String::from("Result"),
                                        found: ::std::string::String::from(found),
                                    },
                                ),
                            );
                        }
                    };
                    ::datom_codec::CompositionDepthing::enter_composition(budget, &body.path)?;
                    let read = if ok {
                        Value::open(body, budget, registry).map(::core::result::Result::Ok)
                    } else {
                        Failure::open(body, budget, registry).map(::core::result::Result::Err)
                    };
                    ::datom_codec::CompositionDepthing::leave_composition(budget);
                    read
                }
            }
        });
        // The scalars Ethos knows, each read by its own `Composing`: every
        // position of the file is read through `Opening`. The trait is the
        // file's own, so each file's impls are its own and none conflicts.
        for scalar in [
            Intrinsic::String,
            Intrinsic::Integer,
            Intrinsic::Decimal,
            Intrinsic::Boolean,
            Intrinsic::Meaning,
        ] {
            items.push(scalar.tokens().plain_open());
        }
        items.push(quote! {
            #[cfg(feature = "datom")]
            #[rustfmt::skip]
            impl<Registry, Value: Opening<Registry>> Opening<Registry> for ::std::boxed::Box<Value> {
                fn open(
                    datom: &::datom_codec::Datom,
                    budget: &mut ::datom_codec::Budget,
                    registry: &Registry,
                ) -> ::core::result::Result<Self, ::datom_codec::Error> {
                    ::core::result::Result::Ok(::std::boxed::Box::new(Value::open(datom, budget, registry)?))
                }
            }
        });
        // The value's field of what it carries, its absence, and what its
        // datom is: the name, and after it the carried value as the type it
        // is, whose own enum variant is not written.
        let (field, absent, datomizing) = if empty {
            (
                TokenStream::new(),
                TokenStream::new(),
                quote! {
                    ::datom_codec::Datom {
                        path: at,
                        form: ::datom_codec::Form::Bare(self.name.0.clone()),
                    }
                },
            )
        } else {
            (
                quote! { pub carried: ::core::option::Option<Carried>, },
                quote! { carried: ::core::option::Option::None, },
                quote! {
                    match &self.carried {
                        ::core::option::Option::None => ::datom_codec::Datom {
                            path: at,
                            form: ::datom_codec::Form::Bare(self.name.0.clone()),
                        },
                        ::core::option::Option::Some(carried) => {
                            let body = match *carried {
                                #( Carried::#names(ref value) => ::datom_codec::Datomizable::datomize(
                                    value,
                                    ::datom_codec::Branchable::child(&at, 1),
                                ), )*
                            };
                            ::datom_codec::Datom {
                                path: at,
                                form: ::datom_codec::Form::Variant(
                                    ::datom_codec::Symbol(self.name.0.clone()),
                                    ::std::boxed::Box::new(body),
                                ),
                            }
                        }
                    }
                },
            )
        };
        for open in opened {
            let segment = open.module();
            let module = quote! { self::#segment };
            let value = open.tokens();
            // The arm that reads a name carrying a type, by the type its
            // entry in the registry names.
            let middle = if empty {
                TokenStream::new()
            } else {
                quote! {
                    (
                        ::core::option::Option::Some(self::#shared::Type::Name(carried)),
                        ::core::option::Option::Some(body),
                    ) => {
                        let carried = <Carried as Carrying>::read(&carried, body, budget)?;
                        ::core::result::Result::Ok(Self { name, carried: ::core::option::Option::Some(carried) })
                    }
                }
            };
            items.push(open.contract());
            items.push(quote! {
                #[rustfmt::skip]
                #[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
                pub struct #value {
                    pub name: self::#shared::Name,
                    #field
                }
            });
            items.push(quote! {
                #[cfg(feature = "datom")]
                #[rustfmt::skip]
                impl ::datom_codec::Datomizable for #value {
                    fn datomize(&self, at: ::datom_codec::Path) -> ::datom_codec::Datom {
                        #datomizing
                    }
                }
            });
            items.push(quote! {
                #[cfg(feature = "datom")]
                #[rustfmt::skip]
                impl<Registry: #module::Registry> Opening<Registry> for #value {
                    fn open(
                        datom: &::datom_codec::Datom,
                        budget: &mut ::datom_codec::Budget,
                        registry: &Registry,
                    ) -> ::core::result::Result<Self, ::datom_codec::Error> {
                        ::datom_codec::Budgeted::spend(budget, &datom.path)?;
                        let (head, body) = match &datom.form {
                            ::datom_codec::Form::Bare(head) => (head, ::core::option::Option::None),
                            ::datom_codec::Form::Variant(head, body) => {
                                (&head.0, ::core::option::Option::Some(&**body))
                            }
                            found => {
                                return ::core::result::Result::Err(
                                    <::datom_codec::Error as ::datom_codec::ErrorRaising>::composition(
                                        datom.path.clone(),
                                        ::datom_codec::ErrorKind::Form {
                                            expected: ::std::string::String::from("Variant"),
                                            found: ::std::string::String::from(::datom_codec::Naming::form_name(found)),
                                        },
                                    ),
                                );
                            }
                        };
                        let name = self::#shared::Name(head.clone());
                        let ::core::option::Option::Some(entry) = registry.variant(&name) else {
                            return ::core::result::Result::Err(
                                <::datom_codec::Error as ::datom_codec::ErrorRaising>::composition(
                                    datom.path.clone(),
                                    ::datom_codec::ErrorKind::Variant {
                                        expected: ::std::string::String::from(stringify!(#value)),
                                        found: head.clone(),
                                    },
                                ),
                            );
                        };
                        match (entry.type_option, body) {
                            (::core::option::Option::None, ::core::option::Option::None) => {
                                ::core::result::Result::Ok(Self { name, #absent })
                            }
                            #middle
                            _ => ::core::result::Result::Err(
                                <::datom_codec::Error as ::datom_codec::ErrorRaising>::composition(
                                    datom.path.clone(),
                                    ::datom_codec::ErrorKind::Value {
                                        expected: ::std::string::String::from("the form of the type its entry names"),
                                        value: head.clone(),
                                    },
                                ),
                            ),
                        }
                    }
                }
            });
        }
        items
    }
}

/// A file read alone names itself by no source: only what it names bare is
/// its own, and its reach is decided from the file alone.
impl Generating for File {
    fn generate(&self) -> Result<String, crate::Error> {
        self.generate_within(&self.reckon(vec![]))
    }
}

/// The trait whose capability generates a file's Rust module by a reach
/// decided where it was read: a Nexus decides it with every file given.
pub(crate) trait ReachGenerating {
    fn generate_within(&self, reach: &Reach) -> Result<String, crate::Error>;
}

impl ReachGenerating for File {
    fn generate_within(&self, reach: &Reach) -> Result<String, crate::Error> {
        crate::Validating::validate(self)?;
        let scope = Scope {
            file: self,
            identity: None,
            associated: &[],
            namespaces: &[],
            reach,
        };
        Ok(self.items(&scope).render())
    }
}
