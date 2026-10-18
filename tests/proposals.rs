//! The proposed syntax this branch accepts, each example as proposed.
//!
//! 1. Any whitespace after a separator before its body.
//! 2. An ethos file is an ordinary datom: `Library.` then the braced sections.
//! 3. The type ethos: `Type.` carrying one declaration.
//! 4. A sealed type: `Name.Sealed.{ … }`, built only by its trait's `:` constructor.
//! 5. `⟦ T ⟧` is `Vector<T>` in a type position.
//! 6. An association headed by a trait lists the
//!    types bearing it: `Trait.[ TypeA TypeB ]`.

use ethos_zero::{
    Actualizing, Conforming, Error, File, Generating, Potential, Printable, Problem,
    TypeDeclaration,
};
use ethos_zero::{Head, Root, Rooted};

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

/// The one root of a file written alone.
fn alone(file: &File) -> &Root {
    match file {
        File::One(root) => root,
        File::Many(_) => panic!("one root"),
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
    let head = alone(&file).head();
    assert_eq!(head, Head::Library);
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
    let enumeration = alone(&file);
    let Root::Type(declared) = enumeration else {
        panic!("a type file, not {file:?}")
    };
    use TypeDeclaration::Enum;
    let Enum(identity, variants) = declared else {
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
    let root = alone(&file);
    let Root::Library(library) = root else {
        panic!("a Library")
    };
    // Every type the Library shows bears a trait.
    for declaration in &library.types {
        let name = match declaration {
            TypeDeclaration::Struct(identity, _)
            | TypeDeclaration::Enum(identity, _)
            | TypeDeclaration::Alias(identity, _)
            | TypeDeclaration::Sealed(identity, _)
            | TypeDeclaration::Shared(identity, _) => &identity.name,
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

#[test]
fn an_imported_type_is_read_from_a_sibling_file() {
    let root = env!("CARGO_MANIFEST_DIR");
    let print = |name: &str| {
        std::fs::read_to_string(format!("{root}/fixtures/print/{name}.ethos")).expect(name)
    };
    let signal = read(&print("flow-signal"));
    let library = read(&print("flow-library"));
    let text = "Report.{ 7 ToolUsed.«a tool» }";
    // Alone, the imported FlowId and Event cannot be checked.
    assert!(signal.conform("Query", text).is_err());
    let siblings = vec![("flow-library".to_owned(), library)];
    signal
        .conform_among(&siblings, "Query", text)
        .expect("the imported types conform");
    let error = signal
        .conform_among(&siblings, "Query", "Report.{ 7 Exploded }")
        .expect_err("Exploded is no Event");
    assert_eq!(error.path, vec![1, 1]);
}

#[test]
fn an_explicit_protos_import_names_the_intrinsic() {
    let library = read(
        "Library.{ [ protos:[ String Integer Boolean ] ] [ FlowId.String Event.[ Started.FlowId Counted.{ Integer Boolean } Named.protos:String ] ] [] [] }",
    );
    for text in [
        "Started.quietRiverStone",
        "Started.flow-000000000000000a",
        "Started.«quiet river stone»",
        "Counted.{ 3 True }",
        "Named.«a name»",
    ] {
        library
            .conform("Event", text)
            .unwrap_or_else(|error| panic!("{text} conforms: {error:?}"));
    }
    assert!(library.conform("Event", "Counted.{ x True }").is_err());
}

// Shared mutable state ------------------------------------------------------

#[test]
fn a_shared_type_is_a_cloneable_handle_on_one_locked_state() {
    let source = fixture("wake-waits");
    let file = read(&source);
    assert_eq!(read(&file.print()), file);
    assert!(
        file.print()
            .contains("WakeWaits.Shared.{ Vector<PaneId> Integer }")
    );
    let rust = generated(&source);
    assert!(rust.contains("pub struct WakeWaits_State"), "{rust}");
    assert!(
        rust.contains("pub(super) pane_id_vector: std::vec::Vec<PaneId>"),
        "{rust}"
    );
    assert!(
        rust.contains("pub(super) state: std::sync::Arc<std::sync::Mutex<WakeWaits_State>>"),
        "{rust}"
    );
    // No archive or datom derive on the state or the handle.
    let handle = &rust[rust.find("pub struct WakeWaits_State").unwrap()..];
    assert!(!handle.contains("rkyv"), "{rust}");
    assert!(
        !handle
            .contains("datom_codec::Datomizable, datom_codec::Composing))]\npub struct WakeWaits")
    );
    assert!(file.conform("WakeWaits", "{ [] 0 }").is_err());
}

#[test]
fn a_shared_type_must_be_borne_and_never_travels() {
    let unborne = "Library.{ [] [ S.Shared.{ Integer } ] [ K.[ make:[ Self ] ] ] [] }";
    assert_eq!(
        refusal(unborne),
        (vec![1, 1, 0, 0], Problem::Shared("S".to_owned()))
    );
    let held =
        "Library.{ [] [ S.Shared.{ Integer } Holder.{ S } ] [ K.[ make:[ Self ] ] ] [ S.[ K ] ] }";
    assert_eq!(
        refusal(held),
        (vec![1, 1, 1, 0], Problem::Shared("Holder".to_owned()))
    );
    let signal = "Signal.{ [] [ Ask.S ] [] [ S.Shared.{ Integer } ] }";
    assert_eq!(refusal(signal).1, Problem::Shared("Query".to_owned()));
    let captured = "Library.{ [] [ S.Shared.{ Integer } S_State.String ] [ K.[ make:[ Self ] ] ] [ S.[ K ] ] }";
    assert_eq!(
        refusal(captured).1,
        Problem::Duplicate("S_State".to_owned())
    );
}

#[test]
fn nothing_read_from_data_builds_a_sealed_type() {
    let rust = generated(SEALED);
    let sealed = &rust[..rust.find("pub struct VerifiedContext").unwrap()];
    let derives = &sealed[sealed.rfind("#[rustfmt::skip]").unwrap()..];
    assert!(
        derives.contains("rkyv::Archive, rkyv::Serialize, Clone"),
        "{rust}"
    );
    assert!(!derives.contains("Deserialize"), "{rust}");
    assert!(!derives.contains("Composing"), "{rust}");
    assert!(
        derives.contains("derive(datom_codec::Datomizable)"),
        "{rust}"
    );
    assert!(read(SEALED).conform("VerifiedContext", "{ a 1 }").is_err());

    let traits = "[ K.[ make:[ Self ] ] ]";
    let held =
        format!("Library.{{ [] [ V.Sealed.{{ String }} Holder.{{ V }} ] {traits} [ V.[ K ] ] }}");
    assert_eq!(
        refusal(&held),
        (vec![1, 1, 1, 0], Problem::Sealed("Holder".to_owned()))
    );
    let aliased = format!("Library.{{ [] [ V.Sealed.{{ String }} W.V ] {traits} [ V.[ K ] ] }}");
    assert_eq!(refusal(&aliased).1, Problem::Sealed("W".to_owned()));
    let carried =
        format!("Library.{{ [] [ V.Sealed.{{ String }} E.[ Has.V None ] ] {traits} [ V.[ K ] ] }}");
    assert_eq!(refusal(&carried).1, Problem::Sealed("E".to_owned()));
    // A sealed or shared type, read from no data, may hold one.
    let nested = format!(
        "Library.{{ [] [ V.Sealed.{{ String }} U.Sealed.{{ V }} S.Shared.{{ V }} ] {traits} [ V.[ K ] U.[ K ] S.[ K ] ] }}"
    );
    generated(&nested);
}

#[test]
fn a_mutable_capability_of_a_trait_a_shared_type_bears_takes_shared_self() {
    let rust = generated(&fixture("wake-waits"));
    assert!(
        rust.contains("fn wait<N: Paned>(&self, input: N) -> i64;"),
        "{rust}"
    );
    assert!(!rust.contains("&mut self"), "{rust}");
    // A trait no shared type bears keeps `&mut self`.
    let plain = generated("Library.{ [] [] [ Counting.[ count![ Integer ] ] ] [] }");
    assert!(plain.contains("fn count(&mut self) -> i64;"), "{plain}");
}

// A capability with no yield, a std import ---------------------------------

#[test]
fn an_empty_yield_bracket_yields_nothing() {
    let source = fixture("yieldless-waits");
    let file = read(&source);
    assert_eq!(read(&file.print()), file);
    assert!(file.print().contains("touch![]"));
    assert!(file.print().contains("wait!{ [ Paned ]"));
    let rust = generated(&source);
    assert!(rust.contains("fn touch(&self);"), "{rust}");
    assert!(
        rust.contains("fn wait<N: Paned>(&self, input: N);"),
        "{rust}"
    );
    let plain = generated("Library.{ [] [] [ Resetting.[ reset![] begin:[] ] ] [] }");
    assert!(plain.contains("fn reset(&mut self);"), "{plain}");
    assert!(plain.contains("fn begin();"), "{plain}");
    assert_eq!(
        refusal("Library.{ [] [] [ K.[ two.[ String Integer ] ] ] [] }").1,
        Problem::Yield
    );
}

#[test]
fn a_std_type_imports_through_a_colon_chain_into_shared_state_only() {
    let source = fixture("std-clock");
    let file = read(&source);
    assert_eq!(read(&file.print()), file);
    let rust = generated(&source);
    assert!(
        rust.contains("pub(super) instant: std::time::Instant"),
        "{rust}"
    );
    assert!(
        rust.contains("std::sync::Mutex<std::vec::Vec<i64>>"),
        "{rust}"
    );
    let traits = "[ K.[ make:[ Self ] ] ]";
    let travels = format!(
        "Library.{{ [ std:sync:[ Mutex ] ] [ Record.{{ Mutex<Integer> }} ] {traits} [ Record.[ K ] ] }}"
    );
    assert_eq!(
        refusal(&travels),
        (vec![1, 1, 0, 0], Problem::Foreign("Record".to_owned()))
    );
    let sealed = format!(
        "Library.{{ [ std:time:Instant ] [ V.Sealed.{{ Instant }} ] {traits} [ V.[ K ] ] }}"
    );
    assert_eq!(refusal(&sealed).1, Problem::Foreign("V".to_owned()));
    let signal = "Signal.{ [ std:time:[ Instant ] ] [ Ask.Instant ] [] [] }";
    assert_eq!(refusal(signal).1, Problem::Foreign("Query".to_owned()));
    // A source that is not std keeps its ordinary meaning.
    assert!(
        generated("Library.{ [ crate:inner:[ Thing ] ] [ Holder.{ Thing } ] [] [] }")
            .contains("pub thing: crate::inner::Thing")
    );
}

// White braces ⦃ ⦄ ----------------------------------------------------------

#[test]
fn white_braces_are_a_struct_and_white_brackets_a_vector() {
    let source = fixture("white-brackets");
    let sugared = read(&source);
    let spelled = read(
        "Library.\n{\n[]\n[ Point.{ Integer Integer }\n  Path.{ Vector<Point> Label.{ String } }\n  Shape.[ Dot.Point Line.{ Point Point } Polygon.Vector<Point> ] ]\n[ Measuring.[ length.[ Integer ] ] ]\n[ Point.[ Measuring ] Path.[ Measuring ] Label.[ Measuring ] Shape.[ Measuring ] ] }",
    );
    assert_eq!(sugared, spelled);
    let root = alone(&sugared);
    let Root::Library(library) = root else {
        panic!("a Library")
    };
    for declaration in &library.types {
        let (TypeDeclaration::Struct(identity, _) | TypeDeclaration::Enum(identity, _)) =
            declaration
        else {
            panic!("a struct or an enum")
        };
        assert!(
            library
                .associations
                .iter()
                .any(|association| association.identity.name == identity.name),
            "{} bears no trait",
            identity.name.as_ref()
        );
    }
    let rust = generated(&source);
    assert!(rust.contains("pub struct Point"), "{rust}");
    assert!(
        rust.contains("pub point_vector: std::vec::Vec<Point>"),
        "{rust}"
    );
    assert!(rust.contains("Line(Line_Data)"), "{rust}");
}

#[test]
fn an_error_inside_white_braces_is_located_there() {
    use ethos_zero::Locating;
    let source = "Library.{\n[]\n[ Holder.⦃ ⟦ String ⟧ Bogus ⦄ ]\n[]\n[] }";
    let potential = Potential::<File>::from(source);
    let error = read(source).generate().expect_err("Bogus is undeclared");
    let location = potential.locate(&error);
    let line = &source[source.find("[ Holder").unwrap()..];
    let column = line[..line.find("Bogus").unwrap()].chars().count() as i64 + 1;
    assert_eq!((location.line, location.column), (3, column), "{error:?}");
}

#[test]
fn an_imported_struct_in_a_vector_in_a_struct_conforms() {
    let library = read(
        "Library.{ [] [ Topic.String Subaspect.[ Vision Intent ] Key.{ Subaspect Topic } ] [] [] }",
    );
    let placements = read(
        "Library.{ [ flow:[ Key ] ] [ SystemPrompt.⟦ Key ⟧ StartupPrompt.⟦ Key ⟧ Placements.⦃ SystemPrompt StartupPrompt ⦄ ] [] [] }",
    );
    let text = "{ [ { Intent core } ] [ { Vision book } ] }";
    let siblings = vec![("flow".to_owned(), library)];
    placements
        .conform_among(&siblings, "Placements", text)
        .expect("the imported Key conforms inside the vectors");
    let error = placements
        .conform_among(&siblings, "Placements", "{ [ { Mission core } ] [] }")
        .expect_err("Mission is no Subaspect");
    assert_eq!(error.path, vec![0, 0, 0]);
    // With no sibling to read it from, the error names the import.
    let error = placements
        .conform("Placements", text)
        .expect_err("Key is unresolved");
    assert_eq!(error.path, vec![0, 0]);
    assert!(
        matches!(&error.kind, datom_codec::ErrorKind::Variant { expected, .. } if expected == "flow:Key declared in a sibling .ethos file"),
        "{error:?}"
    );
}

// 6. A trait head lists the types bearing it --

fn keyed(associations: &str) -> String {
    let key = "Key.{ String }";
    let system = "SystemPrompt.⟦ Key ⟧";
    let startup = "StartupPrompt.⟦ Key ⟧";
    let all = [key, system, startup];
    let types = all.join(" ");
    let count = "count.[ Integer ]";
    let traits = format!("Prompting.[ {count} ]");
    format!(
        "Library.{{ [] [ {types} ] [ {traits} ] \
        [ {associations} ] }}"
    )
}

#[test]
fn a_group_generates_as_its_types() {
    let single = generated(&keyed(
        "SystemPrompt.[ Prompting ] \
        StartupPrompt.[ Prompting ]",
    ));
    let types = "SystemPrompt StartupPrompt";
    let group = format!("Prompting.[ {types} ]");
    let grouped = generated(&keyed(&group));
    assert_eq!(grouped, single);
    let mixed = generated(&keyed(
        "SystemPrompt.[ Prompting ] \
        Prompting.[ StartupPrompt ]",
    ));
    assert_eq!(mixed, single);
    let file = read(&keyed("Prompting.[ Key ]"));
    let printed = file.print();
    let entry = "[ Prompting.[ Key ] ]";
    assert!(printed.contains(entry), "{printed}");
}

#[test]
fn a_head_is_a_trait_or_a_type() {
    let source = keyed("Nothing.[ Key ]");
    let name = "Nothing".to_owned();
    let problem = Problem::Undeclared(name);
    let refused = (vec![1, 3, 0], problem);
    assert_eq!(refusal(&source), refused);
}

#[test]
fn a_trait_head_lists_only_types() {
    let both = "Prompting.[ Key Prompting ]";
    let listed = keyed(both);
    let name = "Prompting".to_owned();
    let at = vec![1, 3, 0, 1, 0, 1];
    let refused = (at, Problem::Role(name));
    assert_eq!(refusal(&listed), refused);
    let unknown = keyed("Prompting.[ Nothing ]");
    let name = "Nothing".to_owned();
    let problem = Problem::Undeclared(name);
    assert_eq!(refusal(&unknown).1, problem);
    let form = ethos_zero::Form::Name;
    let problem = Problem::Expected(form);
    let entry = "Prompting.[ protos:String ]";
    let sourced = keyed(entry);
    assert_eq!(refusal(&sourced).1, problem);
}

// 7. A file is a vector of roots -------------

#[test]
fn a_vector_of_roots_generates_one_module() {
    let source = fixture("root-vector");
    let file = read(&source);
    let File::Many(roots) = &file else {
        panic!("a vector of roots")
    };
    assert_eq!(roots.len(), 2);
    assert_eq!(roots[1].head(), Head::Memory);
    let rust = generated(&source);
    let header = "#![allow(";
    assert_eq!(rust.matches(header).count(), 1);
    assert!(rust.contains("pub enum Shape"));
    // A root names what another declares.
    let field = "pub name: Name";
    assert!(rust.contains(field), "{rust}");
}

#[test]
fn a_vector_of_roots_prints_its_roots_hung() {
    let source = fixture("root-vector");
    let file = read(&source);
    let printed = file.print();
    let start = source.find('[').unwrap();
    let body = &source[start..];
    assert_eq!(printed, body);
    assert_eq!(read(&printed), file);
}

#[test]
fn the_roots_of_a_file_share_one_namespace() {
    let library = "Library.{ [] [ A.String ] \
        [] [] }";
    let memory = "Memory.{ [] \
        [ A.{ String Integer } ] }";
    let roots = [library, memory].join(" ");
    let source = format!("[ {roots} ]");
    let at = vec![1, 1, 1, 0, 0];
    let duplicate = "A".to_owned();
    let problem = Problem::Duplicate(duplicate);
    assert_eq!(refusal(&source), (at, problem));
}

#[test]
fn a_type_root_stands_only_alone() {
    let source = "[ Type.A.String ]";
    let at = vec![0, 0];
    let root = (at, Problem::Root);
    assert_eq!(refusal(source), root);
    let empty = (vec![], Problem::Empty);
    assert_eq!(refusal("[]"), empty);
}

#[test]
fn an_error_in_a_later_root_is_located_there() {
    use ethos_zero::Locating;
    let source = "[ Library.{ [] [] [] [] }\n  \
        Memory.{ [] [ R.{ String Bogus } ] } ]";
    let written: Potential<File> = source.into();
    let generation = read(source).generate();
    let error = generation.unwrap_err();
    let location = written.locate(&error);
    let start = source.find('\n').unwrap();
    let line = &source[start..];
    let bogus = line.find("Bogus").unwrap();
    let column = bogus as i64;
    let at = (location.line, location.column);
    assert_eq!(at, (2, column), "{error:?}");
}
