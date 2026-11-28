//! An import resolves against the namespaces the caller gives: one that names a
//! module of a namespace given finds each name it takes there or is refused as
//! `UnknownName`, where it is written; one that names no module of any is
//! trusted, and listed as unchecked.

use ethos_zero::{
    Actualizing, Assembling, Error, File, Generating, Locating, Module, Name, Namespace, Nexus,
    Potential, Problem, Trusting, Unchecked, UnknownName_Data, Verifying, error,
};

/// Files written in place, each a module as its stem and its text.
fn modules(files: &[(&str, &str)]) -> Vec<Module> {
    let mut read = Vec::new();
    for (stem, text) in files {
        let file = match Potential::<File>::from(*text).actualize() {
            Ok(file) => file,
            Err(error) => panic!("{stem} does not read: {error:?}"),
        };
        read.push(Module {
            stem: (*stem).to_owned(),
            file,
        });
    }
    read
}

/// A namespace given: its name and its files.
fn given(name: &str, files: &[(&str, &str)]) -> Namespace {
    Namespace {
        name: name.to_owned(),
        modules: modules(files),
    }
}

/// The namespace `x`: a Library at its root, a Signal, a Memory and an Emission.
fn x() -> Namespace {
    given(
        "x",
        &[
            (
                "library",
                "Library [] [ T.String U.[ A B ] ] [ Speaking.[] ] []",
            ),
            (
                "signal",
                "Signal [] [ Ask.{ String } Tell ] [ Told.String Said ] [ Payload.String ]",
            ),
            (
                "memory",
                "Memory [] [ Note.{ String } ] [ Add.String ] [ Added.String ] []",
            ),
            ("emission", "Emission [] [ Go.String ] [ Gone ] []"),
        ],
    )
}

/// A file alone, read with the namespaces given.
fn alone(text: &str, namespaces: Vec<Namespace>) -> Nexus {
    Nexus {
        modules: modules(&[("file", text)]),
        namespaces,
    }
}

/// The refusal of a file read alone with `x` given: its path, its problem, and
/// its line and column in the text.
fn refused(text: &str) -> (Vec<i64>, Problem, (i64, i64)) {
    match alone(text, vec![x()]).verify() {
        Err(refusal) => {
            let location = Potential::<File>::from(text).locate(&refusal.error);
            match refusal.error {
                Error::Conceptual(data) => (
                    data.integer_vector,
                    data.problem,
                    (location.line.0, location.column.0),
                ),
                Error::Structural(_) => panic!("a conceptual refusal is wanted"),
            }
        }
        Ok(()) => panic!("{text} must be refused"),
    }
}

fn unknown(namespace: &str, name: &str) -> Problem {
    Problem::UnknownName(UnknownName_Data {
        namespace: error::Namespace(namespace.to_owned()),
        name: error::Name(name.to_owned()),
    })
}

fn names(names: &[&str]) -> Vec<Name> {
    let mut read = Vec::new();
    for name in names {
        read.push(Name::try_from(*name).unwrap());
    }
    read
}

#[test]
fn an_import_a_given_namespace_provides_checks_and_is_not_unchecked() {
    for text in [
        "Library [ :x:T ] [ R.{ T } ] [] []",
        "Library [ :x:[ T U Speaking ] ] [ R.{ T U } ] [] []",
        "Library [ x:[ T ] ] [ R.{ T } ] [] []",
        "Library [ x:T ] [ R.{ T } ] [] []",
        "Library [] [ R.{ :x:T x:U } ] [] []",
        "Library [] [ Topic:x:T ] [] []",
        // Each input, output and declared type of the Signal, and its implied enums and traits.
        "Library [ :x:signal:[ Input Output Ask Tell Told Said Payload Answering Answerable ] ] [] [] []",
        // A record, a change and an answer of the Memory.
        "Library [ :x:memory:[ Note Add Added ] ] [] [] []",
        // The Emission's enums and the traits it generates.
        "Library [ :x:emission:[ Emission Outcome Go Gone Emitted Emittable ] ] [] [] []",
        // A name the source calls otherwise, taken under a name of the file's own.
        "Library [ :x:[ Text.T ] ] [ R.{ Text } ] [] []",
    ] {
        let nexus = alone(text, vec![x()]);
        assert!(nexus.verify().is_ok(), "{text}: {:?}", nexus.verify());
        assert_eq!(nexus.unchecked(), vec![], "{text}");
        assert!(nexus.assemble().is_ok(), "{text}");
    }
}

#[test]
fn a_rooted_import_a_given_namespace_does_not_provide_is_refused_at_its_name() {
    let (path, problem, location) = refused("Library\n[ :x:Ghost ]\n[]\n[]\n[]\n");
    assert_eq!(path, vec![1, 0, 0]);
    assert_eq!(problem, unknown("x", "Ghost"));
    assert_eq!(location, (2, 3));
}

#[test]
fn a_grouped_import_is_refused_at_the_name_the_namespace_does_not_provide() {
    let (path, problem, location) =
        refused("Library\n[ :x:U\n  :x:[ T\n        Ghost ] ]\n[]\n[]\n[]\n");
    assert_eq!(path, vec![1, 0, 2, 1]);
    assert_eq!(problem, unknown("x", "Ghost"));
    assert_eq!(location, (4, 9));
}

#[test]
fn a_chained_import_is_refused_at_the_name_the_namespace_does_not_provide() {
    let (path, problem, location) = refused("Library\n[ x:[ T Ghost ] ]\n[]\n[]\n[]\n");
    assert_eq!(path, vec![1, 0, 0, 1, 1]);
    assert_eq!(problem, unknown("x", "Ghost"));
    assert_eq!(location, (2, 9));
}

#[test]
fn an_inline_import_in_a_field_is_refused_where_it_stands() {
    let (path, problem, location) =
        refused("Library\n[]\n[ R.{ String\n       :x:signal:Ghost } ]\n[]\n[]\n");
    assert_eq!(path, vec![1, 1, 0, 1, 1]);
    assert_eq!(problem, unknown("x:signal", "Ghost"));
    assert_eq!(location, (4, 8));
}

#[test]
fn an_inline_import_in_an_argument_a_variant_and_a_capability_is_refused() {
    // Each refused at the reference, where its source begins.
    for (text, namespace, at) in [
        ("Library [] [ R.{ Option<x:Ghost> } ] [] []", "x", "x:Ghost"),
        ("Library [] [ E.[ A.:x:Ghost B ] ] [] []", "x", ":x:Ghost"),
        (
            "Signal [] [ Ask.x:signal:Ghost ] [ Told ] []",
            "x:signal",
            "x:signal",
        ),
        (
            "Library [] [] [ K.[ run.{ [ Self ] [ x:Ghost ] } ] ] []",
            "x",
            "x:Ghost",
        ),
        (
            "Library [] [] [ K.{ [ x:Ghost ] [] [] [] } ] []",
            "x",
            "x:Ghost",
        ),
        (
            "Library [] [ Topic:x:memory:Ghost ] [] []",
            "x:memory",
            "x:memory",
        ),
    ] {
        let nexus = alone(text, vec![x()]);
        match nexus.verify() {
            Err(refusal) => match refusal.error {
                Error::Conceptual(data) => {
                    assert_eq!(data.problem, unknown(namespace, "Ghost"), "{text}");
                    let location = Potential::<File>::from(text).locate(&Error::Conceptual(data));
                    let at = text.find(at).unwrap();
                    assert_eq!(
                        (location.line.0, location.column.0),
                        (1, text[..at].chars().count() as i64 + 1),
                        "{text}"
                    );
                }
                Error::Structural(_) => panic!("{text}: a conceptual refusal is wanted"),
            },
            Ok(()) => panic!("{text} must be refused"),
        }
    }
}

#[test]
fn a_signals_input_and_output_resolve_and_a_ghost_does_not() {
    let x = given(
        "x",
        &[("signal", "Signal [] [ Ask.{ String } ] [ Told.String ] []")],
    );
    let resolved = alone(
        "Library [ :x:signal:[ Input Output ] ] [] [] []",
        vec![x.clone()],
    );
    assert!(resolved.verify().is_ok(), "{:?}", resolved.verify());
    let ghost = alone("Library [ :x:signal:[ Ghost ] ] [] [] []", vec![x]);
    match ghost.verify() {
        Err(refusal) => assert_eq!(
            refusal.error,
            Error::Conceptual(ethos_zero::Conceptual_Data {
                integer_vector: vec![1, 0, 1, 0],
                problem: unknown("x:signal", "Ghost"),
            })
        ),
        Ok(()) => panic!("a ghost of the signal is refused"),
    }
}

#[test]
fn a_signal_without_inputs_generates_no_input_and_provides_none() {
    let x = given("x", &[("signal", "Signal [] [] [ Told ] []")]);
    let refused = alone("Library [ :x:signal:[ Output Input ] ] [] [] []", vec![x]);
    match refused.verify() {
        Err(refusal) => assert_eq!(
            refusal.error,
            Error::Conceptual(ethos_zero::Conceptual_Data {
                integer_vector: vec![1, 0, 1, 1],
                problem: unknown("x:signal", "Input"),
            })
        ),
        Ok(()) => panic!("an Input the Signal does not generate is refused"),
    }
}

#[test]
fn a_file_named_for_its_crate_is_the_crates_root_namespace_whatever_its_root() {
    let protos = given(
        "protos",
        &[
            ("protos", "Signal [] [ Ask ] [ Told ] [ Extent.String ]"),
            ("protos-kinds", "Library [] [] [ Spendable.[] ] []"),
        ],
    );
    let resolved = alone(
        "Library [ :protos:[ Extent Input Spendable ] ] [] [] []",
        vec![protos.clone()],
    );
    assert!(resolved.verify().is_ok(), "{:?}", resolved.verify());
    // A hyphen of the stem is read as an underscore.
    let codec = given(
        "datom_codec",
        &[("datom-codec", "Signal [] [] [] [ Path.String ]")],
    );
    let resolved = alone("Library [ datom_codec:[ Path ] ] [] [] []", vec![codec]);
    assert!(resolved.verify().is_ok(), "{:?}", resolved.verify());
    let ghost = alone("Library [ :protos:[ Ghost ] ] [] [] []", vec![protos]);
    assert!(matches!(
        ghost.verify(),
        Err(refusal) if refusal.error == Error::Conceptual(ethos_zero::Conceptual_Data {
            integer_vector: vec![1, 0, 1, 0],
            problem: unknown("protos", "Ghost"),
        })
    ));
}

#[test]
fn an_intrinsic_taken_from_protos_is_the_intrinsic_whether_protos_is_given_or_not() {
    let protos = given(
        "protos",
        &[("protos", "Library [] [ Extent.String ] [] []")],
    );
    for namespaces in [vec![protos], vec![]] {
        let nexus = alone(
            "Library [ protos:[ String Extent ] ] [ R.{ :protos:Integer } ] [] []",
            namespaces.clone(),
        );
        assert!(nexus.verify().is_ok(), "{:?}", nexus.verify());
        let expected = if namespaces.is_empty() {
            vec![Unchecked {
                module: 0,
                namespace: "protos".to_owned(),
                names: names(&["Extent"]),
            }]
        } else {
            vec![]
        };
        assert_eq!(nexus.unchecked(), expected);
    }
}

#[test]
fn an_import_of_a_namespace_not_given_checks_and_is_listed_unchecked() {
    let nexus = Nexus {
        modules: modules(&[
            (
                "first",
                "Library [ y:[ A B ] :flow_nexus:generated:memory:[ Event ] x:[ T ] ] [ R.{ y:C :x:deep:sub:D y:A } ] [] []",
            ),
            (
                "second",
                "Signal [ :x:ghost:[ G ] ] [ Ask.z:Q ] [ Told ] []",
            ),
        ]),
        namespaces: vec![x()],
    };
    assert!(nexus.verify().is_ok(), "{:?}", nexus.verify());
    assert_eq!(
        nexus.unchecked(),
        vec![
            Unchecked {
                module: 0,
                namespace: "y".to_owned(),
                names: names(&["A", "B", "C"]),
            },
            Unchecked {
                module: 0,
                namespace: "flow_nexus:generated:memory".to_owned(),
                names: names(&["Event"]),
            },
            Unchecked {
                module: 0,
                namespace: "x:deep:sub".to_owned(),
                names: names(&["D"]),
            },
            Unchecked {
                module: 1,
                namespace: "x:ghost".to_owned(),
                names: names(&["G"]),
            },
            Unchecked {
                module: 1,
                namespace: "z".to_owned(),
                names: names(&["Q"]),
            },
        ]
    );
}

#[test]
fn generate_refuses_what_check_refuses() {
    let nexus = alone("Library [ :x:[ Ghost ] ] [] [] []", vec![x()]);
    assert_eq!(nexus.assemble().unwrap_err(), nexus.verify().unwrap_err());
}

/// The Tally Nexus's roots, as the CLI reads them.
fn tally() -> Vec<Module> {
    let root = env!("CARGO_MANIFEST_DIR");
    let mut read = Vec::new();
    for stem in ["memory", "process", "signal"] {
        let text =
            std::fs::read_to_string(format!("{root}/fixtures/nexus/tally/{stem}.ethos")).unwrap();
        read.push(Module {
            stem: stem.to_owned(),
            file: Potential::<File>::from(text).actualize().unwrap(),
        });
    }
    read
}

#[test]
fn the_sets_own_namespace_resolves_its_imports_given_or_not() {
    let inferred = Nexus {
        modules: tally(),
        namespaces: vec![],
    };
    assert!(inferred.verify().is_ok());
    assert_eq!(inferred.unchecked(), vec![]);
    let given = Nexus {
        modules: tally(),
        namespaces: vec![Namespace {
            name: "tally".to_owned(),
            modules: tally(),
        }],
    };
    assert!(given.verify().is_ok(), "{:?}", given.verify());
    assert_eq!(given.unchecked(), vec![]);
    assert_eq!(given.assemble(), inferred.assemble());
}

#[test]
fn the_sets_own_namespace_given_checks_its_imports_like_any_other() {
    let mut roots = tally();
    // The memory and the signal, and a Library naming what the signal does not declare.
    roots.remove(1);
    roots.push(Module {
        stem: "library".to_owned(),
        file: Potential::<File>::from("Library [ tally:signal:[ Ghost ] ] [] [] []")
            .actualize()
            .unwrap(),
    });
    // Inferred, the namespace is the one whose imports all land; with a ghost
    // none is, and the imports are trusted as before.
    let inferred = Nexus {
        modules: roots.clone(),
        namespaces: vec![],
    };
    assert!(inferred.verify().is_ok(), "{:?}", inferred.verify());
    let given = Nexus {
        namespaces: vec![Namespace {
            name: "tally".to_owned(),
            modules: roots.clone(),
        }],
        modules: roots,
    };
    let refusal = given.verify().unwrap_err();
    assert_eq!(refusal.module, 2);
    assert_eq!(
        refusal.error,
        Error::Conceptual(ethos_zero::Conceptual_Data {
            integer_vector: vec![1, 0, 0, 1, 1, 0],
            problem: unknown("tally:signal", "Ghost"),
        })
    );
}

/// The namespace `flow` of two Signals, `signal` and `meta`, each generating
/// an `Input` and an `Output`.
fn flow() -> Namespace {
    given(
        "flow",
        &[
            ("signal", "Signal [] [ Ask.String ] [ Told.String ] []"),
            (
                "meta",
                "Signal [] [ Refresh.String ] [ Refreshed.String ] []",
            ),
        ],
    )
}

/// The refusal of a file read alone with `flow` given: its path and its problem.
fn refused_with_flow(text: &str) -> (Vec<i64>, Problem) {
    match alone(text, vec![flow()]).verify() {
        Err(refusal) => match refusal.error {
            Error::Conceptual(data) => (data.integer_vector, data.problem),
            Error::Structural(_) => panic!("a conceptual refusal is wanted"),
        },
        Ok(()) => panic!("{text} must be refused"),
    }
}

#[test]
fn one_name_imported_from_two_namespaces_checks_until_it_is_written_bare() {
    let imports = ":flow:signal:[ Input Output ] :flow:meta:[ Input Output ]";
    for text in [
        format!("Library [ {imports} ] [] [] []"),
        format!("Library [ {imports} ] [ Holder.{{ :flow:signal:Input }} ] [] []"),
        format!("Library [ {imports} ] [ Holder.[ Some.:flow:meta:Output None ] ] [] []"),
        // Chained and rooted spellings of the two namespaces.
        "Library [ flow:signal:[ Input ] :flow:meta:Input ] [] [] []".to_owned(),
    ] {
        let nexus = alone(&text, vec![flow()]);
        assert!(nexus.verify().is_ok(), "{text}: {:?}", nexus.verify());
        assert!(nexus.assemble().is_ok(), "{text}");
    }
    let ambiguous = Problem::Ambiguous(ethos_zero::error::Name("Input".to_owned()));
    // Written bare in a position.
    assert_eq!(
        refused_with_flow(&format!(
            "Library [ {imports} ] [ Holder.{{ Input }} ] [] []"
        )),
        (vec![1, 1, 0, 1, 0], ambiguous.clone())
    );
    // Written bare as a variant's payload, and as a bare variant that would carry it.
    assert_eq!(
        refused_with_flow(&format!(
            "Library [ {imports} ] [ Holder.[ Some.Input None ] ] [] []"
        )),
        (vec![1, 1, 0, 1, 0, 1], ambiguous.clone())
    );
    assert_eq!(
        refused_with_flow(&format!(
            "Library [ {imports} ] [ Holder.[ Input None ] ] [] []"
        )),
        (vec![1, 1, 0, 1, 0], ambiguous)
    );
}

#[test]
fn a_name_imported_twice_from_one_namespace_or_also_declared_is_still_a_duplicate() {
    let duplicate = Problem::Duplicate(ethos_zero::error::Name("Input".to_owned()));
    assert_eq!(
        refused_with_flow("Library [ :flow:signal:[ Input ] :flow:signal:[ Input ] ] [] [] []"),
        (vec![1, 0, 3, 0], duplicate.clone())
    );
    assert_eq!(
        refused_with_flow(
            "Library [ :flow:signal:[ Input ] :flow:meta:[ Input ] ] [ Input.String ] [] []"
        ),
        (vec![1, 1, 0, 0], duplicate)
    );
}

#[test]
fn a_qualified_use_of_an_ambiguous_name_generates_its_namespace_path() {
    let text = "Library [ :flow:signal:[ Input ] :flow:meta:[ Input ] ] [ Holder.{ :flow:signal:Input } Both.{ :flow:signal:Input :flow:meta:Input } ] [] []";
    let file = match Potential::<File>::from(text).actualize() {
        Ok(file) => file,
        Err(error) => panic!("{text} does not read: {error:?}"),
    };
    let rust = match file.generate() {
        Ok(rust) => rust,
        Err(error) => panic!("{text} does not generate: {error:?}"),
    };
    assert!(rust.contains("pub input: ::flow::signal::Input,"), "{rust}");
    // Two positions of one name from two namespaces are named first and
    // second, as `Errors.{ :a:Error :b:Error }` is.
    assert!(
        rust.contains("pub first_input: ::flow::signal::Input,"),
        "{rust}"
    );
    assert!(
        rust.contains("pub second_input: ::flow::meta::Input,"),
        "{rust}"
    );
}

/// The Models Nexus's roots, as the CLI reads them: a Signal naming the open
/// enum `Model`, and a Matrix importing from the contract module `model` it
/// generates.
fn models() -> Vec<Module> {
    let root = env!("CARGO_MANIFEST_DIR");
    let mut read = Vec::new();
    for stem in ["matrix", "signal"] {
        let text =
            std::fs::read_to_string(format!("{root}/fixtures/nexus/models/{stem}.ethos")).unwrap();
        read.push(Module {
            stem: stem.to_owned(),
            file: Potential::<File>::from(text).actualize().unwrap(),
        });
    }
    read
}

/// The namespace `models`: the Models Nexus's Signal alone.
fn models_signal() -> Namespace {
    let mut modules = models();
    modules.remove(0);
    Namespace {
        name: "models".to_owned(),
        modules,
    }
}

#[test]
fn an_import_from_an_open_enums_module_checks_against_what_is_generated_in_it() {
    // Every name the contract module generates, and only those.
    for text in [
        "Matrix [ models:signal:model:[ Variant ] ] [ Models.⟦Variant⟧ ]",
        "Library [ :models:signal:model:[ Name Word Namespace Import Declaration Variant Type Input Output Registry ] ] [] [] []",
        "Library [] [ R.{ models:signal:model:Variant } ] [] []",
    ] {
        let nexus = alone(text, vec![models_signal()]);
        assert!(nexus.verify().is_ok(), "{text}: {:?}", nexus.verify());
        assert_eq!(nexus.unchecked(), vec![], "{text}");
        assert!(nexus.assemble().is_ok(), "{text}");
    }
    for (text, path, ghost) in [
        (
            "Matrix [ models:signal:model:[ Nonesuch ] ] [ Models.⟦Nonesuch⟧ ]",
            vec![1, 0, 0, 1, 1, 1, 0],
            "Nonesuch",
        ),
        // The Signal's own names are not its open enum's.
        (
            "Library [ models:signal:model:[ Variant Carried ] ] [] [] []",
            vec![1, 0, 0, 1, 1, 1, 1],
            "Carried",
        ),
        // Rust the module holds that no Ethos name generates.
        (
            "Library [ models:signal:model:[ Applied_Data ] ] [] [] []",
            vec![1, 0, 0, 1, 1, 1, 0],
            "Applied_Data",
        ),
    ] {
        match alone(text, vec![models_signal()]).verify() {
            Err(refusal) => assert_eq!(
                refusal.error,
                Error::Conceptual(ethos_zero::Conceptual_Data {
                    integer_vector: path,
                    problem: unknown("models:signal:model", ghost),
                }),
                "{text}"
            ),
            Ok(()) => panic!("{text} must be refused"),
        }
    }
}

#[test]
fn a_module_of_no_open_enum_the_signal_names_is_not_a_contract() {
    // The Signal names no open enum `Effort`, and a Library has no contract
    // module: neither source names a module of the namespace, and is trusted.
    let nexus = alone(
        "Library [ models:signal:effort:[ Variant ] models:model:[ Variant ] ] [] [] []",
        vec![models_signal()],
    );
    assert!(nexus.verify().is_ok(), "{:?}", nexus.verify());
    assert_eq!(
        nexus.unchecked(),
        vec![
            Unchecked {
                module: 0,
                namespace: "models:signal:effort".to_owned(),
                names: names(&["Variant"]),
            },
            Unchecked {
                module: 0,
                namespace: "models:model".to_owned(),
                names: names(&["Variant"]),
            },
        ]
    );
}

#[test]
fn the_sets_open_enum_module_resolves_its_imports_given_or_not() {
    let inferred = Nexus {
        modules: models(),
        namespaces: vec![],
    };
    assert!(inferred.verify().is_ok(), "{:?}", inferred.verify());
    assert_eq!(inferred.unchecked(), vec![]);
    let given = Nexus {
        modules: models(),
        namespaces: vec![models_signal()],
    };
    assert!(given.verify().is_ok(), "{:?}", given.verify());
    assert_eq!(given.unchecked(), vec![]);
    assert_eq!(given.assemble(), inferred.assemble());
    // The import's Rust path is the module the contract is generated into.
    let assembled = inferred.assemble().unwrap();
    let matrix = &assembled[0].rust;
    assert!(
        matrix.contains(
            "pub struct Models(pub ::alloc::vec::Vec<::models::signal::model::Variant>);"
        ),
        "{matrix}"
    );
    let signal = &assembled[1].rust;
    assert!(signal.contains("pub mod model {"), "{signal}");
    assert!(signal.contains("pub struct Variant {"), "{signal}");
}

#[test]
fn a_ghost_of_the_sets_open_enum_module_is_refused_when_the_namespace_is_given() {
    let mut roots = models();
    roots[0].file = Potential::<File>::from(
        "Matrix [ models:signal:model:[ Nonesuch ] ] [ Models.⟦Nonesuch⟧ ]",
    )
    .actualize()
    .unwrap();
    // Inferred, no namespace lands, and the import is trusted as any ghost of
    // the set's own namespace is.
    let inferred = Nexus {
        modules: roots.clone(),
        namespaces: vec![],
    };
    assert!(inferred.verify().is_ok(), "{:?}", inferred.verify());
    assert_eq!(
        inferred.unchecked(),
        vec![Unchecked {
            module: 0,
            namespace: "models:signal:model".to_owned(),
            names: names(&["Nonesuch"]),
        }]
    );
    let given = Nexus {
        namespaces: vec![Namespace {
            name: "models".to_owned(),
            modules: roots.clone(),
        }],
        modules: roots,
    };
    let refusal = given.verify().unwrap_err();
    assert_eq!(refusal.module, 0);
    assert_eq!(
        refusal.error,
        Error::Conceptual(ethos_zero::Conceptual_Data {
            integer_vector: vec![1, 0, 0, 1, 1, 1, 0],
            problem: unknown("models:signal:model", "Nonesuch"),
        })
    );
}
