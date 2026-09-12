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
    let generated = match read("Library [] [ Node.{ String Self } ] [] []").generate() {
        Ok(generated) => generated,
        Err(_) => panic!("Self in a data position generates"),
    };
    assert!(generated.contains("pub node: std::boxed::Box<Self>"));
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
