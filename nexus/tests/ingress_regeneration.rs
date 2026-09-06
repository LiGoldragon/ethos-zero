use ethos_zero::{File, Generating};
use protos::{Actualizable as _, Potential};

#[test]
fn checked_source_manifest_ingress_matches_the_current_ethos_generator() {
    let source = include_str!("../ethos/ingress.ethos");
    let generated = Potential::<File>::from(source)
        .actualize(())
        .expect("ingress schema reads")
        .generate()
        .expect("ingress schema generates");
    assert_eq!(generated, include_str!("../src/ingress.rs"));
}
