//! A Nexus is generated from all its root files at once: the Process root's
//! module holds each process's positions, what it takes, and the enums over
//! them, read from the roots it names.

use ethos_zero::{
    Actualizing, Assembling, Error, File, Module, Name, Namespace, Nexus, Potential, Problem,
    Unused, Using, Verifying,
};

/// The root files of a fixture Nexus, as the CLI reads them: each module named for its file.
fn fixture(name: &str) -> Nexus {
    let root = env!("CARGO_MANIFEST_DIR");
    let directory = format!("{root}/fixtures/nexus/{name}");
    let mut stems: Vec<String> = Vec::new();
    for entry in std::fs::read_dir(&directory).unwrap() {
        let file = entry.unwrap().file_name().to_string_lossy().into_owned();
        if let Some(stem) = file.strip_suffix(".ethos") {
            stems.push(stem.to_owned());
        }
    }
    stems.sort();
    let mut modules = Vec::new();
    for stem in stems {
        let text = std::fs::read_to_string(format!("{directory}/{stem}.ethos")).unwrap();
        let file = match Potential::<File>::from(text).actualize() {
            Ok(file) => file,
            Err(error) => panic!("{name}/{stem} does not read: {error:?}"),
        };
        modules.push(Module { stem, file });
    }
    Nexus {
        modules,
        namespaces: vec![],
    }
}

/// A Nexus written in place: each module as its stem and its text.
fn nexus(modules: &[(&str, &str)]) -> Nexus {
    let mut read = Vec::new();
    for (stem, text) in modules {
        let file = match Potential::<File>::from(*text).actualize() {
            Ok(file) => file,
            Err(error) => panic!("{stem} does not read: {error:?}"),
        };
        read.push(Module {
            stem: (*stem).to_owned(),
            file,
        });
    }
    Nexus {
        modules: read,
        namespaces: vec![],
    }
}

/// What a Nexus is refused as: the place of the module, the path and the problem.
fn refused(nexus: &Nexus) -> (usize, Vec<i64>, Problem) {
    match nexus.verify() {
        Err(refusal) => match refusal.error {
            Error::Conceptual(data) => (refusal.module, data.integer_vector, data.problem),
            Error::Structural(_) => panic!("a conceptual refusal is wanted"),
        },
        Ok(()) => panic!("the Nexus must be refused"),
    }
}

/// The module of a Nexus as it is generated.
fn generated(nexus: &Nexus, stem: &str) -> String {
    match nexus.assemble() {
        Ok(assembled) => {
            for module in assembled {
                if module.stem == stem {
                    return module.rust;
                }
            }
            panic!("no module {stem}")
        }
        Err(refusal) => panic!("the Nexus must generate: {refusal:?}"),
    }
}

/// A generated text with its whitespace gone, to be searched for a shape and not a layout.
fn squeezed(text: &str) -> String {
    let squeezed: String = text
        .chars()
        .filter(|glyph| !glyph.is_whitespace())
        .collect();
    // Where a list is broken across lines it ends in a comma.
    squeezed.replace(",>", ">").replace(",)", ")")
}

#[test]
fn every_module_of_a_nexus_is_generated_under_its_file_name() {
    let nexus = fixture("tally");
    let assembled = nexus.assemble().expect("the Tally Nexus generates");
    let stems: Vec<&str> = assembled
        .iter()
        .map(|module| module.stem.as_str())
        .collect();
    assert_eq!(stems, ["memory", "process", "signal"]);
}

#[test]
fn a_root_alone_generates_as_it_always_did() {
    let alone = Nexus {
        modules: vec![fixture("tally").modules.remove(2)],
        namespaces: vec![],
    };
    assert_eq!(alone.modules[0].stem, "signal");
    let rust = generated(&alone, "signal");
    assert!(rust.contains("pub enum Input"));
    assert!(alone.verify().is_ok());
}

#[test]
fn the_process_root_of_the_tally_nexus_generates_its_positions_and_its_enums() {
    let rust = squeezed(&generated(&fixture("tally"), "process"));
    // The things given that the module makes: an output over what it carries, and an output that carries nothing.
    assert!(rust.contains("pubstructSummed(pubtally::signal::Total);"));
    assert!(rust.contains("pubstructOverflowed{}"));
    // A process born of a call holds its caller and the input; one born of a process, what its root writes.
    assert!(rust.contains(
        "pubstructSumProcess{pubcaller:process::Caller,pubinteger_vector:alloc::vec::Vec<i64>,}"
    ));
    assert!(rust.contains("pubstructAddendProcess{pubaddend:Addend,}"));
    // R2: where one kind of thing returns, what a process takes is that type, bare.
    assert!(rust.contains("pubtypeSumTaken=Adding;"));
    assert!(rust.contains(
        "pubtypeAddendTaken=core::result::Result<tally::memory::Addition,process::Unremembered>;"
    ));
    assert!(rust.contains("pubenumTaken{Adding(Adding),Addition(core::result::Result<tally::memory::Addition,process::Unremembered>),}"));
    // The enums over the processes; an output that carries nothing is a bare variant of Gift.
    assert!(rust.contains("pubenumProcess{Sum(SumProcess),Addend(AddendProcess),}"));
    assert!(rust.contains("pubenumOutcome{Sum(Summing),Addend(Adding),Unwound,}"));
    assert!(rust.contains(
        "pubenumGift{Addend(Addend),Summed(Summed),Overflowed,Add(tally::memory::Add),}"
    ));
    // Each thing given says what returns for it.
    assert!(rust.contains("implprocess::GivableforAddend{typeReturned=Adding;}"));
    assert!(rust.contains(
        "implprocess::Givablefortally::memory::Add{typeReturned=core::result::Result<tally::memory::Addition,process::Unremembered>;}"
    ));
    assert!(rust.contains("implprocess::GivableforOverflowed{typeReturned=();}"));
    assert!(rust.contains(
        "implcore::convert::From<Overflowed>forGift{fnfrom(_:Overflowed)->Gift{Gift::Overflowed}}"
    ));
    assert!(rust.contains("implprocess::UnwindableforOutcome"));
}

#[test]
fn a_process_born_of_a_call_is_owned_by_the_signal_process_and_one_born_of_a_record_by_the_memory_process()
 {
    let rust = squeezed(&generated(&fixture("claude-harness"), "process"));
    assert!(rust.contains("Process::Watch(_)=>process::Owner::Memory,"));
    assert!(rust.contains(
        "Process::Report(_)|Process::Subscribe(_)|Process::Configure(_)|Process::Prepare(_)=>process::Owner::Signal,"
    ));
}

#[test]
fn a_record_of_an_imported_type_is_a_record_and_a_change_returns_its_answer_or_unremembered() {
    let rust = squeezed(&generated(&fixture("claude-harness"), "process"));
    // A record kept returns the standard Answer.
    assert!(rust.contains(
        "implprocess::Givableforclaude_harness::memory::Watch{typeReturned=process::Answer;}"
    ));
    // A change returns the Result of its answer or Unremembered, as an emission returns the Result of its outcome or Unobserved.
    assert!(rust.contains(
        "implprocess::Givableforclaude_harness::memory::Observe{typeReturned=core::result::Result<claude_harness::memory::Observation,process::Unremembered>;}"
    ));
    assert!(rust.contains(
        "implprocess::Givableforclaude_harness::emission::Follow_Data{typeReturned=core::result::Result<claude_harness::emission::Following_Data,process::Unobserved>;}"
    ));
    // Where two kinds return to a process, its Taken is an enum of them, named for what each returns.
    assert!(rust.contains(
        "pubenumReportTaken{Answer(process::Answer),Observation(core::result::Result<claude_harness::memory::Observation,process::Unremembered>),}"
    ));
    // Where one kind returns, bare.
    assert!(rust.contains(
        "pubtypeSubscribeTaken=core::result::Result<claude_harness::memory::Recollection,process::Unremembered>;"
    ));
    assert!(rust.contains("pubtypeConfigureTaken=process::Answer;"));
    // A process born of a record holds the record's fields and what is configured into it.
    assert!(rust.contains(
        "pubstructWatchProcess{pubflow_id:flow::FlowId,pubfile_path:harness::FilePath,pubread_from:claude_harness::ReadFrom,pubrow_count:claude_harness::RowCount,}"
    ));
    assert!(rust.contains(
        "pubstructPrepareProcess{pubcaller:process::Caller,pubhook_client_option:core::option::Option<claude_harness::meta::HookClient>,pubhooks:claude_harness::meta::Hooks,}"
    ));
}

#[test]
fn the_process_enum_dispatches_each_begin_and_take_to_the_body_of_its_process() {
    let rust = squeezed(&generated(&fixture("claude-harness"), "process"));
    assert!(rust.contains(
        "Process::Report(report)=>{process::Processable::begin(report,giving).map(Outcome::Report)}"
    ));
    assert!(
        rust.contains("process::Processable::begin(subscribe,giving).map(|()|Outcome::Subscribe)")
    );
    // Where a process takes two kinds, the thing taken is handed over as the process's own Taken.
    assert!(rust.contains("(Process::Watch(watch),Taken::Following(carried))=>{process::Processable::take(watch,WatchTaken::Following(carried),giving).map(Outcome::Watch)}"));
    // Where it takes one kind, as that kind, bare.
    assert!(rust.contains("(Process::Configure(configure),Taken::Answer(carried))=>{process::Processable::take(configure,carried,giving).map(Outcome::Configure)}"));
    // A thing taken that no process takes is let go.
    assert!(rust.contains("_=>core::option::Option::None,"));
}

#[test]
fn the_namespace_is_the_one_every_import_of_the_roots_lands_in() {
    // Two namespaces that both land are two Nexuses in one set.
    let both = nexus(&[
        ("signal", "Signal [] [ Ask ] [ Told ] []"),
        (
            "process",
            "Process [ a:signal:[ Ask ] b:signal:[ Told ] ] [ Ask.{ [] [ Told ] } ] []",
        ),
    ]);
    let (module, path, problem) = refused(&both);
    assert_eq!((module, path), (0, vec![1, 0]));
    assert_eq!(problem, Problem::Namespace("b".to_owned()));
}

#[test]
fn what_a_process_names_that_the_nexus_does_not_declare_is_foreign() {
    let foreign = nexus(&[
        ("signal", "Signal [] [ Ask ] [ Told ] []"),
        (
            "process",
            "Process [ x:signal:[ Ask Told ] other:signal:[ Seen ] ] [ Ask.{ [] [ Told Seen ] } ] []",
        ),
    ]);
    let (module, path, problem) = refused(&foreign);
    assert_eq!((module, path), (1, vec![1, 1, 0, 1, 1, 1]));
    assert_eq!(problem, Problem::Foreign("Seen".to_owned()));
}

#[test]
fn a_process_is_born_of_an_input_or_a_record_or_a_process_that_gives_it() {
    let unborn = nexus(&[
        ("signal", "Signal [] [ Ask ] [ Told ] []"),
        (
            "process",
            "Process [ x:signal:[ Ask Told ] ] [ Told.{ [] [] } ] []",
        ),
    ]);
    let (_, path, problem) = refused(&unborn);
    assert_eq!(path, vec![1, 1, 0, 0]);
    assert_eq!(problem, Problem::Unborn("Told".to_owned()));
}

#[test]
fn what_cannot_leave_a_process_is_ungivable() {
    let ungivable = nexus(&[
        ("signal", "Signal [] [ Ask Tell ] [ Told Said ] []"),
        (
            "process",
            "Process [ x:signal:[ Ask Tell Told ] ] [ Ask.{ [] [ Tell Told ] } ] []",
        ),
    ]);
    let (_, path, problem) = refused(&ungivable);
    assert_eq!(path, vec![1, 1, 0, 1, 1, 0]);
    assert_eq!(problem, Problem::Ungivable("Tell".to_owned()));
}

#[test]
fn a_change_is_answered_by_the_answer_of_its_place() {
    let unpaired = nexus(&[(
        "memory",
        "Memory [] [] [ Add.Integer Take.Integer ] [ Added.Integer ] []",
    )]);
    let (module, path, problem) = refused(&unpaired);
    assert_eq!((module, path), (0, vec![1, 2, 1]));
    assert_eq!(problem, Problem::Unpaired("Take".to_owned()));
}

#[test]
fn two_kinds_of_return_under_one_name_are_refused() {
    // A record returns Answer, and so does a change whose answer is the Answer: two types under one name.
    let twice = nexus(&[
        ("signal", "Signal [] [ Ask ] [ Told ] []"),
        (
            "memory",
            "Memory [ process:[ Answer ] ] [ Note.Integer ] [ Add.Integer ] [ Answer ] []",
        ),
        (
            "process",
            "Process [ x:signal:[ Ask ] x:memory:[ Note Add ] ] [ Ask.{ [] [ Note Add ] } ] []",
        ),
    ]);
    let (module, path, problem) = refused(&twice);
    assert_eq!((module, path), (2, vec![1, 1, 0, 1, 1, 1]));
    assert_eq!(problem, Problem::Duplicate("Answer".to_owned()));
}

#[test]
fn a_matrix_provides_its_records_and_memory_to_the_roots_read_with_it() {
    let nexus = nexus(&[
        (
            "matrix",
            "Matrix [ app:signal:[ Observed ] ] [ Watch.{ Integer String } Observed ]",
        ),
        ("signal", "Signal [] [ Ask ] [ Observed.{ Integer } ] []"),
        (
            "library",
            "Library [ app:matrix:[ Watch Observed Memory ] ] [ Held.{ Watch Observed Memory } ] [] []",
        ),
    ]);
    assert!(nexus.verify().is_ok());
    let rust = squeezed(&generated(&nexus, "library"));
    assert!(rust.contains("pubwatch:app::matrix::Watch,"));
    assert!(rust.contains("pubmemory:app::matrix::Memory,"));
    let matrix = squeezed(&generated(&nexus, "matrix"));
    assert!(matrix.contains("pubstructMemory{pubwatch:Watch,pubobserved:app::signal::Observed,}"));
}

#[test]
fn two_roots_of_one_name_are_refused() {
    let twice = nexus(&[
        ("signal", "Signal [] [ Ask ] [ Told ] []"),
        ("signal", "Signal [] [ Ask ] [ Told ] []"),
    ]);
    let (module, _, problem) = refused(&twice);
    assert_eq!(module, 1);
    assert_eq!(problem, Problem::Duplicate("signal".to_owned()));
}

#[test]
fn the_committed_modules_are_what_the_nexus_generates() {
    let root = env!("CARGO_MANIFEST_DIR");
    for name in ["tally", "claude-harness"] {
        let nexus = fixture(name);
        for module in nexus.assemble().expect("the fixture Nexus generates") {
            let committed = std::fs::read_to_string(format!(
                "{root}/tests/generated/nexus/{name}/{}.rs",
                module.stem
            ))
            .unwrap_or_else(|_| panic!("{name}/{} is not committed", module.stem));
            assert_eq!(
                module.rust, committed,
                "tests/generated/nexus/{name}/{}.rs is stale; regenerate from fixtures/nexus/{name}",
                module.stem
            );
        }
    }
}

/// A Nexus whose Signal gives a record and an enum declared in place, and an output
/// named for a declared type. Its Memory and Process use them as types.
fn imported_payloads() -> Nexus {
    nexus(&[
        (
            "signal",
            "Signal [] [ Ask ] [ Changed.{ Integer Integer } Marked.[ Seen Unseen ] Plain ] [ Plain.{ Integer } ]",
        ),
        (
            "memory",
            "Memory [ app:signal:[ Changed Noted.Marked Plain ] ] [ Held.{ Changed ⟦ Changed ⟧ Option<Changed> Noted Plain } Latest.Changed ] [] [] []",
        ),
        (
            "process",
            "Process [ app:signal:[ Ask Changed ] app:memory:[ Held ] ] [ Ask.{ [ Changed ] [ Changed ] } Held.{ [] [ Changed ] } ] []",
        ),
    ])
}

#[test]
fn an_imported_name_that_declares_its_payload_in_place_is_written_as_the_payload_everywhere() {
    let rust = squeezed(&generated(&imported_payloads(), "memory"));
    // A field, a vector and an option each write the payload's derived name.
    assert!(rust.contains(
        "pubstructHeld{pubchanged:app::signal::Changed_Data,pubchanged_vector:alloc::vec::Vec<app::signal::Changed_Data>,pubchanged_option:core::option::Option<app::signal::Changed_Data>,"
    ));
    // A newtype too.
    assert!(rust.contains("pubstructLatest(pubapp::signal::Changed_Data);"));
    // An aliased enum keeps the authored field name, and a declared type keeps its Rust name.
    assert!(rust.contains("pubnoted:app::signal::Marked_Data,pubplain:app::signal::Plain,"));
    assert!(!rust.contains("app::signal::Changed,"));
    assert!(!rust.contains("app::signal::Changed>"));
}

#[test]
fn a_record_held_from_the_process_root_writes_the_payload_its_name_resolved_to() {
    let rust = squeezed(&generated(&imported_payloads(), "process"));
    assert!(rust.contains("pubstructHeldProcess{pubchanged:app::signal::Changed_Data,"));
    assert!(rust.contains(
        "pubstructAskProcess{pubcaller:process::Caller,pubchanged:app::signal::Changed_Data,"
    ));
    assert!(rust.contains("Changed(app::signal::Changed_Data)"));
    assert!(!rust.contains("app::signal::Changed,"));
}

#[test]
fn a_signal_with_an_open_enum_generates_in_a_nexus_and_exports_what_it_generates() {
    let both = nexus(&[
        (
            "signal",
            "Signal [] [ Ask ] [ Told ] [ Mark.String ] [ Model ]",
        ),
        (
            "library",
            "Library [ signal:[ Model Carried Opening Input Output Answering ] ] [ Held.{ Model } ] [] []",
        ),
    ]);
    assert_eq!(both.verify(), Ok(()));
    let rust = generated(&both, "signal");
    assert!(rust.contains("pub struct Model {"));
    assert!(rust.contains("pub enum Carried {"));
}

#[test]
fn an_open_enum_no_input_or_output_carries_is_listed_unused_and_still_verifies() {
    let unused = nexus(&[
        (
            "signal",
            "Signal [] [ Launch.String ] [ Launched ] [ TripleEffort.[ Low Medium High ] ] [ Model ]",
        ),
        ("library", "Library [] [ Thing.String ] [] []"),
        (
            "other",
            "Signal [] [ Launch.Effort ] [ Launched Held.Model ] [] [ Model Effort Spare ]",
        ),
    ]);
    assert_eq!(unused.verify(), Ok(()));
    assert_eq!(
        unused.unused(),
        vec![
            Unused {
                module: 0,
                name: Name::try_from("Model").unwrap(),
            },
            Unused {
                module: 2,
                name: Name::try_from("Spare").unwrap(),
            },
        ]
    );
    let used = nexus(&[(
        "signal",
        "Signal [] [ Launch.Model ] [ Launched ] [ TripleEffort.[ Low Medium High ] ] [ Model ]",
    )]);
    assert_eq!(used.unused(), vec![]);
}

#[test]
fn an_open_enum_reached_through_any_type_the_root_declares_is_carried() {
    let root = env!("CARGO_MANIFEST_DIR");
    let held = std::fs::read_to_string(format!("{root}/fixtures/open-enum-held.ethos")).unwrap();
    let held = nexus(&[("held", held.as_str())]);
    assert_eq!(held.verify(), Ok(()));
    assert_eq!(held.unused(), vec![]);
    // Through a vector, an option, a variant's payload, a position declared in
    // place and a cycle of declared types; Spare is reached by none.
    let deep = nexus(&[(
        "signal",
        "Signal [] [ Launch.Outer ] [ Launched Kept.⟦ Option<Tally> ⟧ ] \
         [ Outer.{ ⟦ Inner ⟧ } Inner.[ Leaf Node.Outer Held.Option<Model> ] \
           Tally.{ Count.[ One Many.Effort ] } ] \
         [ Model Effort Spare ]",
    )]);
    assert_eq!(deep.verify(), Ok(()));
    assert_eq!(
        deep.unused(),
        vec![Unused {
            module: 0,
            name: Name::try_from("Spare").unwrap(),
        }]
    );
}

#[test]
fn the_qualified_name_of_the_roots_own_open_enum_carries_it_where_its_namespace_is_given() {
    let text = "Signal [] [ Launch.:flow:signal:Model ] [ Launched ] [] [ Model ]";
    let alone = nexus(&[("signal", text)]);
    assert_eq!(
        alone.unused(),
        vec![Unused {
            module: 0,
            name: Name::try_from("Model").unwrap(),
        }]
    );
    let given = Nexus {
        namespaces: vec![Namespace {
            name: "flow".to_owned(),
            modules: alone.modules.clone(),
        }],
        ..alone
    };
    assert_eq!(given.verify(), Ok(()));
    assert_eq!(given.unused(), vec![]);
}
