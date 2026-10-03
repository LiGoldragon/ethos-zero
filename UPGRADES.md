# Upgrades

How to deploy each breaking change of ethos-zero.

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
