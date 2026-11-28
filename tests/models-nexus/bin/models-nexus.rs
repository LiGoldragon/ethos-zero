//! The Models Nexus: one process on the Unix socket at its one argument,
//! speaking signal only, its Memory empty at birth.

use std::cell::RefCell;
use std::os::unix::net::UnixListener;
use std::process::ExitCode;

use models_nexus::Serving;
use models_nexus::matrix::{Memory, Models};

fn main() -> ExitCode {
    let Some(socket) = std::env::args().nth(1) else {
        eprintln!("models-nexus takes the path of its socket");
        return ExitCode::FAILURE;
    };
    let nexus = RefCell::new(Memory {
        models: Models(Vec::new()),
    });
    match UnixListener::bind(&socket).and_then(|listener| listener.serve(&nexus)) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("models-nexus: {error}");
            ExitCode::FAILURE
        }
    }
}
