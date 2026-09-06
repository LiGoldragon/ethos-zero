use std::{fs, path::Path, process::Command};

use legacy_meta_signal_ethos_zero as legacy_meta;
use legacy_signal_ethos_zero as legacy_ordinary;
use protos::Actualizable as _;
use sema_engine::{
    Engine, EngineOpen, EngineRecord, FamilyName, QueryPlan, RecordKey, SchemaHash, SchemaVersion,
    TableDescriptor, TableName,
};
use tempfile::TempDir;

#[rustfmt::skip]
#[path = "../src/ingress.rs"]
mod ingress;

const CONFIGURATION_TABLE: TableName = TableName::new("ethos_zero_nexus_configuration");
const ASSEMBLIES_TABLE: TableName = TableName::new("ethos_zero_nexus_assemblies");
const CONFIGURATION_KEY: &str = "configuration";

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone)]
struct V1Configuration {
    configuration: legacy_meta::Configuration,
}
impl EngineRecord for V1Configuration {
    fn record_key(&self) -> RecordKey {
        RecordKey::new(CONFIGURATION_KEY)
    }
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone)]
struct V1Assembly {
    generation: legacy_ordinary::Generation,
}
impl EngineRecord for V1Assembly {
    fn record_key(&self) -> RecordKey {
        RecordKey::new(format!(
            "{}:{}",
            self.generation.file.source_name.as_ref(),
            self.generation.file.relative_path.as_ref()
        ))
    }
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone)]
struct V2Configuration {
    ordinary_socket_path: String,
    meta_socket_path: String,
    source_manifest_path: String,
}
impl EngineRecord for V2Configuration {
    fn record_key(&self) -> RecordKey {
        RecordKey::new(CONFIGURATION_KEY)
    }
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone)]
struct V2Assembly {
    source_name: String,
    relative_path: String,
    artifact_path: String,
}
impl EngineRecord for V2Assembly {
    fn record_key(&self) -> RecordKey {
        RecordKey::new(format!("{}:{}", self.source_name, self.relative_path))
    }
}

fn v1_configuration_table() -> TableDescriptor<V1Configuration> {
    TableDescriptor::new(
        CONFIGURATION_TABLE,
        FamilyName::new("ethos-zero-nexus-configuration"),
        SchemaHash::for_label("ethos-zero-nexus-configuration-v1"),
    )
}
fn v1_assemblies_table() -> TableDescriptor<V1Assembly> {
    TableDescriptor::new(
        ASSEMBLIES_TABLE,
        FamilyName::new("ethos-zero-nexus-assembly-state"),
        SchemaHash::for_label("ethos-zero-nexus-assembly-state-v1"),
    )
}
fn v2_configuration_table() -> TableDescriptor<V2Configuration> {
    TableDescriptor::new(
        CONFIGURATION_TABLE,
        FamilyName::new("ethos-zero-nexus-configuration"),
        SchemaHash::for_label("ethos-zero-nexus-configuration-v2"),
    )
}
fn v2_assemblies_table() -> TableDescriptor<V2Assembly> {
    TableDescriptor::new(
        ASSEMBLIES_TABLE,
        FamilyName::new("ethos-zero-nexus-assembly-state"),
        SchemaHash::for_label("ethos-zero-nexus-assembly-state-v2"),
    )
}

fn seed_v1(directory: &TempDir) -> (std::path::PathBuf, std::path::PathBuf) {
    let source = directory.path().join("v1.sema");
    let manifest = directory.path().join("v1-sources.datom");
    fs::write(&manifest, "« fieldlab /srv/fieldlab »").expect("old manifest writes");
    let configuration = legacy_meta::Configuration {
        ordinary_socket_path: "/run/ordinary.sock".try_into().expect("old ordinary path"),
        meta_socket_path: "/run/meta.sock".try_into().expect("old meta path"),
        source_manifest_path: manifest
            .display()
            .to_string()
            .try_into()
            .expect("old manifest path"),
    };
    let generation = legacy_ordinary::Generation {
        file: legacy_ordinary::FileLocation {
            source_name: "fieldlab".try_into().expect("old source name"),
            relative_path: "schema.ethos".try_into().expect("old relative path"),
        },
        artifact: "schema.rs".try_into().expect("old artifact path"),
    };
    let mut engine =
        Engine::open(EngineOpen::new(&source, SchemaVersion::new(1))).expect("v1 store opens");
    let configuration_table = engine
        .register_table(v1_configuration_table())
        .expect("v1 configuration table");
    let assemblies_table = engine
        .register_table(v1_assemblies_table())
        .expect("v1 assemblies table");
    engine
        .commit_atomic(
            engine
                .begin_atomic_commit()
                .assert(configuration_table, V1Configuration { configuration })
                .assert(assemblies_table, V1Assembly { generation }),
        )
        .expect("v1 source records commit atomically");
    drop(engine);
    (source, manifest)
}
fn migrate(source: &Path, target: &Path) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_ethos-zero-migrate-v1"))
        .arg(source)
        .arg(target)
        .output()
        .expect("migration invokes")
}

#[test]
fn offline_v1_to_v2_migration_preserves_configuration_assemblies_and_manifest() {
    let directory = tempfile::tempdir().expect("temporary fixture");
    let (source, manifest) = seed_v1(&directory);
    let target = directory.path().join("v2.sema");
    let output = migrate(&source, &target);
    assert!(
        output.status.success(),
        "migration failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        fs::read_to_string(&manifest).expect("old manifest remains"),
        "« fieldlab /srv/fieldlab »"
    );

    let mut old_engine =
        Engine::open(EngineOpen::new(&source, SchemaVersion::new(1))).expect("v1 source reopens");
    let old_configuration_table = old_engine
        .register_table(v1_configuration_table())
        .expect("v1 configuration table remains");
    let old_assemblies_table = old_engine
        .register_table(v1_assemblies_table())
        .expect("v1 assemblies table remains");
    assert_eq!(
        old_engine
            .match_records(QueryPlan::all(old_configuration_table))
            .expect("v1 configuration remains queryable")
            .records()
            .len(),
        1
    );
    assert_eq!(
        old_engine
            .match_records(QueryPlan::all(old_assemblies_table))
            .expect("v1 assemblies remain queryable")
            .records()
            .len(),
        1
    );
    drop(old_engine);

    let manifest_target = target.with_extension("sources.datom");
    let typed_manifest = fs::read_to_string(&manifest_target).expect("typed manifest emitted");
    let parsed = datom_codec::Potential::<ingress::SourceManifest>::from(typed_manifest.as_str())
        .actualize(datom_codec::IncorporationBudget::try_from(4096).expect("positive budget"))
        .expect("typed manifest reads");
    assert_eq!(parsed.0.len(), 1);
    assert_eq!(parsed.0[0].0.as_ref(), "fieldlab");
    assert_eq!(parsed.0[0].1.as_ref(), "/srv/fieldlab");

    let mut engine =
        Engine::open(EngineOpen::new(&target, SchemaVersion::new(2))).expect("v2 store opens");
    let configuration_table = engine
        .register_table(v2_configuration_table())
        .expect("v2 configuration table");
    let assemblies_table = engine
        .register_table(v2_assemblies_table())
        .expect("v2 assembly table");
    let configurations = engine
        .match_records(QueryPlan::all(configuration_table))
        .expect("v2 configuration query")
        .records()
        .to_vec();
    assert_eq!(configurations.len(), 1);
    assert_eq!(configurations[0].ordinary_socket_path, "/run/ordinary.sock");
    assert_eq!(configurations[0].meta_socket_path, "/run/meta.sock");
    assert_eq!(
        configurations[0].source_manifest_path,
        manifest_target.display().to_string()
    );
    let assemblies = engine
        .match_records(QueryPlan::all(assemblies_table))
        .expect("v2 assembly query")
        .records()
        .to_vec();
    assert_eq!(assemblies.len(), 1);
    assert_eq!(assemblies[0].source_name, "fieldlab");
    assert_eq!(assemblies[0].relative_path, "schema.ethos");
    assert_eq!(assemblies[0].artifact_path, "schema.rs");
}

#[test]
fn migration_refuses_an_active_v1_store_and_leaves_no_target() {
    let directory = tempfile::tempdir().expect("temporary fixture");
    let (source, _) = seed_v1(&directory);
    let target = directory.path().join("v2.sema");
    let held = Engine::open(EngineOpen::new(&source, SchemaVersion::new(1)))
        .expect("test holds native database lock");
    let output = migrate(&source, &target);
    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("already open"),
        "expected native redb lock refusal: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!target.exists());
    assert!(!target.with_extension("sources.datom").exists());
    drop(held);
    assert!(
        migrate(&source, &target).status.success(),
        "a stopped store becomes migratable"
    );
}

#[test]
fn migration_refuses_existing_destination_without_changing_it() {
    let directory = tempfile::tempdir().expect("temporary fixture");
    let (source, _) = seed_v1(&directory);
    let target = directory.path().join("v2.sema");
    fs::write(&target, b"keep-me").expect("destination sentinel writes");
    let output = migrate(&source, &target);
    assert!(!output.status.success());
    assert_eq!(fs::read(&target).expect("sentinel remains"), b"keep-me");
}
