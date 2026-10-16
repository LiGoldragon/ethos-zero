//! The proposed syntax this branch accepts, each example as proposed.
//!
//! 1. Any whitespace after a separator before its body.
//! 2. An ethos file is an ordinary datom: `Library.` then the braced sections.
//! 3. The type ethos: `Type.` carrying one declaration.
//! 4. A sealed type: `Name.Sealed.{ … }`, built only by its trait's `:` constructor.
//! 5. `⟦ T ⟧` is `Vector<T>` in a type position.

use ethos_zero::{
    Actualizing, Conforming, Error, File, Generating, Potential, Printable, Problem,
    TypeDeclaration,
};

fn read(source: &str) -> File {
    match Potential::<File>::from(source).actualize() {
        Ok(file) => file,
        Err(error) => panic!("source did not read: {source}\n{error:?}"),
    }
}

fn generated(source: &str) -> String {
    match read(source).generate() {
        Ok(rust) => rust,
        Err(error) => panic!("source did not generate: {source}\n{error:?}"),
    }
}

fn refusal(source: &str) -> (Vec<i64>, Problem) {
    let refused = match Potential::<File>::from(source).actualize() {
        Ok(file) => file.generate().map(drop),
        Err(error) => Err(error),
    };
    match refused {
        Err(Error::Conceptual(data)) => (data.integer_vector, data.problem),
        Err(Error::Structural(error)) => panic!("{source} refused structurally: {error:?}"),
        Ok(()) => panic!("{source} must be refused"),
    }
}

fn fixture(name: &str) -> String {
    let root = env!("CARGO_MANIFEST_DIR");
    std::fs::read_to_string(format!("{root}/fixtures/{name}.ethos")).expect(name)
}

// 1. Whitespace after a separator ------------------------------------------

#[test]
fn whitespace_after_a_separator_parses_in_datom() {
    let signal = read(&fixture("orchestrate"));
    let text = "Observed.\n   Locks.\n      []";
    let datom = match signal.conform("Response", text) {
        Ok(datom) => datom,
        Err(error) => panic!("{text} conforms: {error:?}"),
    };
    let same = signal
        .conform("Response", "Observed.Locks.[]")
        .expect("the tight form conforms");
    assert_eq!(datom.form, same.form);
}

#[test]
fn whitespace_after_any_separator_reads_in_ethos() {
    let source = "Library.\n  {\n  []\n  [ Record.\n      { String Integer }\n    State.\t[ Idle Busy ] ]\n  [ Fillable.[ fill!\n      [ Integer ] ] ]\n  [] }";
    let rust = generated(source);
    assert!(rust.contains("pub struct Record"), "{rust}");
    assert!(rust.contains("pub enum State"), "{rust}");
    assert!(rust.contains("fn fill(&mut self) -> i64"), "{rust}");
}

// 2. A file is an ordinary datom ---------------------------------------------

#[test]
fn a_library_is_its_root_the_dot_and_the_braced_sections() {
    let source = "Library.\n{\n[ protos:String ]\n[ Record.{ String Integer } ]\n[]\n[] }";
    let file = read(source);
    assert!(matches!(file, File::Library(_)));
    let rust = generated(source);
    assert!(rust.contains("pub struct Record"), "{rust}");
}

#[test]
fn the_sweet_form_no_longer_reads() {
    let sweet = "Library\n[]\n[ Record.{ String } ]\n[]\n[]";
    assert!(matches!(
        Potential::<File>::from(sweet).actualize(),
        Err(Error::Structural(_))
    ));
}

#[test]
fn a_file_prints_with_its_brace_on_the_next_line_and_sections_unindented() {
    let file = read("Library.{ [] [ Record.{ String Integer } ] [] [] }");
    assert_eq!(
        file.print(),
        "Library.\n{\n[]\n[ Record.{ String Integer } ]\n[]\n[] }\n"
    );
    assert_eq!(read(&file.print()), file);
}

// 3. The type ethos ----------------------------------------------------------

#[test]
fn a_type_file_carries_one_declaration() {
    let source = "Type.CommitMessage.[ Landing.{ String Integer } Fix.String ]";
    let file = read(source);
    let File::Type(TypeDeclaration::Enum(identity, variants)) = &file else {
        panic!("a type file of an enum, not {file:?}")
    };
    assert_eq!(identity.name.as_ref(), "CommitMessage");
    assert_eq!(variants.len(), 2);
    let rust = generated(source);
    assert!(rust.contains("pub enum CommitMessage"), "{rust}");
    assert!(rust.contains("Landing(Landing_Data)"), "{rust}");
    assert!(rust.contains("Fix(String)"), "{rust}");
    assert!(
        file.print()
            .starts_with("Type.CommitMessage.[ Landing.{ String Integer }")
    );
    assert_eq!(read(&file.print()), file);
    let datom = file
        .conform("CommitMessage", "Landing.{ «a landing» 42 }")
        .expect("a landing conforms");
    assert!(matches!(datom.form, datom_codec::Form::Variant(..)));
}

#[test]
fn a_type_files_enum_needs_two_variants() {
    assert_eq!(
        refusal("Type.Lonely.[ Only ]"),
        (
            vec![1, 1],
            Problem::Arity(ethos_zero::Arity_Data {
                first_integer: 2,
                second_integer: 1
            })
        )
    );
}

#[test]
fn a_type_file_may_be_a_struct_or_an_alias_and_knows_no_other_type() {
    assert!(generated("Type.Point.{ Integer Integer }").contains("pub struct Point"));
    assert!(generated("Type.Names.⟦ String ⟧").contains("pub type Names = std::vec::Vec<String>"));
    assert_eq!(
        refusal("Type.Holder.{ Other }"),
        (vec![1, 1, 0], Problem::Undeclared("Other".to_owned()))
    );
}

// 4. A sealed type -----------------------------------------------------------

const SEALED: &str = "Library.\n{\n[]\n[ VerifiedContext.Sealed.{ String Integer }\n  Rejection.[ Empty Malformed.String ] ]\n[ Verifying.[ verify:[ Result<Self Rejection> ] ] ]\n[ VerifiedContext.[ Verifying ]\n  Rejection.[ Verifying ] ] }";

#[test]
fn a_sealed_type_keeps_its_positions_from_other_modules() {
    let rust = generated(SEALED);
    assert!(rust.contains("pub struct VerifiedContext"), "{rust}");
    assert!(rust.contains("pub(super) string: String"), "{rust}");
    assert!(rust.contains("pub(super) integer: i64"), "{rust}");
    let file = read(SEALED);
    assert_eq!(read(&file.print()), file);
    assert!(
        file.print()
            .contains("VerifiedContext.Sealed.{ String Integer }")
    );
}

#[test]
fn a_sealed_type_needs_a_constructor_of_a_trait_it_bears() {
    // No association at all.
    let unborne = "Library.{ [] [ V.Sealed.{ String } ] [ K.[ make:[ Self ] ] ] [] }";
    assert_eq!(
        refusal(unborne),
        (vec![1, 1, 0, 0], Problem::Sealed("V".to_owned()))
    );
    // A trait borne, but its capability takes self.
    let shared = "Library.{ [] [ V.Sealed.{ String } ] [ K.[ make.[ Self ] ] ] [ V.[ K ] ] }";
    assert_eq!(refusal(shared).1, Problem::Sealed("V".to_owned()));
    // A static capability yielding something else.
    let other = "Library.{ [] [ V.Sealed.{ String } ] [ K.[ make:[ String ] ] ] [ V.[ K ] ] }";
    assert_eq!(refusal(other).1, Problem::Sealed("V".to_owned()));
    // Built by `Self`, or by `Result<Self X>`.
    for yielded in ["Self", "Result<Self String>"] {
        let built = format!(
            "Library.{{ [] [ V.Sealed.{{ String }} ] [ K.[ make:[ {yielded} ] ] ] [ V.[ K ] ] }}"
        );
        assert!(generated(&built).contains("pub(super) string: String"));
    }
    // A root with no traits cannot build one.
    assert_eq!(
        refusal("Memory.{ [] [ V.Sealed.{ String } ] }"),
        (vec![1, 1, 0, 0], Problem::Sealed("V".to_owned()))
    );
    // Only a struct is sealed.
    assert_eq!(
        refusal("Library.{ [] [ V.Sealed.[ A B ] ] [] [] }").1,
        Problem::Expected(ethos_zero::Form::Declaration)
    );
}

// 5. ⟦ T ⟧ ------------------------------------------------------------------

#[test]
fn vector_brackets_are_a_vector_in_every_type_position() {
    let sugared = read(
        "Library.{ [] [ Locks.⟦ String ⟧ Record.{ ⟦ ⟦ Integer ⟧ ⟧ Brief.⟦ String ⟧ } Reply.[ Many.⟦ Locks ⟧ None ] ] [ K.[ list.[ ⟦ String ⟧ ] ] ] [] }",
    );
    let spelled = read(
        "Library.{ [] [ Locks.Vector<String> Record.{ Vector<Vector<Integer>> Brief.Vector<String> } Reply.[ Many.Vector<Locks> None ] ] [ K.[ list.[ Vector<String> ] ] ] [] }",
    );
    assert_eq!(sugared, spelled);
}

#[test]
fn an_error_inside_vector_brackets_is_located_there() {
    use ethos_zero::Locating;
    let source = "Library.{ [] [ Record.{ ⟦ Bogus ⟧ String } ] [] [] }";
    let potential = Potential::<File>::from(source);
    let error = match potential.actualize() {
        Ok(file) => file.generate().expect_err("Bogus is undeclared"),
        Err(error) => error,
    };
    let location = potential.locate(&error);
    let column = source[..source.find("Bogus").unwrap()].chars().count() as i64 + 1;
    assert_eq!((location.line, location.column), (1, column), "{error:?}");
    // The position after the brackets keeps its own path.
    let after = "Library.{ [] [ Record.{ ⟦ String ⟧ Bogus } ] [] [] }";
    let potential = Potential::<File>::from(after);
    let error = read(after).generate().expect_err("Bogus is undeclared");
    let location = potential.locate(&error);
    let column = after[..after.find("Bogus").unwrap()].chars().count() as i64 + 1;
    assert_eq!((location.line, location.column), (1, column), "{error:?}");
}

#[test]
fn vector_brackets_take_exactly_one_type() {
    assert!(matches!(
        refusal("Library.{ [] [ Pair.{ ⟦ String Integer ⟧ } ] [] [] }").1,
        Problem::Arity(_)
    ));
}

// A full Library: a declared trait, its association, every type bearing one --

#[test]
fn a_full_library_with_a_trait_borne_by_every_type_checks_and_generates() {
    let source = fixture("sealed-library");
    let file = read(&source);
    let File::Library(library) = &file else {
        panic!("a Library")
    };
    // Every type the Library shows bears a trait.
    for declaration in &library.types {
        let name = match declaration {
            TypeDeclaration::Struct(identity, _)
            | TypeDeclaration::Enum(identity, _)
            | TypeDeclaration::Alias(identity, _)
            | TypeDeclaration::Sealed(identity, _) => &identity.name,
        };
        assert!(
            library
                .associations
                .iter()
                .any(|association| &association.identity.name == name),
            "{} bears no trait",
            name.as_ref()
        );
    }
    let rust = generated(&source);
    assert!(rust.contains("pub trait Verifying"), "{rust}");
    assert!(
        rust.contains("fn verify() -> std::result::Result<Self, Rejection>"),
        "{rust}"
    );
    assert!(rust.contains("pub(super) string: String"), "{rust}");
    assert!(
        rust.contains("pub string_vector: std::vec::Vec<String>"),
        "{rust}"
    );
    // A datom of a sealed type is checked like any struct.
    file.conform("Locks", "{ [ a b ] }")
        .expect("Locks conforms");
    let error = file
        .conform("Rejection", "Malformed.{ x }")
        .expect_err("a String is not a struct");
    assert_eq!(error.path, vec![1]);
}

// Checking a datom against its expected type --------------------------------

#[test]
fn a_datom_is_refused_with_its_typed_error() {
    let signal = read(&fixture("orchestrate"));
    let error = signal
        .conform(
            "Response",
            "Observed.Locks.[ { 1 MyLock flow [ /a ] «why» } x ]",
        )
        .expect_err("x is not a Lock");
    assert_eq!(error.layer, datom_codec::ErrorLayer::Composition);
    assert_eq!(error.path, vec![1, 1, 1]);
    let error = signal
        .conform("Query", "Lock.{ name flow [] }")
        .expect_err("a LockRequest has four positions");
    assert!(matches!(
        error.kind,
        datom_codec::ErrorKind::Arity {
            expected: 4,
            found: 3
        }
    ));
}
