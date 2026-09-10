//! Public reader and generator contracts.  These cover the current roots and
//! the constructs that consumers write; detailed parser failures live beside
//! the reader in `src/lib.rs`.

use ethos_zero::{Actualizing, File, Generating, Potential};
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
    assert!(rust.contains("cfg_attr(\n    feature = \"datom\""));
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
