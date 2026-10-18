#![allow(dead_code, non_camel_case_types, non_snake_case)]
#![allow(clippy::type_complexity)]
#[rustfmt::skip]
pub struct Clock_State {
    pub(super) instant: std::time::Instant,
    pub(super) integer_vector_mutex: std::sync::Mutex<std::vec::Vec<i64>>,
}
#[rustfmt::skip]
#[derive(Clone)]
pub struct Clock {
    pub(super) state: std::sync::Arc<std::sync::Mutex<Clock_State>>,
}
#[rustfmt::skip]
pub trait Clocking {
    fn started(&self) -> std::time::Instant;
    fn lap<N: Lapping>(&self, input: N);
    fn start() -> Self
    where
        Self: Sized;
}
#[rustfmt::skip]
pub trait Lapping {
    fn lap(&self) -> i64;
}
#[rustfmt::skip]
const _: () = {
    fn assert_clock_clocking<T: Clocking>() {}
    let _ = assert_clock_clocking::<Clock>;
};
