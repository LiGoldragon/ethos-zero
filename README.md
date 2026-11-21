# ethos-zero

The ethos schema language, version zero. Ethos specifies the types,
datom fills them with data, and ethos generates the Rust.

## Anatomy

`src/lib.rs` is the ontology: the layers, the traits, the declaration
types. Each pass is a module named for it.

| pass | trait | from | to |
|---|---|---|---|
| canonicalization | `Canonicalizable` | sweet text | `Canonical`, the braced form, with its seam |
| protosization (protos) | `Protosizable` | canonical text | `protos::Protos` |
| conception | `Ethosizable<File>` | `protos::Protos` | `File`, checked whole |
| checking | `Resolving`, `Checkable` | `File` | names resolved, duplicates and undeclared names refused |
| generation | `Generating` | `File` | Rust text, or a whole-file error |
| datomization | | each declared type | its `datom_codec::Datomizable` and `datom_codec::Composing` derives |
| nexus | `Verifying`, `Assembling` | the `File`s of one namespace, each named for its file | each module's Rust, a Process root's with the enums over its processes, or a refusal naming the file |
| ascent | `Protosizable`, `Textualizable` | `File` | canonical text on one line (cannot err) |
| printing | `Printable` | `File` or `protos::Protos` | the sweet form, each section in protos' vertical print (cannot err) |
| actualization | `Actualizing<File>` on `Potential<File>` | text | `File`, or an `Error` |

Every error carries the Protos path of the structure in error, from the
file's root: a headed form puts its head at child zero and body at child
one, qualified head arguments remain below the head, and a reference's
arguments are the angled enclosure beside it in its list
(`Vector<Bogus>` at index 0 puts `Bogus` at `[ 1 0 ]`). `Locating`
follows that path down the source's structure and carries the node's
start back across the sweet form's seam, so every error is situated as a
line and a column in the text as written. Every method
call lives under a trait; there are no free functions, no inherent impls,
no closures beyond what std forces, and no lookup tables: the enums are
walked variant by variant.

The crate eats its own food: `error.ethos` generates `src/error.rs`
and `ethos-zero.ethos` generates `src/ethos-zero.rs`; the freshness test
regenerates both and every fixture under `tests/generated/`.

## File variants

Six roots: Library, Signal, Operation, Memory, Emission, Process. Signal
declares what a Nexus says; Operation what it does, one operation for every
effect; Memory what it remembers; Emission what it sends into the world, each
emission one external call and its outcome what the translator observed;
Process what it is doing, each thing being done one object from its birth to
its end (Process.4); Library what they share.

```
Library    [ imports ] [ types ] [ traits ] [ associations ]
Signal     [ imports ] [ inputs ] [ outputs ] [ types ]       ; Input and Output implied
Operation  [ imports ] [ operations ] [ outcomes ] [ types ]   ; Operation and Outcome implied
Memory     [ imports ] [ records ] [ changes ] [ answers ] [ types ]
Emission   [ imports ] [ emissions ] [ outcomes ] [ types ]    ; Emission and Outcome implied
Process    [ imports ] [ processes ] [ types ]                 ; Process, Gift, Taken and Outcome held
```

Operation's sections are proposed, not yet the living's word.

The roots of one Nexus are generated together: see Nexus below.

A Signal generates `pub enum Input` and `pub enum Output` from its
first two sections, so those two names are the ones a Signal may not
also declare (`Query` and `Response` are ordinary names); an Operation
likewise generates `pub enum Operation` and
`pub enum Outcome`, and an Emission `pub enum Emission` and
`pub enum Outcome`, each variant carrying its declared payload. A Memory
generates nothing but its declared types and reserves no name; a
Memory record may be named `Record`. A file headed `Sema`, Memory's head
before 15.0.0, is refused as `Conceptual.{ [ 0 ] Renamed.Memory }`.

A Signal pairs its sections by position: the n-th output answers the
n-th input, and an output past the last input answers none, a refusal or a
reaction. An input left without an output is refused as
`Unpaired.<input>` at its path. A Signal with inputs generates
`Answering`: one capability per input, the variant in snake case,
borrowing what the input carries (taking nothing where it carries nothing)
and yielding what its output carries (`()` where it carries nothing) or
the refusal, any `Output`, and nothing else. The input is `Answerable`: the
file generates `impl Answerable for Input`, whose associated type `Answer`
is what an answer is, and whose one capability `answer` borrows the bearer
of `Answering` and yields the `Answer`, holding the dispatch. A Signal with
no stream output answers each input once, so its `Answer` is the `Output`.
The dispatch lends each capability the payload it matched, bound as
`carried`. A Nexus answers an input with `input.answer(&nexus)`:

```rust
pub trait Answering {
    fn launch(&self, launch_request: &LaunchRequest) -> core::result::Result<FlowId, Output>;
    fn report(&self, report: &Report_Data) -> core::result::Result<(), Output>;
}
pub trait Answerable {
    type Answer;
    fn answer<Nexus: Answering>(&self, nexus: &Nexus) -> Self::Answer;
}
impl Answerable for Input {
    type Answer = Output;
    fn answer<Nexus: Answering>(&self, nexus: &Nexus) -> Output {
        match self {
            Input::Launch(carried) => nexus.launch(carried).map_or_else(|refusal| refusal, Output::Launched),
            Input::Report(carried) => nexus.report(carried).map_or_else(|refusal| refusal, |()| Output::Reported),
        }
    }
}
```

A capability's parameter is named as a field of its type is
(`launch_request`), or for the variant where the payload is declared in
place (`report`); a keyword is written raw. An input may be named `Answer`:
its capability is `answer`, on `Answering`, and the dispatch is the input's
own (`fixtures/answer-input.ethos`). An input whose capability would be
named as another input's is refused as `Duplicate.<capability>`, and
`Answering`, `Answerable` and `Answer` are names a Signal may not declare. No
standard library crate holds the root-implied traits yet, so `Answerable`
is generated into every Signal's file. `Answering` carries
`#[allow(clippy::result_large_err, clippy::ptr_arg, clippy::borrowed_box)]`:
the refusal is the wire's `Output`, answered whole, however large its
largest variant, and a capability borrows exactly the type its input
carries, `&String`, `&Vec<_>` or `&Box<_>` among them.

An Emission pairs its sections by position: the n-th outcome is the n-th
emission's, and an outcome past the last emission pairs with none. An
emission left without an outcome is refused as `Unpaired.<emission>` at its
path (`Emission [] [ Go Stop ] [ Gone ] []` is
`Conceptual.{ [ 1 1 1 ] Unpaired.Stop }`). An emission is Emittable: the
translator turns it into the call and yields its outcome. No standard
library holds the root-implied traits yet, so the file generates both
traits beside its enums, and binds each emission's carried type to its
outcome's:

```rust
pub trait Emitted { type Outcome; }
pub trait Emittable<Translator>: Emitted {
    fn emit(&self, translator: &Translator) -> Self::Outcome;
}
impl Emitted for Command_Data { type Outcome = Outcome_Data; }   // from Command.{ … } and Outcome.[ … ]
```

The consumer writes `impl Emittable<ItsTranslator> for Command_Data` with
`emit` alone; `Emitted` already fixed the outcome, so `emit` yields it and
no other. An outcome carrying nothing binds `()`. Two emissions
carrying the same type would bind it twice and are refused as
`Duplicate.<type>` at the second.

An emission may carry nothing, as an input may. A bare emission whose
name the file neither declares nor imports is a unit of its own, a
struct of no fields named as the emission is and generated beside the
file's types, so it bears `Emittable` like any other and binds the n-th
outcome; its variant in `Emission` carries nothing, so its datom is
written bare, `Ping`, as a bare output's is (`fixtures/bare-emission.ethos`):

```rust
pub struct Probe {}                                          // from the bare emission Probe
pub enum Emission { Probe, Consume(Consume_Data), Ping }
impl Emitted for Probe { type Outcome = Probed_Data; }        // Probed.[ … ], the first outcome
impl Emitted for Ping { type Outcome = (); }                  // Pong, carrying nothing
```

A bare emission naming a declared or imported type carries that type, as
any bare variant does, and has no unit. The unit is the emission's alone:
an outcome of the same name carries nothing. The unit is a declaration of
the file at its emission, so its name is one a type may take: a bare
emission named `Emission`, `Outcome`, `Emitted`, `Emittable` or a derived
name is refused as `Duplicate.<name>`, and one named as an intrinsic,
`String`, as `Intrinsic.String`. `Emitted` and `Emittable` are names an
Emission may not declare, as `Emission` and `Outcome` are not.
`fixtures/command-emission.ethos` is the vision's example, its import
written rooted.

## Memory

A Memory has five sections: its imports, the records it keeps, the changes it
makes, the answers to them in the order of the changes, and the types they
carry. Each entry of the records, the changes and the answers declares a type
in place, `Watch.{ FlowId FilePath }`, or names one the file imports or
declares in its types, a bare `Observed`: a record may be of an imported type,
and a change may be answered by one, such as the standard `process:[ Answer ]`.
A change left without an answer is refused as `Unpaired.<change>`, as an
emission is without its outcome.

```
Memory
[ harness:[ Event Observed ]
  flow:[ FlowId ] ]
[ Watch.{ FlowId FilePath }
  Observed ]
[ Observe.{ FlowId Vector<Event> }
  Recall.{} ]
[ Observation.Vector<Changed>
  Recollection.Vector<Observed> ]
[]
```

What returns for a record kept is the standard `Answer`; what returns for a
change is the `Result` of its answer or `Unremembered`, as what returns for an
emission is the `Result` of its outcome or `Unobserved`. The old two-section
Memory is gone: a file of two sections is `Arity.{ 5 2 }`.

## Process

A Process declares each process as what it is born of, a call, a record or a
process given, whose name it bears and which the file imports or declares; then a
brace of the configured values moved into it at its birth, what it gives, and
what it ends in. A process that ends one way declares no outcome:

```
Process
[ claude_harness:signal:[ Report Reported Changed ]
  claude_harness:memory:[ Watch Observe ]
  claude_harness:[ RowCount ]
  claude_harness:emission:[ Follow ] ]
[ Report.{ []
           [ Reported Watch Observe Changed ]
           Reporting }
  Watch.{ [ RowCount ]
          [ Follow Observe Changed ]
          Watching } ]
[ Reporting.[ Observed Unremembered ]
  Watching.[ Forgotten Unfollowed ] ]
```

A file alone checks every name it resolves, two processes of one name as
`Duplicate`, and a brace of fewer than two brackets or more than one outcome as
`Arity`; its module is the types its outcomes carry. `Process`, `Gift`, `Taken` and
`Outcome` are held for the enums over the processes: they, and each process's
positions, are generated when the roots of the Nexus are read together.

## Nexus

`ethos-zero` generates a Nexus from all its root files at once: signal, meta,
memory, emission, library and process, each a module named for its file, the roots
naming each other by the Nexus's namespace, the crate that holds them:
`tally:signal:[ Total ]` is a name of the module `signal`, `tally:[ Add ]` a name
of the Library, at the crate's root. The namespace is written nowhere: it is the
one first segment of an import whose imports, those that name a module of the set, all land on one
that declares what it takes (a type, a trait, a variant of an input, an output, an
emission or an outcome, or a record named). Two that land are `Namespace.<second>`;
none, and a Process root that names something is `Foreign.<name>` for it. What is
imported from outside the Nexus, a contract it serves beside its own, is
declared in a root of the Nexus until imports resolve across crates.

A process is born of a **call**, a **record** or a **process that gives it**:

| born of | named for | holds | owned by |
|---|---|---|---|
| a call | an input of a Signal, the Nexus's or its meta | `caller: process::Caller`, the input's payload (a struct variant's positions spread), the configured values | the signal process |
| a record | a record of the Memory | the record's positions spread, or the record as it is where it is not a struct, then the configured values | the memory process |
| a process | a type the Process root declares in its types | that type's positions spread, or the type, then the configured values | the signal process, below its giver |

A name the file declares in its types and also names a process is born of a
process, and nothing else declares it. Anything else is `Unborn.<name>`.

A thing a process gives is an output, a record, a change, an emission or a
process; any other is `Ungivable.<name>`. Each is `Givable`, and what returns
for it is its `Returned`:

| given | its type | returns |
|---|---|---|
| an output | a type the Process module makes: a newtype over what it carries, `pub struct Summed(pub Total)`, or a struct of no fields where it carries nothing; the payload an output declares in place is its own | nothing |
| a record | the record's type | `process::Answer` |
| a change | the change's type | `Result<its answer, process::Unremembered>` |
| an emission | the type it carries (`Follow_Data`, a unit of its own, a carried type) | `Result<its outcome, process::Unobserved>` |
| a process | the type its root declares | its outcome at its end, nothing where it declares none |

The Process root's module then holds, beyond the types it declares: the struct of
each process's positions, `SumProcess`; what each takes, `SumTaken`; and the enums
over the Nexus's processes:

```rust
pub struct SumProcess { pub caller: process::Caller, pub integer_vector: alloc::vec::Vec<i64> }
pub type SumTaken = Adding;
pub struct AddendProcess { pub addend: Addend }
pub type AddendTaken = core::result::Result<tally::memory::Addition, process::Unremembered>;
pub enum Process { Sum(SumProcess), Addend(AddendProcess) }
pub enum Outcome { Sum(Summing), Addend(Adding), Unwound }
pub enum Gift { Addend(Addend), Summed(Summed), Overflowed, Add(tally::memory::Add) }
pub enum Taken { Adding(Adding), Addition(core::result::Result<tally::memory::Addition, process::Unremembered>) }
```

Where one kind of thing returns to a process, what it takes is that type, bare
(`SumTaken`): an enum of one variant is refused, and a wrapper of one type is that
type. Where more than one returns, it is an enum, each variant named for what
returns, the answer or the outcome. The Nexus's `Taken` is the same of all its
processes, and two different types returning under one name are `Duplicate`. An
output that carries nothing is a bare variant of `Gift`. `Outcome` has a variant
for each process, named for it, carrying its outcome where it declares one, and
`Unwound`, the end of a process with its owner. The module also generates
`Givable` and `From<_> for Gift` for each thing given, `Unwindable` for `Outcome`,
`Ownable` for `Process`, and `Processable` for `Process`, which hands each begin and
take to the `Processable` of its own process, written by hand with
`type Taken = SumTaken`, `type Gift = Gift` and its own outcome (`()` where it
declares none). The Process root's imports that name an input or an output are read
as no type, so an outcome's bare variant named for one, `Configuring.[ Configured
Unremembered ]` beside the output `Configured`, carries nothing.

A Process root generated as a module named `process` cannot stand at a crate's root
beside the dependency named `process`, so the crate includes it under another name:
`#[path = "generated/process.rs"] pub mod processes;`.

The standard names the module writes are `process::Caller`, `process::Answer`,
`process::Unremembered`, `process::Unobserved`, and the traits `process::Processable`,
`process::Giving`, `process::Givable`, `process::Unwindable` and `process::Ownable`
with `process::Owner`: the process library holds them (no_std and alloc), and a
Nexus's processes crate depends on it under the name `process`.

Every root's types archive (rkyv) and gate their datom derives behind the
`datom` feature, a Library's, an Operation's and a Memory's as a Signal's:
any type can cross a wire, and a Nexus compiles its whole contract without
datom-codec. A crate holding generated Rust declares a `datom` feature and
enables it where it textualizes (its CLI); it depends on rkyv always, and on
datom-codec only under that feature (or with datom-codec's `rkyv` feature
where a position holds a `Decimal`, a `Meaning` or a datom-codec `Error`).
The `flow-contract` test compiles the Flow Nexus's four modules together in
a separate package, with and without `datom`, and sends a value of each root
through rkyv.

A struct position may declare its type in place: `Brief.String`,
`State.[ Running Ended ]`, `Capsule.{ Home.String Login.Vector<String> }`.
The type is named by its own name, shares the file's namespace (a second
`Brief` anywhere in the file is a `Duplicate`), and the position holds it
by name, so its field is `brief: Brief`. It is declared outside any
enclosing identity, and emitted before the struct that holds it.

A head over one type declares a newtype. `LockId.Integer` emits the tuple
struct `pub struct LockId(pub i64)`, with the same archive and optional
datom derives as every other declared type. The name is therefore its own
Rust type rather than another spelling of the type it holds.

An import starts with a colon, then the uncapitalized namespace
segments, each followed by a colon, then the capitalized name:
`:flow_ethos:message:Type` imports one name, `:protos:[ String Integer ]`
a group (the bare `:protos:` and the bracket beside it), and
`:std:clone:Clonable.Clone` names it `Clonable` where the source calls it
`Clone`. The generated Rust writes the name fully qualified,
`flow_ethos::message::Type`. The chained form, without the leading
colon, reads the same: `protos:String`, `protos:[ String ]`, and
`signal_capsule:signal:[ Input Output ]` as
`:signal_capsule:signal:[ Input Output ]`.

An import may stand where the type is used instead. In a declaration the
colon replaces the period: `Topic:custom:sub:Name` declares `Topic` over
`custom::sub::Name`, in the types section or in place in a struct. A
field takes the import with its leading colon:
`Failure.{ :protos:Error String }`. A capitalized name before a colon
declares; an uncapitalized one is a source, so `Holder.{ custom:Name }`
is a field of `custom::Name`. A position is boxed only
where it closes a by-value cycle: it names, through aliases, `Option` and
`Result` but not `Vector`, a type declared no later than its owner that
reaches the owner by value; in `Twin.{ Twig Twig }  Twig.[ Tip  Grow.Twin ]`
only `Grow` is boxed. Every position that reaches its owner by
any path, `Vector` included, carries `#[rkyv(omit_bounds)]`, and its type
states the serializer, deserializer and validator bounds once, so a
recursive contract archives and restores. The generated Rust writes
every standard name by its core or alloc path, `alloc::string::String`,
`alloc::vec::Vec`, `core::option::Option`, `core::result::Result` and
`alloc::boxed::Box`, so a declaration cannot capture those names and the
module builds with no operating system under it as well as with std: each
generated module opens with `extern crate alloc;`. A variant `Name.{ T1 T2 }` carries a generated
struct `Name_Data` with one field per position.

A field is named for its type alone, in snake case, and never for the
namespace the type comes from: `:protos:Error` gives `error`,
`nexus:ConfigurationState<OrchestrateNexusConfiguration>` gives
`orchestrate_nexus_configuration_configuration_state`. A constructed type
is named type-first, `Vector<String>` as `string_vector`, `Option<Lock>` as
`lock_option`. Positions whose names are the same are told apart as first
and second, `Generation.{ String String }` giving `first_string` and
`second_string`, and `Errors.{ :a:Error :b:Error }` giving `first_error`
and `second_error`. Where a field so named still meets another,
`Pair.{ String String FirstString }` giving `first_string` twice, the
later position is refused as `DuplicateField.first_string` at its path; a
head naming the position, `Brief.String`, tells the two apart.

A type or trait declaration is refused when its name is an intrinsic's
(`Intrinsic.Result`: the declaration would shadow the intrinsic for every
later reference), or does not begin with a capital (`Case.a`). A declared
type with no finite value, one that reaches itself with no `Vector`,
`Option` or other variant to stop at, such as `S.{ Self }`, is refused as
a `Cycle`. Every generated item carries `#[rustfmt::skip]`, an inline
payload's as much as its enum's.

An enum has two variants or more. An enum of one variant or none is
refused as `TooFewVariants.<enum>` at its variants, a declared enum named by
its declaration and an enum a variant declares in place named by that
variant: `Reply.[ Accepted ]` is refused as `TooFewVariants.Reply`,
`Never.[]` as `TooFewVariants.Never`, and
`Reply.[ Accepted Refused.[ Busy ] ]` as `TooFewVariants.Refused`, in every
section's types alike. A root's sections, a Signal's inputs and outputs,
an Operation's operations and outcomes and an Emission's emissions and
outcomes, are counted as the root's and may hold one or none; a section
with none generates no enum.

A bare variant whose name is a type the file declares or imports carries
that type: `Failure.[ SyntaxError Unwritable ]` with `SyntaxError` declared
gives `SyntaxError(SyntaxError)`, and with `[ :sema:Block ]` imported,
`Part.[ Block Text.String ]` gives `Block(sema::Block)`. The rule is a bare variant's
alone. A variant with a payload of its own is a variant of that name
carrying that payload, whatever types the file declares, since a Rust
variant and a Rust type are two names: the input `Lock.LockRequest` beside
a declared `Lock` gives `Input::Lock(LockRequest)`, and `Code.String`,
`Code.Code` or `Code.{ … }` in an enum beside a declared `Code` gives
`Code(String)`, `Code(Code)` or `Code(Code_Data)`, in every section alike
(`fixtures/lock-signal.ethos`). A name a root implies, a Signal's `Input`,
is generated beside the file's own names, and no bare variant carries it.

A variant that declares its payload in place names a derived type:
`X.{ … }` or `X.[ … ]` in an enum gives `X_Data`. Derived names are unique
across the whole file: where two enums each declare an `X` in place, each
payload is named for its enum instead (`P_X_Data`, `Q_X_Data`), and a payload
declared inside a derived enum carries that enum's name as its stem
(`Y_Data_X_Data`). An authored name equal to a derived name is refused as a
`Duplicate` at the authored declaration.
Generated generic parameters similarly move from `A`, `B`, and so on only
when one would capture an authored type reference.

## Stream

`Stream` is an intrinsic, known without import as `Vector` is, and takes
exactly two types: the state a stream opens with and the change it then
carries, `Stream<Vector<Pane> Changed>`. It is answered as a stream: one
subscription, then frames. The signal library holds its two Rust types,
`signal::Frame<State, Change>`, an archived and datomizable enum
`Opened(State) | Changed(Change) | Lagged(State) | Ended(Ending)`, and
`signal::Stream<State, Change>`, the live stream of frames, never archived.
A crate holding a Signal with a stream output depends on that library under
the name `signal`.

A `Stream` stands in two places. One is the whole payload of a Signal's
output: `Signal [] [ Subscribe ] [ Subscribed.Stream<Vector<Pane> Changed> ] [ … ]`.
The `Output` holds one frame of it,
`Subscribed(signal::Frame<alloc::vec::Vec<Pane>, Changed>)`, with the derives
every `Output` bears, so each frame on the wire is one `Output`. The other is a trait capability's yield, where its
types may be the trait's associated types:
`Publishing.{ [] [ State Change ] [] [ open.{ [ State ] [ Stream<State Change> ] } ] }`
gives `fn open(&self, input: Self::State) -> signal::Stream<Self::State, Self::Change>`
(`fixtures/publishing-trait.ethos`). A `Stream` anywhere else is refused as
`Placement.Stream` at its path: in a field, an alias, a variant's payload in
any root's types or in an Operation's or Emission's sections, a Signal's
input, and inside another type's arguments, `Vector<Stream<…>>`,
`Option<Stream<…>>` and a `Stream` among a `Stream`'s own arguments among
them; an output's payload declared in place, `Watched.{ Stream<…> }`, holds
it as a field and is refused too. `Stream<String>` is refused as
`Arity.{ 2 1 }`. The retired block form `Name.Stream.( … )` is no type: it
is refused as it is read, `Expected.Reference` at `Stream.( … )`.

Where an output answering an input is a `Stream`, that input's capability
yields the live stream or the refusal, and the file generates `Answer`, an
enum deriving nothing: `Once(Output)` for every answer given once, a refusal
among them, and one variant per stream output answering an input, named as
that output and holding its stream (`fixtures/stream-signal.ethos`):

```rust
pub enum Answer {
    Once(Output),
    Subscribed(signal::Stream<alloc::vec::Vec<Pane>, Changed>),
}
pub trait Answering {
    fn launch(&self, launch_request: &LaunchRequest) -> core::result::Result<FlowId, Output>;
    fn subscribe(&self) -> core::result::Result<signal::Stream<alloc::vec::Vec<Pane>, Changed>, Output>;
}
impl Answerable for Input {
    type Answer = Answer;
    fn answer<Nexus: Answering>(&self, nexus: &Nexus) -> Answer {
        match self {
            Input::Launch(carried) => Answer::Once(nexus.launch(carried).map_or_else(|refusal| refusal, Output::Launched)),
            Input::Subscribe => nexus.subscribe().map_or_else(Answer::Once, Answer::Subscribed),
        }
    }
}
```

`Answer` carries `#[allow(clippy::large_enum_variant)]`: an answer given
once is the wire's `Output`, held whole. A stream output that answers an
input and is named `Once` would meet `Answer::Once` and is refused as
`Duplicate.Once` at its name; one named `Answer` gives `Answer::Answer`. A
stream output past the last input is a reaction: its frame is an `Output`
as any stream output's is, and no `Answer` variant holds it.

## Traits

A trait is the bearer of capabilities: `Name.[ capabilities ]`, or
`Name.{ [ supertraits ] [ associated types ] [ CONSTANTS ] [ capabilities ] }`.
A capability's separator says who is called: `.` takes `&self`, `!` takes
`&mut self`, `:` takes no self. `name.[ Yield ]` yields alone;
`name.{ [ inputs ] [ Yield ] }` takes inputs; an empty yield bracket, `name.[]`
or `name.{ [ inputs ] [] }`, yields nothing, and the method returns unit
(`after!{ [ Event ] [] }` gives `fn after(&mut self, input: Event);`).

A capability speaks in `Self`, its trait's parameters, other traits and, for
now, concrete types.
A declared trait in an input becomes a parameter of the generated method,
bounded by that trait; an imported name in an input is assumed to be a
trait. A concrete type in an input is taken as that type. Parameters are
lettered from `N`:

```
Resolvable.[ resolve.{ [ Textualizable ] [ Self ] } ]
; -> fn resolve<N: Textualizable>(&self, input: N) -> Self where Self: Sized;
```

Where an associated type of the trait is already bounded by that trait
(`Streamable.{ [] [ Item<Textualizable> ] [] [ push!{ [ Textualizable ] [ Self ] } ] }`),
the input is that associated type, `Self::Item`. `Self`, a parameter of the
trait's head, and an associated type named directly stay as they are. A
concrete type in an input (`String`, `Vector<Self>`, a declared type) is
written as that type (`resolve.{ [ Rec ] [ Self ] }` gives
`fn resolve(&self, input: Rec) -> Self`). An imported name says nothing of
its role, so in an input it is taken as a trait. A yield may name a concrete type; a
declared trait in a yield is a method parameter too (`make:[ Textualizable ]`
gives `fn make<N: Textualizable>() -> N`).

The flat declaration budget applies to a Library's types section: it refuses
more than 512 type declarations. The other roots retain their structural reader
bounds and are not counted against it.

## Print

The canonical print expands vertically. A structure with more than one
element, one of which has a next layer (a headed form or a brace or
bracket), opens on its line and its elements hang beneath the first,
aligned; the closing delimiter ends the last element's line. Elements that
are all leaves sit on one line, one space apart. A space stands inside
every non-empty bracket and brace; an empty one is `[]` or `{}`. An angled
enclosure stays tight against the name it qualifies, `Vector<Event>`. A
file prints in the sweet form, its root's head on the first line and each
section on its own line; comments are not printed.

```
Memory
[ flow:[ FlowId Voice Event ] ]
[ Flow.{ FlowId
         Voice
         State.[ Running Ended ]
         Vector<Event> } ]
[]
[]
[]
```

The layout is protos' canonical print (`protos::Textualizable`); this crate
adds only the sweet form of a file. `Printable::print` writes it for a
`File` and for any `protos::Protos` read from ethos text; the reader accepts
it and every other layout. An empty `[]` or `{}` is a leaf. The
Flow Nexus's four files under `fixtures/print/` print back byte-identical.
The crate's own `ethos-zero.ethos` and `error.ethos` are written in it, and
the no-argument CLI prints `ethos-zero.ethos` through it.

File ascent projects the File directly into its `protos::Protos`
structure; it does not textualize and parse the result again.

## CLI

```
ethos-zero 'Generate.{ [ /abs/signal.ethos /abs/memory.ethos /abs/process.ethos ] /abs/out-dir }'
# -> Generated.[ /abs/out-dir/signal.rs /abs/out-dir/memory.rs /abs/out-dir/process.rs ]
ethos-zero 'Check.[ /abs/file.ethos ]'
# -> Checked.[ /abs/file.ethos ]
# -> Rejected.{ /abs/file.ethos { { 4 } { 19 } } Conceptual.{ [ 1 1 0 1 1 ] Undeclared.Bogus } }
```

Both take the root files of one namespace together, a single file being a set of one.
Each module is written as the stem of its file, `.rs`, in the order the files were
given. `Check` validates the whole set and writes nothing. A rejection, from either
query, names the file it lies in, the line and column of the error (counted from
one), and the error; a rejected `Generate` creates no directory.

One inline datom value, no flags; every reply is a value of the
contract's `Response`, written on one line. With no argument, `ethos-zero` prints its own
ethos in the canonical print.

## Gates

`cargo test` locally; `nix flake check` is the durable gate (build,
test, fmt, clippy, doc, the no-free-functions and no-inherent-methods
guards, and the dependency ethos declarations read by the built tool).
Each Nix derivation applies an 8 GiB virtual-memory limit inside its
builder.
