//! The command accepts one generated `Query` and writes its generated module.

use std::process::Command;

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
    format!("Generate.{{ {} {} }}", opaque(source), opaque(directory))
}

#[test]
fn no_argument_prints_the_crates_own_ethos_ending_with_a_newline() {
    let (success, output) = [].invoke();
    assert!(success);
    assert_eq!(output, include_str!("../ethos-zero.ethos"));
    assert!(output.ends_with('\n'));
    assert!(output.contains("Generate.Generation"));
    assert!(output.contains("Check.String"));
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
        format!("Generated.[ {directory}/record-types.rs ]\n")
    );
    let written = std::fs::read_to_string(format!("{directory}/record-types.rs")).unwrap();
    assert!(written.contains("pub struct Record"));
    assert!(written.contains("pub string: String"));
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
        format!("Generated.[ {directory}/multi-types.rs ]\n")
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
fn a_missing_file_is_unreadable() {
    let argument = generate("/nowhere/missing.ethos", "/nowhere");
    let (success, output) = [argument.as_str()].invoke();
    assert!(!success);
    assert!(
        output.starts_with("Unreadable.{ /nowhere/missing.ethos "),
        "{output}"
    );
}

/// The kind whose capability writes an ethos source into a scratch directory.
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

fn check(source: &str) -> String {
    format!("Check.{}", opaque(source))
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
    assert_eq!(output, format!("Checked.{source}\n"));
}

#[test]
fn check_locates_an_undeclared_name_below_a_comment() {
    let directory = "undeclared".scratch();
    let source = directory.write_source(
        "undeclared",
        "; The comment and the sweet form's seam are counted too.\nLibrary\n[]\n[ Record.{ String\n           Vector<Option<Bogus>> } ]\n[]\n[]\n",
    );
    let argument = check(&source);
    let (success, output) = [argument.as_str()].invoke();
    assert!(!success);
    assert_eq!(
        output,
        format!(
            "Rejected.{{ {source} {{ 5 26 }} Conceptual.{{ [ 1 1 0 1 2 1 0 ] Undeclared.Bogus }} }}\n"
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
