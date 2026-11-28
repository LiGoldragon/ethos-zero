//! The ethos-zero CLI, expressed through its generated Signal contract.

use std::path::Path as SystemPath;
use std::process::ExitCode;

use datom_codec::{Actualizing as _, Budget, Datomizable as _, Path, Potential};
use ethos_zero::{
    Actualizing as _, Assembling as _, File, Locating as _, Module, Namespace as GivenNamespace,
    Nexus, Potential as EthosPotential, Printable as _, Refusal, Trusting as _, Using as _,
    Verifying as _,
};
use protos::{Compactable, Protosizable};

#[rustfmt::skip]
#[path = "ethos-zero.rs"]
mod contract;

use contract::{
    Checked_Data, Conceptual_Data as Generation_Conceptual_Data, Directory, FilePath, Finding,
    Generated_Data, Generation, Generation_Error, Name, Namespace, Query, Reading, Rejected_Data,
    Response, Target, Unchecked, Unreadable_Data, Unused_Data, Unwritable_Data,
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
/// An ethos source file read for structure: its text, kept to situate an error in it.
type Read = (EthosPotential<File>, File);
/// The trait whose capability reads an ethos source file into its checked-for-structure File.
trait Sourcing {
    fn source(&self) -> Result<Read, Response>;
}
/// The trait whose capability reads ethos files, each a module named for its file's stem.
trait Gathering {
    fn gather(&self) -> Result<(Vec<Module>, Vec<EthosPotential<File>>), Response>;
}
/// The trait whose capability reads the root files of a Nexus together, and the
/// files of each namespace given: a file of either that does not read is the
/// rejection, naming it.
trait Opening {
    fn open(&self) -> Result<(Nexus, Vec<EthosPotential<File>>), Response>;
}
/// The trait whose capability turns an error in a read source into a located rejection.
trait Rejecting {
    fn reject(&self, source: &EthosPotential<File>, error: ethos_zero::Error) -> Response;
}
/// The trait whose capability lists the findings of the roots read: what they
/// import that nothing given checks, and the open enums nothing carries, each
/// by its file: the one list Check and Generate both answer.
trait Listing {
    fn list(&self, nexus: &Nexus) -> Vec<Finding>;
}
/// The trait whose capability turns a Nexus's refusal into a rejection of the file it lies in.
trait Refusing {
    fn refuse(&self, sources: &[FilePath], texts: &[EthosPotential<File>]) -> Response;
}

impl Texting for Response {
    /// A reply is one line, so a caller matching it line by line still can.
    fn text(&self) -> String {
        self.datomize(Path::new()).protosize().compact()
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

impl Sourcing for FilePath {
    fn source(&self) -> Result<(EthosPotential<File>, File), Response> {
        let text = match std::fs::read_to_string(&self.0) {
            Ok(text) => text,
            Err(error) => {
                return Err(Response::Unreadable(Unreadable_Data {
                    file_path: self.clone(),
                    string: error.to_string(),
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

impl Gathering for [FilePath] {
    fn gather(&self) -> Result<(Vec<Module>, Vec<EthosPotential<File>>), Response> {
        let mut modules = Vec::with_capacity(self.len());
        let mut texts = Vec::with_capacity(self.len());
        for source in self {
            let (potential, file) = source.source()?;
            let stem = SystemPath::new(&source.0)
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned();
            modules.push(Module { stem, file });
            texts.push(potential);
        }
        Ok((modules, texts))
    }
}

impl Opening for Reading {
    fn open(&self) -> Result<(Nexus, Vec<EthosPotential<File>>), Response> {
        let (modules, texts) = self.file_path_vector.gather()?;
        let mut namespaces = Vec::with_capacity(self.given_namespace_vector.len());
        for namespace in &self.given_namespace_vector {
            let (modules, _) = namespace.file_path_vector.gather()?;
            namespaces.push(GivenNamespace {
                name: namespace.namespace.0.clone(),
                modules,
            });
        }
        Ok((
            Nexus {
                modules,
                namespaces,
            },
            texts,
        ))
    }
}

impl Listing for Reading {
    // What Check and Generate find amiss and pass, file by file in the order
    // given, and within a file in the order of its text: every source it takes
    // from unchecked is first written in its imports or its inputs, outputs and
    // types, before the open enums of its last section.
    fn list(&self, nexus: &Nexus) -> Vec<Finding> {
        let unchecked = nexus.unchecked();
        let unused = nexus.unused();
        let mut findings = Vec::with_capacity(unchecked.len() + unused.len());
        for (module, source) in self.file_path_vector.iter().enumerate() {
            for trusted in &unchecked {
                if trusted.module != module {
                    continue;
                }
                let mut names = Vec::with_capacity(trusted.names.len());
                for name in &trusted.names {
                    names.push(Name(name.as_ref().to_owned()));
                }
                findings.push(Finding::Unchecked(Unchecked {
                    file_path: source.clone(),
                    namespace: Namespace(trusted.namespace.clone()),
                    name_vector: names,
                }));
            }
            for open in &unused {
                if open.module == module {
                    findings.push(Finding::Unused(Unused_Data {
                        file_path: source.clone(),
                        name: Name(open.name.as_ref().to_owned()),
                    }));
                }
            }
        }
        findings
    }
}

impl Refusing for Refusal {
    fn refuse(&self, sources: &[FilePath], texts: &[EthosPotential<File>]) -> Response {
        sources[self.module].reject(&texts[self.module], self.error.clone())
    }
}

impl Rejecting for FilePath {
    fn reject(&self, source: &EthosPotential<File>, error: ethos_zero::Error) -> Response {
        Response::Rejected(Rejected_Data {
            file_path: self.clone(),
            location: source.locate(&error),
            generation__error: error.error(),
        })
    }
}

impl Serving for Generation {
    fn serve(&mut self) -> Response {
        let sources = &self.reading.file_path_vector;
        let Directory(directory) = &self.directory;
        let (nexus, texts) = match self.reading.open() {
            Ok(read) => read,
            Err(response) => return response,
        };
        let assembled = match nexus.assemble() {
            Ok(assembled) => assembled,
            Err(refusal) => return refusal.refuse(sources, &texts),
        };
        if let Err(error) = std::fs::create_dir_all(directory) {
            return Response::Unwritable(Unwritable_Data {
                target: Target::Directory(Directory(directory.clone())),
                string: error.to_string(),
            });
        }
        let mut written = Vec::with_capacity(assembled.len());
        for module in assembled {
            let target = SystemPath::new(directory).join(format!("{}.rs", module.stem));
            if let Err(error) = std::fs::write(&target, module.rust) {
                return Response::Unwritable(Unwritable_Data {
                    target: Target::FilePath(FilePath(target.to_string_lossy().into_owned())),
                    string: error.to_string(),
                });
            }
            written.push(FilePath(target.to_string_lossy().into_owned()));
        }
        Response::Generated(Generated_Data {
            file_path_vector: written,
            finding_vector: self.reading.list(&nexus),
        })
    }
}

impl Serving for Reading {
    fn serve(&mut self) -> Response {
        let (nexus, texts) = match self.open() {
            Ok(read) => read,
            Err(response) => return response,
        };
        if let Err(refusal) = nexus.verify() {
            return refusal.refuse(&self.file_path_vector, &texts);
        }
        Response::Checked(Checked_Data {
            file_path_vector: self.file_path_vector.clone(),
            finding_vector: self.list(&nexus),
        })
    }
}

impl Serving for Query {
    fn serve(&mut self) -> Response {
        match self {
            Self::Generate(generation) => generation.serve(),
            Self::Check(reading) => reading.serve(),
        }
    }
}

impl Serving for Potential<Query> {
    fn serve(&mut self) -> Response {
        let mut budget = Budget {
            remaining: 4_096,
            reader: protos::ReaderBudget::try_from(4_096).unwrap(),
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
                // The crate's own ethos, in the canonical print.
                return match EthosPotential::<File>::from(ETHOS).actualize() {
                    Ok(file) => {
                        print!("{}", file.print());
                        ExitCode::SUCCESS
                    }
                    Err(_) => ExitCode::FAILURE,
                };
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
