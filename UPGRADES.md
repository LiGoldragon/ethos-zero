# Upgrades

How to deploy each breaking change of ethos-zero.

## 16.0.0: every root archives and gates its datom kinds

What breaks:

- A Library's, an Operation's and a Memory's generated structs and enums now
  carry what a Signal's carry: `#[derive(rkyv::Archive, rkyv::Serialize,
  rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]` and
  `#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable,
  datom_codec::Composing))]`. Until 15.0.0 they derived the datom kinds
  unconditionally and did not archive. Any type can now cross a wire, and a
  Nexus compiles its whole contract without datom-codec (ruling 11 of flow
  3ec648). The Flow Nexus's Signal, which holds the Library's `Voice`,
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
only by their checks (kinds, which carry no derive, and anatomy).

## 15.0.0: four roots, Memory and Operation

What breaks:

- The `Sema` root is renamed `Memory`: same sections (imports, record
  types), same generation. A file headed `Sema` is refused as
  `Rejected.{ file { 1 1 } Conceptual.{ [ 0 ] Renamed.Memory } }`. The
  `Error` contract gains `Problem.Renamed.String`.
- The `Operation` root is added: `Operation [ imports ] [ operations ]
  [ outcomes ] [ types ]`, generating `pub enum Operation` and
  `pub enum Outcome` the way a Signal generates `Query` and `Response`,
  each variant carrying its declared payload (`Start.{ Voice Capsule }`
  gives `Start(Start_Data)`). Its types derive the datom kinds
  unconditionally and do not archive, as a Library's and a Memory's.
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

## 14.0.0: capability inputs are kinds

What breaks: a concrete type in a capability's input is refused, where
13.0.0 accepted it and wrote it as the parameter's type. The refusal is
`Rejected.{ file { line column } Conceptual.{ [ path ] KindWanted.<Name> } }`,
the line and column naming the input. A kind name in an input, which
13.0.0 refused as `Role.<Kind>`, now generates a method parameter bounded
by that kind (`fn resolve<N: Textualizable>(&self, input: N) -> Self`), or
the kind's associated type where one is already bounded by that kind.
An imported name in an input is taken as a kind: where it names a type,
the generated bound names a type and rustc refuses the generated module.
The `Error` contract gains `Problem.KindWanted.String`.

To deploy, in each consumer:

1. Run `ethos-zero 'Check./abs/file.ethos'` on every ethos file it owns.
2. For each `KindWanted`, name the kind the input wants and put it in the
   input: declare it (`Textualizable.[ textualize.[ String ] ]`) or import
   it, or use `Self` or a parameter of the kind's head. The implementer
   then writes `fn name<N: Kind>(…, input: N)` and calls the kind's
   capabilities on the input.
3. For an imported type in an input, which Check cannot see, regenerate
   and compile: rustc's `expected trait, found struct` names it; replace
   it as in step 2.
4. Regenerate the committed Rust, update the hand-written implementations
   to the generic signatures, and bump the consumer's own major version
   where the generated kind is public.

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
  (`c:[ Sized ]`, a kind in a yield).

Correction (14.2.0): `datom-codec/datom-codec.ethos` at 58474fd was refused
by 14.x (`KindWanted.Path`, its second declaration of `Datomizable` with
`Path` in the input), so the `dependency-ethos` check against it failed; the
scan above missed it. datom-codec 0930abc gives the kinds one home in
`datom-codec-kinds.ethos` and names them (`Branchable`, `Budgeted`,
`Positional`, `Composable`); protos 0.32 names `Spendable`. Both kinds files
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
