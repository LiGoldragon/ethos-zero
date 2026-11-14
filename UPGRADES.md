# Upgrades

How to deploy each breaking change of ethos-zero.

## 28.0.0: a variant with its own payload carries it; fields that would meet are refused

What breaks, in two parts.

1. The `Error` contract's `Problem.Payload.String` is gone, and `Problem.DuplicateField.String` stands in its place. A consumer matching `Problem` exhaustively replaces the arm.
2. A struct two of whose positions would generate one field name is refused as `Conceptual.{ [ path ] DuplicateField.<field> }`, at the later position: `Pair.{ String String FirstString }` names `first_string` twice. 27.x generated both fields, a struct Rust refuses to compile; no file 27.x generated into compiling Rust is newly refused.

What is now accepted: the carrying rule is a bare variant's alone (vision/ethos/0.md, Shapes and placement). A variant with a payload of its own is a variant of that name carrying that payload, beside a declared or imported type of the same name, in every section: the vision's `Lock.LockRequest` input beside a declared `Lock` gives `Input::Lock(LockRequest)`, and `Code.String`, `Code.Code` or `Code.{ … }` beside a declared `Code` gives `Code(String)`, `Code(Code)` or `Code(Code_Data)`. 21.0.0 to 27.x refused each as `Payload.<Name>`. `fixtures/lock-signal.ethos` is the vision's Signal.

Field names are unchanged: a field is named for its type alone and never its namespace, as under 27.x (`nexus:ConfigurationState<OrchestrateNexusConfiguration>` gives `orchestrate_nexus_configuration_configuration_state`, the constructed type type-first; `:a:Error :b:Error` gives `first_error` and `second_error`). Every file 27.x generated is generated the same, so the wire is unchanged.

To deploy: repin ethos-zero to 28.0.0 and replace a `Problem::Payload` arm with `Problem::DuplicateField`. A variant renamed to satisfy 22.0.0 (orchestrate's `Acquire.LockRequest`) may keep its name or return to the one it was meant to have.

## 27.0.0: the input is Answerable; Answering holds the capabilities alone

What breaks, in three parts.

1. A Signal's generated `Answering` holds one capability per input and nothing else: 26.x's provided `fn answer(&self, input: &Input) -> Output` is gone from it. Each capability is as under 26.x, borrowing what its input carries and yielding `std::result::Result<_, Output>`.
2. The dispatch is the input's. Beside `Answering`, every Signal with inputs generates `pub trait Answerable { fn answer<Nexus: Answering>(&self, nexus: &Nexus) -> Output; }` and `impl Answerable for Input`, holding the match: `Input::Launch(carried) => nexus.launch(carried).map_or_else(|refusal| refusal, Output::Launched)`. Every arm binds the payload as `carried`. No standard library crate holds the root-implied traits yet, so `Answerable` is generated into each Signal's file, as `Answering` is; two Signals' generated modules glob-imported into one module meet two traits of each name.
3. `Answerable` is reserved in a Signal, as `Answering` is: `Signal [] [ Ask ] [ Told ] [ Answerable.String ]` is refused as `Conceptual.{ [ 1 3 0 0 ] Duplicate.Answerable }`.

What is now accepted: an input named `Answer`. 24.0.0 to 26.x refused it as `Duplicate.answer`, the dispatch's name; its capability is now `fn answer(&self, …)` on `Answering`, and the dispatch, on `Input`, meets no name of it. `fixtures/answer-input.ethos` is such a Signal. Two inputs whose capabilities share a name stay refused as `Duplicate.<capability>`.

The wire is unchanged: every generated type is as under 26.x.

To deploy: replace each `nexus.answer(&input)` with `input.answer(&nexus)`, importing `<signal>::Answerable` where `answer` is called; `Answering` need not be imported there. An `impl Answering` is unchanged unless it overrode the provided `answer`, which no longer exists. A Signal that declares or imports a type named `Answerable` renames it.

## 26.0.0: a Signal implies Input and Output

What breaks, in three parts.

1. A Signal's implied enums are renamed as the vision has them, `Signal [ imports ] [ inputs ] [ outputs ] [ types ]` with the implied namespace `signal:[ Input Output ]`: a Signal generates `pub enum Input` over its first section and `pub enum Output` over its second, where 25.x generated `pub enum Query` and `pub enum Response`. Inline payloads follow the enum's name: `Input_Ask_Data` and `Output_Ask_Data` where 25.x gave `Query_Ask_Data` and `Response_Ask_Data`. `Answering` speaks the same names: `fn launch(&self, launch: &Launch_Data) -> std::result::Result<flow::FlowId, Output>` and `fn answer(&self, input: &Input) -> Output`; a capability whose input carries `Vector<Self>` takes `input_vector: &Vec<Input>`.
2. A Signal may not declare `Input` or `Output`, as a type, an inline payload or an import: `Signal [] [ Ask ] [ Told ] [ Input.{ String } ]` is refused as `Conceptual.{ [ 1 3 0 0 ] Duplicate.Input }`. `Query` and `Response` become ordinary names a Signal may declare. A Signal that 25.x accepted with a declared or imported `Input` or `Output` (signal-energy-nexus's `Call.{ Caller Input.[ … ] }` declares `Input` in place) is now refused.
3. The public model's `Signal` struct renames its fields: `queries` is `inputs` and `responses` is `outputs`.

The wire is unchanged: variant order, names and payloads stay as they were, so the rkyv archive of every value is the same. The ethos text of a Signal is unchanged too; only a Signal that declares `Input` or `Output` must be edited, which changes its text.

To deploy: run `ethos-zero 'Check.<file>'` over every Signal and rename a declared or imported `Input` or `Output` (and its uses) that it refuses. In Rust, replace each generated `<signal>::Query` with `<signal>::Input` and `<signal>::Response` with `<signal>::Output`, in paths, `use` lists, matches, constructors, `Potential::<Query>` and rkyv type arguments, and an inline payload's `Query_<X>_Data` or `Response_<X>_Data` with `Input_<X>_Data` or `Output_<X>_Data`; an `impl Answering` returns `Output`. A consumer that reads `Signal { queries, responses, .. }` from the public model reads `inputs` and `outputs`. A Library whose own types are named `Query` and `Response` (ethos-zero's own `ethos-zero.ethos` among them) is not a Signal and keeps them.

## 25.0.0: an Answering capability borrows its input

What breaks: every capability of a Signal's generated `Answering` borrows what its query carries, and the dispatch borrows the query. 24.0.0 generated `fn launch(&self, launch: Launch_Data) -> std::result::Result<flow::FlowId, Response>` and `fn answer(&self, query: Query) -> Response`; 25.0.0 generates `fn launch(&self, launch: &Launch_Data) -> …` and `fn answer(&self, query: &Query) -> Response`, matching on the borrow and lending each capability the payload it matched. The input's type is the query's carried type exactly, borrowed: `&String`, `&Vec<_>` and `&Box<_>` stay as they are, and the trait allows `clippy::ptr_arg` and `clippy::borrowed_box` beside `clippy::result_large_err`. A capability that carries nothing still takes nothing. Only `Answering` changes: a declared trait's capabilities keep their by-value form, and every type the file generates is as under 24.0.0.

To deploy: in each `impl Answering`, write every capability's input as a borrow (`launch: &Launch_Data`), cloning what the capability keeps, and call `answer(&query)`.

## 24.0.0: a Signal pairs its responses with its queries and generates Answering

What breaks, in three parts.

1. A Signal pairs by position: the n-th response answers the n-th query, and a response past the last query answers none (a refusal or a reaction). A query left without a response is refused as `Conceptual.{ [ 1 1 <at> ] Unpaired.<query> }`: `Signal [] [ Ask Tell ] [ Told ] []`, which 23.x accepted, is refused at `Tell`.
2. Every Signal with queries generates `pub trait Answering` beside `Query` and `Response`: one capability per query, its variant in snake case, taking the query's payload by value and yielding `std::result::Result<the paired response's payload or (), Response>`, and `fn answer(&self, query: Query) -> Response`, the dispatch, generated whole. A consumer that declares its own `Answering` in the module holding the generated Rust, or glob-imports it beside one of its own, meets two traits of one name.
3. `Answering` is reserved in a Signal (`Duplicate.Answering`), and a query whose capability would be named as another query's or `answer` is refused as `Duplicate.<capability>` (`Answer` as a query, or `UrlFetch` beside `URLFetch`).

To deploy: run `ethos-zero 'Check.<file>'` over every Signal. Order each Signal's responses so the n-th answers the n-th query and the refusals and reactions come after; reordering changes no variant's name or payload, but it does change the rkyv discriminant of every variant moved, so both ends of a wire repin together. Rename a consumer's own `Answering` trait, or implement the generated one: its capabilities in place of a hand-written match. The crate's own `fixtures/orchestrate.ethos` orders its responses `Locked Released Observed LockRejected ReleaseRejected`, and `fixtures/print/flow-signal.ethos` `Launched Reported Refused`.

## 23.0.0: the Emission root

What breaks, in two parts.

1. The public model gains a fifth root: `File::Emission(Emission)` and `Root::Emission`, with `Emission { imports, emissions, outcomes, types }`. A consumer matching `File` or `Root` exhaustively adds the arm.
2. The `Error` contract gains `Problem.Unpaired.String`, the emission left without an outcome. A consumer matching `Problem` exhaustively adds the arm.

What is added: a file headed `Emission`, with four sections read as an Operation's: imports, emissions, outcomes, types. It generates its declared types, `pub enum Emission` over the emissions and `pub enum Outcome` over the outcomes, and the traits `Emitted` and `Emittable<Translator>: Emitted`, with `impl Emitted for <the n-th emission's carried type> { type Outcome = <the n-th outcome's carried type, or ()>; }`. The consumer writes `emit` for each emission type it sends. An emission past the last outcome is refused as `Unpaired.<emission>` at its path; two emissions carrying the same type as `Duplicate.<type>` at the second; a type, trait or import named `Emission`, `Outcome`, `Emitted` or `Emittable` as `Duplicate`. Every file of the four other roots generates the same Rust as under 22.0.0.

To deploy: repin ethos-zero to 23.0.0 and add the arms. An Emission written as a Signal in `emission.ethos` (the interim form) may be rewritten under the `Emission` head: its inputs are its emissions, its outputs its outcomes. The generated variant types keep their names (`Command.{ … }` still gives `Command_Data`); the consumer's own Emittable trait, if it declares one, gives way to the generated `Emittable` (energy-nexus's `crates/emission` declares `Emittable<T>`).

## 22.0.0: one carrying rule in every section, imported types included

What breaks, in three parts.

1. The `Payload` refusal of 21.0.0 holds in every section: a Signal's inputs and outputs and an Operation's operations and outcomes as much as an enum declaration. `Lock.LockRequest` as a Signal input beside a declared `Lock` is now `Conceptual.{ [ 1 1 0 0 ] Payload.Lock }`.
2. A bare variant named after an imported type carries that type, as one named after a declared type does: with `[ :sema:Block ]` imported, `Part.[ Plain.String Block ]` generates `Block(sema::Block)`, where 21.x generated a `Block` carrying nothing. An imported name is held to the refusal too: `Block.Block` is `Payload.Block`.
3. A name a root implies, a Signal's `Query` or `Response`, is generated beside the file's own names: no variant carries it, so `Query.Ask` stays valid and a bare `Query` carries nothing, as before.

To deploy: run `ethos-zero 'Check.<file>'` over every ethos file. For each `Payload.<Name>`, write the variant bare when it is to carry `<Name>` (`Configured.Configured` becomes `Configured`; the generated Rust is the same, an imported `<Name>` included only from 22.0.0, so repin before writing an imported one bare). Rename the variant when it is to carry something else: the orchestrate input `Lock.LockRequest` becomes `Acquire.LockRequest`, and every consumer matching `Query::Lock` matches `Query::Acquire`. A file whose bare variant named an imported type and meant no payload renames that variant. The crate's own `fixtures/orchestrate.ethos` takes `Acquire.LockRequest`, and `error.ethos` writes `Separator` bare, its Rust unchanged.

## 21.0.0: a variant named as a defined type carries it bare

What breaks: an enum declaration (in a types section, or inline in a variant) whose variant is named as a type the file defines and declares a payload of its own is refused as `Conceptual.{ [ path ] Payload.<Name> }`, the path naming the variant. `Code.String` in `TextPart.[ Plain.String Code.String ]` with `Code.String` declared was accepted and generated `Code(String)`; the vision says such a variant carries the type it is named for. The `Error` contract gains `Problem.Payload.String`. A Signal's or an Operation's own sections are not held to this: `Lock.LockRequest` as a Signal input beside a declared `Lock` stays valid.

To deploy: for each `Payload.<Name>`, write the variant bare when it is to carry `<Name>` (`Code.Code` becomes `Code`, generating the same Rust), or rename the variant when it is to carry something else (`Code.String` becomes, say, `Snippet.String`). A consumer that matches `Problem` exhaustively adds the arm. The crate's own `fixtures/orchestrate.ethos` writes `Observation.[ Locks ]` for `Locks.Locks`; its generated Rust is unchanged.

## 20.0.0: a concrete type in a capability input is accepted

What breaks: the `Error` contract loses `Problem.TraitWanted.String`, which 14.0.0 added and nothing raises now. Every input that 19.x accepted generates the same Rust.

A capability input that names a concrete type (`String`, `Vector<Outline>`, a declared type such as `Rec`) is written as that type: `resolve.{ [ Rec ] [ Self ] }` gives `fn resolve(&self, input: Rec) -> Self where Self: Sized;`. A declared trait in an input is still a method parameter bounded by that trait, and an imported name in an input is still taken as a trait. 14.0.0 had refused the concrete type; 13.0.0 had accepted it.

To deploy, a consumer that matched `TraitWanted` drops that arm; a consumer that worked around the refusal by declaring a trait for a type may keep it. A held `ethos-test` check that asserts `TraitWanted.String` for `String` in an input (`checks/ethos-zero-trait-concrete-input.nix`) asserts the old behavior and is updated when ethos-test repins.

## 19.3.1: the crate's own orchestrate fixture follows 19.3.0

Not breaking: the generator, its command line and its public API are unchanged. `fixtures/orchestrate.ethos` declared `DuplicateName.Lock`, a second head over the struct `Lock`, which 19.3.0 refuses as `Problem.Wrapping`, so `regenerate_library_fixtures` failed. The declaration was never used; `LockRejection` carries `DuplicateName.Lock` as a variant, which stays valid. The fixture drops the declaration and `tests/generated/orchestrate.rs` loses the unused `DuplicateName` newtype. The generated variant `LockRejection::DuplicateName(Lock)` is unchanged.

## 19.3.0: a declaration uses a named composite type directly

An alias-shaped declaration in a root's types section may name a scalar or a
newtype. It is refused as `Problem.Wrapping.{ Head WrappedType }` when the
named type resolves to a struct, enum or root-implied enum: the position uses
that existing type directly instead of giving it a second head. For example,
`DuplicateName.Lock` is refused when `Lock` is a declared struct.

This check applies to declarations in the types section only. A variant may
carry any named type (`Lock.LockRequest`), a declaration may directly carry a
vector or an inline shape, and a newtype may carry another newtype. Generated
Rust for every accepted file is unchanged.

To deploy, check every Ethos file before repinning. Replace a refused wrapper
declaration with the wrapped type at each position, or declare the distinct
type the head actually means and update all of its consumers in the same
change. Version 19.3.0 intentionally refuses 26 tracked source files found by
the release inventory; those consumers are updated separately.

## 19.2.1: selects datom-codec 5cd6219 (0.33.1)

Not breaking: the generator's output, its command line and its public API are unchanged. The crate
now depends on datom-codec 0.33.1 where it depended on 0.33.0 (eabead4), and the flake input
`datom-codec` is locked to the same revision. A consumer that pins datom-codec 5cd6219 (for example
for its checked reconstruction of an archived `Decimal`) and uses ethos-zero, even only as a
dev-dependency, then holds one datom-codec source instead of two; a consumer on eabead4 still holds two.

## 19.0.0: a head over one type is a newtype

What breaks:

- A declaration such as `LockId.Integer` generates
  `pub struct LockId(pub i64)` instead of `pub type LockId = i64`.
- Each generated newtype carries the standard rkyv derives and the
  feature-gated `Datomizable` and `Composing` derives. Consumers construct
  it with `LockId(value)` and access the held value as `.0`.

To deploy, regenerate every committed consumer and update its constructors
and projections before repinning ethos-zero to 19.0.0.

## 18.0.0: imports are rooted, with every namespace segment

What breaks:

- `Source` reads the Ethos spelling: `TryFrom<&str>` takes `:a:b`
  (rooted, any number of segments) or `a` (one chained segment), and
  refuses the Rust path text `a::b` it took before; `AsRef<str>` still
  reads the Rust path. `Spelled` reads its segments and whether it is rooted.
- A namespace segment is uncapitalized: a capitalized chained source,
  `Flow:Type`, is refused.
- A capitalized name before a colon in a struct position declares in place:
  `Holder.{ Topic:custom:Name }` declares `Topic` over `custom::Name`, where
  it was refused as a second inline source.

What is added: `:flow_ethos:message:Type` and `:protos:[ String Integer ]`
in the imports section, `Topic:custom:sub:Name` as an alias, and
`:protos:Error` in a field. The chained forms `protos:String` and
`Topic.custom:Name` still read.

To deploy, repin ethos-zero to 18.0.0; an ethos file may then replace a
chained `crate:[ … ]` workaround with the rooted import it meant.

## 17.0.0: capability declarations are traits

What breaks:

- A Library's third section is named `traits`; its position and the
  four-section shape do not change.
- The public model renames the declaration and body types, Library and
  Association fields, role, resolution and form variants, and refusal
  variant to trait-named forms. The refusal is now `TraitWanted`. Consumers
  update Rust references and regenerate output.
- Fixture paths and generated fixture modules use `-traits`; no old name is
  kept as an alias.

To deploy, update source references, regenerate committed Rust, repin
  ethos-zero to 17.0.0 and bump a consumer's major version where its
  generated public surface changes.

## 16.0.0: every root archives and gates its datom derives

What breaks:

- A Library's, an Operation's and a Memory's generated structs and enums now
  carry what a Signal's carry: `#[derive(rkyv::Archive, rkyv::Serialize,
  rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]` and
  `#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable,
  datom_codec::Composing))]`. Until 15.0.0 the datom-codec derives were
  unconditional and the types did not archive. Any type can now cross a
  wire, and a Nexus compiles its whole contract without datom-codec (ruling
  11 of flow 3ec648). The Flow Nexus's Signal, which holds the Library's `Voice`,
  `FlowId` and `Event`, now compiles; the `flow-contract` test compiles the
  four Flow modules together, with and without `datom`, and sends a value of
  each root through rkyv.
- A position holding a foreign type now needs that type to archive. protos
  0.32.2 (`rkyv` feature) archives `Extent`, `Separator`, `Error` and
  `Problem`; datom-codec 0.32.2 (`rkyv` feature, which enables protos's)
  archives `Error`, `ErrorLayer` and `ErrorKind` besides `Decimal` and
  `Meaning`. ethos-zero now pins protos 15b41da8 and datom-codec 4dff16b4.
- Library API: the generator's `Carriage` and `Carrying` (never exported)
  are gone.

To deploy, in each consumer whose committed or build-script output comes
from a Library, Operation or Memory file:

1. Depend on rkyv 0.8 (`default-features = false`, features `std`,
   `bytecheck`, `little_endian`, `pointer_width_32`, `unaligned`).
2. Declare a `datom` feature, `datom = ["dep:datom-codec"]`, make
   datom-codec optional, and enable `datom` where the crate textualizes:
   its CLI, its tests that datomize, any code that names `Datomizable` or
   `Composing` on a generated type. Where a position holds a datom-codec or
   protos type, enable datom-codec's (or protos's) `rkyv` feature
   unconditionally and repin them to 0.32.2.
3. Regenerate the committed Rust, repin ethos-zero, and run the tests with
   and without `datom`.

Consumers, from a grep of the `.ethos` files of every repository under
`/git` that depends on ethos-zero (2026-10-02); none is changed by this
release, and each pins an ethos-zero older than 15.0.0, so each breaks only
when it repins:

- `github.com/LiGoldragon/chroma/chroma.ethos` (Library; pins b232d35e)
- `github.com/LiGoldragon/claude-answers/claude-answers.ethos` (Library; b232d35e)
- `github.com/LiGoldragon/clavifaber/ethos/clavifaber.ethos` (Library; 4bf73cae)
- `github.com/LiGoldragon/curriculum-deploy/curriculum-deploy.ethos` (Library; b232d35e)
- `github.com/LiGoldragon/lojix/ethos/ingress.ethos` (Library; de3d9928)
- `github.com/LiGoldragon/meaning-language/ethos/meaning.ethos` (Library; 4bf73cae)
- `github.com/LiGoldragon/orchestrate/crates/orchestrate/client.ethos` and
  `crates/orchestrate-meta/client.ethos` (Library; cf7dd128)
- `github.com/LiGoldragon/signal-5f4fea-word-identifiers/ethos/identifiers.ethos`
  (Library; 4bf73cae; already gates datom-codec behind `datom`)

No consumer generates from a Memory or Operation file. Signal consumers are
unaffected; a Signal that holds a type from one of the crates above gains
the rkyv it lacked. protos's and datom-codec's own `.ethos` files are read
only by their checks (traits, which carry no derive, and anatomy).

## 15.0.0: four roots, Memory and Operation

What breaks:

- The `Sema` root is renamed `Memory`: same sections (imports, record
  types), same generation. A file headed `Sema` is refused as
  `Rejected.{ file { { 1 } { 1 } } Conceptual.{ [ 0 ] Renamed.Memory } }`. The
  `Error` contract gains `Problem.Renamed.String`.
- The `Operation` root is added: `Operation [ imports ] [ operations ]
  [ outcomes ] [ types ]`, generating `pub enum Operation` and
  `pub enum Outcome` the way a Signal generates `Query` and `Response`,
  each variant carrying its declared payload (`Start.{ Voice Capsule }`
  gives `Start(Start_Data)`). Its datom-codec derives are unconditional and
  its types do not archive, as a Library's and a Memory's.
  **The section order is proposed, pending the living's word** (ruling 3,
  4b, of flow 3ec648); it may change before it is his.
- A struct position may declare its type in place: `Brief.String`,
  `State.[ Running Ended ]`, `Capsule.{ Home.String Login.Vector<String> }`.
  The type takes its own name in the file's namespace and the position
  holds it by name (`brief: Brief`). 14.x refused such a position as
  `Expected.Reference`; a file that does not write one generates as before.
- Library API: `File::Sema(Sema)` is `File::Memory(Memory)`; `File` and
  `Root` gain `Operation`; `TypeDeclaration::Struct` and `Variant::Struct`
  hold `Vec<Position>` (`Position::Referenced(Reference)` or
  `Position::Declared(TypeDeclaration)`) where they held `Vec<Reference>`.
  No consumer under `/git` names these types; the build-script consumers
  only call `Potential<File>`, `Actualizing` and `Generating`.

To deploy, in each consumer:

1. Rename the head of every file headed `Sema` to `Memory`; nothing else in
   the file changes. Rename the file where its name says `sema`, if the
   consumer wants the generated module named for it.
2. Regenerate the committed Rust and run its tests. The generated Rust of a
   renamed file is byte-identical to 14.2.0's.
3. Repin ethos-zero; bump the consumer's own version where it publishes the
   generated module.

No live consumer uses the Sema root (corrected after 15.0.0; flow 3ec648
report `sema-to-memory.md`). The seven `.ethos` files under `/git` headed
`Sema` are not ethos-zero consumers: spirit-ethos and core-ethos (core-schema)
are frozen, and core-ethos's fixture is its own bootstrap reader's; spirit's
schema is deprecated; five of the seven are in an older dialect ethos-zero
refuses under either head; primary-next holds one as report evidence and two
as flow f6db8d's witnesses. None pins ethos-zero, and nothing is generated
from any of them, so the rename has nowhere to deploy.

The Flow Nexus's four files under `fixtures/print/` now generate, Operation
and Memory included, and still print back byte-identical; their generated
Rust is committed under `tests/generated/flow-*.rs`. The Library, Operation
and Memory modules compile and round-trip as datom text. The Signal's
does not compile yet: its archived types carry the Library's `Voice`,
`FlowId` and `Event`, and a Library type does not derive rkyv (closed by
16.0.0).

## 14.0.0: capability inputs are traits

What breaks: a concrete type in a capability's input is refused, where
13.0.0 accepted it and wrote it as the parameter's type. The refusal is
`Rejected.{ file { line column } Conceptual.{ [ path ] TraitWanted.<Name> } }`,
the line and column naming the input. A trait name in an input, which
13.0.0 refused as `Role.<Trait>`, now generates a method parameter bounded
by that trait (`fn resolve<N: Textualizable>(&self, input: N) -> Self`), or
the trait's associated type where one is already bounded by that trait.
An imported name in an input is taken as a trait: where it names a type,
the generated bound names a type and rustc refuses the generated module.
The `Error` contract gains `Problem.TraitWanted.String`.

To deploy, in each consumer:

1. Run `ethos-zero 'Check./abs/file.ethos'` on every ethos file it owns.
2. For each `TraitWanted`, name the trait the input wants and put it in the
   input: declare it (`Textualizable.[ textualize.[ String ] ]`) or import
   it, or use `Self` or a parameter of the trait's head. The implementer
   then writes `fn name<N: Trait>(…, input: N)` and calls the trait's
   capabilities on the input.
3. For an imported type in an input, which Check cannot see, regenerate
   and compile: rustc's `expected trait, found struct` names it; replace
   it as in step 2.
4. Regenerate the committed Rust, update the hand-written implementations
   to the generic signatures, and bump the consumer's own major version
   where the generated trait is public.

Contracts that depend on this, found by generating every `.ethos` file
under `/git` with 13.0.0 and with 14.0.0 and comparing (2026-10-02):

- Refused by 14.0.0 and accepted by 13.0.0: none.
- Imported concrete types in inputs, now generated as bounds that rustc
  refuses on regeneration:
  - `github.com/LiGoldragon/protos/protos-kinds.ethos`:
    `BoundedProtosizable.protosize_with!{ [ ReaderBudget ] … }`.
  - `github.com/LiGoldragon/datom-codec/datom-codec-kinds.ethos`:
    `DatomForming.datom_form.{ [ Path ] … }`, `Datomizable.datomize.{ [ Path ] … }`,
    `Composing.compose:{ [ Datom Budget ] … }`,
    `Compositional.from_positions:{ [ Positions ] … }`,
    `Composable.compose.{ [ Budget ] … }`, `Composable.compose_positions.{ [ Budget ] … }`,
    `Actualizing.actualize!{ [ Budget ] … }`.
  - The vendored copies of both in
    `github.com/LiGoldragon/primary-next/tools/messaging-codec/vendor/`.
- Newly accepted: `primary-next/flows/f6db8d/witnesses/substrate/probe-ethos/sized-kind.ethos`
  (`c:[ Sized ]`, a trait in a yield).

Correction (14.2.0): `datom-codec/datom-codec.ethos` at 58474fd was refused
by 14.x (`TraitWanted.Path`, its second declaration of `Datomizable` with
`Path` in the input), so the `dependency-ethos` check against it failed; the
scan above missed it. datom-codec 0930abc gives its declarations one home in
`datom-codec-kinds.ethos` and names them (`Branchable`, `Budgeted`,
`Positional`, `Composable`); protos 0.32 names `Spendable`. Both schema files
now generate, and their generated Rust compiles in their own repositories.

## 14.2.0: the print is protos'

The vertical layout moved into protos 0.32, whose `textualize` is now the
canonical vertical print; ethos-zero keeps only the sweet form of a file
(its root's head, then each section from the first column) and writes each
section with protos. One difference from 14.1.0: an empty `[]` or `{}` is a
leaf, so a structure whose only elements with brackets are empty sits on one
line. The CLI's datom replies stay on one line, through protos' `compact`.

Pins: protos 109797e (0.32.1), datom-codec 0930abc (0.32.1); datom-codec
renamed `Pathing`, `Budgeting`, `Positioning` to `Branchable`, `Budgeted`,
`Positional`, and a datom written with `textualize` is vertical (a generated
Sema record prints `{ root\n  [ { first 1 } ] }`).

To deploy: a consumer of the library repins and runs its tests; an expected
one-line datom text of a nested structure becomes the vertical print, or the
consumer calls `protos::Compactable::compact`. CLI users see no change.
