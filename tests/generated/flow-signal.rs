#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    clippy::type_complexity
)]
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Brief(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Launch_Data {
    pub voice: flow::Voice,
    pub brief: Brief,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Report_Data {
    pub flow_id: flow::FlowId,
    pub event: flow::Event,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Query {
    Launch(Launch_Data),
    Report(Report_Data),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Refused_Data {
    NoCapsule,
    VoiceBusy(flow::Voice),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Response {
    Launched(flow::FlowId),
    Reported,
    Refused(Refused_Data),
}
#[rustfmt::skip]
#[allow(clippy::result_large_err)]
pub trait Answering {
    fn launch(&self, launch: Launch_Data) -> std::result::Result<flow::FlowId, Response>;
    fn report(&self, report: Report_Data) -> std::result::Result<(), Response>;
    fn answer(&self, query: Query) -> Response {
        match query {
            Query::Launch(launch) => {
                self.launch(launch).map_or_else(|refusal| refusal, Response::Launched)
            }
            Query::Report(report) => {
                self.report(report)
                    .map_or_else(|refusal| refusal, |()| Response::Reported)
            }
        }
    }
}
