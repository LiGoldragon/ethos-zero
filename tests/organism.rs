//! An Organism points at the files it is made of: each pointer resolves to a
//! file the caller gave, and the file bears the root its position asks. It
//! declares nothing and imports nothing.

use ethos_zero::{
    Actualizing, ArityProblem, Assembling, Error, File, Form, Locating, Misrooted_Data, Module,
    Namespace, Nexus, Potential, Problem, Root, Rooted, Trusting, Verifying,
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

const SIGNAL: &str = "Signal [] [] [] []";

/// The namespaces of the sketch: `flow` and `harness` with a Signal each, the
/// `harness` also an Emission and a Library, and `lock` with an Organism.
fn namespaces() -> Vec<Namespace> {
    vec![
        given(
            "flow",
            &[
                ("signal", SIGNAL),
                ("library", "Library [] [ T.String ] [] []"),
            ],
        ),
        given(
            "harness",
            &[("signal", SIGNAL), ("emission", "Emission [] [] [] []")],
        ),
        given(
            "lock",
            &[("organism", "Organism\n[ :flow:signal ]\n[]\n[]\n[]\n")],
        ),
    ]
}

/// An Organism read with the namespaces given.
fn organism(text: &str) -> Nexus {
    Nexus {
        modules: modules(&[("organism", text)]),
        namespaces: namespaces(),
    }
}

/// The refusal of an Organism: its path, its problem, and its line and column.
fn refused(text: &str) -> (Vec<i64>, Problem, (i64, i64)) {
    match organism(text).verify() {
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

fn misrooted(pointer: &str, root: &str) -> Problem {
    Problem::Misrooted(Misrooted_Data {
        first_string: pointer.to_owned(),
        second_string: root.to_owned(),
    })
}

#[test]
fn the_sketch_checks_with_its_memory_empty() {
    let nexus =
        organism("Organism\n[ :flow:signal ]\n[ :harness:signal ]\n[]\n[ :lock:organism ]\n");
    assert_eq!(nexus.verify(), Ok(()));
    assert_eq!(nexus.unchecked(), vec![]);
    assert!(nexus.assemble().is_ok());
    assert_eq!(nexus.modules[0].file.root(), Root::Organism);
}

#[test]
fn an_emitter_points_at_a_signal_of_a_peer_or_an_emission() {
    for text in [
        "Organism [ :flow:signal ] [ :harness:signal ] [] []",
        "Organism [ :flow:signal ] [ :harness:emission ] [] []",
        "Organism [ :flow:signal ] [ :harness:signal :harness:emission ] [] []",
        "Organism [ :flow:signal ] [] [] []",
        "Organism [ :flow:signal :harness:signal ] [] [] []",
        // The chained spelling reads as the rooted one does.
        "Organism [ flow:signal ] [ harness:signal ] [] [ lock:organism ]",
    ] {
        assert_eq!(organism(text).verify(), Ok(()), "{text}");
    }
}

#[test]
fn a_pointer_reads_a_file_of_the_set_read_together() {
    // The Signal takes from the Library the set's namespace `x`, so `:x:signal` is the Signal.
    let nexus = Nexus {
        modules: modules(&[
            ("library", "Library [] [ T.String ] [] []"),
            ("signal", "Signal [ x:[ T ] ] [ Ask.T ] [ Told ] []"),
            ("organism", "Organism [ :x:signal ] [] [] [ :x:organism ]"),
        ]),
        namespaces: vec![],
    };
    assert_eq!(nexus.verify(), Ok(()));
    let nexus = Nexus {
        modules: modules(&[
            ("library", "Library [] [ T.String ] [] []"),
            ("signal", "Signal [ x:[ T ] ] [ Ask.T ] [ Told ] []"),
            ("organism", "Organism [ :x:signal ] [] [] [ :x:nothing ]"),
        ]),
        namespaces: vec![],
    };
    let refusal = nexus.verify().unwrap_err();
    assert_eq!(refusal.module, 2);
}

#[test]
fn an_organism_generates_the_minimal_module() {
    let nexus = organism("Organism [ :flow:signal ] [] [] []");
    let assembled = nexus.assemble().unwrap();
    assert_eq!(assembled.len(), 1);
    assert_eq!(assembled[0].stem, "organism");
    assert!(!assembled[0].rust.contains("pub "), "{}", assembled[0].rust);
    assert!(assembled[0].rust.contains("extern crate alloc;"));
}

#[test]
fn an_organism_of_another_count_of_sections_is_an_arity_error() {
    for (text, found) in [
        ("Organism\n[ :flow:signal ]\n[]\n[]\n", 3),
        ("Organism\n[ :flow:signal ]\n[]\n[]\n[]\n[]\n", 5),
    ] {
        match Potential::<File>::from(text).actualize() {
            Err(Error::Conceptual(data)) => {
                assert_eq!(data.integer_vector, vec![1], "{text}");
                assert_eq!(data.problem, Problem::arity(4, found), "{text}");
            }
            other => panic!("{text}: an arity error is wanted: {other:?}"),
        }
    }
}

#[test]
fn an_empty_receptor_is_refused() {
    let (path, problem, location) = refused("Organism\n[]\n[ :harness:signal ]\n[]\n[]\n");
    assert_eq!(path, vec![1, 0]);
    assert_eq!(problem, Problem::Empty);
    assert_eq!(location, (2, 1));
}

#[test]
fn two_memory_pointers_are_refused() {
    let (path, problem, location) =
        refused("Organism\n[ :flow:signal ]\n[]\n[ :flow:signal\n  :harness:signal ]\n[]\n");
    assert_eq!(path, vec![1, 2]);
    assert_eq!(problem, Problem::arity(1, 2));
    assert_eq!(location, (4, 1));
}

#[test]
fn a_pointer_to_no_file_given_is_refused_at_its_line_and_column() {
    let (path, problem, location) =
        refused("Organism\n[ :flow:signal ]\n[ :harness:signal\n  :harness:ghost ]\n[]\n[]\n");
    assert_eq!(path, vec![1, 1, 1]);
    assert_eq!(problem, Problem::Unpointed("harness:ghost".to_owned()));
    assert_eq!(location, (4, 3));
    // A namespace the caller did not give names no file.
    let (path, problem, location) = refused("Organism\n[ :elsewhere:signal ]\n[]\n[]\n[]\n");
    assert_eq!(path, vec![1, 0, 0]);
    assert_eq!(problem, Problem::Unpointed("elsewhere:signal".to_owned()));
    assert_eq!(location, (2, 3));
    // A Library is no file of a stem: the namespace alone names it.
    let (_, problem, _) = refused("Organism\n[ :flow:library ]\n[]\n[]\n[]\n");
    assert_eq!(problem, Problem::Unpointed("flow:library".to_owned()));
}

#[test]
fn a_pointer_to_a_file_of_another_root_is_refused() {
    for (text, path, pointer, root, location) in [
        (
            "Organism\n[ :lock:organism ]\n[]\n[]\n[]\n",
            vec![1, 0, 0],
            "lock:organism",
            "Organism",
            (2, 3),
        ),
        (
            "Organism\n[ :flow ]\n[]\n[]\n[]\n",
            vec![1, 0, 0],
            "flow",
            "Library",
            (2, 3),
        ),
        (
            "Organism\n[ :flow:signal ]\n[ :lock:organism ]\n[]\n[]\n",
            vec![1, 1, 0],
            "lock:organism",
            "Organism",
            (3, 3),
        ),
        // The memory asks a Matrix file: a Signal is not one.
        (
            "Organism\n[ :flow:signal ]\n[]\n[ :flow:signal ]\n[]\n",
            vec![1, 2, 0],
            "flow:signal",
            "Signal",
            (4, 3),
        ),
        (
            "Organism\n[ :flow:signal ]\n[]\n[]\n[ :harness:emission ]\n",
            vec![1, 3, 0],
            "harness:emission",
            "Emission",
            (5, 3),
        ),
    ] {
        let (found, problem, at) = refused(text);
        assert_eq!(found, path, "{text}");
        assert_eq!(problem, misrooted(pointer, root), "{text}");
        assert_eq!(at, location, "{text}");
    }
}

#[test]
fn a_headed_pair_to_one_file_checks_and_the_same_unheaded_twice_does_not() {
    let nexus =
        organism("Organism\n[ Codex:harness:signal\n  Claude:harness:signal ]\n[]\n[]\n[]\n");
    assert_eq!(nexus.verify(), Ok(()));
    let (path, problem, location) =
        refused("Organism\n[ :harness:signal\n  harness:signal ]\n[]\n[]\n[]\n");
    assert_eq!(path, vec![1, 0, 1]);
    assert_eq!(problem, Problem::Duplicate("harness:signal".to_owned()));
    assert_eq!(location, (3, 3));
    // Two pointers of one head are one too many, though they point at two files.
    let (path, problem, location) =
        refused("Organism\n[ Codex:harness:signal\n  Codex:flow:signal ]\n[]\n[]\n[]\n");
    assert_eq!(path, vec![1, 0, 1]);
    assert_eq!(problem, Problem::Duplicate("Codex:flow:signal".to_owned()));
    assert_eq!(location, (3, 3));
    // The same file may stand once in each position.
    let nexus = organism("Organism [ :harness:signal ] [ :harness:signal ] [] []");
    assert_eq!(nexus.verify(), Ok(()));
}

#[test]
fn a_pointer_with_a_name_after_it_is_no_pointer() {
    for text in [
        "Organism\n[ :flow:Signal ]\n[]\n[]\n[]\n",
        "Organism\n[ :flow:signal:[ Ask ] ]\n[]\n[]\n[]\n",
        "Organism\n[ Codex ]\n[]\n[]\n[]\n",
    ] {
        let error = Potential::<File>::from(text).actualize().unwrap_err();
        match error {
            Error::Conceptual(data) => {
                assert_eq!(data.problem, Problem::Expected(Form::Pointer), "{text}");
                assert_eq!(data.integer_vector, vec![1, 0, 0], "{text}");
            }
            Error::Structural(_) => panic!("{text}: a conceptual error is wanted"),
        }
    }
}

/// Enabled once the Matrix root is merged: a memory pointing at a Matrix file checks.
#[test]
#[ignore = "enable after the merge with the Matrix root, which this branch does not read"]
fn a_memory_points_at_a_matrix_file() {
    let mut namespaces = namespaces();
    namespaces.push(given("memory", &[("matrix", "Matrix\n[]\n[]\n")]));
    let nexus = Nexus {
        modules: modules(&[(
            "organism",
            "Organism [ :flow:signal ] [] [ :memory:matrix ] []",
        )]),
        namespaces,
    };
    assert_eq!(nexus.verify(), Ok(()));
}
