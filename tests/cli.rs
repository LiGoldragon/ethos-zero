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
    assert!(output.contains("[ Generate.Generation ]"));
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

#[test]
fn a_bad_ethos_file_returns_a_typed_generation_error() {
    let directory = "faulty".scratch();
    std::fs::create_dir_all(&directory).unwrap();
    let source = format!("{directory}/faulty.ethos");
    std::fs::write(&source, "Library [] [ Record.{ String Bogus } ] [] []").unwrap();
    let argument = generate(&source, &directory);
    let (success, output) = [argument.as_str()].invoke();
    assert!(!success);
    assert!(output.starts_with("GenerationRejected.{"), "{output}");
    assert!(output.contains("Conceptual.{"), "{output}");
    assert!(output.contains("Undeclared.Bogus"), "{output}");
}
