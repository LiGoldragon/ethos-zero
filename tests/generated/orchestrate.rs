#![allow(dead_code, non_camel_case_types, non_snake_case)]
#![allow(clippy::type_complexity)]
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct LockId(pub i64);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct LockName(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct FlowId(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct LockPath(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct LockPaths(pub std::vec::Vec<LockPath>);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct LockReason(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct LockRequest {
    pub lock_name: LockName,
    pub flow_id: FlowId,
    pub lock_paths: LockPaths,
    pub lock_reason: LockReason,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Lock {
    pub lock_id: LockId,
    pub lock_name: LockName,
    pub flow_id: FlowId,
    pub lock_paths: LockPaths,
    pub lock_reason: LockReason,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct DuplicateName(pub Lock);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct LockOverlap {
    pub lock_path: LockPath,
    pub lock: Lock,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum LockRejection {
    DuplicateName(Lock),
    PathOverlap(LockOverlap),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ReleaseRejection {
    UnknownLockId,
    NotHolder,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ObserveSelection {
    Locks(Locks),
    Held(FlowId),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Locks(pub std::vec::Vec<Lock>);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Observation {
    Locks(Locks),
    Held(Locks),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Query {
    Lock(LockRequest),
    Release(LockId),
    Observe(ObserveSelection),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Response {
    Locked(Lock),
    LockRejected(LockRejection),
    Released(Lock),
    ReleaseRejected(ReleaseRejection),
    Observed(Observation),
}
#[rustfmt::skip]
const _: () = {
    fn assert_lockid_sized<T: Sized>() {}
    let _ = assert_lockid_sized::<LockId>;
};
#[rustfmt::skip]
const _: () = {
    fn assert_lockname_sized<T: Sized>() {}
    let _ = assert_lockname_sized::<LockName>;
};
#[rustfmt::skip]
const _: () = {
    fn assert_flowid_sized<T: Sized>() {}
    let _ = assert_flowid_sized::<FlowId>;
};
#[rustfmt::skip]
const _: () = {
    fn assert_lockpath_sized<T: Sized>() {}
    let _ = assert_lockpath_sized::<LockPath>;
};
#[rustfmt::skip]
const _: () = {
    fn assert_lockpaths_sized<T: Sized>() {}
    let _ = assert_lockpaths_sized::<LockPaths>;
};
#[rustfmt::skip]
const _: () = {
    fn assert_lockreason_sized<T: Sized>() {}
    let _ = assert_lockreason_sized::<LockReason>;
};
#[rustfmt::skip]
const _: () = {
    fn assert_lockrequest_sized<T: Sized>() {}
    let _ = assert_lockrequest_sized::<LockRequest>;
};
#[rustfmt::skip]
const _: () = {
    fn assert_lock_sized<T: Sized>() {}
    let _ = assert_lock_sized::<Lock>;
};
#[rustfmt::skip]
const _: () = {
    fn assert_duplicatename_sized<T: Sized>() {}
    let _ = assert_duplicatename_sized::<DuplicateName>;
};
#[rustfmt::skip]
const _: () = {
    fn assert_lockoverlap_sized<T: Sized>() {}
    let _ = assert_lockoverlap_sized::<LockOverlap>;
};
#[rustfmt::skip]
const _: () = {
    fn assert_lockrejection_sized<T: Sized>() {}
    let _ = assert_lockrejection_sized::<LockRejection>;
};
#[rustfmt::skip]
const _: () = {
    fn assert_releaserejection_sized<T: Sized>() {}
    let _ = assert_releaserejection_sized::<ReleaseRejection>;
};
#[rustfmt::skip]
const _: () = {
    fn assert_observeselection_sized<T: Sized>() {}
    let _ = assert_observeselection_sized::<ObserveSelection>;
};
#[rustfmt::skip]
const _: () = {
    fn assert_locks_sized<T: Sized>() {}
    let _ = assert_locks_sized::<Locks>;
};
#[rustfmt::skip]
const _: () = {
    fn assert_observation_sized<T: Sized>() {}
    let _ = assert_observation_sized::<Observation>;
};
#[rustfmt::skip]
const _: () = {
    fn assert_query_sized<T: Sized>() {}
    let _ = assert_query_sized::<Query>;
};
#[rustfmt::skip]
const _: () = {
    fn assert_response_sized<T: Sized>() {}
    let _ = assert_response_sized::<Response>;
};
