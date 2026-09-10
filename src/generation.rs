//! Generation: File to Rust text (cannot fault, the file having been checked).
//!
//! Each declaration emits itself: a struct declaration its struct and
//! its datomic machinery, an enum declaration its enum and its
//! machinery, a kind declaration its trait, an association its
//! assertion; the file emits by walking its variant's sections. Names
//! are resolved through the scope, never a table; the generated Rust
//! carries no `use` and writes every foreign name fully qualified.
//!
//! One rule decides boxing: a position whose type reaches the type
//! that declares it, walking through declared types, aliases, `Option`
//! and `Result` but not through `Vector`, is boxed as a whole
//! (`std::boxed::Box<std::option::Option<Tree>>`), and the datom machinery
//! never sees the box.

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use crate::checking::{Checkable, Declaring};
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
            Intrinsic::Decimal => quote! { f64 },
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
// Reaching: the boxing rule
// ---------------------------------------------------------------------------

/// The kind whose capability tells whether a value holds the target type by value, through declared types, aliases, Option and Result.
trait Reaching {
    fn reaches(&self, target: &Name, file: &File, visited: &mut Vec<Name>) -> bool;
}

impl Reaching for Reference {
    fn reaches(&self, target: &Name, file: &File, visited: &mut Vec<Name>) -> bool {
        if self.source.is_none() {
            if &self.name == target || self.name.0 == "Self" {
                return true;
            }
            match file.resolve(&self.name) {
                Resolution::Intrinsic(Intrinsic::Vector) => return false,
                Resolution::Type(name) if !visited.contains(&name) => {
                    visited.push(name.clone());
                    if let Some(declaration) = file.declaration(&name)
                        && declaration.reaches(target, file, visited)
                    {
                        return true;
                    }
                }
                _ => {}
            }
        }
        for argument in &self.arguments {
            if argument.reaches(target, file, visited) {
                return true;
            }
        }
        false
    }
}

impl Reaching for [Reference] {
    fn reaches(&self, target: &Name, file: &File, visited: &mut Vec<Name>) -> bool {
        for reference in self {
            if reference.reaches(target, file, visited) {
                return true;
            }
        }
        false
    }
}

impl Reaching for TypeDeclaration {
    fn reaches(&self, target: &Name, file: &File, visited: &mut Vec<Name>) -> bool {
        match self {
            TypeDeclaration::Struct(_, positions) => positions.reaches(target, file, visited),
            TypeDeclaration::Enum(_, variants) => variants.reaches(target, file, visited),
            TypeDeclaration::Alias(_, aliased) => aliased.reaches(target, file, visited),
        }
    }
}

impl Reaching for Variant {
    fn reaches(&self, target: &Name, file: &File, visited: &mut Vec<Name>) -> bool {
        match self {
            Variant::Bare(_) => false,
            Variant::Typed(_, reference) => reference.reaches(target, file, visited),
            Variant::Struct(_, positions) => positions.reaches(target, file, visited),
            Variant::Enum(_, variants) => variants.reaches(target, file, visited),
        }
    }
}

impl Reaching for [Variant] {
    fn reaches(&self, target: &Name, file: &File, visited: &mut Vec<Name>) -> bool {
        for variant in self {
            if variant.reaches(target, file, visited) {
                return true;
            }
        }
        false
    }
}

/// The kind whose capabilities yield a position's Rust type, boxed when it reaches its owner.
pub(crate) trait Positioning {
    fn boxed(&self, scope: &Scope, owner: &Name) -> bool;
    fn position(&self, scope: &Scope, owner: &Name) -> TokenStream;
}

impl Positioning for Reference {
    fn boxed(&self, scope: &Scope, owner: &Name) -> bool {
        self.reaches(owner, scope.file, &mut vec![])
    }

    fn position(&self, scope: &Scope, owner: &Name) -> TokenStream {
        // Put the indirection immediately around the recursive argument.
        // `Option<Box<Tree>>` lets the Datom derive see the recursive edge,
        // while `Box<Option<Tree>>` hides it behind the container and causes
        // an infinitely recursive derive bound.
        if self.source.is_none()
            && matches!(
                scope.file.resolve(&self.name),
                Resolution::Intrinsic(Intrinsic::Option | Intrinsic::Result)
            )
        {
            let name = self.name.tokens();
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
}

trait Fielding {
    fn field_base(&self) -> String;
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
    fn field_base(&self) -> String {
        let mut parts: Vec<String> = self.arguments.iter().map(Self::field_base).collect();
        parts.push(self.name.0.as_str().snake_case());
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

trait FieldNaming {
    fn field_names(&self) -> Vec<Ident>;
}

impl FieldNaming for [Reference] {
    fn field_names(&self) -> Vec<Ident> {
        let bases: Vec<String> = self.iter().map(Reference::field_base).collect();
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
    fn definition(&self, scope: &Scope, owner: &Name, enclosing: &Identity) -> TokenStream;
    fn nested(
        &self,
        scope: &Scope,
        owner: &Name,
        enclosing: &Identity,
        conditional: bool,
    ) -> TokenStream;
}

/// The kind whose capability names the enum type an inline enum variant declares.
trait Nesting {
    fn nested_identity(&self, name: &Name) -> Identity;
}

impl Nesting for Identity {
    fn nested_identity(&self, name: &Name) -> Identity {
        let stem = if self.name.0.ends_with("_Data") {
            format!("{}_{}", self.name.0, name.0)
        } else {
            name.0.clone()
        };
        let name = Name::try_from(format!("{stem}_Data"))
            .expect("derived inline identifiers are identifiers");
        Identity {
            name,
            constraints: self.constraints.clone(),
        }
    }
}

impl Varianted for Variant {
    fn definition(&self, scope: &Scope, owner: &Name, enclosing: &Identity) -> TokenStream {
        match self {
            Variant::Bare(name) => {
                let variant = name.tokens();
                if scope.file.declaration(name).is_some() {
                    let ty = name.tokens();
                    quote! { #variant(#ty) }
                } else {
                    quote! { #variant }
                }
            }
            Variant::Typed(name, reference) => {
                let name = name.tokens();
                let ty = reference.position(scope, owner);
                quote! { #name(#ty) }
            }
            Variant::Struct(name, _) | Variant::Enum(name, _) => {
                let nested = enclosing.nested_identity(name);
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
        conditional: bool,
    ) -> TokenStream {
        match self {
            Variant::Struct(name, positions) => {
                let nested = enclosing.nested_identity(name);
                positions.structure(scope, owner, &nested, conditional)
            }
            Variant::Enum(name, variants) => {
                let nested = enclosing.nested_identity(name);
                variants.enumeration(scope, owner, &nested, conditional)
            }
            Variant::Bare(_) | Variant::Typed(_, _) => TokenStream::new(),
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
        conditional: bool,
    ) -> TokenStream;
}

/// The kind whose capability emits an enum of these variants with its datomic machinery.
trait Enumerating {
    fn enumeration(
        &self,
        scope: &Scope,
        owner: &Name,
        identity: &Identity,
        conditional: bool,
    ) -> TokenStream;
}

trait DatomDeriving {
    fn datom_derives(&self) -> TokenStream;
}

impl DatomDeriving for bool {
    fn datom_derives(&self) -> TokenStream {
        if *self {
            quote! {
                #[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
                #[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Compositional))]
            }
        } else {
            quote! { #[derive(datom_codec::Datomizable, datom_codec::Compositional)] }
        }
    }
}

impl Structuring for [Reference] {
    fn structure(
        &self,
        scope: &Scope,
        owner: &Name,
        identity: &Identity,
        conditional: bool,
    ) -> TokenStream {
        let name = identity.name.tokens();
        let parameters = identity.parameters(scope);
        let mut types = Vec::with_capacity(self.len());
        for position in self {
            types.push(position.position(scope, owner));
        }
        let fields = self.field_names();
        let derive = conditional.datom_derives();
        quote! {
            #derive
            pub struct #name #parameters { #( pub #fields: #types ),* }
        }
    }
}

impl Enumerating for [Variant] {
    fn enumeration(
        &self,
        scope: &Scope,
        owner: &Name,
        identity: &Identity,
        conditional: bool,
    ) -> TokenStream {
        let name = identity.name.tokens();
        let parameters = identity.parameters(scope);
        let derive = conditional.datom_derives();
        let mut definitions = Vec::with_capacity(self.len());
        let mut nested = Vec::new();
        for variant in self {
            definitions.push(variant.definition(scope, owner, identity));
            nested.push(variant.nested(scope, owner, identity, conditional));
        }
        quote! {
            #( #nested )*
            #derive
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
                positions.structure(
                    &inner,
                    &identity.name,
                    identity,
                    matches!(scope.file, File::Signal(_)),
                )
            }
            TypeDeclaration::Enum(identity, variants) => {
                let inner = Scope {
                    file: scope.file,
                    identity: Some(identity),
                    associated: scope.associated,
                };
                variants.enumeration(
                    &inner,
                    &identity.name,
                    identity,
                    matches!(scope.file, File::Signal(_)),
                )
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
                quote! { pub type #name #parameters = #aliased; }
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
        let mut items = vec![quote! { #![allow(dead_code, non_camel_case_types, non_snake_case)] }];
        match self {
            File::Library(library) => {
                for declaration in &library.types {
                    items.push(declaration.emit(scope));
                }
                for declaration in &library.kinds {
                    items.push(declaration.emit(scope));
                }
                for association in &library.associations {
                    items.push(association.emit(scope));
                }
            }
            File::Signal(signal) => {
                for declaration in &signal.types {
                    items.push(declaration.emit(scope));
                }
                let request = TypeDeclaration::Enum(
                    Identity {
                        name: Name::try_from("Query").expect("static identifier"),
                        constraints: vec![],
                    },
                    signal.requests.clone(),
                );
                let response = TypeDeclaration::Enum(
                    Identity {
                        name: Name::try_from("Response").expect("static identifier"),
                        constraints: vec![],
                    },
                    signal.responses.clone(),
                );
                items.push(request.emit(scope));
                items.push(response.emit(scope));
            }
            File::Sema(sema) => {
                for declaration in &sema.types {
                    items.push(declaration.emit(scope));
                }
            }
        }
        quote! { #( #items )* }
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
        Ok(prettyplease::unparse(&file))
    }
}
