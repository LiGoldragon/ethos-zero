//! Checking: the file validated whole (may err).
//!
//! Resolution is borne by the declarations: an import resolves the
//! names it carries, a declaration its own name, an identity its
//! parameters, a kind its associated types, and the file walks its
//! variant's sections in turn, then the intrinsics. Checking walks the
//! concept as the structure was laid out, so every error is at the
//! path of the structure in error, relative to the checked value.

use datom_codec::{Integer, Path};

use crate::{
    ArityProblem, AssociatedConstant, AssociatedType, Association, Capability, ConceptualErroring,
    Constraint, Error, File, Identifiable, Identity, Import, Intrinsic, KindBody, KindDeclaration,
    Name, Placing, Problem, Reference, Resolution, Resolving, Role, Scope, Sema, Signal, Signature,
    TypeDeclaration, Variant,
};

/// A schema must remain small enough for complete whole-file checking to have
/// a caller-visible, finite cost. Deep structure has a separate reader bound;
/// this bounds the flat `Types` declaration graph. Other file roots have their
/// own bounded structural readers and do not share this declaration budget.
const TYPE_DECLARATION_LIMIT: usize = 512;

// ---------------------------------------------------------------------------
// Resolution
// ---------------------------------------------------------------------------

impl Resolving for Import {
    fn resolve(&self, name: &Name) -> Resolution {
        match self {
            Import::One(source, imported) if &imported.name == name => {
                Resolution::Imported(source.clone(), imported.emitted.clone())
            }
            Import::One(_, _) => Resolution::Undeclared,
            Import::Many(source, imports) => {
                for imported in imports {
                    if &imported.name == name {
                        return Resolution::Imported(source.clone(), imported.emitted.clone());
                    }
                }
                Resolution::Undeclared
            }
        }
    }
}

impl Resolving for [Import] {
    fn resolve(&self, name: &Name) -> Resolution {
        for import in self {
            let resolution = import.resolve(name);
            if resolution != Resolution::Undeclared {
                return resolution;
            }
        }
        Resolution::Undeclared
    }
}

impl Resolving for TypeDeclaration {
    fn resolve(&self, name: &Name) -> Resolution {
        let declared = match self {
            TypeDeclaration::Struct(identity, _)
            | TypeDeclaration::Enum(identity, _)
            | TypeDeclaration::Alias(identity, _) => &identity.name,
        };
        if declared == name {
            Resolution::Type(name.clone())
        } else {
            Resolution::Undeclared
        }
    }
}

impl Resolving for [TypeDeclaration] {
    fn resolve(&self, name: &Name) -> Resolution {
        for declaration in self {
            let resolution = declaration.resolve(name);
            if resolution != Resolution::Undeclared {
                return resolution;
            }
        }
        Resolution::Undeclared
    }
}

impl Resolving for KindDeclaration {
    fn resolve(&self, name: &Name) -> Resolution {
        if &self.identity.name == name {
            Resolution::Kind(name.clone())
        } else {
            Resolution::Undeclared
        }
    }
}

impl Resolving for [KindDeclaration] {
    fn resolve(&self, name: &Name) -> Resolution {
        for declaration in self {
            let resolution = declaration.resolve(name);
            if resolution != Resolution::Undeclared {
                return resolution;
            }
        }
        Resolution::Undeclared
    }
}

/// The kind whose capability yields the names a file variant implies: a Signal's query and response types.
pub(crate) trait Implying {
    /// The implied type names.
    fn implied(&self) -> Vec<Name>;
}

impl Implying for File {
    fn implied(&self) -> Vec<Name> {
        match self {
            File::Library(_) => vec![],
            File::Signal(_) => vec![
                Name::try_from("Query").expect("static identifier"),
                Name::try_from("Response").expect("static identifier"),
            ],
            File::Sema(_) => vec![],
        }
    }
}

impl Resolving for File {
    fn resolve(&self, name: &Name) -> Resolution {
        let resolution = match self {
            File::Library(library) => library
                .types
                .resolve(name)
                .or(library.kinds.resolve(name))
                .or(library.imports.resolve(name)),
            File::Signal(signal) => signal.types.resolve(name).or(signal.imports.resolve(name)),
            File::Sema(sema) => sema.types.resolve(name).or(sema.imports.resolve(name)),
        };
        if resolution != Resolution::Undeclared {
            return resolution;
        }
        if self.implied().contains(name) {
            return Resolution::Type(name.clone());
        }
        match Intrinsic::identify(&name.0) {
            Some(intrinsic) => Resolution::Intrinsic(intrinsic),
            None => Resolution::Undeclared,
        }
    }
}

/// The kind whose capability yields the first resolution that is not undeclared.
trait Falling {
    fn or(self, other: Resolution) -> Resolution;
}

impl Falling for Resolution {
    fn or(self, other: Resolution) -> Resolution {
        if self == Resolution::Undeclared {
            other
        } else {
            self
        }
    }
}

impl Resolving for Identity {
    fn resolve(&self, name: &Name) -> Resolution {
        let mut parameter = None;
        for (index, constraint) in self.constraints.iter().enumerate() {
            let references = match constraint {
                Constraint::One(reference) => std::slice::from_ref(reference),
                Constraint::Many(references) => references,
            };
            if references.iter().any(|reference| {
                reference.source.is_none()
                    && reference.arguments.is_empty()
                    && &reference.name == name
            }) {
                if parameter.is_some() {
                    return Resolution::Ambiguous(name.clone());
                }
                parameter = Some(index as Integer);
            }
        }
        match parameter {
            Some(index) => Resolution::Parameter(index),
            None => Resolution::Undeclared,
        }
    }
}

impl Resolving for [AssociatedType] {
    fn resolve(&self, name: &Name) -> Resolution {
        for associated in self {
            if &associated.name == name {
                return Resolution::Associated(name.clone());
            }
        }
        Resolution::Undeclared
    }
}

impl Resolving for Scope<'_> {
    fn resolve(&self, name: &Name) -> Resolution {
        let parameter = match self.identity {
            Some(identity) => identity.resolve(name),
            None => Resolution::Undeclared,
        };
        parameter
            .or(self.associated.resolve(name))
            .or(self.file.resolve(name))
    }
}

// ---------------------------------------------------------------------------
// Intrinsic arity
// ---------------------------------------------------------------------------

/// The kind whose capability yields how many arguments an intrinsic takes.
pub(crate) trait Taking {
    /// The argument count.
    fn arity(&self) -> usize;
}

impl Taking for Intrinsic {
    fn arity(&self) -> usize {
        match self {
            Intrinsic::Vector | Intrinsic::Option => 1,
            Intrinsic::Result => 2,
            Intrinsic::String
            | Intrinsic::Integer
            | Intrinsic::Decimal
            | Intrinsic::Boolean
            | Intrinsic::Meaning
            | Intrinsic::Itself
            | Intrinsic::Sized => 0,
        }
    }
}

// ---------------------------------------------------------------------------
// Naming: every name a value declares, with the path it was declared at
// ---------------------------------------------------------------------------

/// The kind whose capability lists the names a value declares, each at its path relative to the value.
trait Naming {
    /// The declared names and their paths.
    fn names(&self) -> Vec<DeclarationSite>;
}

/// A declared name and its structural path.
pub(crate) struct DeclarationSite {
    name: Name,
    path: Path,
}

impl Naming for Import {
    fn names(&self) -> Vec<DeclarationSite> {
        match self {
            Import::One(_, imported) => vec![DeclarationSite {
                name: imported.name.clone(),
                path: vec![1],
            }],
            Import::Many(_, imports) => {
                let mut names = Vec::with_capacity(imports.len());
                for (index, imported) in imports.iter().enumerate() {
                    names.push(DeclarationSite {
                        name: imported.name.clone(),
                        path: vec![1, index as Integer],
                    });
                }
                names
            }
        }
    }
}

impl Naming for TypeDeclaration {
    fn names(&self) -> Vec<DeclarationSite> {
        match self {
            TypeDeclaration::Struct(identity, _)
            | TypeDeclaration::Enum(identity, _)
            | TypeDeclaration::Alias(identity, _) => vec![DeclarationSite {
                name: identity.name.clone(),
                path: vec![0],
            }],
        }
    }
}

impl Naming for KindDeclaration {
    fn names(&self) -> Vec<DeclarationSite> {
        vec![DeclarationSite {
            name: self.identity.name.clone(),
            path: vec![0],
        }]
    }
}

impl Naming for Variant {
    fn names(&self) -> Vec<DeclarationSite> {
        match self {
            Variant::Bare(name) => vec![DeclarationSite {
                name: name.clone(),
                path: vec![],
            }],
            Variant::Typed(name, _) | Variant::Struct(name, _) | Variant::Enum(name, _) => {
                vec![DeclarationSite {
                    name: name.clone(),
                    path: vec![0],
                }]
            }
        }
    }
}

impl Naming for AssociatedType {
    fn names(&self) -> Vec<DeclarationSite> {
        vec![DeclarationSite {
            name: self.name.clone(),
            path: vec![],
        }]
    }
}

impl Naming for AssociatedConstant {
    fn names(&self) -> Vec<DeclarationSite> {
        vec![DeclarationSite {
            name: self.name.clone(),
            path: vec![0],
        }]
    }
}

impl Naming for Capability {
    fn names(&self) -> Vec<DeclarationSite> {
        vec![DeclarationSite {
            name: self.name.clone(),
            path: vec![0],
        }]
    }
}

/// The kind whose capability lists the names of every element of a section, each under its index.
trait Sectioned {
    fn names_in(&self, section: Integer) -> Vec<DeclarationSite>;
}

impl<N: Naming> Sectioned for [N] {
    fn names_in(&self, section: Integer) -> Vec<DeclarationSite> {
        let mut names = Vec::new();
        for (index, element) in self.iter().enumerate() {
            for declared in element.names() {
                let mut placed = vec![section, index as Integer];
                placed.extend(declared.path);
                names.push(DeclarationSite {
                    name: declared.name,
                    path: placed,
                });
            }
        }
        names
    }
}

/// The kind whose capability errs on the second occurrence of a name.
trait Distinct {
    fn distinct(&self) -> Result<(), Error>;
}

impl Distinct for [DeclarationSite] {
    fn distinct(&self) -> Result<(), Error> {
        for (later, declared) in self.iter().enumerate() {
            for earlier in &self[..later] {
                if earlier.name == declared.name {
                    return Err(Error::conceptual(
                        declared.path.clone(),
                        Problem::Duplicate(declared.name.0.clone()),
                    ));
                }
            }
        }
        Ok(())
    }
}

/// The kind whose capability makes sure a name can occupy a Rust declaration
/// position. `Self` remains a valid unsourced type reference, but is never a
/// declaration or imported emitted name.
trait Defining {
    fn define(&self) -> Result<(), Error>;
}

impl Defining for Name {
    fn define(&self) -> Result<(), Error> {
        if self.0 == "Self" {
            return Err(Error::conceptual(vec![], Problem::Name(self.0.clone())));
        }
        Ok(())
    }
}

/// The kind whose capability makes sure a name can be declared as a type or a
/// kind: a name a Rust declaration may take, not an intrinsic's, which a
/// declaration would shadow for every later reference, and capitalized.
trait Typing {
    fn declare(&self) -> Result<(), Error>;
}

impl Typing for Name {
    fn declare(&self) -> Result<(), Error> {
        self.define()?;
        if Intrinsic::identify(&self.0).is_some() {
            return Err(Error::conceptual(
                vec![],
                Problem::Intrinsic(self.0.clone()),
            ));
        }
        if !self.0.starts_with(|glyph: char| glyph.is_uppercase()) {
            return Err(Error::conceptual(vec![], Problem::Case(self.0.clone())));
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Inhabitation: every declared type has a finite value
// ---------------------------------------------------------------------------

/// Whether each declared type is known to have a finite value, as the file is walked to a fixpoint.
struct Inhabitation {
    inhabited: Vec<Name>,
}

/// The kind whose capability tells whether a value has a finite value, given the types known to.
trait Inhabiting {
    fn inhabited(&self, file: &File, owner: &Name, known: &Inhabitation) -> bool;
}

impl Inhabiting for Reference {
    fn inhabited(&self, file: &File, owner: &Name, known: &Inhabitation) -> bool {
        if self.source.is_some() {
            return true;
        }
        match file.resolve(&self.name) {
            Resolution::Intrinsic(Intrinsic::Itself) => known.inhabited.contains(owner),
            Resolution::Intrinsic(Intrinsic::Result) => {
                self.arguments.is_empty()
                    || self
                        .arguments
                        .iter()
                        .any(|argument| argument.inhabited(file, owner, known))
            }
            Resolution::Type(name) if file.declaration(&name).is_some() => {
                known.inhabited.contains(&name)
            }
            _ => true,
        }
    }
}

impl Inhabiting for Variant {
    fn inhabited(&self, file: &File, owner: &Name, known: &Inhabitation) -> bool {
        match self {
            Variant::Bare(name) => {
                file.declaration(name).is_none() || known.inhabited.contains(name)
            }
            Variant::Typed(_, reference) => reference.inhabited(file, owner, known),
            Variant::Struct(_, positions) => positions
                .iter()
                .all(|position| position.inhabited(file, owner, known)),
            Variant::Enum(_, variants) => {
                variants.is_empty()
                    || variants
                        .iter()
                        .any(|variant| variant.inhabited(file, owner, known))
            }
        }
    }
}

impl Inhabiting for TypeDeclaration {
    fn inhabited(&self, file: &File, owner: &Name, known: &Inhabitation) -> bool {
        match self {
            TypeDeclaration::Struct(_, positions) => positions
                .iter()
                .all(|position| position.inhabited(file, owner, known)),
            // An enum of no variants is declared empty on purpose, not by recursion.
            TypeDeclaration::Enum(_, variants) => {
                variants.is_empty()
                    || variants
                        .iter()
                        .any(|variant| variant.inhabited(file, owner, known))
            }
            TypeDeclaration::Alias(_, aliased) => aliased.inhabited(file, owner, known),
        }
    }
}

/// The kind whose capability refuses a declared type that has no finite value,
/// such as `S.{ Self }`: it reaches itself with no `Vector`, `Option` or
/// variant to stop at.
trait Finite {
    fn finite(&self) -> Result<(), Error>;
}

impl Finite for File {
    fn finite(&self) -> Result<(), Error> {
        let (declarations, section) = match self {
            File::Library(library) => (&library.types, 1),
            File::Signal(signal) => (&signal.types, 3),
            File::Sema(sema) => (&sema.types, 1),
        };
        let mut known = Inhabitation {
            inhabited: Vec::new(),
        };
        loop {
            let mut grown = false;
            for declaration in declarations {
                let name = declaration.identity().name.clone();
                if !known.inhabited.contains(&name) && declaration.inhabited(self, &name, &known) {
                    known.inhabited.push(name);
                    grown = true;
                }
            }
            if !grown {
                break;
            }
        }
        for (index, declaration) in declarations.iter().enumerate() {
            let name = &declaration.identity().name;
            if !known.inhabited.contains(name) {
                return Err(Error::conceptual(
                    vec![section, index as Integer, 0],
                    Problem::Cycle(name.0.clone()),
                ));
            }
        }
        Ok(())
    }
}

/// The kind whose capability yields a type declaration's identity.
pub(crate) trait Identified {
    fn identity(&self) -> &Identity;
}

impl Identified for TypeDeclaration {
    fn identity(&self) -> &Identity {
        match self {
            TypeDeclaration::Struct(identity, _)
            | TypeDeclaration::Enum(identity, _)
            | TypeDeclaration::Alias(identity, _) => identity,
        }
    }
}

// ---------------------------------------------------------------------------
// Checking
// ---------------------------------------------------------------------------

/// The kind whose capability checks a value in a scope, erring at a path relative to the value.
pub(crate) trait Checkable {
    /// Check the value whole.
    fn check(&self, scope: &Scope) -> Result<(), Error>;
}

/// The kind whose capabilities check enclosed children, or a section that
/// contains enclosed children.
trait Checking {
    fn check_children(&self, scope: &Scope) -> Result<(), Error>;
    fn check_each(&self, scope: &Scope, section: Integer) -> Result<(), Error>;
}

impl<C: Checkable> Checking for [C] {
    fn check_children(&self, scope: &Scope) -> Result<(), Error> {
        for (index, element) in self.iter().enumerate() {
            element.check(scope).place(index as Integer)?;
        }
        Ok(())
    }

    fn check_each(&self, scope: &Scope, section: Integer) -> Result<(), Error> {
        self.check_children(scope).place(section)
    }
}

impl Checkable for Import {
    fn check(&self, scope: &Scope) -> Result<(), Error> {
        match self {
            Import::One(_, imported) => imported.check(scope).place(1),
            Import::Many(_, imported) => imported.check_children(scope).place(1),
        }
    }
}

impl Checkable for crate::Imported {
    fn check(&self, _: &Scope) -> Result<(), Error> {
        self.name.define()?;
        self.emitted.define()?;
        Ok(())
    }
}

impl Checkable for File {
    fn check(&self, scope: &Scope) -> Result<(), Error> {
        let checked = match self {
            File::Library(library) => library.check(scope),
            File::Signal(signal) => signal.check(scope),
            File::Sema(sema) => sema.check(scope),
        };
        checked.place(1)?;
        // The derived names come first, so an authored name that would
        // capture one is the occurrence refused.
        let mut names = self.inline_sites();
        names.extend(self.declared());
        names.distinct().place(1)?;
        self.finite().place(1)
    }
}

// ---------------------------------------------------------------------------
// Inline payloads: the names derived for them, unique file-wide
// ---------------------------------------------------------------------------

/// An enum whose variants may declare inline payloads: a declared enum type,
/// or a Signal's implied `Query` or `Response`.
struct InlineRoot<'a> {
    owner: Name,
    variants: &'a [Variant],
    path: Path,
}

/// The kind whose capabilities name every inline payload of a file.
///
/// A payload a variant `X` declares in place is named `X_Data`. Where two
/// declared enums each declare an `X` in place, `X_Data` would be two
/// types, so each is named for its enum instead: `P_X_Data`, `Q_X_Data`.
/// A payload declared inside a derived enum carries that enum's name as
/// its stem: `A_Data_X_Data`.
pub(crate) trait Inlining {
    /// The name of the payload `variant` declares in the enum `enclosing`,
    /// which is the declared type `owner` itself or a payload derived under it.
    fn inline_name(&self, owner: &Name, enclosing: &Name, variant: &Name) -> Name;
    /// Every derived name, at the path of the variant that declares it.
    fn inline_sites(&self) -> Vec<DeclarationSite>;
}

/// The kind whose capability lists the enums of a file whose variants are named first-level.
trait Rooting {
    fn inline_roots(&self) -> Vec<InlineRoot<'_>>;
}

/// The kind whose capability lists the enum declarations of a section at their paths.
trait EnumSectioned {
    fn enums_in(&self, section: Integer) -> Vec<InlineRoot<'_>>;
}

impl EnumSectioned for [TypeDeclaration] {
    fn enums_in(&self, section: Integer) -> Vec<InlineRoot<'_>> {
        let mut roots = Vec::new();
        for (index, declaration) in self.iter().enumerate() {
            if let TypeDeclaration::Enum(identity, variants) = declaration {
                roots.push(InlineRoot {
                    owner: identity.name.clone(),
                    variants,
                    path: vec![section, index as Integer, 1],
                });
            }
        }
        roots
    }
}

impl Rooting for File {
    fn inline_roots(&self) -> Vec<InlineRoot<'_>> {
        match self {
            File::Library(library) => library.types.enums_in(1),
            File::Sema(sema) => sema.types.enums_in(1),
            File::Signal(signal) => {
                let mut roots = vec![
                    InlineRoot {
                        owner: Name::try_from("Query").expect("static identifier"),
                        variants: &signal.queries,
                        path: vec![1],
                    },
                    InlineRoot {
                        owner: Name::try_from("Response").expect("static identifier"),
                        variants: &signal.responses,
                        path: vec![2],
                    },
                ];
                roots.extend(signal.types.enums_in(3));
                roots
            }
        }
    }
}

/// The kind whose capability names a payload declared in place, if the variant declares one.
trait Payloading {
    fn payload(&self) -> Option<&Name>;
}

impl Payloading for Variant {
    fn payload(&self) -> Option<&Name> {
        match self {
            Variant::Struct(name, _) | Variant::Enum(name, _) => Some(name),
            Variant::Bare(_) | Variant::Typed(_, _) => None,
        }
    }
}

/// The kind whose capability walks the derived names below an enclosing enum.
trait InlineWalking {
    fn walk_inline(
        &self,
        file: &File,
        owner: &Name,
        enclosing: &Name,
        path: &Path,
        sites: &mut Vec<DeclarationSite>,
    );
}

impl InlineWalking for [Variant] {
    fn walk_inline(
        &self,
        file: &File,
        owner: &Name,
        enclosing: &Name,
        path: &Path,
        sites: &mut Vec<DeclarationSite>,
    ) {
        for (index, variant) in self.iter().enumerate() {
            let Some(name) = variant.payload() else {
                continue;
            };
            let derived = file.inline_name(owner, enclosing, name);
            let mut placed = path.clone();
            placed.push(index as Integer);
            let mut named = placed.clone();
            named.push(0);
            sites.push(DeclarationSite {
                name: derived.clone(),
                path: named,
            });
            if let Variant::Enum(_, nested) = variant {
                placed.push(1);
                nested.walk_inline(file, owner, &derived, &placed, sites);
            }
        }
    }
}

impl Inlining for File {
    fn inline_name(&self, owner: &Name, enclosing: &Name, variant: &Name) -> Name {
        let name = if enclosing != owner {
            format!("{}_{}_Data", enclosing.0, variant.0)
        } else {
            let mut declaring = 0;
            for root in self.inline_roots() {
                for other in root.variants {
                    if other.payload() == Some(variant) {
                        declaring += 1;
                    }
                }
            }
            if declaring > 1 {
                format!("{}_{}_Data", owner.0, variant.0)
            } else {
                format!("{}_Data", variant.0)
            }
        };
        Name::try_from(name).expect("derived inline identifiers are identifiers")
    }

    fn inline_sites(&self) -> Vec<DeclarationSite> {
        let mut sites = Vec::new();
        for root in self.inline_roots() {
            root.variants
                .walk_inline(self, &root.owner, &root.owner, &root.path, &mut sites);
        }
        sites
    }
}

/// The kind whose capability lists every name a file root declares by
/// authorship: its imports, its types and kinds, and the types it implies.
trait Declared {
    fn declared(&self) -> Vec<DeclarationSite>;
}

impl Declared for crate::Library {
    fn declared(&self) -> Vec<DeclarationSite> {
        let mut names = self.imports.names_in(0);
        names.extend(self.types.names_in(1));
        names.extend(self.kinds.names_in(2));
        names
    }
}

impl Declared for Signal {
    fn declared(&self) -> Vec<DeclarationSite> {
        let mut names = vec![
            DeclarationSite {
                name: Name::try_from("Query").expect("static identifier"),
                path: vec![1],
            },
            DeclarationSite {
                name: Name::try_from("Response").expect("static identifier"),
                path: vec![2],
            },
        ];
        names.extend(self.imports.names_in(0));
        names.extend(self.types.names_in(3));
        names
    }
}

impl Declared for Sema {
    fn declared(&self) -> Vec<DeclarationSite> {
        let mut names = self.imports.names_in(0);
        names.extend(self.types.names_in(1));
        names
    }
}

impl Declared for File {
    fn declared(&self) -> Vec<DeclarationSite> {
        match self {
            File::Library(library) => library.declared(),
            File::Signal(signal) => signal.declared(),
            File::Sema(sema) => sema.declared(),
        }
    }
}

impl Checkable for crate::Library {
    fn check(&self, scope: &Scope) -> Result<(), Error> {
        if self.types.len() > TYPE_DECLARATION_LIMIT {
            return Err(Error::conceptual(vec![1], Problem::Depth));
        }
        self.declared().distinct()?;
        self.imports.check_each(scope, 0)?;
        self.types.check_each(scope, 1)?;
        self.kinds.check_each(scope, 2)?;
        self.associations.check_each(scope, 3)
    }
}

impl Checkable for Signal {
    fn check(&self, scope: &Scope) -> Result<(), Error> {
        self.declared().distinct()?;
        self.imports.check_each(scope, 0)?;
        self.queries.names_in(1).distinct()?;
        self.responses.names_in(2).distinct()?;
        self.queries.check_each(scope, 1)?;
        self.responses.check_each(scope, 2)?;
        self.types.check_each(scope, 3)
    }
}

impl Checkable for Sema {
    fn check(&self, scope: &Scope) -> Result<(), Error> {
        self.declared().distinct()?;
        self.imports.check_each(scope, 0)?;
        self.types.check_each(scope, 1)
    }
}

impl Checkable for Identity {
    fn check(&self, scope: &Scope) -> Result<(), Error> {
        self.name.define()?;
        if self.constraints.len() > 26 {
            return Err(Error::conceptual(
                vec![],
                Problem::arity(26, self.constraints.len() as Integer),
            ));
        }
        let mut parameters: Vec<&Name> = Vec::new();
        for (index, constraint) in self.constraints.iter().enumerate() {
            if let Constraint::One(reference) = constraint
                && reference.source.is_none()
                && reference.arguments.is_empty()
            {
                if parameters.contains(&&reference.name) {
                    return Err(Error::conceptual(
                        vec![index as Integer],
                        Problem::Duplicate(reference.name.0.clone()),
                    ));
                }
                parameters.push(&reference.name);
            }
            constraint.check(scope).place(index as Integer)?;
        }
        Ok(())
    }
}

impl Checkable for Constraint {
    fn check(&self, scope: &Scope) -> Result<(), Error> {
        match self {
            Constraint::One(reference) => reference.refer(scope, Role::Kind),
            Constraint::Many(references) => {
                if references.is_empty() {
                    return Err(Error::conceptual(vec![], Problem::Empty));
                }
                for (index, reference) in references.iter().enumerate() {
                    reference.refer(scope, Role::Kind).place(index as Integer)?;
                }
                Ok(())
            }
        }
    }
}

/// The kind whose capability yields whether an intrinsic is a type or a kind.
trait Roled {
    fn role(&self) -> Role;
}

impl Roled for Intrinsic {
    fn role(&self) -> Role {
        match self {
            Intrinsic::Sized => Role::Kind,
            Intrinsic::String
            | Intrinsic::Integer
            | Intrinsic::Decimal
            | Intrinsic::Boolean
            | Intrinsic::Meaning
            | Intrinsic::Vector
            | Intrinsic::Option
            | Intrinsic::Result
            | Intrinsic::Itself => Role::Type,
        }
    }
}

/// The kind whose capability checks a reference in the role its position gives it.
pub(crate) trait Referring {
    /// Check the reference as a type or as a kind.
    fn refer(&self, scope: &Scope, role: Role) -> Result<(), Error>;
}

/// The role and optional argument count a resolved reference requires.
struct ReferenceRequirement {
    role: Role,
    arity: Option<usize>,
}

/// The kind whose capability checks every reference of a section in one role.
trait ReferringEach {
    fn refer_each(&self, scope: &Scope, role: Role, section: Integer) -> Result<(), Error>;
}

impl ReferringEach for [Reference] {
    fn refer_each(&self, scope: &Scope, role: Role, section: Integer) -> Result<(), Error> {
        for (index, reference) in self.iter().enumerate() {
            reference
                .refer(scope, role)
                .place(index as Integer)
                .place(section)?;
        }
        Ok(())
    }
}

impl Referring for Reference {
    fn refer(&self, scope: &Scope, role: Role) -> Result<(), Error> {
        if self.source.is_some() && self.name.0 == "Self" {
            return Err(Error::conceptual(
                vec![],
                Problem::Name(self.name.0.clone()),
            ));
        }
        // A direct Protos intrinsic has the same contract whether it arrives
        // through an import or an explicit qualification. Other sources own
        // their declaration metadata.
        if self
            .source
            .as_ref()
            .is_some_and(|source| source.as_ref() == "protos")
            && let Some(intrinsic) = Intrinsic::identify(&self.name.0)
        {
            if intrinsic.role() != role {
                return Err(Error::conceptual(
                    vec![],
                    Problem::Role(self.name.0.clone()),
                ));
            }
            if intrinsic.arity() != self.arguments.len() {
                return Err(Error::conceptual(
                    vec![],
                    Problem::arity(
                        intrinsic.arity() as Integer,
                        self.arguments.len() as Integer,
                    ),
                ));
            }
        }
        // A sourced reference carries the foreign name in the headed body's
        // structural child at index one.
        if self.source.is_none() {
            let requirement = match scope.resolve(&self.name) {
                Resolution::Ambiguous(name) => {
                    return Err(Error::conceptual(vec![], Problem::Duplicate(name.0)));
                }
                Resolution::Undeclared => {
                    return Err(Error::conceptual(
                        vec![],
                        Problem::Undeclared(self.name.0.clone()),
                    ));
                }
                Resolution::Intrinsic(intrinsic) => ReferenceRequirement {
                    role: intrinsic.role(),
                    arity: Some(intrinsic.arity()),
                },
                Resolution::Parameter(_) | Resolution::Associated(_) => ReferenceRequirement {
                    role: Role::Type,
                    arity: Some(0),
                },
                Resolution::Type(name) => {
                    let arity =
                        scope
                            .file
                            .declaration(&name)
                            .map(|declaration| match declaration {
                                TypeDeclaration::Struct(identity, _)
                                | TypeDeclaration::Enum(identity, _)
                                | TypeDeclaration::Alias(identity, _) => identity.constraints.len(),
                            });
                    ReferenceRequirement {
                        role: Role::Type,
                        arity,
                    }
                }
                Resolution::Kind(_) => ReferenceRequirement {
                    role: Role::Kind,
                    arity: None,
                },
                Resolution::Imported(source, emitted)
                    if source.as_ref() == "protos" && emitted == self.name =>
                {
                    match Intrinsic::identify(&self.name.0) {
                        Some(intrinsic) => ReferenceRequirement {
                            role: intrinsic.role(),
                            arity: Some(intrinsic.arity()),
                        },
                        None => ReferenceRequirement { role, arity: None },
                    }
                }
                Resolution::Imported(_, _) => ReferenceRequirement { role, arity: None },
            };
            if requirement.role != role {
                return Err(Error::conceptual(
                    vec![],
                    Problem::Role(self.name.0.clone()),
                ));
            }
            if let Some(expected) = requirement.arity
                && expected != self.arguments.len()
            {
                return Err(Error::conceptual(
                    vec![],
                    Problem::arity(expected as Integer, self.arguments.len() as Integer),
                ));
            }
        }
        for (index, argument) in self.arguments.iter().enumerate() {
            let checked = argument.refer(scope, Role::Type).place(index as Integer);
            match self.source {
                Some(_) => checked.place(1)?,
                None => checked?,
            }
        }
        Ok(())
    }
}

impl Checkable for Reference {
    fn check(&self, scope: &Scope) -> Result<(), Error> {
        self.refer(scope, Role::Type)
    }
}

/// The kind whose capability tells whether an alias reaches a name through aliases and intrinsic containers alone.
trait Cycling {
    fn cycles(&self, target: &Name, file: &File, visited: &mut Vec<Name>) -> bool;
    fn cycles_substituting(
        &self,
        target: &Name,
        file: &File,
        visited: &mut Vec<Name>,
        application: AliasApplication<'_>,
    ) -> bool;
}

/// The identity and actual arguments of an alias while its body is followed.
/// Parameter references in that body are replaced before cycle detection.
#[derive(Clone, Copy)]
struct AliasApplication<'a> {
    identity: &'a Identity,
    arguments: &'a [Reference],
}

impl Cycling for Reference {
    fn cycles(&self, target: &Name, file: &File, visited: &mut Vec<Name>) -> bool {
        if self.source.is_some() {
            return self
                .arguments
                .iter()
                .any(|argument| argument.cycles(target, file, visited));
        }
        if &self.name == target {
            return true;
        }
        match file.resolve(&self.name) {
            Resolution::Type(name) => {
                if visited.contains(&name) {
                    return false;
                }
                visited.push(name.clone());
                match file.declaration(&name) {
                    Some(TypeDeclaration::Alias(identity, aliased)) => aliased.cycles_substituting(
                        target,
                        file,
                        visited,
                        AliasApplication {
                            identity,
                            arguments: &self.arguments,
                        },
                    ),
                    _ => false,
                }
            }
            Resolution::Intrinsic(_) | Resolution::Imported(_, _) => {
                for argument in &self.arguments {
                    if argument.cycles(target, file, visited) {
                        return true;
                    }
                }
                false
            }
            Resolution::Kind(_)
            | Resolution::Parameter(_)
            | Resolution::Associated(_)
            | Resolution::Ambiguous(_)
            | Resolution::Undeclared => false,
        }
    }

    fn cycles_substituting(
        &self,
        target: &Name,
        file: &File,
        visited: &mut Vec<Name>,
        application: AliasApplication<'_>,
    ) -> bool {
        if self.source.is_none()
            && self.arguments.is_empty()
            && let Resolution::Parameter(index) = application.identity.resolve(&self.name)
        {
            return application.arguments[index as usize].cycles(target, file, visited);
        }
        if self.source.is_some() {
            return self
                .arguments
                .iter()
                .any(|argument| argument.cycles_substituting(target, file, visited, application));
        }
        if &self.name == target {
            return true;
        }
        match file.resolve(&self.name) {
            Resolution::Type(name) => {
                if visited.contains(&name) {
                    return false;
                }
                visited.push(name.clone());
                match file.declaration(&name) {
                    Some(TypeDeclaration::Alias(identity, aliased)) => aliased.cycles_substituting(
                        target,
                        file,
                        visited,
                        AliasApplication {
                            identity,
                            arguments: &self.arguments,
                        },
                    ),
                    _ => false,
                }
            }
            Resolution::Intrinsic(_) | Resolution::Imported(_, _) => self
                .arguments
                .iter()
                .any(|argument| argument.cycles_substituting(target, file, visited, application)),
            Resolution::Kind(_)
            | Resolution::Parameter(_)
            | Resolution::Associated(_)
            | Resolution::Ambiguous(_)
            | Resolution::Undeclared => false,
        }
    }
}

/// The kind whose capability finds the declaration of a name in a file.
pub(crate) trait Declaring {
    /// The type declaration named, if the file declares one.
    fn declaration(&self, name: &Name) -> Option<&TypeDeclaration>;
}

impl Declaring for File {
    fn declaration(&self, name: &Name) -> Option<&TypeDeclaration> {
        let declarations = match self {
            File::Library(library) => &library.types,
            File::Signal(signal) => &signal.types,
            File::Sema(sema) => &sema.types,
        };
        for declaration in declarations {
            if declaration.resolve(name) != Resolution::Undeclared {
                return Some(declaration);
            }
        }
        None
    }
}

/// The kind whose capability finds a kind declaration in a kinds file.
trait KindDeclaring {
    fn kind_declaration(&self, name: &Name) -> Option<&KindDeclaration>;
}

impl KindDeclaring for File {
    fn kind_declaration(&self, name: &Name) -> Option<&KindDeclaration> {
        let File::Library(library) = self else {
            return None;
        };
        library
            .kinds
            .iter()
            .find(|declaration| declaration.identity.name == *name)
    }
}

/// The kind whose capability finds an indirect superkind cycle.
trait Supercycling {
    fn reaches_superkind(&self, target: &Name, file: &File, visited: &mut Vec<Name>) -> bool;
}

impl Supercycling for Reference {
    fn reaches_superkind(&self, target: &Name, file: &File, visited: &mut Vec<Name>) -> bool {
        if self.source.is_some() {
            return false;
        }
        if &self.name == target {
            return true;
        }
        if visited.contains(&self.name) {
            return false;
        }
        let Some(declaration) = file.kind_declaration(&self.name) else {
            return false;
        };
        let KindBody::Complex { superkinds, .. } = &declaration.body else {
            return false;
        };
        visited.push(self.name.clone());
        for superkind in superkinds {
            if superkind.reaches_superkind(target, file, visited) {
                return true;
            }
        }
        false
    }
}

impl Checkable for TypeDeclaration {
    fn check(&self, scope: &Scope) -> Result<(), Error> {
        let identity = match self {
            TypeDeclaration::Struct(identity, _)
            | TypeDeclaration::Enum(identity, _)
            | TypeDeclaration::Alias(identity, _) => identity,
        };
        if !identity.constraints.is_empty() {
            return Err(Error::conceptual(
                vec![0],
                Problem::Expected(crate::Form::Declaration),
            ));
        }
        identity.name.declare().place(0)?;
        identity.check(scope).place(0)?;
        let inner = Scope {
            file: scope.file,
            identity: Some(identity),
            associated: scope.associated,
        };
        match self {
            TypeDeclaration::Struct(_, positions) => positions.check_children(&inner).place(1),
            TypeDeclaration::Enum(_, variants) => {
                let mut names = variants.names_in(0);
                for declared in &mut names {
                    declared.path.remove(0);
                    declared.path.insert(0, 1);
                }
                names.distinct()?;
                variants.check_children(&inner).place(1)
            }
            TypeDeclaration::Alias(identity, aliased) => {
                aliased.check(&inner).place(1)?;
                if aliased.cycles(&identity.name, scope.file, &mut vec![]) {
                    return Err(Error::conceptual(
                        vec![0],
                        Problem::Cycle(identity.name.0.clone()),
                    ));
                }
                Ok(())
            }
        }
    }
}

impl Checkable for Variant {
    fn check(&self, scope: &Scope) -> Result<(), Error> {
        match self {
            Variant::Bare(name) => name.define(),
            Variant::Typed(name, reference) => {
                name.define().place(0)?;
                reference.check(scope).place(1)
            }
            Variant::Struct(name, positions) => {
                name.define().place(0)?;
                positions.check_children(scope).place(1)
            }
            Variant::Enum(name, variants) => {
                name.define().place(0)?;
                let mut names = variants.names_in(0);
                for declared in &mut names {
                    declared.path.remove(0);
                    declared.path.insert(0, 1);
                }
                names.distinct()?;
                variants.check_children(scope).place(1)
            }
        }
    }
}

impl Checkable for KindDeclaration {
    fn check(&self, scope: &Scope) -> Result<(), Error> {
        self.identity.name.declare().place(0)?;
        self.identity.check(scope).place(0)?;
        let inner = Scope {
            file: scope.file,
            identity: Some(&self.identity),
            associated: match &self.body {
                KindBody::Simple(_) => &[],
                KindBody::Complex { types, .. } => types,
            },
        };
        match &self.body {
            KindBody::Simple(capabilities) => {
                let mut names = capabilities.names_in(0);
                for declared in &mut names {
                    declared.path.remove(0);
                    declared.path.insert(0, 1);
                }
                names.distinct()?;
                capabilities.check_children(&inner).place(1)
            }
            KindBody::Complex {
                superkinds,
                types,
                constants,
                capabilities,
            } => {
                for (index, superkind) in superkinds.iter().enumerate() {
                    if superkind.reaches_superkind(
                        &self.identity.name,
                        scope.file,
                        &mut vec![self.identity.name.clone()],
                    ) {
                        return Err(Error::conceptual(
                            vec![1, 0, index as Integer],
                            Problem::Cycle(self.identity.name.0.clone()),
                        ));
                    }
                }
                let mut names = types.names_in(1);
                names.extend(constants.names_in(2));
                names.extend(capabilities.names_in(3));
                for declared in &mut names {
                    declared.path.insert(0, 1);
                }
                names.distinct()?;
                superkinds.refer_each(&inner, Role::Kind, 0).place(1)?;
                types.check_each(&inner, 1).place(1)?;
                constants.check_each(&inner, 2).place(1)?;
                capabilities.check_each(&inner, 3).place(1)
            }
        }
    }
}

impl Checkable for AssociatedType {
    fn check(&self, scope: &Scope) -> Result<(), Error> {
        self.name.declare()?;
        for (index, bound) in self.bounds.iter().enumerate() {
            bound.refer(scope, Role::Kind).place(index as Integer)?;
        }
        Ok(())
    }
}

impl Checkable for AssociatedConstant {
    fn check(&self, scope: &Scope) -> Result<(), Error> {
        self.name.define().place(0)?;
        if self.name.0 != self.name.0.to_uppercase() {
            return Err(Error::conceptual(
                vec![],
                Problem::Name(self.name.0.clone()),
            ));
        }
        self.ty.check(scope).place(1)
    }
}

impl Checkable for Capability {
    fn check(&self, scope: &Scope) -> Result<(), Error> {
        self.name.define().place(0)?;
        match &self.signature {
            Signature::Yielding(yields) => yields.check(scope).place(0).place(1),
            Signature::Taking(inputs, yields) => {
                inputs.check_each(scope, 0).place(0).place(1)?;
                yields.check(scope).place(0).place(1).place(1)
            }
        }
    }
}

impl Checkable for Association {
    fn check(&self, scope: &Scope) -> Result<(), Error> {
        if scope.resolve(&self.identity.name) == Resolution::Undeclared {
            return Err(Error::conceptual(
                vec![],
                Problem::Undeclared(self.identity.name.0.clone()),
            ));
        }
        self.identity.check(scope).place(0)?;
        // The kinds borne are named outside the identity that bears them.
        self.kinds.refer_each(scope, Role::Kind, 0).place(1)
    }
}
