//! Conforming (proposal): a datom text checked against a type an ethos file declares.
//!
//! The generated Rust is not compiled for this: the declaration is walked
//! instead, by the rules the generated derives compose by. A struct is a
//! brace of its positions; an enum variant carrying nothing is its head
//! alone, one carrying a type is `Head.body`, one declaring a struct or an
//! enum in place carries that payload; an alias is its aliased type. The
//! intrinsic scalars are composed by datom-codec itself (`String`,
//! `Integer`, `Decimal`, `Boolean`, `Meaning`); `Vector`, `Option` and
//! `Result` by their forms. A type the file imports
//! is read from the sibling ethos files given (`conform_among`): a source's
//! own `<source>.ethos` first, then any sibling declaring the name. One no
//! sibling declares, or a parameter, cannot be checked here and is refused
//! as a `Form` error naming it.

use datom_codec::{
    Budget, Composable, Datom, DatomForming, Decimal, Error, ErrorKind, ErrorLayer, Form, Meaning,
    Naming as _,
};
use protos::BoundedProtosizable;

use crate::checking::Declaring;
use crate::sectioning::{Referencing, Sectioning};
use crate::{
    File, Identifiable, Intrinsic, Name, Position, Reference, Resolution, Resolving,
    TypeDeclaration, Variant,
};

/// The trait whose capability checks a datom text against a declared type.
pub trait Conforming {
    /// The datom read from `text` if it is a value of the type `name`, or
    /// the datom error, its layer, path and kind, if it is not.
    fn conform(&self, name: &str, text: &str) -> Result<Datom, Error>;
    /// The same, an imported type read from the sibling files given, each
    /// with its file stem: a source's own `<source>.ethos` first, then any
    /// sibling that declares the name.
    fn conform_among(
        &self,
        siblings: &[(String, File)],
        name: &str,
        text: &str,
    ) -> Result<Datom, Error>;
}

/// A file being conformed to, and the sibling files its imports resolve in.
struct Context<'a> {
    file: &'a File,
    siblings: &'a [(String, File)],
}

/// The trait whose capability finds the sibling file declaring an imported name.
trait Importing {
    fn imported(&self, source: &crate::Source, name: &Name) -> Option<&File>;
}

impl Importing for Context<'_> {
    fn imported(&self, source: &crate::Source, name: &Name) -> Option<&File> {
        let stem = source.as_ref().rsplit("::").next().unwrap_or_default();
        let mut found = None;
        for (sibling, file) in self.siblings {
            let declares = file.declaration(name).is_some()
                || file
                    .implied_enums()
                    .iter()
                    .any(|implied| &implied.name == name);
            if !declares {
                continue;
            }
            if sibling == stem {
                return Some(file);
            }
            if found.is_none() {
                found = Some(file);
            }
        }
        found
    }
}

/// The trait whose capabilities check one datom against one shape.
trait Shaping {
    fn reference(
        &self,
        reference: &Reference,
        datom: &Datom,
        budget: &mut Budget,
    ) -> Result<(), Error>;
    fn named(&self, name: &Name, datom: &Datom, budget: &mut Budget) -> Result<(), Error>;
    fn positions(
        &self,
        positions: &[Position],
        datom: &Datom,
        budget: &mut Budget,
    ) -> Result<(), Error>;
    fn variants(
        &self,
        owner: &Name,
        variants: &[Variant],
        datom: &Datom,
        budget: &mut Budget,
    ) -> Result<(), Error>;
}

/// The trait whose capability raises a composition error at a datom.
trait Refusing {
    fn refuse(&self, kind: ErrorKind) -> Error;
    fn form(&self, expected: &str) -> Error;
}

impl Refusing for Datom {
    fn refuse(&self, kind: ErrorKind) -> Error {
        Error {
            layer: ErrorLayer::Composition,
            path: self.path.clone(),
            kind,
        }
    }

    fn form(&self, expected: &str) -> Error {
        self.refuse(ErrorKind::Form {
            expected: expected.to_owned(),
            found: self.form.form_name().to_owned(),
        })
    }
}

impl Conforming for File {
    fn conform(&self, name: &str, text: &str) -> Result<Datom, Error> {
        self.conform_among(&[], name, text)
    }

    fn conform_among(
        &self,
        siblings: &[(String, File)],
        name: &str,
        text: &str,
    ) -> Result<Datom, Error> {
        let mut budget = Budget {
            remaining: 4_096,
            reader: protos::ReaderBudget { remaining: 4_096 },
            depth: 0,
            maximum_depth: 4_096,
        };
        let protos = text
            .protosize_with(&mut budget.reader)
            .map_err(|error| Error {
                layer: ErrorLayer::Protos,
                path: vec![],
                kind: ErrorKind::Structural(error),
            })?;
        let datom = protos.datom_form(vec![])?;
        let Ok(name) = Name::try_from(name) else {
            return Err(datom.refuse(ErrorKind::Variant {
                expected: "a type name".to_owned(),
                found: name.to_owned(),
            }));
        };
        Context {
            file: self,
            siblings,
        }
        .named(&name, &datom, &mut budget)?;
        Ok(datom)
    }
}

impl Shaping for Context<'_> {
    fn reference(
        &self,
        reference: &Reference,
        datom: &Datom,
        budget: &mut Budget,
    ) -> Result<(), Error> {
        // `protos:String` and an imported `protos:[ String … ]` name the
        // intrinsic, as the checker reads them.
        let resolution = match (&reference.source, self.file.resolve(&reference.name)) {
            (Some(source), _) if source.as_ref() == "protos" => {
                match Intrinsic::identify(reference.name.as_ref()) {
                    Some(intrinsic) => Resolution::Intrinsic(intrinsic),
                    None => Resolution::Undeclared,
                }
            }
            (None, Resolution::Imported(source, emitted)) if source.as_ref() == "protos" => {
                match Intrinsic::identify(emitted.as_ref()) {
                    Some(intrinsic) => Resolution::Intrinsic(intrinsic),
                    None => Resolution::Imported(source, emitted),
                }
            }
            (_, resolution) => resolution,
        };
        if let Some(source) = &reference.source
            && source.as_ref() != "protos"
        {
            return match self.imported(source, &reference.name) {
                Some(file) => Context {
                    file,
                    siblings: self.siblings,
                }
                .named(&reference.name, datom, budget),
                None => Err(datom.form(reference.name.as_ref())),
            };
        }
        let argument = reference.arguments.first();
        match (resolution, argument) {
            (Resolution::Intrinsic(Intrinsic::String), _) => {
                datom.compose::<String>(budget).map(drop)
            }
            (Resolution::Intrinsic(Intrinsic::Integer), _) => {
                datom.compose::<i64>(budget).map(drop)
            }
            (Resolution::Intrinsic(Intrinsic::Decimal), _) => {
                datom.compose::<Decimal>(budget).map(drop)
            }
            (Resolution::Intrinsic(Intrinsic::Boolean), _) => {
                datom.compose::<bool>(budget).map(drop)
            }
            (Resolution::Intrinsic(Intrinsic::Meaning), _) => {
                datom.compose::<Meaning>(budget).map(drop)
            }
            (Resolution::Intrinsic(Intrinsic::Vector), Some(element)) => {
                let Form::Vector(children) = &datom.form else {
                    return Err(datom.form("Vector"));
                };
                for child in children {
                    self.reference(element, child, budget)?;
                }
                Ok(())
            }
            (Resolution::Intrinsic(Intrinsic::Option), Some(carried)) => match &datom.form {
                Form::Bare(head) if head == "None" => Ok(()),
                Form::Variant(head, body) if head.0 == "Some" => {
                    self.reference(carried, body, budget)
                }
                Form::Variant(head, _) => Err(datom.refuse(ErrorKind::Variant {
                    expected: "Option".to_owned(),
                    found: head.0.clone(),
                })),
                _ => Err(datom.form("Variant")),
            },
            (Resolution::Intrinsic(Intrinsic::Result), Some(success)) => match &datom.form {
                Form::Variant(head, body) if head.0 == "Ok" => {
                    self.reference(success, body, budget)
                }
                Form::Variant(head, body) if head.0 == "Err" => match reference.arguments.get(1) {
                    Some(failure) => self.reference(failure, body, budget),
                    None => Err(datom.form("Result")),
                },
                Form::Variant(head, _) => Err(datom.refuse(ErrorKind::Variant {
                    expected: "Result".to_owned(),
                    found: head.0.clone(),
                })),
                _ => Err(datom.form("Variant")),
            },
            (Resolution::Type(name), _) => self.named(&name, datom, budget),
            (Resolution::Imported(source, emitted), _) => match self.imported(&source, &emitted) {
                Some(file) => Context {
                    file,
                    siblings: self.siblings,
                }
                .named(&emitted, datom, budget),
                None => Err(datom.form(reference.name.as_ref())),
            },
            _ => Err(datom.form(reference.name.as_ref())),
        }
    }

    fn named(&self, name: &Name, datom: &Datom, budget: &mut Budget) -> Result<(), Error> {
        for implied in self.file.implied_enums() {
            if &implied.name == name {
                return self.variants(name, implied.variants, datom, budget);
            }
        }
        match self.file.declaration(name) {
            Some(TypeDeclaration::Struct(_, positions) | TypeDeclaration::Sealed(_, positions)) => {
                self.positions(positions, datom, budget)
            }
            Some(TypeDeclaration::Enum(_, variants)) => {
                self.variants(name, variants, datom, budget)
            }
            Some(TypeDeclaration::Alias(_, aliased)) => self.reference(aliased, datom, budget),
            None => Err(datom.form(name.as_ref())),
        }
    }

    fn positions(
        &self,
        positions: &[Position],
        datom: &Datom,
        budget: &mut Budget,
    ) -> Result<(), Error> {
        let Form::Struct(children) = &datom.form else {
            return Err(datom.form("Struct"));
        };
        if children.len() != positions.len() {
            return Err(datom.refuse(ErrorKind::Arity {
                expected: positions.len() as datom_codec::Integer,
                found: children.len() as datom_codec::Integer,
            }));
        }
        for (position, child) in positions.iter().zip(children) {
            self.reference(&position.reference(), child, budget)?;
        }
        Ok(())
    }

    fn variants(
        &self,
        owner: &Name,
        variants: &[Variant],
        datom: &Datom,
        budget: &mut Budget,
    ) -> Result<(), Error> {
        let (head, body) = match &datom.form {
            Form::Bare(head) => (head.as_str(), None),
            Form::Variant(head, body) => (head.0.as_str(), Some(body.as_ref())),
            _ => return Err(datom.form("Variant")),
        };
        for variant in variants {
            match (variant, body) {
                (Variant::Bare(name), None)
                    if name.as_ref() == head && self.file.declaration(name).is_none() =>
                {
                    return Ok(());
                }
                // A bare variant naming a declared type carries that type.
                (Variant::Bare(name), Some(body)) if name.as_ref() == head => {
                    return self.named(name, body, budget);
                }
                (Variant::Typed(name, reference), Some(body)) if name.as_ref() == head => {
                    return self.reference(reference, body, budget);
                }
                (Variant::Struct(name, positions), Some(body)) if name.as_ref() == head => {
                    return self.positions(positions, body, budget);
                }
                (Variant::Enum(name, nested), Some(body)) if name.as_ref() == head => {
                    return self.variants(name, nested, body, budget);
                }
                _ => {}
            }
        }
        Err(datom.refuse(ErrorKind::Variant {
            expected: owner.as_ref().to_owned(),
            found: head.to_owned(),
        }))
    }
}
