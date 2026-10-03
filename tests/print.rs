//! The canonical print expands vertically: a structure with more than one
//! element, one of which has a next layer, opens on its line and its
//! elements hang beneath the first, aligned; the closing delimiter ends the
//! last element's line; a space inside every non-empty bracket. The text is
//! the product here, so the text is what is asserted.

use ethos_zero::{Actualizing, Canonicalizable, File, Potential, Printable};
use protos::Protosizable;

/// The kind whose capability reads ethos text to its structure and prints it.
trait Reprinting {
    fn reprint(&self) -> String;
}

impl Reprinting for str {
    fn reprint(&self) -> String {
        let canonical = self.to_owned().canonicalize().expect("sweet form opens");
        let protos = canonical.text.protosize().expect("canonical text reads");
        protos.print()
    }
}

/// The kind whose capability reads a fixture of this crate.
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
fn the_flow_nexus_four_files_round_trip_byte_identical() {
    for name in [
        "flow-library",
        "flow-signal",
        "flow-operation",
        "flow-memory",
    ] {
        let source = name.fixture();
        assert_eq!(source.reprint(), source, "{name}");
    }
}

#[test]
fn a_checked_file_prints_as_it_was_written_in_the_canonical_form() {
    let source = "flow-library".fixture();
    let file = match Potential::<File>::from(source.as_str()).actualize() {
        Ok(file) => file,
        Err(error) => panic!("the Library reads: {error:?}"),
    };
    assert_eq!(file.print(), source);
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
        "Sema [ crate:[ Handle ] ] [ Pair.{ Vector<Option<String>> Result<String Integer> } ]"
            .reprint(),
        "Sema\n[ crate:[ Handle ] ]\n[ Pair.{ Vector<Option<String>> Result<String Integer> } ]\n"
    );
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
    assert_eq!(sources.len(), 20);
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
        assert_eq!(file, repeated, "{path}");
        assert_eq!(repeated.print(), printed, "{path}");
    }
}

#[test]
fn a_reference_with_arguments_prints_as_it_was_written() {
    let source = "Library\n[]\n[ Lock.String\n  Locks.Vector<Lock>\n  Reply.[ Many.Vector<Lock>\n          Nothing ] ]\n[ Holding.{ [ Fillable ]\n            [ Item<Fillable> ]\n            [ LIMIT.Integer ]\n            [ take![ Option<Item> ] ] }\n  Fillable.[ fill![ Vector<Lock> ] ] ]\n[]\n";
    let file = match Potential::<File>::from(source).actualize() {
        Ok(file) => file,
        Err(error) => panic!("the Library reads: {error:?}"),
    };
    assert_eq!(file.print(), source);
}

#[test]
fn the_crates_own_ethos_is_written_in_the_canonical_print() {
    let root = env!("CARGO_MANIFEST_DIR");
    for name in ["ethos-zero.ethos", "error.ethos"] {
        let source = std::fs::read_to_string(format!("{root}/{name}")).expect(name);
        let uncommented: String = source
            .lines()
            .filter(|line| !line.starts_with(';'))
            .map(|line| format!("{line}\n"))
            .collect();
        let file = match Potential::<File>::from(source).actualize() {
            Ok(file) => file,
            Err(error) => panic!("{name} reads: {error:?}"),
        };
        assert_eq!(file.print(), uncommented, "{name}");
    }
}
