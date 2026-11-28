//! The beat: the Models Nexus started as its own process on a socket in a
//! temporary directory, and the client asked one datom at a time, each answer
//! asserted in order.

use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::process::{Child, Command};
use std::time::{Duration, Instant};

use models_nexus::Asking;
use models_nexus::signal::{Carried, Input, Model, Output, QuadrupleEffort, TripleEffort, model};

/// A binary of this crate: by its variable when a builder moved it, else
/// where cargo built it.
fn binary(variable: &str, built: Option<&str>) -> PathBuf {
    std::env::var(variable)
        .ok()
        .or(built.map(String::from))
        .map(PathBuf::from)
        .unwrap_or_else(|| panic!("{variable} names no binary"))
}

/// The running Nexus, killed and its directory removed when the beat ends.
struct Running {
    nexus: Child,
    directory: PathBuf,
    socket: PathBuf,
    client: PathBuf,
}

impl Drop for Running {
    fn drop(&mut self) {
        let _ = self.nexus.kill();
        let _ = self.nexus.wait();
        let _ = std::fs::remove_dir_all(&self.directory);
    }
}

fn start() -> Running {
    let directory = std::env::temp_dir().join(format!("models-nexus-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory).expect("a temporary directory");
    let socket = directory.join("nexus.sock");
    let nexus = Command::new(binary(
        "MODELS_NEXUS_BIN",
        option_env!("CARGO_BIN_EXE_models-nexus"),
    ))
    .arg(&socket)
    .spawn()
    .expect("the Nexus starts");
    let running = Running {
        nexus,
        directory,
        socket,
        client: binary("MODELS_BIN", option_env!("CARGO_BIN_EXE_models")),
    };
    let started = Instant::now();
    while UnixStream::connect(&running.socket).is_err() {
        assert!(
            started.elapsed() < Duration::from_secs(10),
            "the Nexus never listened"
        );
        std::thread::sleep(Duration::from_millis(20));
    }
    running
}

fn ask(running: &Running, datom: &str) -> String {
    let output = Command::new(&running.client)
        .arg(datom)
        .env("MODELS_NEXUS_SOCKET", &running.socket)
        .output()
        .expect("the client runs");
    assert!(
        output.status.success(),
        "{datom}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let answer = String::from_utf8(output.stdout)
        .expect("text")
        .trim_end()
        .to_owned();
    println!("{datom} -> {answer}");
    answer
}

fn refused(answer: &str) -> bool {
    answer.starts_with("Refused.")
}

#[test]
fn a_model_added_to_the_running_nexus_is_launched_by_its_name() {
    let running = start();

    // 1. A head never added is refused.
    let answer = ask(&running, "Launch.Jev");
    assert!(refused(&answer), "1: {answer}");

    // 2. A model that carries nothing is added.
    assert_eq!(
        ask(&running, "Model.Add.{ Jev None }"),
        "Model.Added.{ Jev None }",
        "2"
    );

    // 3. It is launched by its name alone.
    assert_eq!(ask(&running, "Launch.Jev"), "Launched", "3");

    // 4. A model that carries a type the file declares is added.
    assert_eq!(
        ask(&running, "Model.Add.{ Fable Some.Name.QuadrupleEffort }"),
        "Model.Added.{ Fable Some.Name.QuadrupleEffort }",
        "4"
    );

    // 5. It is launched with what it carries.
    assert_eq!(ask(&running, "Launch.Fable.High"), "Launched", "5");

    // 6. Its name alone, a value its type has not, and a value for a model
    // that carries nothing are refused.
    for datom in ["Launch.Fable", "Launch.Fable.Extreme", "Launch.Jev.High"] {
        let answer = ask(&running, datom);
        assert!(refused(&answer), "6 {datom}: {answer}");
    }

    // 7. A type the file does not declare is refused.
    let answer = ask(&running, "Model.Add.{ Opus Some.Name.Unknown }");
    assert!(answer.starts_with("Model.Refused."), "7: {answer}");

    // 8. The two are listed.
    assert_eq!(
        ask(&running, "Model.List"),
        "Model.Listed.[ { Jev None } { Fable Some.Name.QuadrupleEffort } ]",
        "8"
    );

    // 9. One is removed.
    assert_eq!(ask(&running, "Model.Remove.Jev"), "Model.Removed.Jev", "9");

    // 10. And is refused again.
    let answer = ask(&running, "Launch.Jev");
    assert!(refused(&answer), "10: {answer}");

    // And the Nexus refuses on its own what no client read: the signal of a
    // Launch of a head it does not hold, and of one carrying a value its
    // entry does not name.
    let mut stream = UnixStream::connect(&running.socket).expect("the socket");
    let launch = |name: &str, carried: Option<Carried>| {
        Input::Launch(Model {
            name: model::Name(String::from(name)),
            carried,
        })
    };
    assert_eq!(
        stream.ask(&launch("Jev", None)),
        Ok(Output::Refused(String::from("Jev was never added")))
    );
    assert_eq!(
        stream.ask(&launch(
            "Fable",
            Some(Carried::TripleEffort(TripleEffort::High))
        )),
        Ok(Output::Refused(String::from(
            "Fable does not carry what its entry names"
        )))
    );
    assert_eq!(
        stream.ask(&launch(
            "Fable",
            Some(Carried::QuadrupleEffort(QuadrupleEffort::Maximum))
        )),
        Ok(Output::Launched)
    );
}
