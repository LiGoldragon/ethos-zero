//! The command accepts one generated `Query` and writes its generated module.

use std::process::Command;

use ethos_zero::{Actualizing, File, Potential, Printable};

trait Invoking {
    fn invoke(&self) -> (bool, String);
}

impl Invoking for [&str] {
    fn invoke(&self) -> (bool, String) {
        let output = Command::new(env!("CARGO_BIN_EXE_ethos-zero"))
            .args(self)
            .output()
            .expect("the binary runs");
        (
            output.status.success(),
            String::from_utf8(output.stdout).expect("utf-8 output"),
        )
    }
}

trait Scratching {
    fn scratch(&self) -> String;
}

impl Scratching for str {
    fn scratch(&self) -> String {
        let directory = format!(
            "{}/cli-{}-{}",
            env!("CARGO_TARGET_TMPDIR"),
            self,
            std::process::id()
        );
        let _ = std::fs::remove_dir_all(&directory);
        directory
    }
}

fn opaque(text: &str) -> String {
    format!("«{text}»")
}

fn generate(source: &str, directory: &str) -> String {
    format!(
        "Generate.{{ {{ [ {} ] [] }} {} }}",
        opaque(source),
        opaque(directory)
    )
}

#[test]
fn no_argument_prints_the_crates_own_ethos_ending_with_a_newline() {
    let (success, output) = [].invoke();
    assert!(success);
    // The CLI prints the
    // canonical form of the crate's white-bracket declaration.
    let source = include_str!("../ethos-zero.ethos");
    let file = Potential::<File>::from(source).actualize().unwrap();
    assert_eq!(output, file.print());
    assert!(output.ends_with('\n'));
    assert!(output.contains("Generate.Generation"));
    assert!(output.contains("Check.Reading"));
    assert!(!output.contains("Vector<"));
}

#[test]
fn generate_writes_a_named_field_module_and_replies_generated() {
    let directory = "generate".scratch();
    let source = concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures/record-types.ethos");
    let argument = generate(source, &directory);
    let (success, output) = [argument.as_str()].invoke();
    assert!(success, "{output}");
    assert_eq!(
        output,
        format!("Generated.{{ [ {directory}/record-types.rs ] [] }}\n")
    );
    let written = std::fs::read_to_string(format!("{directory}/record-types.rs")).unwrap();
    assert!(written.contains("pub struct Record"));
    assert!(written.contains("pub string: ::std::string::String"));
    assert!(written.contains("pub integer: i64"));
}

#[test]
fn quoted_inputs_canonicalize_paths_as_bare_strings() {
    let directory = "quoted".scratch();
    let source = concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures/multi-types.ethos");
    let argument = generate(source, &directory);
    let (success, output) = [argument.as_str()].invoke();
    assert!(success, "{output}");
    assert_eq!(
        output,
        format!("Generated.{{ [ {directory}/multi-types.rs ] [] }}\n")
    );
}

#[test]
fn malformed_query_and_wrong_shape_are_reported_as_typed_errors() {
    let (success, output) = ["--help"].invoke();
    assert!(!success);
    assert!(
        output.starts_with("Malformed.Error.{ Composition"),
        "{output}"
    );
    assert!(output.contains("Query --help"), "{output}");

    let (success, output) = ["Generate.{ /only }"].invoke();
    assert!(!success);
    assert!(
        output.starts_with("Malformed.Error.{ Composition"),
        "{output}"
    );
    assert!(output.contains("Arity.{ 2 1 }"), "{output}");
}

#[test]
fn more_than_one_argument_is_refused_with_the_count() {
    let (success, output) = ["Generate.{", "/a", "/b", "}"].invoke();
    assert!(!success);
    assert_eq!(output, "Arguments.4\n");
}

#[test]
fn the_roots_of_a_nexus_are_generated_together_each_into_a_module_of_its_files_name() {
    let directory = "nexus".scratch();
    let root = format!("{}/fixtures/nexus/tally", env!("CARGO_MANIFEST_DIR"));
    let argument = format!(
        "Generate.{{ {{ [ {} {} {} ] [] }} {} }}",
        opaque(&format!("{root}/signal.ethos")),
        opaque(&format!("{root}/memory.ethos")),
        opaque(&format!("{root}/process.ethos")),
        opaque(&directory)
    );
    let (success, output) = [argument.as_str()].invoke();
    assert!(success, "{output}");
    assert_eq!(
        output,
        format!(
            "Generated.{{ [ {directory}/signal.rs {directory}/memory.rs {directory}/process.rs ] [] }}\n"
        )
    );
    let process = std::fs::read_to_string(format!("{directory}/process.rs")).unwrap();
    assert!(process.contains("pub enum Process"));
    assert!(process.contains("pub struct SumProcess"));
    let checked = format!(
        "Check.{{ [ {} {} ] [] }}",
        opaque(&format!("{root}/signal.ethos")),
        opaque(&format!("{root}/process.ethos"))
    );
    let (success, output) = [checked.as_str()].invoke();
    // Without its memory the Process root names what the set does not declare.
    assert!(!success, "{output}");
    assert!(output.contains("Foreign.Add"), "{output}");
}

#[test]
fn a_refusal_of_a_nexus_names_the_file_it_lies_in() {
    let directory = "refusal".scratch();
    let signal = directory.write_source("signal", "Signal\n[]\n[ Ask ]\n[ Told ]\n[]\n");
    let process = directory.write_source(
        "process",
        "Process\n[ x:signal:[ Ask Told ] other:signal:[ Seen ] ]\n[ Ask.{ [] [ Told Seen ] } ]\n[]\n",
    );
    let argument = check_all(&[&signal, &process]);
    let (success, output) = [argument.as_str()].invoke();
    assert!(!success);
    assert_eq!(
        output,
        format!(
            "Rejected.{{ {process} {{ 3 19 }} Conceptual.{{ [ 1 1 0 1 1 1 ] Foreign.Seen }} }}\n"
        )
    );
}

#[test]
fn a_missing_file_is_unreadable() {
    let argument = generate("/nowhere/missing.ethos", "/nowhere");
    let (success, output) = [argument.as_str()].invoke();
    assert!(!success);
    assert!(
        output.starts_with("Unreadable.{ /nowhere/missing.ethos "),
        "{output}"
    );
}

/// The trait whose capability writes an ethos source into a scratch directory.
trait Writing {
    fn write_source(&self, name: &str, text: &str) -> String;
}

impl Writing for str {
    fn write_source(&self, name: &str, text: &str) -> String {
        std::fs::create_dir_all(self).unwrap();
        let source = format!("{self}/{name}.ethos");
        std::fs::write(&source, text).unwrap();
        source
    }
}

fn check_all(sources: &[&str]) -> String {
    let mut written = Vec::new();
    for source in sources {
        written.push(opaque(source));
    }
    format!("Check.{{ [ {} ] [] }}", written.join(" "))
}

fn check(source: &str) -> String {
    format!("Check.{{ [ {} ] [] }}", opaque(source))
}

#[test]
fn a_bad_ethos_file_is_rejected_at_its_line_and_column_and_nothing_is_written() {
    let directory = "erroneous".scratch();
    let source = directory.write_source(
        "erroneous",
        "Library\n[]\n[ Record.{ String Bogus } ]\n[]\n[]\n",
    );
    let target = format!("{directory}/out");
    let argument = generate(&source, &target);
    let (success, output) = [argument.as_str()].invoke();
    assert!(!success);
    assert_eq!(
        output,
        format!(
            "Rejected.{{ {source} {{ 3 19 }} Conceptual.{{ [ 1 1 0 1 1 ] Undeclared.Bogus }} }}\n"
        )
    );
    assert!(!std::path::Path::new(&target).exists());
}

#[test]
fn check_validates_a_good_file_without_writing() {
    let source = concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures/tree-types.ethos");
    let argument = check(source);
    let (success, output) = [argument.as_str()].invoke();
    assert!(success, "{output}");
    assert_eq!(output, format!("Checked.{{ [ {source} ] [] }}\n"));
}

#[test]
fn check_locates_an_undeclared_name_below_a_comment() {
    let directory = "undeclared".scratch();
    let source = directory.write_source(
        "undeclared",
        "; The comment and the sweet form's seam are counted too.\nLibrary\n[]\n[ Record.{ String\n           ⟦ Option<Bogus> ⟧ } ]\n[]\n[]\n",
    );
    let argument = check(&source);
    let (success, output) = [argument.as_str()].invoke();
    assert!(!success);
    assert_eq!(
        output,
        format!(
            "Rejected.{{ {source} {{ 5 21 }} Conceptual.{{ [ 1 1 0 1 1 1 0 ] Undeclared.Bogus }} }}\n"
        )
    );
}

#[test]
fn check_and_generate_refuse_the_same_legacy_builtin_vector_at_its_name() {
    let directory = "legacy-vector".scratch();
    let source = directory.write_source("legacy", "Library [] [ Alias.Vector<String> ] [] []");
    let outputs = format!("{directory}/unwritten");
    let expected = format!(
        "Rejected.{{ {source} {{ 1 20 }} Conceptual.{{ [ 1 1 0 1 ] Renamed.«⟦ T ⟧» }} }}\n"
    );
    for request in [check(&source), generate(&source, &outputs)] {
        assert_eq!([request.as_str()].invoke(), (false, expected.clone()));
    }
    assert!(!std::path::Path::new(&outputs).exists());
}

#[test]
fn check_refuses_a_capitalized_inner_segment_at_its_name() {
    let directory = "inline-source".scratch();
    let source = directory.write_source(
        "inline-source",
        // A chained source reads every segment, `std:sync:Mutex`; a
        // capitalized one, `Sync`, is the refusal, at that segment.
        "Library\n[]\n[ Holder.{ std:Sync:Mutex String } ]\n[]\n[]\n",
    );
    let argument = check(&source);
    let (success, output) = [argument.as_str()].invoke();
    assert!(!success);
    assert_eq!(
        output,
        format!(
            "Rejected.{{ {source} {{ 3 16 }} Conceptual.{{ [ 1 1 0 1 0 1 0 ] Name.Sync }} }}\n"
        )
    );
}

#[test]
fn check_locates_a_duplicate_at_its_second_declaration() {
    let directory = "duplicate".scratch();
    let source = directory.write_source(
        "duplicate",
        "Library\n[]\n[ Record.{ String }\n  Record.{ Integer } ]\n[]\n[]\n",
    );
    let argument = check(&source);
    let (success, output) = [argument.as_str()].invoke();
    assert!(!success);
    assert_eq!(
        output,
        format!(
            "Rejected.{{ {source} {{ 4 3 }} Conceptual.{{ [ 1 1 1 0 ] Duplicate.Record }} }}\n"
        )
    );
}

#[test]
fn check_locates_an_arity_error_at_the_reference() {
    let directory = "arity".scratch();
    let source = directory.write_source(
        "arity",
        "Library\n[]\n[ Pair.{ Integer\n         Result<String> } ]\n[]\n[]\n",
    );
    let argument = check(&source);
    let (success, output) = [argument.as_str()].invoke();
    assert!(!success);
    assert_eq!(
        output,
        format!(
            "Rejected.{{ {source} {{ 4 10 }} Conceptual.{{ [ 1 1 0 1 1 ] Arity.{{ 2 1 }} }} }}\n"
        )
    );
}

#[test]
fn check_locates_a_structural_error_at_its_extent() {
    let directory = "structural".scratch();
    let source = directory.write_source("structural", "Library\n[]\n[ Record.{ String ]\n[]\n[]\n");
    let argument = check(&source);
    let (success, output) = [argument.as_str()].invoke();
    assert!(!success);
    assert!(
        output.starts_with(&format!("Rejected.{{ {source} {{ 3 19 }} Structural.")),
        "{output}"
    );
}

#[test]
fn check_of_a_missing_file_is_unreadable() {
    let (success, output) = [check("/nowhere/missing.ethos").as_str()].invoke();
    assert!(!success);
    assert!(
        output.starts_with("Unreadable.{ /nowhere/missing.ethos "),
        "{output}"
    );
}

#[test]
fn check_takes_a_variant_named_as_a_defined_type_declaring_a_payload() {
    let directory = "payload".scratch();
    let taken = directory.write_source(
        "taken",
        "Signal\n[]\n[ Lock.LockRequest ]\n[ Locked.Lock ]\n[ LockRequest.String\n  Lock.Integer ]\n",
    );
    let argument = check(&taken);
    let (success, output) = [argument.as_str()].invoke();
    assert!(success, "{output}");
    assert_eq!(output, format!("Checked.{{ [ {taken} ] [] }}\n"));
}

#[test]
fn check_locates_an_inline_enum_of_too_few_variants_naming_the_file_and_the_enum() {
    let directory = "single".scratch();
    let refused = directory.write_source(
        "refused",
        "Library\n[]\n[ Reply.[ Accepted\n          Refused.[ Busy ] ] ]\n[]\n[]\n",
    );
    let argument = check(&refused);
    let (success, output) = [argument.as_str()].invoke();
    assert!(!success);
    assert_eq!(
        output,
        format!(
            "Rejected.{{ {refused} {{ 4 19 }} Conceptual.{{ [ 1 1 0 1 1 1 ] TooFewVariants.Refused }} }}\n"
        )
    );
}

#[test]
fn check_locates_positions_whose_fields_would_meet_naming_the_file_and_the_field() {
    let directory = "fields".scratch();
    let refused = directory.write_source(
        "refused",
        "Library\n[]\n[ FirstString.String\n  Pair.{ String\n         String\n         FirstString } ]\n[]\n[]\n",
    );
    let argument = check(&refused);
    let (success, output) = [argument.as_str()].invoke();
    assert!(!success);
    assert_eq!(
        output,
        format!(
            "Rejected.{{ {refused} {{ 6 10 }} Conceptual.{{ [ 1 1 1 1 2 ] DuplicateField.first_string }} }}\n"
        )
    );
}

#[test]
fn check_takes_a_trait_and_a_concrete_type_in_an_input() {
    let directory = "traits".scratch();
    let taken = directory.write_source(
        "taken",
        "Library\n[]\n[]\n[ Textualizable.[ textualize.[ String ] ]\n  Resolvable.[ resolve.{ [ Textualizable ]\n                         [ Self ] } ] ]\n[]\n",
    );
    let argument = check(&taken);
    let (success, output) = [argument.as_str()].invoke();
    assert!(success, "{output}");
    assert_eq!(output, format!("Checked.{{ [ {taken} ] [] }}\n"));

    let typed = directory.write_source(
        "typed",
        "Library\n[]\n[ Rec.String ]\n[ Resolvable.[ resolve.{ [ Rec ]\n                         [ Self ] } ] ]\n[]\n",
    );
    let argument = check(&typed);
    let (success, output) = [argument.as_str()].invoke();
    assert!(success, "{output}");
    assert_eq!(output, format!("Checked.{{ [ {typed} ] [] }}\n"));
}

#[test]
fn a_file_headed_sema_is_rejected_naming_memory() {
    let directory = "sema".scratch();
    let source = directory.write_source("sema", "Sema\n[]\n[ Record.{ String } ]\n");
    let argument = check(&source);
    let (success, output) = [argument.as_str()].invoke();
    assert!(!success);
    assert_eq!(
        output,
        format!("Rejected.{{ {source} {{ 1 1 }} Conceptual.{{ [ 0 ] Renamed.Memory }} }}\n")
    );
}

#[test]
fn the_four_flow_nexus_files_check_trusting_flow_until_it_is_given() {
    let root = format!("{}/fixtures/print", env!("CARGO_MANIFEST_DIR"));
    let library = format!("{root}/flow-library.ethos");
    for name in [
        "flow-library",
        "flow-signal",
        "flow-operation",
        "flow-memory",
    ] {
        let source = format!("{root}/{name}.ethos");
        let argument = check(&source);
        let (success, output) = [argument.as_str()].invoke();
        assert!(success, "{output}");
        // Without the flow namespace, what the file imports from it is unchecked.
        let unchecked = if name == "flow-library" {
            "[]".to_owned()
        } else {
            format!("[ Unchecked.{{ {source} flow [ FlowId Voice Event ] }} ]")
        };
        assert_eq!(output, format!("Checked.{{ [ {source} ] {unchecked} }}\n"));
        // Given, the flow Library is the crate's root, and every import is checked there.
        let argument = format!(
            "Check.{{ [ {} ] [ {{ flow [ {} ] }} ] }}",
            opaque(&source),
            opaque(&library)
        );
        let (success, output) = [argument.as_str()].invoke();
        assert!(success, "{output}");
        assert_eq!(output, format!("Checked.{{ [ {source} ] [] }}\n"));
    }
}

#[test]
fn an_import_a_given_namespace_does_not_provide_is_rejected_at_its_line_and_column() {
    let directory = "unknown-name".scratch();
    let given = directory.write_source("protos", "Library\n[]\n[ Extent.String ]\n[]\n[]\n");
    let importing = directory.write_source(
        "importing",
        "Library\n[ :protos:[ Extent\n             Ghost ] ]\n[ Holder.{ :elsewhere:Thing } ]\n[]\n[]\n",
    );
    let checked = format!(
        "Check.{{ [ {} ] [ {{ protos [ {} ] }} ] }}",
        opaque(&importing),
        opaque(&given)
    );
    let expected = format!(
        "Rejected.{{ {importing} {{ 3 14 }} Conceptual.{{ [ 1 0 1 1 ] UnknownName.{{ protos Ghost }} }} }}\n"
    );
    assert_eq!([checked.as_str()].invoke(), (false, expected.clone()));
    let outputs = format!("{directory}/unwritten");
    let generate = format!(
        "Generate.{{ {{ [ {} ] [ {{ protos [ {} ] }} ] }} {} }}",
        opaque(&importing),
        opaque(&given),
        opaque(&outputs)
    );
    assert_eq!([generate.as_str()].invoke(), (false, expected));
    assert!(!std::path::Path::new(&outputs).exists());

    // What the namespace provides checks; what names no namespace given is listed unchecked.
    let resolved = directory.write_source(
        "resolved",
        "Library\n[ :protos:[ Extent ] ]\n[ Holder.{ Extent :elsewhere:sub:Thing } ]\n[]\n[]\n",
    );
    let argument = format!(
        "Check.{{ [ {} ] [ {{ protos [ {} ] }} ] }}",
        opaque(&resolved),
        opaque(&given)
    );
    let (success, output) = [argument.as_str()].invoke();
    assert!(success, "{output}");
    assert_eq!(
        output,
        format!(
            "Checked.{{ [ {resolved} ] [ Unchecked.{{ {resolved} elsewhere:sub [ Thing ] }} ] }}\n"
        )
    );

    // Generate lists the same unchecked imports Check does, beside what it wrote.
    let generated = format!("{directory}/generated");
    let argument = format!(
        "Generate.{{ {{ [ {} ] [ {{ protos [ {} ] }} ] }} {} }}",
        opaque(&resolved),
        opaque(&given),
        opaque(&generated)
    );
    let (success, output) = [argument.as_str()].invoke();
    assert!(success, "{output}");
    assert_eq!(
        output,
        format!(
            "Generated.{{ [ {generated}/resolved.rs ] [ Unchecked.{{ {resolved} elsewhere:sub [ Thing ] }} ] }}\n"
        )
    );
}

#[test]
fn a_namespace_file_that_does_not_read_is_rejected_naming_it() {
    let directory = "unreadable-namespace".scratch();
    let broken = directory.write_source("protos", "Library\n[]\n[ Record.{ String ]\n[]\n[]\n");
    let importing = directory.write_source("importing", "Library [ protos:[ Extent ] ] [] [] []");
    let argument = format!(
        "Check.{{ [ {} ] [ {{ protos [ {} ] }} ] }}",
        opaque(&importing),
        opaque(&broken)
    );
    let (success, output) = [argument.as_str()].invoke();
    assert!(!success);
    assert!(
        output.starts_with(&format!("Rejected.{{ {broken} {{ 3 19 }} Structural.")),
        "{output}"
    );
    let argument = format!(
        "Check.{{ [ {} ] [ {{ protos [ {} ] }} ] }}",
        opaque(&importing),
        opaque("/nowhere/protos.ethos")
    );
    let (success, output) = [argument.as_str()].invoke();
    assert!(!success);
    assert!(
        output.starts_with("Unreadable.{ /nowhere/protos.ethos "),
        "{output}"
    );
}

/// The namespaces `flow` and `harness` of one Signal each, and `lock` of an Organism, given with a query.
fn organism_namespaces(directory: &str) -> (String, String) {
    let flow = format!("{directory}/flow").write_source("signal", "Signal [] [] [] []");
    let harness = format!("{directory}/harness").write_source("signal", "Signal [] [] [] []");
    let lock = format!("{directory}/lock")
        .write_source("organism", "Organism\n[ :flow:signal ]\n[]\n[]\n[]\n");
    let namespaces = format!(
        "[ {{ flow [ {} ] }} {{ harness [ {} ] }} {{ lock [ {} ] }} ]",
        opaque(&flow),
        opaque(&harness),
        opaque(&lock)
    );
    (namespaces, lock)
}

#[test]
fn an_organism_checks_and_generates_its_minimal_module() {
    let directory = "organism-checks".scratch();
    let (namespaces, lock) = organism_namespaces(&directory);
    let organism = directory.write_source(
        "organism",
        "Organism\n[ :flow:signal ]\n[ :harness:signal ]\n[]\n[ :lock:organism ]\n",
    );
    let argument = format!("Check.{{ [ {} ] {namespaces} }}", opaque(&organism));
    assert_eq!(
        [argument.as_str()].invoke(),
        (true, format!("Checked.{{ [ {organism} ] [] }}\n"))
    );
    // A headed pair to one file checks.
    let headed = directory.write_source(
        "headed",
        "Organism\n[ Codex:harness:signal\n  Claude:harness:signal ]\n[]\n[]\n[]\n",
    );
    let argument = format!("Check.{{ [ {} ] {namespaces} }}", opaque(&headed));
    assert_eq!(
        [argument.as_str()].invoke(),
        (true, format!("Checked.{{ [ {headed} ] [] }}\n"))
    );
    let outputs = format!("{directory}/generated");
    let argument = format!(
        "Generate.{{ {{ [ {} ] {namespaces} }} {} }}",
        opaque(&lock),
        opaque(&outputs)
    );
    assert_eq!(
        [argument.as_str()].invoke(),
        (
            true,
            format!("Generated.{{ [ {outputs}/organism.rs ] [] }}\n")
        )
    );
    let written = std::fs::read_to_string(format!("{outputs}/organism.rs")).unwrap();
    assert!(written.contains("#![allow("));
    assert!(!written.contains("pub "));
}

#[test]
fn an_organism_is_rejected_at_the_line_and_column_of_what_is_amiss() {
    let directory = "organism-rejects".scratch();
    let (namespaces, _) = organism_namespaces(&directory);
    for (name, text, expected) in [
        (
            "empty",
            "Organism\n[]\n[ :harness:signal ]\n[]\n[]\n",
            "{ 2 1 } Conceptual.{ [ 1 0 ] Empty }",
        ),
        (
            "memories",
            "Organism\n[ :flow:signal ]\n[]\n[ :flow:signal :harness:signal ]\n[]\n",
            "{ 4 1 } Conceptual.{ [ 1 2 ] Arity.{ 1 2 } }",
        ),
        (
            "unpointed",
            "Organism\n[ :flow:signal ]\n[ :harness:signal\n  :harness:ghost ]\n[]\n[]\n",
            "{ 4 3 } Conceptual.{ [ 1 1 1 ] Unpointed.harness:ghost }",
        ),
        (
            "misrooted",
            "Organism\n[ :flow:signal ]\n[]\n[]\n[ :flow:signal ]\n",
            "{ 5 3 } Conceptual.{ [ 1 3 0 ] Misrooted.{ flow:signal Signal } }",
        ),
        (
            "duplicate",
            "Organism\n[ :harness:signal\n  harness:signal ]\n[]\n[]\n[]\n",
            "{ 3 3 } Conceptual.{ [ 1 0 1 ] Duplicate.harness:signal }",
        ),
    ] {
        let source = directory.write_source(name, text);
        let argument = format!("Check.{{ [ {} ] {namespaces} }}", opaque(&source));
        assert_eq!(
            [argument.as_str()].invoke(),
            (false, format!("Rejected.{{ {source} {expected} }}\n")),
            "{name}"
        );
    }
}

/// A memory pointing at a Matrix file checks.
#[test]
fn an_organism_points_its_memory_at_a_matrix_file() {
    let directory = "organism-matrix".scratch();
    let (namespaces, _) = organism_namespaces(&directory);
    let matrix = format!("{directory}/memory").write_source("matrix", "Matrix\n[]\n[]\n");
    let organism = directory.write_source(
        "organism",
        "Organism\n[ :flow:signal ]\n[]\n[ :memory:matrix ]\n[]\n",
    );
    let namespaces = format!(
        "{} {{ memory [ {} ] }} ]",
        namespaces.strip_suffix(" ]").unwrap(),
        opaque(&matrix)
    );
    let argument = format!("Check.{{ [ {} ] {namespaces} }}", opaque(&organism));
    assert_eq!(
        [argument.as_str()].invoke(),
        (true, format!("Checked.{{ [ {organism} ] [] }}\n"))
    );
}

#[test]
fn generate_writes_a_signal_with_an_open_enum_and_its_contract() {
    let directory = "open-enum".scratch();
    let source = directory.write_source(
        "open-enum",
        "Signal\n[]\n[ Launch.Model ]\n[ Launched ]\n[]\n[ Model ]\n",
    );
    let target = format!("{directory}/out");
    let argument = generate(&source, &target);
    let (success, output) = [argument.as_str()].invoke();
    assert!(success, "{output}");
    assert_eq!(
        output,
        format!("Generated.{{ [ {target}/open-enum.rs ] [] }}\n")
    );
    let rust = std::fs::read_to_string(format!("{target}/open-enum.rs")).expect("written");
    assert!(rust.contains("pub mod model {"));
    assert!(rust.contains("pub struct Model {"));
    let argument = check(&source);
    let (success, output) = [argument.as_str()].invoke();
    assert!(success, "{output}");
}

#[test]
fn check_and_generate_report_an_open_enum_no_input_or_output_carries() {
    let directory = "unused-open-enum".scratch();
    let unused = directory.write_source(
        "unused",
        "Signal [] [ Launch.String ] [ Launched ] [ TripleEffort.[ Low Medium High ] ] [ Model ]",
    );
    assert_eq!(
        [check(&unused).as_str()].invoke(),
        (
            true,
            format!("Checked.{{ [ {unused} ] [ Unused.{{ {unused} Model }} ] }}\n")
        )
    );
    let target = format!("{directory}/out");
    assert_eq!(
        [generate(&unused, &target).as_str()].invoke(),
        (
            true,
            format!("Generated.{{ [ {target}/unused.rs ] [ Unused.{{ {unused} Model }} ] }}\n")
        )
    );
    let used = directory.write_source(
        "used",
        "Signal [] [ Launch.Model ] [ Launched ] [ TripleEffort.[ Low Medium High ] ] [ Model ]",
    );
    assert_eq!(
        [check(&used).as_str()].invoke(),
        (true, format!("Checked.{{ [ {used} ] [] }}\n"))
    );
    // An unchecked import is a finding beside it, listed first.
    let both = directory.write_source(
        "both",
        "Signal [ serde_json:[ Value ] ] [ Launch.Value ] [ Launched ] [] [ Model ]",
    );
    assert_eq!(
        [check(&both).as_str()].invoke(),
        (
            true,
            format!(
                "Checked.{{ [ {both} ] [ Unchecked.{{ {both} serde_json [ Value ] }} Unused.{{ {both} Model }} ] }}\n"
            )
        )
    );
    // Generate lists the same findings, from the same list, beside what it wrote.
    assert_eq!(
        [generate(&both, &target).as_str()].invoke(),
        (
            true,
            format!(
                "Generated.{{ [ {target}/both.rs ] [ Unchecked.{{ {both} serde_json [ Value ] }} Unused.{{ {both} Model }} ] }}\n"
            )
        )
    );
}

#[test]
fn check_reports_no_open_enum_a_type_the_file_declares_carries() {
    let held = concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures/open-enum-held.ethos");
    assert_eq!(
        [check(held).as_str()].invoke(),
        (true, format!("Checked.{{ [ {held} ] [] }}\n"))
    );
}

#[test]
fn check_reads_the_qualified_name_of_the_files_own_open_enum_as_carrying_it() {
    let directory = "qualified-open-enum".scratch();
    let signal = format!("{directory}/flow").write_source(
        "signal",
        "Signal [] [ Launch.:flow:signal:Model ] [ Launched ] [] [ Model ]",
    );
    let argument = format!(
        "Check.{{ [ {} ] [ {{ flow [ {} ] }} ] }}",
        opaque(&signal),
        opaque(&signal)
    );
    assert_eq!(
        [argument.as_str()].invoke(),
        (true, format!("Checked.{{ [ {signal} ] [] }}\n"))
    );
    // Its namespace not given, the name is trusted, and nothing says it is the file's own.
    assert_eq!(
        [check(&signal).as_str()].invoke(),
        (
            true,
            format!(
                "Checked.{{ [ {signal} ] [ Unchecked.{{ {signal} flow:signal [ Model ] }} Unused.{{ {signal} Model }} ] }}\n"
            )
        )
    );
}

#[test]
fn check_lists_findings_file_by_file_in_the_order_given_each_in_the_order_of_its_text() {
    let directory = "findings-order".scratch();
    let first = directory.write_source(
        "first",
        "Signal [ serde_json:[ Value ] ] [ Launch.Value ] [ Launched ] [] [ Model ]",
    );
    let second = directory.write_source(
        "second",
        "Signal [ elsewhere:[ Thing ] ] [ Go.Thing ] [ Gone ] [] [ Spare ]",
    );
    assert_eq!(
        [check_all(&[&second, &first]).as_str()].invoke(),
        (
            true,
            format!(
                "Checked.{{ [ {second} {first} ] [ Unchecked.{{ {second} elsewhere [ Thing ] }} Unused.{{ {second} Spare }} Unchecked.{{ {first} serde_json [ Value ] }} Unused.{{ {first} Model }} ] }}\n"
            )
        )
    );
}

#[test]
fn a_signal_exports_carried_only_where_its_open_enums_carry_a_type() {
    let directory = "exported-carried".scratch();
    let importing = directory.write_source(
        "importing",
        "Library\n[ :flow:signal:[ Carried ] ]\n[]\n[]\n[]\n",
    );
    let bare =
        format!("{directory}/bare/flow").write_source("signal", "Signal [] [] [] [] [ Model ]");
    let carrying = format!("{directory}/carrying/flow")
        .write_source("signal", "Signal [] [] [] [ Mark.String ] [ Model ]");
    let checked = |signal: &str| {
        format!(
            "Check.{{ [ {} ] [ {{ flow [ {} ] }} ] }}",
            opaque(&importing),
            opaque(signal)
        )
    };
    assert_eq!(
        [checked(&bare).as_str()].invoke(),
        (
            false,
            format!(
                "Rejected.{{ {importing} {{ 2 18 }} Conceptual.{{ [ 1 0 1 0 ] UnknownName.{{ flow:signal Carried }} }} }}\n"
            )
        )
    );
    assert_eq!(
        [checked(&carrying).as_str()].invoke(),
        (true, format!("Checked.{{ [ {importing} ] [] }}\n"))
    );
}

#[test]
fn a_signal_exports_carrying_only_where_it_exports_carried() {
    let directory = "exported-carrying".scratch();
    let bare =
        format!("{directory}/bare/flow").write_source("signal", "Signal [] [] [] [] [ Model ]");
    let carrying = format!("{directory}/carrying/flow")
        .write_source("signal", "Signal [] [] [] [ Mark.String ] [ Model ]");
    for generated in ["Carried", "Carrying"] {
        let importing = directory.write_source(
            &generated.to_lowercase(),
            &format!("Library\n[ :flow:signal:[ {generated} ] ]\n[]\n[]\n[]\n"),
        );
        let checked = |signal: &str| {
            format!(
                "Check.{{ [ {} ] [ {{ flow [ {} ] }} ] }}",
                opaque(&importing),
                opaque(signal)
            )
        };
        assert_eq!(
            [checked(&bare).as_str()].invoke(),
            (
                false,
                format!(
                    "Rejected.{{ {importing} {{ 2 18 }} Conceptual.{{ [ 1 0 1 0 ] UnknownName.{{ flow:signal {generated} }} }} }}\n"
                )
            )
        );
        assert_eq!(
            [checked(&carrying).as_str()].invoke(),
            (true, format!("Checked.{{ [ {importing} ] [] }}\n"))
        );
    }
}

#[test]
fn check_resolves_an_import_from_an_open_enums_module_with_its_namespace_given() {
    let root = env!("CARGO_MANIFEST_DIR");
    let signal = format!("{root}/fixtures/nexus/models/signal.ethos");
    let matrix = format!("{root}/fixtures/nexus/models/matrix.ethos");
    let checked = |matrix: &str| {
        format!(
            "Check.{{ [ {} ] [ {{ models [ {} ] }} ] }}",
            opaque(matrix),
            opaque(&signal)
        )
    };
    // What the module generates checks, and no finding is left unchecked.
    assert_eq!(
        [checked(&matrix).as_str()].invoke(),
        (true, format!("Checked.{{ [ {matrix} ] [] }}\n"))
    );
    // A name it does not generate is refused at the name.
    let ghost = "open-enum-module".scratch().write_source(
        "matrix",
        "Matrix\n[ models:signal:model:[ Nonesuch ] ]\n[ Models.⟦Nonesuch⟧ ]\n",
    );
    assert_eq!(
        [checked(&ghost).as_str()].invoke(),
        (
            false,
            format!(
                "Rejected.{{ {ghost} {{ 2 25 }} Conceptual.{{ [ 1 0 0 1 1 1 0 ] UnknownName.{{ models:signal:model Nonesuch }} }} }}\n"
            )
        )
    );
}

/// The namespace `flow` of two Signals, `signal` and `meta`, each of one input
/// and one output, given with a query.
fn two_signal_namespaces(directory: &str) -> String {
    let signal = format!("{directory}/flow")
        .write_source("signal", "Signal [] [ Ask.String ] [ Told.String ] []");
    let meta = format!("{directory}/flow").write_source(
        "meta",
        "Signal [] [ Refresh.String ] [ Refreshed.String ] []",
    );
    format!("[ {{ flow [ {} {} ] }} ]", opaque(&signal), opaque(&meta))
}

#[test]
fn a_name_imported_from_two_namespaces_is_ambiguous_only_where_it_is_written_bare() {
    let directory = "ambiguous".scratch();
    let namespaces = two_signal_namespaces(&directory);
    let imports = "Library\n[ :flow:signal:[ Input Output ]\n  :flow:meta:[ Input Output ] ]\n";
    let unused = directory.write_source("unused", &format!("{imports}[]\n[]\n[]\n"));
    let argument = format!("Check.{{ [ {} ] {namespaces} }}", opaque(&unused));
    assert_eq!(
        [argument.as_str()].invoke(),
        (true, format!("Checked.{{ [ {unused} ] [] }}\n"))
    );
    let bare = directory.write_source(
        "bare",
        &format!("{imports}[ Holder.{{ Input }} ]\n[]\n[]\n"),
    );
    let argument = format!("Check.{{ [ {} ] {namespaces} }}", opaque(&bare));
    assert_eq!(
        [argument.as_str()].invoke(),
        (
            false,
            format!(
                "Rejected.{{ {bare} {{ 4 12 }} Conceptual.{{ [ 1 1 0 1 0 ] Ambiguous.Input }} }}\n"
            )
        )
    );
    let qualified = directory.write_source(
        "qualified",
        &format!("{imports}[ Holder.{{ :flow:signal:Input }} ]\n[]\n[]\n"),
    );
    let argument = format!("Check.{{ [ {} ] {namespaces} }}", opaque(&qualified));
    assert_eq!(
        [argument.as_str()].invoke(),
        (true, format!("Checked.{{ [ {qualified} ] [] }}\n"))
    );
    let outputs = format!("{directory}/generated");
    let generate = format!(
        "Generate.{{ {{ [ {} ] {namespaces} }} {} }}",
        opaque(&qualified),
        opaque(&outputs)
    );
    assert_eq!(
        [generate.as_str()].invoke(),
        (
            true,
            format!("Generated.{{ [ {outputs}/qualified.rs ] [] }}\n")
        )
    );
    let written = std::fs::read_to_string(format!("{outputs}/qualified.rs")).unwrap();
    assert!(
        written.contains("pub input: ::flow::signal::Input,"),
        "{written}"
    );
}

#[test]
fn an_emission_declaring_emission_or_outcome_is_rejected_where_it_declares_it() {
    let directory = "emission-collision".scratch();
    for name in ["Emission", "Outcome"] {
        let source = directory.write_source(
            name,
            &format!("Emission\n[]\n[ Go ]\n[ Gone ]\n[ {name}.String ]\n"),
        );
        assert_eq!(
            [check(&source).as_str()].invoke(),
            (
                false,
                format!(
                    "Rejected.{{ {source} {{ 5 3 }} Conceptual.{{ [ 1 3 0 0 ] Duplicate.{name} }} }}\n"
                )
            )
        );
    }
}

#[test]
fn a_file_headed_by_no_root_is_unrooted_and_an_entry_that_is_no_bare_name_expects_a_symbol() {
    let directory = "renamed-variants".scratch();
    let unrooted = directory.write_source("unrooted", "Nonsense\n[]\n[]\n[]\n[]\n");
    let argument = check(&unrooted);
    let (success, output) = [argument.as_str()].invoke();
    assert!(!success);
    assert_eq!(
        output,
        format!("Rejected.{{ {unrooted} {{ 1 1 }} Conceptual.{{ [ 0 ] Unrooted }} }}\n")
    );

    let unnamed = directory.write_source("unnamed", "Signal [] [] [] [] [ Model.String ]\n");
    let argument = check(&unnamed);
    let (success, output) = [argument.as_str()].invoke();
    assert!(!success);
    assert!(
        output.ends_with("Conceptual.{ [ 1 4 0 ] Expected.Symbol } }\n"),
        "{output}"
    );
}

#[test]
fn what_could_not_be_written_is_named_as_the_directory_or_the_file_it_is() {
    let directory = "unwritable".scratch();
    let source = directory.write_source("signal", "Signal [] [] [] []");
    // The directory written into is a file: it is the Directory that could not be made.
    let blocked = format!("{directory}/blocked");
    std::fs::write(&blocked, "").unwrap();
    let (succeeded, answer) = [generate(&source, &blocked).as_str()].invoke();
    assert!(!succeeded);
    assert!(
        answer.starts_with(&format!("Unwritable.{{ Directory.{blocked} «")),
        "{answer}"
    );
    // The module's file is a directory: it is the FilePath that could not be written.
    let out = format!("{directory}/out");
    std::fs::create_dir_all(format!("{out}/signal.rs")).unwrap();
    let (succeeded, answer) = [generate(&source, &out).as_str()].invoke();
    assert!(!succeeded);
    assert!(
        answer.starts_with(&format!("Unwritable.{{ FilePath.{out}/signal.rs «")),
        "{answer}"
    );
    std::fs::remove_dir_all(&directory).unwrap();
}
