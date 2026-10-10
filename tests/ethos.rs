//! Public reader and generator contracts.  These cover the current roots and
//! the constructs that consumers write; detailed parser failures live beside
//! the reader in `src/lib.rs`.

use ethos_zero::{Actualizing, Error, File, Form, Generating, Potential, Problem};
use protos::{Protosizable, Textualizable};

fn read(source: &str) -> File {
    match Potential::<File>::from(source).actualize() {
        Ok(file) => file,
        Err(_) => panic!("source did not read: {source}"),
    }
}

#[test]
fn full_library_round_trips_and_generates_named_types_and_kinds() {
    let source = "Library [ std:[ Clonable Sendable Serializable ] ] [ SinkError.[ Closed ] Sink.{ String } ] [ Fillable.[ push!{ [ Serializable ] [ Result<Integer SinkError> ] } drain![ Vector<String> ] create:[ Self ] ] Streamable.{ [ Fillable ] [ Item<Serializable> ] [ CAPACITY.Integer ] [ next![ Option<Item> ] ] } Processable<[Clonable Sendable] Serializable>.[ process.[ String ] ] ] [ Sink.[ Fillable ] ]";
    let file = read(source);
    let repeated = read(&file.protosize().textualize());
    assert_eq!(file, repeated);

    let rust = match file.generate() {
        Ok(rust) => rust,
        Err(_) => panic!("checked Library generates"),
    };
    assert!(rust.contains("pub struct Sink"));
    assert!(rust.contains("pub string: String"));
    assert!(rust.contains("std::result::Result<i64, SinkError>"));
    assert!(rust.contains("pub trait Streamable"));
    assert!(rust.contains("pub trait Processable"));
}

#[test]
fn signal_generates_query_response_and_optional_datom_derives() {
    let file = read(include_str!("../fixtures/orchestrate.ethos"));
    let rust = match file.generate() {
        Ok(rust) => rust,
        Err(_) => panic!("checked Signal generates"),
    };
    assert!(rust.contains("pub enum Query"));
    assert!(rust.contains("pub enum Response"));
    assert!(rust.contains("feature = \"datom\""));
    assert!(rust.contains("datom_codec::Datomizable, datom_codec::Composing"));
    assert!(rust.contains("pub type LockId = i64"));
}

#[test]
fn memory_has_only_imports_and_record_type_sections() {
    let file = read("Memory [ crate:[ Handle ] ] [ Record.{ Handle String } ]");
    assert_eq!(file, read(&file.protosize().textualize()));
    assert!(
        Potential::<File>::from("Memory [] [] []")
            .actualize()
            .is_err()
    );
}

#[test]
fn a_file_headed_sema_is_refused() {
    assert!(
        Potential::<File>::from("Sema [] [ Record.{ String } ]")
            .actualize()
            .is_err()
    );
}

#[test]
fn operation_generates_its_operation_and_outcome_enums_with_their_payloads() {
    let file = read(
        "Operation [] [ Start.{ Voice Brief } Stop.Voice Pause ] [ Started.Integer Failed.[ Busy Gone.String ] Done ] [ Voice.String Brief.String ]",
    );
    assert_eq!(file, read(&file.protosize().textualize()));
    let rust = match file.generate() {
        Ok(rust) => rust,
        Err(_) => panic!("checked Operation generates"),
    };
    assert!(rust.contains("pub enum Operation {"), "{rust}");
    assert!(rust.contains("Start(Start_Data)"), "{rust}");
    assert!(rust.contains("pub struct Start_Data {"), "{rust}");
    assert!(rust.contains("pub voice: Voice"), "{rust}");
    assert!(rust.contains("Stop(Voice)"), "{rust}");
    assert!(rust.contains("pub enum Outcome {"), "{rust}");
    assert!(rust.contains("Started(i64)"), "{rust}");
    assert!(rust.contains("Failed(Failed_Data)"), "{rust}");
    assert!(rust.contains("pub enum Failed_Data {"), "{rust}");
    assert!(rust.contains("rkyv::Archive"), "{rust}");
}

#[test]
fn an_operation_declaring_the_operation_or_outcome_type_is_refused() {
    assert!(
        read("Operation [] [ Go.Operation ] [ Gone ] [ Operation.{ String } ]")
            .generate()
            .is_err()
    );
    assert!(
        read("Operation [] [ Go ] [ Gone.Outcome ] [ Outcome.{ String } ]")
            .generate()
            .is_err()
    );
}

#[test]
fn a_named_position_declares_its_type_in_place() {
    let file = read(
        "Memory [] [ Flow.{ Integer Brief.String State.[ Running Ended ] Capsule.{ Home.String Login.Vector<String> } } ]",
    );
    assert_eq!(file, read(&file.protosize().textualize()));
    let rust = match file.generate() {
        Ok(rust) => rust,
        Err(_) => panic!("a struct with named positions generates"),
    };
    assert!(rust.contains("pub type Brief = String;"), "{rust}");
    assert!(rust.contains("pub enum State {"), "{rust}");
    assert!(rust.contains("pub struct Capsule {"), "{rust}");
    assert!(
        rust.contains("pub type Login = std::vec::Vec<String>;"),
        "{rust}"
    );
    assert!(rust.contains("pub home: Home"), "{rust}");
    assert!(rust.contains("pub login: Login"), "{rust}");
    assert!(rust.contains("pub brief: Brief"), "{rust}");
    assert!(rust.contains("pub state: State"), "{rust}");
    assert!(rust.contains("pub capsule: Capsule"), "{rust}");
}

#[test]
fn a_named_position_shares_the_file_namespace() {
    assert!(
        read("Memory [] [ A.{ Brief.String } B.{ Brief.Integer } ]")
            .generate()
            .is_err()
    );
    assert!(
        read("Library [] [ Brief.String A.{ Brief.Integer } ] [] []")
            .generate()
            .is_err()
    );
}

#[test]
fn a_variant_payload_declares_named_positions_in_place() {
    let rust =
        match read("Signal [] [ Launch.{ Integer Brief.String } ] [ Launched ] []").generate() {
            Ok(rust) => rust,
            Err(_) => panic!("a payload with a named position generates"),
        };
    assert!(rust.contains("pub type Brief = String;"), "{rust}");
    assert!(rust.contains("pub brief: Brief"), "{rust}");
}

#[test]
fn retired_roots_are_not_accepted() {
    assert!(
        Potential::<File>::from("Types [] [] []")
            .actualize()
            .is_err()
    );
    assert!(Potential::<File>::from("Kinds [] []").actualize().is_err());
}

#[test]
fn operation_free_signal_generates_only_its_declared_shared_data() {
    let generated = match read("Signal [] [] [] [ Shared.{ Name } Name.String ]").generate() {
        Ok(generated) => generated,
        Err(_) => panic!("operation-free Signal generates"),
    };
    assert!(generated.contains("pub struct Shared"));
    assert!(!generated.contains("pub enum Query"));
    assert!(!generated.contains("pub enum Response"));
}

#[test]
fn a_signal_declaring_the_query_type_is_refused() {
    // The generator emits `pub enum Query` (Vision/ethos.md: "pub enum Query
    // { Lock(LockRequest), Release(LockId) }"), so `Query` is the name a
    // Signal may not also declare.
    let signal = read("Signal [] [ Ask.Query ] [ Told.Query ] [ Query.{ String } ]");
    assert!(signal.generate().is_err());
    // `Request` is an ordinary name: nothing is generated under it.
    let generated = match read("Signal [] [ Ask.Request ] [ Told.Request ] [ Request.{ String } ]")
        .generate()
    {
        Ok(generated) => generated,
        Err(_) => panic!("a Signal declaring Request generates"),
    };
    assert!(generated.contains("pub struct Request"));
    assert_eq!(generated.matches("pub enum Query").count(), 1);
}

#[test]
fn a_signal_declaring_the_response_type_is_refused() {
    assert!(
        read("Signal [] [ Ask.Response ] [ Told.Response ] [ Response.{ String } ]")
            .generate()
            .is_err()
    );
}

#[test]
fn memory_generates_a_record_type_named_record() {
    // Memory's second section is its record types and reserves no name;
    // Memory generates no implied type of its own.
    let generated = match read("Memory [] [ Record.{ String Integer } ]").generate() {
        Ok(generated) => generated,
        Err(_) => panic!("a Memory record named Record generates"),
    };
    assert!(generated.contains("pub struct Record"));
    assert!(generated.contains("pub string: String"));
    assert!(generated.contains("pub integer: i64"));
}

#[test]
fn memory_generates_every_declared_record_and_refuses_a_duplicate() {
    let generated =
        match read("Memory [] [ Entry.{ String } Record.{ Entry Vector<Entry> } ]").generate() {
            Ok(generated) => generated,
            Err(_) => panic!("a two-record Memory generates"),
        };
    assert!(generated.contains("pub struct Entry"));
    assert!(generated.contains("pub entry_vector: std::vec::Vec<Entry>"));
    assert!(
        read("Memory [] [ Entry.{ String } Entry.{ Integer } ]")
            .generate()
            .is_err()
    );
}

#[test]
fn self_in_a_data_position_is_named_for_the_enclosing_type() {
    // `Self` is an intrinsic (Vision/ethos.md) and `self` is not a name a
    // field may bear, so the field is named for the type `Self` stands for.
    let generated = match read("Library [] [ Node.{ String Option<Self> } ] [] []").generate() {
        Ok(generated) => generated,
        Err(_) => panic!("Self in a data position generates"),
    };
    assert!(generated.contains("pub node_option: std::option::Option<std::boxed::Box<Self>>"));
}

#[test]
fn a_sourced_generic_in_a_data_position_reads_and_reprints() {
    // ethos-zero's own printer emits `external:Vector<String>`; its reader
    // must take that shape back.
    let file = read("Library [ external:[ Vector ] ] [ Record.{ external:Vector<String> } ] [] []");
    assert_eq!(file, read(&file.protosize().textualize()));
    let generated = match file.generate() {
        Ok(generated) => generated,
        Err(_) => panic!("a sourced generic position generates"),
    };
    assert!(generated.contains("pub string_vector: external::Vector<String>"));
}

#[test]
fn inline_imports_use_the_lowercase_source_in_type_and_struct_positions() {
    let type_declaration = read("Library [] [ Topic.custom:Name ] [] []");
    let generated = match type_declaration.generate() {
        Ok(generated) => generated,
        Err(_) => panic!("a lowercase inline source in a type declaration generates"),
    };
    assert!(generated.contains("custom::Name"));

    let struct_position = read("Library [] [ Holder.{ Topic.custom:Name String } ] [] []");
    let generated = match struct_position.generate() {
        Ok(generated) => generated,
        Err(_) => panic!("a lowercase inline source in a struct position generates"),
    };
    assert!(generated.contains("custom::Name"));
}

#[test]
fn a_second_inline_source_is_refused_instead_of_overwriting_the_first() {
    for source in [
        "Library [] [ Holder.{ Topic:custom:Name String } ] [] []",
        "Library [] [ Holder.{ std:sync:Mutex<String> String } ] [] []",
    ] {
        match Potential::<File>::from(source).actualize() {
            Err(Error::Conceptual(data)) => {
                assert_eq!(data.problem, Problem::Expected(Form::Reference));
                assert_eq!(data.integer_vector, vec![1, 1, 0, 1, 0, 1]);
            }
            Err(Error::Structural(_)) => panic!("{source} is a conceptual refusal"),
            Ok(_) => panic!("{source} must be refused"),
        }
    }

    match Potential::<File>::from("Library [] [ MyType.std.custom:Mutex ] [] []").actualize() {
        Err(Error::Conceptual(data)) => {
            assert_eq!(data.problem, Problem::Expected(Form::Reference));
        }
        Err(Error::Structural(_)) => panic!("the dotted source form is a conceptual refusal"),
        Ok(_) => panic!("the dotted source form remains refused"),
    }
}

#[test]
fn an_authored_name_capturing_a_derived_inline_name_is_refused() {
    // The audit's row-15 probe: two enums each declare X in place, and an
    // authored X_Data would capture the short name. The authored name is
    // the occurrence refused, at its declaration.
    let probe = "Library [] [ P.[ X.{ String } ] Q.[ X.{ Integer } ] X_Data.String ] [] []";
    assert!(read(probe).generate().is_ok());
    let captured = "Library [] [ P.[ X.{ String } ] P_X_Data.String ] [] []";
    assert!(read(captured).generate().is_ok());
    let captured = "Library [] [ P.[ X.{ String } ] Q.[ X.{ Integer } ] P_X_Data.String ] [] []";
    match read(captured).generate() {
        Err(Error::Conceptual(data)) => {
            assert_eq!(data.problem, Problem::Duplicate("P_X_Data".to_owned()));
            assert_eq!(data.integer_vector, vec![1, 1, 2, 0]);
        }
        _ => panic!("an authored name capturing a derived one is refused"),
    }
    let captured = "Library [] [ P.[ X.{ String } ] X_Data.String ] [] []";
    match read(captured).generate() {
        Err(Error::Conceptual(data)) => {
            assert_eq!(data.problem, Problem::Duplicate("X_Data".to_owned()));
            assert_eq!(data.integer_vector, vec![1, 1, 1, 0]);
        }
        _ => panic!("an authored X_Data beside a unique inline X is refused"),
    }
}

#[test]
fn signal_query_and_response_inline_payloads_are_unique_file_wide() {
    let generated = match read("Signal [] [ Ask.{ String } ] [ Ask.{ Integer } ] []").generate() {
        Ok(generated) => generated,
        Err(_) => panic!("Query and Response each declaring Ask in place generate"),
    };
    assert!(generated.contains("pub struct Query_Ask_Data"));
    assert!(generated.contains("pub struct Response_Ask_Data"));
    syn::parse_file(&generated).expect("generated Rust parses");
}

/// The kind whose capability yields the conceptual refusal a source meets on generation.
trait Refusing {
    fn refusal(&self) -> (Vec<i64>, Problem);
}

impl Refusing for str {
    fn refusal(&self) -> (Vec<i64>, Problem) {
        match read(self).generate() {
            Err(Error::Conceptual(data)) => (data.integer_vector, data.problem),
            Err(Error::Structural(_)) => panic!("{self} must be refused conceptually"),
            Ok(_) => panic!("{self} must be refused"),
        }
    }
}

#[test]
fn a_declared_intrinsic_name_is_refused_by_name() {
    // Declaring Result used to shadow the intrinsic, and the later
    // Result<String Integer> failed as an obscure Arity.{ 0 2 }.
    assert_eq!(
        "Library [] [ Result.{ String } Pair.{ Result<String Integer> } ] [] []".refusal(),
        (vec![1, 1, 0, 0], Problem::Intrinsic("Result".to_owned()))
    );
    assert_eq!(
        "Library [] [ Meaning.String ] [] []".refusal(),
        (vec![1, 1, 0, 0], Problem::Intrinsic("Meaning".to_owned()))
    );
    assert_eq!(
        "Library [] [] [ Option.[] ] []".refusal(),
        (vec![1, 2, 0, 0], Problem::Intrinsic("Option".to_owned()))
    );
    assert_eq!(
        "Library [] [] [ Listing.{ [] [ Vector ] [] [] } ] []"
            .refusal()
            .1,
        Problem::Intrinsic("Vector".to_owned())
    );
    assert_eq!(
        "Signal [] [] [] [ Integer.{ String } ]".refusal(),
        (vec![1, 3, 0, 0], Problem::Intrinsic("Integer".to_owned()))
    );
}

#[test]
fn a_lowercase_type_or_kind_name_is_refused() {
    assert_eq!(
        "Library [] [ a.{ String } ] [] []".refusal(),
        (vec![1, 1, 0, 0], Problem::Case("a".to_owned()))
    );
    assert_eq!(
        "Library [] [] [ runnable.[] ] []".refusal(),
        (vec![1, 2, 0, 0], Problem::Case("runnable".to_owned()))
    );
    assert_eq!(
        "Memory [] [ record.{ String } ]".refusal(),
        (vec![1, 1, 0, 0], Problem::Case("record".to_owned()))
    );
}

#[test]
fn a_type_with_no_finite_value_is_refused() {
    assert_eq!(
        "Library [] [ S.{ Self } ] [] []".refusal(),
        (vec![1, 1, 0, 0], Problem::Cycle("S".to_owned()))
    );
    assert_eq!(
        "Library [] [ Leaf.String A.{ Leaf B } B.{ A } ] [] []".refusal(),
        (vec![1, 1, 1, 0], Problem::Cycle("A".to_owned()))
    );
    assert_eq!(
        "Library [] [ E.[ Only.E ] ] [] []".refusal(),
        (vec![1, 1, 0, 0], Problem::Cycle("E".to_owned()))
    );
    // A Vector, an Option or another variant is a way out.
    for finite in [
        "Library [] [ S.{ Vector<Self> } ] [] []",
        "Library [] [ S.{ Option<Self> } ] [] []",
        "Library [] [ E.[ Leaf Node.{ E E } ] ] [] []",
        "Library [] [ S.{ Result<Self String> } ] [] []",
        "Library [] [ Never.[] Holder.{ Never } ] [] []",
    ] {
        assert!(read(finite).generate().is_ok(), "{finite} generates");
    }
}

#[test]
fn outer_option_and_result_are_written_fully_qualified() {
    let generated = match read(
        "Library [] [ Wrapped.{ Option<Integer> Result<String Integer> } Chain.{ Option<Chain> } ] [] []",
    )
    .generate()
    {
        Ok(generated) => generated,
        Err(_) => panic!("outer containers generate"),
    };
    assert!(generated.contains("pub integer_option: std::option::Option<i64>"));
    assert!(generated.contains("pub string_integer_result: std::result::Result<String, i64>"));
    assert!(generated.contains("std::option::Option<std::boxed::Box<Chain>>"));
    assert!(!generated.contains(" Option<"));
    assert!(!generated.contains(" Result<"));
}

#[test]
fn every_generated_item_carries_rustfmt_skip() {
    let root = env!("CARGO_MANIFEST_DIR");
    let mut generated = vec![
        format!("{root}/src/error.rs"),
        format!("{root}/src/ethos-zero.rs"),
    ];
    for entry in std::fs::read_dir(format!("{root}/tests/generated")).expect("generated fixtures") {
        generated.push(entry.expect("fixture entry").path().display().to_string());
    }
    for path in generated {
        let text = std::fs::read_to_string(&path).expect("generated module reads");
        assert!(
            text.starts_with("#![allow(dead_code, non_camel_case_types, non_snake_case)]\n"),
            "{path} opens with the generated-file header"
        );
        let file = syn::parse_file(&text).expect("generated module parses");
        for item in &file.items {
            let attributes = match item {
                syn::Item::Struct(item) => &item.attrs,
                syn::Item::Enum(item) => &item.attrs,
                syn::Item::Type(item) => &item.attrs,
                syn::Item::Trait(item) => &item.attrs,
                syn::Item::Const(item) => &item.attrs,
                _ => panic!("{path} holds an item generation does not emit"),
            };
            assert!(
                attributes.iter().any(|attribute| {
                    let segments = &attribute.path().segments;
                    segments.len() == 2
                        && segments[0].ident == "rustfmt"
                        && segments[1].ident == "skip"
                }),
                "{path}: an item lacks #[rustfmt::skip]"
            );
        }
    }
}

/// The generated Rust with its whitespace removed, so an assertion names the
/// signature and not where prettyplease breaks it.
trait Compact {
    fn compact(&self) -> String;
}

impl Compact for str {
    fn compact(&self) -> String {
        match read(self).generate() {
            Ok(rust) => rust
                .chars()
                .filter(|glyph| !glyph.is_whitespace())
                .collect(),
            Err(error) => panic!("{self} must generate: {error:?}"),
        }
    }
}

#[test]
fn a_kind_in_an_input_becomes_a_parameter_bounded_by_it() {
    let rust = "Library [] [] [ Textualizable.[ textualize.[ String ] ] Resolvable.[ resolve.{ [ Textualizable ] [ Self ] } ] ] []".compact();
    assert!(
        rust.contains("fnresolve<N:Textualizable>(&self,input:N)->SelfwhereSelf:Sized;"),
        "{rust}"
    );
}

#[test]
fn each_kind_in_the_inputs_takes_its_own_parameter() {
    let rust = "Library [ protos:Textualizable ] [] [ Joinable.[ join!{ [ Textualizable Textualizable Self ] [ Integer ] } ] ] []".compact();
    assert!(
        rust.contains("fnjoin<N:protos::Textualizable,O:protos::Textualizable>(&mutself,input_0:N,input_1:O,input_2:Self,)->i64whereSelf:Sized;"),
        "{rust}"
    );
}

#[test]
fn a_kind_in_a_yield_becomes_a_parameter_bounded_by_it() {
    let rust = "Library [] [] [ Textualizable.[ textualize.[ String ] ] Making.[ make:[ Textualizable ] ] ] []".compact();
    assert!(rust.contains("fnmake<N:Textualizable>()->N;"), "{rust}");
}

#[test]
fn a_kind_the_head_already_binds_stays_the_associated_type() {
    let rust = "Library [] [] [ Textualizable.[ textualize.[ String ] ] Streamable.{ [] [ Item<Textualizable> ] [] [ push!{ [ Textualizable ] [ Self ] } ] } ] []".compact();
    assert!(
        rust.contains("fnpush(&mutself,input:Self::Item)->SelfwhereSelf:Sized;"),
        "{rust}"
    );
}

#[test]
fn self_and_the_kinds_own_parameters_stay_as_they_are() {
    let rust = "Library [ serde:Serializable ] [] [ Processable<Serializable>.[ process.{ [ Serializable Self ] [ Self ] } ] ] []".compact();
    assert!(
        rust.contains("fnprocess(&self,input_0:A,input_1:Self)->SelfwhereSelf:Sized;"),
        "{rust}"
    );
}

#[test]
fn a_concrete_type_in_an_input_is_refused_as_wanting_a_kind() {
    assert_eq!(
        "Library [] [] [ Resolvable.[ resolve.{ [ String ] [ Self ] } ] ] []".refusal(),
        (
            vec![1, 2, 0, 1, 0, 1, 0, 0],
            Problem::KindWanted("String".to_owned())
        )
    );
    assert_eq!(
        "Library [] [ Rec.String ] [ Resolvable.[ resolve.{ [ Self Rec ] [ Self ] } ] ] []"
            .refusal(),
        (
            vec![1, 2, 0, 1, 0, 1, 0, 1],
            Problem::KindWanted("Rec".to_owned())
        )
    );
    assert_eq!(
        "Library [] [] [ Fillable.[ push!{ [ Vector<Self> ] [ Self ] } ] ] []".refusal(),
        (
            vec![1, 2, 0, 1, 0, 1, 0, 0],
            Problem::KindWanted("Vector".to_owned())
        )
    );
    assert_eq!(
        "Library [] [] [ Fillable.[ push!{ [ protos:String ] [ Self ] } ] ] []"
            .refusal()
            .1,
        Problem::KindWanted("String".to_owned())
    );
}
