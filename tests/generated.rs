//! Every generated fixture compiles with Datom enabled.

// The Flow Nexus files import `flow:[ FlowId Voice Event ]`; here the crate
// itself is `flow`, re-exporting the generated Flow Library.
extern crate self as flow;
pub use flow_library::{Event, FlowId, Voice};

pub struct Wrapper<T>(pub T);

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
#[path = "generated/multi-types.rs"]
mod multi_types;
#[path = "generated/nested-collision.rs"]
mod nested_collision;
#[path = "generated/orchestrate.rs"]
mod orchestrate;
#[path = "generated/placed-types.rs"]
mod placed_types;
#[path = "generated/processable-traits.rs"]
mod processable_traits;
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
#[path = "generated/streamable-trait.rs"]
mod streamable_trait;
#[path = "generated/tree-types.rs"]
mod tree_types;

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
