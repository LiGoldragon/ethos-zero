//! The canonical print expands vertically: a structure with more than one
//! element, one of which has a next layer, opens on its line and its
//! elements hang beneath the first, aligned; the closing delimiter ends the
//! last element's line; a space inside every non-empty bracket. The text is
//! the product here, so the text is what is asserted.

use ethos_zero::{Actualizing, File, Generating, Potential, Printable};

/// The trait whose capability reads ethos text to its structure and prints it.
trait Reprinting {
    fn reprint(&self) -> String;
}

impl Reprinting for str {
    fn reprint(&self) -> String {
        let file = Potential::<File>::from(self)
            .actualize()
            .expect("sweet form reads");
        file.print()
    }
}

/// The trait whose capability reads a fixture of this crate.
trait Fixture {
    fn fixture(&self) -> String;
}

impl Fixture for str {
    fn fixture(&self) -> String {
        let root = env!("CARGO_MANIFEST_DIR");
        std::fs::read_to_string(format!("{root}/fixtures/print/{self}.ethos")).expect(self)
    }
}

#[test]
fn the_flow_nexus_four_files_round_trip_with_tight_vectors() {
    for name in [
        "flow-library",
        "flow-signal",
        "flow-operation",
        "flow-memory",
    ] {
        let source = name.fixture();
        let file = Potential::<File>::from(source.as_str())
            .actualize()
            .unwrap_or_else(|error| panic!("{name}: {error:?}"));
        let printed = file.print();
        let reread = Potential::<File>::from(printed.as_str())
            .actualize()
            .unwrap_or_else(|error| panic!("{name}: {error:?}"));
        assert_eq!(reread.print(), printed, "{name}");
        assert_eq!(
            reread.generate().unwrap(),
            file.generate().unwrap(),
            "{name}"
        );
    }
}

#[test]
fn a_flow_file_keeps_its_white_vectors_and_meaning_when_printed() {
    let source = "flow-memory".fixture();
    let file = match Potential::<File>::from(source.as_str()).actualize() {
        Ok(file) => file,
        Err(error) => panic!("the Library reads: {error:?}"),
    };
    let printed = file.print();
    assert!(printed.contains('⟦'));
    let reread = Potential::<File>::from(printed.as_str())
        .actualize()
        .unwrap();
    assert_eq!(reread.print(), printed);
    assert_eq!(reread.generate().unwrap(), file.generate().unwrap());
}

#[test]
fn a_one_line_layout_expands_to_the_canonical_print() {
    let written =
        "Library [] [ Flow.{ FlowId Voice State.[ Running Ended ] Vector<Event> } ] [] []";
    assert_eq!(
        written.reprint(),
        "Library\n[]\n[ Flow.{ FlowId\n         Voice\n         State.[ Running Ended ]\n         Vector<Event> } ]\n[]\n[]\n"
    );
}

#[test]
fn leaves_stay_on_one_line_and_angles_stay_tight() {
    assert_eq!(
        "Memory [ crate:[ Handle ] ] [ Pair.{ Vector<Option<String>> Result<String Integer> } ] [] [] []"
            .reprint(),
        "Memory\n[ crate:[ Handle ] ]\n[ Pair.{ Vector<Option<String>> Result<String Integer> } ]\n[]\n[]\n[]\n"
    );
}

#[test]
fn a_matrix_prints_back_as_written() {
    let written = "Matrix\n[ flow:[ FlowId Event ] ]\n[ Watch.{ FlowId String }\n  Configured.Option<Watch>\n  Event ]\n";
    assert_eq!(
        "Matrix [ flow:[ FlowId Event ] ] [ Watch.{ FlowId String } Configured.Option<Watch> Event ]".reprint(),
        written
    );
    assert_eq!(written.reprint(), written);
}

#[test]
fn a_capability_hangs_its_inputs_and_yield() {
    assert_eq!(
        "Library [] [] [ Fillable.[ push!{ [ Summarizable ] [ Result<Integer SinkError> ] } create:[ Self ] ] ] []".reprint(),
        "Library\n[]\n[]\n[ Fillable.[ push!{ [ Summarizable ]\n                    [ Result<Integer SinkError> ] }\n             create:[ Self ] ] ]\n[]\n"
    );
}

#[test]
fn a_badly_hung_layout_is_realigned() {
    let written = "Library\n[]\n[ E.[ A.String\n          B.String ] ]\n[]\n[]\n";
    assert_eq!(
        written.reprint(),
        "Library\n[]\n[ E.[ A.String\n      B.String ] ]\n[]\n[]\n"
    );
}

#[test]
fn every_fixture_prints_to_a_text_that_reads_back_to_it_and_prints_the_same() {
    let root = env!("CARGO_MANIFEST_DIR");
    let mut sources = vec![
        format!("{root}/ethos-zero.ethos"),
        format!("{root}/error.ethos"),
    ];
    for entry in std::fs::read_dir(format!("{root}/fixtures")).expect("fixtures") {
        let path = entry.expect("fixture entry").path();
        if path.extension().and_then(|extension| extension.to_str()) == Some("ethos") {
            sources.push(path.display().to_string());
        }
    }
    // The roots of each fixture Nexus print too.
    for nexus in std::fs::read_dir(format!("{root}/fixtures/nexus")).expect("fixture Nexuses") {
        let directory = nexus.expect("Nexus entry").path();
        for entry in std::fs::read_dir(directory).expect("the roots of a Nexus") {
            sources.push(entry.expect("root entry").path().display().to_string());
        }
    }
    assert_eq!(sources.len(), 45);
    for path in sources {
        let source = std::fs::read_to_string(&path).expect("fixture reads");
        let file = match Potential::<File>::from(source).actualize() {
            Ok(file) => file,
            Err(error) => panic!("{path} reads: {error:?}"),
        };
        let printed = file.print();
        let repeated = match Potential::<File>::from(printed.as_str()).actualize() {
            Ok(file) => file,
            Err(error) => panic!("{path} printed does not read: {error:?}\n{printed}"),
        };
        assert_eq!(repeated.print(), printed, "{path}");
        if let Ok(expected) = file.generate() {
            assert_eq!(repeated.generate().unwrap(), expected, "{path}");
        }
    }
}

#[test]
fn vector_positions_print_white_while_other_angles_stay_tight() {
    let source = "Library\n[]\n[ Lock.String\n  Locks.⟦ Lock ⟧\n  Reply.[ Many.⟦ Lock ⟧\n          Nothing ] ]\n[ Holding.{ [ Fillable ]\n            [ Item<Fillable> ]\n            [ LIMIT.Integer ]\n            [ take![ Option<Item> ] ] }\n  Fillable.[ fill![ ⟦ Lock ⟧ ] ] ]\n[]\n";
    let file = match Potential::<File>::from(source).actualize() {
        Ok(file) => file,
        Err(error) => panic!("the Library reads: {error:?}"),
    };
    assert_eq!(file.print(), source.reprint());
}

#[test]
fn the_crates_own_ethos_round_trips_to_the_canonical_print() {
    let root = env!("CARGO_MANIFEST_DIR");
    for name in ["ethos-zero.ethos", "error.ethos"] {
        let source = std::fs::read_to_string(format!("{root}/{name}")).expect(name);
        let file = match Potential::<File>::from(source).actualize() {
            Ok(file) => file,
            Err(error) => panic!("{name} reads: {error:?}"),
        };
        let printed = file.print();
        let reread = Potential::<File>::from(printed.as_str())
            .actualize()
            .unwrap();
        assert_eq!(reread.print(), printed, "{name}");
        assert_eq!(
            reread.generate().unwrap(),
            file.generate().unwrap(),
            "{name}"
        );
    }
}

#[test]
fn white_vectors_round_trip_in_mixed_type_positions() {
    let source = "Library [] [ Item.String Items.⟦ Item ⟧ Reply.[ Items.⟦ Option<⟦ Item ⟧> ⟧ Empty ] Mixed.{ ⟦ ⟦ Item ⟧ ⟧ ⟦⟦Item⟧⟧ } ] [ Listing.[ list![ ⟦ Item ⟧ ] ] ] []";
    let file = Potential::<File>::from(source).actualize().unwrap();
    let printed = file.print();
    for spelling in ["Items.⟦Item⟧", "Items.⟦Option<⟦Item⟧>⟧", "⟦⟦Item⟧⟧"] {
        assert!(printed.contains(spelling), "{printed}");
    }
    assert_eq!(printed.matches("⟦⟦Item⟧⟧").count(), 2);
    assert!(!printed.contains("Vector"));
    let reread = Potential::<File>::from(printed.as_str())
        .actualize()
        .unwrap();
    assert_eq!(reread.print(), printed);
    assert_eq!(reread.generate().unwrap(), file.generate().unwrap());
}

#[test]
fn valid_builtin_and_external_vectors_keep_their_own_forms_in_print() {
    for source in [
        "Library [] [ Alias.⟦ ⟦ String ⟧ ⟧ ] [] []",
        "Library [ protos:Vector ] [ Alias.⟦ String ⟧ ] [] []",
        "Library [ external:Vector ] [ Alias.Vector<⟦ String ⟧> ] [] []",
        "Library [] [ Alias.external:Vector<⟦ String ⟧> ] [] []",
    ] {
        let original = Potential::<File>::from(source).actualize().unwrap();
        let printed = original.print();
        let reread = Potential::<File>::from(printed.as_str())
            .actualize()
            .unwrap();
        assert_eq!(printed, source.reprint(), "{source}");
        assert_eq!(reread.print(), printed);
        assert_eq!(reread.generate().unwrap(), original.generate().unwrap());
    }
}

#[test]
fn unchecked_printing_does_not_repair_legacy_or_malformed_vectors() {
    let source =
        "Library [] [ Legacy.Vector<String> Bad.Vector<String Integer> Empty.Vector ] [] []";
    let file = Potential::<File>::from(source).actualize().unwrap();
    let printed = file.print();
    assert_eq!(printed, source.reprint());
    let reread = Potential::<File>::from(printed.as_str())
        .actualize()
        .unwrap();
    assert_eq!(reread.print(), printed);
    assert_eq!(reread.generate().unwrap_err(), file.generate().unwrap_err());
}

#[test]
fn an_organism_prints_back_as_written() {
    let sketch =
        "Organism\n[ :flow:signal ]\n[ :harness:signal ]\n[ :flow:matrix ]\n[ :lock:organism ]\n";
    assert_eq!(sketch.reprint(), sketch);
    // A one-line layout expands to the same print.
    assert_eq!(
        "Organism [ :flow:signal ] [ :harness:signal ] [ :flow:matrix ] [ :lock:organism ]"
            .reprint(),
        sketch
    );
    // A chained pointer stays chained, and a head stays at the pointer.
    for written in [
        "Organism\n[ flow:signal ]\n[]\n[]\n[]\n",
        "Organism\n[ Codex:harness:signal\n  Claude:harness:signal ]\n[ :flow:signal ]\n[]\n[]\n",
        "Organism\n[ :flow:signal ]\n[]\n[]\n[]\n",
        "Organism\n[]\n[]\n[]\n[]\n",
    ] {
        assert_eq!(written.reprint(), written);
        let reread = Potential::<File>::from(written).actualize().unwrap();
        assert_eq!(
            Potential::<File>::from(reread.print()).actualize().unwrap(),
            reread
        );
    }
}

#[test]
fn a_signal_of_five_sections_prints_back_as_written() {
    let written = "Signal\n[]\n[ Launch.Model\n  Stop.Model ]\n[ Launched\n  Refused.String ]\n[ TripleEffort.[ Low Medium High ]\n  QuadrupleEffort.[ Low Medium High Maximum ] ]\n[ Model ]\n";
    assert_eq!(
        "Signal [] [ Launch.Model Stop.Model ] [ Launched Refused.String ] [ TripleEffort.[ Low Medium High ] QuadrupleEffort.[ Low Medium High Maximum ] ] [ Model ]".reprint(),
        written
    );
    assert_eq!(written.reprint(), written);
    let four = "Signal\n[]\n[ Ask ]\n[ Told ]\n[]\n";
    assert_eq!(four.reprint(), four);
}
