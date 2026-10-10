//! The Flow Nexus's whole contract (its Library, Signal, Operation and
//! Memory modules together) compiles in a separate Cargo package, once with
//! no `datom-codec` at all, as a Nexus builds it, and once with its `datom`
//! feature, as a CLI builds it. In both, a value of each root crosses the
//! wire: it archives with rkyv and restores equal.

use std::process::Command;

/// The scratch package's own test: one value of every root through rkyv.
const ROUND_TRIP: &str = r#"
#[cfg(test)]
mod wire {
    use super::{library, memory, operation, signal};

    fn crosses<T>(value: T)
    where
        T: rkyv::Archive + for<'a> rkyv::Serialize<rkyv::api::high::HighSerializer<rkyv::util::AlignedVec, rkyv::ser::allocator::ArenaHandle<'a>, rkyv::rancor::Error>> + PartialEq + std::fmt::Debug,
        T::Archived: for<'a> rkyv::bytecheck::CheckBytes<rkyv::api::high::HighValidator<'a, rkyv::rancor::Error>> + rkyv::Deserialize<T, rkyv::api::high::HighDeserializer<rkyv::rancor::Error>>,
    {
        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&value).expect("archives");
        let restored = rkyv::from_bytes::<T, rkyv::rancor::Error>(&bytes).expect("restores");
        assert_eq!(restored, value);
    }

    #[test]
    fn every_root_crosses_the_wire() {
        crosses(library::Voice::Mind(library::Rank::Primary));
        crosses(signal::Query::Report(signal::Report_Data {
            flow_id: library::FlowId(7),
            event: library::Event::ToolUsed("Bash".to_owned()),
        }));
        crosses(signal::Response::Refused(signal::Refused_Data::VoiceBusy(
            library::Voice::Field(library::Rank::Tertiary),
        )));
        crosses(operation::Operation::Start(operation::Start_Data {
            voice: library::Voice::Psyche(library::Rank::Secondary),
            capsule: operation::Capsule {
                home: operation::Home("/home/flow".to_owned()),
                login: vec!["claude".to_owned()],
            },
        }));
        crosses(operation::Outcome::Failed("no capsule".to_owned()));
        crosses(memory::Flow {
            flow_id: library::FlowId(7),
            voice: library::Voice::Mind(library::Rank::Primary),
            state: memory::State::Ended,
            event_vector: vec![library::Event::Started, library::Event::Stopped],
        });
    }

    #[cfg(feature = "datom")]
    #[test]
    fn every_root_bears_its_datom_kinds_under_the_feature() {
        fn bears<T: datom_codec::Datomizable + datom_codec::Composing>() {}
        bears::<library::Voice>();
        bears::<signal::Query>();
        bears::<operation::Operation>();
        bears::<memory::Flow>();
    }
}
"#;

#[test]
fn the_flow_nexus_contract_compiles_and_crosses_the_wire_with_and_without_datom() {
    let directory = format!(
        "{}/flow-contract-{}",
        env!("CARGO_TARGET_TMPDIR"),
        std::process::id()
    );
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(format!("{directory}/src")).expect("temporary source directory");

    // The scratch package pins datom-codec exactly as this one does, so an
    // offline build resolves it from the same vendored source.
    let manifest = include_str!("../Cargo.toml");
    let datom_codec = manifest
        .lines()
        .find(|line| line.starts_with("datom-codec"))
        .expect("this package depends on datom-codec")
        .replacen("{ ", "{ optional = true, ", 1);
    std::fs::write(
        format!("{directory}/Cargo.toml"),
        format!(
            "[package]\nname = \"flow-contract\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[workspace]\n[features]\ndatom = [\"dep:datom-codec\"]\n[dependencies]\nrkyv = {{ version = \"0.8\", default-features = false, features = [\"std\", \"bytecheck\", \"little_endian\", \"pointer_width_32\", \"unaligned\"] }}\n{datom_codec}\n"
        ),
    )
    .expect("temporary manifest");
    // A git source resolves against vendored sources (the Nix sandbox) only
    // from a lock file, so the scratch package starts from this one's; cargo
    // prunes it to what the scratch package needs.
    std::fs::write(
        format!("{directory}/Cargo.lock"),
        include_str!("../Cargo.lock"),
    )
    .expect("temporary lock file");
    for (stem, source) in [
        ("library", include_str!("generated/flow-library.rs")),
        ("signal", include_str!("generated/flow-signal.rs")),
        ("operation", include_str!("generated/flow-operation.rs")),
        ("memory", include_str!("generated/flow-memory.rs")),
    ] {
        std::fs::write(format!("{directory}/src/{stem}.rs"), source).expect("generated source");
    }
    std::fs::write(
        format!("{directory}/src/lib.rs"),
        format!(
            "extern crate self as flow;\npub use library::{{Event, FlowId, Voice}};\npub mod library;\npub mod signal;\npub mod operation;\npub mod memory;\n{ROUND_TRIP}"
        ),
    )
    .expect("scratch crate root");

    for features in [&[][..], &["--features", "datom"][..]] {
        let status = Command::new("cargo")
            .args([
                "test",
                "--manifest-path",
                &format!("{directory}/Cargo.toml"),
            ])
            .args(features)
            .status()
            .expect("cargo is available");
        assert!(
            status.success(),
            "the Flow Nexus contract must compile and cross the wire ({features:?})"
        );
    }
}
