#![allow(dead_code, non_camel_case_types, non_snake_case)]
pub type LockId = i64;
pub type LockName = String;
pub type FlowId = String;
pub type LockPath = String;
pub type LockPaths = std::vec::Vec<LockPath>;
pub type LockReason = String;
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct LockRequest {
    pub lock_name: LockName,
    pub flow_id: FlowId,
    pub lock_paths: LockPaths,
    pub lock_reason: LockReason,
}
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct Lock {
    pub lock_id: LockId,
    pub lock_name: LockName,
    pub flow_id: FlowId,
    pub lock_paths: LockPaths,
    pub lock_reason: LockReason,
}
pub type DuplicateName = Lock;
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct LockOverlap {
    pub lock_path: LockPath,
    pub lock: Lock,
}
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum LockRejection {
    DuplicateName(Lock),
    PathOverlap(LockOverlap),
}
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum ReleaseRejection {
    UnknownLockId,
}
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum ObserveSelection {
    Locks(Locks),
}
pub type Locks = std::vec::Vec<Lock>;
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum Observation {
    Locks(Locks),
}
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum Query {
    Lock(LockRequest),
    Release(LockId),
    Observe(ObserveSelection),
}
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum Response {
    Locked(Lock),
    LockRejected(LockRejection),
    Released(Lock),
    ReleaseRejected(ReleaseRejection),
    Observed(Observation),
}
