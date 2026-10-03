# Upgrades

## 0.32.0

Breaking: the canonical print expands vertically.

- `Textualizable::textualize` writes the vertical print: a `{ }` or `[ ]`
  enclosure with more than one element, one of which has a next layer, hangs
  its elements aligned beneath the first, the closer on the last line. An
  angled enclosure is written tight against the element it follows
  (`Vector<Event>`, formerly `Vector <Event>`). Text that held no such
  enclosure is unchanged.
- `Canonicalizable::canonicalize` assigns the extents of that vertical text.
- The former one-line print is `Compactable::compact`. A consumer that needs one
  line (a line-oriented log, a reply matched as a single line) replaces
  `.textualize()` with `.compact()` and imports `protos::Compactable`.
- `ReaderBudgeting` (never exported) is the exported kind `Spendable`;
  `ReaderBudget` bears it.

To deploy: repin protos, then run the consumer's tests. Every expected text
that asserted a one-line print of a nested structure fails; either update it
to the vertical print or call `compact`. Reading is unchanged: both prints read
back to the same structure.
