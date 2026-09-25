//! Public reader and generator contracts.  These cover the current roots and
//! the constructs that consumers write; detailed parser failures live beside
//! the reader in `src/lib.rs`.

use ethos_zero::{Actualizing, Error, File, Generating, Potential, Problem};
use protos::{Protosizable, Textualizable};

fn read(source: &str) -> File {
    match Potential::<File>::from(source).actualize() {
        Ok(file) => file,
        Err(_) => panic!("source did not read: {source}"),
    }
}

#[test]
fn full_library_round_trips_and_generates_named_types_and_kinds() {
    let source = "Library [ std:[ Clonable Sendable Serializable ] ] [ SinkError.[ Closed ] Sink.{ String } ] [ Fillable.[ push!{ [ String ] [ Result<Integer SinkError> ] } drain![ Vector<String> ] create:[ Self ] ] Streamable.{ [ Fillable ] [ Item<Serializable> ] [ CAPACITY.Integer ] [ next![ Option<Item> ] ] } Processable<[Clonable Sendable] Serializable>.[ process.[ String ] ] ] [ Sink.[ Fillable ] ]";
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
fn sema_has_only_imports_and_record_type_sections() {
    let file = read("Sema [ crate:[ Handle ] ] [ Record.{ Handle String } ]");
    assert!(matches!(file, File::Sema(_)));
    assert_eq!(file, read(&file.protosize().textualize()));
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
fn sema_generates_a_record_type_named_record() {
    // Vision/sema.md names Sema's second section "record types" and reserves
    // no name; Sema generates no implied type of its own.
    let generated = match read("Sema [] [ Record.{ String Integer } ]").generate() {
        Ok(generated) => generated,
        Err(_) => panic!("a Sema record named Record generates"),
    };
    assert!(generated.contains("pub struct Record"));
    assert!(generated.contains("pub string: String"));
    assert!(generated.contains("pub integer: i64"));
}

#[test]
fn sema_generates_every_declared_record_and_refuses_a_duplicate() {
    let generated =
        match read("Sema [] [ Entry.{ String } Record.{ Entry Vector<Entry> } ]").generate() {
            Ok(generated) => generated,
            Err(_) => panic!("a two-record Sema generates"),
        };
    assert!(generated.contains("pub struct Entry"));
    assert!(generated.contains("pub entry_vector: std::vec::Vec<Entry>"));
    assert!(
        read("Sema [] [ Entry.{ String } Entry.{ Integer } ]")
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
        "Sema [] [ record.{ String } ]".refusal(),
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
