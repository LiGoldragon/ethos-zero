//! Actualization: textual Ethos to its one conceptual File.

use protos::Protosizable;

use crate::{
    Actualizing, Canonicalizable, Error, Ethosizable, File, Potential, Resituating,
    Structural_Error,
};

impl Actualizing<File> for Potential<File> {
    type Error = Error;

    fn actualize(&self) -> Result<File, Self::Error> {
        let canonical = self.0.canonicalize().map_err(|error| {
            Error::Structural(Structural_Error {
                extent: error.extent,
                problem: error.problem,
            })
        })?;
        let protos = canonical.text.protosize().map_err(|error| {
            Error::Structural(Structural_Error {
                extent: canonical.resituate(error.extent),
                problem: error.problem,
            })
        })?;
        protos.ethosize()
    }
}
