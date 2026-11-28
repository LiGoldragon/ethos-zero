//! The Models Nexus's client: one datom, read here, asked of the Nexus as
//! signal, its answer printed as one line of text.
//!
//! PICK: the socket is named by MODELS_NEXUS_SOCKET, so the one argument is
//! the datom alone.

use std::os::unix::net::UnixStream;
use std::process::ExitCode;

use models_nexus::Calling;

fn main() -> ExitCode {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let [datom] = arguments.as_slice() else {
        eprintln!("models takes one datom");
        return ExitCode::FAILURE;
    };
    let Ok(socket) = std::env::var("MODELS_NEXUS_SOCKET") else {
        eprintln!("models: MODELS_NEXUS_SOCKET names no socket");
        return ExitCode::FAILURE;
    };
    match UnixStream::connect(&socket)
        .map_err(|error| error.to_string())
        .and_then(|mut stream| stream.call(datom))
    {
        Ok(answer) => {
            println!("{answer}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("models: {error}");
            ExitCode::FAILURE
        }
    }
}
