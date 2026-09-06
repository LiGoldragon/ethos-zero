//! Typed Datom edge client for the privileged Ethos-zero Nexus socket.

use datom_codec::{Actualizable, IncorporationBudget, Potential, Textualizable};
use meta_signal_ethos_zero as signal;
use std::{
    env,
    io::{Read, Write},
    os::unix::net::UnixStream,
    path::PathBuf,
    process::ExitCode,
};
fn socket_path() -> Result<PathBuf, String> {
    env::var_os("XDG_RUNTIME_DIR")
        .map(PathBuf::from)
        .or_else(|| env::var_os("XDG_STATE_HOME").map(PathBuf::from))
        .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".local/state")))
        .map(|root| root.join("ethos-zero-nexus/meta-ethos-zero.sock"))
        .ok_or_else(|| "XDG_RUNTIME_DIR, XDG_STATE_HOME, or HOME is required".to_owned())
}
fn parse_arguments(values: &[String]) -> Result<String, String> {
    match values {
        [value] if !value.starts_with('-') => Ok(value.clone()),
        _ => Err("accepts exactly one Datom object and no flags".into()),
    }
}
fn read_frame(stream: &mut UnixStream) -> Result<Vec<u8>, String> {
    let mut prefix = [0; 4];
    stream.read_exact(&mut prefix).map_err(|e| e.to_string())?;
    let mut frame = prefix.to_vec();
    frame.resize(4 + u32::from_be_bytes(prefix) as usize, 0);
    stream
        .read_exact(&mut frame[4..])
        .map_err(|e| e.to_string())?;
    Ok(frame)
}
fn run() -> Result<(), String> {
    let request = Potential::<signal::Request>::from(parse_arguments(
        &env::args().skip(1).collect::<Vec<_>>(),
    )?)
    .actualize(IncorporationBudget::try_from(1024).unwrap())
    .map_err(|e| format!("Datom request: {e:?}"))?;
    let exchange = signal_frame::ExchangeIdentifier::new(
        signal_frame::SessionEpoch::new(0),
        signal_frame::ExchangeLane::Connector,
        signal_frame::LaneSequence::new(0),
    );
    let mut stream = UnixStream::connect(socket_path()?).map_err(|e| e.to_string())?;
    stream
        .write_all(&signal::encode_request(exchange, request).map_err(|e| format!("{e:?}"))?)
        .map_err(|e| e.to_string())?;
    let (returned, response) =
        signal::decode_response(&read_frame(&mut stream)?).map_err(|e| format!("{e:?}"))?;
    if returned != exchange {
        return Err("Nexus returned a reply for another exchange".into());
    }
    println!("{}", response.textualize());
    Ok(())
}
fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("ethos-zero-meta: {error}");
            ExitCode::FAILURE
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn accepts_exactly_one_non_flag_datom() {
        assert!(parse_arguments(&["Observe.Configuration".into()]).is_ok());
        assert!(parse_arguments(&[]).is_err());
        assert!(parse_arguments(&["--help".into()]).is_err());
        assert!(parse_arguments(&["Observe.Configuration".into(), "extra".into()]).is_err());
    }
}
