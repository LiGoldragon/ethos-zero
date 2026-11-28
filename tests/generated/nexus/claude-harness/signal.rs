#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    unused_extern_crates,
    clippy::type_complexity
)]
#[rustfmt::skip]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "datom",
    derive(::datom_codec::Datomizable, ::datom_codec::Composing)
)]
pub enum Source {
    Startup,
    Resume(::core::option::Option<::harness::Tokens>),
    Compact,
}
#[rustfmt::skip]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "datom",
    derive(::datom_codec::Datomizable, ::datom_codec::Composing)
)]
pub struct SessionStart_Data {
    pub session_id: ::harness::SessionId,
    pub file_path: ::harness::FilePath,
    pub source: Source,
}
#[rustfmt::skip]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "datom",
    derive(::datom_codec::Datomizable, ::datom_codec::Composing)
)]
pub struct Stop_Data {
    pub session_id: ::harness::SessionId,
    pub response_option: ::core::option::Option<::harness::Response>,
    pub background_work_vector: ::std::vec::Vec<::harness::BackgroundWork>,
}
#[rustfmt::skip]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "datom",
    derive(::datom_codec::Datomizable, ::datom_codec::Composing)
)]
pub struct SessionEnd_Data {
    pub session_id: ::harness::SessionId,
    pub end_reason: ::harness::EndReason,
}
#[rustfmt::skip]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "datom",
    derive(::datom_codec::Datomizable, ::datom_codec::Composing)
)]
pub enum Hook {
    SessionStart(SessionStart_Data),
    UserPromptSubmit(::harness::SessionId),
    PermissionRequest(::harness::SessionId),
    PostToolUse(::harness::SessionId),
    Stop(Stop_Data),
    SessionEnd(SessionEnd_Data),
}
#[rustfmt::skip]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "datom",
    derive(::datom_codec::Datomizable, ::datom_codec::Composing)
)]
pub enum Input {
    Report(Hook),
    Listening,
}
#[rustfmt::skip]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "datom",
    derive(::datom_codec::Datomizable, ::datom_codec::Composing)
)]
pub enum Output {
    Reported,
    Acknowledged,
}
#[rustfmt::skip]
#[allow(clippy::result_large_err, clippy::ptr_arg, clippy::borrowed_box)]
pub trait Answering {
    fn report(&self, hook: &Hook) -> ::core::result::Result<(), Output>;
    fn listening(&self) -> ::core::result::Result<(), Output>;
}
#[rustfmt::skip]
pub trait Answerable {
    type Answer;
    fn answer<Nexus: Answering>(&self, nexus: &Nexus) -> Self::Answer;
}
#[rustfmt::skip]
impl Answerable for Input {
    type Answer = Output;
    fn answer<Nexus: Answering>(&self, nexus: &Nexus) -> Output {
        match self {
            Input::Report(carried) => {
                nexus
                    .report(carried)
                    .map_or_else(|refusal| refusal, |()| Output::Reported)
            }
            Input::Listening => {
                nexus
                    .listening()
                    .map_or_else(|refusal| refusal, |()| Output::Acknowledged)
            }
        }
    }
}
