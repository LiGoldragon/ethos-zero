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

Every error carries the Protos path of the structure in error: a headed
form puts its head at child zero and body at child one, while qualified
head arguments remain below the head. Actualization follows that
structure directly to situate the error in its source text. Every method
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
`std:clone:[ Clonable.Clone ]` imports a group. A position that reaches
its declaring type is boxed, and only there. The generated Rust writes
standard containers as `std::vec::Vec`, `std::option::Option`,
`std::result::Result`, and `std::boxed::Box`, so a declaration cannot
capture those names. `Name.{ T1 T2 }` is a tuple variant.

Inline enum bodies receive an internal Rust type name. When concatenating the
enclosing type and variant name would occupy an authored type name, generation
allocates an `EthosNested` name instead; authored identities stay unchanged.
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
```

One inline datom value, no flags; every reply is a value of the
contract's `Response`. With no argument, `ethos-zero` prints its own
ethos.

## Gates

`cargo test` locally; `nix flake check` is the durable gate (build,
test, fmt, clippy, doc, the no-free-functions and no-inherent-methods
guards, and the dependency ethos declarations read by the built tool).
Each Nix derivation applies an 8 GiB virtual-memory limit inside its
builder.
