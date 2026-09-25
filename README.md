# ethos-zero

The ethos schema language, version zero. Ethos specifies the types,
datom fills them with data, and ethos generates the Rust.

## Anatomy

`src/lib.rs` is the ontology: the layers, the kinds, the declaration
types. Each pass is a module named for it.

| pass | kind | from | to |
|---|---|---|---|
| canonicalization | `Canonicalizable` | sweet text | `Canonical`, the braced form, with its seam |
| protosization (protos) | `Protosizable` | canonical text | `protos::Protos` |
| conception | `Ethosizable<File>` | `protos::Protos` | `File`, checked whole |
| checking | `Resolving`, `Checkable` | `File` | names resolved, duplicates and undeclared names refused |
| generation | `Generating` | `File` | Rust text, or a whole-file error |
| datomization | | each declared type | its `datom_codec::Datomizable` and `datom_codec::Composing` derives |
| ascent | `Protosizable`, `Textualizable` | `File` | canonical text (cannot err) |
| actualization | `Actualizing<File>` on `Potential<File>` | text | `File`, or an `Error` |

Every error carries the Protos path of the structure in error, from the
file's root: a headed form puts its head at child zero and body at child
one, qualified head arguments remain below the head, and a reference's
arguments are the angled enclosure beside it in its list
(`Vector<Bogus>` at index 0 puts `Bogus` at `[ 1 0 ]`). `Locating`
follows that path down the source's structure and carries the node's
start back across the sweet form's seam, so every error is situated as a
line and a column in the text as written. Every method
call lives under a kind; there are no free functions, no inherent impls,
no closures beyond what std forces, and no lookup tables: the enums are
walked variant by variant.

The crate eats its own food: `error.ethos` generates `src/error.rs`
and `ethos-zero.ethos` generates `src/ethos-zero.rs`; the freshness test
regenerates both and every fixture under `tests/generated/`.

## File variants

```
Library  [ imports ] [ types ] [ kinds ] [ associations ]
Signal   [ imports ] [ queries ] [ responses ] [ types ]   ; Query and Response implied
Sema     [ imports ] [ record types ]
```

A Signal generates `pub enum Query` and `pub enum Response` from its
first two sections, so those two names are the ones a Signal may not
also declare. Sema generates nothing but its declared record types and
reserves no name; a Sema record may be named `Record`.

An import names a Rust path prefix and the names taken from it:
`std:clone:Clonable.Clone` imports one name, and
`std:clone:[ Clonable.Clone ]` imports a group. A position is boxed only
where it closes a by-value cycle: it names, through aliases, `Option` and
`Result` but not `Vector`, a type declared no later than its owner that
reaches the owner by value; in `Twin.{ Twig Twig }  Twig.[ Tip  Grow.Twin ]`
only `Grow` is boxed. In a Signal, every position that reaches its owner by
any path, `Vector` included, carries `#[rkyv(omit_bounds)]`, and its type
states the serializer, deserializer and validator bounds once, so a
recursive contract archives and restores. The generated Rust writes
standard containers as `std::vec::Vec`, `std::option::Option`,
`std::result::Result`, and `std::boxed::Box`, so a declaration cannot
capture those names. A variant `Name.{ T1 T2 }` carries a generated
struct `Name_Data` with one field per position.

A type or kind declaration is refused when its name is an intrinsic's
(`Intrinsic.Result`: the declaration would shadow the intrinsic for every
later reference), or does not begin with a capital (`Case.a`). A declared
type with no finite value, one that reaches itself with no `Vector`,
`Option` or other variant to stop at, such as `S.{ Self }`, is refused as
a `Cycle`. Every generated item carries `#[rustfmt::skip]`, an inline
payload's as much as its enum's.

A variant that declares its payload in place names a derived type:
`X.{ … }` or `X.[ … ]` in an enum gives `X_Data`. Derived names are unique
across the whole file: where two enums each declare an `X` in place, each
payload is named for its enum instead (`P_X_Data`, `Q_X_Data`), and a payload
declared inside a derived enum carries that enum's name as its stem
(`Y_Data_X_Data`). An authored name equal to a derived name is refused as a
`Duplicate` at the authored declaration.
Generated generic parameters similarly move from `A`, `B`, and so on only
when one would capture an authored type reference.

The flat declaration budget applies to a Library's types section: it refuses
more than 512 type declarations. The other roots retain their structural reader
bounds and are not counted against it.

File ascent projects the File directly into its `protos::Protos`
structure; it does not textualize and parse the result again.

## CLI

```
ethos-zero 'Generate.{ /abs/file.ethos /abs/out-dir }'
# -> Generated.[ /abs/out-dir/file.rs ]
ethos-zero 'Check./abs/file.ethos'
# -> Checked./abs/file.ethos
# -> Rejected.{ /abs/file.ethos { 4 19 } Conceptual.{ [ 1 1 0 1 1 ] Undeclared.Bogus } }
```

`Check` validates the whole file and writes nothing. A rejection, from
either query, names the file, the line and column of the error (counted
from one), and the error; a rejected `Generate` creates no directory.

One inline datom value, no flags; every reply is a value of the
contract's `Response`. With no argument, `ethos-zero` prints its own
ethos.

## Gates

`cargo test` locally; `nix flake check` is the durable gate (build,
test, fmt, clippy, doc, the no-free-functions and no-inherent-methods
guards, and the dependency ethos declarations read by the built tool).
Each Nix derivation applies an 8 GiB virtual-memory limit inside its
builder.
