# Upgrades

How to deploy each breaking change of ethos-zero.

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
