//! The structural layer shared by every Protos dialect.
mod core;
mod layout;
mod rendering;
mod traversing;
pub use core::{
    Boundary, BoundedProtosizable, Canonicalizable, Compactable, Enclosure, Error, Extent, Problem,
    Protos, Protosizable, ReaderBudget, Separator, Spendable, Symbol, Textualizable,
};
