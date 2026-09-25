//! Every generated fixture compiles with Datom enabled.

pub struct Wrapper<T>(pub T);

/// What the kind fixtures import from `super`: the error a capability
/// yields, and the kinds a type is asserted to bear.
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
#[path = "generated/capability-kinds.rs"]
mod capability_kinds;
#[path = "generated/composition-types.rs"]
mod composition_types;
#[path = "generated/empty-signal.rs"]
mod empty_signal;
#[path = "generated/entry-sema.rs"]
mod entry_sema;
#[path = "generated/generic-shadow.rs"]
mod generic_shadow;
#[path = "generated/inline-collision.rs"]
mod inline_collision;
#[path = "generated/multi-types.rs"]
mod multi_types;
#[path = "generated/nested-collision.rs"]
mod nested_collision;
#[path = "generated/orchestrate.rs"]
mod orchestrate;
#[path = "generated/placed-types.rs"]
mod placed_types;
#[path = "generated/processable-kinds.rs"]
mod processable_kinds;
#[path = "generated/record-types.rs"]
mod record_types;
#[path = "generated/self-kinds.rs"]
mod self_kinds;
#[path = "generated/signal-decimal.rs"]
mod signal_decimal;
#[path = "generated/sink-associations.rs"]
mod sink_associations;
#[path = "generated/streamable-kind.rs"]
mod streamable_kind;
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
    let authored: inline_collision::X_Data = String::new();
    let unique = inline_collision::R::Z(inline_collision::Z_Data { string: authored });
    assert!(matches!(p, inline_collision::P::X(_)));
    assert!(matches!(nested, inline_collision::P::Y(_)));
    assert!(matches!(q, inline_collision::Q::X(_)));
    assert!(matches!(unique, inline_collision::R::Z(_)));
}

#[test]
fn named_fields_and_signal_query_response_compile() {
    let record = record_types::Record {
        string: "Ada".to_owned(),
        integer: 1990,
    };
    assert_eq!(record.integer, 1990);
    let query = orchestrate::Query::Observe(orchestrate::ObserveSelection::Locks(vec![]));
    let response = orchestrate::Response::Observed(orchestrate::Observation::Locks(vec![]));
    assert!(matches!(query, orchestrate::Query::Observe(_)));
    assert!(matches!(response, orchestrate::Response::Observed(_)));
}

#[test]
fn generated_signal_query_round_trips_as_a_portable_archive() {
    let query = orchestrate::Query::Observe(orchestrate::ObserveSelection::Locks(vec![]));
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&query).expect("archive query");
    let restored =
        rkyv::from_bytes::<orchestrate::Query, rkyv::rancor::Error>(&bytes).expect("restore query");
    assert!(matches!(restored, orchestrate::Query::Observe(_)));
}

#[test]
fn generated_decimal_signal_archives_and_bears_datom_derives() {
    let query = signal_decimal::Query::Measure(signal_decimal::Measurement {
        decimal: datom_codec::Decimal::try_from(1.25).expect("1.25 is finite"),
    });
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&query).expect("archive decimal query");
    let restored = rkyv::from_bytes::<signal_decimal::Query, rkyv::rancor::Error>(&bytes)
        .expect("restore decimal query");
    assert_eq!(restored, query);
}

#[test]
fn recursive_generated_signal_archives_and_restores() {
    use tree_types::{Chain, Node_Data, Query, Response, Tree, Twig, Twin};
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
    let queries = [Query::Plant(tree.clone()), Query::Twine(twin.clone())];
    for query in queries {
        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&query).expect("archive recursive query");
        let restored = rkyv::from_bytes::<Query, rkyv::rancor::Error>(&bytes)
            .expect("restore recursive query");
        assert_eq!(restored, query);
    }
    let responses = [
        Response::Planted(vec![tree, Tree::Leaf(3)]),
        Response::Twined(Twig::Grow(Box::new(twin))),
    ];
    for response in responses {
        let bytes =
            rkyv::to_bytes::<rkyv::rancor::Error>(&response).expect("archive recursive response");
        let restored = rkyv::from_bytes::<Response, rkyv::rancor::Error>(&bytes)
            .expect("restore recursive response");
        assert_eq!(restored, response);
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
    fn object_safe(value: &dyn self_kinds::Mixed) -> String {
        value.inspect()
    }
    let _ = object_safe;
    let generated = include_str!("generated/self-kinds.rs");
    assert!(generated.contains("fn factory(&self) -> Self"));
    assert!(generated.contains("Self: Sized;"));
    assert!(generated.contains("crate::Wrapper<Self>"));
    assert!(!generated.contains("pub trait Mixed: Sized"));
}

#[test]
fn constrained_kind_identity_compiles() {
    assert!(include_str!("generated/processable-kinds.rs").contains("pub trait Processable"));
}

#[test]
fn operation_free_signal_shared_record_archives_and_round_trips_as_datom_text() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};

    let shared = empty_signal::Shared {
        name: "domain".to_owned(),
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
                reader: ReaderBudget { remaining: 1024 },
                depth: 0,
                maximum_depth: 1024,
            })
            .expect("restore shared datom"),
        shared
    );
}

#[test]
fn generated_sema_records_round_trip_as_datom_text() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};

    let record = entry_sema::Record {
        string: "root".to_owned(),
        entry_vector: vec![entry_sema::Entry {
            string: "first".to_owned(),
            integer: 1,
        }],
    };
    let text = record.clone().datomize(vec![]).protosize().textualize();
    assert_eq!(text, "{ root [ { first 1 } ] }");
    let mut pending = Potential::<entry_sema::Record>::from(text);
    assert_eq!(
        pending
            .actualize(&mut Budget {
                remaining: 1024,
                reader: ReaderBudget { remaining: 1024 },
                depth: 0,
                maximum_depth: 1024,
            })
            .expect("restore sema record"),
        record
    );
}

#[test]
fn aliases_name_their_types() {
    let short: alias_format::Short = String::new();
    let provider: alias_format::OptionalSpiritGuardianProviderName = Some(short);
    let tokens: alias_format::OptionalSpiritGuardianMaximumOutputTokens = Some(4_096);
    let nested: alias_format::Nested = vec![Some(Ok(String::new())), Some(Err(1)), None];
    assert!(provider.is_some() && tokens.is_some());
    assert_eq!(nested.len(), 3);
}

/// A sink bearing the fixture's own capability kind.
struct Buffer {
    lines: Vec<String>,
}

impl capability_kinds::Fillable for Buffer {
    fn push(&mut self, input: String) -> Result<i64, SinkError> {
        if self.lines.len() > 1 {
            return Err(SinkError::Full);
        }
        self.lines.push(input);
        Ok(self.lines.len() as i64)
    }
    fn drain(&mut self) -> Vec<String> {
        std::mem::take(&mut self.lines)
    }
    fn create() -> Self {
        Self { lines: Vec::new() }
    }
}

impl capability_kinds::Summarizable for Buffer {
    fn summarize(&self) -> String {
        self.lines.join(" ")
    }
}

#[test]
fn capability_kinds_are_implementable_traits() {
    use capability_kinds::{Fillable, Summarizable};
    let mut buffer = Buffer::create();
    assert_eq!(buffer.push("a".to_owned()).ok(), Some(1));
    assert_eq!(buffer.push("b".to_owned()).ok(), Some(2));
    assert!(matches!(buffer.push("c".to_owned()), Err(SinkError::Full)));
    assert_eq!(buffer.summarize(), "a b");
    assert_eq!(buffer.drain().len(), 2);
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
    let id: multi_types::LockId = 7;
    assert_eq!(report.integer_vector, vec![1]);
    assert!(matches!(
        multi_types::SinkError::Closed,
        multi_types::SinkError::Closed
    ));
    assert_eq!(id, 7);
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
fn an_association_asserts_the_kinds_a_type_bears() {
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

/// A stream bearing the fixture's complex kind.
struct Counter {
    count: i64,
}

impl Serializable for i64 {}
impl Fillable for Counter {}

impl streamable_kind::Streamable for Counter {
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
fn a_complex_kind_carries_its_superkind_type_and_constant() {
    use streamable_kind::Streamable;
    let mut counter = Counter { count: 0 };
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), None);
}
