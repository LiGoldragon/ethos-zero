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
pub struct LaunchRequest {
    pub string: ::std::string::String,
}
#[rustfmt::skip]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
    feature = "datom",
    derive(::datom_codec::Datomizable, ::datom_codec::Composing)
)]
pub struct FlowId(pub i64);
#[rustfmt::skip]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "datom",
    derive(::datom_codec::Datomizable, ::datom_codec::Composing)
)]
pub enum Selection {
    All,
    Mine,
}
#[rustfmt::skip]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "datom",
    derive(::datom_codec::Datomizable, ::datom_codec::Composing)
)]
pub struct Pane {
    pub string: ::std::string::String,
    pub integer: i64,
}
#[rustfmt::skip]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "datom",
    derive(::datom_codec::Datomizable, ::datom_codec::Composing)
)]
pub enum Changed {
    Opened(Pane),
    Closed(i64),
}
#[rustfmt::skip]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "datom",
    derive(::datom_codec::Datomizable, ::datom_codec::Composing)
)]
pub enum Input {
    Launch(LaunchRequest),
    Watch(Selection),
    Subscribe,
}
#[rustfmt::skip]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "datom",
    derive(::datom_codec::Datomizable, ::datom_codec::Composing)
)]
pub enum Output {
    Launched(FlowId),
    Watched(::signal::Frame<::std::vec::Vec<Pane>, Changed>),
    Subscribed(::signal::Frame<FlowId, Changed>),
    Refused(::std::string::String),
}
#[rustfmt::skip]
#[allow(clippy::large_enum_variant)]
pub enum Answer {
    Once(Output),
    Watched(::signal::Stream<::std::vec::Vec<Pane>, Changed>),
    Subscribed(::signal::Stream<FlowId, Changed>),
}
#[rustfmt::skip]
#[allow(clippy::result_large_err, clippy::ptr_arg, clippy::borrowed_box)]
pub trait Answering {
    fn launch(
        &self,
        launch_request: &LaunchRequest,
    ) -> ::core::result::Result<FlowId, Output>;
    fn watch(
        &self,
        selection: &Selection,
    ) -> ::core::result::Result<
        ::signal::Stream<::std::vec::Vec<Pane>, Changed>,
        Output,
    >;
    fn subscribe(
        &self,
    ) -> ::core::result::Result<::signal::Stream<FlowId, Changed>, Output>;
}
#[rustfmt::skip]
pub trait Answerable {
    type Answer;
    fn answer<Nexus: Answering>(&self, nexus: &Nexus) -> Self::Answer;
}
#[rustfmt::skip]
impl Answerable for Input {
    type Answer = Answer;
    fn answer<Nexus: Answering>(&self, nexus: &Nexus) -> Answer {
        match self {
            Input::Launch(carried) => {
                Answer::Once(
                    nexus
                        .launch(carried)
                        .map_or_else(|refusal| refusal, Output::Launched),
                )
            }
            Input::Watch(carried) => {
                nexus.watch(carried).map_or_else(Answer::Once, Answer::Watched)
            }
            Input::Subscribe => {
                nexus.subscribe().map_or_else(Answer::Once, Answer::Subscribed)
            }
        }
    }
}
