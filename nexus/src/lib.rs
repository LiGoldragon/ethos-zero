//! The durable, two-socket Ethos-zero Nexus runtime.
//!
//! Public requests and replies are current typed Datom values. Socket bytes
//! use the Signal contracts' structural rkyv projection inside the
//! signal-frame binding. Durable state deliberately stores only scalar facts;
//! it never archives generated public contract values.

use std::{
    collections::BTreeMap,
    env, fs,
    io::{Read, Write},
    os::unix::net::{UnixListener, UnixStream},
    path::{Component, Path, PathBuf},
    sync::mpsc,
    thread,
};

use ethos_zero::{File, Generating};
use meta::WireConversion as _;
use meta_signal_ethos_zero as meta;
use ordinary::WireConversion as _;
use protos::{Actualizable, Potential, Text};
use sema_engine::{
    Engine, EngineOpen, EngineRecord, FamilyName, QueryPlan, RecordKey, SchemaHash, SchemaVersion,
    TableDescriptor, TableName, TableReference,
};
use signal_ethos_zero as ordinary;
use signal_frame::{
    BoundStreamingFrame, LaneSequence, RootCode, SessionEpoch, StreamEventIdentifier,
    StreamingFrameBody, SubscriptionTokenInner, VariantCode, WireRoute,
};
use thiserror::Error;

#[rustfmt::skip]
mod ingress;

const SCHEMA_VERSION: SchemaVersion = SchemaVersion::new(2);
const CONFIGURATION_TABLE: TableName = TableName::new("ethos_zero_nexus_configuration");
const ASSEMBLIES_TABLE: TableName = TableName::new("ethos_zero_nexus_assemblies");
const CONFIGURATION_KEY: &str = "configuration";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Paths {
    pub state_path: PathBuf,
    pub ordinary_socket: PathBuf,
    pub meta_socket: PathBuf,
    pub source_manifest: PathBuf,
}
impl Paths {
    pub fn from_environment() -> Result<Self, RuntimeError> {
        let state_home = env::var_os("XDG_STATE_HOME")
            .map(PathBuf::from)
            .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".local/state")))
            .ok_or(RuntimeError::MissingHome)?;
        let state_root = state_home.join("ethos-zero-nexus");
        let runtime_root = env::var_os("XDG_RUNTIME_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|| state_root.clone());
        Ok(Self {
            state_path: state_root.join("ethos-zero-nexus.sema"),
            ordinary_socket: runtime_root.join("ethos-zero-nexus/ethos-zero.sock"),
            meta_socket: runtime_root.join("ethos-zero-nexus/meta-ethos-zero.sock"),
            source_manifest: state_root.join("sources.datom"),
        })
    }
    pub fn configuration(&self) -> Result<meta::Configuration, RuntimeError> {
        Ok(meta::Configuration(
            text(&self.ordinary_socket.display().to_string())?,
            text(&self.meta_socket.display().to_string())?,
            text(&self.source_manifest.display().to_string())?,
        ))
    }
}

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("HOME is required when XDG_STATE_HOME is unset")]
    MissingHome,
    #[error("filesystem: {0}")]
    Filesystem(#[from] std::io::Error),
    #[error("sema engine: {0}")]
    Engine(#[from] sema_engine::Error),
    #[error("a runtime string is not representable by Protos Text")]
    Text,
    #[error("the durable configuration singleton has {0} rows")]
    ConfigurationInvariant(usize),
    #[error("the actor mailbox closed")]
    ActorClosed,
}
fn text(value: &str) -> Result<Text, RuntimeError> {
    Text::try_from(value.to_owned()).map_err(|_| RuntimeError::Text)
}

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone)]
struct StoredConfiguration {
    ordinary_socket_path: String,
    meta_socket_path: String,
    source_manifest_path: String,
}
impl EngineRecord for StoredConfiguration {
    fn record_key(&self) -> RecordKey {
        RecordKey::new(CONFIGURATION_KEY)
    }
}
impl TryFrom<StoredConfiguration> for meta::Configuration {
    type Error = RuntimeError;
    fn try_from(value: StoredConfiguration) -> Result<Self, Self::Error> {
        Ok(Self(
            text(&value.ordinary_socket_path)?,
            text(&value.meta_socket_path)?,
            text(&value.source_manifest_path)?,
        ))
    }
}
impl From<&meta::Configuration> for StoredConfiguration {
    fn from(value: &meta::Configuration) -> Self {
        Self {
            ordinary_socket_path: value.0.to_string(),
            meta_socket_path: value.1.to_string(),
            source_manifest_path: value.2.to_string(),
        }
    }
}

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone)]
struct StoredAssembly {
    source_name: String,
    relative_path: String,
    artifact_path: String,
}
impl EngineRecord for StoredAssembly {
    fn record_key(&self) -> RecordKey {
        RecordKey::new(format!("{}:{}", self.source_name, self.relative_path))
    }
}
impl TryFrom<StoredAssembly> for ordinary::Generation {
    type Error = RuntimeError;
    fn try_from(value: StoredAssembly) -> Result<Self, Self::Error> {
        Ok(ordinary::Generation(
            ordinary::FileLocation(text(&value.source_name)?, text(&value.relative_path)?),
            text(&value.artifact_path)?,
        ))
    }
}
impl From<&ordinary::Generation> for StoredAssembly {
    fn from(value: &ordinary::Generation) -> Self {
        Self {
            source_name: value.0.0.to_string(),
            relative_path: value.0.1.to_string(),
            artifact_path: value.1.to_string(),
        }
    }
}

struct NexusStore {
    engine: Engine,
    configuration_table: TableReference<StoredConfiguration>,
    assemblies_table: TableReference<StoredAssembly>,
    configuration: meta::Configuration,
}
impl NexusStore {
    fn open(path: &Path, defaults: meta::Configuration) -> Result<Self, RuntimeError> {
        let parent = path.parent().expect("state path has a parent");
        fs::create_dir_all(parent)?;
        let mut engine = Engine::open(EngineOpen::new(path, SCHEMA_VERSION))?;
        let configuration_table = engine.register_table(TableDescriptor::new(
            CONFIGURATION_TABLE,
            FamilyName::new("ethos-zero-nexus-configuration"),
            SchemaHash::for_label("ethos-zero-nexus-configuration-v2"),
        ))?;
        let assemblies_table = engine.register_table(TableDescriptor::new(
            ASSEMBLIES_TABLE,
            FamilyName::new("ethos-zero-nexus-assembly-state"),
            SchemaHash::for_label("ethos-zero-nexus-assembly-state-v2"),
        ))?;
        let configuration = match engine
            .match_records(QueryPlan::all(configuration_table))?
            .records()
        {
            [] => {
                engine.commit_atomic(
                    engine
                        .begin_atomic_commit()
                        .assert(configuration_table, StoredConfiguration::from(&defaults)),
                )?;
                defaults
            }
            [stored] => stored.clone().try_into()?,
            rows => return Err(RuntimeError::ConfigurationInvariant(rows.len())),
        };
        Ok(Self {
            engine,
            configuration_table,
            assemblies_table,
            configuration,
        })
    }
    fn assemblies(&self) -> Result<Vec<ordinary::AssemblySummary>, RuntimeError> {
        let mut rows: Vec<_> = self
            .engine
            .match_records(QueryPlan::all(self.assemblies_table))?
            .records()
            .iter()
            .cloned()
            .map(ordinary::Generation::try_from)
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|generation| ordinary::AssemblySummary(generation.0, generation.1))
            .collect();
        rows.sort_by(|left, right| {
            left.0
                .0
                .as_ref()
                .cmp(right.0.0.as_ref())
                .then_with(|| left.0.1.as_ref().cmp(right.0.1.as_ref()))
        });
        Ok(rows)
    }
    fn ordinary_observation(&self) -> Result<ordinary::Observation, RuntimeError> {
        Ok(ordinary::Observation::Assemblies(
            ordinary::AssemblySnapshot(self.assemblies()?),
        ))
    }
    fn source_map(&self) -> Result<SourceMap, RuntimeError> {
        let source = fs::read_to_string(self.configuration.2.as_ref())?;
        SourceMap::from_datom(&source)
    }
    fn source_index(&self) -> Result<meta::SourceIndex, RuntimeError> {
        let sources = self
            .source_map()?
            .roots
            .into_iter()
            .map(|(name, path)| {
                Ok(meta::Source(
                    text(&name)?,
                    text(&path.display().to_string())?,
                ))
            })
            .collect::<Result<Vec<_>, RuntimeError>>()?;
        Ok(meta::SourceIndex(sources))
    }
    fn configure(&mut self, value: meta::Configuration) -> Result<meta::Response, RuntimeError> {
        if value.0.as_ref().is_empty() {
            return Ok(meta::Response::ConfigurationRejected(
                meta::ConfigurationRefusal::InvalidOrdinarySocketPath,
            ));
        }
        if value.1.as_ref().is_empty() {
            return Ok(meta::Response::ConfigurationRejected(
                meta::ConfigurationRefusal::InvalidMetaSocketPath,
            ));
        }
        if value.2.as_ref().is_empty() {
            return Ok(meta::Response::ConfigurationRejected(
                meta::ConfigurationRefusal::InvalidSourceManifestPath,
            ));
        }
        self.engine.commit_atomic(
            self.engine
                .begin_atomic_commit()
                .mutate(self.configuration_table, StoredConfiguration::from(&value)),
        )?;
        self.configuration = value.clone();
        Ok(meta::Response::Configured(value))
    }
    fn generate(
        &mut self,
        request: ordinary::GenerationRequest,
    ) -> Result<ordinary::Response, RuntimeError> {
        let ordinary::GenerationRequest(file) = request;
        let Some(relative) = contained_relative(file.1.as_ref()) else {
            return Ok(ordinary::Response::GenerationRejected(
                ordinary::GenerationRefusal::InvalidRelativePath(file.1),
            ));
        };
        let sources = match self.source_map() {
            Ok(value) => value,
            Err(_) => {
                return Ok(ordinary::Response::GenerationRejected(
                    ordinary::GenerationRefusal::ImportUnresolved(file),
                ));
            }
        };
        let Some(root) = sources.roots.get(file.0.as_ref()) else {
            return Ok(ordinary::Response::GenerationRejected(
                ordinary::GenerationRefusal::UnknownSource(file.0),
            ));
        };
        let source_path = root.join(&relative);
        let source = match fs::read_to_string(&source_path) {
            Ok(source) => source,
            Err(_) => {
                return Ok(ordinary::Response::GenerationRejected(
                    ordinary::GenerationRefusal::FileAbsent(file),
                ));
            }
        };
        let file_schema = match Potential::<File>::from(source).actualize(()) {
            Ok(file) => file,
            Err(error) => {
                return Ok(ordinary::Response::GenerationRejected(
                    ordinary::GenerationRefusal::InvalidEthos(syntax_fault(&format!("{error:?}"))?),
                ));
            }
        };
        let generated = match file_schema.generate() {
            Ok(generated) => generated,
            Err(error) => {
                return Ok(ordinary::Response::GenerationRejected(
                    ordinary::GenerationRefusal::RustProjectionRejected(projection_fault(
                        &format!("{error:?}"),
                    )?),
                ));
            }
        };
        let artifact_path = relative.with_extension("rs");
        let destination = root.join(&artifact_path);
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&destination, generated)?;
        let generation = ordinary::Generation(file, text(&artifact_path.display().to_string())?);
        let stored = StoredAssembly::from(&generation);
        let exists = !self
            .engine
            .match_records(QueryPlan::key(self.assemblies_table, stored.record_key()))?
            .records()
            .is_empty();
        let commit = if exists {
            self.engine
                .begin_atomic_commit()
                .mutate(self.assemblies_table, stored)
        } else {
            self.engine
                .begin_atomic_commit()
                .assert(self.assemblies_table, stored)
        };
        self.engine.commit_atomic(commit)?;
        Ok(ordinary::Response::Generated(generation))
    }
}

struct SourceMap {
    roots: BTreeMap<String, PathBuf>,
}
impl SourceMap {
    fn from_datom(source: &str) -> Result<Self, RuntimeError> {
        let manifest = datom_codec::Potential::<ingress::SourceManifest>::from(source)
            .actualize(
                datom_codec::IncorporationBudget::try_from(4_096)
                    .expect("a positive source-manifest budget"),
            )
            .map_err(|_| RuntimeError::Text)?;
        Ok(Self {
            roots: manifest
                .0
                .into_iter()
                .map(|ingress::SourceEntry(name, path)| {
                    (name.to_string(), PathBuf::from(path.as_ref()))
                })
                .collect(),
        })
    }
}
fn contained_relative(value: &str) -> Option<PathBuf> {
    let path = Path::new(value);
    if path.is_absolute() || path.as_os_str().is_empty() {
        return None;
    }
    path.components()
        .all(|component| matches!(component, Component::Normal(_) | Component::CurDir))
        .then(|| path.to_path_buf())
}
fn syntax_fault(reason: &str) -> Result<ordinary::SyntaxFault, RuntimeError> {
    Ok(ordinary::SyntaxFault(
        ordinary::SourceExtent(0, 0),
        text(reason)?,
    ))
}
fn projection_fault(reason: &str) -> Result<ordinary::ProjectionFault, RuntimeError> {
    Ok(ordinary::ProjectionFault(text(reason)?))
}

enum Command {
    Ordinary {
        request: ordinary::Request,
        events: Option<mpsc::Sender<(u64, ordinary::Response)>>,
        reply: mpsc::Sender<Result<ordinary::Response, RuntimeError>>,
    },
    Meta {
        request: meta::Request,
        events: Option<mpsc::Sender<(u64, meta::Response)>>,
        reply: mpsc::Sender<Result<meta::Response, RuntimeError>>,
    },
}
type OrdinarySubscription = (
    ordinary::SubscriptionRequest,
    u64,
    mpsc::Sender<(u64, ordinary::Response)>,
);
type MetaSubscription = (
    meta::MetaSubscriptionRequest,
    u64,
    mpsc::Sender<(u64, meta::Response)>,
);
#[derive(Clone)]
struct Actor {
    sender: mpsc::Sender<Command>,
}
impl Actor {
    fn start(paths: &Paths) -> Result<Self, RuntimeError> {
        let defaults = paths.configuration()?;
        let mut store = NexusStore::open(&paths.state_path, defaults)?;
        let (sender, receiver) = mpsc::channel::<Command>();
        thread::spawn(move || {
            let mut ordinary_subscribers: Vec<OrdinarySubscription> = Vec::new();
            let mut meta_subscribers: Vec<MetaSubscription> = Vec::new();
            let mut next_token = 0u64;
            while let Ok(command) = receiver.recv() {
                match command {
                    Command::Ordinary {
                        request,
                        events,
                        reply,
                    } => {
                        let result = match request {
                            ordinary::Request::Observe(_) => store
                                .ordinary_observation()
                                .map(ordinary::Response::Observed),
                            ordinary::Request::Subscribe(subscription) => {
                                if let Some(events) = events {
                                    ordinary_subscribers.push((subscription, next_token, events));
                                    next_token += 1;
                                }
                                store
                                    .ordinary_observation()
                                    .map(ordinary::Response::Observed)
                            }
                            ordinary::Request::Unsubscribe(subscription) => {
                                ordinary_subscribers
                                    .retain(|(current, _, _)| current != &subscription);
                                store
                                    .ordinary_observation()
                                    .map(ordinary::Response::Observed)
                            }
                            ordinary::Request::Generate(request) => {
                                let started =
                                    ordinary::Response::GenerationStarted(request.clone());
                                ordinary_subscribers.retain(|(_, token, sender)| {
                                    sender.send((*token, started.clone())).is_ok()
                                });
                                let result = store.generate(request);
                                if let Ok(response) = &result {
                                    let event = match response {
                                        ordinary::Response::Generated(generation) => {
                                            ordinary::Response::GenerationCompleted(
                                                generation.clone(),
                                            )
                                        }
                                        ordinary::Response::GenerationRejected(refusal) => {
                                            ordinary::Response::GenerationRefused(refusal.clone())
                                        }
                                        _ => unreachable!("Generate response"),
                                    };
                                    ordinary_subscribers.retain(|(_, token, sender)| {
                                        sender.send((*token, event.clone())).is_ok()
                                    });
                                }
                                result
                            }
                        };
                        let _ = reply.send(result);
                    }
                    Command::Meta {
                        request,
                        events,
                        reply,
                    } => {
                        let result = match request {
                            meta::Request::Configure(configuration) => {
                                let result = store.configure(configuration);
                                if let Ok(meta::Response::Configured(configuration)) = &result {
                                    let event =
                                        meta::Response::ConfigurationChanged(configuration.clone());
                                    meta_subscribers.retain(|(subscription, token, sender)| {
                                        !matches!(
                                            subscription.0,
                                            meta::MetaObservationSelection::Configuration
                                        ) || sender.send((*token, event.clone())).is_ok()
                                    });
                                    if let Ok(index) = store.source_index() {
                                        let event = meta::Response::SourcesChanged(index);
                                        meta_subscribers.retain(|(subscription, token, sender)| {
                                            !matches!(
                                                subscription.0,
                                                meta::MetaObservationSelection::Sources
                                            ) || sender.send((*token, event.clone())).is_ok()
                                        });
                                    }
                                }
                                result
                            }
                            meta::Request::Observe(selection) => match selection {
                                meta::MetaObservationSelection::Configuration => Ok(
                                    meta::Response::Observed(meta::MetaObservation::Configuration(
                                        store.configuration.clone(),
                                    )),
                                ),
                                meta::MetaObservationSelection::Sources => {
                                    match store.source_index() {
                                        Ok(index) => Ok(meta::Response::Observed(
                                            meta::MetaObservation::Sources(index),
                                        )),
                                        Err(_) => Ok(meta::Response::ConfigurationRejected(
                                            meta::ConfigurationRefusal::UnreadableSourceManifest,
                                        )),
                                    }
                                }
                            },
                            meta::Request::Subscribe(subscription) => {
                                let selection = subscription.0;
                                if let Some(events) = events {
                                    meta_subscribers.push((subscription, next_token, events));
                                    next_token += 1;
                                }
                                match selection {
                                    meta::MetaObservationSelection::Configuration => {
                                        Ok(meta::Response::Observed(
                                            meta::MetaObservation::Configuration(
                                                store.configuration.clone(),
                                            ),
                                        ))
                                    }
                                    meta::MetaObservationSelection::Sources => match store
                                        .source_index()
                                    {
                                        Ok(index) => Ok(meta::Response::Observed(
                                            meta::MetaObservation::Sources(index),
                                        )),
                                        Err(_) => Ok(meta::Response::ConfigurationRejected(
                                            meta::ConfigurationRefusal::UnreadableSourceManifest,
                                        )),
                                    },
                                }
                            }
                            meta::Request::Unsubscribe(subscription) => {
                                meta_subscribers.retain(|(current, _, _)| current != &subscription);
                                Ok(meta::Response::Observed(
                                    meta::MetaObservation::Configuration(
                                        store.configuration.clone(),
                                    ),
                                ))
                            }
                        };
                        let _ = reply.send(result);
                    }
                }
            }
        });
        Ok(Self { sender })
    }
    fn ordinary(
        &self,
        request: ordinary::Request,
        events: Option<mpsc::Sender<(u64, ordinary::Response)>>,
    ) -> Result<ordinary::Response, RuntimeError> {
        let (reply, receive) = mpsc::channel();
        self.sender
            .send(Command::Ordinary {
                request,
                events,
                reply,
            })
            .map_err(|_| RuntimeError::ActorClosed)?;
        receive.recv().map_err(|_| RuntimeError::ActorClosed)?
    }
    fn meta(
        &self,
        request: meta::Request,
        events: Option<mpsc::Sender<(u64, meta::Response)>>,
    ) -> Result<meta::Response, RuntimeError> {
        let (reply, receive) = mpsc::channel();
        self.sender
            .send(Command::Meta {
                request,
                events,
                reply,
            })
            .map_err(|_| RuntimeError::ActorClosed)?;
        receive.recv().map_err(|_| RuntimeError::ActorClosed)?
    }
}

fn read_frame(stream: &mut UnixStream) -> std::io::Result<Vec<u8>> {
    let mut prefix = [0; 4];
    stream.read_exact(&mut prefix)?;
    let mut bytes = prefix.to_vec();
    bytes.resize(4 + u32::from_be_bytes(prefix) as usize, 0);
    stream.read_exact(&mut bytes[4..])?;
    Ok(bytes)
}
fn bind(path: &Path) -> Result<UnixListener, RuntimeError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(UnixListener::bind(path)?)
}
fn ordinary_response_route(response: &ordinary::Response) -> WireRoute {
    let variant = match response {
        ordinary::Response::Generated(_) => 0,
        ordinary::Response::Observed(_) => 1,
        ordinary::Response::GenerationRejected(_) => 2,
        ordinary::Response::GenerationStarted(_) => 3,
        ordinary::Response::GenerationCompleted(_) => 4,
        ordinary::Response::GenerationRefused(_) => 5,
    };
    WireRoute::new(RootCode::new(1), VariantCode::new(variant))
}
fn meta_response_route(response: &meta::Response) -> WireRoute {
    let variant = match response {
        meta::Response::Configured(_) => 0,
        meta::Response::Observed(_) => 1,
        meta::Response::ConfigurationRejected(_) => 2,
        meta::Response::ConfigurationChanged(_) => 3,
        meta::Response::SourcesChanged(_) => 4,
    };
    WireRoute::new(RootCode::new(1), VariantCode::new(variant))
}
type OrdinaryFrame = BoundStreamingFrame<
    ordinary::EthosZeroWire,
    ordinary::RequestWire,
    ordinary::ResponseWire,
    ordinary::ResponseWire,
>;
type MetaFrame = BoundStreamingFrame<
    meta::MetaEthosZeroWire,
    meta::RequestWire,
    meta::ResponseWire,
    meta::ResponseWire,
>;
fn ordinary_socket(mut stream: UnixStream, actor: Actor) {
    let Ok(bytes) = read_frame(&mut stream) else {
        return;
    };
    let Ok((exchange, request)) = ordinary::decode_request(&bytes) else {
        return;
    };
    let subscribe = matches!(request, ordinary::Request::Subscribe(_));
    let (events, receiver) = mpsc::channel();
    let Ok(response) = actor.ordinary(request, subscribe.then_some(events)) else {
        return;
    };
    let Ok(reply) = ordinary::encode_response(exchange, response) else {
        return;
    };
    if stream.write_all(&reply).is_err() {
        return;
    }
    if subscribe {
        for (sequence, (token, event)) in receiver.into_iter().enumerate() {
            let frame = OrdinaryFrame::new(
                ordinary_response_route(&event),
                StreamingFrameBody::SubscriptionEvent {
                    event_identifier: StreamEventIdentifier::acceptor(
                        SessionEpoch::new(0),
                        LaneSequence::new(sequence as u64),
                    ),
                    token: SubscriptionTokenInner::new(token),
                    event: event.into_wire(),
                },
            );
            if stream
                .write_all(&frame.encode_length_prefixed().unwrap_or_default())
                .is_err()
            {
                break;
            }
        }
    }
}
fn meta_socket(mut stream: UnixStream, actor: Actor) {
    let Ok(bytes) = read_frame(&mut stream) else {
        return;
    };
    let Ok((exchange, request)) = meta::decode_request(&bytes) else {
        return;
    };
    let subscribe = matches!(request, meta::Request::Subscribe(_));
    let (events, receiver) = mpsc::channel();
    let Ok(response) = actor.meta(request, subscribe.then_some(events)) else {
        return;
    };
    let Ok(reply) = meta::encode_response(exchange, response) else {
        return;
    };
    if stream.write_all(&reply).is_err() {
        return;
    }
    if subscribe {
        for (sequence, (token, event)) in receiver.into_iter().enumerate() {
            let frame = MetaFrame::new(
                meta_response_route(&event),
                StreamingFrameBody::SubscriptionEvent {
                    event_identifier: StreamEventIdentifier::acceptor(
                        SessionEpoch::new(0),
                        LaneSequence::new(sequence as u64),
                    ),
                    token: SubscriptionTokenInner::new(token),
                    event: event.into_wire(),
                },
            );
            if stream
                .write_all(&frame.encode_length_prefixed().unwrap_or_default())
                .is_err()
            {
                break;
            }
        }
    }
}
pub fn run(paths: Paths) -> Result<(), RuntimeError> {
    let actor = Actor::start(&paths)?;
    let ordinary_listener = bind(&paths.ordinary_socket)?;
    let meta_listener = bind(&paths.meta_socket)?;
    let meta_actor = actor.clone();
    thread::spawn(move || {
        for stream in meta_listener.incoming().flatten() {
            let actor = meta_actor.clone();
            thread::spawn(move || meta_socket(stream, actor));
        }
    });
    for stream in ordinary_listener.incoming().flatten() {
        let actor = actor.clone();
        thread::spawn(move || ordinary_socket(stream, actor));
    }
    Ok(())
}
pub fn run_default() -> Result<(), RuntimeError> {
    run(Paths::from_environment()?)
}
