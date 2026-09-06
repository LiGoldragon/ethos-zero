use std::{
    fs,
    io::{Read, Write},
    os::unix::net::UnixStream,
    thread,
    time::Duration,
};

use ethos_zero_nexus::{Paths, run};
use meta_signal_ethos_zero as meta;
use signal_ethos_zero as ordinary;

fn exchange() -> signal_frame::ExchangeIdentifier {
    signal_frame::ExchangeIdentifier::new(
        signal_frame::SessionEpoch::new(0),
        signal_frame::ExchangeLane::Connector,
        signal_frame::LaneSequence::new(7),
    )
}
fn read_frame(stream: &mut UnixStream) -> std::io::Result<Vec<u8>> {
    let mut prefix = [0; 4];
    stream.read_exact(&mut prefix)?;
    let mut frame = prefix.to_vec();
    frame.resize(4 + u32::from_be_bytes(prefix) as usize, 0);
    stream.read_exact(&mut frame[4..])?;
    Ok(frame)
}
fn server() -> (tempfile::TempDir, Paths) {
    let directory = tempfile::tempdir().expect("temporary runtime root");
    let root = directory.path();
    let paths = Paths {
        state_path: root.join("state.sema"),
        ordinary_socket: root.join("ordinary.sock"),
        meta_socket: root.join("meta.sock"),
        source_manifest: root.join("sources.datom"),
    };
    fs::write(&paths.source_manifest, "{ [] }").expect("typed empty source manifest writes");
    let serving = paths.clone();
    thread::spawn(move || run(serving).expect("Nexus starts"));
    for _ in 0..100 {
        if paths.ordinary_socket.exists() && paths.meta_socket.exists() {
            return (directory, paths);
        }
        thread::sleep(Duration::from_millis(5));
    }
    panic!("Nexus sockets were not created");
}

#[test]
fn ordinary_exchange_round_trip_and_foreign_contract_rejection() {
    let (_directory, paths) = server();
    let request = ordinary::Request::Observe(ordinary::ObservationSelection::Assemblies);
    let mut ordinary_socket =
        UnixStream::connect(&paths.ordinary_socket).expect("ordinary socket connects");
    ordinary_socket
        .set_read_timeout(Some(Duration::from_secs(2)))
        .expect("ordinary read timeout");
    ordinary_socket
        .write_all(&ordinary::encode_request(exchange(), request).expect("ordinary request binds"))
        .expect("ordinary request writes");
    let (returned, response) =
        ordinary::decode_response(&read_frame(&mut ordinary_socket).expect("ordinary reply reads"))
            .expect("ordinary reply validates");
    assert_eq!(returned, exchange());
    assert!(matches!(response, ordinary::Response::Observed(_)));

    let foreign = meta::encode_request(
        exchange(),
        meta::Request::Observe(meta::MetaObservationSelection::Configuration),
    )
    .expect("foreign meta request binds");
    let mut wrong_socket = UnixStream::connect(&paths.ordinary_socket)
        .expect("ordinary socket connects for foreign frame");
    wrong_socket
        .set_read_timeout(Some(Duration::from_secs(2)))
        .expect("foreign read timeout");
    wrong_socket
        .write_all(&foreign)
        .expect("foreign bytes write");
    assert!(
        read_frame(&mut wrong_socket).is_err(),
        "foreign contract must be rejected before a reply is decoded"
    );
}
