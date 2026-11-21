//! The Tally Nexus's modules as the built tool generates them from
//! fixtures/nexus/tally, compiled together as a crate of its own with the
//! process library's names stood in for, once with no `datom-codec` at all
//! and once with its `datom` feature: what the Process root's module says of
//! each process compiles against bodies written by hand, and the enum over
//! the processes hands each begin and take to the body it is for.

extern crate self as process;
extern crate self as tally;

use core::marker::PhantomData;

macro_rules! standard {
    ($($item:item)*) => { $(
        #[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
        #[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
        $item
    )* };
}

// The stand-in for the process library: its traits, and the standard types
// a Nexus's processes name.
pub trait Processable: Sized {
    type Taken;
    type Gift;
    type Outcome;
    fn begin<N: Giving<Self>>(&mut self, giving: N) -> Option<Self::Outcome>;
    fn take<N: Giving<Self>>(&mut self, taken: Self::Taken, giving: N) -> Option<Self::Outcome>;
}

/// What a process is handed to give with. To give is to move.
pub trait Giving<A: Processable> {
    fn give<N: Givable<A> + Into<A::Gift>>(&mut self, gift: N);
}

/// A thing declared to leave a particular process; Returned is what comes back for it.
pub trait Givable<A: Processable> {
    type Returned;
}

/// The dispatch boundary of the aggregate Process the manager holds.
/// Its implementation supplies each concrete Processable body with its typed giver.
pub trait Dispatchable: Sized {
    type Taken;
    type Gift;
    type Outcome;

    fn dispatch_begin(&mut self, receive: impl FnMut(Self::Gift)) -> Option<Self::Outcome>;
    fn dispatch_take(
        &mut self,
        taken: Self::Taken,
        receive: impl FnMut(Self::Gift),
    ) -> Option<Self::Outcome>;
}

/// Turns the manager's gift receiver into the giving capability of one concrete process.
pub struct Giver<A, Receive> {
    receive: Receive,
    process: PhantomData<fn() -> A>,
}

impl<A, Receive> From<Receive> for Giver<A, Receive>
where
    A: Processable,
    Receive: FnMut(A::Gift),
{
    fn from(receive: Receive) -> Self {
        Self {
            receive,
            process: PhantomData,
        }
    }
}

impl<A, Receive> Giving<A> for Giver<A, Receive>
where
    A: Processable,
    Receive: FnMut(A::Gift),
{
    fn give<N: Givable<A> + Into<A::Gift>>(&mut self, gift: N) {
        (self.receive)(gift.into());
    }
}

pub trait Unwindable {
    fn unwound() -> Self;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Owner {
    Signal,
    Memory,
}
pub trait Ownable {
    fn owner(&self) -> Owner;
}
standard! {
    pub struct ProcessId(pub i64);
    pub struct UserId(pub i64);
    pub struct Caller { pub process_id: ProcessId, pub user_id: UserId }
    pub struct Unremembered;
    pub enum Unobserved { Unsubmitted, Unanswered }
    pub enum Answer { Succeeded, Failed }
}

#[path = "../generated/nexus/tally/memory.rs"]
pub mod memory;
#[path = "../generated/nexus/tally/process.rs"]
pub mod processes;
#[path = "../generated/nexus/tally/signal.rs"]
pub mod signal;

// The bodies of the two processes, written by hand, and what tests give them to give with.
mod bodies {
    use super::memory::Addition;
    use super::processes::*;
    use super::signal::Total;
    use super::{
        Caller, Dispatchable, Givable, Giving, Ownable, Owner, ProcessId, Processable,
        Unremembered, UserId,
    };

    impl Processable for SumProcess {
        type Taken = SumTaken;
        type Gift = Gift;
        type Outcome = Summing;

        fn begin<N: Giving<Self>>(&mut self, mut giving: N) -> Option<Summing> {
            for addend in &self.integer_vector {
                giving.give(Addend(*addend));
            }
            None
        }

        fn take<N: Giving<Self>>(&mut self, taken: Adding, mut giving: N) -> Option<Summing> {
            match taken {
                Adding::Added(total) => {
                    giving.give(Summed(total));
                    Some(Summing::Answered)
                }
                Adding::Refused | Adding::Unremembered => {
                    giving.give(Overflowed {});
                    Some(Summing::Refused)
                }
            }
        }
    }

    impl Processable for AddendProcess {
        type Taken = AddendTaken;
        type Gift = Gift;
        type Outcome = Adding;

        fn begin<N: Giving<Self>>(&mut self, mut giving: N) -> Option<Adding> {
            giving.give(crate::memory::Add(self.addend.0));
            None
        }

        fn take<N: Giving<Self>>(&mut self, taken: AddendTaken, _giving: N) -> Option<Adding> {
            Some(match taken {
                Ok(Addition::Added(total)) => Adding::Added(total),
                Ok(Addition::Overflowed) => Adding::Refused,
                Err(Unremembered) => Adding::Unremembered,
            })
        }
    }

    fn sum() -> Process {
        Process::Sum(SumProcess {
            caller: Caller {
                process_id: ProcessId(1),
                user_id: UserId(2),
            },
            integer_vector: vec![1, 2],
        })
    }

    #[test]
    fn each_begin_goes_to_the_body_of_its_process() {
        let mut kept = Vec::new();
        let ended = sum().dispatch_begin(|gift| kept.push(gift));
        assert_eq!(ended, None);
        assert_eq!(kept, [Gift::Addend(Addend(1)), Gift::Addend(Addend(2))]);

        let mut kept = Vec::new();
        let mut addend = Process::Addend(AddendProcess { addend: Addend(5) });
        assert_eq!(addend.dispatch_begin(|gift| kept.push(gift)), None);
        assert_eq!(kept, [Gift::Add(crate::memory::Add(5))]);
    }

    #[test]
    fn each_take_goes_to_the_process_it_is_for_as_that_process_takes_it() {
        let mut kept = Vec::new();
        let mut addend = Process::Addend(AddendProcess { addend: Addend(5) });
        let taken: Taken = Taken::Addition(Ok(Addition::Added(Total(5))));
        let ended = addend.dispatch_take(taken, |gift| kept.push(gift));
        assert_eq!(ended, Some(Outcome::Addend(Adding::Added(Total(5)))));

        let mut sum = sum();
        let ended = sum.dispatch_take(Taken::Adding(Adding::Added(Total(3))), |gift| {
            kept.push(gift)
        });
        assert_eq!(ended, Some(Outcome::Sum(Summing::Answered)));
        assert_eq!(kept, [Gift::Summed(Summed(Total(3)))]);
        // What no process takes is let go.
        let ended = sum.dispatch_take(Taken::Addition(Err(Unremembered)), |gift| kept.push(gift));
        assert_eq!(ended, None);
    }

    #[test]
    fn a_process_born_of_a_call_waits_on_the_signal_process() {
        assert_eq!(sum().owner(), Owner::Signal);
    }

    #[test]
    fn what_is_given_says_what_returns_for_it() {
        fn returned<A: Processable, G: Givable<A>>() -> &'static str {
            std::any::type_name::<G::Returned>()
        }
        assert_eq!(returned::<SumProcess, Summed>(), "()");
        assert_eq!(returned::<SumProcess, Overflowed>(), "()");
        assert!(returned::<SumProcess, Addend>().ends_with("Adding"));
        assert!(returned::<AddendProcess, crate::memory::Add>().contains("Unremembered"));
    }
}
