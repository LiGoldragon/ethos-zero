//! The Models Nexus: a model added to a running Nexus on the open enum of
//! notion:ethos:1. Every type and the Model contract are what the built tool
//! generates from fixtures/nexus/models; what is written here by hand is the
//! bodies, each a trait impl: the Registry over the Memory, the Nexus's
//! answers, the signal frame on the socket, and the client's reading of text.
//!
//! The variants of Model are data of the Nexus, held in its Memory as the
//! Matrix record `Models.⟦Variant⟧`; no model is named in this crate.

extern crate self as models;

#[path = "../generated/nexus/models/matrix.rs"]
pub mod matrix;
#[path = "../generated/nexus/models/signal.rs"]
pub mod signal;

use std::cell::RefCell;
use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};

use matrix::{Memorizable, Memory, Models};
use signal::model::{self, Registry};
use signal::{Answerable, Answering, Carried, Input, Model, Output};

/// The Registry the generated Opening reads a Model through: the variants
/// the Memory holds, found by name.
impl Registry for Memory {
    fn variant(&self, name: &model::Name) -> Option<model::Variant> {
        Models::recall(self)
            .0
            .iter()
            .find(|variant| variant.name == *name)
            .cloned()
    }
}

/// The types a variant may carry: those the Signal file declares.
///
/// PICK: written by hand. The generated code holds these names only inside
/// the body of `Opening::open`, where no body can read them, so the Nexus's
/// refusal of `Some.Name.Unknown` and its check that a carried value is of
/// the type its entry names repeat the Signal's types here.
pub trait Declaring {
    fn declares(name: &model::Name) -> bool;
    fn declared(&self) -> model::Name;
}

impl Declaring for Carried {
    fn declares(name: &model::Name) -> bool {
        matches!(name.0.as_str(), "TripleEffort" | "QuadrupleEffort")
    }

    fn declared(&self) -> model::Name {
        model::Name(String::from(match self {
            Carried::TripleEffort(_) => "TripleEffort",
            Carried::QuadrupleEffort(_) => "QuadrupleEffort",
        }))
    }
}

/// Whether a Model value is one the Memory holds: its head added, and what
/// it carries of the type its entry names. The Nexus asks this of every
/// value the signal brings, whoever read the text.
pub trait Admitting {
    fn admit(&self, model: &Model) -> Result<(), Output>;
}

impl Admitting for Memory {
    fn admit(&self, model: &Model) -> Result<(), Output> {
        let Some(entry) = self.variant(&model.name) else {
            return Err(Output::Refused(format!("{} was never added", model.name.0)));
        };
        match (&entry.type_option, &model.carried) {
            (None, None) => Ok(()),
            (Some(model::Type::Name(name)), Some(carried)) if carried.declared() == *name => Ok(()),
            _ => Err(Output::Refused(format!(
                "{} does not carry what its entry names",
                model.name.0
            ))),
        }
    }
}

/// The Nexus: its Memory, changed by its own answers alone.
impl Answering for RefCell<Memory> {
    fn launch(&self, model: &Model) -> Result<(), Output> {
        self.borrow().admit(model)
    }

    // PICK: the Signal pairs Stop with Refused.String by position; nothing
    // runs in this prototype, so a stop of an admitted model says so.
    fn stop(&self, model: &Model) -> Result<String, Output> {
        self.borrow().admit(model)?;
        Ok(format!("{} is not running", model.name.0))
    }

    fn model(&self, input: &model::Input) -> Result<model::Output, Output> {
        let mut memory = self.borrow_mut();
        Ok(match input {
            model::Input::Add(variant) => {
                if memory.variant(&variant.name).is_some() {
                    model::Output::Refused(format!("{} is already added", variant.name.0))
                } else {
                    match &variant.type_option {
                        None => {
                            memory.models.0.push(variant.clone());
                            model::Output::Added(variant.clone())
                        }
                        Some(model::Type::Name(name)) if Carried::declares(name) => {
                            memory.models.0.push(variant.clone());
                            model::Output::Added(variant.clone())
                        }
                        Some(_) => model::Output::Refused(format!(
                            "{} carries a type the Signal does not declare",
                            variant.name.0
                        )),
                    }
                }
            }
            model::Input::Remove(name) => {
                let before = memory.models.0.len();
                memory.models.0.retain(|variant| variant.name != *name);
                if memory.models.0.len() == before {
                    model::Output::Refused(format!("{} was never added", name.0))
                } else {
                    model::Output::Removed(name.clone())
                }
            }
            model::Input::List => model::Output::Listed(memory.models.0.clone()),
        })
    }
}

/// One signal frame on the socket.
///
/// PICK: no frame is generated or designed; a frame here is the length of
/// the rkyv archive as four little-endian bytes, then the archive.
pub trait Framing {
    fn send_frame(&mut self, bytes: &[u8]) -> std::io::Result<()>;
    fn receive_frame(&mut self) -> std::io::Result<Option<Vec<u8>>>;
}

impl Framing for UnixStream {
    fn send_frame(&mut self, bytes: &[u8]) -> std::io::Result<()> {
        self.write_all(&(bytes.len() as u32).to_le_bytes())?;
        self.write_all(bytes)?;
        self.flush()
    }

    fn receive_frame(&mut self) -> std::io::Result<Option<Vec<u8>>> {
        let mut length = [0u8; 4];
        match self.read_exact(&mut length) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(None),
            Err(error) => return Err(error),
        }
        let mut bytes = vec![0u8; u32::from_le_bytes(length) as usize];
        self.read_exact(&mut bytes)?;
        Ok(Some(bytes))
    }
}

/// A value as signal: its rkyv archive, and back.
pub trait Signaling: Sized {
    fn signal(&self) -> Vec<u8>;
    fn unsignal(bytes: &[u8]) -> Result<Self, String>;
}

impl Signaling for Input {
    fn signal(&self) -> Vec<u8> {
        rkyv::to_bytes::<rkyv::rancor::Error>(self)
            .expect("an input archives")
            .to_vec()
    }

    fn unsignal(bytes: &[u8]) -> Result<Self, String> {
        rkyv::from_bytes::<Self, rkyv::rancor::Error>(bytes).map_err(|error| error.to_string())
    }
}

impl Signaling for Output {
    fn signal(&self) -> Vec<u8> {
        rkyv::to_bytes::<rkyv::rancor::Error>(self)
            .expect("an output archives")
            .to_vec()
    }

    fn unsignal(bytes: &[u8]) -> Result<Self, String> {
        rkyv::from_bytes::<Self, rkyv::rancor::Error>(bytes).map_err(|error| error.to_string())
    }
}

/// The Nexus on its socket: every frame an Input, answered by an Output.
pub trait Serving {
    fn serve(&self, nexus: &RefCell<Memory>) -> std::io::Result<()>;
}

impl Serving for UnixListener {
    fn serve(&self, nexus: &RefCell<Memory>) -> std::io::Result<()> {
        for stream in self.incoming() {
            let mut stream = stream?;
            while let Some(frame) = stream.receive_frame()? {
                let output = match Input::unsignal(&frame) {
                    Ok(input) => input.answer(nexus),
                    Err(error) => Output::Refused(format!("no input: {error}")),
                };
                stream.send_frame(&output.signal())?;
            }
        }
        Ok(())
    }
}

/// One Input asked of the Nexus, its Output answered.
pub trait Asking {
    fn ask(&mut self, input: &Input) -> Result<Output, String>;
}

impl Asking for UnixStream {
    fn ask(&mut self, input: &Input) -> Result<Output, String> {
        self.send_frame(&input.signal())
            .map_err(|error| error.to_string())?;
        let frame = self
            .receive_frame()
            .map_err(|error| error.to_string())?
            .ok_or_else(|| String::from("the Nexus closed the socket"))?;
        Output::unsignal(&frame)
    }
}

/// The client's reading of one datom into an Input, through a Registry.
///
/// PICK: the generated derive composes an Input whose Model carries nothing,
/// without its registry, and refuses one that carries a type
/// (`Launch.Fable.High`); the generated Opening reads a Model alone. So the
/// Input is composed with its Model's body cut to the bare head, and the
/// Model of a Launch or a Stop is then read again, whole, by Opening.
#[cfg(feature = "datom")]
pub trait Reading {
    fn read<R: Registry>(
        &self,
        registry: &R,
        budget: &mut datom_codec::Budget,
    ) -> Result<Input, datom_codec::Error>;
}

#[cfg(feature = "datom")]
impl Reading for datom_codec::Datom {
    fn read<R: Registry>(
        &self,
        registry: &R,
        budget: &mut datom_codec::Budget,
    ) -> Result<Input, datom_codec::Error> {
        use datom_codec::{Composing, Datom, Form};
        use signal::Opening;
        let Form::Variant(head, body) = &self.form else {
            return Input::compose(self, budget);
        };
        let input = match Input::compose(self, budget) {
            Ok(input) => input,
            Err(error) => {
                let Form::Variant(name, _) = &body.form else {
                    return Err(error);
                };
                let cut = Datom {
                    path: self.path.clone(),
                    form: Form::Variant(
                        head.clone(),
                        Box::new(Datom {
                            path: body.path.clone(),
                            form: Form::Bare(name.0.clone()),
                        }),
                    ),
                };
                Input::compose(&cut, budget).map_err(|_| error)?
            }
        };
        Ok(match input {
            Input::Launch(_) => Input::Launch(Model::open(body, budget, registry)?),
            Input::Stop(_) => Input::Stop(Model::open(body, budget, registry)?),
            other => other,
        })
    }
}

/// The client: one datom of text, asked of the Nexus as signal, its answer
/// written back as text.
///
/// PICK: the client has no registry of its own; it asks the Nexus for
/// `Model.List` first and reads the text through the variants listed.
#[cfg(feature = "datom")]
pub trait Calling {
    fn call(&mut self, text: &str) -> Result<String, String>;
}

#[cfg(feature = "datom")]
impl Calling for UnixStream {
    fn call(&mut self, text: &str) -> Result<String, String> {
        use datom_codec::{DatomForming, Datomizable};
        use protos::{BoundedProtosizable, Compactable, Protosizable};
        let mut budget = datom_codec::Budget {
            remaining: 4_096,
            reader: protos::ReaderBudget::try_from(4_096).expect("a bounded budget"),
            depth: 0,
            maximum_depth: 4_096,
        };
        let listed = self.ask(&Input::Model(model::Input::List))?;
        let Output::Model(model::Output::Listed(variants)) = listed else {
            return Err(format!("the Nexus listed no models: {listed:?}"));
        };
        let registry = Memory {
            models: Models(variants),
        };
        let written = |error: datom_codec::Error| error.datomize(Vec::new()).protosize().compact();
        let output = match String::from(text).protosize_with(&mut budget.reader) {
            Err(error) => Output::Refused(format!("{error:?}")),
            Ok(protos) => match protos.datom_form(Vec::new()) {
                Err(error) => Output::Refused(written(error)),
                Ok(datom) => match datom.read(&registry, &mut budget) {
                    Err(error) => Output::Refused(written(error)),
                    Ok(input) => self.ask(&input)?,
                },
            },
        };
        Ok(output.datomize(Vec::new()).protosize().compact())
    }
}
