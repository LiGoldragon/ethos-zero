//! Public reader and generator contracts.  These cover the current roots and
//! the constructs that consumers write; detailed parser failures live beside
//! the reader in `src/lib.rs`.

use ethos_zero::{Actualizing, Error, File, Form, Generating, Potential, Problem, Wrapping_Data};
use protos::{Protosizable, Textualizable};

fn read(source: &str) -> File {
    match Potential::<File>::from(source).actualize() {
        Ok(file) => file,
        Err(_) => panic!("source did not read: {source}"),
    }
}

#[test]
fn full_library_round_trips_and_generates_named_types_and_traits() {
    let source = "Library [ std:[ Clonable Sendable Serializable ] ] [ SinkError.[ Closed Full ] Sink.{ String } ] [ Fillable.[ push!{ [ Serializable ] [ Result<Integer SinkError> ] } drain![ Vector<String> ] create:[ Self ] ] Streamable.{ [ Fillable ] [ Item<Serializable> ] [ CAPACITY.Integer ] [ next![ Option<Item> ] ] } Processable<[Clonable Sendable] Serializable>.[ process.[ String ] ] ] [ Sink.[ Fillable ] ]";
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
fn signal_generates_input_output_and_optional_datom_derives() {
    let file = read(include_str!("../fixtures/orchestrate.ethos"));
    let rust = match file.generate() {
        Ok(rust) => rust,
        Err(_) => panic!("checked Signal generates"),
    };
    assert!(rust.contains("pub enum Input"));
    assert!(rust.contains("pub enum Output"));
    assert!(rust.contains("feature = \"datom\""));
    assert!(rust.contains("datom_codec::Datomizable, datom_codec::Composing"));
    assert!(rust.contains("pub struct LockId(pub i64)"));
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
fn emission_generates_its_enums_and_binds_each_emission_to_its_outcome() {
    let source = include_str!("../fixtures/command-emission.ethos");
    let file = read(source);
    assert!(matches!(file, File::Emission(_)));
    assert_eq!(file, read(&file.protosize().textualize()));
    let rust = source.compact();
    for expected in [
        "pubstructCommand_Data{pubverb:Verb,pubargument_vector:std::vec::Vec<Argument>,}",
        "pubenumEmission{Command(Command_Data),}",
        "pubenumOutcome_Data{Done,Refused(i64),}",
        "pubenumOutcome{Outcome(Outcome_Data),}",
        "pubtraitEmitted{typeOutcome;}",
        "pubtraitEmittable<Translator>:Emitted{fnemit(&self,translator:&Translator)->Self::Outcome;}",
        "implEmittedforCommand_Data{typeOutcome=Outcome_Data;}",
    ] {
        assert!(rust.contains(expected), "{expected} in {rust}");
    }
    // The imported Signal names are written where they are used, and the
    // fixture uses neither.
    assert!(!rust.contains("signal_herdr"), "{rust}");
}

#[test]
fn an_outcome_past_the_last_emission_pairs_with_none_and_a_bare_outcome_binds_unit() {
    let rust = "Emission [] [ Go.Vector<String> Stop Ask.Question ] [ Gone Stopped.Integer Answer Extra.String ] [ Question.String Answer.Integer ]".compact();
    assert!(
        rust.contains("implEmittedforstd::vec::Vec<String>{typeOutcome=();}"),
        "{rust}"
    );
    assert!(
        rust.contains("implEmittedforQuestion{typeOutcome=Answer;}"),
        "{rust}"
    );
    // `Stop` carries no type to bear the trait; `Extra` pairs with none.
    assert_eq!(rust.matches("implEmittedfor").count(), 2, "{rust}");
}

#[test]
fn self_in_an_emission_or_outcome_is_bound_as_the_enum_it_stands_for() {
    let rust = "Emission [] [ Go.Vector<Self> ] [ Gone.Option<Self> ] []".compact();
    assert!(
        rust.contains("implEmittedforstd::vec::Vec<Emission>{typeOutcome=std::option::Option<std::boxed::Box<Outcome>>;}"),
        "{rust}"
    );
}

#[test]
fn an_emission_left_without_an_outcome_is_refused_at_its_path() {
    assert_eq!(
        "Emission [] [ Go.Vector<String> Stop ] [ Gone ] []".refusal(),
        (vec![1, 1, 2], Problem::Unpaired("Stop".to_owned()))
    );
    assert_eq!(
        "Emission [] [ Go ] [] []".refusal(),
        (vec![1, 1, 0], Problem::Unpaired("Go".to_owned()))
    );
}

#[test]
fn two_emissions_carrying_one_type_are_refused_at_the_second() {
    assert_eq!(
        "Emission [] [ Show.Pane Close.Pane ] [ Shown Closed ] [ Pane.String ]".refusal(),
        (vec![1, 1, 1], Problem::Duplicate("Pane".to_owned()))
    );
    assert_eq!(
        "Emission [] [ Show.Pane Pane ] [ Shown Closed ] [ Pane.String ]".refusal(),
        (vec![1, 1, 1], Problem::Duplicate("Pane".to_owned()))
    );
}

#[test]
fn an_emission_declaring_an_implied_enum_or_trait_name_is_refused() {
    for name in ["Emission", "Outcome", "Emitted", "Emittable"] {
        assert_eq!(
            format!("Emission [] [ Go ] [ Gone ] [ {name}.String ]").refusal(),
            (vec![1, 3, 0, 0], Problem::Duplicate(name.to_owned()))
        );
    }
}

#[test]
fn a_signal_generates_answering_with_one_capability_per_input_and_the_input_answerable_by_it() {
    let rust = "Signal [] [ Launch.LaunchRequest Report.{ String } Ping ] [ Launched.FlowId Reported Pong Refused.String ] [ LaunchRequest.{ String } FlowId.Integer ]".compact();
    // prettyplease ends a parameter list it wraps with a comma.
    let rust = rust.replace(",)", ")");
    for expected in [
        "pubtraitAnswering{fnlaunch(&self,launch_request:&LaunchRequest)->std::result::Result<FlowId,Output>;",
        "fnreport(&self,report:&Report_Data)->std::result::Result<(),Output>;",
        "fnping(&self)->std::result::Result<(),Output>;}",
        "pubtraitAnswerable{fnanswer<Nexus:Answering>(&self,nexus:&Nexus)->Output;}",
        "implAnswerableforInput{fnanswer<Nexus:Answering>(&self,nexus:&Nexus)->Output{matchself{",
        "Input::Launch(carried)=>",
        "nexus.launch(carried).map_or_else(|refusal|refusal,Output::Launched)",
        "Input::Report(carried)=>",
        "nexus.report(carried).map_or_else(|refusal|refusal,|()|Output::Reported)",
        "Input::Ping=>",
        "nexus.ping().map_or_else(|refusal|refusal,|()|Output::Pong)",
    ] {
        assert!(rust.contains(expected), "{expected} in {rust}");
    }
    // `Refused` is past the last input: it answers none.
    assert!(!rust.contains("Output::Refused"), "{rust}");
}

#[test]
fn an_input_s_capability_borrows_self_as_the_input_and_a_keyword_raw() {
    let rust =
        "Signal [] [ Again.Vector<Self> Match.Type ] [ Done.Option<Self> Matched ] [ Type.String ]"
            .compact();
    // prettyplease ends a parameter list it wraps with a comma.
    let rust = rust.replace(",)", ")");
    for expected in [
        "fnagain(&self,input_vector:&std::vec::Vec<Input>)->std::result::Result<std::option::Option<std::boxed::Box<Output>>,Output>;",
        "fnr#match(&self,r#type:&Type)->std::result::Result<(),Output>;",
    ] {
        assert!(rust.contains(expected), "{expected} in {rust}");
    }
}

#[test]
fn a_signal_without_inputs_generates_no_answering() {
    let rust = "Signal [] [] [ Told ] []".compact();
    assert!(!rust.contains("Answering"), "{rust}");
    assert!(!rust.contains("Answerable"), "{rust}");
}

#[test]
fn an_input_named_answer_has_the_capability_answer() {
    let rust = "Signal [] [ Answer ] [ Answered ] []".compact();
    for expected in [
        "pubtraitAnswering{fnanswer(&self)->std::result::Result<(),Output>;}",
        "Input::Answer=>",
        "nexus.answer().map_or_else(|refusal|refusal,|()|Output::Answered)",
    ] {
        assert!(rust.contains(expected), "{expected} in {rust}");
    }
}

#[test]
fn an_input_left_without_an_output_is_refused_at_its_path() {
    assert_eq!(
        "Signal [] [ Ask.Vector<String> Tell ] [ Told ] []".refusal(),
        (vec![1, 1, 2], Problem::Unpaired("Tell".to_owned()))
    );
}

#[test]
fn an_input_whose_capability_name_is_taken_is_refused() {
    assert_eq!(
        "Signal [] [ UrlFetch URLFetch ] [ Fetched Fetched2 ] []".refusal(),
        (vec![1, 1, 1], Problem::Duplicate("url_fetch".to_owned()))
    );
    for name in ["Answering", "Answerable"] {
        assert_eq!(
            format!("Signal [] [ Ask ] [ Told ] [ {name}.String ]").refusal(),
            (vec![1, 3, 0, 0], Problem::Duplicate(name.to_owned()))
        );
    }
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
    assert!(rust.contains("pub struct Brief(pub String);"), "{rust}");
    assert!(rust.contains("pub enum State {"), "{rust}");
    assert!(rust.contains("pub struct Capsule {"), "{rust}");
    assert!(
        rust.contains("pub struct Login(pub std::vec::Vec<String>);"),
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
    assert!(rust.contains("pub struct Brief(pub String);"), "{rust}");
    assert!(rust.contains("pub brief: Brief"), "{rust}");
}

#[test]
fn retired_roots_are_not_accepted() {
    assert!(
        Potential::<File>::from("Types [] [] []")
            .actualize()
            .is_err()
    );
    assert!(Potential::<File>::from("Traits [] []").actualize().is_err());
}

#[test]
fn operation_free_signal_generates_only_its_declared_shared_data() {
    let generated = match read("Signal [] [] [] [ Shared.{ Name } Name.String ]").generate() {
        Ok(generated) => generated,
        Err(_) => panic!("operation-free Signal generates"),
    };
    assert!(generated.contains("pub struct Shared"));
    assert!(!generated.contains("pub enum Input"));
    assert!(!generated.contains("pub enum Output"));
}

#[test]
fn a_signal_declaring_the_input_type_is_refused() {
    // The generator emits `pub enum Input` (the vision's implied namespace
    // `signal:[ Input Output ]`), so `Input` is the name a Signal may not
    // also declare.
    assert_eq!(
        "Signal [] [ Ask.Input ] [ Told.Input ] [ Input.{ String } ]".refusal(),
        (vec![1, 3, 0, 0], Problem::Duplicate("Input".to_owned()))
    );
    // `Query` and `Response` are ordinary names: nothing is generated under them.
    let generated = match read(
        "Signal [] [ Ask.Query ] [ Told.Response ] [ Query.{ String } Response.{ String } ]",
    )
    .generate()
    {
        Ok(generated) => generated,
        Err(_) => panic!("a Signal declaring Query and Response generates"),
    };
    assert!(generated.contains("pub struct Query"));
    assert!(generated.contains("pub struct Response"));
    assert_eq!(generated.matches("pub enum Input").count(), 1);
    assert_eq!(generated.matches("pub enum Output").count(), 1);
}

#[test]
fn a_signal_declaring_the_output_type_is_refused() {
    assert_eq!(
        "Signal [] [ Ask.Output ] [ Told.Output ] [ Output.{ String } ]".refusal(),
        (vec![1, 3, 0, 0], Problem::Duplicate("Output".to_owned()))
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

    let memory_record = read("Memory [] [ Holder.{ Topic.custom:Name String } ]");
    let generated = match memory_record.generate() {
        Ok(generated) => generated,
        Err(_) => panic!("a lowercase inline source in a record field generates"),
    };
    assert!(generated.contains("custom::Name"));
}

#[test]
fn a_second_inline_source_is_refused_instead_of_overwriting_the_first() {
    // A chained source is one segment; more segments are written rooted.
    let source = "Library [] [ Holder.{ std:sync:Mutex<String> String } ] [] []";
    match Potential::<File>::from(source).actualize() {
        Err(Error::Conceptual(data)) => {
            assert_eq!(data.problem, Problem::Expected(Form::Reference));
            assert_eq!(data.integer_vector, vec![1, 1, 0, 1, 0, 1]);
        }
        Err(Error::Structural(_)) => panic!("{source} is a conceptual refusal"),
        Ok(_) => panic!("{source} must be refused"),
    }

    match Potential::<File>::from("Library [] [ MyType.std.custom:Mutex ] [] []").actualize() {
        Err(Error::Conceptual(data)) => {
            assert_eq!(data.problem, Problem::Expected(Form::Reference));
        }
        Err(Error::Structural(_)) => panic!("the dotted source form is a conceptual refusal"),
        Ok(_) => panic!("the dotted source form remains refused"),
    }
}

/// Generate a source that must read, check and generate, and reprint to the same file.
fn generates(source: &str) -> String {
    let file = read(source);
    assert_eq!(
        file,
        read(&file.protosize().textualize()),
        "{source} reprints"
    );
    match file.generate() {
        Ok(generated) => {
            syn::parse_file(&generated).expect("generated Rust parses");
            generated
        }
        Err(_) => panic!("{source} generates"),
    }
}

#[test]
fn rooted_imports_in_the_imports_section_take_every_namespace_segment() {
    let source = "Library [ :flow_ethos:message:Type :flow_ethos:signal:[ Input Output ] :protos:[ Textualizable ] ] [ Record.{ Type Input Vector<Output> } ] [] [ Record.[ Textualizable ] ]";
    let generated = generates(source);
    assert!(
        generated.contains(": flow_ethos::message::Type"),
        "{generated}"
    );
    assert!(
        generated.contains(": flow_ethos::signal::Input"),
        "{generated}"
    );
    assert!(
        generated.contains("std::vec::Vec<flow_ethos::signal::Output>"),
        "{generated}"
    );

    let print = read(source).protosize().textualize();
    assert!(print.contains(":flow_ethos:message:Type"), "{print}");
    // protos lays the bare source and its bracket out as two siblings.
    assert!(print.contains(":flow_ethos:signal:"), "{print}");
    assert!(print.contains("[ Input Output ]"), "{print}");

    let memory = generates("Memory [ :crate:message:[ FlowId ] ] [ Flow.{ FlowId } ]");
    assert!(
        memory.contains("pub flow_id: crate::message::FlowId"),
        "{memory}"
    );
}

#[test]
fn a_rooted_import_inline_in_an_alias_puts_the_colon_where_the_period_was() {
    let generated = generates(
        "Library [] [ Topic:custom:Name Nested:flow_ethos:message:Type Listed:external:sub:Vector<String> ] [] []",
    );
    assert!(
        generated.contains("pub struct Topic(pub custom::Name);"),
        "{generated}"
    );
    assert!(
        generated.contains("pub struct Nested(pub flow_ethos::message::Type);"),
        "{generated}"
    );
    assert!(
        generated.contains("pub struct Listed(pub external::sub::Vector<String>);"),
        "{generated}"
    );

    let in_place =
        generates("Library [] [ Holder.{ Topic:flow_ethos:message:Type String } ] [] []");
    assert!(
        in_place.contains("pub struct Topic(pub flow_ethos::message::Type);"),
        "{in_place}"
    );
    assert!(in_place.contains("pub topic: Topic"), "{in_place}");
}

#[test]
fn a_rooted_import_inline_in_a_field_takes_its_leading_colon() {
    let generated = generates(
        "Library [] [ Failure.{ :protos:Error String } Carrier.{ :flow_ethos:message:Type :external:sub:Vector<String> } ] [] []",
    );
    assert!(
        generated.contains("pub error: protos::Error"),
        "{generated}"
    );
    assert!(
        generated.contains("flow_ethos::message::Type"),
        "{generated}"
    );
    assert!(
        generated.contains("external::sub::Vector<String>"),
        "{generated}"
    );

    let record = generates("Memory [] [ Holder.{ :flow_ethos:message:Type String } ]");
    assert!(record.contains("flow_ethos::message::Type"), "{record}");
}

#[test]
fn a_rooted_namespace_segment_is_uncapitalized_and_a_name_seen_twice_is_placed_beside() {
    for source in [
        "Library [ :Flow:Type ] [] [] []",
        "Library [] [ Holder.{ :flow:Message:Type } ] [] []",
        "Library [] [ Topic:Custom:Name ] [] []",
    ] {
        match Potential::<File>::from(source).actualize() {
            Err(Error::Conceptual(data)) => {
                assert!(
                    matches!(data.problem, Problem::Name(_)),
                    "{source}: {:?}",
                    data.problem
                );
            }
            Err(Error::Structural(_)) => panic!("{source} is a conceptual refusal"),
            Ok(_) => panic!("{source} must be refused"),
        }
    }

    // `:a:b:` and its bracket are two siblings, so the second import's
    // names sit in the bracket at index three.
    match read("Library [ :a:b:[ X ] :c:[ Y X ] ] [] [] []").generate() {
        Err(Error::Conceptual(data)) => {
            assert_eq!(data.problem, Problem::Duplicate("X".to_owned()));
            assert_eq!(data.integer_vector, vec![1, 0, 3, 1]);
        }
        other => panic!("a name imported twice is refused: {other:?}"),
    }
}

#[test]
fn an_authored_name_capturing_a_derived_inline_name_is_refused() {
    // The audit's row-15 probe: two enums each declare X in place, and an
    // authored X_Data would capture the short name. The authored name is
    // the occurrence refused, at its declaration.
    let probe = "Library [] [ P.[ X.{ String } Y ] Q.[ X.{ Integer } Y ] X_Data.String ] [] []";
    assert!(read(probe).generate().is_ok());
    let captured = "Library [] [ P.[ X.{ String } Y ] P_X_Data.String ] [] []";
    assert!(read(captured).generate().is_ok());
    let captured =
        "Library [] [ P.[ X.{ String } Y ] Q.[ X.{ Integer } Y ] P_X_Data.String ] [] []";
    match read(captured).generate() {
        Err(Error::Conceptual(data)) => {
            assert_eq!(data.problem, Problem::Duplicate("P_X_Data".to_owned()));
            assert_eq!(data.integer_vector, vec![1, 1, 2, 0]);
        }
        _ => panic!("an authored name capturing a derived one is refused"),
    }
    let captured = "Library [] [ P.[ X.{ String } Y ] X_Data.String ] [] []";
    match read(captured).generate() {
        Err(Error::Conceptual(data)) => {
            assert_eq!(data.problem, Problem::Duplicate("X_Data".to_owned()));
            assert_eq!(data.integer_vector, vec![1, 1, 1, 0]);
        }
        _ => panic!("an authored X_Data beside a unique inline X is refused"),
    }
}

#[test]
fn signal_input_and_output_inline_payloads_are_unique_file_wide() {
    let generated = match read("Signal [] [ Ask.{ String } ] [ Ask.{ Integer } ] []").generate() {
        Ok(generated) => generated,
        Err(_) => panic!("Input and Output each declaring Ask in place generate"),
    };
    assert!(generated.contains("pub struct Input_Ask_Data"));
    assert!(generated.contains("pub struct Output_Ask_Data"));
    syn::parse_file(&generated).expect("generated Rust parses");
}

/// The trait whose capability yields the conceptual refusal a source meets on generation.
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

/// The wrapping problem carrying the head and the structural type it wrapped.
fn wrapping(head: &str, wrapped: &str) -> Problem {
    Problem::Wrapping(Wrapping_Data {
        first_string: head.to_owned(),
        second_string: wrapped.to_owned(),
    })
}

#[test]
fn types_section_declarations_refuse_wrapping_named_structs_and_enums() {
    for (source, problem) in [
        (
            "Library [] [ Record.{ String } Wrapped.Record ] [] []",
            wrapping("Wrapped", "Record"),
        ),
        (
            "Library [] [ Choice.[ First Second ] Wrapped.Choice ] [] []",
            wrapping("Wrapped", "Choice"),
        ),
        (
            "Signal [] [ Ask ] [] [ Wrapped.Input ]",
            wrapping("Wrapped", "Input"),
        ),
    ] {
        assert_eq!(source.refusal().1, problem, "{source}");
    }
}

#[test]
fn a_variant_named_as_a_defined_type_declaring_a_payload_carries_that_payload() {
    // The carrying rule is a bare variant's: a variant with a payload of its
    // own is a variant of that name carrying that payload, the variant and
    // the type being two names in Rust.
    for (source, carried) in [
        (
            "Library [] [ Code.String TextPart.[ Plain.String Code.String ] ] [] []",
            "pubenumTextPart{Plain(String),Code(String),}",
        ),
        (
            "Library [] [ Code.String TextPart.[ Plain.String Code.Code ] ] [] []",
            "pubenumTextPart{Plain(String),Code(Code),}",
        ),
        (
            "Library [] [ Code.String TextPart.[ Plain Code.{ String Integer } ] ] [] []",
            "pubenumTextPart{Plain,Code(Code_Data),}",
        ),
        (
            "Library [] [ Code.String Outer.[ Plain Inner.[ Other Code.[ A B ] ] ] ] [] []",
            "Code(Inner_Data_Code_Data)",
        ),
        (
            "Signal [] [ Ask ] [ Told ] [ Code.String Part.[ Plain Code.Integer ] ]",
            "pubenumPart{Plain,Code(i64),}",
        ),
    ] {
        let rust = source.compact();
        assert!(rust.contains(carried), "{source}: {rust}");
        assert!(
            rust.contains("pubstructCode(pubString);"),
            "{source}: {rust}"
        );
    }
}

#[test]
fn a_root_section_variant_named_as_a_defined_type_declaring_a_payload_carries_that_payload() {
    for (source, carried) in [
        (
            "Signal [] [ Lock.Request ] [ Locked.Lock ] [ Request.String Lock.String ]",
            "pubenumInput{Lock(Request),}",
        ),
        (
            "Signal [] [ Ask.Lock ] [ Lock.Lock ] [ Lock.String ]",
            "pubenumOutput{Lock(Lock),}",
        ),
        (
            "Operation [] [ Lock.{ String Integer } ] [ Done ] [ Lock.String ]",
            "pubenumOperation{Lock(Lock_Data),}",
        ),
        (
            "Operation [] [ Start.Lock ] [ Lock.[ Held Free ] ] [ Lock.String ]",
            "pubenumOutcome{Lock(Lock_Data),}",
        ),
    ] {
        let rust = source.compact();
        assert!(rust.contains(carried), "{source}: {rust}");
    }
}

#[test]
fn the_vision_lock_signal_takes_lock_beside_a_declared_lock() {
    let rust = include_str!("../fixtures/lock-signal.ethos").compact();
    assert!(
        rust.contains("pubenumInput{Lock(LockRequest),Release(LockId),}"),
        "{rust}"
    );
    assert!(
        rust.contains("pubenumOutput{Locked(Lock),Released(Lock),}"),
        "{rust}"
    );
    assert!(
        rust.contains("pubstructLock{publock_id:LockId,publock_name:LockName,}"),
        "{rust}"
    );
    assert!(
        rust.contains("fnlock(&self,lock_request:&LockRequest)->std::result::Result<Lock,Output>;"),
        "{rust}"
    );
}

#[test]
fn a_bare_variant_named_as_an_imported_type_carries_it() {
    let rust = "Library [ :sema:Block ] [ Part.[ Plain.String Block ] ] [] []".compact();
    assert!(rust.contains("Block(sema::Block)"), "{rust}");
    let rust = "Library [ :sema:Block ] [ Part.[ Plain Block.Block ] ] [] []".compact();
    assert!(
        rust.contains("pubenumPart{Plain,Block(sema::Block),}"),
        "{rust}"
    );
}

#[test]
fn a_field_is_named_for_its_type_alone_never_its_namespace() {
    let rust = "Library [ nexus:[ ConfigurationState ] signal_orchestrate:[ OrchestrateNexusConfiguration ] ] [ StoredMetadata.{ ConfigurationState<OrchestrateNexusConfiguration> } Errors.{ :a:Error :b:sub:Error } Sourced.{ :flow_ethos:message:Type external:Vector<String> } ] [] []".compact();
    assert!(
        rust.contains("puborchestrate_nexus_configuration_configuration_state:nexus::ConfigurationState<signal_orchestrate::OrchestrateNexusConfiguration,>"),
        "{rust}"
    );
    assert!(
        rust.contains("pubstructErrors{pubfirst_error:a::Error,pubsecond_error:b::sub::Error,}"),
        "{rust}"
    );
    assert!(
        rust.contains("pubstructSourced{pubr#type:flow_ethos::message::Type,pubstring_vector:external::Vector<String>,}"),
        "{rust}"
    );
}

#[test]
fn positions_whose_fields_would_meet_are_refused_at_the_later_position() {
    for (source, path) in [
        (
            "Library [] [ Pair.{ String String FirstString } FirstString.String ] [] []",
            vec![1, 1, 0, 1, 2],
        ),
        (
            "Library [] [ Holder.{ Inner.{ String String FirstString } } FirstString.String ] [] []",
            vec![1, 1, 0, 1, 0, 1, 2],
        ),
        (
            "Library [] [ Tree.[ Leaf Node.{ String String FirstString } ] FirstString.String ] [] []",
            vec![1, 1, 0, 1, 1, 1, 2],
        ),
        (
            "Signal [] [ Ask.{ String String FirstString } ] [ Told ] [ FirstString.String ]",
            vec![1, 1, 0, 1, 2],
        ),
        (
            "Memory [] [ Record.{ String FirstString String } FirstString.String ]",
            vec![1, 1, 0, 1, 1],
        ),
    ] {
        assert_eq!(
            source.refusal(),
            (path, Problem::DuplicateField("first_string".to_owned())),
            "{source}"
        );
    }
}

#[test]
fn an_enum_of_one_variant_is_refused_naming_the_enum() {
    // An enum has two variants or more: a declared enum is named by its
    // declaration, an enum a variant declares in place by that variant.
    for (source, path, enumeration) in [
        (
            "Library [] [ Reply.[ Accepted ] ] [] []",
            vec![1, 1, 0, 1],
            "Reply",
        ),
        ("Memory [] [ Mode.[ Strict ] ]", vec![1, 1, 0, 1], "Mode"),
        (
            "Signal [] [ Ask ] [ Told ] [ Mode.[ Strict ] ]",
            vec![1, 3, 0, 1],
            "Mode",
        ),
    ] {
        assert_eq!(
            source.refusal(),
            (path, Problem::SingleVariant(enumeration.to_owned())),
            "{source}"
        );
    }
}

#[test]
fn an_inline_enum_of_one_variant_is_refused_naming_its_variant() {
    for (source, path, enumeration) in [
        (
            "Library [] [ Reply.[ Accepted Refused.[ Busy ] ] ] [] []",
            vec![1, 1, 0, 1, 1, 1],
            "Refused",
        ),
        (
            "Library [] [ Reply.[ Accepted Refused.[ Busy Down.[ Hard ] ] ] ] [] []",
            vec![1, 1, 0, 1, 1, 1, 1, 1],
            "Down",
        ),
        (
            "Signal [] [ Ask.[ Strict ] ] [ Told ] []",
            vec![1, 1, 0, 1],
            "Ask",
        ),
    ] {
        assert_eq!(
            source.refusal(),
            (path, Problem::SingleVariant(enumeration.to_owned())),
            "{source}"
        );
    }
}

#[test]
fn an_enum_of_two_variants_is_accepted_declared_and_inline() {
    let rust = "Library [] [ Reply.[ Accepted Refused.[ Busy Down ] ] Never.[] ] [] []".compact();
    assert!(
        rust.contains("pubenumReply{Accepted,Refused(Refused_Data),}"),
        "{rust}"
    );
    assert!(rust.contains("pubenumRefused_Data{Busy,Down,}"), "{rust}");
    // A root's sections are not enums the file declares: a Signal of one
    // input and one output is accepted.
    assert!(read("Signal [] [ Ask ] [ Told ] []").generate().is_ok());
}

#[test]
fn a_repeated_type_is_told_apart_as_first_and_second() {
    let rust = "Library [] [ Pair.{ String String } StringVector.String Both.{ Vector<String> StringVector } ] [] []".compact();
    assert!(
        rust.contains("pubstructPair{pubfirst_string:String,pubsecond_string:String,}"),
        "{rust}"
    );
    assert!(
        rust.contains("pubstructBoth{pubfirst_string_vector:std::vec::Vec<String>,pubsecond_string_vector:StringVector,}"),
        "{rust}"
    );
}

#[test]
fn a_variant_named_as_an_implied_type_carries_nothing_and_takes_a_payload() {
    let rust = "Signal [] [ Input.Ask ] [ Told ] [ Ask.String ]".compact();
    assert!(rust.contains("pubenumInput{Input(Ask),}"), "{rust}");
}

#[test]
fn a_variant_named_as_a_defined_type_and_written_bare_carries_it() {
    let rust = "Library [] [ Code.String TextPart.[ Plain.String Code ] ] [] []".compact();
    assert!(
        rust.contains("pubenumTextPart{Plain(String),Code(Code),}"),
        "{rust}"
    );
    let rust = "Signal [] [ Acquire.Request ] [ Lock ] [ Request.String Lock.String ]".compact();
    assert!(rust.contains("pubenumOutput{Lock(Lock),}"), "{rust}");
}

#[test]
fn variant_wrapping_of_named_types_and_inline_shapes_remains_valid() {
    let source = "Library [] [ Record.{ String } Inner.[ First Second ] Outer.[ Other Wrapped.Record Nested.Inner Items.Vector<String> Inline.{ String Integer } ] ] [] []";
    assert!(read(source).generate().is_ok(), "{source}");
}

#[test]
fn scalar_vector_inline_and_nested_newtype_wrapping_remains_valid() {
    for source in [
        "Library [] [ Distance.Integer ] [] []",
        "Library [] [ Meters.Integer Distance.Meters ] [] []",
        "Library [] [ Names.Vector<String> More.Names ] [] []",
        "Library [] [ Record.{ String } Holder.{ Inline.Record } ] [] []",
        "Signal [] [ Acquire.LockRequest ] [ Locked.Lock ] [ LockRequest.{ String } Lock.{ String } ]",
    ] {
        assert!(read(source).generate().is_ok(), "{source}");
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
fn a_lowercase_type_or_trait_name_is_refused() {
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
        "Library [] [ E.[ Only.E Also.E ] ] [] []".refusal(),
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
            text.starts_with(
                "#![allow(\n    dead_code,\n    non_camel_case_types,\n    non_snake_case,\n    clippy::type_complexity\n)]\n"
            ),
            "{path} opens with the generated-file header"
        );
        let file = syn::parse_file(&text).expect("generated module parses");
        for item in &file.items {
            let attributes = match item {
                syn::Item::Struct(item) => &item.attrs,
                syn::Item::Enum(item) => &item.attrs,
                syn::Item::Type(_) => panic!("{path} holds a public type alias"),
                syn::Item::Trait(item) => &item.attrs,
                syn::Item::Const(item) => &item.attrs,
                syn::Item::Impl(item) => &item.attrs,
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
fn a_trait_in_an_input_becomes_a_parameter_bounded_by_it() {
    let rust = "Library [] [] [ Textualizable.[ textualize.[ String ] ] Resolvable.[ resolve.{ [ Textualizable ] [ Self ] } ] ] []".compact();
    assert!(
        rust.contains("fnresolve<N:Textualizable>(&self,input:N)->SelfwhereSelf:Sized;"),
        "{rust}"
    );
}

#[test]
fn each_trait_in_the_inputs_takes_its_own_parameter() {
    let rust = "Library [ protos:Textualizable ] [] [ Joinable.[ join!{ [ Textualizable Textualizable Self ] [ Integer ] } ] ] []".compact();
    assert!(
        rust.contains("fnjoin<N:protos::Textualizable,O:protos::Textualizable>(&mutself,input_0:N,input_1:O,input_2:Self,)->i64whereSelf:Sized;"),
        "{rust}"
    );
}

#[test]
fn a_trait_in_a_yield_becomes_a_parameter_bounded_by_it() {
    let rust = "Library [] [] [ Textualizable.[ textualize.[ String ] ] Making.[ make:[ Textualizable ] ] ] []".compact();
    assert!(rust.contains("fnmake<N:Textualizable>()->N;"), "{rust}");
}

#[test]
fn a_trait_the_head_already_binds_stays_the_associated_type() {
    let rust = "Library [] [] [ Textualizable.[ textualize.[ String ] ] Streamable.{ [] [ Item<Textualizable> ] [] [ push!{ [ Textualizable ] [ Self ] } ] } ] []".compact();
    assert!(
        rust.contains("fnpush(&mutself,input:Self::Item)->SelfwhereSelf:Sized;"),
        "{rust}"
    );
}

#[test]
fn self_and_the_traits_own_parameters_stay_as_they_are() {
    let rust = "Library [ serde:Serializable ] [] [ Processable<Serializable>.[ process.{ [ Serializable Self ] [ Self ] } ] ] []".compact();
    assert!(
        rust.contains("fnprocess(&self,input_0:A,input_1:Self)->SelfwhereSelf:Sized;"),
        "{rust}"
    );
}

#[test]
fn a_concrete_type_in_an_input_is_written_as_that_type() {
    let rust = "Library [] [ Rec.String ] [ Resolvable.[ resolve.{ [ Self Rec ] [ Self ] } ] ] []"
        .compact();
    assert!(
        rust.contains("fnresolve(&self,input_0:Self,input_1:Rec)->SelfwhereSelf:Sized;"),
        "{rust}"
    );
    let rust = "Library [] [] [ Resolvable.[ resolve.{ [ String ] [ Self ] } ] ] []".compact();
    assert!(
        rust.contains("fnresolve(&self,input:String)->SelfwhereSelf:Sized;"),
        "{rust}"
    );
    let rust = "Library [] [ Outline.String ] [ Outlined.[ outline.{ [ Vector<Outline> ] [ Self ] } ] ] []".compact();
    assert!(
        rust.contains("fnoutline(&self,input:std::vec::Vec<Outline>)->SelfwhereSelf:Sized;"),
        "{rust}"
    );
}
