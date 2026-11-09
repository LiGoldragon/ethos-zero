//! The Flow Nexus's whole contract, its Library, Signal, Operation and
//! Memory modules as the built tool generates them from fixtures/print,
//! compiled as a crate of its own: once with no `datom-codec` at all, as a
//! Nexus builds it, and once with its `datom` feature, as a CLI builds it.
//! In both, a value of each root crosses the wire: it archives with rkyv and
//! restores equal.

extern crate self as flow;

pub use library::{Event, FlowId, Voice};

#[path = "../generated/flow-library.rs"]
pub mod library;
#[path = "../generated/flow-memory.rs"]
pub mod memory;
#[path = "../generated/flow-operation.rs"]
pub mod operation;
#[path = "../generated/flow-signal.rs"]
pub mod signal;

#[cfg(test)]
mod wire {
    use super::{library, memory, operation, signal};

    fn crosses<T>(value: T)
    where
        T: rkyv::Archive
            + for<'a> rkyv::Serialize<
                rkyv::api::high::HighSerializer<
                    rkyv::util::AlignedVec,
                    rkyv::ser::allocator::ArenaHandle<'a>,
                    rkyv::rancor::Error,
                >,
            > + PartialEq
            + std::fmt::Debug,
        T::Archived: for<'a> rkyv::bytecheck::CheckBytes<
                rkyv::api::high::HighValidator<'a, rkyv::rancor::Error>,
            > + rkyv::Deserialize<T, rkyv::api::high::HighDeserializer<rkyv::rancor::Error>>,
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
                login: operation::Login(vec!["claude".to_owned()]),
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
