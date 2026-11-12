//! A separately compiled producer, deliberately named `signal`.
pub use library::Voice;
#[path = "../generated/flow-library.rs"]
pub mod library;
#[path = "../generated/namespace-ordinary.rs"]
pub mod namespace_ordinary;
#[path = "../generated/namespace-recursive.rs"]
pub mod namespace_recursive;
