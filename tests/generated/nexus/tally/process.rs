#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    unused_extern_crates,
    clippy::type_complexity
)]
#[rustfmt::skip]
extern crate alloc;
#[rustfmt::skip]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
    feature = "datom",
    derive(::datom_codec::Datomizable, ::datom_codec::Composing)
)]
pub struct Addend(pub i64);
#[rustfmt::skip]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "datom",
    derive(::datom_codec::Datomizable, ::datom_codec::Composing)
)]
pub enum Summing {
    Answered,
    Refused,
}
#[rustfmt::skip]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "datom",
    derive(::datom_codec::Datomizable, ::datom_codec::Composing)
)]
pub enum Adding {
    Added(::tally::signal::Total),
    Refused,
    Unremembered,
}
#[rustfmt::skip]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "datom",
    derive(::datom_codec::Datomizable, ::datom_codec::Composing)
)]
pub struct Summed(pub ::tally::signal::Total);
#[rustfmt::skip]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "datom",
    derive(::datom_codec::Datomizable, ::datom_codec::Composing)
)]
pub struct Overflowed {}
#[rustfmt::skip]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "datom",
    derive(::datom_codec::Datomizable, ::datom_codec::Composing)
)]
pub struct SumProcess {
    pub caller: ::process::Caller,
    pub integer_vector: ::alloc::vec::Vec<i64>,
}
#[rustfmt::skip]
pub type SumTaken = Adding;
#[rustfmt::skip]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "datom",
    derive(::datom_codec::Datomizable, ::datom_codec::Composing)
)]
pub struct AddendProcess {
    pub addend: Addend,
}
#[rustfmt::skip]
pub type AddendTaken = ::core::result::Result<
    ::tally::memory::Addition,
    ::process::Unremembered,
>;
#[rustfmt::skip]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "datom",
    derive(::datom_codec::Datomizable, ::datom_codec::Composing)
)]
pub enum Process {
    Sum(SumProcess),
    Addend(AddendProcess),
}
#[rustfmt::skip]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "datom",
    derive(::datom_codec::Datomizable, ::datom_codec::Composing)
)]
pub enum Outcome {
    Sum(Summing),
    Addend(Adding),
    Unwound,
}
#[rustfmt::skip]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "datom",
    derive(::datom_codec::Datomizable, ::datom_codec::Composing)
)]
pub enum Gift {
    Addend(Addend),
    Summed(Summed),
    Overflowed,
    Add(::tally::memory::Add),
}
#[rustfmt::skip]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "datom",
    derive(::datom_codec::Datomizable, ::datom_codec::Composing)
)]
pub enum Taken {
    Adding(Adding),
    Addition(::core::result::Result<::tally::memory::Addition, ::process::Unremembered>),
}
#[rustfmt::skip]
impl ::process::Givable for Addend {
    type Returned = Adding;
}
#[rustfmt::skip]
impl ::core::convert::From<Addend> for Gift {
    fn from(gift: Addend) -> Gift {
        Gift::Addend(gift)
    }
}
#[rustfmt::skip]
impl ::process::Givable for Summed {
    type Returned = ();
}
#[rustfmt::skip]
impl ::core::convert::From<Summed> for Gift {
    fn from(gift: Summed) -> Gift {
        Gift::Summed(gift)
    }
}
#[rustfmt::skip]
impl ::process::Givable for Overflowed {
    type Returned = ();
}
#[rustfmt::skip]
impl ::core::convert::From<Overflowed> for Gift {
    fn from(_: Overflowed) -> Gift {
        Gift::Overflowed
    }
}
#[rustfmt::skip]
impl ::process::Givable for ::tally::memory::Add {
    type Returned = ::core::result::Result<
        ::tally::memory::Addition,
        ::process::Unremembered,
    >;
}
#[rustfmt::skip]
impl ::core::convert::From<::tally::memory::Add> for Gift {
    fn from(gift: ::tally::memory::Add) -> Gift {
        Gift::Add(gift)
    }
}
#[rustfmt::skip]
impl ::process::Unwindable for Outcome {
    fn unwound() -> Self {
        Outcome::Unwound
    }
}
#[rustfmt::skip]
impl ::process::Ownable for Process {
    fn owner(&self) -> ::process::Owner {
        match self {
            Process::Sum(_) | Process::Addend(_) => ::process::Owner::Signal,
        }
    }
}
#[rustfmt::skip]
impl ::process::Processable for Process {
    type Taken = Taken;
    type Gift = Gift;
    type Outcome = Outcome;
    fn begin<N: ::process::Giving<Gift>>(
        &mut self,
        giving: N,
    ) -> ::core::option::Option<Outcome> {
        match self {
            Process::Sum(sum) => {
                ::process::Processable::begin(sum, giving).map(Outcome::Sum)
            }
            Process::Addend(addend) => {
                ::process::Processable::begin(addend, giving).map(Outcome::Addend)
            }
        }
    }
    fn take<N: ::process::Giving<Gift>>(
        &mut self,
        taken: Taken,
        giving: N,
    ) -> ::core::option::Option<Outcome> {
        match (self, taken) {
            (Process::Sum(sum), Taken::Adding(carried)) => {
                ::process::Processable::take(sum, carried, giving).map(Outcome::Sum)
            }
            (Process::Addend(addend), Taken::Addition(carried)) => {
                ::process::Processable::take(addend, carried, giving)
                    .map(Outcome::Addend)
            }
            _ => ::core::option::Option::None,
        }
    }
}
