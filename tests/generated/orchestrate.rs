#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
pub type LockId = i64;
#[rustfmt::skip]
pub type LockName = String;
#[rustfmt::skip]
pub type FlowId = String;
#[rustfmt::skip]
pub type LockPath = String;
#[rustfmt::skip]
pub type LockPaths = std::vec::Vec<LockPath>;
#[rustfmt::skip]
pub type LockReason = String;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
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
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
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
#[rustfmt::skip]
pub type DuplicateName = Lock;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct LockOverlap {
    pub lock_path: LockPath,
    pub lock: Lock,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum LockRejection {
    DuplicateName(Lock),
    PathOverlap(LockOverlap),
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum ReleaseRejection {
    UnknownLockId,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum ObserveSelection {
    Locks(Locks),
}
#[rustfmt::skip]
pub type Locks = std::vec::Vec<Lock>;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum Observation {
    Locks(Locks),
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum Query {
    Lock(LockRequest),
    Release(LockId),
    Observe(ObserveSelection),
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
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
