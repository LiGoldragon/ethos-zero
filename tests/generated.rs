//! Current generated contracts compile with Datom enabled.

pub struct Wrapper<T>(pub T);

#[path = "generated/nested-collision.rs"]
mod nested_collision;
#[path = "generated/orchestrate.rs"]
mod orchestrate;
#[path = "generated/processable-kinds.rs"]
mod processable_kinds;
#[path = "generated/record-types.rs"]
mod record_types;
#[path = "generated/self-kinds.rs"]
mod self_kinds;
#[path = "generated/signal-decimal.rs"]
mod signal_decimal;
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
    let restored = rkyv::from_bytes::<orchestrate::Query, rkyv::rancor::Error>(&bytes)
        .expect("restore query");
    assert!(matches!(restored, orchestrate::Query::Observe(_)));
}

#[test]
fn generated_decimal_signal_archives_and_bears_datom_derives() {
    let query = signal_decimal::Query::Measure(signal_decimal::Measurement { decimal: 1.25 });
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&query).expect("archive decimal query");
    let restored = rkyv::from_bytes::<signal_decimal::Query, rkyv::rancor::Error>(&bytes)
        .expect("restore decimal query");
    assert_eq!(restored, query);
}

#[test]
fn recursive_generated_types_compile() {
    let tree = tree_types::Tree::Leaf(1);
    assert!(matches!(tree, tree_types::Tree::Leaf(1)));
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
