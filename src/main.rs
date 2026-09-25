//! The ethos-zero CLI, expressed through its generated Signal contract.

use std::path::Path as FilePath;
use std::process::ExitCode;

use datom_codec::{Actualizing as _, Budget, Datomizable as _, Path, Potential};
use ethos_zero::{
    Actualizing as _, File, Generating, Locating as _, Potential as EthosPotential, Validating as _,
};
use protos::{Protosizable, Textualizable};

#[rustfmt::skip]
#[path = "ethos-zero.rs"]
mod contract;

use contract::{
    Conceptual_Data as Generation_Conceptual_Data, Generation, Generation_Error, Query,
    Rejected_Data, Response, Unreadable_Data, Unwritable_Data,
};

const ETHOS: &str = include_str!("../ethos-zero.ethos");

trait Serving {
    fn serve(&mut self) -> Response;
}
trait Exiting {
    fn exit(&self) -> ExitCode;
}
trait Invoking {
    fn invoke(&self) -> ExitCode;
}
trait Texting {
    fn text(&self) -> String;
}
trait Erroring<T> {
    fn error(self) -> T;
}
/// The kind whose capability reads an ethos source file into its checked-for-structure File.
trait Sourcing {
    fn source(&self) -> Result<(EthosPotential<File>, File), Response>;
}
/// The kind whose capability turns an error in a read source into a located rejection.
trait Rejecting {
    fn reject(&self, source: &EthosPotential<File>, error: ethos_zero::Error) -> Response;
}

impl Texting for Response {
    fn text(&self) -> String {
        self.datomize(Path::new()).protosize().textualize()
    }
}

impl Erroring<Generation_Error> for ethos_zero::Error {
    fn error(self) -> Generation_Error {
        match self {
            ethos_zero::Error::Structural(error) => Generation_Error::Structural(protos::Error {
                extent: error.extent,
                problem: error.problem,
            }),
            ethos_zero::Error::Conceptual(data) => {
                Generation_Error::Conceptual(Generation_Conceptual_Data {
                    integer_vector: data.integer_vector,
                    problem: data.problem,
                })
            }
        }
    }
}

impl Sourcing for String {
    fn source(&self) -> Result<(EthosPotential<File>, File), Response> {
        let text = match std::fs::read_to_string(self) {
            Ok(text) => text,
            Err(error) => {
                return Err(Response::Unreadable(Unreadable_Data {
                    first_string: self.clone(),
                    second_string: error.to_string(),
                }));
            }
        };
        let potential = EthosPotential::<File>::from(text);
        match potential.actualize() {
            Ok(file) => Ok((potential, file)),
            Err(error) => Err(self.reject(&potential, error)),
        }
    }
}

impl Rejecting for String {
    fn reject(&self, source: &EthosPotential<File>, error: ethos_zero::Error) -> Response {
        Response::Rejected(Rejected_Data {
            string: self.clone(),
            location: source.locate(&error),
            generation__error: error.error(),
        })
    }
}

impl Serving for Generation {
    fn serve(&mut self) -> Response {
        let source = &self.first_string;
        let directory = &self.second_string;
        let (potential, file) = match source.source() {
            Ok(read) => read,
            Err(response) => return response,
        };
        let stem = FilePath::new(source)
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy();
        let target = FilePath::new(directory).join(format!("{stem}.rs"));
        let generated = match file.generate() {
            Ok(generated) => generated,
            Err(error) => return source.reject(&potential, error),
        };
        if let Err(error) = std::fs::create_dir_all(directory) {
            return Response::Unwritable(Unwritable_Data {
                first_string: directory.clone(),
                second_string: error.to_string(),
            });
        }
        match std::fs::write(&target, generated) {
            Ok(()) => Response::Generated(vec![target.to_string_lossy().into_owned()]),
            Err(error) => Response::Unwritable(Unwritable_Data {
                first_string: target.to_string_lossy().into_owned(),
                second_string: error.to_string(),
            }),
        }
    }
}

impl Serving for String {
    fn serve(&mut self) -> Response {
        let (potential, file) = match self.source() {
            Ok(read) => read,
            Err(response) => return response,
        };
        match file.validate() {
            Ok(()) => Response::Checked(self.clone()),
            Err(error) => self.reject(&potential, error),
        }
    }
}

impl Serving for Query {
    fn serve(&mut self) -> Response {
        match self {
            Self::Generate(generation) => generation.serve(),
            Self::Check(source) => source.serve(),
        }
    }
}

impl Serving for Potential<Query> {
    fn serve(&mut self) -> Response {
        let mut budget = Budget {
            remaining: 4_096,
            reader: protos::ReaderBudget { remaining: 4_096 },
            depth: 0,
            maximum_depth: 4_096,
        };
        match self.actualize(&mut budget) {
            Ok(mut request) => request.serve(),
            Err(error) => Response::Malformed(error),
        }
    }
}

impl Exiting for Response {
    fn exit(&self) -> ExitCode {
        match self {
            Self::Generated(_) | Self::Checked(_) => ExitCode::SUCCESS,
            Self::Arguments(_)
            | Self::Malformed(_)
            | Self::Unreadable(_)
            | Self::Rejected(_)
            | Self::Unwritable(_) => ExitCode::FAILURE,
        }
    }
}

impl Invoking for [String] {
    fn invoke(&self) -> ExitCode {
        let response = match self {
            [] => {
                print!("{ETHOS}");
                if !ETHOS.ends_with('\n') {
                    println!();
                }
                return ExitCode::SUCCESS;
            }
            [argument] => {
                let mut potential = Potential::<Query>::from(argument.as_str());
                potential.serve()
            }
            many => Response::Arguments(many.len() as datom_codec::Integer),
        };
        println!("{}", response.text());
        response.exit()
    }
}

fn main() -> ExitCode {
    std::env::args().skip(1).collect::<Vec<_>>().invoke()
}
