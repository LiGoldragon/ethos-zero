//! Every generated fixture compiles with Datom enabled.

// The Flow Nexus files import `flow:[ FlowId Voice Event ]`; here the crate
// itself is `flow`, re-exporting the generated Flow Library.
extern crate self as flow;
pub use flow_library::{Event, FlowId, Voice};

pub struct Wrapper<T>(pub T);

// The stand-in for the signal library, which is not yet released: a
// generated Signal writes `signal::Frame` and `signal::Stream`, and here the
// crate itself is `signal`, holding the two at its root.
extern crate self as signal;

/// The stand-in for the signal library's `Frame`: one frame of a stream,
/// archived as every Output is, so each frame on the wire is one Output.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Composing)
)]
pub enum Frame<State, Change> {
    Opened(State),
    Changed(Change),
    Lagged(State),
    Ended(Ending),
}

/// The stand-in for how the signal library's stream ends.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Composing)
)]
pub enum Ending {
    Closed,
    Refused,
}

/// The stand-in for the signal library's live `Stream` of frames, never
/// archived: here the frames it will yield, in order.
pub struct Stream<State, Change>(pub std::vec::IntoIter<Frame<State, Change>>);

impl<State, Change> Iterator for Stream<State, Change> {
    type Item = Frame<State, Change>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

/// What the trait fixtures import from `super`: the error a capability
/// yields, and the traits a type is asserted to bear.
pub enum SinkError {
    Closed,
    Full,
}
pub trait Summarizable {
    fn summarize(&self) -> String;
}
pub trait Fillable {}
pub trait Serializable {}

impl Summarizable for sink_associations::Sink {
    fn summarize(&self) -> String {
        self.string.clone()
    }
}
impl Fillable for sink_associations::Sink {}

#[path = "generated/alias-format.rs"]
mod alias_format;
#[path = "generated/answer-input.rs"]
mod answer_input;
#[path = "generated/bare-emission.rs"]
mod bare_emission;
#[path = "generated/capability-traits.rs"]
mod capability_traits;
#[path = "generated/command-emission.rs"]
mod command_emission;
#[path = "generated/composition-types.rs"]
mod composition_types;
#[path = "generated/empty-signal.rs"]
mod empty_signal;
#[path = "generated/entry-memory.rs"]
mod entry_memory;
#[path = "generated/flow-library.rs"]
mod flow_library;
#[path = "generated/flow-memory.rs"]
mod flow_memory;
#[path = "generated/flow-operation.rs"]
mod flow_operation;
#[path = "generated/flow-signal.rs"]
mod flow_signal;
#[path = "generated/generic-shadow.rs"]
mod generic_shadow;
#[path = "generated/inline-collision.rs"]
mod inline_collision;
#[path = "generated/launch-newtypes.rs"]
mod launch_newtypes;
#[path = "generated/lock-signal.rs"]
mod lock_signal;
#[path = "generated/matrix-records.rs"]
mod matrix_records;
#[path = "generated/multi-types.rs"]
mod multi_types;
#[path = "generated/nested-collision.rs"]
mod nested_collision;
#[path = "generated/open-enum-bare.rs"]
mod open_enum_bare;
#[path = "generated/open-enum-held.rs"]
mod open_enum_held;
#[path = "generated/open-enum-recursive.rs"]
mod open_enum_recursive;
#[path = "generated/open-enum-shadow.rs"]
mod open_enum_shadow;
#[path = "generated/open-enum-signal.rs"]
mod open_enum_signal;
#[path = "generated/orchestrate.rs"]
mod orchestrate;
#[path = "generated/placed-types.rs"]
mod placed_types;
#[path = "generated/processable-traits.rs"]
mod processable_traits;
#[path = "generated/publishing-trait.rs"]
mod publishing_trait;
#[path = "generated/record-types.rs"]
mod record_types;
#[path = "generated/scalar-newtypes.rs"]
mod scalar_newtypes;
#[path = "generated/self-traits.rs"]
mod self_traits;
#[path = "generated/signal-decimal.rs"]
mod signal_decimal;
#[path = "generated/sink-associations.rs"]
mod sink_associations;
#[path = "generated/stream-signal.rs"]
mod stream_signal;
#[path = "generated/streamable-trait.rs"]
mod streamable_trait;
#[path = "generated/tree-types.rs"]
mod tree_types;

// The qualified Nexus names its own open enum by its namespace,
// `:qualified:contract:Model`: here the crate itself is `qualified`, holding
// the module `contract` its Nexus generated.
extern crate self as qualified;
#[path = "generated/nexus/qualified/contract.rs"]
mod contract;

#[test]
fn nested_inline_payloads_have_distinct_rust_types() {
    let left = nested_collision::Outer::A(nested_collision::A_Data::X(
        nested_collision::A_Data_X_Data {
            string: String::new(),
        },
    ));
    let right = nested_collision::Outer::B(nested_collision::B_Data::X(
        nested_collision::B_Data_X_Data { integer: 1 },
    ));
    assert!(matches!(left, nested_collision::Outer::A(_)));
    assert!(matches!(right, nested_collision::Outer::B(_)));
}

#[test]
fn colliding_inline_payloads_are_named_for_their_enums() {
    let p = inline_collision::P::X(inline_collision::P_X_Data {
        string: String::new(),
    });
    let nested = inline_collision::P::Y(inline_collision::Y_Data::X(
        inline_collision::Y_Data_X_Data { integer: 2 },
    ));
    let q = inline_collision::Q::X(inline_collision::Q_X_Data { integer: 1 });
    let authored = inline_collision::X_Data(String::new());
    let unique = inline_collision::R::Z(inline_collision::Z_Data { string: authored.0 });
    assert!(matches!(p, inline_collision::P::X(_)));
    assert!(matches!(nested, inline_collision::P::Y(_)));
    assert!(matches!(q, inline_collision::Q::X(_)));
    assert!(matches!(unique, inline_collision::R::Z(_)));
}

#[test]
fn named_fields_and_signal_input_output_compile() {
    let record = record_types::Record {
        string: "Ada".to_owned(),
        integer: 1990,
    };
    assert_eq!(record.integer, 1990);
    let input = orchestrate::Input::Observe(orchestrate::ObserveSelection::Locks(
        orchestrate::Locks(vec![]),
    ));
    let output =
        orchestrate::Output::Observed(orchestrate::Observation::Locks(orchestrate::Locks(vec![])));
    assert!(matches!(input, orchestrate::Input::Observe(_)));
    assert!(matches!(output, orchestrate::Output::Observed(_)));
}

#[test]
fn generated_signal_input_round_trips_as_a_portable_archive() {
    let input = orchestrate::Input::Observe(orchestrate::ObserveSelection::Locks(
        orchestrate::Locks(vec![]),
    ));
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&input).expect("archive input");
    let restored =
        rkyv::from_bytes::<orchestrate::Input, rkyv::rancor::Error>(&bytes).expect("restore input");
    assert!(matches!(restored, orchestrate::Input::Observe(_)));
}

#[test]
fn generated_decimal_signal_archives_and_bears_datom_derives() {
    let input = signal_decimal::Input::Measure(signal_decimal::Measurement {
        decimal: datom_codec::Decimal::try_from(1.25).expect("1.25 is finite"),
    });
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&input).expect("archive decimal input");
    let restored = rkyv::from_bytes::<signal_decimal::Input, rkyv::rancor::Error>(&bytes)
        .expect("restore decimal input");
    assert_eq!(restored, input);
}

#[test]
fn recursive_generated_signal_archives_and_restores() {
    use tree_types::{Chain, Input, Node_Data, Output, Tree, Twig, Twin};
    let tree = Tree::Node(Node_Data {
        first_tree: Box::new(Tree::Many(vec![Tree::Leaf(1), Tree::Maybe(None)])),
        second_tree: Box::new(Tree::Maybe(Some(Box::new(Tree::Many(vec![
            Tree::Leaf(2),
            Tree::Many(vec![]),
        ]))))),
    });
    let twin = Twin {
        first_twig: Twig::Grow(Box::new(Twin {
            first_twig: Twig::Tip,
            second_twig: Twig::Tip,
        })),
        second_twig: Twig::Tip,
    };
    let inputs = [Input::Plant(tree.clone()), Input::Twine(twin.clone())];
    for input in inputs {
        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&input).expect("archive recursive input");
        let restored = rkyv::from_bytes::<Input, rkyv::rancor::Error>(&bytes)
            .expect("restore recursive input");
        assert_eq!(restored, input);
    }
    let outputs = [
        Output::Planted(tree_types::Forest(vec![tree, Tree::Leaf(3)])),
        Output::Twined(Twig::Grow(Box::new(twin))),
    ];
    for output in outputs {
        let bytes =
            rkyv::to_bytes::<rkyv::rancor::Error>(&output).expect("archive recursive output");
        let restored = rkyv::from_bytes::<Output, rkyv::rancor::Error>(&bytes)
            .expect("restore recursive output");
        assert_eq!(restored, output);
    }
    let chain = Chain {
        string: "head".to_owned(),
        chain_option: Some(Box::new(Chain {
            string: "tail".to_owned(),
            chain_option: None,
        })),
    };
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&chain).expect("archive chain");
    let restored = rkyv::from_bytes::<Chain, rkyv::rancor::Error>(&bytes).expect("restore chain");
    assert_eq!(restored, chain);
    let knot = tree_types::Knot::Loop(tree_types::Loop {
        knot: Box::new(tree_types::Knot::Loop(tree_types::Loop {
            knot: Box::new(tree_types::Knot::End),
        })),
    });
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&knot).expect("archive knot");
    let restored =
        rkyv::from_bytes::<tree_types::Knot, rkyv::rancor::Error>(&bytes).expect("restore knot");
    assert_eq!(restored, knot);
}

#[test]
fn self_bearing_methods_are_sized_without_sizing_the_trait() {
    fn object_safe(value: &dyn self_traits::Mixed) -> String {
        value.inspect()
    }
    let _ = object_safe;
    let generated = include_str!("generated/self-traits.rs");
    assert!(generated.contains("fn factory(&self) -> Self"));
    assert!(generated.contains("Self: Sized;"));
    assert!(generated.contains("crate::Wrapper<Self>"));
    assert!(!generated.contains("pub trait Mixed: Sized"));
}

#[test]
fn constrained_trait_identity_compiles() {
    assert!(include_str!("generated/processable-traits.rs").contains("pub trait Processable"));
}

#[test]
fn operation_free_signal_shared_record_archives_and_round_trips_as_datom_text() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};

    let shared = empty_signal::Shared {
        name: empty_signal::Name("domain".to_owned()),
    };
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&shared).expect("archive shared record");
    assert_eq!(
        rkyv::from_bytes::<empty_signal::Shared, rkyv::rancor::Error>(&bytes)
            .expect("restore shared record"),
        shared
    );
    let text = shared.clone().datomize(vec![]).protosize().textualize();
    let mut pending = Potential::<empty_signal::Shared>::from(text);
    assert_eq!(
        pending
            .actualize(&mut Budget {
                remaining: 1024,
                reader: ReaderBudget::try_from(1024).unwrap(),
                depth: 0,
                maximum_depth: 1024,
            })
            .expect("restore shared datom"),
        shared
    );
}

#[test]
fn generated_memory_records_round_trip_as_datom_text() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};

    let record = entry_memory::Record {
        string: "root".to_owned(),
        entry_vector: vec![entry_memory::Entry {
            string: "first".to_owned(),
            integer: 1,
        }],
    };
    let text = record.clone().datomize(vec![]).protosize().textualize();
    assert_eq!(text, "{ root\n  [ { first 1 } ] }");
    let mut pending = Potential::<entry_memory::Record>::from(text);
    assert_eq!(
        pending
            .actualize(&mut Budget {
                remaining: 1024,
                reader: ReaderBudget::try_from(1024).unwrap(),
                depth: 0,
                maximum_depth: 1024,
            })
            .expect("restore memory record"),
        record
    );
}

#[test]
fn declared_newtypes_round_trip_as_datom_text() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};

    let short = alias_format::Short("Ada".to_owned());
    let text = short.clone().datomize(vec![]).protosize().textualize();
    assert_eq!(text, "Ada");
    let mut pending = Potential::<alias_format::Short>::from(text);
    let restored = pending
        .actualize(&mut Budget {
            remaining: 1024,
            reader: ReaderBudget::try_from(1024).unwrap(),
            depth: 0,
            maximum_depth: 1024,
        })
        .expect("restore declared newtype");
    assert_eq!(restored, short);

    let provider = alias_format::OptionalSpiritGuardianProviderName(Some(
        alias_format::SpiritGuardianProviderName("provider".to_owned()),
    ));
    let tokens = alias_format::OptionalSpiritGuardianMaximumOutputTokens(Some(
        alias_format::SpiritGuardianMaximumOutputTokens(4_096),
    ));
    let nested = alias_format::Nested(vec![Some(Ok(String::new())), Some(Err(1)), None]);
    assert!(provider.0.is_some() && tokens.0.is_some());
    assert_eq!(nested.0.len(), 3);
}

/// A sink bearing the fixture's own capability trait.
struct Buffer {
    lines: Vec<String>,
}

/// A line of text, which summarizes as itself.
struct Line(&'static str);

impl capability_traits::Summarizable for Line {
    fn summarize(&self) -> String {
        self.0.to_owned()
    }
}

impl capability_traits::Fillable for Buffer {
    fn push<N: capability_traits::Summarizable>(&mut self, input: N) -> Result<i64, SinkError> {
        if self.lines.len() > 1 {
            return Err(SinkError::Full);
        }
        self.lines.push(input.summarize());
        Ok(self.lines.len() as i64)
    }
    fn drain(&mut self) -> Vec<String> {
        std::mem::take(&mut self.lines)
    }
    fn create() -> Self {
        Self { lines: Vec::new() }
    }
}

impl capability_traits::Summarizable for Buffer {
    fn summarize(&self) -> String {
        self.lines.join(" ")
    }
}

#[test]
fn capability_traits_are_implementable_traits() {
    use capability_traits::{Fillable, Summarizable};
    let mut buffer = Buffer::create();
    assert_eq!(buffer.push(Line("a")).ok(), Some(1));
    assert_eq!(buffer.push(Line("b")).ok(), Some(2));
    assert!(matches!(buffer.push(Line("c")), Err(SinkError::Full)));
    assert_eq!(buffer.summarize(), "a b");
    // Any Summarizable is taken, another Buffer among them.
    let mut outer = Buffer::create();
    assert_eq!(outer.push(buffer).ok(), Some(1));
    assert_eq!(outer.summarize(), "a b");
    assert_eq!(outer.drain().len(), 1);
    assert!(!matches!(SinkError::Closed, SinkError::Full));
}

#[test]
fn declared_container_names_do_not_capture_the_generated_containers() {
    let tree = composition_types::Tree {
        tree_option: Some(std::boxed::Box::new(composition_types::Tree {
            tree_option: None,
            vec_integer_result: Err(1),
        })),
        vec_integer_result: Ok(composition_types::Vec {
            string: String::new(),
        }),
    };
    let nested = composition_types::Nested::Choice(composition_types::Choice_Data::Item(
        composition_types::Choice_Data_Item_Data {
            vec: composition_types::Vec {
                string: String::new(),
            },
            integer: 0,
        },
    ));
    assert!(tree.tree_option.is_some());
    assert!(matches!(nested, composition_types::Nested::Choice(_)));
}

#[test]
fn a_declared_single_letter_type_is_not_shadowed() {
    let holder = generic_shadow::Holder {
        string: String::new(),
        a: generic_shadow::A {
            string: "a".to_owned(),
        },
    };
    assert_eq!(holder.a.string, "a");
}

#[test]
fn several_declarations_generate_side_by_side() {
    let record = multi_types::Record {
        string: String::new(),
        integer: 1,
    };
    let report = multi_types::Report {
        string: String::new(),
        integer_vector: vec![record.integer],
    };
    let id = multi_types::LockId(7);
    assert_eq!(report.integer_vector, vec![1]);
    assert!(matches!(
        multi_types::SinkError::Closed,
        multi_types::SinkError::Closed
    ));
    assert_eq!(id.0, 7);
}

#[test]
fn every_scalar_intrinsic_has_a_position() {
    let placed = placed_types::Placed {
        integer_option: None,
        integer: 0,
    };
    let score = placed_types::Score {
        decimal: datom_codec::Decimal::try_from(0.5).expect("0.5 is finite"),
        boolean: true,
        meaning: datom_codec::Meaning(String::new()),
    };
    assert!(placed.integer_option.is_none());
    assert!(score.boolean);
}

#[test]
fn an_association_asserts_the_traits_a_type_bears() {
    let sink = sink_associations::Sink {
        string: "sunk".to_owned(),
        string_vector: vec![],
    };
    assert_eq!(sink.summarize(), "sunk");
    assert!(matches!(
        sink_associations::SinkError::Full,
        sink_associations::SinkError::Full
    ));
}

/// A stream bearing the fixture's complex trait.
struct Counter {
    count: i64,
}

impl Serializable for i64 {}
impl Fillable for Counter {}

impl streamable_trait::Streamable for Counter {
    type Item = i64;
    const CAPACITY: i64 = 2;
    fn next(&mut self) -> Option<i64> {
        if self.count >= Self::CAPACITY {
            return None;
        }
        self.count += 1;
        Some(self.count)
    }
}

#[test]
fn a_complex_trait_carries_its_supertrait_type_and_constant() {
    use streamable_trait::Streamable;
    let mut counter = Counter { count: 0 };
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), None);
}

/// A translator that observes what it was set to observe, whatever the call.
struct Scripted(command_emission::Outcome_Data);

/// The hand-written body an Emission leaves to its consumer: `Emitted`
/// already binds `Command_Data` to `Outcome_Data`, the first outcome, so
/// `emit` yields that type and no other.
impl command_emission::Emittable<Scripted> for command_emission::Command_Data {
    fn emit(&self, translator: &Scripted) -> command_emission::Outcome_Data {
        translator.0.clone()
    }
}

#[test]
fn an_emission_yields_the_outcome_its_position_binds() {
    use command_emission::{Argument, Command_Data, Emittable, Emitted, Outcome_Data, Verb};
    let command = Command_Data {
        verb: Verb::Show,
        argument_vector: vec![Argument::Name("pane".to_owned()), Argument::Quiet],
    };
    let outcome: <Command_Data as Emitted>::Outcome =
        command.emit(&Scripted(Outcome_Data::Refused(2)));
    assert_eq!(outcome, Outcome_Data::Refused(2));
    let whole = command_emission::Outcome::Outcome(outcome);
    assert!(matches!(
        whole,
        command_emission::Outcome::Outcome(Outcome_Data::Refused(2))
    ));
}

/// Every record of a Matrix bears `Memorizable`: it names the `Memory` it is
/// held in and recalls itself from that Memory's field for it, a declared
/// record and an imported one alike.
#[test]
fn every_record_of_a_matrix_recalls_itself_from_the_memory() {
    use matrix_records::{Configured, Memorizable, Memory, Watch};
    let memory = Memory {
        watch: Watch {
            flow_id: flow::FlowId(7),
            string: "pane".to_owned(),
        },
        configured: Configured(None),
        event: flow::Event::Started,
    };
    let watch: &Watch = <Watch as Memorizable>::recall(&memory);
    assert!(std::ptr::eq(watch, &memory.watch));
    let configured: &Configured = Configured::recall(&memory);
    assert!(std::ptr::eq(configured, &memory.configured));
    let event: &flow::Event = <flow::Event as Memorizable>::recall(&memory);
    assert!(std::ptr::eq(event, &memory.event));
    let held: <Watch as Memorizable>::Memory = memory.clone();
    assert_eq!(held, memory);
}

/// A translator that observes what it was set to observe, for the emissions
/// written bare.
struct Prober(bare_emission::Probed_Data);

/// A bare emission carries a unit of its own, so it bears `Emittable` as an
/// emission with a payload does: `Probe` is bound to `Probed_Data`, the first
/// outcome, and `Ping` to `()`, as its outcome `Pong` carries nothing.
impl bare_emission::Emittable<Prober> for bare_emission::Probe {
    fn emit(&self, translator: &Prober) -> bare_emission::Probed_Data {
        translator.0.clone()
    }
}
impl bare_emission::Emittable<Prober> for bare_emission::Ping {
    fn emit(&self, _translator: &Prober) {}
}

#[test]
fn a_bare_emission_carries_its_unit_and_yields_the_outcome_its_position_binds() {
    use bare_emission::{Emission, Emittable, Emitted, Ping, Probe, Probed_Data};
    let outcome: <Probe as Emitted>::Outcome = Probe {}.emit(&Prober(Probed_Data::Observed(7)));
    assert_eq!(outcome, Probed_Data::Observed(7));
    let (): <Ping as Emitted>::Outcome = Ping {}.emit(&Prober(Probed_Data::Down));
    let emissions = [Emission::Probe, Emission::Ping];
    assert!(matches!(emissions[0], Emission::Probe));
    assert!(matches!(emissions[1], Emission::Ping));
}

/// A Flow Nexus that launches every voice but one, as flow 7, and takes
/// every report.
struct Launcher(Voice);

/// The capabilities a Signal leaves to its consumer: each input's own,
/// borrowing what the input carries; the input's generated `answer`
/// dispatches to them.
impl flow_signal::Answering for Launcher {
    fn launch(&self, launch: &flow_signal::Launch_Data) -> Result<FlowId, flow_signal::Output> {
        if launch.voice == self.0 {
            return Err(flow_signal::Output::Refused(
                flow_signal::Refused_Data::VoiceBusy(launch.voice.clone()),
            ));
        }
        Ok(FlowId(7))
    }

    fn report(&self, _report: &flow_signal::Report_Data) -> Result<(), flow_signal::Output> {
        Ok(())
    }
}

#[test]
fn a_signal_answers_each_input_by_its_capability_and_the_output_its_position_pairs() {
    use flow_signal::{Answerable, Brief, Input, Launch_Data, Output, Refused_Data, Report_Data};
    let busy = Voice::Mind(flow_library::Rank::Primary);
    let launcher = Launcher(busy.clone());
    let launch = |voice: Voice| {
        Input::Launch(Launch_Data {
            voice,
            brief: Brief("work".to_owned()),
        })
    };
    assert_eq!(
        launch(Voice::Psyche(flow_library::Rank::Secondary)).answer(&launcher),
        Output::Launched(FlowId(7))
    );
    assert_eq!(
        launch(busy.clone()).answer(&launcher),
        Output::Refused(Refused_Data::VoiceBusy(busy))
    );
    assert_eq!(
        Input::Report(Report_Data {
            flow_id: FlowId(7),
            event: Event::Stopped,
        })
        .answer(&launcher),
        Output::Reported
    );
}

/// A Nexus whose Signal has an input named `Answer`: its capability is
/// `answer`, which the dispatch, the input's own `answer`, leaves free.
struct Oracle;

impl answer_input::Answering for Oracle {
    fn answer(
        &self,
        question: &answer_input::Question,
    ) -> Result<answer_input::Reply, answer_input::Output> {
        Ok(answer_input::Reply(format!("{} answered", question.0)))
    }

    fn wake(&self, nexus: &answer_input::Nexus) -> Result<(), answer_input::Output> {
        if nexus.0.is_empty() {
            return Err(answer_input::Output::Woken);
        }
        Ok(())
    }
}

#[test]
fn an_input_named_answer_is_answered_by_its_own_capability() {
    use answer_input::{Answerable, Input, Nexus, Output, Question, Reply};
    assert_eq!(
        Input::Answer(Question("why".to_owned())).answer(&Oracle),
        Output::Answered(Reply("why answered".to_owned()))
    );
    assert_eq!(
        Input::Wake(Nexus("flow".to_owned())).answer(&Oracle),
        Output::Woken
    );
}

#[test]
fn the_flow_nexus_operation_and_memory_compile_and_round_trip_as_datom_text() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};
    let budget = || Budget {
        remaining: 1024,
        reader: ReaderBudget::try_from(1024).unwrap(),
        depth: 0,
        maximum_depth: 1024,
    };
    let operation = flow_operation::Operation::Start(flow_operation::Start_Data {
        voice: Voice::Mind(flow_library::Rank::Primary),
        capsule: flow_operation::Capsule {
            home: flow_operation::Home("/home/flow".to_owned()),
            login: flow_operation::Login(vec!["claude".to_owned()]),
        },
    });
    let text = operation.datomize(vec![]).protosize().textualize();
    let mut pending = Potential::<flow_operation::Operation>::from(text);
    assert_eq!(
        pending.actualize(&mut budget()).expect("restore operation"),
        operation
    );
    let outcome = flow_operation::Outcome::Started(FlowId(7));
    assert!(matches!(
        outcome,
        flow_operation::Outcome::Started(FlowId(7))
    ));
    let memory = flow_memory::Flow {
        flow_id: FlowId(7),
        voice: Voice::Psyche(flow_library::Rank::Secondary),
        state: flow_memory::State::Running,
        event_vector: vec![
            Event::Started,
            Event::ToolUsed("Bash".to_owned()),
            Event::Stopped,
        ],
    };
    let text = memory.datomize(vec![]).protosize().textualize();
    let mut pending = Potential::<flow_memory::Flow>::from(text);
    assert_eq!(
        pending.actualize(&mut budget()).expect("restore memory"),
        memory
    );
    let _: FlowId = memory.flow_id;
}

#[test]
fn generated_launch_parses_the_supplied_newtype_form() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use launch_newtypes::{Address, Aspect, Brief, Input, Launch_Data, Layer, Remembering, Topic};
    use protos::{Compactable, Protosizable, ReaderBudget};
    let text = "Launch.{ { Mind flowNexus Tertiary } «» «Report that this test flow started.» }";
    let actual = Potential::<Input>::from(text)
        .actualize(&mut Budget {
            remaining: 1024,
            reader: ReaderBudget::try_from(1024).unwrap(),
            depth: 0,
            maximum_depth: 128,
        })
        .expect("the supplied Launch form composes through generated newtypes");
    assert_eq!(
        actual,
        Input::Launch(Launch_Data {
            address: Address {
                aspect: Aspect::Mind,
                topic: Topic("flowNexus".into()),
                layer: Layer::Tertiary
            },
            remembering: Remembering(String::new()),
            brief: Brief("Report that this test flow started.".into()),
        })
    );
    assert_eq!(actual.datomize(vec![]).protosize().compact(), text);
}

#[test]
fn a_scalar_newtype_derives_what_its_scalar_supports() {
    use scalar_newtypes::{Alias, Enabled, ExchangeId, Host, Port, Weight};
    fn copyable<T: Copy + Ord + std::hash::Hash>() {}
    fn ordered<T: Ord + std::hash::Hash>() {}
    // Integer, Boolean and Decimal newtypes, direct and chained: Copy and Ord.
    copyable::<ExchangeId>();
    copyable::<Port>();
    copyable::<Enabled>();
    copyable::<Weight>();
    // String newtypes, direct and chained: Ord without Copy.
    ordered::<Host>();
    ordered::<Alias>();
    let first = ExchangeId(0);
    let copy = first;
    assert!(first < ExchangeId(1) && copy == first);
    assert!(Port(ExchangeId(1)) > Port(first));
    assert!(Host("a".into()) < Host("b".into()));
}

/// The derive list the generated text carries for a declared type.
fn derives_of(declaration: &str) -> &'static str {
    let text = include_str!("generated/scalar-newtypes.rs");
    let marker = format!("pub struct {declaration}");
    let before = text.split(&marker).next().unwrap();
    let list = before.rsplit("#[derive(").next().unwrap();
    list.split(")]").next().unwrap().to_owned().leak()
}

#[test]
fn the_capability_matrix_adds_only_what_the_scalar_supports() {
    let supported = |name: &str, copy: bool, ordering: bool| {
        let derives = derives_of(name);
        let has = |word: &str| derives.split(", ").any(|derived| derived == word);
        assert_eq!(has("Copy"), copy, "{name}: Copy");
        assert_eq!(has("PartialOrd"), ordering, "{name}: PartialOrd");
        assert_eq!(has("Ord"), ordering, "{name}: Ord");
        assert!(has("Eq") && has("Hash"), "{name}: Eq and Hash stay");
        for arithmetic in ["Add", "Sub", "Mul", "Div", "Default"] {
            assert!(!has(arithmetic), "{name}: {arithmetic} is never derived");
        }
    };
    for name in ["ExchangeId", "Port", "Enabled", "Weight"] {
        supported(name, true, true);
    }
    for name in ["Host", "Alias"] {
        supported(name, false, true);
    }
    // Meaning has no order; a container and an imported name establish none.
    for name in ["Note", "Names", "Failure", "Record"] {
        supported(name, false, false);
    }
}

/// A Nexus that launches every request as flow 7, streams the panes it
/// holds, and refuses to stream another's.
struct Publisher;

impl stream_signal::Answering for Publisher {
    fn launch(
        &self,
        _launch_request: &stream_signal::LaunchRequest,
    ) -> Result<stream_signal::FlowId, stream_signal::Output> {
        Ok(stream_signal::FlowId(7))
    }

    fn watch(
        &self,
        selection: &stream_signal::Selection,
    ) -> Result<Stream<Vec<stream_signal::Pane>, stream_signal::Changed>, stream_signal::Output>
    {
        use stream_signal::{Changed, Pane, Selection};
        match selection {
            Selection::All => {
                let pane = Pane {
                    string: "editor".to_owned(),
                    integer: 1,
                };
                Ok(Stream(
                    vec![
                        Frame::Opened(vec![pane.clone()]),
                        Frame::Changed(Changed::Opened(pane)),
                        Frame::Changed(Changed::Closed(1)),
                        Frame::Ended(Ending::Closed),
                    ]
                    .into_iter(),
                ))
            }
            Selection::Mine => Err(stream_signal::Output::Refused("none held".to_owned())),
        }
    }

    fn subscribe(
        &self,
    ) -> Result<Stream<stream_signal::FlowId, stream_signal::Changed>, stream_signal::Output> {
        Ok(Stream(
            vec![
                Frame::Opened(stream_signal::FlowId(7)),
                Frame::Lagged(stream_signal::FlowId(8)),
            ]
            .into_iter(),
        ))
    }
}

#[test]
fn a_streaming_signal_answers_once_or_with_a_stream_whose_every_frame_is_one_output() {
    use stream_signal::{Answer, Answerable, FlowId, Input, LaunchRequest, Output, Selection};
    let launched = Input::Launch(LaunchRequest {
        string: "work".to_owned(),
    })
    .answer(&Publisher);
    assert!(matches!(
        launched,
        Answer::Once(Output::Launched(FlowId(7)))
    ));
    let refused = Input::Watch(Selection::Mine).answer(&Publisher);
    assert!(matches!(refused, Answer::Once(Output::Refused(_))));
    let Answer::Watched(stream) = Input::Watch(Selection::All).answer(&Publisher) else {
        panic!("watching every pane is answered with a stream");
    };
    let mut frames = 0;
    for frame in stream {
        let output = Output::Watched(frame);
        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&output).expect("archive a frame");
        let restored =
            rkyv::from_bytes::<Output, rkyv::rancor::Error>(&bytes).expect("restore a frame");
        assert_eq!(restored, output);
        frames += 1;
    }
    assert_eq!(frames, 4);
    let Answer::Subscribed(stream) = Input::Subscribe.answer(&Publisher) else {
        panic!("subscribing is answered with a stream");
    };
    let outputs: Vec<Output> = stream.map(Output::Subscribed).collect();
    assert_eq!(
        outputs,
        vec![
            Output::Subscribed(Frame::Opened(FlowId(7))),
            Output::Subscribed(Frame::Lagged(FlowId(8))),
        ]
    );
}

#[test]
fn a_frame_round_trips_as_datom_text() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};
    let output = stream_signal::Output::Watched(Frame::Changed(stream_signal::Changed::Closed(3)));
    let text = output.datomize(vec![]).protosize().textualize();
    let mut pending = Potential::<stream_signal::Output>::from(text);
    assert_eq!(
        pending
            .actualize(&mut Budget {
                remaining: 1024,
                reader: ReaderBudget::try_from(1024).unwrap(),
                depth: 0,
                maximum_depth: 1024,
            })
            .expect("restore a frame"),
        output
    );
}

/// A publisher of a count, bearing the capability that opens a stream of
/// the trait's own state and change.
struct Count(i64);

impl publishing_trait::Publishing for Count {
    type State = i64;
    type Change = i64;

    fn open(&self, input: i64) -> Stream<i64, i64> {
        Stream(vec![Frame::Opened(input), Frame::Changed(self.0)].into_iter())
    }
}

#[test]
fn a_capability_yields_a_stream_of_its_associated_types() {
    use publishing_trait::Publishing;
    let frames: Vec<Frame<i64, i64>> = Count(2).open(1).collect();
    assert_eq!(frames, vec![Frame::Opened(1), Frame::Changed(2)]);
    assert_ne!(
        Frame::<i64, i64>::Ended(Ending::Refused),
        Frame::Ended(Ending::Closed)
    );
}

/// The budget a datom is read against.
fn budget() -> datom_codec::Budget {
    datom_codec::Budget {
        remaining: 1024,
        reader: protos::ReaderBudget::try_from(1024).unwrap(),
        depth: 0,
        maximum_depth: 1024,
    }
}

/// The datom a text is, formed but not composed into any type: what a
/// runtime holds before it knows which type a name carries.
fn formed(text: &str) -> datom_codec::Datom {
    use datom_codec::DatomForming;
    use protos::Protosizable;
    text.to_owned()
        .protosize()
        .expect("text reads")
        .datom_form(vec![])
        .expect("text forms a datom")
}

#[test]
fn an_open_enums_contract_round_trips_as_datom_text() {
    use datom_codec::{Actualizing, Datomizable, Potential};
    use open_enum_signal::{Input, Opening, Output, model};
    use protos::{Compactable, Protosizable};

    // Input reaches the open enum, so it is read through Opening, with a
    // registry; the contract's own variants need none of its entries.
    let none = Added(vec![]);
    let written = "Model.Add.{ Fable Some.Name.QuadrupleEffort }";
    let added = Input::Model(model::Input::Add(model::Variant {
        name: model::Name("Fable".to_owned()),
        type_option: Some(model::Type::Name(model::Name("QuadrupleEffort".to_owned()))),
    }));
    assert_eq!(
        Input::open(&formed(written), &mut budget(), &none).expect("the notion's datom reads"),
        added
    );
    assert_eq!(added.datomize(vec![]).protosize().compact(), written);
    for (text, input) in [
        ("Model.List", Input::Model(model::Input::List)),
        (
            "Model.Remove.Fable",
            Input::Model(model::Input::Remove(model::Name("Fable".to_owned()))),
        ),
        (
            "Model.Add.{ Plain None }",
            Input::Model(model::Input::Add(model::Variant {
                name: model::Name("Plain".to_owned()),
                type_option: None,
            })),
        ),
    ] {
        let read = Input::open(&formed(text), &mut budget(), &none).expect("reads");
        assert_eq!(read, input, "{text}");
        assert_eq!(input.datomize(vec![]).protosize().compact(), text);
    }
    let output = Output::Model(model::Output::Listed(vec![]));
    let text = output.datomize(vec![]).protosize().compact();
    assert_eq!(
        Potential::<Output>::from(text)
            .actualize(&mut budget())
            .expect("an output reads"),
        output
    );
}

#[test]
fn an_open_enums_type_is_any_type_of_the_syntax_which_the_runtime_refuses() {
    use open_enum_signal::{Input, Opening, model};
    // Only a name is carried at run time, but every form of a type reads, so
    // the runtime can answer Refused to the others.
    let read = Input::open(
        &formed("Model.Add.{ Fable Some.Vector.Name.Low }"),
        &mut budget(),
        &Added(vec![]),
    )
    .expect("a vector of a type reads");
    let Input::Model(model::Input::Add(variant)) = read else {
        panic!("an add")
    };
    assert!(matches!(variant.type_option, Some(model::Type::Vector(_))));
}

/// A runtime's registry: the variants added so far.
struct Added(Vec<open_enum_signal::model::Variant>);

impl open_enum_signal::model::Registry for Added {
    fn variant(
        &self,
        name: &open_enum_signal::model::Name,
    ) -> Option<open_enum_signal::model::Variant> {
        self.0.iter().find(|variant| &variant.name == name).cloned()
    }
}

impl Added {
    fn entry(name: &str, carried: Option<open_enum_signal::model::Type>) -> Self {
        Added(vec![open_enum_signal::model::Variant {
            name: open_enum_signal::model::Name(name.to_owned()),
            type_option: carried,
        }])
    }

    fn naming(name: &str, carried: &str) -> Self {
        Self::entry(
            name,
            Some(open_enum_signal::model::Type::Name(
                open_enum_signal::model::Name(carried.to_owned()),
            )),
        )
    }
}

#[test]
fn an_open_enums_value_is_read_with_the_registry_of_added_variants() {
    use datom_codec::{Datomizable, ErrorKind};
    use open_enum_signal::{Carried, Model, Opening, QuadrupleEffort, TripleEffort, model};
    use protos::{Compactable, Protosizable};

    let registry = Added::naming("Fable", "QuadrupleEffort");
    let value = Model::open(&formed("Fable.High"), &mut budget(), &registry).expect("reads");
    assert_eq!(
        value,
        Model {
            name: model::Name("Fable".to_owned()),
            carried: Some(Carried::QuadrupleEffort(QuadrupleEffort::High)),
        }
    );
    assert_eq!(value.datomize(vec![]).protosize().compact(), "Fable.High");
    // The same head reads as the other type once the registry says so.
    let triple = Added::naming("Fable", "TripleEffort");
    assert_eq!(
        Model::open(&formed("Fable.High"), &mut budget(), &triple)
            .expect("reads")
            .carried,
        Some(Carried::TripleEffort(TripleEffort::High))
    );
    // `Maximum` is a QuadrupleEffort and no TripleEffort.
    assert!(Model::open(&formed("Fable.Maximum"), &mut budget(), &triple).is_err());
    assert!(Model::open(&formed("Fable.Maximum"), &mut budget(), &registry).is_ok());

    // A name that carries nothing is read bare, with or without a registry.
    let plain = Added::entry("Plain", None);
    let bare = Model::open(&formed("Plain"), &mut budget(), &plain).expect("reads");
    assert_eq!(bare.carried, None);
    assert_eq!(bare.datomize(vec![]).protosize().compact(), "Plain");

    // A head that was never added is refused where it is read.
    let kind = |text: &str, registry: &Added| {
        Model::open(&formed(text), &mut budget(), registry)
            .expect_err(text)
            .kind
    };
    assert_eq!(
        kind("Ghost.High", &registry),
        ErrorKind::Variant {
            expected: "Model".to_owned(),
            found: "Ghost".to_owned(),
        }
    );
    // A name that carries nothing is no head of a value, and one that carries
    // a type is no bare name.
    assert!(matches!(
        kind("Plain.High", &plain),
        ErrorKind::Value { .. }
    ));
    assert!(matches!(kind("Fable", &registry), ErrorKind::Value { .. }));
    // A type the file does not declare, or any form of type but a name.
    assert_eq!(
        kind("Fable.High", &Added::naming("Fable", "Missing")),
        ErrorKind::Variant {
            expected: "a type the file declares".to_owned(),
            found: "Missing".to_owned(),
        }
    );
    let vector = Added::entry(
        "Fable",
        Some(model::Type::Vector(Box::new(model::Type::Name(
            model::Name("TripleEffort".to_owned()),
        )))),
    );
    assert!(matches!(
        kind("Fable.High", &vector),
        ErrorKind::Value { .. }
    ));
}

#[test]
fn a_whole_input_is_read_through_opening_with_its_registry() {
    use datom_codec::ErrorKind;
    use open_enum_signal::{Carried, Input, Model, Opening, QuadrupleEffort, model};
    let registry = Added::naming("Fable", "QuadrupleEffort");
    assert_eq!(
        Input::open(&formed("Launch.Fable.High"), &mut budget(), &registry).expect("reads"),
        Input::Launch(Model {
            name: model::Name("Fable".to_owned()),
            carried: Some(Carried::QuadrupleEffort(QuadrupleEffort::High)),
        })
    );
    // A head never added is refused where it is read, inside the Input too.
    assert_eq!(
        Input::open(&formed("Launch.Jev"), &mut budget(), &registry)
            .expect_err("never added")
            .kind,
        ErrorKind::Variant {
            expected: "Model".to_owned(),
            found: "Jev".to_owned(),
        }
    );
    assert!(matches!(
        Input::open(&formed("Halt.Fable.High"), &mut budget(), &registry)
            .expect_err("no such input")
            .kind,
        ErrorKind::Variant { .. }
    ));
}

#[test]
fn a_body_reads_what_carried_holds_by_the_name_of_its_type() {
    use open_enum_signal::{Carried, Carrying, QuadrupleEffort, TripleEffort};
    assert_eq!(Carried::NAMES, ["TripleEffort", "QuadrupleEffort"]);
    let high = Carried::read("QuadrupleEffort", &formed("High"), &mut budget()).expect("reads");
    assert_eq!(high, Carried::QuadrupleEffort(QuadrupleEffort::High));
    assert_eq!(high.name(), "QuadrupleEffort");
    assert_eq!(
        Carried::TripleEffort(TripleEffort::Low).name(),
        "TripleEffort"
    );
    assert!(Carried::read("Missing", &formed("High"), &mut budget()).is_err());
    assert!(Carried::read("TripleEffort", &formed("Maximum"), &mut budget()).is_err());
}

/// A registry of both open enums of the held Signal: Fable a Model, High an
/// Effort carrying a Mark.
struct Both;

impl open_enum_held::model::Registry for Both {
    fn variant(
        &self,
        name: &open_enum_held::model::Name,
    ) -> Option<open_enum_held::model::Variant> {
        (name.0 == "Fable").then(|| open_enum_held::model::Variant {
            name: name.clone(),
            type_option: None,
        })
    }
}

impl open_enum_held::effort::Registry for Both {
    fn variant(
        &self,
        name: &open_enum_held::effort::Name,
    ) -> Option<open_enum_held::effort::Variant> {
        (name.0 == "High").then(|| open_enum_held::effort::Variant {
            name: name.clone(),
            type_option: Some(open_enum_held::effort::Type::Name(
                open_enum_held::effort::Name("Mark".to_owned()),
            )),
        })
    }
}

#[test]
fn a_type_that_holds_two_open_enums_is_read_with_a_registry_of_each() {
    use datom_codec::Datomizable;
    use open_enum_held::{Carried, Effort, Input, Mark, Model, Opening, Pair, effort, model};
    use protos::{Compactable, Protosizable};
    let text = "Launch.{ Fable High.seal }";
    let input = Input::open(&formed(text), &mut budget(), &Both).expect("reads");
    assert_eq!(
        input,
        Input::Launch(Pair {
            model: Model {
                name: model::Name("Fable".to_owned()),
                carried: None,
            },
            effort: Effort {
                name: effort::Name("High".to_owned()),
                carried: Some(Carried::Mark(Mark("seal".to_owned()))),
            },
        })
    );
    assert_eq!(input.datomize(vec![]).protosize().compact(), text);
    // Either head never added is refused inside the struct.
    assert!(Input::open(&formed("Launch.{ Jev High.seal }"), &mut budget(), &Both).is_err());
    assert!(Input::open(&formed("Launch.{ Fable Low.seal }"), &mut budget(), &Both).is_err());
    assert!(Input::open(&formed("Launch.{ Fable }"), &mut budget(), &Both).is_err());
}

/// A Nexus of the open-enum Signal: it launches, and answers the contract of
/// the open enum by a capability of its own.
struct Roster;

impl open_enum_signal::Answering for Roster {
    fn launch(&self, model: &open_enum_signal::Model) -> Result<(), open_enum_signal::Output> {
        if model.carried.is_some() {
            return Err(open_enum_signal::Output::Refused("carried".to_owned()));
        }
        Ok(())
    }

    fn stop(&self, model: &open_enum_signal::Model) -> Result<String, open_enum_signal::Output> {
        Ok(model.name.0.clone())
    }

    fn model(
        &self,
        model: &open_enum_signal::model::Input,
    ) -> Result<open_enum_signal::model::Output, open_enum_signal::Output> {
        use open_enum_signal::model::{Input, Output};
        Ok(match model {
            Input::Add(variant) => Output::Added(variant.clone()),
            Input::Remove(name) => Output::Removed(name.clone()),
            Input::List => Output::Refused("empty".to_owned()),
        })
    }
}

#[test]
fn an_input_of_an_open_enum_is_answered_by_the_capability_named_for_it() {
    use open_enum_signal::{Answerable, Input, Model, Output, model};
    let name = model::Name("Fable".to_owned());
    let variant = model::Variant {
        name: name.clone(),
        type_option: None,
    };
    assert_eq!(
        Input::Model(model::Input::Add(variant.clone())).answer(&Roster),
        Output::Model(model::Output::Added(variant))
    );
    assert_eq!(
        Input::Model(model::Input::Remove(name.clone())).answer(&Roster),
        Output::Model(model::Output::Removed(name.clone()))
    );
    assert_eq!(
        Input::Model(model::Input::List).answer(&Roster),
        Output::Model(model::Output::Refused("empty".to_owned()))
    );
    let value = Model {
        name,
        carried: None,
    };
    assert_eq!(
        Input::Launch(value.clone()).answer(&Roster),
        Output::Launched
    );
    assert_eq!(
        Input::Stop(value).answer(&Roster),
        Output::Refused("Fable".to_owned())
    );
}

#[test]
fn a_type_that_holds_an_open_enum_is_not_carried_by_it() {
    use open_enum_held::{Carried, Effort, Mark, Model, effort, model};
    // `Carried` holds `Mark` alone: `Pair` holds both open enums, so would
    // make the value hold itself.
    let carried = Carried::Mark(Mark("seal".to_owned()));
    let Carried::Mark(mark) = &carried;
    assert_eq!(mark.0, "seal");
    let model = Model {
        name: model::Name("Fable".to_owned()),
        carried: Some(carried.clone()),
    };
    let effort = Effort {
        name: effort::Name("High".to_owned()),
        carried: Some(carried),
    };
    assert_eq!(
        open_enum_held::Pair { model, effort }.model.name,
        model::Name("Fable".to_owned())
    );
    assert_eq!(effort::Input::List, effort::Input::List);
}

#[test]
fn an_open_enum_named_for_a_crate_shadows_no_path_the_code_writes() {
    use open_enum_shadow::{Alloc, Core, Flow, Pair, Std, alloc, core, flow, std};
    // `core`, `flow`, `alloc` and `std` are the contract modules of `Core`,
    // `Flow`, `Alloc` and `Std`; the `Voice` imported from flow is still the
    // crate's own, and no name is refused for the sake of Rust.
    let pair = Pair {
        core: Core {
            name: core::Name("Rank".to_owned()),
            carried: None,
        },
        flow: Flow {
            name: flow::Name("Voice".to_owned()),
            carried: None,
        },
        alloc: Alloc {
            name: alloc::Name("Heap".to_owned()),
            carried: None,
        },
        std: Std {
            name: std::Name("Library".to_owned()),
            carried: None,
        },
        voice: flow_library::Voice::Mind(flow_library::Rank::Primary),
    };
    assert_eq!(pair.core.name, core::Name("Rank".to_owned()));
    assert_eq!(pair.alloc.name, alloc::Name("Heap".to_owned()));
    assert_eq!(pair.std.name.0, "Library");
    assert_eq!(flow::Input::List, flow::Input::List);
    assert_eq!(alloc::Input::List, alloc::Input::List);
    assert_eq!(std::Input::List, std::Input::List);
}

/// The registry of the Signal that declares no type.
struct Bare(Vec<open_enum_bare::model::Variant>);

impl open_enum_bare::model::Registry for Bare {
    fn variant(
        &self,
        name: &open_enum_bare::model::Name,
    ) -> Option<open_enum_bare::model::Variant> {
        self.0.iter().find(|variant| &variant.name == name).cloned()
    }
}

#[test]
fn an_open_enum_of_a_signal_that_declares_no_type_carries_nothing() {
    use datom_codec::Datomizable;
    use open_enum_bare::{Input, Model, Opening, model};
    use protos::{Compactable, Protosizable};
    // There is no `Carried`, and the value is its name alone.
    let value = Model {
        name: model::Name("Plain".to_owned()),
    };
    assert_eq!(value.datomize(vec![]).protosize().compact(), "Plain");
    let registry = Bare(vec![model::Variant {
        name: model::Name("Plain".to_owned()),
        type_option: None,
    }]);
    assert_eq!(
        Model::open(&formed("Plain"), &mut budget(), &registry).expect("reads"),
        value
    );
    assert!(Model::open(&formed("Plain.High"), &mut budget(), &registry).is_err());
    assert_eq!(
        Input::Model(model::Input::List),
        Input::Model(model::Input::List)
    );
}

/// The qualified Nexus's registry: Fable, carrying an Effort.
struct Qualified;

impl contract::model::Registry for Qualified {
    fn variant(&self, name: &contract::model::Name) -> Option<contract::model::Variant> {
        (name.0 == "Fable").then(|| contract::model::Variant {
            name: name.clone(),
            type_option: Some(contract::model::Type::Name(contract::model::Name(
                "Effort".to_owned(),
            ))),
        })
    }
}

#[cfg(feature = "datom")]
#[test]
fn an_open_enum_named_by_its_own_namespace_is_read_through_opening() {
    use contract::{Carried, Effort, Input, Model, Opening, Pair};
    // Launch carries Model by `:qualified:contract:Model`, and Pair holds it
    // so: each is read only with the registry, and Pair is carried by none.
    let fable = Model {
        name: contract::model::Name("Fable".to_owned()),
        carried: Some(Carried::Effort(Effort::Low)),
    };
    assert_eq!(
        Input::open(&formed("Launch.Fable.Low"), &mut budget(), &Qualified)
            .expect("a launch reads"),
        Input::Launch(fable.clone())
    );
    assert_eq!(
        Input::open(
            &formed("Hold.{ Fable.Low High }"),
            &mut budget(),
            &Qualified
        )
        .expect("a held pair reads"),
        Input::Hold(Pair {
            model: fable,
            effort: Effort::High,
        })
    );
}

#[cfg(feature = "datom")]
#[test]
fn a_recursive_type_that_holds_no_open_enum_keeps_composing() {
    use datom_codec::{Actualizing, Potential};
    use open_enum_recursive::{Carried, Tree};
    // Self adds nothing to reach: Tree composes without a registry and is a
    // type Model carries.
    let tree = Potential::<Tree>::from("{ Some.{ None 1 } 2 }")
        .actualize(&mut budget())
        .expect("a tree composes");
    assert_eq!(
        tree,
        Tree {
            tree_option: Some(Box::new(Tree {
                tree_option: None,
                integer: 1,
            })),
            integer: 2,
        }
    );
    assert_eq!(<Carried as open_enum_recursive::Carrying>::NAMES, &["Tree"]);
}
