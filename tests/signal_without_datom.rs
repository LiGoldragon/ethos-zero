//! A generated Signal contract is a Rust-only surface when its optional
//! `datom` feature is absent.  This deliberately compiles it in a separate
//! Cargo package with no `datom-codec` dependency.

use std::process::Command;

#[test]
fn generated_signal_compiles_without_datom_codec() {
    let directory = format!(
        "{}/signal-without-datom-{}",
        env!("CARGO_TARGET_TMPDIR"),
        std::process::id()
    );
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(format!("{directory}/src")).expect("temporary source directory");
    std::fs::write(
        format!("{directory}/Cargo.toml"),
        "[package]\nname = \"signal-without-datom\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[features]\ndatom = []\n[dependencies]\nrkyv = { version = \"0.8\", default-features = false, features = [\"std\", \"bytecheck\", \"little_endian\", \"pointer_width_32\", \"unaligned\"] }\n",
    )
    .expect("temporary manifest");
    std::fs::write(
        format!("{directory}/src/lib.rs"),
        include_str!("generated/orchestrate.rs"),
    )
    .expect("generated Signal source");

    let status = Command::new("cargo")
        .args([
            "check",
            "--manifest-path",
            &format!("{directory}/Cargo.toml"),
            "--no-default-features",
        ])
        .status()
        .expect("cargo is available");
    assert!(
        status.success(),
        "generated Signal must not require datom-codec"
    );
}
