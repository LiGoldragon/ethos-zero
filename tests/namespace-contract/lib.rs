//! Owner and emission contracts import a real dependency named `signal`.
#[path = "../generated/namespace-emission.rs"]
pub mod namespace_emission;
#[path = "../generated/namespace-owner.rs"]
pub mod namespace_owner;

#[cfg(test)]
mod wire {
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
    fn independent_namespaces_and_external_contracts_cross_the_wire() {
        use super::{namespace_emission, namespace_owner};
        use signal::{library, namespace_ordinary};
        let authored = namespace_ordinary::Input {
            string: "authored".to_owned(),
        };
        let input = namespace_ordinary::signal::Input::Put(namespace_ordinary::signal::Put_Data {
            first_input: authored.clone(),
            output: namespace_ordinary::Output(5),
            voice: library::Voice::Psyche(library::Rank::Primary),
            inline: namespace_ordinary::Inline("inline".to_owned()),
            second_input: authored.clone(),
        });
        crosses(input.clone());
        crosses(namespace_owner::signal::Input::Forward(input.clone()));
        crosses(namespace_owner::signal::Input::Inspect(
            namespace_owner::signal::Inspect_Data {
                first_input: authored,
                second_input: input.clone(),
            },
        ));
        crosses(namespace_emission::signal::Input::Send(input));
        crosses(namespace_ordinary::signal::Output::Stored(
            namespace_ordinary::Output(7),
        ));
        crosses(namespace_owner::signal::Output::Forwarded(
            namespace_ordinary::signal::Output::Empty,
        ));
        crosses(namespace_emission::signal::Output::Sent(
            namespace_ordinary::signal::Output::Empty,
        ));
    }

    #[cfg(feature = "datom")]
    #[test]
    fn imported_signal_values_keep_their_datom_behavior() {
        use super::{namespace_emission, namespace_owner};
        use datom_codec::{Actualizing, Budget, Potential};
        use signal::{library, namespace_ordinary};
        let mut budget = Budget {
            remaining: 4096,
            reader: 4096.try_into().expect("reader budget"),
            depth: 0,
            maximum_depth: 4096,
        };
        let request = Potential::<namespace_owner::signal::Input>::from("Forward.Inspect")
            .actualize(&mut budget)
            .expect("owner input");
        assert_eq!(
            request,
            namespace_owner::signal::Input::Forward(namespace_ordinary::signal::Input::Inspect)
        );
        let emission = Potential::<namespace_emission::signal::Input>::from("Send.Inspect")
            .actualize(&mut budget)
            .expect("emission input");
        assert_eq!(
            emission,
            namespace_emission::signal::Input::Send(namespace_ordinary::signal::Input::Inspect)
        );
    }

    #[test]
    fn generated_self_and_authored_input_recur_independently() {
        use signal::namespace_recursive::{
            Input,
            signal::{Input as Request, Output},
        };
        crosses(Request::Recur(Box::new(Request::Leaf)));
        crosses(Request::Parent(Input {
            input_option: Some(Box::new(Input { input_option: None })),
        }));
        crosses(Output::Again(Some(Box::new(Output::Done))));
    }

    #[cfg(feature = "datom")]
    #[test]
    fn recursive_generated_enums_keep_their_datom_behavior() {
        use datom_codec::{Actualizing, Budget, Datomizable, Potential};
        use signal::namespace_recursive::signal::{Input, Output};
        let mut budget = Budget {
            remaining: 4096,
            reader: 4096.try_into().unwrap(),
            depth: 0,
            maximum_depth: 4096,
        };
        let value = Potential::<Input>::from("Recur.Leaf")
            .actualize(&mut budget)
            .expect("recursive generated input");
        assert_eq!(value, Input::Recur(Box::new(Input::Leaf)));
        let value = Potential::<Output>::from("Again.Some.Done")
            .actualize(&mut budget)
            .expect("recursive generated output");
        assert_eq!(value, Output::Again(Some(Box::new(Output::Done))));
        let _ = value.datomize(datom_codec::Path::root());
    }
}
