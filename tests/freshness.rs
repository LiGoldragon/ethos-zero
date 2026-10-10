//! Freshness: every committed generated module equals a fresh generation
//! by the library. The text is the product here, so the text is what is
//! asserted: src/error.rs from error.ethos, src/ethos-zero.rs from
//! ethos-zero.ethos, and tests/generated/<stem>.rs from every fixture,
//! the four Flow Nexus files in fixtures/print included.

use ethos_zero::{Actualizing, File, Generating, Potential};

/// The trait whose capability asserts a committed generation is fresh.
trait Fresh {
    fn fresh(&self, generated: &str);
}

impl Fresh for str {
    fn fresh(&self, generated: &str) {
        let root = env!("CARGO_MANIFEST_DIR");
        let source = std::fs::read_to_string(format!("{root}/{self}")).expect(self);
        let committed = std::fs::read_to_string(format!("{root}/{generated}")).expect(generated);
        let file = match Potential::<File>::from(source).actualize() {
            Ok(file) => file,
            Err(_) => panic!("{self} does not read"),
        };
        assert_eq!(
            match file.generate() {
                Ok(rust) => rust,
                Err(_) => panic!("checked source generates"),
            },
            committed,
            "{generated} is stale; regenerate from {self}"
        );
    }
}

#[test]
fn the_error_module_is_fresh() {
    "error.ethos".fresh("src/error.rs");
}

#[test]
fn the_contract_module_is_fresh() {
    "ethos-zero.ethos".fresh("src/ethos-zero.rs");
}

#[test]
fn every_fixture_module_is_fresh() {
    let root = env!("CARGO_MANIFEST_DIR");
    let mut fixtures: Vec<String> = Vec::new();
    for entry in std::fs::read_dir(format!("{root}/fixtures")).unwrap() {
        let name = entry.unwrap().file_name().to_string_lossy().into_owned();
        if let Some(stem) = name.strip_suffix(".ethos") {
            fixtures.push(stem.to_owned());
        }
    }
    fixtures.sort();
    assert_eq!(fixtures.len(), 18);
    for stem in fixtures {
        format!("fixtures/{stem}.ethos").fresh(&format!("tests/generated/{stem}.rs"));
    }
}

#[test]
fn the_flow_nexus_modules_are_fresh() {
    for stem in [
        "flow-library",
        "flow-signal",
        "flow-operation",
        "flow-memory",
    ] {
        format!("fixtures/print/{stem}.ethos").fresh(&format!("tests/generated/{stem}.rs"));
    }
}
