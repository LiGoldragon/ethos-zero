#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Pane {
    pub string: String,
}
#[rustfmt::skip]
pub struct WakeWaits_State {
    pub(super) string_vector: std::vec::Vec<String>,
    pub(super) integer: i64,
}
#[rustfmt::skip]
#[derive(Clone)]
pub struct WakeWaits {
    pub(super) state: std::sync::Arc<std::sync::Mutex<WakeWaits_State>>,
}
#[rustfmt::skip]
pub trait Paned {
    fn pane(&self) -> String;
}
#[rustfmt::skip]
pub trait Waiting {
    fn waits(&self) -> std::vec::Vec<String>;
    fn touch(&self);
    fn wait<N: Paned>(&self, input: N);
    fn create() -> Self
    where
        Self: Sized;
}
#[rustfmt::skip]
const _: () = {
    fn assert_pane_paned<T: Paned>() {}
    let _ = assert_pane_paned::<Pane>;
};
#[rustfmt::skip]
const _: () = {
    fn assert_wakewaits_waiting<T: Waiting>() {}
    let _ = assert_wakewaits_waiting::<WakeWaits>;
};
