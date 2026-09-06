//! Offline, one-time migration from the retired v1 Ethos-zero Nexus store.
//!
//! The converter opens the v1 source exclusively, reads every old archive row,
//! produces a separate v2 store and typed source manifest, then atomically
//! publishes each completed output.  It never starts the serving Nexus and it
//! never mutates the v1 files.

use std::{
    collections::BTreeMap,
    env, fs,
    path::{Path, PathBuf},
    process::ExitCode,
};

use legacy_datomic::{DatomicString, Text as LegacyText, TextEdge as _};
use legacy_meta_signal_ethos_zero as legacy_meta;
use legacy_signal_ethos_zero as legacy_ordinary;
use protos::{Conceivable as _, Text, Textualizable as _};
use sema_engine::{
    Engine, EngineOpen, EngineRecord, FamilyName, QueryPlan, RecordKey, SchemaHash, SchemaVersion,
    TableDescriptor, TableName,
};
use thiserror::Error;

#[rustfmt::skip]
#[path = "../ingress.rs"]
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

#[derive(Debug, Error)]
enum MigrationError {
    #[error("usage: ethos-zero-migrate-v1 <v1-store.sema> <new-v2-store.sema>")]
    Usage,
    #[error("the v1 source and v2 target must be distinct")]
    SamePath,
    #[error("the v2 target already exists: {0}")]
    TargetExists(PathBuf),
    #[error("the v2 source-manifest target already exists: {0}")]
    ManifestTargetExists(PathBuf),
    #[error("the v1 store has {0} configuration rows; expected exactly one")]
    ConfigurationInvariant(usize),
    #[error("v1 source manifest cannot be decoded as its retired map shape")]
    Manifest,
    #[error("current Protos Text rejected a preserved v1 field")]
    Text,
    #[error("filesystem: {0}")]
    Filesystem(#[from] std::io::Error),
    #[error("sema engine: {0}")]
    Engine(#[from] sema_engine::Error),
}

fn temporary(path: &Path) -> PathBuf {
    path.with_extension(format!("migration-{}.tmp", std::process::id()))
}
fn manifest_target(target: &Path) -> PathBuf {
    target.with_extension("sources.datom")
}
fn configuration_table() -> TableDescriptor<V1Configuration> {
    TableDescriptor::new(
        CONFIGURATION_TABLE,
        FamilyName::new("ethos-zero-nexus-configuration"),
        SchemaHash::for_label("ethos-zero-nexus-configuration-v1"),
    )
}
fn assemblies_table() -> TableDescriptor<V1Assembly> {
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
fn current_text(value: &str) -> Result<Text, MigrationError> {
    Text::try_from(value.to_owned()).map_err(|_| MigrationError::Text)
}
fn source_manifest(text: &str) -> Result<ingress::SourceManifest, MigrationError> {
    let old = LegacyText::<BTreeMap<DatomicString, DatomicString>>::from(text)
        .embody()
        .map_err(|_| MigrationError::Manifest)?;
    let entries = old
        .into_iter()
        .map(|(name, path)| {
            Ok(ingress::SourceEntry(
                current_text(name.as_ref())?,
                current_text(path.as_ref())?,
            ))
        })
        .collect::<Result<_, MigrationError>>()?;
    Ok(ingress::SourceManifest(entries))
}
fn canonical_manifest(manifest: &ingress::SourceManifest) -> String {
    manifest
        .conceive()
        .expect("generated ingress ascent is infallible")
        .1
        .textualize()
}
fn migrate(source: &Path, target: &Path) -> Result<(), MigrationError> {
    if source == target {
        return Err(MigrationError::SamePath);
    }
    if target.exists() {
        return Err(MigrationError::TargetExists(target.to_path_buf()));
    }
    let target_manifest = manifest_target(target);
    if target_manifest.exists() {
        return Err(MigrationError::ManifestTargetExists(target_manifest));
    }

    // All source reads and external-manifest validation complete while this
    // process owns the v1 database's native redb writer lock.
    let mut v1 = Engine::open(EngineOpen::new(source, SchemaVersion::new(1)))?;
    let configuration_table = v1.register_table(configuration_table())?;
    let assemblies_table = v1.register_table(assemblies_table())?;
    let configuration = match v1
        .match_records(QueryPlan::all(configuration_table))?
        .records()
    {
        [configuration] => configuration.clone(),
        rows => return Err(MigrationError::ConfigurationInvariant(rows.len())),
    };
    let assemblies = v1
        .match_records(QueryPlan::all(assemblies_table))?
        .records()
        .to_vec();
    let manifest_source = PathBuf::from(configuration.configuration.source_manifest_path.as_ref());
    let manifest = source_manifest(&fs::read_to_string(&manifest_source)?)?;
    let migrated_configuration = V2Configuration {
        ordinary_socket_path: configuration
            .configuration
            .ordinary_socket_path
            .as_ref()
            .to_owned(),
        meta_socket_path: configuration
            .configuration
            .meta_socket_path
            .as_ref()
            .to_owned(),
        source_manifest_path: target_manifest.display().to_string(),
    };
    let migrated_assemblies = assemblies
        .into_iter()
        .map(|assembly| V2Assembly {
            source_name: assembly.generation.file.source_name.as_ref().to_owned(),
            relative_path: assembly.generation.file.relative_path.as_ref().to_owned(),
            artifact_path: assembly.generation.artifact.as_ref().to_owned(),
        })
        .collect::<Vec<_>>();
    drop(v1);

    let parent = target
        .parent()
        .ok_or_else(|| std::io::Error::other("target has no parent"))?;
    fs::create_dir_all(parent)?;
    let temporary_target = temporary(target);
    let temporary_manifest = temporary(&target_manifest);
    if temporary_target.exists() || temporary_manifest.exists() {
        return Err(MigrationError::TargetExists(temporary_target));
    }
    let result = (|| -> Result<(), MigrationError> {
        fs::write(&temporary_manifest, canonical_manifest(&manifest))?;
        let mut v2 = Engine::open(EngineOpen::new(&temporary_target, SchemaVersion::new(2)))?;
        let configuration_table = v2.register_table(v2_configuration_table())?;
        let assemblies_table = v2.register_table(v2_assemblies_table())?;
        let mut commit = v2
            .begin_atomic_commit()
            .assert(configuration_table, migrated_configuration);
        for assembly in migrated_assemblies {
            commit = commit.assert(assemblies_table, assembly);
        }
        v2.commit_atomic(commit)?;
        drop(v2);
        fs::rename(&temporary_manifest, &target_manifest)?;
        fs::rename(&temporary_target, target)?;
        Ok(())
    })();
    if result.is_err() {
        let _ = fs::remove_file(&temporary_manifest);
        let _ = fs::remove_file(&temporary_target);
    }
    result
}
fn main() -> ExitCode {
    let arguments = env::args_os().skip(1).collect::<Vec<_>>();
    let [source, target] = arguments.as_slice() else {
        eprintln!("{}", MigrationError::Usage);
        return ExitCode::FAILURE;
    };
    match migrate(Path::new(source), Path::new(target)) {
        Ok(()) => {
            println!("Migrated.{{ {} {} }}", source.display(), target.display());
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
