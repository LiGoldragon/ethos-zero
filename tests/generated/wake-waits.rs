#![allow(dead_code, non_camel_case_types, non_snake_case)]
#![allow(clippy::type_complexity)]
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct PaneId(pub String);
#[rustfmt::skip]
pub struct WakeWaits_State {
    pub(super) pane_id_vector: std::vec::Vec<PaneId>,
    pub(super) integer: i64,
}
#[rustfmt::skip]
#[derive(Clone)]
pub struct WakeWaits {
    pub(super) state: std::sync::Arc<std::sync::Mutex<WakeWaits_State>>,
}
#[rustfmt::skip]
pub trait Paned {
    fn pane(&self) -> PaneId;
}
#[rustfmt::skip]
pub trait Waiting {
    fn waits(&self) -> std::vec::Vec<PaneId>;
    fn wait<N: Paned>(&self, input: N) -> i64;
    fn create() -> Self
    where
        Self: Sized;
}
#[rustfmt::skip]
const _: () = {
    fn assert_wakewaits_waiting<T: Waiting>() {}
    let _ = assert_wakewaits_waiting::<WakeWaits>;
};
